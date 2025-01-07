#ifdef __APPLE__
#import <Cocoa/Cocoa.h>
#include "../../../include/window.hpp"

struct Window::WindowImpl {
    NSWindow* window;
    bool shouldClose = false;
};

@interface WindowDelegate : NSObject<NSWindowDelegate>
@property Window::WindowImpl* impl;
@end

@implementation WindowDelegate
- (void)windowWillClose:(NSNotification *)notification {
    self.impl->shouldClose = true;
}
@end

Window::Window(const char* title, int width, int height) {
    impl = new WindowImpl();
    
    [NSApplication sharedApplication];
    [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];
    
    WindowDelegate* delegate = [[WindowDelegate alloc] init];
    delegate.impl = impl;
    
    impl->window = [[NSWindow alloc] 
        initWithContentRect:NSMakeRect(0, 0, width, height)
        styleMask:NSWindowStyleMaskTitled | 
                 NSWindowStyleMaskClosable | 
                 NSWindowStyleMaskMiniaturizable |
                 NSWindowStyleMaskResizable
        backing:NSBackingStoreBuffered
        defer:NO];
    
    [impl->window setTitle:[NSString stringWithUTF8String:title]];
    [impl->window setDelegate:delegate];
    [impl->window center];
    [impl->window makeKeyAndOrderFront:nil];
    
    [NSApp activateIgnoringOtherApps:YES];
}

Window::~Window() {
    delete impl;
}

void Window::update() {
    @autoreleasepool {
        NSEvent* event;
        do {
            event = [NSApp nextEventMatchingMask:NSEventMaskAny
                                     untilDate:nil
                                        inMode:NSDefaultRunLoopMode
                                       dequeue:YES];
            if (event) {
                [NSApp sendEvent:event];
            }
        } while (event);
    }
}

bool Window::shouldClose() {
    return impl->shouldClose;
}
#endif
