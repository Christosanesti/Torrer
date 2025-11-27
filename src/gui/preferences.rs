// Preferences panel for GUI settings
use gtk4::prelude::*;
use gtk4::{Box, Label, Switch, SpinButton, ComboBoxText, Orientation, Separator};
use std::sync::Arc;
use std::sync::Mutex;

/// GUI preferences
#[derive(Debug, Clone)]
pub struct GuiPreferences {
    pub theme: String,
    pub update_frequency_ms: u32,
    pub show_notifications: bool,
    pub auto_start: bool,
}

impl Default for GuiPreferences {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            update_frequency_ms: 2000,
            show_notifications: true,
            auto_start: false,
        }
    }
}

/// Preferences panel widget
pub struct PreferencesPanel {
    container: Box,
    preferences: Arc<Mutex<GuiPreferences>>,
    theme_combo: ComboBoxText,
    update_frequency_spin: SpinButton,
    notifications_switch: Switch,
    auto_start_switch: Switch,
}

impl PreferencesPanel {
    /// Create a new preferences panel
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 10);
        container.set_margin_top(10);
        container.set_margin_bottom(10);
        container.set_margin_start(10);
        container.set_margin_end(10);

        let preferences = Arc::new(Mutex::new(GuiPreferences::default()));

        // Theme selection
        let theme_box = Box::new(Orientation::Horizontal, 10);
        let theme_label = Label::new(Some("Theme:"));
        theme_label.set_halign(gtk4::Align::Start);
        theme_label.set_hexpand(true);
        theme_box.append(&theme_label);

        let theme_combo = ComboBoxText::new();
        theme_combo.append_text("Default");
        theme_combo.append_text("Dark");
        theme_combo.append_text("Light");
        theme_combo.set_active(0);
        theme_box.append(&theme_combo);
        container.append(&theme_box);

        // Update frequency
        let freq_box = Box::new(Orientation::Horizontal, 10);
        let freq_label = Label::new(Some("Update Frequency (ms):"));
        freq_label.set_halign(gtk4::Align::Start);
        freq_label.set_hexpand(true);
        freq_box.append(&freq_label);

        let update_frequency_spin = SpinButton::new(
            Some(&gtk4::Adjustment::new(2000.0, 500.0, 10000.0, 100.0, 500.0, 0.0)),
            1.0,
            0,
        );
        update_frequency_spin.set_value(2000.0);
        freq_box.append(&update_frequency_spin);
        container.append(&freq_box);

        container.append(&Separator::new(Orientation::Horizontal));

        // Notifications toggle
        let notif_box = Box::new(Orientation::Horizontal, 10);
        let notif_label = Label::new(Some("Show Notifications:"));
        notif_label.set_halign(gtk4::Align::Start);
        notif_label.set_hexpand(true);
        notif_box.append(&notif_label);

        let notifications_switch = Switch::new();
        notifications_switch.set_active(true);
        notif_box.append(&notifications_switch);
        container.append(&notif_box);

        // Auto-start toggle
        let auto_box = Box::new(Orientation::Horizontal, 10);
        let auto_label = Label::new(Some("Auto-start on Launch:"));
        auto_label.set_halign(gtk4::Align::Start);
        auto_label.set_hexpand(true);
        auto_box.append(&auto_label);

        let auto_start_switch = Switch::new();
        auto_start_switch.set_active(false);
        auto_box.append(&auto_start_switch);
        container.append(&auto_box);

        // Connect signals
        let prefs_clone = preferences.clone();
        let theme_combo_clone = theme_combo.clone();
        theme_combo.connect_changed(move |_| {
            let mut prefs = prefs_clone.lock().unwrap();
            let active = theme_combo_clone.active().unwrap_or(0);
            prefs.theme = match active {
                1 => "dark".to_string(),
                2 => "light".to_string(),
                _ => "default".to_string(),
            };
        });

        let prefs_clone = preferences.clone();
        let freq_clone = update_frequency_spin.clone();
        update_frequency_spin.connect_value_changed(move |_| {
            let mut prefs = prefs_clone.lock().unwrap();
            prefs.update_frequency_ms = freq_clone.value() as u32;
        });

        let prefs_clone = preferences.clone();
        let notif_clone = notifications_switch.clone();
        notifications_switch.connect_state_set(move |_, state| {
            let mut prefs = prefs_clone.lock().unwrap();
            prefs.show_notifications = state;
            gtk4::glib::Propagation::Proceed
        });

        let prefs_clone = preferences.clone();
        let auto_clone = auto_start_switch.clone();
        auto_start_switch.connect_state_set(move |_, state| {
            let mut prefs = prefs_clone.lock().unwrap();
            prefs.auto_start = state;
            gtk4::glib::Propagation::Proceed
        });

        Self {
            container,
            preferences,
            theme_combo,
            update_frequency_spin,
            notifications_switch,
            auto_start_switch,
        }
    }

    /// Get current preferences
    pub fn get_preferences(&self) -> GuiPreferences {
        self.preferences.lock().unwrap().clone()
    }

    /// Set preferences
    pub fn set_preferences(&self, prefs: GuiPreferences) {
        *self.preferences.lock().unwrap() = prefs.clone();
        
        // Update UI
        let theme_idx = match prefs.theme.as_str() {
            "dark" => 1,
            "light" => 2,
            _ => 0,
        };
        self.theme_combo.set_active(Some(theme_idx));
        self.update_frequency_spin.set_value(prefs.update_frequency_ms as f64);
        self.notifications_switch.set_active(prefs.show_notifications);
        self.auto_start_switch.set_active(prefs.auto_start);
    }

    /// Get the container widget
    pub fn widget(&self) -> &Box {
        &self.container
    }
}





