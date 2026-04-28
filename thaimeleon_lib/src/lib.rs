//! A library for generating color schemes from images.
//!
//! This library is designed for [Thaimeleon](https://codeberg.org/thairanaru/thaimeleon).
//!
//! Note that this project has abnormal license(s) compared to most other Rust libraries.

// For color scheme structs
pub mod scheme_formats;
pub mod thaimeleon_scheme;

// Higher API
pub mod scheme_builder;
pub mod scheme_config;

// For color types struct
// Oklrab is public since palette doesn't support it normally and pretty handy to have for debugging
pub mod oklrab;
mod ab;
mod color_profile;
mod ab_profile;


// For calculation
mod accent_hues;
mod cluster;
mod neutral_color_builder;
mod contrast;
mod parse_image;

// For traits 
pub mod color_traits;

// Misc utils
mod hue_utils;
mod utils;

#[cfg(feature = "wasm")]
pub mod wasm;
