use env_logger::{Builder, Target};
use std::env;

/// Initialize the logging system
pub fn init_logger() {
    let mut builder = Builder::from_default_env();
    
    // Set default log level if RUST_LOG is not set
    if env::var("RUST_LOG").is_err() {
        builder.filter_level(log::LevelFilter::Info);
    }
    
    // Configure formatting with timestamps
    builder.format(|buf, record| {
        use std::io::Write;
        writeln!(
            buf,
            "[{}] {}: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.args()
        )
    });
    
    // Log to stderr by default
    builder.target(Target::Stderr);
    
    builder.init();
}

/// Initialize structured JSON logging
pub fn init_json_logger() {
    let mut builder = Builder::from_default_env();
    
    if env::var("RUST_LOG").is_err() {
        builder.filter_level(log::LevelFilter::Info);
    }
    
    // JSON format
    builder.format(|buf, record| {
        use std::io::Write;
        let json = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "level": record.level().to_string(),
            "target": record.target(),
            "message": record.args().to_string(),
        });
        
        writeln!(buf, "{}", json)
    });
    
    builder.target(Target::Stderr);
    builder.init();
}


