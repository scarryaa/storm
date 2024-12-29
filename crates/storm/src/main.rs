use ui_framework::platform::error::PlatformError;
use ui_framework::platform::ApplicationBehavior;
use ui_framework::{Application, Window, WindowOptions};

fn main() -> Result<(), PlatformError> {
    let mut app = Application::new()?;
    let window = Window::new(
        &app,
        "storm".to_string(),
        800,
        600,
        WindowOptions::default(),
    )?;

    app.set_window(window);
    app.setup()?;
    app.show();
    app.run()
}
