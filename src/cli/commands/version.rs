use crate::error::TorrerResult;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Torrer Contributors";
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Display version information
pub fn show_version() -> TorrerResult<()> {
    println!("Torrer {}", VERSION);
    println!("{}", DESCRIPTION);
    println!("Author: {}", AUTHOR);
    println!();
    println!("Built with Rust {}", get_rust_version());
    Ok(())
}

fn get_rust_version() -> String {
    std::env::var("RUSTC_VERSION")
        .unwrap_or_else(|_| "unknown".to_string())
}

