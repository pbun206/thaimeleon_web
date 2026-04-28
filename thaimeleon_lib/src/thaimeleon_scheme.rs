use ordered_float::OrderedFloat;
use palette::{OklabHue, Srgb};
use serde::{Deserialize, Serialize};

use crate::{
    color_traits::{GetChroma, WithChroma},
    hue_utils::{hue_difference, is_between_hues},
    oklrab::Oklrab,
};

/// Thaimeleon Scheme
#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct ThaimeleonScheme {
    pub is_light_theme: bool,
    pub surface_low: Srgb<u8>,
    pub base: Srgb<u8>,
    pub base_high: Srgb<u8>,
    pub surface_high: Srgb<u8>,
    pub surface_higher: Srgb<u8>,
    pub surface_highest: Srgb<u8>,
    pub muted: Srgb<u8>,
    pub subtext: Srgb<u8>,
    pub text: Srgb<u8>,
    pub high_contrast_fg_accents: [Srgb<u8>; 6],
    pub fg_accents: [Srgb<u8>; 6],
    pub rg_accents: [Srgb<u8>; 6],
    pub bg_accents: [Srgb<u8>; 6],
    pub high_contrast_fg_named_accents: NamedHues<Srgb<u8>>,
    pub fg_named_accents: NamedHues<Srgb<u8>>,
    pub rg_named_accents: NamedHues<Srgb<u8>>,
    pub bg_named_accents: NamedHues<Srgb<u8>>,
    pub black: Srgb<u8>,
    pub white: Srgb<u8>,
}

/// A struct to give each name hue a RGB color.
#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug, Default)]
pub struct NamedHues<T> {
    pub red: T,
    pub orange: T,
    pub yellow: T,
    pub cyan: T,
    pub green: T,
    pub blue: T,
    pub purple: T,
    pub magenta: T,
}

impl<T: Copy> NamedHues<T> {
    /// Shortcut to convert every value in Named Hues using iterator like map.
    pub fn map<B, F>(&self, mut f: F) -> NamedHues<B>
    where
        F: FnMut(T) -> B,
    {
        NamedHues {
            red: f(self.red),
            orange: f(self.orange),
            yellow: f(self.yellow),
            green: f(self.green),
            cyan: f(self.cyan),
            blue: f(self.blue),
            purple: f(self.purple),
            magenta: f(self.magenta),
        }
    }
    /// Shortcut to convert every value in Named Hues using iterator like try_map.
    pub fn try_map<B, F, E>(&self, mut f: F) -> Result<NamedHues<B>, E>
    where
        F: FnMut(T) -> Result<B, E>,
    {
        Ok(NamedHues {
            red: f(self.red)?,
            orange: f(self.orange)?,
            yellow: f(self.yellow)?,
            green: f(self.green)?,
            cyan: f(self.cyan)?,
            blue: f(self.blue)?,
            purple: f(self.purple)?,
            magenta: f(self.magenta)?,
        })
    }
}

impl NamedHues<OklabHue> {
    /// Create named accent hues based on preexisting accent hues
    pub fn generate_named_accents_hues(existing_accent_hues: &[OklabHue]) -> Self {
        let calculate_hue = |left: f32, compare_hue: f32, right: f32| {
            let mut closest_hue = *existing_accent_hues
                .iter()
                .min_by_key(|&&h| {
                    OrderedFloat(hue_difference(OklabHue::from_degrees(compare_hue), h))
                })
                .expect("Iter should be larger than one");

            // Check if hue is outside range
            let right_hue = OklabHue::from_degrees(right);
            let left_hue = OklabHue::from_degrees(left);

            if !is_between_hues(left_hue, closest_hue, right_hue) {
                if hue_difference(closest_hue, left_hue) < hue_difference(closest_hue, right_hue) {
                    closest_hue = left_hue;
                } else {
                    closest_hue = right_hue;
                }
            }
            closest_hue
        };

        // Compare hues are based from this website: https://idl.uw.edu/color-naming-in-different-languages/vis/full_color_maps.html
        // https://idl.uw.edu/color-naming-in-different-languages/vis/color-name-summaries.html
        // Max and mins are subjective arbitary values
        // TODO better way to determine min and maxs
        let red = calculate_hue(357.5, 25.0, 37.5);
        let orange = calculate_hue(37.5, 55.0, 57.5);
        let yellow = calculate_hue(57.5, 95.0, 105.0);
        let green = calculate_hue(125.0, 129.0, 155.0);
        let cyan = calculate_hue(175.0, 197.0, 220.0);
        let blue = calculate_hue(220.0, 260.0, 290.0);
        let purple = calculate_hue(310.0, 320.0, 337.5);
        let magenta = calculate_hue(310.0, 350.0, 350.0);

        Self {
            red,
            orange,
            yellow,
            green,
            cyan,
            blue,
            purple,
            magenta,
        }
    }
}

