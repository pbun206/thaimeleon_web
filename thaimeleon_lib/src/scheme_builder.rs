use crate::{
    ab::Ab,
    accent_hues::{AccentHueConfigBuilder, AccentHueBuilderError},
    color_profile::ColorProfile,
    color_traits::{ClampByChromaRange, GetChroma, IntoRgb, SrgbClamp, WithLightness},
    contrast::{self, Contrast},
    hue_utils::{find_reference_color, hue_difference},
    neutral_color_builder::{NeutralColorBuilder, NeutralColorBuilderError},
    oklrab::Oklrab,
    parse_image::ParseImageError,
    thaimeleon_scheme::{NamedHues, ThaimeleonScheme},
    utils::{filter_and_average_distances, map_range},
};
use derive_more::{Display, Error, From};
use palette::{
    GetHue, Lab, OklabHue, Srgb,
    color_difference::Ciede2000,
    convert::{FromColorUnclamped, IntoColorUnclamped},
};
use quantette::ImageRef;

const N_ACCENTS: usize = 6;

/// This error describes errors from scheme builder
#[derive(Debug, Display, From, Error)]
#[non_exhaustive]
pub enum GenerationError {
    #[display("failed to parse image: {_0}")]
    ParseImage(ParseImageError),
    #[display("failed to generate neutral color: {_0}")]
    NeutralColorGeneration(NeutralColorBuilderError),
    #[display("failed to generate accent hues: {_0}")]
    AccentHueGeneration(AccentHueBuilderError),
    #[display("no room to place surface_low between base and gamut boundary")]
    NoRoomForLowSurface,
    #[display("baseline chroma too low to derive accent color at requested hue")]
    LowBaselineChroma,
    #[display("accent hue generator returned wrong number of hues (expected {N_ACCENTS})")]
    WrongAccentHueCount,
}

/// This struct stores configuration values to build a chroma value based on lightness and hue
#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub struct ChromaBuilder {
    pub chroma_intercept: f32,
    pub lightness_to_chroma_slope: f32,
    pub hue_low_point: OklabHue,
    pub low_point_chroma_intercept: f32,
}

impl ChromaBuilder {
    /// Creates a chroma with a single output
    pub fn new_constant(chroma: f32) -> Self {
        ChromaBuilder {
            chroma_intercept: chroma,
            lightness_to_chroma_slope: 0.0,
            hue_low_point: OklabHue::default(),
            low_point_chroma_intercept: chroma,
        }
    }

    /// Generate a chroma value using lightness and hue greater or equal than zero
    pub fn generate(self, lightness: f32, hue: OklabHue) -> f32 {
        // Range: [0.0, 1.0]
        let hue_delta = hue_difference(hue, self.hue_low_point) / 180.0;
        let intercept = self.chroma_intercept
            - (1.0 - hue_delta) * (self.chroma_intercept - self.low_point_chroma_intercept);
        (intercept + self.lightness_to_chroma_slope * lightness).max(0.0)
    }
}

/// This struct stores configuration values for SchemeBuilder
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct ThemeConfig {
    pub base_lightness_minimum: f32,
    pub base_lightness_maximum: f32,
    pub surface_distance: f32,

    pub set_2_lightness_correction: f32,
    pub faint_dps_contrast: f32,
    pub set_3_dps_contrast: f32,
    pub set_4_dps_contrast: f32,
    pub set_5_dps_contrast: f32,

    pub neutral_chroma_blend: f32,
    pub bg_neutral_chroma_builder: ChromaBuilder,
    pub fg_neutral_chroma_builder: ChromaBuilder,

    pub prefered_hue_angle: f32,
    pub minimum_hue_angle: f32,
    pub chroma_weight_priority: f32,
    pub penalty_weight_priority: f32,

    pub maximum_accent_hue_center_translation: f32,
    pub high_contrast_fg_accent_radius_baseline: f32,
    pub fg_accent_radius_baseline: f32,
    pub rg_accent_radius_baseline: f32,
    pub bg_accent_radius_baseline: f32,

    pub red_chroma_minimum: f32,
    pub orange_chroma_minimum: f32,
    pub yellow_chroma_minimum: f32,
    pub green_chroma_minimum: f32,
}

