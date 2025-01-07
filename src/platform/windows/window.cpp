#ifdef _WIN32
#include "../../../include/window.hpp"
#include <windows.h>

struct Window::WindowImpl {
  HWND hwnd;
  bool shouldClose = false;
};

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam,
                            LPARAM lParam) {
  switch (uMsg) {
  case WM_CLOSE:
    DestroyWindow(hwnd);
    return 0;
  case WM_DESTROY:
    PostQuitMessage(0);
    return 0;
  }
  return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

Window::Window(const char *title, int width, int height) {
  impl = new WindowImpl();

  WNDCLASS wc = {};
  wc.lpfnWndProc = WindowProc;
  wc.hInstance = GetModuleHandle(nullptr);
  wc.lpszClassName = "WindowClass";
  RegisterClass(&wc);

  impl->hwnd =
      CreateWindowEx(0, "WindowClass", title, WS_OVERLAPPEDWINDOW,
                     CW_USEDEFAULT, CW_USEDEFAULT, width, height, nullptr,
                     nullptr, GetModuleHandle(nullptr), nullptr);

  ShowWindow(impl->hwnd, SW_SHOW);
}

Window::~Window() { delete impl; }

void Window::update() {
  MSG msg;
  while (PeekMessage(&msg, nullptr, 0, 0, PM_REMOVE)) {
    if (msg.message == WM_QUIT) {
      impl->shouldClose = true;
    }
    TranslateMessage(&msg);
    DispatchMessage(&msg);
  }
}

bool Window::shouldClose() { return impl->shouldClose; }
#endif
