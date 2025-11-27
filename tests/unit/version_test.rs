// Unit tests for version utilities

#[cfg(test)]
mod tests {
    use torrer::utils::Version;

    #[test]
    fn test_version_compare_equal() {
        assert_eq!(Version::compare("1.0.0", "1.0.0"), Some(std::cmp::Ordering::Equal));
        assert_eq!(Version::compare("2.5.3", "2.5.3"), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn test_version_compare_newer() {
        assert_eq!(Version::compare("1.0.1", "1.0.0"), Some(std::cmp::Ordering::Greater));
        assert_eq!(Version::compare("2.0.0", "1.9.9"), Some(std::cmp::Ordering::Greater));
    }

    #[test]
    fn test_version_compare_older() {
        assert_eq!(Version::compare("1.0.0", "1.0.1"), Some(std::cmp::Ordering::Less));
        assert_eq!(Version::compare("1.9.9", "2.0.0"), Some(std::cmp::Ordering::Less));
    }

    #[test]
    fn test_is_newer() {
        assert!(Version::is_newer("1.0.1", "1.0.0"));
        assert!(!Version::is_newer("1.0.0", "1.0.1"));
        assert!(!Version::is_newer("1.0.0", "1.0.0"));
    }
}

