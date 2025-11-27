use std::process::Command;

/// Check if running as root
pub fn is_root() -> bool {
    std::env::var("USER").unwrap_or_default() == "root" || 
    Command::new("id")
        .arg("-u")
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout).trim() == "0"
        })
        .unwrap_or(false)
}

/// Check if command exists
pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get system information
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: get_os_name(),
        kernel: get_kernel_version(),
        is_root: is_root(),
    }
}

fn get_os_name() -> String {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line.split('=').nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Unknown".to_string()
}

fn get_kernel_version() -> String {
    Command::new("uname")
        .arg("-r")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub is_root: bool,
}

