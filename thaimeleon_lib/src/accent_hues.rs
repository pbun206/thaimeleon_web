use crate::{ab_profile::AbProfile};
use crate::cluster::*;
use crate::color_profile::ColorProfile;
use crate::hue_utils::*;
use derive_more::{Display, Error, From};
use ordered_float::OrderedFloat;
use palette::{OklabHue};

pub const DEFUALT_CHROMA_WEIGHT: f32 = 0.75;
pub const DEFUALT_N_HUES: usize = 6;
pub const DEFUALT_PENALTY_WEIGHT: f32 = 2.0;
pub const DEFAULT_MINIMUM_DEGREES: f32 = 45.0;
pub const DEFAULT_PREFERED_DEGREES: f32 = 50.0;

/// Errors raised while picking accent hues.
#[derive(Debug, Display, Error, From)]
#[non_exhaustive]
pub enum AccentHueBuilderError {
    #[display("no color profiles supplied to accent hue builder")]
    EmptyInput,
    #[display("clustering failed while preparing accent hue candidates: {_0}")]
    Cluster(ClusterError),
}

pub struct AccentHueConfigBuilder {
    pub chroma_weight: f32,
    pub penalty_weight: f32,
    pub miniumum_degrees: f32,
    pub prefered_degrees: f32,
}

impl AccentHueConfigBuilder {
    pub fn chroma_weight(&mut self, weight: f32) -> &mut Self {
        let new = self;
        new.chroma_weight = weight;
        new
    }
    pub fn penalty_weight(&mut self, weight: f32) -> &mut Self {
        let new = self;
        new.penalty_weight = weight;
        new
    }
    pub fn miniumum_degrees(&mut self, degrees: f32) -> &mut Self {
        let new = self;
        new.miniumum_degrees = degrees;
        new
    }
    pub fn prefered_degrees(&mut self, degrees: f32) -> &mut Self {
        let new = self;
        new.prefered_degrees = degrees;
        new
    }
    pub fn default() -> Self {
        Self {
            chroma_weight: DEFUALT_CHROMA_WEIGHT,
            penalty_weight: DEFUALT_PENALTY_WEIGHT,
            miniumum_degrees: DEFAULT_MINIMUM_DEGREES,
            prefered_degrees: DEFAULT_PREFERED_DEGREES,
        }
    }
    pub fn generate(
        &self,
        colors_to_be_parse: &[ColorProfile],
        hue_center: OklabHue,
        n_hues: usize,
    ) -> Result<Vec<OklabHue>, AccentHueBuilderError> {
        // First, we filter the color profiles of low chroma to remove some noise.
        // However, this can create the side product of no colors being filtered out.
        let colors = cluster_colors(colors_to_be_parse)?;

        // Filter out low chroma colors, that are about proximatiely 5th of a JND to actual monotone colors
        let filtered_colors: Vec<AbProfile> = colors.into_iter().filter(|ab_profile| ab_profile.ab.chroma() > 0.004).collect();

        AccentHueBuilder {
            chroma_weight: self.chroma_weight,
            penalty_weight: self.penalty_weight,
            miniumum_degrees: self.miniumum_degrees,
            prefered_degrees: self.prefered_degrees,
            colors: filtered_colors,
            n_hues,
            hue_center,
            accent_hues: vec![],
        }
        .generate()
    }
}

struct AccentHueBuilder {
    chroma_weight: f32,
    penalty_weight: f32,
    miniumum_degrees: f32,
    prefered_degrees: f32,
    n_hues: usize,
    hue_center: OklabHue,
    colors: Vec<AbProfile>,
    accent_hues: Vec<OklabHue>,
}

impl AccentHueBuilder {
    /// Create a default accent hue builder with set accent hues, mainly for testing purposes
    fn default_with_accent_hues(accent_hues: Vec<OklabHue>) -> Self {
        AccentHueBuilder {
            chroma_weight: DEFUALT_CHROMA_WEIGHT,
            penalty_weight: DEFUALT_PENALTY_WEIGHT,
            miniumum_degrees: DEFAULT_MINIMUM_DEGREES,
            prefered_degrees: DEFAULT_PREFERED_DEGREES,
            n_hues: DEFUALT_N_HUES,
            hue_center: OklabHue::default(),
            colors: vec![],
            accent_hues,
        }
    }

    /// Create a default accent hue builder with set color profiles, mainly for testing purposes
    fn default_with_color_profiles(color_profiles: Vec<AbProfile>) -> Self {
        AccentHueBuilder {
            chroma_weight: DEFUALT_CHROMA_WEIGHT,
            penalty_weight: DEFUALT_PENALTY_WEIGHT,
            miniumum_degrees: DEFAULT_MINIMUM_DEGREES,
            prefered_degrees: DEFAULT_PREFERED_DEGREES,
            n_hues: DEFUALT_N_HUES,
            hue_center: OklabHue::default(),
            colors: color_profiles,
            accent_hues: vec![],
        }
    }

