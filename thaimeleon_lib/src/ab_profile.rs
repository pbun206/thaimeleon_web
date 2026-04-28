use crate::{ab::Ab, color_profile::ColorProfile};

#[derive(Debug, Clone, Copy)]
pub struct AbProfile {
    pub ab: Ab,
    pub portion: f32,
}

impl AbProfile {
    pub fn new(ab: Ab, portion: f32) -> Self {
        Self { ab, portion }
    }
    pub fn squared_portion(&self) -> f32 {
        self.portion.sqrt()
    }
}

impl Default for AbProfile {
    fn default() -> Self {
        Self {
            ab: Ab::default(),
            portion: 0.0,
        }
    }
}

impl From<&ColorProfile> for AbProfile {
    fn from(color_profile: &ColorProfile) -> Self {
        Self::new(
            Ab::new(color_profile.oklrab.a, color_profile.oklrab.b),
            color_profile.portion,
        )
    }
}


