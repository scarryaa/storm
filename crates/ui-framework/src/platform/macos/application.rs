use crate::platform::ApplicationBehavior;
use crate::platform::PlatformError;
use crate::Window;
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};
use cocoa::base::{id, nil, YES};
use cocoa::foundation::NSAutoreleasePool;

pub struct Application {
    app: id,
    window: Option<Window>,
}

impl ApplicationBehavior for Application {
    fn new() -> Result<Self, PlatformError> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(
                NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            );
            app.activateIgnoringOtherApps_(YES);

            Ok(Self {
                app,
                window: None, // Initialize as None
            })
        }
    }

    fn set_window(&mut self, window: Window) {
        self.window = Some(window);
    }

    fn show(&mut self) {
        // Show the window if we have one
        if let Some(window) = &mut self.window {
            window.show().expect("Failed to show window");
        }
    }

    fn run(&self) -> Result<(), PlatformError> {
        unsafe {
            self.app.run();
            Ok(())
        }
    }

    fn setup(&mut self) -> Result<(), PlatformError> {
        Ok(())
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new().expect("Unable to create Application")
    }
}
