use cocoa::appkit::{NSApplication, NSApplicationActivationPolicy};
use cocoa::base::{id, nil, YES};
use cocoa::foundation::NSAutoreleasePool;

pub struct Application {
    app: id,
}

impl Application {
    pub fn new() -> Self {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let app = NSApplication::sharedApplication(nil);
            app.setActivationPolicy_(
                NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
            );
            app.activateIgnoringOtherApps_(YES);

            Self { app }
        }
    }

    pub fn run(&self) {
        unsafe {
            self.app.run();
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
