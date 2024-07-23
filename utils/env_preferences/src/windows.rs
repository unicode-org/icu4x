// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod windows_locale {
    use windows::{Globalization, System::UserProfile::GlobalizationPreferences};
    pub fn get_locales() -> Vec<String> {
        let mut locale_vec_str: Vec<String> = Vec::new();
        let locale = GlobalizationPreferences::Languages();

        match locale {
            Ok(languages) => {
                for i in 0..languages.Size().unwrap() {
                    let hstring = languages.GetAt(i).unwrap();
                    let string: String = hstring.to_string_lossy();
                    locale_vec_str.push(string);
                }
            }
            Err(_e) => return Vec::new(),
        }
        locale_vec_str
    }

    pub fn get_system_calendars() -> Vec<(String, String)> {
        let calendar = Globalization::Calendar::new().unwrap();
        let system_calendar = Globalization::Calendar::GetCalendarSystem(&calendar).unwrap();
        let calendar_type: String = system_calendar.to_string();
        let locale_list: Vec<String> = get_locale();

        let result: Vec<(String, String)> = locale_list
            .into_iter()
            .map(|locale| (locale, calendar_type.clone()))
            .collect();

        result
    }
}
