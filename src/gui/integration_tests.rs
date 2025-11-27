// Integration tests for GUI features
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::gui::*;
    use crate::core::monitoring::{Monitoring, Statistics};
    use crate::tor::circuit::CircuitInfo;
    use std::time::Duration;

    /// Test integration of statistics dashboard with monitoring
    #[test]
    fn test_statistics_dashboard_integration() {
        let mut monitoring = Monitoring::new();
        monitoring.start();
        monitoring.record_bytes_sent(1024);
        monitoring.record_bytes_received(2048);
        monitoring.record_successful_connection();

        let stats = monitoring.get_stats();
        let dashboard = StatisticsDashboard::new();
        dashboard.update_stats(&stats);

        // Verify stats are displayed
        assert!(stats.bytes_sent > 0);
        assert!(stats.bytes_received > 0);
    }

    /// Test integration of circuit visualization with circuit data
    #[test]
    fn test_circuit_visualization_integration() {
        let circuits = vec![
            CircuitInfo {
                id: "12345".to_string(),
                status: "BUILT".to_string(),
                purpose: Some("GENERAL".to_string()),
                flags: Some("FAST STABLE".to_string()),
            },
            CircuitInfo {
                id: "67890".to_string(),
                status: "BUILT".to_string(),
                purpose: Some("HS_CLIENT_HSDIR".to_string()),
                flags: Some("FAST".to_string()),
            },
        ];

        let viz = CircuitVisualization::new();
        viz.update_circuits(circuits.clone());

        // Verify circuits are stored
        let stored = viz.get_circuits();
        assert_eq!(stored.len(), 2);
        assert_eq!(stored[0].id, "12345");
        assert_eq!(stored[1].id, "67890");
    }

    /// Test preferences persistence simulation
    #[test]
    fn test_preferences_persistence() {
        let panel = PreferencesPanel::new();

        // Set preferences
        let mut prefs = GuiPreferences {
            theme: "dark".to_string(),
            update_frequency_ms: 3000,
            show_notifications: true,
            auto_start: true,
        };
        panel.set_preferences(prefs.clone());

        // Retrieve and verify
        let retrieved = panel.get_preferences();
        assert_eq!(retrieved.theme, prefs.theme);
        assert_eq!(retrieved.update_frequency_ms, prefs.update_frequency_ms);
        assert_eq!(retrieved.show_notifications, prefs.show_notifications);
        assert_eq!(retrieved.auto_start, prefs.auto_start);
    }

    /// Test notification system integration
    #[test]
    fn test_notification_system_integration() {
        // Test that notifications can be triggered for different events
        // (actual system notifications may not work in test environment)
        
        // Connection events
        GuiNotificationManager::notify_connection_established();
        GuiNotificationManager::notify_connection_lost();
        
        // Circuit events
        GuiNotificationManager::notify_circuit_established();
        
        // Error events
        GuiNotificationManager::notify_error("Integration test error");
        
        // All should complete without panic
    }

    /// Test real-time update simulation
    #[test]
    fn test_realtime_updates_simulation() {
        let dashboard = StatisticsDashboard::new();
        let mut monitoring = Monitoring::new();
        monitoring.start();

        // Simulate multiple updates
        for i in 0..10 {
            monitoring.record_bytes_sent(1024 * i as u64);
            monitoring.record_bytes_received(2048 * i as u64);
            let stats = monitoring.get_stats();
            dashboard.update_stats(&stats);
        }

        // Verify final state
        let final_stats = monitoring.get_stats();
        assert!(final_stats.bytes_sent > 0);
        assert!(final_stats.bytes_received > 0);
    }

    /// Test GUI component interaction
    #[test]
    fn test_gui_component_interaction() {
        // Test that all GUI components can work together
        let dashboard = StatisticsDashboard::new();
        let viz = CircuitVisualization::new();
        let prefs = PreferencesPanel::new();

        // Update each component
        let stats = Statistics {
            uptime: Some(Duration::from_secs(100)),
            bytes_sent: 5000,
            bytes_received: 10000,
            connection_attempts: 5,
            successful_connections: 5,
            success_rate: 100.0,
        };
        dashboard.update_stats(&stats);

        let circuits = vec![CircuitInfo {
            id: "1".to_string(),
            status: "BUILT".to_string(),
            purpose: Some("GENERAL".to_string()),
            flags: Some("FAST".to_string()),
        }];
        viz.update_circuits(circuits);

        let prefs_data = GuiPreferences::default();
        prefs.set_preferences(prefs_data);

        // All components should work together without conflicts
        let _dashboard_widget = dashboard.widget();
        let _viz_widget = viz.widget();
        let _prefs_widget = prefs.widget();
    }
}

