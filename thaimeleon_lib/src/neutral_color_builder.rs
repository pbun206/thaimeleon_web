use crate::{
    color_profile::ColorProfile,
    color_traits::{GetChroma, SrgbClamp, WithChroma},
    oklrab::Oklrab,
    scheme_builder::ChromaBuilder,
};
use derive_more::{Display, Error, From};
use palette::{
    GetHue,
    color_difference::{EuclideanDistance, HyAb},
};
#[cfg(feature = "parallel")]
#[allow(unused_imports)]
use rayon::prelude::*;

/// Errors raised while computing a neutral color.
#[derive(PartialEq, Debug, Display, Error, From)]
#[non_exhaustive]
pub enum NeutralColorBuilderError {
    #[display("lightness center was not configured on the neutral color builder")]
    NoLightnessCenterSet,
    #[display("no color profiles supplied to neutral color builder")]
    EmptyInput,
    #[display("neutral color builder failed to converge")]
    Other,
}

/// Builder struct to generate a neutral color based on a vector of color profiles.
#[derive(Clone, Copy, PartialEq)]
pub struct NeutralColorBuilder {
    lightness_weight: f32,
    lightness_center: Option<f32>,
    force_lightness: bool,
    chroma_builder_max: Option<ChromaBuilder>,
    srgb_clamp: bool,
}

impl Default for NeutralColorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl NeutralColorBuilder {
    pub fn new() -> Self {
        Self {
            lightness_weight: 1.0,
            lightness_center: None,
            force_lightness: false,
            chroma_builder_max: None,
            srgb_clamp: false,
        }
    }

    // TODO make this better...
    /// Return new self with ideal lightness of the neutral color
    pub fn lightness_center(self, lightness: f32) -> Self {
        let mut new = self;
        new.lightness_center = Some(lightness);
        new
    }

    /// Return new self with weight of the ideal lightness of the neutral color
    pub fn lightness_weight(self, weight: f32) -> Self {
        let mut new = self;
        new.lightness_weight = weight;
        new
    }

    pub fn force_lightness(self, force_lightness: bool) -> Self {
        let mut new = self;
        new.force_lightness = force_lightness;
        new
    }

    /// Return new self with a chroma max for the final result
    pub fn chroma_max(self, chroma_builder: Option<ChromaBuilder>) -> Self {
        let mut new = self;
        new.chroma_builder_max = chroma_builder;
        new
    }

    /// Return new self with Srgb clamp set.
    pub fn srgb_clamp(self, srgb_clamp: bool) -> Self {
        let mut new = self;
        new.srgb_clamp = srgb_clamp;
        new
    }

    /// Generate neutral color in OKLrAB based on NeutralColorBuilder fields and series of color profiles
    pub fn generate(&self, colors: &[ColorProfile]) -> Result<Oklrab, NeutralColorBuilderError> {
        if colors.is_empty() {
            return Err(NeutralColorBuilderError::EmptyInput);
        }
        let (acc, weight_acc) = if let Some(lightness_center) = self.lightness_center {
            let center_color = Oklrab::new(lightness_center, 0.0, 0.0);
            colors
                .iter()
                .fold((Oklrab::new(0.0, 0.0, 0.0), 0.0), |mut acc, c| {
                    // If two colors are basically the same, we add a min of 0.005 so it doesn't straight out give zero weight
                    let weight = ((c.oklrab.distance(center_color))
                        .min(0.005)
                        .powf(-self.lightness_weight))
                        * c.portion;
                    // Add to weighted acc
                    acc.1 += weight;
                    // Add to acc
                    acc.0.lightness += c.oklrab.lightness * weight;
                    acc.0.a += c.oklrab.a * weight;
                    acc.0.b += c.oklrab.b * weight;
                    acc
                })
        } else {
            colors
                .iter()
                .fold((Oklrab::new(0.0, 0.0, 0.0), 1.0), |mut acc, c| {
                    // Add to acc
                    acc.0.lightness += c.oklrab.lightness * c.portion;
                    acc.0.a += c.oklrab.a * c.portion;
                    acc.0.b += c.oklrab.b * c.portion;
                    acc
                })
        };
        if weight_acc == 0.0 {
            return Err(NeutralColorBuilderError::Other);
        }
        let mut color = Oklrab::new(
            acc.lightness / weight_acc,
            acc.a / weight_acc,
            acc.b / weight_acc,
        );
        if self.force_lightness {
            color.lightness = self
                .lightness_center
                .ok_or(NeutralColorBuilderError::NoLightnessCenterSet)?;
        }
        if let Some(chroma_builder_max) = self.chroma_builder_max
            && let chroma_max = chroma_builder_max.generate(color.lightness, color.get_hue())
            && color.chroma() > chroma_max
        {
            color = color.with_chroma(chroma_max);
        }
        if self.srgb_clamp {
            color = color.srgb_clamp();
        }
        Ok(color)
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        color_profile::ColorProfile,
        neutral_color_builder::{NeutralColorBuilder, NeutralColorBuilderError},
        oklrab::Oklrab,
        scheme_builder::ChromaBuilder,
    };

    #[test]
    fn basic_neutral_color_builder_empty() {
        let colors = vec![];
        let result = NeutralColorBuilder::default().generate(&colors);
        assert_eq!(result, Err(NeutralColorBuilderError::EmptyInput));
    }

    #[test]
    fn basic_neutral_color_builder_averages() {
        let colors = vec![
            ColorProfile::new(Oklrab::new(0.5, 0.0, 0.0), 0.20),
            ColorProfile::new(Oklrab::new(0.25, 0.0, 0.0), 0.20),
            ColorProfile::new(Oklrab::new(0.75, 0.0, 0.0), 0.20),
            ColorProfile::new(Oklrab::new(1.0, 0.0, 0.0), 0.20),
            ColorProfile::new(Oklrab::new(0.0, 0.0, 0.0), 0.20),
        ];
        let result = NeutralColorBuilder::default().generate(&colors);
        assert_eq!(result, Ok(Oklrab::new(0.5, 0.0, 0.0)));
    }

    #[test]
    fn basic_neutral_color_builder_with_set_lightness_and_max_chroma() {
        let colors = vec![ColorProfile::new(Oklrab::new(0.25, 0.5, 0.0), 0.20)];
        let result = NeutralColorBuilder::default()
            .lightness_center(0.3)
            .chroma_max(Some(ChromaBuilder::new_constant(0.4)))
            .force_lightness(true)
            .generate(&colors);
        assert_eq!(result, Ok(Oklrab::new(0.3, 0.4, 0.0)));
    }

    #[test]
    fn basic_neutral_color_builder_with_srgb_clamping() {
        let colors = vec![ColorProfile::new(Oklrab::new(1.0, 0.5, 0.5), 1.0)];
        let result = NeutralColorBuilder::default()
            .srgb_clamp(true)
            .generate(&colors);
        assert_eq!(result, Ok(Oklrab::new(1.0, 0.0, 0.0)));
    }
}
