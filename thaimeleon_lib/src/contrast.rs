use derive_more::{Display, Error};
use palette::convert::FromColorUnclamped;
use palette::{Lab, Oklab};

use crate::oklrab::Oklrab;

// /// ColorBuilder errors:
// /// Contrast too light -> Found a lightness value greater than 1.0
// /// Contrast too dark -> Found a lightness value lesser than 0.0
// #[derive(Debug, Display, Error)]
// #[non_exhaustive]
// pub enum ContrastError {
//     ContrastTooLight(),
//     ContrastTooDark(),
// }

/// Contrast formulas
pub enum Contrast {
    DPSContrast(f32),
    LStar(f32),
    LrDifference(f32),
}

/// Generate a lightness value in OKLrAB space which has a contrast value with a given OKLrAB lightness
pub fn lightness_from_contrast(
    lightness: f32,
    contrast: Contrast,
    is_new_color_darker: bool,
) -> f32 {
    let new_lightness = match contrast {
        Contrast::DPSContrast(val) => {
            // Since DPS uses CIELAB, convert OKLrAB lightness to CIELAB lightness
            let l_star = Lab::from_color_unclamped(Oklrab::new(lightness, 0.0, 0.0)).l;
            let new_l_star =
                dps_contrast::lightness_from_dps_contrast(l_star, val, is_new_color_darker);
            // Convert back
            Oklrab::from_color_unclamped(Lab::new(new_l_star, 0.0, 0.0)).lightness
        }
        Contrast::LStar(val) => {
            let l_star = Lab::from_color_unclamped(Oklab::new(lightness, 0.0, 0.0)).l;
            let new_l_star = if is_new_color_darker {
                l_star - val
            } else {
                l_star + val
            };
            // Convert back
            Oklrab::from_color_unclamped(Lab::new(new_l_star, 0.0, 0.0)).lightness
        }
        Contrast::LrDifference(val) => if is_new_color_darker {lightness - val} else {lightness + val},
    };
    new_lightness
}

/// All rights to the DPS formula are reserved to Andrew Somers. This dps_contrast module
/// is licensed under the DPS CONTRAST (DELTA PHI STAR) LICENSE, a modified version of the AGPL v3 license.
mod dps_contrast {
    /// Generate a lightness value in CIELAB space which has a contrast value with a given CIELAB lightness
    pub fn lightness_from_dps_contrast(
        lightness: f32,
        contrast: f32,
        is_new_color_darker: bool,
    ) -> f32 {
        if is_new_color_darker {
            (((40.0 + contrast) / 1.414).powf(1.0 / 0.618) * -1.0 + lightness.powf(1.618))
                .powf(1.0 / 1.618)
        } else {
            (((40.0 + contrast) / 1.414).powf(1.0 / 0.618) + lightness.powf(1.618))
                .powf(1.0 / 1.618)
        }
    }
}
