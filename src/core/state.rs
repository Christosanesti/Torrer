use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::error::TorrerResult;

/// Application state management
pub struct StateManager {
    state: Arc<Mutex<ApplicationState>>,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ApplicationState::default())),
        }
    }

    /// Get current state
    pub fn get_state(&self) -> ApplicationState {
        if let Ok(state) = self.state.lock() {
            state.clone()
        } else {
            ApplicationState::default()
        }
    }

    /// Update state
    pub fn update_state<F>(&self, f: F) -> TorrerResult<()>
    where
        F: FnOnce(&mut ApplicationState),
    {
        if let Ok(mut state) = self.state.lock() {
            f(&mut state);
            Ok(())
        } else {
            Err(crate::error::TorrerError::Config("Failed to lock state".to_string()))
        }
    }

    /// Save state to file
    pub fn save(&self, path: &str) -> TorrerResult<()> {
        let state = self.get_state();
        let content = toml::to_string_pretty(&state).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to serialize state: {}", e))
        })?;
        
        std::fs::write(path, content).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to write state: {}", e))
        })?;
        
        Ok(())
    }

    /// Load state from file
    pub fn load(&self, path: &str) -> TorrerResult<()> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to read state: {}", e))
        })?;
        
        let state: ApplicationState = toml::from_str(&content).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to parse state: {}", e))
        })?;
        
        if let Ok(mut current_state) = self.state.lock() {
            *current_state = state;
        }
        
        Ok(())
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationState {
    pub is_running: bool,
    pub start_time: Option<u64>,
    pub last_update: u64,
    pub connection_count: u32,
    pub fallback_count: u32,
    pub current_country: Option<String>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            is_running: false,
            start_time: None,
            last_update: crate::utils::current_timestamp(),
            connection_count: 0,
            fallback_count: 0,
            current_country: None,
        }
    }
}

