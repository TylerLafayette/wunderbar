use thiserror::Error;

use self::geometry::{Bounds, Size};

pub use crate::ffi::CGError;

pub mod app;
pub mod block;
pub mod color;
pub mod geometry;
pub mod window;

type Context = core_graphics::context::CGContext;

pub type UiResult<T> = Result<T, Error>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("core graphics internal error: {0}")]
    CgError(#[from] CGError),
}

pub trait Drawable {
    fn content_size(&self, bounds: Bounds) -> Size;
    fn draw(&self, ctx: &Context, bounds: Bounds) -> UiResult<()>;
}

impl Drawable for () {
    fn content_size(&self, _bounds: Bounds) -> Size {
        Size::new(0, 0)
    }

    fn draw(&self, _ctx: &Context, _bounds: Bounds) -> UiResult<()> {
        Ok(())
    }
}
