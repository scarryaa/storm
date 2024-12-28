use ui_framework::platform::ApplicationBehavior;
use ui_framework::{Application, Window, WindowOptions};

fn main() {
    let app = Application::new().unwrap();

    let options = WindowOptions::default();
    let window = Window::new(&app, "storm".to_string(), 800, 600, options).unwrap();

    let _ = app.run();
}
