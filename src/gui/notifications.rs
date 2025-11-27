// Notification system for GUI
use notify_rust::Notification;
use crate::core::notifications::NotificationLevel;

/// GUI notification manager
pub struct GuiNotificationManager;

impl GuiNotificationManager {
    /// Show a notification
    pub fn notify(title: &str, message: &str, level: NotificationLevel) {
        let urgency = match level {
            NotificationLevel::Error => notify_rust::Urgency::Critical,
            NotificationLevel::Warning => notify_rust::Urgency::Normal,
            NotificationLevel::Info | NotificationLevel::Success => notify_rust::Urgency::Low,
        };

        // Try to show system notification
        if let Ok(mut notification) = Notification::new() {
            notification
                .summary(title)
                .body(message)
                .urgency(urgency)
                .timeout(notify_rust::Timeout::Milliseconds(5000));
            
            // Ignore errors - notifications are optional
            let _ = notification.show();
        }
    }

    /// Notify about connection state change
    pub fn notify_connection_established() {
        Self::notify(
            "Torrer",
            "Tor connection established",
            NotificationLevel::Success,
        );
    }

    /// Notify about connection lost
    pub fn notify_connection_lost() {
        Self::notify(
            "Torrer",
            "Tor connection lost",
            NotificationLevel::Warning,
        );
    }

    /// Notify about circuit establishment
    pub fn notify_circuit_established() {
        Self::notify(
            "Torrer",
            "Tor circuit established",
            NotificationLevel::Success,
        );
    }

    /// Notify about errors
    pub fn notify_error(message: &str) {
        Self::notify("Torrer Error", message, NotificationLevel::Error);
    }

    /// Notify about warnings
    pub fn notify_warning(message: &str) {
        Self::notify("Torrer Warning", message, NotificationLevel::Warning);
    }

    /// Notify about info
    pub fn notify_info(message: &str) {
        Self::notify("Torrer", message, NotificationLevel::Info);
    }
}





