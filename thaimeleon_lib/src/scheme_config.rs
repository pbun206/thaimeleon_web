//! Serde-friendly partial config that maps into [`crate::scheme_builder::SchemeBuilder`].
//!
//! Every field is `Option<...>`; missing fields fall back to the per-theme defaults
//! ([`ThemeConfig::default_light`] / [`ThemeConfig::default_dark`]).

use palette::OklabHue;
use serde::{Deserialize, Serialize};

use crate::scheme_builder::{ChromaBuilder, SchemeBuilder, ThemeConfig};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ThemeConfigPartial {
    pub base_lightness_minimum: Option<f32>,
    pub base_lightness_maximum: Option<f32>,
    pub surface_distance: Option<f32>,

    pub set_2_lightness_correction: Option<f32>,
    pub faint_dps_contrast: Option<f32>,
    pub set_3_dps_contrast: Option<f32>,
    pub set_4_dps_contrast: Option<f32>,
    pub set_5_dps_contrast: Option<f32>,

    pub neutral_chroma_blend: Option<f32>,
    pub bg_neutral_chroma_intercept: Option<f32>,
    pub bg_neutral_lightness_to_chroma_slope: Option<f32>,
    pub bg_neutral_hue_low_point: Option<f32>,
    pub bg_neutral_low_point_chroma_intercept: Option<f32>,
    pub fg_neutral_chroma_intercept: Option<f32>,
    pub fg_neutral_lightness_to_chroma_slope: Option<f32>,
    pub fg_neutral_hue_low_point: Option<f32>,
    pub fg_neutral_low_point_chroma_intercept: Option<f32>,

    pub prefered_hue_angle: Option<f32>,
    pub minimum_hue_angle: Option<f32>,
    pub chroma_weight_priority: Option<f32>,
    pub penalty_weight_priority: Option<f32>,

    pub maximum_accent_hue_center_translation: Option<f32>,
    pub high_contrast_fg_accent_radius_baseline: Option<f32>,
    pub fg_accent_radius_baseline: Option<f32>,
    pub rg_accent_radius_baseline: Option<f32>,
    pub bg_accent_radius_baseline: Option<f32>,

    pub red_chroma_minimum: Option<f32>,
    pub orange_chroma_minimum: Option<f32>,
    pub yellow_chroma_minimum: Option<f32>,
    pub green_chroma_minimum: Option<f32>,
}