    /// Pick an accent hue. If colors is empty, return none
    fn generate_accent_hue_scorewise(&self) -> Option<OklabHue> {
        Some(
            self.colors
                .iter()
                .max_by_key(|h| {
                    let base_score = self.generate_base_accent_score(**h);
                    let accent_hue_factor = self.generate_accent_hue_factor(**h);
                    OrderedFloat(base_score * accent_hue_factor)
                })?
                .ab
                .hue(),
        )
    }

    /// Create an accent hue factor based on the preexisting accent hue
    fn generate_accent_hue_factor(&self, h: AbProfile) -> f32 {
        (1.0 - self.accent_hues.iter().fold(0.0, |acc, &p| {
            let diff = hue_difference(h.ab.hue(), p);
            if diff < self.prefered_degrees {
                acc + (50.0 - diff) / self.prefered_degrees
            } else {
                acc
            }
        }))
        .max(0.0)
        .powf(self.penalty_weight)
    }

    /// Generaete base accent score
    fn generate_base_accent_score(&self, color: AbProfile) -> f32 {
        color.ab.chroma().powf(self.chroma_weight) * color.portion
    }

    /// Find primary hue from a vector of ABProfiles and a hue center. Returns none if colors is empty.
    fn generate_primary_hue(&self) -> Option<OklabHue> {
        let adjacent_colors = self
            .colors
            .iter()
            .filter(|c| self.prefered_degrees > hue_difference(c.ab.hue(), self.hue_center));
        match adjacent_colors.max_by_key(|&c| OrderedFloat(self.generate_base_accent_score(*c))) {
            Some(c) => Some(c.ab.hue()),
            // None means all colors are filtered out
            None => Some(
                self.colors
                    .iter()
                    .min_by_key(|c| OrderedFloat(hue_difference(c.ab.hue(), self.hue_center)))?
                    .ab
                    .hue(),
            ),
        }
    }

    /// Generate a number of accent hues
    pub fn generate(mut self) -> Result<Vec<OklabHue>, AccentHueBuilderError> {
        // Checks if the image is basically monotone. If it's empty, it should also run this.
        if self.colors.is_empty() {
            return Ok((0..self.n_hues)
                .map(|i| OklabHue::from_degrees(i as f32 * 360.0 / self.n_hues as f32))
                .collect());
        }

        // Primary color calculations. Outer is_empty check guarantees Some, but propagate
        // EmptyInput rather than panicking if the invariant is ever violated.
        let primary_hue = self
            .generate_primary_hue()
            .ok_or(AccentHueBuilderError::EmptyInput)?;

        self.accent_hues.push(primary_hue);

        // Generate hues based on accent hue scores
        while self.accent_hues.len() < self.n_hues
            && let Some(new_hue) = self.generate_accent_hue_scorewise_with_gaps()
        {
            self.accent_hues.push(new_hue);
        }

        // Generate hues based on closest to hue center
        while self.accent_hues.len() < self.n_hues {
            match self.generate_accent_hue_centerwise() {
                Some(hue) => self.accent_hues.push(hue),
                None => break,
            }
        }

        // Generate hues based on largest gaps
        while self.accent_hues.len() < self.n_hues {
            let mut sorted_hues = self.accent_hues.clone();
            let (index_with_largest_gap, largest_gap) = find_largest_gap(&mut sorted_hues);
            self.accent_hues.push(OklabHue::from_degrees(
                sorted_hues[index_with_largest_gap].into_raw_degrees() + largest_gap / 2.0,
            ))
        }
        Ok(self.accent_hues)
    }

    fn generate_accent_hue_scorewise_with_gaps(&mut self) -> Option<OklabHue> {
        let mut new_hue = self.generate_accent_hue_scorewise().unwrap();
        let mut has_moved = false;
        for hue in self.accent_hues.clone() {
            if hue_difference(hue, new_hue) < self.miniumum_degrees {
                if has_moved {
                    // We want to move to the next hue selection method when faced with this because
                    // this means that the color is squished between two other accent hues
                    return None;
                } else {
                    // Check which side are the hues
                    if hue_difference_directional(hue, new_hue) < 180.0 {
                        new_hue =
                            OklabHue::from_degrees(hue.into_degrees() + self.miniumum_degrees);
                    } else {
                        new_hue =
                            OklabHue::from_degrees(hue.into_degrees() - self.miniumum_degrees);
                    }
                    has_moved = true;
                }
            }
        }
        Some(new_hue)
    }

