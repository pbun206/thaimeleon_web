use palette::{
    Lab, Oklab, OklabHue,
    convert::{FromColorUnclamped, IntoColorUnclamped},
    white_point::D65,
};
use std::ops::{Add, Div, Mul};

use crate::oklrab::Oklrab;

#[derive(Debug, Clone, Copy)]
pub struct ColorProfile {
    pub oklrab: Oklrab<f32>,
    pub portion: f32,
}

impl ColorProfile {
    pub fn new(color: Oklrab, portion: f32) -> Self {
        Self {
            oklrab: color,
            portion,
        }
    }
    pub fn chroma(&self) -> f32 {
        (self.oklrab.a.powi(2) + self.oklrab.b.powi(2)).sqrt()
    }
    pub fn hue(&self) -> OklabHue {
        OklabHue::from_cartesian(self.oklrab.a, self.oklrab.b)
    }
    pub fn squared_portion(&self) -> f32 {
        self.portion.sqrt()
    }
}

impl From<(Oklab, f32)> for ColorProfile {
    fn from(val: (Oklab, f32)) -> Self {
        Self {
            oklrab: val.0.into_color_unclamped(),
            portion: val.1,
        }
    }
}
