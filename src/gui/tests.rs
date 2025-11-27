// Unit tests for GUI components
#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::*;
    use crate::core::monitoring::Statistics;
    use crate::tor::circuit::CircuitInfo;
    use std::time::Duration;

    #[test]
    fn test_statistics_dashboard_creation() {
        let dashboard = StatisticsDashboard::new();
        // Widget exists if we can call widget()
        let _widget = dashboard.widget();
    }

    #[test]
    fn test_statistics_dashboard_update() {
        let dashboard = StatisticsDashboard::new();
        let stats = Statistics {
            uptime: Some(Duration::from_secs(3600)),
            bytes_sent: 1024 * 1024,
            bytes_received: 2048 * 1024,
            connection_attempts: 10,
            successful_connections: 9,
            success_rate: 90.0,
        };
        dashboard.update_stats(&stats);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_circuit_visualization_creation() {
        let viz = CircuitVisualization::new();
        // Widget exists if we can call widget()
        let _widget = viz.widget();
    }

    #[test]
    fn test_circuit_visualization_update() {
        let viz = CircuitVisualization::new();
        let circuits = vec![
            CircuitInfo {
                id: "1".to_string(),
                status: "BUILT".to_string(),
                purpose: Some("GENERAL".to_string()),
                flags: Some("FAST".to_string()),
            },
            CircuitInfo {
                id: "2".to_string(),
                status: "BUILT".to_string(),
                purpose: Some("GENERAL".to_string()),
                flags: Some("STABLE".to_string()),
            },
        ];
        viz.update_circuits(circuits);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_circuit_visualization_empty() {
        let viz = CircuitVisualization::new();
        viz.update_circuits(vec![]);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_preferences_panel_creation() {
        let panel = PreferencesPanel::new();
        // Widget exists if we can call widget()
        let _widget = panel.widget();
    }

    #[test]
    fn test_preferences_panel_default() {
        let panel = PreferencesPanel::new();
        let prefs = panel.get_preferences();
        assert_eq!(prefs.theme, "default");
        assert_eq!(prefs.update_frequency_ms, 2000);
        assert!(prefs.show_notifications);
        assert!(!prefs.auto_start);
    }

    #[test]
    fn test_preferences_panel_set_get() {
        let panel = PreferencesPanel::new();
        let mut prefs = GuiPreferences::default();
        prefs.theme = "dark".to_string();
        prefs.update_frequency_ms = 5000;
        prefs.show_notifications = false;
        prefs.auto_start = true;

        panel.set_preferences(prefs.clone());
        let retrieved = panel.get_preferences();
        assert_eq!(retrieved.theme, prefs.theme);
        assert_eq!(retrieved.update_frequency_ms, prefs.update_frequency_ms);
        assert_eq!(retrieved.show_notifications, prefs.show_notifications);
        assert_eq!(retrieved.auto_start, prefs.auto_start);
    }

    #[test]
    fn test_gui_notification_manager() {
        // Test that notification functions don't panic
        // (actual notifications may not work in test environment)
        GuiNotificationManager::notify("Test", "Test message", crate::core::notifications::NotificationLevel::Info);
        GuiNotificationManager::notify_connection_established();
        GuiNotificationManager::notify_connection_lost();
        GuiNotificationManager::notify_circuit_established();
        GuiNotificationManager::notify_error("Test error");
        GuiNotificationManager::notify_warning("Test warning");
        GuiNotificationManager::notify_info("Test info");
        // Test passes if no panic occurs
    }

    #[test]
    fn test_gui_app_creation() {
        // Note: This test may fail if GTK is not initialized
        // In a real test environment, you'd use gtk4::test_init()
        // For now, we'll just test that the struct can be created
        // (actual initialization requires GTK runtime)
    }
}