    /// Generate an accent hue that is closest to the cluster center without breaking the min degree rules.
    fn generate_accent_hue_centerwise(&mut self) -> Option<OklabHue> {
        //the idea here is that we collect a list of possible hues and choose the best of them
        let mut possible_hues: Vec<OklabHue> = Vec::with_capacity(3);
        let mut sorted_hues = self.accent_hues.clone();
        sorted_hues.sort_by_key(|c| OrderedFloat(c.into_positive_degrees()));
        let mut iter = sorted_hues.iter().peekable();

        while let Some(&hue) = iter.next() {
            let next_hue = match iter.peek() {
                Some(x) => **x,
                None => sorted_hues[0],
            };

            // Checks if there is room to fit a new hue
            if hue_difference_directional(hue, next_hue) > self.miniumum_degrees * 2.0
                || self.accent_hues.len() == 1
            {
                possible_hues.push(OklabHue::from_degrees(
                    hue.into_degrees() + self.miniumum_degrees,
                ));
                possible_hues.push(OklabHue::from_degrees(
                    next_hue.into_degrees() - self.miniumum_degrees,
                ));
                // Check if the hue center is actually between the two hues
                if is_between_hues(hue, self.hue_center, next_hue) {
                    // Checks if there is enough room for hue center
                    if hue_difference_directional(hue, self.hue_center) >= self.miniumum_degrees
                        && hue_difference_directional(self.hue_center, next_hue)
                            >= self.miniumum_degrees
                    {
                        possible_hues.push(self.hue_center);
                    }
                }
            }
        }
        possible_hues
            .into_iter()
            .min_by_key(|hue| OrderedFloat(hue_difference(self.hue_center, *hue)))
    }
}

#[cfg(test)]
mod tests {
    use palette::{Oklch, convert::IntoColorUnclamped};

    use crate::ab::Ab;

    use super::*;

    #[test]
    fn generate_accent_hues_example() {
        let img_profile: Vec<ColorProfile> = vec![
            ColorProfile::new(Oklch::new(0.8, 0.2, 130.0).into_color_unclamped(), 0.3333),
            ColorProfile::new(Oklch::new(0.8, 0.1, 210.0).into_color_unclamped(), 0.3333),
            ColorProfile::new(Oklch::new(0.8, 0.02, 250.0).into_color_unclamped(), 0.3334),
        ];
        let accent_hues: [OklabHue; 6] = AccentHueConfigBuilder::default()
            .generate(&img_profile, OklabHue::from_degrees(50.0), 6)
            .unwrap()
            .try_into()
            .unwrap();

        // First value: find the most obvious accent
        // Second value: find the second most obvious accent + don't cluster with hue 250
        // Third value: find the third most obvious accent + restore hue angle min
        // Fourth value: pick the hue center
        // Fifth and sixth: pick accent hue nearby the hue center
        // Convert to positive degrees to prevent from weird things to happen
        assert_eq!(
            accent_hues.map(|h| h.into_positive_degrees()),
            [
                OklabHue::from_degrees(130.0),
                OklabHue::from_degrees(210.0),
                OklabHue::from_degrees(255.0),
                OklabHue::from_degrees(50.0),
                OklabHue::from_degrees(5.0),
                OklabHue::from_degrees(320.0),
            ]
            .map(|h| h.into_positive_degrees())
        );
    }

    /// This test is mainly to see if the function returns gaps as intended
    #[test]
    fn generate_accent_hue_scorewise_with_gaps_example() {
        // Numbers are unrealistic, but whatever
        let mut builder = AccentHueBuilder::default_with_color_profiles(vec![
            // 
            AbProfile::new(
                Ab::from_ch(0.5, OklabHue::new(90.0)),
                0.25,
            ),
            AbProfile::new(
                Ab::from_ch(0.6, OklabHue::new(180.0)),
                0.30,
            ),
            AbProfile::new(
                Ab::from_ch(0.3, OklabHue::new(95.0)),
                0.20,
            ),
            AbProfile::new(
                Ab::from_ch(0.4, OklabHue::new(85.0)),
                0.24,
            ),
            AbProfile::new(
                Ab::from_ch(0.3, OklabHue::new(87.0)),
                0.01,
            ),
        ]);
        // Get obvious first choice first
        let next = builder.generate_accent_hue_scorewise_with_gaps();
        assert!((next.unwrap().into_positive_degrees() - 180.0).abs() < 0.01 );
        builder.accent_hues.push(next.unwrap());
        // Get obvious second choice first
        let next = builder.generate_accent_hue_scorewise_with_gaps();
        assert!((next.unwrap().into_positive_degrees() - 90.0).abs() < 0.01);
        builder.accent_hues.push(next.unwrap());
        // Keep minimum degrees here
        let next = builder.generate_accent_hue_scorewise_with_gaps();
        assert!((next.unwrap().into_positive_degrees() - 45.0).abs() < 0.01);
        builder.accent_hues.push(next.unwrap());
        // Keep minimum degrees here again
        let next = builder.generate_accent_hue_scorewise_with_gaps();
        assert!((next.unwrap().into_positive_degrees() - 135.0).abs() < 0.01);
        builder.accent_hues.push(next.unwrap());
        // Return none!
        let next = builder.generate_accent_hue_scorewise_with_gaps();
        assert_eq!(next, None);
    }
}
