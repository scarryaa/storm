#ifdef __linux__
#include "../../../include/window.hpp"
#include <X11/Xlib.h>

struct Window::WindowImpl {
  Display *display;
  ::Window window;
  bool shouldClose = false;
};

Window::Window(const char *title, int width, int height) {
  impl = new WindowImpl();

  impl->display = XOpenDisplay(nullptr);
  int screen = DefaultScreen(impl->display);

  impl->window = XCreateSimpleWindow(
      impl->display, RootWindow(impl->display, screen), 0, 0, width, height, 1,
      BlackPixel(impl->display, screen), WhitePixel(impl->display, screen));

  XStoreName(impl->display, impl->window, title);
  XSelectInput(impl->display, impl->window, ExposureMask | KeyPressMask);
  XMapWindow(impl->display, impl->window);
}

Window::~Window() {
  XDestroyWindow(impl->display, impl->window);
  XCloseDisplay(impl->display);
  delete impl;
}

void Window::update() {
  XEvent event;
  while (XPending(impl->display)) {
    XNextEvent(impl->display, &event);
    if (event.type == ClientMessage) {
      impl->shouldClose = true;
    }
  }
}

bool Window::shouldClose() { return impl->shouldClose; }
#endif
