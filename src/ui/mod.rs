use thiserror::Error;

use self::geometry::{Bounds, Size};

pub use crate::ffi::CGError;

pub mod app;
pub mod block;
pub mod color;
pub mod geometry;
pub mod layout;
pub mod text;
pub mod window;

type Context = core_graphics::context::CGContext;

pub type UiResult<T> = Result<T, Error>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("core graphics internal error: {0}")]
    CgError(#[from] CGError),
}

pub trait Drawable {
    fn content_size(&self, ctx: &Context, bounds: Bounds) -> Size;
    fn draw(&self, ctx: &Context, bounds: Bounds) -> UiResult<()>;
}

impl Drawable for () {
    fn content_size(&self, _ctx: &Context, _bounds: Bounds) -> Size {
        Size::new(0, 0)
    }

    fn draw(&self, _ctx: &Context, _bounds: Bounds) -> UiResult<()> {
        Ok(())
    }
}

pub trait IntoBoxDrawable {
    fn into_box_drawable(self) -> Box<dyn Drawable>;

    /// Shorthand for [`IntoBoxDrawable::into_box_drawable`]
    fn erase(self) -> Box<dyn Drawable>
    where
        Self: Sized,
    {
        self.into_box_drawable()
    }
}

impl<T> IntoBoxDrawable for T
where
    T: Drawable + 'static,
{
    fn into_box_drawable(self) -> Box<dyn Drawable> {
        Box::new(self)
    }
}

impl Drawable for Box<dyn Drawable> {
    fn content_size(&self, ctx: &Context, bounds: Bounds) -> Size {
        Drawable::content_size(&**self, ctx, bounds)
    }

    fn draw(&self, ctx: &Context, bounds: Bounds) -> UiResult<()> {
        Drawable::draw(&**self, ctx, bounds)
    }
}
