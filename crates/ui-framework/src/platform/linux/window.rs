use super::vulkan_renderer::VulkanRenderer;
use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use std::any::Any;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_uint};
use std::ptr;
use x11_dl::xlib::Window as XWindow;
use x11_dl::xlib::{self, Display, Xlib};

#[derive(Clone)]
pub(crate) struct LinuxWindow {
    pub native_handle: XWindow,
    xlib: *const Xlib,
    display: *mut Display,
    renderer: VulkanRenderer,
}

pub(crate) fn create_window(
    xlib: &Xlib,
    display: *mut Display,
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    unsafe {
        Ok(Box::new(LinuxWindow::new(
            xlib, display, title, width, height, options,
        )?))
    }
}

impl LinuxWindow {
    fn new(
        xlib: &Xlib,
        display: *mut Display,
        title: String,
        width: u32,
        height: u32,
        options: &WindowOptions,
    ) -> Result<Self, PlatformError> {
        unsafe {
            let screen = (xlib.XDefaultScreen)(display);
            let root = (xlib.XRootWindow)(display, screen);
            let mut attributes: xlib::XSetWindowAttributes =
                mem::MaybeUninit::uninit().assume_init();
            attributes.background_pixel = (xlib.XWhitePixel)(display, screen);

            let window = (xlib.XCreateWindow)(
                display,
                root,
                0,
                0,
                width,
                height,
                0,
                0,
                xlib::InputOutput as c_uint,
                ptr::null_mut(),
                xlib::CWBackPixel,
                &mut attributes,
            );

            let title_str = CString::new(title).unwrap();
            (xlib.XStoreName)(display, window, title_str.as_ptr() as *mut c_char);

            // Map window and ensure it's visible
            (xlib.XMapWindow)(display, window);
            (xlib.XFlush)(display);

            // Add small delay to ensure window is mapped
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Final sync before creating Vulkan renderer
            (xlib.XSync)(display, 0);

            // Verify window attributes are valid
            let mut window_attributes = mem::MaybeUninit::uninit();
            if (xlib.XGetWindowAttributes)(display, window, window_attributes.as_mut_ptr()) == 0 {
                (xlib.XDestroyWindow)(display, window);
                return Err(PlatformError::WindowCreationError(
                    "XGetWindowAttributes failed".to_string(),
                ));
            }

            let vulkan_renderer = VulkanRenderer::new(window, display, width, height, xlib);

            Ok(Self {
                native_handle: window,
                xlib,
                display,
                renderer: vulkan_renderer?,
            })
        }
    }
}

impl WindowBehavior for LinuxWindow {
    fn show(&mut self) -> Result<(), PlatformError> {
        unsafe {
            ((*self.xlib).XMapWindow)(self.display, self.native_handle);
            Ok(())
        }
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drop for LinuxWindow {
    fn drop(&mut self) {
        unsafe {
            ((*self.xlib).XDestroyWindow)(self.display, self.native_handle);
        }
    }
}
