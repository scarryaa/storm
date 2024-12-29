#[derive(Debug)]
pub enum PlatformError {
    WindowCreationError(String),
    EventError(String),
    PlatformNotSupported,
    DisplayInitFailed,
    NoWindowSet,
    VulkanError(String),
}

impl From<Box<dyn std::error::Error>> for PlatformError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        PlatformError::VulkanError(error.to_string())
    }
}
