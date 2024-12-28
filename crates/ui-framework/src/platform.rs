use crate::window::Window;
use error::PlatformError;

pub mod error;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

pub trait Platform {
    fn create_window(&self, title: &str, width: u32, height: u32) -> Result<Window, PlatformError>;
}
