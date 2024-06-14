#[cfg(target_os = "linux")]
mod linux_locale {
    use locale::{
        self,
        linux::ffi::{
            LC_ADDRESS, LC_ALL, LC_COLLATE, LC_CTYPE, LC_IDENTIFICATION, LC_MEASUREMENT,
            LC_MESSAGES, LC_MONETARY, LC_NAME, LC_NUMERIC, LC_PAPER, LC_TELEPHONE, LC_TIME,
        },
    };
    use std::{
        collections::{HashMap, HashSet},
        ffi::CStr,
        ptr::null,
    };

    pub unsafe fn fetch_locale_settings() -> HashMap<i32, String> {
        let locale_ptr = locale::linux::ffi::setlocale(LC_ALL, null());
        let c_str: &CStr = CStr::from_ptr(locale_ptr as *const u8);
        let mut locale_map = HashMap::new();

        if let Ok(str_slice) = c_str.to_str() {
            if str_slice.contains(';') {
                for part in str_slice.split(';') {
                    let mut splitted = part.split('=');
                    if let (Some(key), Some(value)) = (splitted.next(), splitted.next()) {
                        let key_constant = match key {
                            "LC_CTYPE" => LC_CTYPE,
                            "LC_NUMERIC" => LC_NUMERIC,
                            "LC_TIME" => LC_TIME,
                            "LC_COLLATE" => LC_COLLATE,
                            "LC_MONETARY" => LC_MONETARY,
                            "LC_MESSAGES" => LC_MESSAGES,
                            "LC_PAPER" => LC_PAPER,
                            "LC_NAME" => LC_NAME,
                            "LC_ADDRESS" => LC_ADDRESS,
                            "LC_TELEPHONE" => LC_TELEPHONE,
                            "LC_MEASUREMENT" => LC_MEASUREMENT,
                            "LC_IDENTIFICATION" => LC_IDENTIFICATION,
                            _ => LC_ALL,
                        };
                        locale_map.insert(key_constant, value.to_string());
                    }
                }
            } else {
                locale_map.insert(LC_ALL, str_slice.to_string());
            }
        }

        locale_map
    }

    pub fn get_loclae() -> Vec<String> {
        let mut unique_locales = HashSet::new();
        unsafe {
            let locale_map = fetch_locale_settings();
            for value in locale_map.values() {
                unique_locales.insert(value.clone());
            }
        }
        unique_locales.into_iter().collect()
    }

    pub fn get_system_calendar() -> Vec<(String, String)> {
        unsafe {
            let locale_ptr = locale::linux::ffi::setlocale(LC_TIME, null());
            if !locale_ptr.is_null() {
                let c_str = CStr::from_ptr(locale_ptr);
                if let Ok(str_slice) = c_str.to_str() {
                    return vec![(str_slice.to_string(), String::from("Gregorian"))];
                }
            }
        }
        Vec::new() // Return an empty vector if there is an error
    }
}

#[cfg(target_os = "linux")]
pub use linux_locale::get_loclae;
pub use linux_locale::get_system_calendar;
