use core_graphics::context::{CGContext, CGContextRef};

use crate::ffi::sls::SlsWindow;

use super::{app::App, geometry::Bounds, Drawable, UiResult};

pub use crate::ffi::sls::CgsWindowTags as WindowTags;

pub struct Window<'app> {
    inner: SlsWindow<'app>,
    drawing_context: Option<CGContext>,
    bounds: Bounds,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WindowInitOptions {
    pub bounds: Bounds,
    pub tags: Option<WindowTags>,
    pub resolution: Option<f64>,
}

impl<'app> Window<'app> {
    pub(super) fn new(
        app: &'app App,
        WindowInitOptions {
            bounds,
            tags,
            resolution,
        }: WindowInitOptions,
    ) -> UiResult<Self> {
        let mut inner = app
            .conn
            .new_window(bounds.position.into(), bounds.size.into())?;

        if let Some(tags) = tags {
            inner.set_window_tags(tags)?;
        }

        if let Some(resolution) = resolution {
            inner.set_window_resolution(resolution)?;
        }

        Ok(Self {
            inner,
            bounds,
            drawing_context: None,
        })
    }

    pub fn bring_to_front(&mut self) -> UiResult<()> {
        self.inner.set_window_level(0)?;
        self.inner.order_window(1, None)?;

        Ok(())
    }

    fn get_context_ref(&mut self) -> UiResult<&CGContext> {
        if let Some(ref ctx) = self.drawing_context {
            Ok(ctx)
        } else {
            let ctx = self.inner.get_cg_context()?;
            self.drawing_context = Some(ctx);

            if let Some(ref ctx) = self.drawing_context {
                Ok(ctx)
            } else {
                unreachable!("`self.drawing_context` set to Some(_) in previous instruction")
            }
        }
    }

    pub fn draw(&mut self, drawable: impl Drawable) -> UiResult<()> {
        let bounds = self.bounds;
        let ctx = self.get_context_ref()?;

        drawable.draw(ctx, bounds)?;
        ctx.flush();
        self.inner.flush_window_content_region()?;

        Ok(())
    }
}
