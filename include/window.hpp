namespace Storm {

class Window {
public:
  struct WindowImpl;
  Window() = delete;
  Window(const char *title, int width, int height);
  ~Window();

  void update();
  bool shouldClose();

private:
  WindowImpl *impl;
};

} // namespace Storm
