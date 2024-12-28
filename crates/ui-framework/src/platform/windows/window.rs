use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub(crate) struct WindowsWindow {
    native_handle: HWND,
}

pub(crate) fn create_window(
    app: &Application,
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    Ok(Box::new(WindowsWindow::new(
        app.instance,
        title,
        width,
        height,
        options,
    )?))
}

impl WindowsWindow {
    fn new(
        instance: HINSTANCE,
        title: String,
        width: u32,
        height: u32,
        options: &WindowOptions,
    ) -> Result<Self, PlatformError> {
        unsafe {
            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(core::ptr::null_mut(), IDC_ARROW),
                hInstance: instance,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: core::ptr::null_mut(),
                hbrBackground: core::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(
                0,
                window_class,
                s!("window"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                instance,
                std::ptr::null(),
            );

            Self {
                native_handle: handle,
            }
        }
    }
}

impl WindowBehavior for WindowsWindow {
    fn show(&mut self) -> Result<(), PlatformError> {
        unsafe {
            ShowWindow(self.native_handle, SW_SHOW);
            Ok(())
        }
    }

    fn hide(&mut self) -> Result<(), PlatformError> {
        unsafe {
            ShowWindow(self.native_handle, SW_HIDE);
            Ok(())
        }
    }

    fn set_title(&mut self, title: &str) -> Result<(), PlatformError> {
        unsafe {
            let title = std::ffi::CString::new(title).unwrap();
            SetWindowTextA(self.native_handle, title.as_ptr());
            Ok(())
        }
    }

    fn set_size(&mut self, width: u32, height: u32) -> Result<(), PlatformError> {
        unsafe {
            SetWindowPos(
                self.native_handle,
                0,
                0,
                0,
                width as i32,
                height as i32,
                SWP_NOMOVE | SWP_NOZORDER,
            );
            Ok(())
        }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    DefWindowProcA(hwnd, msg, wparam, lparam)
}
