pub mod error;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use crate::platform::linux::application::Application;
#[cfg(target_os = "macos")]
pub use crate::platform::macos::application::Application;
#[cfg(target_os = "windows")]
pub use crate::platform::windows::application::Application;

use crate::window::{WindowBehavior, WindowOptions};
use crate::Window;
use error::PlatformError;

pub trait ApplicationBehavior {
    fn new() -> Result<Self, PlatformError>
    where
        Self: Sized;
    fn set_window(&mut self, window: Window);
    fn setup(&mut self) -> Result<(), PlatformError>;
    fn run(&self) -> Result<(), PlatformError>;
    fn show(&mut self);
}

pub fn create_platform_window(
    app: &Application,
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    #[cfg(target_os = "macos")]
    return macos::window::create_window(title, width, height, options);

    #[cfg(target_os = "windows")]
    return windows::window::create_window(app, title, width, height, options);

    #[cfg(target_os = "linux")]
    return linux::window::create_window(&app.xlib, app.display, title, width, height, options);

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err(PlatformError::PlatformNotSupported)
}
