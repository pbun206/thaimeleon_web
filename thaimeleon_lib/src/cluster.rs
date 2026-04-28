use derive_more::{Display, Error, From};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

use crate::ab::Ab;
use crate::ab_profile::AbProfile;
use crate::color_profile::ColorProfile;
use crate::hue_utils::hue_difference;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy)]
struct KeyedIndex {
    index: usize,
    key: f32,
}

impl KeyedIndex {
    fn new(index: usize, key: f32) -> Self {
        Self { index, key }
    }
}

impl Ord for KeyedIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(OrderedFloat(self.key)).cmp(&Reverse(OrderedFloat(other.key)))
    }
}
impl Eq for KeyedIndex {}

impl PartialEq for KeyedIndex {
    fn eq(&self, other: &Self) -> bool {
        other.key == self.key
    }
}

impl PartialOrd for KeyedIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Reverse(self.key).partial_cmp(&Reverse(other.key))
    }
}

#[derive(Debug, Display, From, Error)]
#[non_exhaustive]
pub enum ClusterError {
    #[display("internal clustering error")]
    InternalError,
}


// TODO double check to see something stupid
// Based on Müllner 2011's "Modern hierarchical, agglomerative clustering algorithms", genric clustering algorithm variant
pub fn cluster_colors(colors_to_be_parse: &[ColorProfile]) -> Result<Vec<AbProfile>, ClusterError> {
    let mut clusters: Vec<Cluster> = colors_to_be_parse
        .iter()
        .map(|&c| Cluster::new(vec![AbProfile::from(&c)]))
        .collect();
    let n = clusters.len();
    let mut m = 0; //to track labels
    let mut s: Vec<usize> = (0..n).collect();
    // bc the skip one, everytime we call this vector, we gotta subtract by one
    let mut n_nghbr: Vec<usize> = s
        .clone()
        .iter()
        .skip(1)
        .map(|&p| {
            clusters
                .iter()
                .take(p)
                .position_min_by_key(|p2| {
                    OrderedFloat(clusters[p].custom_ward_linkage_dissimilarity(p2))
                })
                .expect(
                    "Clusters shouldn't be empty. There also could be two colors in the image too.",
                )
        })
        .collect();
    let mut q: BinaryHeap<KeyedIndex> = s
        .clone()
        .iter()
        .skip(1)
        .map(|&p| {
            KeyedIndex::new(
                p,
                clusters[p].custom_ward_linkage_dissimilarity(&clusters[n_nghbr[p - 1]]),
            )
        })
        .collect::<Vec<KeyedIndex>>()
        .into();
    while s.len() > 1 {
        let mut a = q.pop().expect("q should not be empty").index;
        let mut b = n_nghbr[a - 1];
        while s.iter().all(|&p| p != b) {
            match s.iter().take_while(|&&p| p < a).min_by_key(|&&p| {
                OrderedFloat(clusters[a].custom_ward_linkage_dissimilarity(&clusters[p]))
            }) {
                Some(x) => {
                    n_nghbr[a - 1] = *x;
                    q.push(KeyedIndex::new(
                        a,
                        clusters[a].custom_ward_linkage_dissimilarity(&clusters[n_nghbr[a - 1]]),
                    ));
                }
                None => {}
            }
            a = q.pop().expect("q should not be empty").index;
            b = n_nghbr[a - 1];
        }

        //Break condition based on experimentation with this clustering - higher the number -> more clustering
        if clusters[a].custom_ward_linkage_dissimilarity(&clusters[b]) > 0.0008 {
            break;
        }
        q.retain(|&p| p.index != b);
        s.retain(|&p| p != a && p != b);
        m += 1;
        let new_index = n - 1 + m;

        s.push(new_index);
        let new_cluster = clusters[a].merge(&clusters[b]);
        if s.len() == 2 {
            clusters.push(new_cluster);
            break;
        }

        n_nghbr.push(
            *s.iter()
                //should be all of them
                .take_while(|&&p| p < new_index)
                .min_by_key(|&&p| {
                    OrderedFloat(clusters[p].custom_ward_linkage_dissimilarity(&new_cluster))
                })
                .unwrap(),
        );
        q.push(KeyedIndex::new(
            new_index,
            clusters[n_nghbr[new_index - 1]].custom_ward_linkage_dissimilarity(&new_cluster),
        ));
        clusters.push(new_cluster);
    }
    Ok(s.iter().map(|&p| (&clusters[p]).into()).collect())
}

#[derive(Clone)]
struct Cluster {
    pub points: Vec<AbProfile>,
}

impl Cluster {
    pub fn new(points: Vec<AbProfile>) -> Self {
        Self { points }
    }
    pub fn portion(&self) -> f32 {
        self.points.iter().fold(0.0, |acc, p| acc + p.portion)
    }
    pub fn get_custom_rooted_chroma_centriod(&self) -> Ab {
        self.points.iter().fold(Ab::new(0.0, 0.0), |acc, p| {
            acc + p.ab.custom_root_chroma() * p.portion
        }) / self.portion()
    }
    pub fn get_centriod(&self) -> Ab {
        self.points
            .iter()
            .fold(Ab::new(0.0, 0.0), |acc, p| acc + p.ab * p.portion)
            / self.portion()
    }

