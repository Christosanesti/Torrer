// Unit tests for utility functions

#[cfg(test)]
mod tests {
    use torrer::utils::format;
    use std::time::Duration;

    #[test]
    fn test_format_duration_seconds() {
        let duration = Duration::from_secs(45);
        let formatted = format::format_duration(duration);
        assert_eq!(formatted, "45s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = Duration::from_secs(125);
        let formatted = format::format_duration(duration);
        assert!(formatted.contains("m"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format::format_bytes(0), "0 B");
        assert_eq!(format::format_bytes(1024), "1.00 KB");
        assert_eq!(format::format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format::format_percentage(50.0), "50.00%");
        assert_eq!(format::format_percentage(99.99), "99.99%");
    }
}