impl ThemeConfig {
    /// Default light theme config. Mirrors the shipped CLI defaults.
    pub fn default_light() -> Self {
        Self {
            base_lightness_minimum: 0.964,
            base_lightness_maximum: 0.964,
            surface_distance: 0.0225,
            set_2_lightness_correction: 0.01,
            faint_dps_contrast: 15.0,
            set_3_dps_contrast: 40.0,
            set_4_dps_contrast: 65.0,
            set_5_dps_contrast: 80.0,
            neutral_chroma_blend: 1.0,
            bg_neutral_chroma_builder: ChromaBuilder {
                chroma_intercept: 0.492,
                lightness_to_chroma_slope: -0.482,
                hue_low_point: OklabHue::from_degrees(230.0),
                low_point_chroma_intercept: 0.466,
            },
            fg_neutral_chroma_builder: ChromaBuilder {
                chroma_intercept: 0.05,
                lightness_to_chroma_slope: 0.0,
                hue_low_point: OklabHue::from_degrees(90.0),
                low_point_chroma_intercept: 0.03,
            },
            prefered_hue_angle: 50.0,
            minimum_hue_angle: 45.0,
            chroma_weight_priority: 0.015,
            penalty_weight_priority: 2.0,
            maximum_accent_hue_center_translation: 0.005,
            high_contrast_fg_accent_radius_baseline: 0.02,
            fg_accent_radius_baseline: 0.09,
            rg_accent_radius_baseline: 0.065,
            bg_accent_radius_baseline: 0.0175,
            red_chroma_minimum: 0.12,
            orange_chroma_minimum: 0.08,
            yellow_chroma_minimum: 0.08,
            green_chroma_minimum: 0.07,
        }
    }

    /// Default dark theme config. Mirrors the shipped CLI defaults.
    pub fn default_dark() -> Self {
        Self {
            base_lightness_minimum: 0.1,
            base_lightness_maximum: 0.185,
            surface_distance: 0.06,
            set_2_lightness_correction: 0.01,
            faint_dps_contrast: 15.0,
            set_3_dps_contrast: 30.0,
            set_4_dps_contrast: 72.5,
            set_5_dps_contrast: 82.5,
            neutral_chroma_blend: 1.0,
            bg_neutral_chroma_builder: ChromaBuilder {
                chroma_intercept: 0.07,
                lightness_to_chroma_slope: 0.0833,
                hue_low_point: OklabHue::from_degrees(270.0),
                low_point_chroma_intercept: 0.025,
            },
            fg_neutral_chroma_builder: ChromaBuilder {
                chroma_intercept: 0.02,
                lightness_to_chroma_slope: 0.0,
                hue_low_point: OklabHue::from_degrees(90.0),
                low_point_chroma_intercept: 0.01,
            },
            prefered_hue_angle: 50.0,
            minimum_hue_angle: 45.0,
            chroma_weight_priority: 0.015,
            penalty_weight_priority: 2.0,
            maximum_accent_hue_center_translation: 0.005,
            high_contrast_fg_accent_radius_baseline: 0.035,
            fg_accent_radius_baseline: 0.08,
            rg_accent_radius_baseline: 0.07,
            bg_accent_radius_baseline: 0.06,
            red_chroma_minimum: 0.12,
            orange_chroma_minimum: 0.08,
            yellow_chroma_minimum: 0.08,
            green_chroma_minimum: 0.07,
        }
    }
}

/// Builder config struct to generate Thaimeleon scheme.
pub struct SchemeBuilder {
    pub k_means_count: u8,
    pub light_theme_threshold: f32,
    pub run_in_parallel: bool,
    pub light: ThemeConfig,
    pub dark: ThemeConfig,
}

