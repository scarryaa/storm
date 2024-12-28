#[derive(Debug)]
pub enum PlatformError {
    WindowCreationError(String),
    EventError(String),
    PlatformNotSupported,
}
