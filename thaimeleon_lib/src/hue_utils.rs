use crate::ab::Ab;
use crate::oklrab::Oklrab;
use ordered_float::OrderedFloat;
use palette::convert::FromColorUnclamped;
use palette::{Okhsl, OklabHue};

/// Based on multiple hues, find a reference color with the same base radius and given OKLrAB lightness
pub fn find_reference_color(
    color_to_compare: Ab,
    lightness: f32,
    base_radius: f32,
    hues: &[OklabHue],
) -> Oklrab {
    let new_chroma = base_radius + color_to_compare.chroma();
    let ideal_ref_color = Oklrab::new(
        lightness,
        new_chroma * color_to_compare.hue().into_radians().cos(),
        new_chroma * color_to_compare.hue().into_radians().sin(),
    );

    // TODO better way to do once?
    std::iter::once(ideal_ref_color)
        .chain(
            // We use take here so unimportant accent hues don't get too much attention
            hues.iter().take(3)
                .map(|&h| Oklrab::from_color_unclamped(Okhsl::new(h, 1.0, lightness))),
        )
        .min_by_key(|&c| OrderedFloat(color_to_compare.distance((c).into())))
        .unwrap()
}

/// Find hue difference
pub fn hue_difference(x: OklabHue, y: OklabHue) -> f32 {
    let hue_difference = (x - y).into_positive_degrees().rem_euclid(360.0);
    if hue_difference > 180.0 {
        360.0 - hue_difference
    } else {
        hue_difference
    }
}

/// Find degree diffference but assuming y being on the right direction (in a number line)
pub fn hue_difference_directional(x: OklabHue, y: OklabHue) -> f32 {
    if y.into_positive_degrees() == x.into_positive_degrees() {
        0.0
    } else if y.into_positive_degrees() > x.into_positive_degrees() {
        y.into_positive_degrees() - x.into_positive_degrees()
    } else {
        y.into_positive_degrees() + 360.0 - x.into_positive_degrees()
    }
}
pub fn is_between_hues(min: OklabHue, test_hue: OklabHue, max: OklabHue) -> bool {
    hue_difference_directional(min, test_hue) <= hue_difference_directional(min, max)
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     #[test]
//     fn create_specified_accents_example() {
//         let accent_hues = vec![
//             OklabHue::from_degrees(10.0),
//             OklabHue::from_degrees(90.0),
//             OklabHue::from_degrees(120.0),
//             OklabHue::from_degrees(200.0),
//             OklabHue::from_degrees(250.0),
//             OklabHue::from_degrees(320.0),
//         ];
//         let reference_color = Oklch::new(0.8, 0.05, 0.0);
//         let comparsion_color = Oklch::new(0.8, 0.00, 0.0);
//         let output = create_specified_accents(
//             &accent_hues,
//             |h| {
//                 reference_color
//                     .transform_hue_by_comparsion(comparsion_color, h)
//                     .unwrap()
//                     .chroma_clip()
//             },
//             0.1,
//             0.09,
//             0.08,
//         );
//         assert_eq!(output[0].hue.into_positive_degrees(), 10.0);
//         assert_eq!(output[1].hue.into_positive_degrees(), 90.0);
//         assert_eq!(output[2].hue.into_positive_degrees(), 125.0);
//         assert_eq!(output[3].hue.into_positive_degrees(), 200.0);
//         assert_eq!(output[4].hue.into_positive_degrees(), 250.0);
//         assert_eq!(output[5].hue.into_positive_degrees(), 320.0);

//         assert_eq!(output[0].chroma, 0.1);
//         assert_eq!(output[1].chroma, 0.09);
//         assert_eq!(output[2].chroma, 0.08);
//     }
// }
// TODO: Make less expensive
/// Find the largest gap between a vector of hues
pub fn find_largest_gap(hues: &mut [OklabHue]) -> (usize, f32) {
    hues.sort_by_key(|c| OrderedFloat(c.into_positive_degrees()));
    let mut index_with_largest_gap = 0;
    let mut largest_gap_length = f32::NEG_INFINITY;
    for i in 0..hues.len() {
        // If all the elements are the same and it's the last element, give 360 degrees as the vector represents one circle
        let gap_length = if i == hues.len() - 1 && (hues[i] == hues[(i + 1) % hues.len()]) {
            360.0
        } else {
            hue_difference_directional(hues[i], hues[(i + 1) % hues.len()])
        };
        if gap_length >= largest_gap_length {
            largest_gap_length = gap_length;
            index_with_largest_gap = i;
        }
    }
    (index_with_largest_gap, largest_gap_length)
}

#[cfg(test)]
mod tests {
    use palette::OklabHue;

    use crate::hue_utils::find_largest_gap;

    #[test]
    fn find_largest_hue_of_one_example() {
        let mut hues = vec![OklabHue::new(0.0)];
        let (index_of_largest_hue_gap, size_of_largest_hue_gap) = find_largest_gap(&mut hues);
        assert_eq!(hues, vec![OklabHue::new(0.0)]);
        assert_eq!(index_of_largest_hue_gap, 0);
        assert_eq!(size_of_largest_hue_gap, 360.0);
    }

    #[test]
    fn find_largest_hue_of_two_equal_example() {
        let mut hues = vec![OklabHue::new(0.0), OklabHue::new(0.0)];
        let (index_of_largest_hue_gap, size_of_largest_hue_gap) = find_largest_gap(&mut hues);
        // Checks if sorted
        assert_eq!(hues, vec![OklabHue::new(0.0), OklabHue::new(0.0)]);
        assert_eq!(index_of_largest_hue_gap, 1);
        assert_eq!(size_of_largest_hue_gap, 360.0);
    }

    #[test]
    fn find_largest_hue_of_two_unequal_example() {
        let mut hues = vec![OklabHue::new(15.0), OklabHue::new(0.0)];
        let (index_of_largest_hue_gap, size_of_largest_hue_gap) = find_largest_gap(&mut hues);
        // Checks if sorted
        assert_eq!(hues, vec![OklabHue::new(0.0), OklabHue::new(15.0)]);
        assert_eq!(index_of_largest_hue_gap, 1);
        assert_eq!(size_of_largest_hue_gap, 345.0);
    }
}