impl NamedHues<Oklrab<f32>> {
    pub(crate) fn ensure_chroma_of_hues(
        self,
        red_chroma_min: f32,
        orange_chroma_min: f32,
        yellow_chroma_min: f32,
        green_chroma_min: f32,
    ) -> Self {
        let mut new = self;
        new.red = new.red.with_chroma(new.red.chroma().max(red_chroma_min));
        new.orange = new
            .orange
            .with_chroma(new.orange.chroma().max(orange_chroma_min));
        new.yellow = new
            .yellow
            .with_chroma(new.yellow.chroma().max(yellow_chroma_min));
        new.green = new
            .green
            .with_chroma(new.green.chroma().max(green_chroma_min));
        new
    }
}

#[cfg(test)]
mod tests {
    use palette::{OklabHue, Oklch, convert::FromColorUnclamped};

    use crate::{color_traits::GetChroma, oklrab::Oklrab, thaimeleon_scheme::NamedHues};

    /// Based on one accent hue, generate best named accent hues
    #[test]
    fn generate_named_accent_hues() {
        let existing_accent_hues = vec![OklabHue::new(180.0)];
        let named_accent_hues = NamedHues::generate_named_accents_hues(&existing_accent_hues);
        assert_eq!(
            named_accent_hues,
            NamedHues {
                red: OklabHue::new(37.5),
                orange: OklabHue::new(57.5),
                yellow: OklabHue::new(105.0),
                green: OklabHue::new(155.0),
                cyan: OklabHue::new(180.0),
                blue: OklabHue::new(220.0),
                purple: OklabHue::new(310.0),
                magenta: OklabHue::new(310.0),
            }
        )
    }

    /// Tests ensure chroma method
    #[test]
    fn ensure_chromas() {
        let current_hues = NamedHues {
            red: OklabHue::new(37.5),
            orange: OklabHue::new(57.5),
            yellow: OklabHue::new(105.0),
            green: OklabHue::new(155.0),
            cyan: OklabHue::new(180.0),
            blue: OklabHue::new(220.0),
            purple: OklabHue::new(310.0),
            magenta: OklabHue::new(310.0),
        };
        let current_colors =
            current_hues.map(|h| Oklrab::from_color_unclamped(Oklch::new(0.5, 0.02, h)));
        let ensured_chroma_colors = current_colors.ensure_chroma_of_hues(0.05, 0.05, 0.04, 0.00);
        assert!(
            (ensured_chroma_colors.red.chroma() - 0.05).abs() < 0.005,
            "Red is incorrect"
        );
        assert!(
            (ensured_chroma_colors.orange.chroma() - 0.05).abs() < 0.005,
            "Orange is incorrect"
        );
        assert!(
            (ensured_chroma_colors.yellow.chroma() - 0.04).abs() < 0.005,
            "Yellow is incorrect"
        );
        assert!(
            (ensured_chroma_colors.green.chroma() - 0.02).abs() < 0.005,
            "Green is incorrect"
        );
    }
}
