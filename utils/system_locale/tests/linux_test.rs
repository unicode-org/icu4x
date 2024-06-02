// tests/locale_parser.rs
#[cfg(target_os = "linux")]
mod tests {
    use system_locale::fetch_locale_settings;

    #[test]
    fn test_fetch_locale_settings() {
        let locale_map = unsafe { fetch_locale_settings() };
        // More meaningful test need to be added
        assert!(locale_map.contains_key(&locale::linux::ffi::LC_ALL));
    }
}

#[cfg(not(target_os = "linux"))]
mod tests {
    #[test]
    #[should_panic(expected = "This function is only supported on Linux.")]
    fn test_fetch_locale_settings_not_supported() {
        locale_parser::fetch_locale_settings();
    }
}
