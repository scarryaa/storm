use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use windows_sys::Win32::Foundation::HWND;

pub(crate) struct WindowsWindow {
    native_handle: HWND,
}

pub(crate) fn create_window(
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    Ok(Box::new(WindowsWindow::new(title, width, height, options)?))
}

impl WindowsWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        options: &WindowOptions,
    ) -> Result<Self, PlatformError> {
        todo!()
    }
}

impl WindowBehavior for WindowsWindow {
    fn show(&mut self) -> Result<(), PlatformError> {
        todo!()
    }

    fn hide(&mut self) -> Result<(), PlatformError> {
        todo!()
    }

    fn set_title(&mut self, title: &str) -> Result<(), PlatformError> {
        todo!()
    }

    fn set_size(&mut self, width: u32, height: u32) -> Result<(), PlatformError> {
        todo!()
    }
}
