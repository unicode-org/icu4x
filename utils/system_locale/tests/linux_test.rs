mod tests {
    #[test]
    fn test_fetch_locale_settings() {
        let locales = system_locale::get_locale();
        // More meaningful test need to be added
        assert!(!locales.is_empty(), "Locale list should not be empty");
    }
}
