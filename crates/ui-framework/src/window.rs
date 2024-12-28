#[cfg(target_os = "linux")]
use crate::platform::linux::window::LinuxWindow;
use crate::{platform::error::PlatformError, Application};
use std::any::Any;

pub trait WindowBehavior: Any {
    fn show(&mut self) -> Result<(), PlatformError>;
    fn hide(&mut self) -> Result<(), PlatformError>;
    fn set_title(&mut self, title: &str) -> Result<(), PlatformError>;
    fn set_size(&mut self, width: u32, height: u32) -> Result<(), PlatformError>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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

    #[cfg(not(target_os = "linux"))]
    pub(crate) fn into_inner(self) -> Option<Box<dyn WindowBehavior>> {
        None
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn into_inner(self) -> Option<LinuxWindow> {
        let window_ptr = Box::into_raw(self.inner);
        unsafe {
            let behavior = &*window_ptr;
            if behavior.as_any().is::<LinuxWindow>() {
                let boxed = Box::from_raw(window_ptr);
                let any_box = boxed.as_any();
                any_box.downcast_ref::<LinuxWindow>().cloned()
            } else {
                None
            }
        }
    }
}
