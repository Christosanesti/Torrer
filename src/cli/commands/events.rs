use crate::error::TorrerResult;
use crate::core::EventManager;

/// Monitor events
pub fn monitor_events() -> TorrerResult<()> {
    println!("Monitoring Torrer events...");
    println!("Press Ctrl+C to stop");
    println!();

    let event_manager = EventManager::new();

    // Start event listener
    event_manager.start_listener(|event| {
        use chrono::Local;
        let timestamp_str = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        println!("[{}] {}: {:?}", timestamp_str, event.name(), event);
    });

    // Keep running
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/// Show recent events
pub fn show_events(count: usize) -> TorrerResult<()> {
    println!("Recent events (last {}):", count);
    println!();

    let event_manager = EventManager::new();
    let mut events = Vec::new();
    
    // Collect recent events (non-blocking)
    for _ in 0..count {
        if let Some(event) = event_manager.try_receive() {
            events.push(event);
        } else {
            break;
        }
    }
    
    if events.is_empty() {
        println!("No recent events.");
        println!();
        println!("Events are logged when:");
        println!("  - Tor routing starts/stops");
        println!("  - Circuits are established/failed");
        println!("  - Fallback is triggered");
        println!("  - Bridges are added/removed");
        println!("  - Configuration changes");
        println!();
        println!("Use 'torrer logs' to view application logs.");
    } else {
        use chrono::Local;
        for (index, event) in events.iter().enumerate() {
            let timestamp_str = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            println!("  {}. [{}] {}: {:?}", 
                index + 1, 
                timestamp_str, 
                event.name(), 
                event
            );
        }
    }

    Ok(())
}

