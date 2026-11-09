//! Theme colors.
//!
//! Dark is the default (user story 34); the palette follows the system
//! appearance, and an unknown appearance resolves to dark.

use gpui::WindowAppearance;
use gpui::{rgb, Rgba};

/// The palette for one appearance.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub background: Rgba,
    pub panel: Rgba,
    pub foreground: Rgba,
    pub muted_foreground: Rgba,
    pub accent: Rgba,
    pub accent_foreground: Rgba,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: rgb(0x18181b),
            panel: rgb(0x27272a),
            foreground: rgb(0xf4f4f5),
            muted_foreground: rgb(0xa1a1aa),
            accent: rgb(0x2f81f7),
            accent_foreground: rgb(0xffffff),
        }
    }

    pub fn light() -> Self {
        Self {
            background: rgb(0xfafafa),
            panel: rgb(0xffffff),
            foreground: rgb(0x18181b),
            muted_foreground: rgb(0x52525b),
            accent: rgb(0x2f81f7),
            accent_foreground: rgb(0xffffff),
        }
    }

    /// Follows the system appearance; anything not explicitly light is dark.
    pub fn from_appearance(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::light(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::dark(),
        }
    }
}