impl ThemeConfigPartial {
    fn fill(&self, default: ThemeConfig) -> ThemeConfig {
        ThemeConfig {
            base_lightness_minimum: self
                .base_lightness_minimum
                .unwrap_or(default.base_lightness_minimum),
            base_lightness_maximum: self
                .base_lightness_maximum
                .unwrap_or(default.base_lightness_maximum),
            surface_distance: self.surface_distance.unwrap_or(default.surface_distance),
            set_2_lightness_correction: self
                .set_2_lightness_correction
                .unwrap_or(default.set_2_lightness_correction),
            faint_dps_contrast: self
                .faint_dps_contrast
                .unwrap_or(default.faint_dps_contrast),
            set_3_dps_contrast: self
                .set_3_dps_contrast
                .unwrap_or(default.set_3_dps_contrast),
            set_4_dps_contrast: self
                .set_4_dps_contrast
                .unwrap_or(default.set_4_dps_contrast),
            set_5_dps_contrast: self
                .set_5_dps_contrast
                .unwrap_or(default.set_5_dps_contrast),
            neutral_chroma_blend: self
                .neutral_chroma_blend
                .unwrap_or(default.neutral_chroma_blend),
            bg_neutral_chroma_builder: ChromaBuilder {
                chroma_intercept: self
                    .bg_neutral_chroma_intercept
                    .unwrap_or(default.bg_neutral_chroma_builder.chroma_intercept),
                lightness_to_chroma_slope: self
                    .bg_neutral_lightness_to_chroma_slope
                    .unwrap_or(default.bg_neutral_chroma_builder.lightness_to_chroma_slope),
                low_point_chroma_intercept: self
                    .bg_neutral_low_point_chroma_intercept
                    .unwrap_or(default.bg_neutral_chroma_builder.low_point_chroma_intercept),
                hue_low_point: self
                    .bg_neutral_hue_low_point
                    .map(OklabHue::from_degrees)
                    .unwrap_or(default.bg_neutral_chroma_builder.hue_low_point),
            },
            fg_neutral_chroma_builder: ChromaBuilder {
                chroma_intercept: self
                    .fg_neutral_chroma_intercept
                    .unwrap_or(default.fg_neutral_chroma_builder.chroma_intercept),
                lightness_to_chroma_slope: self
                    .fg_neutral_lightness_to_chroma_slope
                    .unwrap_or(default.fg_neutral_chroma_builder.lightness_to_chroma_slope),
                low_point_chroma_intercept: self
                    .fg_neutral_low_point_chroma_intercept
                    .unwrap_or(default.fg_neutral_chroma_builder.low_point_chroma_intercept),
                hue_low_point: self
                    .fg_neutral_hue_low_point
                    .map(OklabHue::from_degrees)
                    .unwrap_or(default.fg_neutral_chroma_builder.hue_low_point),
            },
            prefered_hue_angle: self
                .prefered_hue_angle
                .unwrap_or(default.prefered_hue_angle),
            minimum_hue_angle: self.minimum_hue_angle.unwrap_or(default.minimum_hue_angle),
            chroma_weight_priority: self
                .chroma_weight_priority
                .unwrap_or(default.chroma_weight_priority),
            penalty_weight_priority: self
                .penalty_weight_priority
                .unwrap_or(default.penalty_weight_priority),
            maximum_accent_hue_center_translation: self
                .maximum_accent_hue_center_translation
                .unwrap_or(default.maximum_accent_hue_center_translation),
            high_contrast_fg_accent_radius_baseline: self
                .high_contrast_fg_accent_radius_baseline
                .unwrap_or(default.high_contrast_fg_accent_radius_baseline),
            fg_accent_radius_baseline: self
                .fg_accent_radius_baseline
                .unwrap_or(default.fg_accent_radius_baseline),
            rg_accent_radius_baseline: self
                .rg_accent_radius_baseline
                .unwrap_or(default.rg_accent_radius_baseline),
            bg_accent_radius_baseline: self
                .bg_accent_radius_baseline
                .unwrap_or(default.bg_accent_radius_baseline),
            red_chroma_minimum: self
                .red_chroma_minimum
                .unwrap_or(default.red_chroma_minimum),
            orange_chroma_minimum: self
                .orange_chroma_minimum
                .unwrap_or(default.orange_chroma_minimum),
            yellow_chroma_minimum: self
                .yellow_chroma_minimum
                .unwrap_or(default.yellow_chroma_minimum),
            green_chroma_minimum: self
                .green_chroma_minimum
                .unwrap_or(default.green_chroma_minimum),
        }
    }

    pub fn to_light_theme(&self) -> ThemeConfig {
        self.fill(ThemeConfig::default_light())
    }

    pub fn to_dark_theme(&self) -> ThemeConfig {
        self.fill(ThemeConfig::default_dark())
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FullConfigPartial {
    pub k_means_count: Option<u8>,
    pub light_theme_threshold: Option<f32>,
    #[serde(default)]
    pub light: ThemeConfigPartial,
    #[serde(default)]
    pub dark: ThemeConfigPartial,
}

impl FullConfigPartial {
    pub fn into_scheme_builder(self) -> SchemeBuilder {
        SchemeBuilder {
            k_means_count: self.k_means_count.unwrap_or(255),
            light_theme_threshold: self.light_theme_threshold.unwrap_or(0.55),
            run_in_parallel: false,
            light: self.light.to_light_theme(),
            dark: self.dark.to_dark_theme(),
        }
    }
}