impl Default for SchemeBuilder {
    fn default() -> Self {
        Self {
            k_means_count: 255,
            light_theme_threshold: 0.55,
            run_in_parallel: false,
            light: ThemeConfig::default_light(),
            dark: ThemeConfig::default_dark(),
        }
    }
}

impl SchemeBuilder {
    pub fn generate_from_image(
        &self,
        img: ImageRef<'_, Srgb<u8>>,
    ) -> Result<ThaimeleonScheme, GenerationError> {
        #[cfg(feature = "parallel")]
        let img_profile = if self.run_in_parallel {
            crate::parse_image::par_parse_image(img, self.k_means_count).map_err(GenerationError::ParseImage)?
        } else {
            crate::parse_image::parse_image(img, self.k_means_count).map_err(GenerationError::ParseImage)?
        };
        #[cfg(not(feature = "parallel"))]
        let img_profile = crate::parse_image::parse_image(img, self.k_means_count).map_err(GenerationError::ParseImage)?;
        println!("Clustered colors sucessfully.");
        self.generate_from_slice(&img_profile)
    }

    pub fn generate_from_slice(
        &self,
        profile: &[ColorProfile],
    ) -> Result<ThaimeleonScheme, GenerationError> {
        let mean_color = NeutralColorBuilder::default()
            .generate(profile)
            .map_err(GenerationError::NeutralColorGeneration)?;
        println!("Mean lightness: {}", mean_color.lightness);
        let is_light_theme = mean_color.lightness > self.light_theme_threshold;
        let theme_config = if is_light_theme {
            self.light
        } else {
            self.dark
        };
        println!("mean hue {}", mean_color.get_hue().into_positive_degrees());
        println!("mean chroma {}", mean_color.chroma());
        let accent_hues: [OklabHue; N_ACCENTS] = AccentHueConfigBuilder::default()
            .chroma_weight(theme_config.chroma_weight_priority)
            .penalty_weight(theme_config.penalty_weight_priority)
            .miniumum_degrees(theme_config.minimum_hue_angle)
            .prefered_degrees(theme_config.prefered_hue_angle)
            .generate(profile, mean_color.get_hue(), N_ACCENTS)?
            .try_into()
            .map_err(|_| GenerationError::WrongAccentHueCount)?;
        let named_accent_hues = NamedHues::generate_named_accents_hues(&accent_hues);
        println!("Light mode: {}", is_light_theme);
        SchemeBuilderWithHints {
            light_theme_threshold: self.light_theme_threshold,
            run_in_parallel: self.run_in_parallel,
            is_light_theme,
            theme_config,
            profile,
            dominant_color: mean_color,
            accent_hues,
            named_accent_hues,
        }
        .generate()
    }
}

// The alternative of this struct is a giant function or functions with 6+ parameters with a lot of boiler plate variables
/// A context struct builds a color scheme with hints on accent hues placement
struct SchemeBuilderWithHints<'a> {
    light_theme_threshold: f32,
    run_in_parallel: bool,
    is_light_theme: bool,
    theme_config: ThemeConfig,
    profile: &'a [ColorProfile],
    dominant_color: Oklrab,
    accent_hues: [OklabHue; 6],
    named_accent_hues: NamedHues<OklabHue>,
}