    // pub fn ward_linkage_dissimilarity(&self, other: &Self) -> f32 {
    //     let centriod_a = self.get_centriod();
    //     let centriod_b = other.get_centriod();
    //     if centriod_a.hue_difference(&centriod_b) > 60.0 {
    //         std::f32::INFINITY
    //     } else {
    //         centriod_a.distance_squared(centriod_b) * (self.portion() * other.portion())
    //             / (self.portion() + other.portion())
    //     }
    // }
    // 
    pub fn custom_ward_linkage_dissimilarity(&self, other: &Self) -> f32 {
        let centriod_a = self.get_custom_rooted_chroma_centriod();
        let centriod_b = other.get_custom_rooted_chroma_centriod();
        let max_hue_difference = self
            .points
            .iter()
            .map(|p| {
                other
                    .points
                    .iter()
                    .map(|q| OrderedFloat(hue_difference(q.ab.hue(), p.ab.hue())))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        if max_hue_difference > OrderedFloat(90.0) || centriod_a.hue_difference(&centriod_b) > 60.0
        {
            std::f32::INFINITY
        } else {
            centriod_a.distance_squared(centriod_b) * (self.portion() * other.portion())
                / (self.portion() + other.portion())
        }
    }
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            points: vec![self.points.clone(), other.points.clone()].concat(),
        }
    }
}

impl From<&Cluster> for AbProfile {
    fn from(cluster: &Cluster) -> Self {
        Self::new(cluster.get_centriod(), cluster.portion())
    }
}
impl Default for Cluster {
    fn default() -> Self {
        Self { points: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use palette::{OklabHue, Oklch, convert::IntoColorUnclamped};

    use crate::hue_utils::hue_difference;

    use super::*;

    #[test]
    fn unclusters_low_frequencies() {
        //! Tests if clustering is good by seeing if it doesn't cluster at all.
        //! This test should pass in release.
        let img_profile: Vec<ColorProfile> = vec![
            ColorProfile::new(
                Oklch::new(1.0, 0.0, 0.0).into_color_unclamped(),
                101.0 / 144.0,
            ),
            ColorProfile::new(
                Oklch::new(0.9385, 0.027, 255.0).into_color_unclamped(),
                2.0 / 144.0,
            ),
            ColorProfile::new(
                Oklch::new(0.9385, 0.027, 345.0).into_color_unclamped(),
                2.0 / 144.0,
            ),
            ColorProfile::new(
                Oklch::new(0.9385, 0.027, 75.0).into_color_unclamped(),
                38.0 / 144.0,
            ),
            ColorProfile::new(
                Oklch::new(0.9385, 0.027, 165.0).into_color_unclamped(),
                1.0 / 144.0,
            ),
        ];

        let clustered_colors = cluster_colors(&img_profile).unwrap();
        // Use h instead of c
        for c in &clustered_colors {
            println!("Found hue of {}", c.ab.hue().into_positive_degrees());
        }
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(75.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(255.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(345.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(165.0)) < 0.001)
        );
    }

    #[test]
    fn unclusters_hue_differences() {
        //! Tests if clustering is good by seeing if it doesn't cluster at all.
        //! This test should pass in release.
        let img_profile: Vec<ColorProfile> = vec![
            ColorProfile::new(Oklch::new(0.8, 0.2, 130.0).into_color_unclamped(), 0.25),
            ColorProfile::new(Oklch::new(0.8, 0.1, 210.0).into_color_unclamped(), 0.5),
            ColorProfile::new(Oklch::new(0.8, 0.1, 255.0).into_color_unclamped(), 0.25),
        ];

        let clustered_colors = cluster_colors(&img_profile).unwrap();
        for c in &clustered_colors {
            println!("Found hue of {}", c.ab.hue().into_positive_degrees());
        }
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(130.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(210.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(255.0)) < 0.001)
        );
    }

    #[test]
    fn clusters_low_hue_differences() {
        //! Tests if clustering is good by seeing if it clusters
        //! This test should pass in release.
        let img_profile: Vec<ColorProfile> = vec![
            ColorProfile::new(Oklch::new(0.8, 0.2, 130.0).into_color_unclamped(), 0.25),
            ColorProfile::new(Oklch::new(0.8, 0.1, 210.0).into_color_unclamped(), 0.65),
            ColorProfile::new(Oklch::new(0.8, 0.07, 230.0).into_color_unclamped(), 0.1),
        ];

        let clustered_colors = cluster_colors(&img_profile).unwrap();
        for c in &clustered_colors {
            println!("Found hue of {}", c.ab.hue().into_positive_degrees());
        }
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(130.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .all(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(210.0)) > 0.05)
        );
    }

    #[test]
    fn unclusters_high_chroma_differences() {
        //! Tests if clustering is good by seeing if it doesn't cluster colors with high and low chroma
        //! This test should pass in release.
        let img_profile: Vec<ColorProfile> = vec![
            ColorProfile::new(Oklch::new(0.8, 0.2, 130.0).into_color_unclamped(), 0.25),
            ColorProfile::new(Oklch::new(0.8, 0.02, 210.0).into_color_unclamped(), 0.615),
            ColorProfile::new(Oklch::new(0.8, 0.10, 210.0).into_color_unclamped(), 0.035),
        ];

        let clustered_colors = cluster_colors(&img_profile).unwrap();
        for c in &clustered_colors {
            println!("Found hue of {}", c.ab.hue().into_positive_degrees());
        }
        assert!(
            clustered_colors
                .iter()
                .any(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(130.0)) < 0.001)
        );
        assert!(
            clustered_colors
                .iter()
                .filter(|c| hue_difference(c.ab.hue(), OklabHue::from_degrees(210.0)) < 0.001)
                .count()
                == 2
        );
    }
}
