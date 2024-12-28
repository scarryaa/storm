use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use x11_dl::xlib::Window as XWindow;

pub(crate) struct LinuxWindow {
    native_handle: XWindow,
}

pub(crate) fn create_window(
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    Ok(Box::new(LinuxWindow::new(title, width, height, options)?))
}

impl LinuxWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        options: &WindowOptions,
    ) -> Result<Self, PlatformError> {
        todo!()
    }
}

impl WindowBehavior for LinuxWindow {
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
