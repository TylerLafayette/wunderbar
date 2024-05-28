use core_graphics::color::CGColor;

/// Represents an RGBA color value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(f64, f64, f64, f64);

impl Color {
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);

    pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self(r, g, b, 1.0)
    }

    pub const fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self(r, g, b, a)
    }
}

impl Into<CGColor> for Color {
    fn into(self) -> CGColor {
        let Color(r, g, b, a) = self;

        CGColor::rgb(r, g, b, a)
    }
}
