//! WASM bindings for `thaimeleon_lib`. Enabled by the `wasm` feature.
//!
//! Single entry point: [`generate_scheme`].

use palette::Srgb;
use quantette::ImageRef;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::scheme_config::FullConfigPartial;
use crate::thaimeleon_scheme::{NamedHues, ThaimeleonScheme};

const FORCE_LIGHT: i32 = 1;
const FORCE_DARK: i32 = 0;

/// Generate a Thaimeleon scheme from raw RGBA pixel data.
///
/// `rgba` must be `width * height * 4` bytes. Alpha is dropped.
/// `force_theme`: -1 = auto, 0 = force dark, 1 = force light.
/// `config` is a [`FullConfigPartial`] serialized to a JS object — any field
/// can be omitted, in which case the per-theme default is used.
#[wasm_bindgen]
pub fn generate_scheme(
    rgba: &[u8],
    width: u32,
    height: u32,
    force_theme: i32,
    config: JsValue,
) -> Result<JsValue, JsError> {
    console_error_panic_hook::set_once();

    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|p| p.checked_mul(4))
        .ok_or_else(|| JsError::new("width * height * 4 overflows usize"))?;
    if rgba.len() != expected {
        return Err(JsError::new(&format!(
            "rgba length {} does not match width*height*4 = {}",
            rgba.len(),
            expected
        )));
    }

    let mut rgb: Vec<u8> = Vec::with_capacity((width as usize) * (height as usize) * 3);
    for chunk in rgba.chunks_exact(4) {
        rgb.extend_from_slice(&chunk[..3]);
    }

    let pixels: &[Srgb<u8>] = palette::cast::from_component_slice::<Srgb<u8>>(&rgb);
    let img = ImageRef::new(width, height, pixels)
        .map_err(|e| JsError::new(&format!("invalid image dimensions: {:?}", e.error)))?;

    let partial: FullConfigPartial = if config.is_undefined() || config.is_null() {
        FullConfigPartial::default()
    } else {
        serde_wasm_bindgen::from_value(config)
            .map_err(|e| JsError::new(&format!("invalid config: {}", e)))?
    };

    let mut builder = partial.into_scheme_builder();
    builder.light_theme_threshold = match force_theme {
        FORCE_LIGHT => 0.0,
        FORCE_DARK => 1.0,
        _ => builder.light_theme_threshold,
    };

    let scheme = builder
        .generate_from_image(img)
        .map_err(|e| JsError::new(&format!("scheme generation failed: {}", e)))?;

    let web = WebScheme::from(&scheme);
    serde_wasm_bindgen::to_value(&web).map_err(|e| JsError::new(&e.to_string()))
}

fn hex(c: Srgb<u8>) -> String {
    format!("#{:02x}{:02x}{:02x}", c.red, c.green, c.blue)
}

#[derive(Serialize)]
struct WebScheme {
    is_light_theme: bool,
    surfaces: WebSurfaces,
    high_contrast_fg_accents: [String; 6],
    fg_accents: [String; 6],
    rg_accents: [String; 6],
    bg_accents: [String; 6],
    high_contrast_fg_named: WebNamedHues,
    fg_named: WebNamedHues,
    rg_named: WebNamedHues,
    bg_named: WebNamedHues,
}

#[derive(Serialize)]
struct WebSurfaces {
    surface_low: String,
    base: String,
    base_high: String,
    surface_high: String,
    surface_higher: String,
    surface_highest: String,
    faint: String,
    muted: String,
    subtext: String,
    text: String,
    black: String,
    white: String,
}

#[derive(Serialize)]
struct WebNamedHues {
    red: String,
    orange: String,
    yellow: String,
    green: String,
    cyan: String,
    blue: String,
    purple: String,
    magenta: String,
}

impl From<&NamedHues<Srgb<u8>>> for WebNamedHues {
    fn from(n: &NamedHues<Srgb<u8>>) -> Self {
        Self {
            red: hex(n.red),
            orange: hex(n.orange),
            yellow: hex(n.yellow),
            green: hex(n.green),
            cyan: hex(n.cyan),
            blue: hex(n.blue),
            purple: hex(n.purple),
            magenta: hex(n.magenta),
        }
    }
}

impl From<&ThaimeleonScheme> for WebScheme {
    fn from(s: &ThaimeleonScheme) -> Self {
        let arr = |a: &[Srgb<u8>; 6]| -> [String; 6] {
            [hex(a[0]), hex(a[1]), hex(a[2]), hex(a[3]), hex(a[4]), hex(a[5])]
        };
        Self {
            is_light_theme: s.is_light_theme,
            surfaces: WebSurfaces {
                surface_low: hex(s.surface_low),
                base: hex(s.base),
                base_high: hex(s.base_high),
                surface_high: hex(s.surface_high),
                surface_higher: hex(s.surface_higher),
                surface_highest: hex(s.surface_highest),
                faint: hex(s.faint),
                muted: hex(s.muted),
                subtext: hex(s.subtext),
                text: hex(s.text),
                black: hex(s.black),
                white: hex(s.white),
            },
            high_contrast_fg_accents: arr(&s.high_contrast_fg_accents),
            fg_accents: arr(&s.fg_accents),
            rg_accents: arr(&s.rg_accents),
            bg_accents: arr(&s.bg_accents),
            high_contrast_fg_named: (&s.high_contrast_fg_named_accents).into(),
            fg_named: (&s.fg_named_accents).into(),
            rg_named: (&s.rg_named_accents).into(),
            bg_named: (&s.bg_named_accents).into(),
        }
    }
}
