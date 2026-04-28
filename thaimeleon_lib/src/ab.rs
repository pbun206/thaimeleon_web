use std::ops::{Add, Div, Mul};

use palette::{Oklab, OklabHue};

use crate::oklrab::Oklrab;

/// OKLAB without L, with OK removed because IMO OKAB sounds weird
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Ab {
    pub a: f32,
    pub b: f32,
}

impl Ab {
    pub fn new(a: f32, b: f32) -> Self {
        Self { a, b }
    }
    /// Create new Ab from chroma and hue
    pub fn from_ch(chroma: f32, hue: OklabHue) -> Self {
        Self::new(chroma * hue.into_radians().cos(), chroma * hue.into_radians().sin())
    }
    pub fn hue(&self) -> OklabHue {
        OklabHue::from_cartesian(self.a, self.b)
    }
    pub fn chroma(&self) -> f32 {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }
    pub fn square_chroma(&self) -> Self {
        Self::new(
            self.a.powi(2) / self.hue().into_radians().cos(),
            self.b.powi(2) / self.hue().into_radians().cos(),
        )
    }
    // I forgot what this was for
    pub fn custom_root_chroma(&self) -> Self {
        let hue_cos = self.hue().into_radians().cos();
        let hue_sin = self.hue().into_radians().sin();
        Self::new(
            (self.a / hue_cos).powf(0.6) * hue_cos,
            (self.b / hue_sin).powf(0.6) * hue_sin,
        )
    }
    pub fn cube_chroma(&self) -> Self {
        Self::new(
            self.a.powi(5) / self.hue().into_radians().cos().powi(4),
            self.b.powi(5) / self.hue().into_radians().cos().powi(4),
        )
    }

    pub fn distance_squared(self, other: Ab) -> f32 {
        (self.a - other.a).powi(2) + (self.b - other.b).powi(2)
    }

    pub fn distance(self, other: Ab) -> f32 {
        ((self.a - other.a).powi(2) + (self.b - other.b).powi(2)).sqrt()
    }

    pub fn hue_difference(&self, other: &Ab) -> f32 {
        crate::hue_utils::hue_difference(self.hue(), other.hue())
    }
}
impl Default for Ab {
    fn default() -> Self {
        Self { a: 0.0, b: 0.0 }
    }
}

impl Add for Ab {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl Div<f32> for Ab {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            a: self.a / rhs,
            b: self.b / rhs,
        }
    }
}

impl Mul<f32> for Ab {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}
impl From<&Oklab> for Ab {
    fn from(oklab: &Oklab) -> Self {
        Ab::new(oklab.a, oklab.b)
    }
}
impl From<Oklab> for Ab {
    fn from(oklab: Oklab) -> Self {
        Ab::new(oklab.a, oklab.b)
    }
}

impl From<Oklrab> for Ab {
    fn from(oklrab: Oklrab) -> Self {
        Ab::new(oklrab.a, oklrab.b)
    }
}

