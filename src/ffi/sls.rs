use bitflags::bitflags;
use core_graphics::sys::CGContextRef as CGContextRefSys;
use libc::{c_double, c_float, c_int, c_void};

use super::{
    core_services::{CFDictionary, CFRelease, CFRunLoopRun, CFValue, CGSNewRegionWithRect},
    CGError, CGPoint, CGRect, CGResult, CGSize,
};

type CFDictionaryRef = *const c_void;

#[link(name = "SkyLight", kind = "framework")]
extern "C" {
    fn SLSMainConnectionID() -> c_int;
    fn SLSNewWindow(
        cid: c_int,
        type_: c_int,
        x: c_float,
        y: c_float,
        region: *const c_void,
        wid: *mut u32,
    ) -> i32;
    fn SLSReleaseWindow(cid: c_int, wid: u32) -> i32;
    fn SLSSetWindowTags(cid: c_int, wid: u32, tags: *const u64, tag_size: c_int) -> i32;
    fn SLSClearWindowTags(cid: c_int, wid: u32, tags: *const u64, tag_size: c_int) -> i32;
    fn SLSOrderWindow(cid: c_int, wid: u32, mode: c_int, relativeToWID: u32) -> i32;
    fn SLSSetWindowLevel(cid: c_int, wid: u32, level: c_int) -> i32;
    fn SLSSetWindowResolution(cid: c_int, wid: u32, res: c_double) -> i32;
    fn SLWindowContextCreate(cid: c_int, wid: u32, options: *const c_void) -> *mut c_void;
    fn SLSFlushWindowContentRegion(cid: c_int, wid: u32, dirty: *const c_void) -> i32;
    fn SLSWindowSetShadowProperties(wid: u32, properties: CFDictionaryRef) -> i32;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CgsWindowTags: u64 {
        const ExposeFade = 1 << 1;
        const PreventsActivation = 1 << 16;
        const ModalWindow = 1 << 31;
        const DisableShadow = 1 << 3;
        const HighQualityResampling = 1 << 4;
        const IgnoreForExposeTagBit = 1 << 7;
        const Sticky = 1 << 11;
    }
}

#[derive(Debug)]
pub struct SlsConnection {
    conn_id: c_int,
}

impl SlsConnection {
    /// Constructs a new [`SlsConnection`]. This function does not actually allocate, as the
    /// SkyLight framework automatically assigns a global connection ID to each process on
    /// initiation. [`SlsConnecction`] simply acts as an abstraction for maintaining state.
    pub fn new() -> Self {
        // SAFETY: FFI call
        let conn_id = unsafe { SLSMainConnectionID() };

        Self { conn_id }
    }

