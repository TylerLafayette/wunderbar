use cocoa::{
    appkit::{NSApp, NSApplication, NSWindow},
    base::{id, nil},
    foundation::NSAutoreleasePool,
};
use objc::runtime::Object;

pub struct App {
    pool: id,
    app: *mut Object,
}

impl App {
    pub fn new() -> Self {
        // SAFETY: cleanup of initialized objects is handled by `App::drop`
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        let app = unsafe { NSApp() };

        Self { pool, app }
    }

    pub fn run(self) {
        unsafe { self.app.run() }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // SAFETY: we know this is safe because the only way to initialize App is through
        // `App::new`, which always constructs an `NSAutoreleasePool`
        unsafe { self.pool.drain() };
    }
}
