use crate::{platform::error::PlatformError, Application};

pub trait WindowBehavior {
    fn show(&mut self) -> Result<(), PlatformError>;
    fn hide(&mut self) -> Result<(), PlatformError>;
    fn set_title(&mut self, title: &str) -> Result<(), PlatformError>;
    fn set_size(&mut self, width: u32, height: u32) -> Result<(), PlatformError>;
}

pub struct WindowOptions {
    pub resizable: bool,
    pub decorations: bool,
    pub always_on_top: bool,
    pub visible: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            resizable: true,
            decorations: true,
            always_on_top: false,
            visible: true,
        }
    }
}

pub struct Window {
    inner: Box<dyn WindowBehavior>,
    title: String,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(
        app: &Application,
        title: String,
        width: u32,
        height: u32,
        options: WindowOptions,
    ) -> Result<Self, PlatformError> {
        let inner =
            crate::platform::create_platform_window(app, title.clone(), width, height, &options)?;

        Ok(Self {
            inner,
            title,
            width,
            height,
        })
    }

    pub fn show(&mut self) -> Result<(), PlatformError> {
        self.inner.show()
    }

    pub fn hide(&mut self) -> Result<(), PlatformError> {
        self.inner.hide()
    }
}