impl<'a> SchemeBuilderWithHints<'a> {
    /// Generate Thaimeleon scheme based from img
    pub fn generate(&self) -> Result<ThaimeleonScheme, GenerationError> {
        // Online k-means with sampling
        let polarity = if self.is_light_theme { -1.0 } else { 1.0 };

        let base_lightness = map_range(
            if self.is_light_theme {
                (self.light_theme_threshold, 1.0)
            } else {
                (0.0, self.light_theme_threshold)
            },
            (
                self.theme_config.base_lightness_minimum,
                self.theme_config.base_lightness_maximum,
            ),
            self.dominant_color.lightness,
        );
        let set_2_lightness = base_lightness + polarity * self.theme_config.surface_distance * 4.0;
        let faint_lightness = contrast::lightness_from_contrast(
            base_lightness,
            Contrast::DPSContrast(self.theme_config.faint_dps_contrast),
            self.is_light_theme,
        );
        let set_3_lightness = contrast::lightness_from_contrast(
            base_lightness,
            Contrast::DPSContrast(self.theme_config.set_3_dps_contrast),
            self.is_light_theme,
        );
        let set_4_lightness = contrast::lightness_from_contrast(
            base_lightness,
            Contrast::DPSContrast(self.theme_config.set_4_dps_contrast),
            self.is_light_theme,
        );
        let set_5_lightness = contrast::lightness_from_contrast(
            base_lightness,
            Contrast::DPSContrast(self.theme_config.set_5_dps_contrast),
            self.is_light_theme,
        );

        let base_color = self
            .neutral_background_builder_template(base_lightness)
            .generate(self.profile)?;

        let surface_low_color = self.generate_surface_low(polarity, base_color)?;

        let text_color = self
            .neutral_foreground_builder_template(set_5_lightness)
            .generate(self.profile)?;

        // Used for black and white colors
        let (black_color, white_color) = if self.is_light_theme {
            (text_color, surface_low_color)
        } else {
            (surface_low_color, text_color)
        };

        let comparision_color = self
            .dominant_color
            .clamp_by_chroma_range(0.0, self.theme_config.maximum_accent_hue_center_translation);

        let (high_contrast_fg_accents, high_contrast_fg_named_accents) = self
            .generate_accent_colors(
                comparision_color,
                set_5_lightness,
                self.theme_config.high_contrast_fg_accent_radius_baseline,
            )?;

        let (fg_accents, fg_named_accents) = self.generate_accent_colors(
            comparision_color,
            set_4_lightness,
            self.theme_config.fg_accent_radius_baseline,
        )?;

        let (rg_accents, rg_named_accents) = self.generate_accent_colors(
            comparision_color,
            set_3_lightness,
            self.theme_config.rg_accent_radius_baseline,
        )?;

        let (bg_accents, bg_named_accents) = self.generate_accent_colors(
            comparision_color,
            set_2_lightness,
            self.theme_config.bg_accent_radius_baseline,
        )?;

        // Generate the rest of the schmee
        Ok(ThaimeleonScheme {
            is_light_theme: self.is_light_theme,
            surface_low: surface_low_color.into_rgb(),
            base: base_color.into_rgb(),
            base_high: self
                .neutral_background_builder_template(
                    base_lightness + polarity * self.theme_config.surface_distance / 2.0,
                )
                .generate(self.profile)?
                .into_rgb(),
            surface_high: self
                .neutral_background_builder_template(
                    base_lightness + polarity * self.theme_config.surface_distance,
                )
                .generate(self.profile)?
                .into_rgb(),
            surface_higher: self
                .neutral_background_builder_template(
                    base_lightness + polarity * self.theme_config.surface_distance * 2.0,
                )
                .generate(self.profile)?
                .into_rgb(),
            surface_highest: self
                .neutral_background_builder_template(
                    base_lightness + polarity * self.theme_config.surface_distance * 3.0,
                )
                .generate(self.profile)?
                .into_rgb(),
            faint: self
                .neutral_foreground_builder_template(faint_lightness)
                .generate(self.profile)?
                .into_rgb(),
            muted: self
                .neutral_foreground_builder_template(set_3_lightness)
                .generate(self.profile)?
                .into_rgb(),
            subtext: self
                .neutral_foreground_builder_template(set_4_lightness)
                .generate(self.profile)?
                .into_rgb(),
            text: text_color.into_rgb(),
            high_contrast_fg_accents,
            fg_accents,
            rg_accents,
            bg_accents,
            high_contrast_fg_named_accents,
            fg_named_accents,
            rg_named_accents,
            bg_named_accents,
            black: black_color.into_rgb(),
            white: white_color.into_rgb(),
        })
    }

