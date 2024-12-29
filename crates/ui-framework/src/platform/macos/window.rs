use super::metal_renderer::{MetalRenderer, Quad};
use crate::platform::error::PlatformError;
use crate::window::{WindowBehavior, WindowOptions};
use cocoa::appkit::NSBackingStoreType::NSBackingStoreBuffered;
use cocoa::appkit::{NSMainMenuWindowLevel, NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::NSString;
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize};
use metal::foreign_types::ForeignType;
use metal::NSInteger;
use objc::{class, msg_send, sel, sel_impl};
use std::any::Any;

#[allow(non_upper_case_globals)]
const NSViewLayerContentsRedrawDuringViewResize: NSInteger = 2;

pub(crate) struct MacOSWindow {
    native_handle: id,
    native_view: Box<dyn Any>,
    renderer: MetalRenderer,
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

            let renderer = MetalRenderer::new();

            let native_view: id = msg_send![class!(NSView), alloc];
            let native_view = NSView::init(native_view);

            let layer_obj: id = renderer.layer.as_ptr() as *mut objc::runtime::Object;
            native_view.setWantsLayer(YES);
            let _: () = msg_send![native_view, setLayer:layer_obj];
            let _: () = msg_send![layer_obj, setFrame: NSView::frame(native_view)];
            let _: () = msg_send![layer_obj, setOpaque: YES];
            let _: () = msg_send![
                native_view,
                setLayerContentsRedrawPolicy: NSViewLayerContentsRedrawDuringViewResize
            ];
            window.setContentView_(native_view);

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
                renderer,
                native_view: Box::new(native_view),
            })
        }
    }
}

impl WindowBehavior for MacOSWindow {
    fn show(&mut self) -> Result<(), PlatformError> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            self.native_handle.makeKeyAndOrderFront_(nil);

            let quad = Quad {
                position: [100.0, 100.0],
                size: [200.0, 200.0],
                color: [1.0, 0.0, 0.0, 1.0], // Red
            };

            // Get the window size for viewport
            let frame = NSView::frame(*self.native_view.downcast_ref::<id>().unwrap());
            let viewport_size = [frame.size.width as f32, frame.size.height as f32];

            // Draw the quad
            self.renderer.draw(&[quad], viewport_size);
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
