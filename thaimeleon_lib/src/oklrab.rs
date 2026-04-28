use std::ops::{Add, Mul};

use palette::{
    GetHue, Lab, Okhsl, Oklab, OklabHue, Oklch, Srgb,
    color_difference::{Ciede2000, EuclideanDistance, HyAb},
    convert::{FromColorUnclamped, FromColorUnclampedMut, IntoColorUnclamped},
};

use crate::{
    color_traits::{GetChroma, SetLightness, SrgbClamp, WithChroma, WithLightness},
    scheme_builder::ChromaBuilder,
    utils::quadratic_root_positive,
};

// https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab
const K1: f32 = 0.206;
const K2: f32 = 0.03;
const K3: f32 = (1.0 + K1) / (1.0 + K2);
// For gamut mapping functions
const LIGHTNESS_STEP: f32 = 0.0005;

/// OKLAB but with Lr
#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Oklrab<T = f32> {
    pub lightness: T,
    pub a: T,
    pub b: T,
}

impl Oklrab {
    pub fn new(lightness: f32, a: f32, b: f32) -> Self {
        Self { lightness, a, b }
    }
}

impl Oklrab<f32> {
    // Checks if the color is fully saturated within sRGB gamut
    pub fn is_saturated(self) -> bool {
        let saturated_color = Okhsl::new(self.get_hue(), 1.0, self.lightness);
        let difference =
            Lab::from_color_unclamped(self).difference(saturated_color.into_color_unclamped());
        // Check if difference is under fifth of a JND
        difference < 0.04
    }

    /// Generate color with a new hue, perserving eucildean distance and lightness. When perservation is impossible, returns None
    pub(crate) fn transform_hue_by_comparsion(
        &self,
        other: Self,
        new_hue: OklabHue,
    ) -> Option<Self> {
        let new_chroma = quadratic_root_positive(
            1.0,
            -2.0 * (new_hue.into_radians().sin() * other.b
                + new_hue.into_radians().cos() * other.a),
            -(self.a.powi(2) + self.b.powi(2) + -2.0 * self.a * other.a + -2.0 * self.b * other.b),
        )?;

        if new_chroma >= 0.0 {
            Some(Self::new(
                self.lightness,
                new_chroma * new_hue.into_radians().cos(),
                new_chroma * new_hue.into_radians().sin(),
            ))
        } else {
            None
        }
    }

    // TODO
    pub fn difference_perserving_gamut_map(self, other: Self) -> Option<Self> {
        let other_lab = Lab::from_color_unclamped(other);
        let difference = Lab::from_color_unclamped(self).difference(other_lab);
        let mut new = self.srgb_clamp();
        if self.lightness > other.lightness {
            while Lab::from_color_unclamped(new).difference(other_lab) > difference {
                new.lightness -= LIGHTNESS_STEP;
                new = new.srgb_clamp();
                if new.lightness < other.lightness {
                    return None;
                }
            }
        } else {
            while Lab::from_color_unclamped(new).difference(other_lab) > difference {
                new.lightness += LIGHTNESS_STEP;
                new = new.srgb_clamp();
                if new.lightness > other.lightness {
                    return None;
                }
            }
        }
        Some(new)
    }

    // TODO
    pub fn gamut_map_with_difference(
        self,
        other: Self,
        difference: f32,
        chroma_max: ChromaBuilder,
    ) -> Option<Self> {
        let other_lab = Lab::from_color_unclamped(other);
        let mut new = self
            .with_chroma(
                self.chroma()
                    .min(chroma_max.generate(self.lightness, self.get_hue())),
            )
            .srgb_clamp();
        let intial_difference = Lab::from_color_unclamped(new).difference(other_lab);
        // For the case of Thaimeleon, this is usually true
        let needs_less_difference = intial_difference > difference;
        if (self.lightness > other.lightness) == needs_less_difference {
            while (Lab::from_color_unclamped(new).difference(other_lab) > difference)
                == needs_less_difference
            {
                new = self.with_lightness(new.lightness - LIGHTNESS_STEP);
                new = new
                    .with_chroma(
                        new.chroma()
                            .min(chroma_max.generate(new.lightness, new.get_hue())),
                    )
                    .srgb_clamp();
                if (new.lightness < other.lightness && needs_less_difference)
                    || (new.lightness < 0.0 && !needs_less_difference)
                {
                    return None;
                }
            }
        } else {
            while (Lab::from_color_unclamped(new).difference(other_lab) > difference)
                == needs_less_difference
            {
                new = self.with_lightness(new.lightness + LIGHTNESS_STEP);
                new = new
                    .with_chroma(
                        new.chroma()
                            .min(chroma_max.generate(new.lightness, new.get_hue())),
                    )
                    .srgb_clamp();
                if (new.lightness > other.lightness && needs_less_difference)
                    || (new.lightness > 1.0 && !needs_less_difference)
                {
                    return None;
                }
            }
        }
        Some(new)
    }
}

impl GetHue for Oklrab<f32> {
    type Hue = OklabHue;

    fn get_hue(&self) -> Self::Hue {
        Self::Hue::from_cartesian(self.a, self.b)
    }
}

impl GetChroma for Oklrab<f32> {
    type Scalar = f32;

    fn chroma(&self) -> f32 {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }
}

impl WithChroma for Oklrab<f32> {
    type Scalar = f32;

    fn with_chroma(self, val: Self::Scalar) -> Self {
        let hue = self.get_hue();

        Oklrab::new(
            self.lightness,
            val * hue.into_radians().cos(),
            val * hue.into_radians().sin(),
        )
    }
}

