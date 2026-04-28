use crate::color_profile::ColorProfile;
use crate::oklrab::Oklrab;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use palette::Srgb;
use palette::convert::FromColorUnclamped;
use palette::convert::IntoColorUnclamped;
use quantette::ImageRef;
use quantette::PaletteSize;
use quantette::{Pipeline, QuantizeMethod};

#[derive(Debug, Display, Error, From)]
#[non_exhaustive]
pub enum ParseImageError {
    #[display("inserted image is empty")]
    EmptyInput,
    #[display("image quantization failed")]
    Other,
}

/// Parse image into a vector of color profiles
pub fn parse_image(
    img: ImageRef<'_, Srgb<u8>>,
    k_means_count: u8,
) -> Result<Vec<ColorProfile>, ParseImageError> {
    let total_pixels = img.height() * img.width();
    let pipeline = Pipeline::new()
        .palette_size(PaletteSize::from_u8_clamped(k_means_count))
        .ditherer(None)
        .dedup(false)
        .quantize_method(QuantizeMethod::kmeans())
        .parallel(false);
    let indexed_image_with_count = pipeline
        .input_image(img)
        .output_oklab_indexed_image()
        .into_indexed_image_counts();
    let index_colors = indexed_image_with_count.palette();
    let pixels_with_index = indexed_image_with_count.counts();
    let profiles: Vec<ColorProfile> = (0..index_colors.len())
        .map(|i| {
            ColorProfile::new(
                index_colors[i].into_color_unclamped(),
                pixels_with_index[i] as f32 / total_pixels as f32,
            )
        })
        .collect();
    if profiles.is_empty() {
        Err(ParseImageError::EmptyInput)
    } else {
        Ok(profiles)
    }
}

/// Parse image into a vector of color profiles
#[cfg(feature = "parallel")]
pub fn par_parse_image(
    img: ImageRef<'_, Srgb<u8>>,
    k_means_count: u8,
) -> Result<Vec<ColorProfile>, ParseImageError> {
    let total_pixels = img.height() * img.width();
    let pipeline = Pipeline::new()
        .palette_size(PaletteSize::from_u8_clamped(k_means_count))
        .ditherer(None)
        .dedup(false)
        .quantize_method(QuantizeMethod::kmeans())
        .parallel(true);
    let indexed_image_with_count = pipeline
        .input_image(img)
        .output_oklab_indexed_image()
        .into_indexed_image_counts();
    let index_colors = indexed_image_with_count.palette();
    let pixels_with_index = indexed_image_with_count.counts();
    let profiles: Vec<ColorProfile> = (0..index_colors.len())
        .map(|i| {
            ColorProfile::new(
                index_colors[i].into_color_unclamped(),
                pixels_with_index[i] as f32 / total_pixels as f32,
            )
        })
        .collect();
    if profiles.is_empty() {
        Err(ParseImageError::EmptyInput)
    } else {
        Ok(profiles)
    }
}


