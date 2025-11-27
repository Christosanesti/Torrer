use crate::error::TorrerResult;
use crate::core::StateManager;

/// Show application state
pub fn show_state() -> TorrerResult<()> {
    let state_manager = StateManager::new();
    let state = state_manager.get_state();

    println!("=== Torrer Application State ===");
    println!();
    println!("Running: {}", state.is_running);
    
    if let Some(start_time) = state.start_time {
        use crate::utils::format_timestamp;
        println!("Start time: {}", format_timestamp(start_time));
        
        use crate::utils::elapsed_since;
        let elapsed = elapsed_since(start_time);
        use crate::utils::format_duration;
        println!("Uptime: {}", format_duration(elapsed));
    }
    
    println!("Connection count: {}", state.connection_count);
    println!("Fallback count: {}", state.fallback_count);
    
    if let Some(ref country) = state.current_country {
        println!("Current country: {}", country);
    }
    
    Ok(())
}

/// Save state to file
pub fn save_state(path: &str) -> TorrerResult<()> {
    let state_manager = StateManager::new();
    state_manager.save(path)?;
    println!("State saved to: {}", path);
    Ok(())
}

/// Load state from file
pub fn load_state(path: &str) -> TorrerResult<()> {
    let state_manager = StateManager::new();
    state_manager.load(path)?;
    println!("State loaded from: {}", path);
    Ok(())
}

