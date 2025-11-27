use std::time::{Duration, SystemTime};
use crate::error::TorrerResult;
use tokio::time::interval;

/// Task scheduler for periodic operations
pub struct Scheduler {
    tasks: Vec<ScheduledTask>,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    /// Add a scheduled task
    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.push(task);
    }

    /// Start scheduler
    pub async fn start(self) -> TorrerResult<()> {
        log::info!("Starting scheduler with {} tasks", self.tasks.len());

        for task in self.tasks {
            let task_name = task.name.clone();
            let task_interval = task.interval;
            let task_handler = task.handler;
            
            tokio::spawn(async move {
                let mut interval_timer = interval(task_interval);
                loop {
                    interval_timer.tick().await;
                    log::debug!("Running scheduled task: {}", task_name);
                    (task_handler)().await;
                }
            });
        }

        Ok(())
    }

    /// Run all tasks once
    pub async fn run_once(&self) -> TorrerResult<()> {
        for task in &self.tasks {
            (task.handler)().await;
        }
        Ok(())
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduled task
pub struct ScheduledTask {
    pub name: String,
    pub interval: Duration,
    pub handler: Box<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>,
}

// Note: ScheduledTask cannot implement Clone due to the handler trait object
// Tasks should be created fresh rather than cloned

/// Task builder
pub struct TaskBuilder {
    name: String,
    interval: Duration,
}

impl TaskBuilder {
    /// Create a new task builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            interval: Duration::from_secs(60),
        }
    }

    /// Set interval
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Build task
    pub fn build<F, Fut>(self, handler: F) -> ScheduledTask
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        ScheduledTask {
            name: self.name,
            interval: self.interval,
            handler: Box::new(move || Box::pin(handler())),
        }
    }
}