    pub fn new_window(&self, origin: CGPoint, size: CGSize) -> CGResult<SlsWindow<'_>> {
        SlsWindow::new(&*self, origin, size)
    }

    pub fn run_app(&self) -> CGResult<()> {
        unsafe {
            CFRunLoopRun();
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SlsWindow<'conn> {
    conn: &'conn SlsConnection,
    window_id: u32,
}

impl<'conn> SlsWindow<'conn> {
    fn new(conn: &'conn SlsConnection, origin: CGPoint, size: CGSize) -> CGResult<Self> {
        unsafe {
            let rect = CGRect::new(&origin, &size);
            let mut region_ptr: *const c_void = std::ptr::null_mut();
            // TODO: error handling
            CGError::result_from(CGSNewRegionWithRect(
                &rect as *const _,
                &mut region_ptr as *mut _,
            ))?;

            let mut window_id: u32 = 0;
            CGError::result_from(SLSNewWindow(
                conn.conn_id,
                2,
                0.0,
                0.0,
                region_ptr,
                &mut window_id as *mut _,
            ))?;

            CFRelease(region_ptr);

            Ok(Self { conn, window_id })
        }
    }

    pub fn set_window_tags(&mut self, tags: CgsWindowTags) -> Result<(), CGError> {
        let tag_bits = tags.bits();
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe {
            SLSSetWindowTags(self.conn.conn_id, self.window_id, &tag_bits as *const _, 64)
        };

        CGError::result_from(err)
    }

    pub fn clear_window_tags(&mut self, tags_to_clear: CgsWindowTags) -> Result<(), CGError> {
        let tag_bits = tags_to_clear.bits();
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe {
            SLSClearWindowTags(self.conn.conn_id, self.window_id, &tag_bits as *const _, 64)
        };

        CGError::result_from(err)
    }

    pub fn order_window(&mut self, mode: i32, relative_to: Option<&Self>) -> Result<(), CGError> {
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe {
            SLSOrderWindow(
                self.conn.conn_id,
                self.window_id,
                mode,
                if let Some(other) = relative_to {
                    other.window_id
                } else {
                    self.window_id
                },
            )
        };

        CGError::result_from(err)
    }

    pub fn set_window_level(&mut self, level: i32) -> Result<(), CGError> {
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe { SLSSetWindowLevel(self.conn.conn_id, self.window_id, level) };

        CGError::result_from(err)
    }

    pub fn set_window_resolution(&mut self, resolution: f64) -> Result<(), CGError> {
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe { SLSSetWindowResolution(self.conn.conn_id, self.window_id, resolution) };

        CGError::result_from(err)
    }

    pub fn flush_window_content_region(&mut self) -> Result<(), CGError> {
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe {
            SLSFlushWindowContentRegion(self.conn.conn_id, self.window_id, std::ptr::null())
        };

        CGError::result_from(err)
    }

    /// Creates a graphics context which can be used to draw graphics on the [`SlsWindow`].
    ///
    /// If the framework returns a null context pointer, signaling that there was an issue creating
    /// the context, this function will return an error value of [`CGError::Failure`].
    pub fn get_cg_context(&mut self) -> CGResult<core_graphics::context::CGContext> {
        // SAFETY: The context returned from [`SLWindowContextCreate`] refers to the same
        // object [`CGContext`] that is also used by the `core_graphics` crate. The use of
        // transmute here is only to convince Rust to accept the pointer as a [`CGContextRefSys`].
        //
        // The `core_graphics` crate handles dropping the [`CGContext`], using Apple's
        // `CGContextRelease` method to decrement the reference count of the object, maintaining
        // safety.
        unsafe {
            let ctx = SLWindowContextCreate(self.conn.conn_id, self.window_id, std::ptr::null());
            if ctx.is_null() {
                return Err(CGError::Failure);
            }

            let ctx = std::mem::transmute::<*const c_void, CGContextRefSys>(ctx);

            Ok(core_graphics::context::CGContext::from_existing_context_ptr(ctx))
        }
    }

    pub fn disable_shadow(&self) -> CGResult<()> {
        let shadow_density_key = CFValue::String("com.apple.WindowShadowDensity".to_string());
        let shadow_density = CFValue::Index(0);
        let dictionary = CFDictionary::new_from_entries([(shadow_density_key, shadow_density)])?;

        CGError::result_from(unsafe {
            SLSWindowSetShadowProperties(self.window_id, dictionary.as_ptr())
        })?;

        Ok(())
    }
}

impl<'conn> Drop for SlsWindow<'conn> {
    fn drop(&mut self) {
        let err = unsafe { SLSReleaseWindow(self.conn.conn_id, self.window_id) };

        assert_eq!(err, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::color::Color;

    use super::*;

    #[test]
    fn window_test() {
        let conn = SlsConnection::new();
        let mut win = conn
            .new_window(CGPoint::new(0.0, 0.0), CGSize::new(1280.0, 40.0))
            .unwrap();
        win.set_window_tags(CgsWindowTags::ExposeFade | CgsWindowTags::PreventsActivation)
            .unwrap();
        win.clear_window_tags(CgsWindowTags::empty()).unwrap();
        win.set_window_resolution(2.0).unwrap();
        win.set_window_level(1).unwrap();
        // NOTE: it seems that this step is necessary to get the window to appear, at least on my
        // machine
        win.order_window(1, None).unwrap();

        let ctx = win.get_cg_context().unwrap();
        ctx.set_fill_color(&Color::GREEN.into());
        ctx.fill_rect(CGRect::new(
            &CGPoint::new(0.0, 0.0),
            &CGSize::new(200.0, 40.0),
        ));
        ctx.flush();

        win.flush_window_content_region().unwrap();

        conn.run_app().unwrap();
        loop {}
    }
}
