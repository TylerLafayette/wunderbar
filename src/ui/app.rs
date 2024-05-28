use crate::ffi::sls::SlsConnection;

use super::{
    window::{Window, WindowInitOptions},
    UiResult,
};

pub struct App {
    pub(super) conn: SlsConnection,
}

impl App {
    pub fn new() -> Self {
        Self {
            conn: SlsConnection::new(),
        }
    }

    pub fn create_window(&self, options: WindowInitOptions) -> UiResult<Window<'_>> {
        Window::new(self, options)
    }

    pub fn run(&self) -> UiResult<()> {
        self.conn.run_app()?;

        Ok(())
    }
}
