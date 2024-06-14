mod tests {
    #[test]
    // More meaningful test need to be added
    fn test_get_locale() {
        let locales = system_locale::get_locale();
        assert!(!locales.is_empty(), "Locale list should not be empty");
    }

    #[test]
    fn test_get_calendar() {
        let system_calendar = system_locale::get_system_calendar();
        assert!(!system_calendar.is_empty(), "Calendar should not be emptys")
    }
}
