pub mod error;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use self::linux::Application;
#[cfg(target_os = "windows")]
pub use self::windows::Application;
#[cfg(target_os = "macos")]
pub use crate::platform::macos::application::Application;

use crate::window::{WindowBehavior, WindowOptions};
use error::PlatformError;

pub fn create_platform_window(
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    #[cfg(target_os = "macos")]
    return macos::window::create_window(title, width, height, options);

    #[cfg(target_os = "windows")]
    return windows::window::create_window(title, width, height, options);

    #[cfg(target_os = "linux")]
    return linux::window::create_window(title, width, height, options);

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err(PlatformError::PlatformNotSupported)
}
