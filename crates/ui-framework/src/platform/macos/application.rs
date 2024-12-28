use crate::platform::ApplicationBehavior;
use crate::platform::PlatformError;
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
}

impl Default for Application {
    fn default() -> Self {
        Self::new().expect("Unable to create Application")
    }
}
