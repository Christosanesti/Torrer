// GUI application - Story 3.7 and 3.8 implementation
use gtk4::prelude::*;
use gtk4::Application;
use crate::error::TorrerResult;
use crate::gui::window::MainWindow;

/// GUI application
pub struct GuiApp;

impl GuiApp {
    /// Create a new GUI app
    pub fn new() -> TorrerResult<Self> {
        Ok(Self)
    }

    /// Run the GUI application
    pub fn run(&self) -> TorrerResult<()> {
        let app = Application::builder()
            .application_id("com.torrer.gui")
            .build();

        app.connect_activate(|app| {
            let window = MainWindow::new(app);
            window.present();
        });

        app.run();
        Ok(())
    }
}
