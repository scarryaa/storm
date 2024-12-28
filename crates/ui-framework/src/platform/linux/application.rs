use crate::platform::error::PlatformError;
use crate::platform::linux::window::LinuxWindow;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_int;
use std::ptr;
use x11_dl::xlib::{self, Display, Xlib};

pub struct Application {
    xlib: Xlib,
    display: *mut Display,
}

impl Application {
    pub fn new() -> Result<Self, PlatformError> {
        unsafe {
            let xlib = xlib::Xlib::open().unwrap();
            let display = (xlib.XOpenDisplay)(ptr::null());

            if display.is_null() {
                return Err(PlatformError::DisplayInitFailed);
            }

            Ok(Self { xlib, display })
        }
    }

    pub fn run(&self, window: &LinuxWindow) -> Result<(), PlatformError> {
        unsafe {
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

            let wm_protocols =
                (xlib.XInternAtom)(self.display, wm_protocols_str.as_ptr(), xlib::False);
            let wm_delete_window =
                (xlib.XInternAtom)(self.display, wm_delete_window_str.as_ptr(), xlib::False);

            let mut protocols = [wm_delete_window];

            (xlib.XSetWMProtocols)(
                self.display,
                window.native_handle,
                protocols.as_mut_ptr(),
                protocols.len() as c_int,
            );

            // Show window
            (xlib.XMapWindow)(self.display, window.native_handle);

            // Main loop
            let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();

            loop {
                (xlib.XNextEvent)(self.display, &mut event);

                match event.get_type() {
                    xlib::ClientMessage => {
                        let xclient = xlib::XClientMessageEvent::from(event);

                        if xclient.message_type == wm_protocols && xclient.format == 32 {
                            let protocol = xclient.data.get_long(0) as xlib::Atom;

                            if protocol == wm_delete_window {
                                return Ok(());
                            }
                        }
                    }

                    _ => (),
                }
            }
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            (self.xlib.XCloseDisplay)(self.display);
        }
    }
}
