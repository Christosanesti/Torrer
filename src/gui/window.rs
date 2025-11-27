// Main window for GUI - Story 3.7 and 3.8 implementation
use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box, Button, Label, Notebook, HeaderBar, MenuButton, 
    ScrolledWindow, TextView, Orientation, Align, ResponseType, 
    FileChooserDialog, FileFilter, FileChooserAction, glib
};
use gio::SimpleAction;
use std::sync::Arc;
use std::sync::Mutex;
use crate::error::{TorrerError, TorrerResult};
use crate::core::engine::{TorrerEngine, EngineStatus};
use crate::gui::settings::SettingsPanel;
use crate::gui::statistics::StatisticsDashboard;
use crate::gui::circuit_viz::CircuitVisualization;
use crate::gui::preferences::PreferencesPanel;
use crate::gui::notifications::GuiNotificationManager;
use crate::gui::dialogs;

/// Main application window
pub struct MainWindow {
    window: ApplicationWindow,
    engine: Arc<Mutex<Option<TorrerEngine>>>,
    status_label: Label,
    start_button: Button,
    stop_button: Button,
    restart_button: Button,
    settings_panel: Arc<Mutex<SettingsPanel>>,
    statistics: Arc<Mutex<StatisticsDashboard>>,
    circuit_viz: Arc<Mutex<CircuitVisualization>>,
    preferences: Arc<Mutex<PreferencesPanel>>,
    notifications: Arc<Mutex<GuiNotificationManager>>,
    logs_text: TextView,
    update_source_id: Option<glib::SourceId>,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(app: &gtk4::Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Torrer - Tor Routing GUI")
            .default_width(1000)
            .default_height(700)
            .build();

        // Create engine (lazy initialization)
        let engine = Arc::new(Mutex::new(None::<TorrerEngine>));

        // Header bar with menu
        let header = HeaderBar::new();
        window.set_titlebar(Some(&header));

        // Menu button
        let menu_button = MenuButton::new();
        let menu = gio::Menu::new();
        menu.append(Some("Import Config"), Some("app.import_config"));
        menu.append(Some("Export Config"), Some("app.export_config"));
        menu.append(Some("Help"), Some("app.help"));
        menu.append(Some("About"), Some("app.about"));
        menu_button.set_menu_model(Some(&menu));

        // Actions
        let import_action = SimpleAction::new("import_config", None);
        let export_action = SimpleAction::new("export_config", None);
        let help_action = SimpleAction::new("help", None);
        let about_action = SimpleAction::new("about", None);
        
        app.add_action(&import_action);
        app.add_action(&export_action);
        app.add_action(&help_action);
        app.add_action(&about_action);

        header.pack_end(&menu_button);

        // Main container
        let main_box = Box::new(Orientation::Vertical, 10);
        main_box.set_margin_start(10);
        main_box.set_margin_end(10);
        main_box.set_margin_top(10);
        main_box.set_margin_bottom(10);

        // Status panel
        let status_box = Box::new(Orientation::Horizontal, 10);
        let status_label = Label::new(Some("Status: Not Connected"));
        status_label.set_halign(Align::Start);
        status_box.append(&status_label);
        main_box.append(&status_box);

        // Control buttons
        let button_box = Box::new(Orientation::Horizontal, 10);
        let start_button = Button::with_label("Start");
        let stop_button = Button::with_label("Stop");
        let restart_button = Button::with_label("Restart");
        
        stop_button.set_sensitive(false);
        restart_button.set_sensitive(false);
        
        button_box.append(&start_button);
        button_box.append(&stop_button);
        button_box.append(&restart_button);
        main_box.append(&button_box);

        // Notebook for tabs
        let notebook = Notebook::new();
        
        // Settings tab
        let settings_panel = Arc::new(Mutex::new(SettingsPanel::new()));
        let settings_widget = settings_panel.lock().unwrap().widget().clone();
        notebook.append_page(&settings_widget, Some(&Label::new(Some("Settings"))));

        // Statistics tab
        let statistics = Arc::new(Mutex::new(StatisticsDashboard::new()));
        let stats_widget = statistics.lock().unwrap().widget().clone();
        notebook.append_page(&stats_widget, Some(&Label::new(Some("Statistics"))));

        // Circuit visualization tab
        let circuit_viz = Arc::new(Mutex::new(CircuitVisualization::new()));
        let circuit_widget = circuit_viz.lock().unwrap().widget().clone();
        notebook.append_page(&circuit_widget, Some(&Label::new(Some("Circuits"))));

        // Preferences tab
        let preferences = Arc::new(Mutex::new(PreferencesPanel::new()));
        let prefs_widget = preferences.lock().unwrap().widget().clone();
        notebook.append_page(&prefs_widget, Some(&Label::new(Some("Preferences"))));

        // Logs tab
        let logs_scroll = ScrolledWindow::new();
        let logs_text = TextView::new();
        logs_text.set_editable(false);
        logs_text.set_monospace(true);
        logs_scroll.set_child(Some(&logs_text));
        notebook.append_page(&logs_scroll, Some(&Label::new(Some("Logs"))));

        main_box.append(&notebook);
        window.set_child(Some(&main_box));

        // Notifications
        let notifications = Arc::new(Mutex::new(GuiNotificationManager::new()));

        let window_clone = window.clone();
        let engine_clone = engine.clone();
        let status_label_clone = status_label.clone();
        let start_button_clone = start_button.clone();
        let stop_button_clone = stop_button.clone();
        let restart_button_clone = restart_button.clone();
        let notifications_clone = notifications.clone();

        // Start button handler
        start_button.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            let engine = engine_clone.clone();
            let status_label = status_label_clone.clone();
            let start_btn = start_button_clone.clone();
            let stop_btn = stop_button_clone.clone();
            let restart_btn = restart_button_clone.clone();
            let notifications = notifications_clone.clone();
            
            glib::spawn_future_local(async move {
                // Initialize engine if needed
                let mut engine_guard = engine.lock().unwrap();
                if engine_guard.is_none() {
                    match TorrerEngine::new() {
                        Ok(e) => *engine_guard = Some(e),
                        Err(e) => {
                            glib::idle_add_local(move || {
                                status_label.set_text(&format!("Error: {}", e));
                                start_btn.set_sensitive(true);
                                false
                            });
                            return;
                        }
                    }
                }

                if let Some(ref mut eng) = *engine_guard {
                    match eng.start().await {
                        Ok(_) => {
                            glib::idle_add_local(move || {
                                status_label.set_text("Status: Connected");
                                start_btn.set_sensitive(false);
                                stop_btn.set_sensitive(true);
                                restart_btn.set_sensitive(true);
                                notifications.lock().unwrap().notify_connected();
                                false
                            });
                        }
                        Err(e) => {
                            glib::idle_add_local(move || {
                                status_label.set_text(&format!("Error: {}", e));
                                start_btn.set_sensitive(true);
                                false
                            });
                        }
                    }
                }
            });
        });

        // Stop button handler
        stop_button.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            let engine = engine_clone.clone();
            let status_label = status_label_clone.clone();
            let start_btn = start_button_clone.clone();
            let stop_btn = stop_button_clone.clone();
            let restart_btn = restart_button_clone.clone();
            let notifications = notifications_clone.clone();
            
            glib::spawn_future_local(async move {
                let mut engine_guard = engine.lock().unwrap();
                if let Some(ref mut eng) = *engine_guard {
                    match eng.stop().await {
                        Ok(_) => {
                            glib::idle_add_local(move || {
                                status_label.set_text("Status: Disconnected");
                                start_btn.set_sensitive(true);
                                stop_btn.set_sensitive(false);
                                restart_btn.set_sensitive(false);
                                notifications.lock().unwrap().notify_disconnected();
                                false
                            });
                        }
                        Err(e) => {
                            glib::idle_add_local(move || {
                                status_label.set_text(&format!("Error: {}", e));
                                stop_btn.set_sensitive(true);
                                false
                            });
                        }
                    }
                }
            });
        });

        // Restart button handler
        restart_button.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            let engine = engine_clone.clone();
            let status_label = status_label_clone.clone();
            let start_btn = start_button_clone.clone();
            let stop_btn = stop_button_clone.clone();
            let restart_btn = restart_button_clone.clone();
            let notifications = notifications_clone.clone();
            
            glib::spawn_future_local(async move {
                let mut engine_guard = engine.lock().unwrap();
                if let Some(ref mut eng) = *engine_guard {
                    match eng.restart().await {
                        Ok(_) => {
                            glib::idle_add_local(move || {
                                status_label.set_text("Status: Restarted");
                                start_btn.set_sensitive(false);
                                stop_btn.set_sensitive(true);
                                restart_btn.set_sensitive(true);
                                notifications.lock().unwrap().notify_connected();
                                false
                            });
                        }
                        Err(e) => {
                            glib::idle_add_local(move || {
                                status_label.set_text(&format!("Error: {}", e));
                                restart_btn.set_sensitive(true);
                                false
                            });
                        }
                    }
                }
            });
        });

        // Import config action
        let window_for_import = window.clone();
        import_action.connect_activate(move |_, _| {
            let dialog = FileChooserDialog::builder()
                .title("Import Configuration")
                .action(FileChooserAction::Open)
                .modal(true)
                .build();
            
            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Import", ResponseType::Accept);
            
            let filter = FileFilter::new();
            filter.add_pattern("*.toml");
            filter.set_name(Some("TOML Files"));
            dialog.add_filter(&filter);
            
            dialog.set_transient_for(Some(&window_for_import));
            
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            if let Some(path_str) = path.to_str() {
                                use crate::config::ConfigManager;
                                match ConfigManager::new() {
                                    Ok(manager) => {
                                        match manager.import(path_str) {
                                            Ok(_) => {
                                                dialogs::show_info(Some(&window_for_import), "Success", "Configuration imported successfully");
                                            }
                                            Err(e) => {
                                                dialogs::show_torrer_error(Some(&window_for_import), &e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        dialogs::show_torrer_error(Some(&window_for_import), &e);
                                    }
                                }
                            }
                        }
                    }
                }
                dialog.close();
            });
            
            dialog.show();
        });

        // Export config action
        let window_for_export = window.clone();
        export_action.connect_activate(move |_, _| {
            let dialog = FileChooserDialog::builder()
                .title("Export Configuration")
                .action(FileChooserAction::Save)
                .modal(true)
                .build();
            
            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Export", ResponseType::Accept);
            
            let filter = FileFilter::new();
            filter.add_pattern("*.toml");
            filter.set_name(Some("TOML Files"));
            dialog.add_filter(&filter);
            
            dialog.set_transient_for(Some(&window_for_export));
            
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            if let Some(path_str) = path.to_str() {
                                use crate::config::ConfigManager;
                                match ConfigManager::new() {
                                    Ok(manager) => {
                                        match manager.export(path_str) {
                                            Ok(_) => {
                                                dialogs::show_info(Some(&window_for_export), "Success", "Configuration exported successfully");
                                            }
                                            Err(e) => {
                                                dialogs::show_torrer_error(Some(&window_for_export), &e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        dialogs::show_torrer_error(Some(&window_for_export), &e);
                                    }
                                }
                            }
                        }
                    }
                }
                dialog.close();
            });
            
            dialog.show();
        });

        // Help action
        let window_for_help = window.clone();
        help_action.connect_activate(move |_, _| {
            dialogs::show_help_dialog(Some(&window_for_help));
        });

        // About action
        let window_for_about = window.clone();
        about_action.connect_activate(move |_, _| {
            dialogs::show_about_dialog(Some(&window_for_about));
        });

        // Setup real-time updates
        let engine_for_updates = engine.clone();
        let status_label_for_updates = status_label.clone();
        let statistics_for_updates = statistics.clone();
        let circuit_viz_for_updates = circuit_viz.clone();
        
        let update_source_id = glib::timeout_add_local(
            std::time::Duration::from_secs(2),
            move || {
                let engine = engine_for_updates.clone();
                let status_label = status_label_for_updates.clone();
                let statistics = statistics_for_updates.clone();
                let circuit_viz = circuit_viz_for_updates.clone();
                
                glib::spawn_future_local(async move {
                    let mut engine_guard = engine.lock().unwrap();
                    if let Some(ref mut eng) = *engine_guard {
                        match eng.status().await {
                            Ok(status) => {
                                let status_text = format!(
                                    "Status: {} | Tor: {} | Circuit: {}",
                                    if status.is_running { "Running" } else { "Stopped" },
                                    if status.tor_connected { "Connected" } else { "Disconnected" },
                                    if status.circuit_established { "Established" } else { "Not Established" }
                                );
                                
                                glib::idle_add_local(move || {
                                    status_label.set_text(&status_text);
                                    // Update statistics and circuit visualization
                                    // (These would need methods to update from EngineStatus)
                                    false
                                });
                            }
                            Err(_) => {}
                        }
                    }
                });
                
                glib::Continue(true)
            }
        );

        Self {
            window,
            engine,
            status_label,
            start_button,
            stop_button,
            restart_button,
            settings_panel,
            statistics,
            circuit_viz,
            preferences,
            notifications,
            logs_text,
            update_source_id: Some(update_source_id),
        }
    }

    /// Present the window
    pub fn present(&self) {
        self.window.present();
    }

    /// Get the underlying window
    pub fn window(&self) -> &ApplicationWindow {
        &self.window
    }
}

impl Drop for MainWindow {
    fn drop(&mut self) {
        if let Some(source_id) = self.update_source_id {
            source_id.remove();
        }
    }
}

