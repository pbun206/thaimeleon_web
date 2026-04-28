use std::ops::{Add, Div, Mul, Sub};

use crate::{ab::Ab, color_profile::ColorProfile};

//This module is the protists of the rest of the modules.

// Source: https://rosettacode.org/wiki/Map_range#Rust
pub fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

// Filter color profiles by passing the baseline distance then takes the averages of distances
pub fn filter_and_average_distances(
    profiles: &[ColorProfile],
    baseline_distance: f32,
    comparsion_color: Ab,
) -> f32 {
    let (summed_ab, summed_weight) = profiles
        .iter()
        .map(|h| (Ab::from(h.oklrab).distance(comparsion_color), h.portion))
        .filter(|(h, _)| *h > baseline_distance)
        .fold((0.0, 0.0), |(sum, weight), (c, portion)| {
            (sum + portion * c, weight + portion)
        });

    if summed_weight == 0.0 {
        // Means everything was filtered
        baseline_distance
    } else {
        (summed_ab / summed_weight).max(baseline_distance)
    }
}

pub fn quadratic_root_positive(a: f32, b: f32, c: f32) -> Option<f32> {
    //! Only considers real solutions
    if b.powi(2) - 4.0 * a * c >= 0.0 {
        Some((-b + (b.powi(2) - 4.0 * a * c).sqrt()) / 2.0 / a)
    } else {
        None
    }
}