    /// Generate the surface low color based polarity (light theme is -1, dark theme is 1) and base color
    fn generate_surface_low(
        &self,
        polarity: f32,
        base_color: Oklrab,
    ) -> Result<Oklrab, GenerationError> {
        let pre_distance_gamut_map = self
            .neutral_background_builder_template(
                base_color.lightness - polarity * self.theme_config.surface_distance * 0.5,
            )
            .srgb_clamp(false)
            .chroma_max(None)
            .generate(self.profile)?;
        pre_distance_gamut_map
            .gamut_map_with_difference(base_color, 2.0, self.theme_config.bg_neutral_chroma_builder)
            .ok_or(GenerationError::NoRoomForLowSurface)
    }

    fn neutral_builder_template(
        &self,
        lightness: f32,
        chroma_max: ChromaBuilder,
    ) -> NeutralColorBuilder {
        NeutralColorBuilder::default()
            .lightness_weight(self.theme_config.neutral_chroma_blend)
            .lightness_center(lightness)
            .force_lightness(true)
            .chroma_max(Some(chroma_max))
            .srgb_clamp(true)
    }

    fn neutral_background_builder_template(&self, lightness: f32) -> NeutralColorBuilder {
        self.neutral_builder_template(lightness, self.theme_config.bg_neutral_chroma_builder)
    }

    fn neutral_foreground_builder_template(&self, lightness: f32) -> NeutralColorBuilder {
        self.neutral_builder_template(lightness, self.theme_config.fg_neutral_chroma_builder)
    }

    fn generate_accent_colors(
        &self,
        comparision_color: Oklrab,
        lightness: f32,
        radius_baseline: f32,
    ) -> Result<([Srgb<u8>; 6], NamedHues<Srgb<u8>>), GenerationError> {
        let base_radius = filter_and_average_distances(
            self.profile,
            radius_baseline,
            Ab::from(comparision_color),
        );

        let accent_reference_color = find_reference_color(
            comparision_color.into(),
            lightness,
            base_radius,
            &self.accent_hues,
        );

        // Simple alternative to try_map which deals with error wells
        let mut accent_colors: [Srgb<u8>; N_ACCENTS] = [Srgb::default(); N_ACCENTS];
        for i in 0..accent_colors.len() {
            accent_colors[i] = accent_reference_color
                .transform_hue_by_comparsion(comparision_color, self.accent_hues[i])
                .ok_or(GenerationError::LowBaselineChroma)?
                .srgb_clamp()
                .into_rgb()
        }

        let named_accent_colors = self
            .named_accent_hues
            .try_map(|h| {
                accent_reference_color
                    .transform_hue_by_comparsion(comparision_color, h)
                    .ok_or(GenerationError::LowBaselineChroma)
            })?
            .ensure_chroma_of_hues(
                self.theme_config.red_chroma_minimum,
                self.theme_config.orange_chroma_minimum,
                self.theme_config.yellow_chroma_minimum,
                self.theme_config.green_chroma_minimum,
            )
            .map(|c| c.srgb_clamp().into_rgb());
        Ok((accent_colors, named_accent_colors))
    }
}

#[cfg(test)]
mod tests {
    use palette::OklabHue;

    use crate::scheme_builder::ChromaBuilder;

    /// Test chroma builder on different hues
    #[test]
    fn generate_high_end_chroma() {
        // Very unrealistic values
        let chroma_builder = ChromaBuilder {
            chroma_intercept: 1.0,
            lightness_to_chroma_slope: 1.0,
            hue_low_point: OklabHue::from_degrees(270.0),
            low_point_chroma_intercept: 0.0,
        };
        assert_eq!(
            chroma_builder.generate(0.0, OklabHue::from_degrees(270.0)),
            0.0
        );
        assert_eq!(
            chroma_builder.generate(0.0, OklabHue::from_degrees(90.0)),
            1.0
        );
    }
}
