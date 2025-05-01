use core_foundation::{
    attributed_string::{CFAttributedString, CFAttributedStringCreate},
    base::{TCFType, ToVoid},
    dictionary::{CFDictionary, CFMutableDictionary},
    number::kCFBooleanTrue,
    string::CFString,
};
use core_text::{
    font::{kCTFontSystemFontType, CTFont, CTFontUIFontType},
    line::CTLine,
    string_attributes::{kCTFontAttributeName, kCTForegroundColorFromContextAttributeName},
};

use super::{
    color::Color,
    geometry::{Bounds, Size},
    Drawable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    ct_font: CTFont,
}

const DEFAULT_FONT_TYPE: CTFontUIFontType = kCTFontSystemFontType;
const DEFAULT_FONT_SIZE: f64 = 16.0;

impl Default for Font {
    fn default() -> Self {
        let ct_font =
            core_text::font::new_ui_font_for_language(DEFAULT_FONT_TYPE, DEFAULT_FONT_SIZE, None);

        Self { ct_font }
    }
}

impl Font {
    pub fn new(font_name: &str, font_size_pt: f64) -> Option<Self> {
        let ct_font = core_text::font::new_from_name(font_name, font_size_pt).ok()?;

        Some(Self { ct_font })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextProps {
    pub font: Font,
    pub color: Color,
}

impl Default for TextProps {
    fn default() -> Self {
        Self {
            font: Default::default(),
            color: Color::BLACK,
        }
    }
}

pub struct Text {
    line: CTLine,
    props: TextProps,
}

impl Text {
    pub fn new(src: &str, props: TextProps) -> Self {
        let line = Self::create_ct_line_from_str(src, &props.font);

        Self { line, props }
    }

    fn create_ct_line_from_str(src: &str, font: &Font) -> CTLine {
        let attr_string = Self::create_attributed_string_from_str(src, font);
        CTLine::new_with_attributed_string(attr_string.as_concrete_TypeRef())
    }

    fn create_attributed_string_from_str(src: &str, font: &Font) -> CFAttributedString {
        let cf_string = CFString::new(src);

        let mut attr_dict: CFMutableDictionary = CFMutableDictionary::new();
        unsafe {
            // SAFETY: this unsafe block is required because `k*` constants are external statics.
            // however, these constants are static constants imported from CoreFoundation so we
            // know that taking a reference to them should always be valid.
            attr_dict.add(
                &kCTFontAttributeName.to_void(),
                &font.ct_font.as_CFTypeRef(),
            );
            attr_dict.add(
                &kCTForegroundColorFromContextAttributeName.to_void(),
                &kCFBooleanTrue.to_void(),
            );
        }

        // SAFETY: we wrap the created `CFAttributedString` so that its reference count is properly
        // decreased on Drop.
        let attr_str = unsafe {
            let str_ref = CFAttributedStringCreate(
                std::ptr::null(),
                cf_string.as_concrete_TypeRef(),
                attr_dict.as_concrete_TypeRef(),
            );

            CFAttributedString::wrap_under_create_rule(str_ref)
        };

        attr_str
    }

    fn get_text_bounds(&self, ctx: &super::Context) -> Bounds {
        let bounds = self.line.get_image_bounds(&ctx);

        bounds.into()
    }
}

impl Drawable for Text {
    fn content_size(&self, ctx: &super::Context, _bounds: super::geometry::Bounds) -> Size {
        // TODO: respect maximum bounds from parent
        self.get_text_bounds(ctx).size
    }

    fn draw(&self, ctx: &super::Context, bounds: super::geometry::Bounds) -> super::UiResult<()> {
        ctx.set_fill_color(&self.props.color.into_cg_color());
        ctx.set_text_position(bounds.position.x as f64, bounds.position.y as f64);

        self.line.draw(ctx);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_equivalency_test() {
        let font1 = Font::new("Times New Roman", 12.0).expect("font should be available");
        let font1_ = Font::new("Times New Roman", 12.0).expect("font should be available");
        let font2 = Font::new("Arial", 14.0).expect("font should be available");

        assert!(font1 == font1_);
        assert!(font1 != font2);
    }
}
