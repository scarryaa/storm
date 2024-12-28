use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use cocoa::appkit::NSBackingStoreType::NSBackingStoreBuffered;
use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, NO};
use cocoa::foundation::NSString;
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize};

pub(crate) struct MacOSWindow {
    native_handle: id,
}

pub(crate) fn create_window(
    title: String,
    width: u32,
    height: u32,
    options: &WindowOptions,
) -> Result<Box<dyn WindowBehavior>, PlatformError> {
    Ok(Box::new(MacOSWindow::new(title, width, height, options)?))
}

impl MacOSWindow {
    fn new(
        title: String,
        width: u32,
        height: u32,
        options: &WindowOptions,
    ) -> Result<Self, PlatformError> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let mut style_mask = NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask
                | NSWindowStyleMask::NSUnifiedTitleAndToolbarWindowMask;
            if options.resizable {
                style_mask |= NSWindowStyleMask::NSResizableWindowMask;
            }
            if !options.decorations {
                style_mask |= NSWindowStyleMask::NSBorderlessWindowMask;
            }

            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(
                    NSPoint::new(0., 0.),
                    NSSize::new(width as f64, height as f64),
                ),
                style_mask,
                NSBackingStoreBuffered,
                NO,
            );

            let title_str = NSString::alloc(nil).init_str(&title);
            window.setTitle_(title_str);

            if options.always_on_top {
                window.setLevel_(NSMainMenuWindowLevel.into());
            }

            window.center();

            if options.visible {
                window.makeKeyAndOrderFront_(nil);
            }

            Ok(Self {
                native_handle: window,
            })
        }
    }
}

impl WindowBehavior for MacOSWindow {
    fn show(&mut self) -> Result<(), PlatformError> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            self.native_handle.makeKeyAndOrderFront_(nil);
        }
        Ok(())
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
