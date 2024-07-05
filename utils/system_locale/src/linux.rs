mod linux_locale {
    use std::{
        borrow::Cow,
        collections::{HashMap, HashSet},
        ffi::CStr,
        ptr,
    };

    use libc::{setlocale, LC_ALL, LC_TIME};

    fn fetch_locale_settings() -> HashMap<String, String> {
        let mut locale_map = HashMap::new();

        // Thread safety is ensured by fallbacking to the default locale of `linux` which is `C`
        unsafe {
            let locales_ptr = setlocale(LC_ALL, ptr::null());
            let locales = CStr::from_ptr(locales_ptr);

            if let Ok(locales_str) = locales.to_str() {
                let locales_slice = locales_str.split(';');
                for locale in locales_slice {
                    let mut locale_parts = locale.split('=');
                    if let (Some(key), Some(value)) = (locale_parts.next(), locale_parts.next()) {
                        locale_map.insert(key.to_string(), value.to_string());
                    }
                }
            } else {
                locale_map.insert(String::from("LC_ALL"), String::from("C"));
            }
        }

        locale_map
    }

    pub fn get_locales() -> Vec<String> {
        let mut unique_locales = HashSet::new();
        let locale_map = fetch_locale_settings();
        for value in locale_map.values() {
            unique_locales.insert(value.clone());
        }

        unique_locales.into_iter().collect()
    }

    pub fn get_system_calendars() -> Box<dyn Iterator<Item = (Cow<'static, str>, Cow<'static, str>)>>
    {
        unsafe {
            let locale_ptr = setlocale(LC_TIME, ptr::null());
            if !locale_ptr.is_null() {
                let c_str = CStr::from_ptr(locale_ptr);
                if let Ok(str_slice) = c_str.to_str() {
                    return Box::new(
                        Some((
                            Cow::Owned(str_slice.to_string()),
                            Cow::Borrowed("Gregorian"),
                        ))
                        .into_iter(),
                    );
                }
            }
            Box::new(
                None.into_iter()
                    .chain(Some((Cow::Borrowed("C"), Cow::Borrowed("Gregorian"))).into_iter()),
            )
        }
    }
}
