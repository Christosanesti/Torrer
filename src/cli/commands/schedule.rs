use crate::error::TorrerResult;
use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};

const TASKS_FILE: &str = "/var/lib/torrer/scheduled_tasks.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScheduledTask {
    name: String,
    interval_seconds: u64,
    enabled: bool,
}

/// List scheduled tasks
pub fn list_tasks() -> TorrerResult<()> {
    println!("Scheduled Tasks:");
    println!();

    let tasks = load_tasks()?;
    
    if tasks.is_empty() {
        println!("No scheduled tasks configured.");
        println!();
        println!("To add a task, use:");
        println!("  torrer schedule add <name> <interval_seconds>");
        println!();
        println!("Example:");
        println!("  torrer schedule add bridge-collection 86400  # Daily bridge collection");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            println!("  {}. {} (interval: {}s, enabled: {})", 
                index + 1, 
                task.name, 
                task.interval_seconds,
                task.enabled
            );
        }
    }

    Ok(())
}

/// Add a scheduled task
pub fn add_task(name: &str, interval_seconds: u64) -> TorrerResult<()> {
    println!("Adding scheduled task: {} (interval: {}s)", name, interval_seconds);

    let mut tasks = load_tasks()?;
    
    // Check if task already exists
    if tasks.iter().any(|t| t.name == name) {
        return Err(crate::error::TorrerError::Config(
            format!("Task '{}' already exists", name)
        ));
    }
    
    let new_task = ScheduledTask {
        name: name.to_string(),
        interval_seconds,
        enabled: true,
    };
    
    tasks.push(new_task);
    save_tasks(&tasks)?;
    
    println!("✓ Task '{}' added successfully", name);
    println!("  Interval: {} seconds ({} minutes)", 
        interval_seconds, 
        interval_seconds / 60
    );

    Ok(())
}

/// Remove a scheduled task
pub fn remove_task(name: &str) -> TorrerResult<()> {
    println!("Removing scheduled task: {}", name);
    
    let mut tasks = load_tasks()?;
    let initial_len = tasks.len();
    
    tasks.retain(|t| t.name != name);
    
    if tasks.len() == initial_len {
        return Err(crate::error::TorrerError::Config(
            format!("Task '{}' not found", name)
        ));
    }
    
    save_tasks(&tasks)?;
    println!("✓ Task '{}' removed successfully", name);
    
    Ok(())
}

/// Load tasks from file
fn load_tasks() -> TorrerResult<Vec<ScheduledTask>> {
    let tasks_path = PathBuf::from(TASKS_FILE);
    
    if !tasks_path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&tasks_path).map_err(|e| {
        crate::error::TorrerError::Config(format!("Failed to read tasks file: {}", e))
    })?;
    
    let tasks: Vec<ScheduledTask> = toml::from_str(&content).map_err(|e| {
        crate::error::TorrerError::Config(format!("Failed to parse tasks file: {}", e))
    })?;
    
    Ok(tasks)
}

/// Save tasks to file
fn save_tasks(tasks: &[ScheduledTask]) -> TorrerResult<()> {
    let tasks_path = PathBuf::from(TASKS_FILE);
    
    // Ensure directory exists
    if let Some(parent) = tasks_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            crate::error::TorrerError::Config(format!("Failed to create tasks directory: {}", e))
        })?;
    }
    
    let content = toml::to_string_pretty(tasks).map_err(|e| {
        crate::error::TorrerError::Config(format!("Failed to serialize tasks: {}", e))
    })?;
    
    fs::write(&tasks_path, content).map_err(|e| {
        crate::error::TorrerError::Config(format!("Failed to write tasks file: {}", e))
    })?;
    
    Ok(())
}

