#ifdef __linux__

#include "../../../include/window.hpp"

extern "C" {
    struct _XDisplay;
    typedef struct _XDisplay Display;
}

#include <X11/Xlib.h>
#include <stdexcept>

namespace Storm {

class Window::WindowImpl {
public:
    Display* display;
    ::Window window;  
    bool shouldClose;
    
    WindowImpl() : display(nullptr), window(0), shouldClose(false) {}
};

Window::Window(const char* title, int width, int height) {
    impl = new WindowImpl();
    impl->display = XOpenDisplay(nullptr);
    if (!impl->display) {
        throw std::runtime_error("Failed to open X display");
    }
    
    int screen = DefaultScreen(impl->display);
    impl->window = XCreateSimpleWindow(
        impl->display, 
        RootWindow(impl->display, screen),
        0, 0,  // x, y position
        width, height,
        1,  // border width
        BlackPixel(impl->display, screen),
        WhitePixel(impl->display, screen)
    );
    
    // Set window title
    XStoreName(impl->display, impl->window, title);
    
    // Set up window close notification
    Atom wmDeleteMessage = XInternAtom(impl->display, "WM_DELETE_WINDOW", False);
    XSetWMProtocols(impl->display, impl->window, &wmDeleteMessage, 1);
    
    // Select input events
    XSelectInput(impl->display, impl->window, 
                ExposureMask | KeyPressMask | StructureNotifyMask);
    
    // Show the window
    XMapWindow(impl->display, impl->window);
    XFlush(impl->display);
}

Window::~Window() {
    if (impl) {
        if (impl->display) {
            if (impl->window) {
                XDestroyWindow(impl->display, impl->window);
            }
            XCloseDisplay(impl->display);
        }
        delete impl;
    }
}

void Window::update() {
    if (!impl || !impl->display) return;
    
    while (XPending(impl->display)) {
        XEvent e;
        XNextEvent(impl->display, &e);
        
        if (e.type == ClientMessage) {
            impl->shouldClose = true;
        }
    }
    XFlush(impl->display);
}

bool Window::shouldClose() {
    return impl && impl->shouldClose;
}

} 

#endif
