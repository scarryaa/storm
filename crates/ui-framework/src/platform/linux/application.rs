use crate::platform::error::PlatformError;
use crate::platform::linux::window::LinuxWindow;
use crate::platform::ApplicationBehavior;
use crate::Window;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_int;
use std::ptr;
use x11_dl::xlib::{self, Display, Xlib};

pub struct Application {
    pub xlib: Xlib,
    pub display: *mut Display,
    window: Option<LinuxWindow>,
    wm_protocols: xlib::Atom,
    wm_delete_window: xlib::Atom,
}

impl ApplicationBehavior for Application {
    fn new() -> Result<Self, PlatformError> {
        unsafe {
            let xlib = xlib::Xlib::open().unwrap();
            let display = (xlib.XOpenDisplay)(ptr::null());

            if display.is_null() {
                return Err(PlatformError::DisplayInitFailed);
            }

            Ok(Self {
                xlib,
                display,
                window: None,
                wm_protocols: 0,
                wm_delete_window: 0,
            })
        }
    }

    fn run(&self) -> Result<(), PlatformError> {
        unsafe {
            // Main loop
            let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();

            loop {
                (self.xlib.XNextEvent)(self.display, &mut event);

                match event.get_type() {
                    xlib::ClientMessage => {
                        let xclient = xlib::XClientMessageEvent::from(event);

                        if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                            let protocol = xclient.data.get_long(0) as xlib::Atom;

                            if protocol == self.wm_delete_window {
                                return Ok(());
                            }
                        }
                    }

                    _ => (),
                }
            }
        }
    }

    fn set_window(&mut self, window: Window) {
        if let Some(linux_window) = window.into_inner() {
            self.window = Some(linux_window);
        }
    }

    fn setup(&mut self) -> Result<(), PlatformError> {
        unsafe {
            if let Some(window) = &self.window {
                let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
                let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

                // Store these as struct fields so run() can access them
                self.wm_protocols =
                    (self.xlib.XInternAtom)(self.display, wm_protocols_str.as_ptr(), xlib::False);
                self.wm_delete_window = (self.xlib.XInternAtom)(
                    self.display,
                    wm_delete_window_str.as_ptr(),
                    xlib::False,
                );

                let mut protocols = [self.wm_delete_window];

                (self.xlib.XSetWMProtocols)(
                    self.display,
                    window.native_handle,
                    protocols.as_mut_ptr(),
                    protocols.len() as c_int,
                );

                // Show window
                (self.xlib.XMapWindow)(self.display, window.native_handle);
                Ok(())
            } else {
                Err(PlatformError::NoWindowSet)
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
