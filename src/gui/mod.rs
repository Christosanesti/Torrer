// GUI module - Story 3.7 and 3.8 implementation

pub mod app;
pub mod window;
pub mod settings;
pub mod statistics;
pub mod circuit_viz;
pub mod notifications;
pub mod preferences;
pub mod dialogs;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

pub use app::GuiApp;
pub use window::MainWindow;
pub use settings::SettingsPanel;
pub use statistics::StatisticsDashboard;
pub use circuit_viz::CircuitVisualization;
pub use notifications::GuiNotificationManager;
pub use preferences::{PreferencesPanel, GuiPreferences};
pub use dialogs;

