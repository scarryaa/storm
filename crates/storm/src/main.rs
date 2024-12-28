use ui_framework::{Application, Window, WindowOptions};

fn main() {
    let app = Application::new();

    let options = WindowOptions::default();
    Window::new(&app, "storm".to_string(), 800, 600, options).unwrap();

    app.run();
}
