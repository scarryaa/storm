use crate::platform::ApplicationBehavior;
use crate::platform::PlatformError;
use crate::Window;
use windows_sys::Win32::Foundation::HINSTANCE;

pub struct Application {
    instance: HINSTANCE,
}

impl ApplicationBehavior for Application {
    fn new() -> Result<Self, PlatformError> {
        unsafe {
            let instance = GetModuleHandleA(std::ptr::null());
            Ok(Self { instance })
        }
    }

    fn run(&self) -> Result<(), PlatformError> {
        unsafe {
            let mut message = std::mem::zeroed();
            while GetMessageA(&mut message, core::ptr::null_mut(), 0, 0) != 0 {
                DispatchMessageA(&message);
            }
            Ok(())
        }
    }

    fn set_window(&mut self, _window: Window) {
        // No-op for Windows
    }

    fn setup(&mut self) -> Result<(), PlatformError> {
        Ok(()) // No-op for Windows
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new().expect("Unable to create Application")
    }
}
