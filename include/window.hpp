namespace Storm {

class Window {
public:
    Window() = delete;
    Window(const char* title, int width, int height);
    ~Window();
    
    void update();
    bool shouldClose();
    
private:
    struct WindowImpl;
    WindowImpl* impl;
};

} 