impl HyAb for Oklrab<f32> {
    type Scalar = f32;

    fn hybrid_distance(self, other: Self) -> Self::Scalar {
        (self.lightness - other.lightness).abs()
            + ((self.a - other.a).powi(2) + (self.b - other.b).powi(2)).sqrt()
    }
}

impl EuclideanDistance for Oklrab<f32> {
    type Scalar = f32;

    fn distance_squared(self, other: Self) -> Self::Scalar {
        (self.lightness - other.lightness).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2) 
    }
}

impl FromColorUnclamped<Oklab> for Oklrab {
    fn from_color_unclamped(val: Oklab) -> Self {
        // Find L to Lr
        let lr =
            (K3 * val.l - K1 + ((K3 * val.l - K1).powi(2) + 4.0 * K2 * K3 * val.l).sqrt()) / 2.0;
        Self::new(lr, val.a, val.b)
    }
}

impl FromColorUnclamped<Oklrab> for Oklab {
    fn from_color_unclamped(val: Oklrab) -> Self {
        // Find Lr to L
        let l = val.lightness * (val.lightness + K1) / (K3 * (val.lightness + K2));
        Oklab::new(l, val.a, val.b)
    }
}

// When converting from Lab to Oklrab, we use Oklab as the transition
impl FromColorUnclamped<Oklrab> for Lab {
    fn from_color_unclamped(val: Oklrab) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Lab> for Oklrab {
    fn from_color_unclamped(val: Lab) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Oklrab> for Srgb {
    fn from_color_unclamped(val: Oklrab) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Srgb> for Oklrab {
    fn from_color_unclamped(val: Srgb) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Oklrab> for Okhsl {
    fn from_color_unclamped(val: Oklrab) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Okhsl> for Oklrab {
    fn from_color_unclamped(val: Okhsl) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Oklrab> for Oklch {
    fn from_color_unclamped(val: Oklrab) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Oklch> for Oklrab {
    fn from_color_unclamped(val: Oklch) -> Self {
        Oklab::from_color_unclamped(val).into_color_unclamped()
    }
}

impl FromColorUnclamped<Srgb<u8>> for Oklrab {
    fn from_color_unclamped(val: Srgb<u8>) -> Self {
        let srgbf32: Srgb<f32> = val.into_format();
        Oklab::from_color_unclamped(srgbf32).into_color_unclamped()
    }
}

impl Mul<f32> for Oklrab {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.lightness * rhs, self.a * rhs, self.b * rhs)
    }
}

impl Add<Self> for Oklrab {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.lightness + rhs.lightness,
            self.a + rhs.a,
            self.b + rhs.b,
        )
    }
}

impl WithLightness for Oklrab {
    fn with_lightness(self, lightness: f32) -> Self {
        Oklrab::new(lightness, self.a, self.b)
    }
}

impl SetLightness for Oklrab {
    fn set_lightness(&mut self, lightness: f32) {
        self.lightness = lightness;
    }
}

#[cfg(test)]
mod tests {
    use palette::{
        FromColor, IntoColor, Lab, Okhsl, Oklab, OklabHue, Srgb,
        color_difference::Ciede2000,
        convert::{FromColorUnclamped, IntoColorUnclamped},
    };

    use crate::{color_traits::GetChroma, oklrab::Oklrab, scheme_builder::ChromaBuilder};

    #[test]
    fn gamut_map_with_difference_some_simple() {
        let color = Oklrab::new(0.95, 0.0, 0.0);
        let expected = Oklrab::new(0.975, 0.0, 0.0);
        let other = Oklrab::new(0.925, 0.0, 0.0);
        let difference = Lab::from_color(expected).difference(other.into_color_unclamped());
        let new_color =
            color.gamut_map_with_difference(other, difference, ChromaBuilder::new_constant(1.0));
        assert!((new_color.unwrap().lightness - expected.lightness) < 0.01);
    }

    #[test]
    fn gamut_map_with_chroma_with_difference_some_with_chroma() {
        let color = Oklrab::new(0.99, 0.00, 0.04);
        let expected = Oklrab::from_color_unclamped(Okhsl::new(OklabHue::new(90.0), 1.0, 0.975));
        let other = Oklrab::new(0.925, 0.0, 0.04);
        let difference = Lab::from_color(expected).difference(other.into_color_unclamped());
        // let difference = c;
        let new_color =
            color.gamut_map_with_difference(other, difference, ChromaBuilder::new_constant(1.0));
        println!("{:?}", new_color);
        assert!((new_color.unwrap().lightness - expected.lightness) < 0.01);
        assert!((new_color.unwrap().chroma() - expected.chroma()) < 0.01);
    }

    #[test]
    fn gamut_map_with_difference_none() {
        let color = Oklrab::new(0.95, 0.0, 0.0);
        let other = Oklrab::new(0.925, 0.0, 0.0);
        // 20 is a crazy number given the colors
        let new_color =
            color.gamut_map_with_difference(other, 20.0, ChromaBuilder::new_constant(1.0));
        assert_eq!(new_color, None);
    }

    #[test]
    fn oklab_into_okrlab_example() {
        // White
        let white: Oklab<f32> = Oklab::new(1.0, 0.0, 0.0);
        assert_eq!(
            Oklrab::from_color_unclamped(white),
            Oklrab::new(1.0, 0.0, 0.0)
        );

        // Black
        let black: Oklab<f32> = Oklab::new(0.0, 0.0, 0.0);
        assert_eq!(
            Oklrab::from_color_unclamped(black),
            Oklrab::new(0.0, 0.0, 0.0)
        );
    }
}
