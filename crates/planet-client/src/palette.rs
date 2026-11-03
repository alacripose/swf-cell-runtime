//! The palette: the one place a role word becomes a colour.
//!
//! `planet-package` declares paper and ink as **roles** and knows nothing about colour. This module is
//! the other half, and it lives in the client because a colour is a rendering decision: `surface` is a
//! role, `#14161c` is this client's answer for it. A package that declares a role the palette does not
//! carry is not drawn — the frame is refused rather than painted in a neighbour's colour.

/// One colour, as the raster carries it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }

    /// No paper at all: the window shows through.
    pub const TRANSPARENT: Rgba = Rgba::rgba(0, 0, 0, 0);
}

/// The role words this client can draw, and the colour it draws each one in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palette {
    entries: Vec<(String, Rgba)>,
}

impl Palette {
    /// The declared palette: every paper and ink role the format names, in its own order.
    pub fn declared() -> Palette {
        let mut palette = Palette { entries: Vec::new() };
        for (role, colour) in [
            ("surface", Rgba::rgba(20, 22, 28, 255)),
            ("chrome", Rgba::rgba(27, 30, 38, 255)),
            ("background", Rgba::rgba(11, 12, 16, 255)),
            ("divider", Rgba::rgba(42, 47, 58, 255)),
            ("transparent", Rgba::TRANSPARENT),
            ("text", Rgba::rgba(230, 232, 240, 255)),
            ("dim", Rgba::rgba(154, 160, 173, 255)),
            ("disabled", Rgba::rgba(90, 96, 109, 255)),
            ("accent", Rgba::rgba(108, 182, 255, 255)),
            ("red", Rgba::rgba(255, 107, 107, 255)),
            ("green", Rgba::rgba(100, 214, 138, 255)),
            ("orange", Rgba::rgba(255, 171, 94, 255)),
        ] {
            palette.entries.push((role.to_string(), colour));
        }
        palette
    }

    /// The colour for `role`, or `None` when this palette does not carry it.
    pub fn resolve(&self, role: &str) -> Option<Rgba> {
        self.entries
            .iter()
            .find(|(name, _)| name == role)
            .map(|(_, colour)| *colour)
    }

    /// The same palette without one role, which is how a caller states that it cannot draw it.
    pub fn without(&self, role: &str) -> Palette {
        Palette {
            entries: self
                .entries
                .iter()
                .filter(|(name, _)| name != role)
                .cloned()
                .collect(),
        }
    }

    /// The role words this palette carries, in declared order.
    pub fn roles(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(name, _)| name.as_str())
    }
}

impl Default for Palette {
    fn default() -> Self {
        Palette::declared()
    }
}
