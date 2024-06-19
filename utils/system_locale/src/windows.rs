#[cfg(target_os = "windows")]
mod windows_locale {
    use windows::System::UserProfile::GlobalizationPreferences;
    pub fn get_locale() -> Vec<String> {
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

    pub fn get_system_calendar() -> Vec<(String, String)> {
        Vec::new()
    }
}

#[cfg(target_os = "windows")]
pub use windows_locale::get_locale;
pub use windows_locale::get_system_calendar;
