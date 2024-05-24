use bitflags::bitflags;
use core_graphics::sys::CGContextRef as CGContextRefSys;
use libc::{c_double, c_float, c_int, c_void};

use crate::ffi::core_services::{CFRelease, CGSNewRegionWithRect};

use super::{CGError, CGPoint, CGRect, CGSize};

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
    fn SLSSetWindowTags(cid: c_int, wid: u32, tags: *const u64, tag_size: c_int) -> CGError;
    fn SLSClearWindowTags(cid: c_int, wid: u32, tags: *const u64, tag_size: c_int) -> CGError;
    fn SLSOrderWindow(cid: c_int, wid: u32, mode: c_int, relativeToWID: u32) -> CGError;
    fn SLSSetWindowLevel(cid: c_int, wid: u32, level: c_int) -> CGError;
    fn SLSSetWindowResolution(cid: c_int, wid: u32, res: c_double) -> CGError;
    fn SLWindowContextCreate(cid: c_int, wid: u32, options: *const c_void) -> *mut c_void;
    fn SLSFlushWindowContentRegion(cid: c_int, wid: u32, dirty: *const c_void);
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
    pub fn new() -> Self {
        // SAFETY: FFI call
        let conn_id = unsafe { SLSMainConnectionID() };

        Self { conn_id }
    }

    pub fn new_window(&mut self, origin: CGPoint, size: CGSize) -> Result<SlsWindow<'_>, ()> {
        SlsWindow::new(&*self, origin, size)
    }
}

#[derive(Debug)]
pub struct SlsWindow<'conn> {
    conn: &'conn SlsConnection,
    window_id: u32,
}

impl<'conn> SlsWindow<'conn> {
    fn new(conn: &'conn SlsConnection, origin: CGPoint, size: CGSize) -> Result<Self, ()> {
        unsafe {
            let rect = CGRect::new(&origin, &size);
            let mut region_ptr: *const c_void = std::ptr::null_mut();
            // TODO: error handling
            if CGSNewRegionWithRect(&rect as *const _, &mut region_ptr as *mut _) > 0 {
                return Err(());
            }

            let mut window_id: u32 = 0;
            let res = SLSNewWindow(
                conn.conn_id,
                2,
                0.0,
                0.0,
                region_ptr,
                &mut window_id as *mut _,
            );
            if res > 0 {
                return Err(());
            }

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

        if err > 0 {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn clear_window_tags(&mut self, tags_to_clear: CgsWindowTags) -> Result<(), CGError> {
        let tag_bits = tags_to_clear.bits();
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe {
            SLSClearWindowTags(self.conn.conn_id, self.window_id, &tag_bits as *const _, 64)
        };

        if err > 0 {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn order_window(&mut self, mode: i32, relative_to: &Self) -> Result<(), CGError> {
        // SAFETY: we know the connection and window are valid due to the lifetimes of the structs
        let err = unsafe {
            SLSOrderWindow(
                self.conn.conn_id,
                self.window_id,
                mode,
                relative_to.window_id,
            )
        };

        if err > 0 {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn get_cg_context(&mut self) -> core_graphics::context::CGContext {
        // SAFETY: The context returned from [`SLWindowContextCreate`] refers to the same
        // object [`CGContext`] that is also used by the `core_graphics` crate. The use of
        // transmute here is only to convince Rust to accept the pointer as a [`CGContextRefSys`].
        //
        // The `core_graphics` crate handles dropping the [`CGContext`], using Apple's
        // `CGContextRelease` method to decrement the reference count of the object, maintaining
        // safety.
        unsafe {
            let ctx = SLWindowContextCreate(self.conn.conn_id, self.window_id, std::ptr::null());
            let ctx = std::mem::transmute::<*const c_void, CGContextRefSys>(ctx);

            core_graphics::context::CGContext::from_existing_context_ptr(ctx)
        }
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
    use super::*;

    #[test]
    fn it_works() {
        let mut conn = SlsConnection::new();
    }
}
