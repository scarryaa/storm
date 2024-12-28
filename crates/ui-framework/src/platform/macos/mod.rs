use super::Platform;

pub mod window;

pub struct PlatformImpl {}

impl Platform for PlatformImpl {
    fn create_window(
        &self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<crate::window::Window, super::error::PlatformError> {
        todo!()
    }
}
