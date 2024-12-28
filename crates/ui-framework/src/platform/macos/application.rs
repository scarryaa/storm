use crate::platform::ApplicationBehavior;
use crate::platform::PlatformError;
use crate::Window;
use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};
use cocoa::base::{id, nil, YES};
use cocoa::foundation::NSAutoreleasePool;

pub struct Application {
    app: id,
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

            Ok(Self { app })
        }
    }

    fn run(&self) -> Result<(), PlatformError> {
        unsafe {
            self.app.run();
            Ok(())
        }
    }

    fn set_window(&mut self, _window: Window) {
        // No-op for macOS
    }

    fn setup(&mut self) -> Result<(), PlatformError> {
        Ok(()) // No-op for macOS
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new().expect("Unable to create Application")
    }
}
