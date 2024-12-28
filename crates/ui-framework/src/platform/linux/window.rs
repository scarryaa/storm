use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_uint};
use std::ptr;
use x11_dl::xlib::Window as XWindow;
use x11_dl::xlib::{self, Display, Xlib};

pub(crate) struct LinuxWindow {
    pub native_handle: XWindow,
    xlib: *const Xlib,
    display: *mut Display,
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

            Ok(Self {
                native_handle: window,
                xlib,
                display,
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
}
