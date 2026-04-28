use crate::oklrab::Oklrab;
use crate::thaimeleon_scheme::NamedHues;
use crate::utils::*;
use palette::color_difference::EuclideanDistance;
use palette::convert::{FromColorUnclamped, IntoColorUnclamped};
use palette::{GetHue, Okhsl, Oklab, OklabHue, Oklch, Srgb};

pub trait GetChroma {
    type Scalar;
    /// Gets the chroma of a value
    fn chroma(&self) -> Self::Scalar;
}

pub trait WithChroma {
    type Scalar;
    /// Assigns new chroma value
    fn with_chroma(self, val: Self::Scalar) -> Self;
}

/// A trait to convert colors into RGB, unclamped.
pub trait IntoRgb {
    /// Convert color to Rgb
    fn into_rgb(self) -> Srgb<u8>;
}

impl IntoRgb for Oklab {
    fn into_rgb(self) -> Srgb<u8> {
        Srgb::from_color_unclamped(self).into_format::<u8>()
    }
}

impl IntoRgb for Oklch {
    fn into_rgb(self) -> Srgb<u8> {
        Srgb::from_color_unclamped(self).into_format::<u8>()
    }
}

impl IntoRgb for Oklrab {
    fn into_rgb(self) -> Srgb<u8> {
        Srgb::from_color_unclamped(self).into_format::<u8>()
    }
}

pub trait WithLightness {
    fn with_lightness(self, lightness: f32) -> Self;
}

impl WithLightness for Oklab {
    fn with_lightness(self, lightness: f32) -> Self {
        Oklab::new(lightness, self.a, self.b)
    }
}

impl WithLightness for Oklch {
    fn with_lightness(self, lightness: f32) -> Self {
        Oklch::new(lightness, self.chroma, self.hue)
    }
}

pub trait SetLightness {
    fn set_lightness(&mut self, lightness: f32);
}


pub trait SrgbClampAssign {
    fn srgb_clamp_assign(&mut self);
}

impl SrgbClampAssign for Oklrab {
    fn srgb_clamp_assign(&mut self) {
        let hue = self.get_hue();
        let min_chroma =
            Oklch::from_color_unclamped(Okhsl::new(self.get_hue(), 1.0, self.lightness))
                .chroma
                .min(self.chroma());
        self.a = min_chroma * hue.into_radians().cos();
        self.b = min_chroma * hue.into_radians().sin();
    }
}

pub trait SrgbClamp {
    fn srgb_clamp(&self) -> Self;
}

impl SrgbClamp for Oklrab {
    fn srgb_clamp(&self) -> Self {
        let hue = self.get_hue();
        let min_chroma =
            Oklch::from_color_unclamped(Okhsl::new(hue, 1.0, self.lightness))
                .chroma
                .min(self.chroma());
        Oklrab::new(
            self.lightness,
            min_chroma * hue.into_radians().cos(),
            min_chroma * hue.into_radians().sin(),
        )
    }
}

pub trait ClampByChromaRange {
    /// Clamps the color based on an arbitary chroma range
    fn clamp_by_chroma_range(&self, min: f32, max: f32) -> Self;
}

impl ClampByChromaRange for Oklrab {
    fn clamp_by_chroma_range(&self, min: f32, max: f32) -> Self {
        let hue = self.get_hue();
        let chroma = self.chroma().max(min).min(max);
        Oklrab::new(
            self.lightness,
            chroma * hue.into_radians().cos(),
            chroma * hue.into_radians().sin(),
        )
    }
}
#[cfg(test)]
mod tests {
    use palette::{Okhsl, OklabHue, convert::IntoColorUnclamped};

    use crate::{color_traits::SrgbClamp, oklrab::Oklrab};

    
    #[test]
    fn srgb_clamp_example() {
        assert_eq!(Oklrab::new(1.0, 0.4, 0.4).srgb_clamp(), Oklrab::new(1.0, 0.0, 0.0));
        assert_eq!(Oklrab::new(0.0, 0.4, 0.4).srgb_clamp(), Oklrab::new(0.0, 0.0, 0.0));
        assert_eq!(Oklrab::new(0.5, 0.0, 0.4).srgb_clamp(), Okhsl::new(OklabHue::new(90.0), 1.0, 0.5).into_color_unclamped());
    }
}
