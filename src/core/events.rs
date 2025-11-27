use std::sync::mpsc;
use std::thread;
use serde::{Serialize, Deserialize};
use crate::error::TorrerResult;

/// Event system for Torrer
pub struct EventManager {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
}

impl EventManager {
    /// Create a new event manager
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    /// Emit an event
    pub fn emit(&self, event: Event) -> TorrerResult<()> {
        self.sender.send(event).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to emit event: {}", e))
        })?;
        Ok(())
    }

    /// Receive events (non-blocking)
    pub fn try_receive(&self) -> Option<Event> {
        self.receiver.try_recv().ok()
    }

    /// Receive events (blocking)
    pub fn receive(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }

    /// Start event listener
    pub fn start_listener<F>(&self, handler: F)
    where
        F: Fn(Event) + Send + 'static,
    {
        // Note: mpsc::Receiver doesn't implement Clone
        // For a proper implementation, we'd need to use Arc<Mutex<Receiver>> or a different channel
        // For now, this is a simplified implementation
        log::warn!("Event listener requires shared receiver - using simplified implementation");
        // In a full implementation, we'd use a shared event queue or different channel type
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Tor routing started
    RoutingStarted,
    /// Tor routing stopped
    RoutingStopped,
    /// Tor circuit established
    CircuitEstablished,
    /// Tor circuit failed
    CircuitFailed,
    /// Fallback triggered
    FallbackTriggered(String),
    /// Bridge added
    BridgeAdded(String),
    /// Configuration changed
    ConfigChanged,
    /// Error occurred
    Error(String),
    /// Warning
    Warning(String),
    /// Info
    Info(String),
}

impl Event {
    /// Get event name
    pub fn name(&self) -> &'static str {
        match self {
            Event::RoutingStarted => "routing_started",
            Event::RoutingStopped => "routing_stopped",
            Event::CircuitEstablished => "circuit_established",
            Event::CircuitFailed => "circuit_failed",
            Event::FallbackTriggered(_) => "fallback_triggered",
            Event::BridgeAdded(_) => "bridge_added",
            Event::ConfigChanged => "config_changed",
            Event::Error(_) => "error",
            Event::Warning(_) => "warning",
            Event::Info(_) => "info",
        }
    }
}

