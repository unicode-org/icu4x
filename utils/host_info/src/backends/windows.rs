// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    backends::{HostInfoBackend, RawHostInfoBackend},
    error::HostInfoError,
    locale::WindowsLocale,
};
use icu_locale_core::{
    extensions::unicode, preferences::extensions::unicode::keywords::CalendarAlgorithm, Locale,
};
use std::str::FromStr;

pub struct WindowsHostInfoBackend;

impl HostInfoBackend for WindowsHostInfoBackend {
    fn requested_locales() -> Result<Vec<Locale>, HostInfoError> {
        Ok(Self::raw_requested_locales()?
            .into_iter()
            .filter_map(|s| {
                WindowsLocale::try_from_str(&s)
                    .map_err(|_| HostInfoError::HostLocaleError)
                    .and_then(|wl| Locale::try_from(wl).map_err(Into::into))
                    .ok()
            })
            .collect())
    }

    fn calendar() -> Result<Option<CalendarAlgorithm>, HostInfoError> {
        Ok(Self::raw_calendar()?
            .and_then(|raw| {
                let canonical = match raw.as_str() {
                    "GregorianCalendar" => "gregory",
                    "JapaneseCalendar" => "japanese",
                    "TaiwanCalendar" => "roc",
                    "KoreanCalendar" => "dangi",
                    "HebrewCalendar" => "hebrew",
                    "HijriCalendar" => "islamic",
                    "UmmAlQuraCalendar" => "islamic-umalqura",
                    "PersianCalendar" => "persian",
                    "ThaiCalendar" => "buddhist",
                    "JulianCalendar" => "julian",
                    r => r,
                };
                unicode::Value::from_str(canonical).ok()
            })
            .and_then(|value| CalendarAlgorithm::try_from(&value).ok()))
    }
}

impl RawHostInfoBackend for WindowsHostInfoBackend {
    fn raw_region() -> Result<Option<String>, HostInfoError> {
        let region =
            windows::System::UserProfile::GlobalizationPreferences::HomeGeographicRegion()?;
        let s = region.to_string_lossy();
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }

    fn raw_requested_locales() -> Result<Vec<String>, HostInfoError> {
        let locale = windows::System::UserProfile::GlobalizationPreferences::Languages()?;
        let len = locale.Size()?;

        let mut locale_vec_str: Vec<String> = Vec::with_capacity(len as usize);

        for i in 0..len {
            let hstring = locale.GetAt(i)?;
            let string = hstring.to_string_lossy();
            locale_vec_str.push(string);
        }
        Ok(locale_vec_str)
    }

    fn raw_calendar() -> Result<Option<String>, HostInfoError> {
        let calendar = ::windows::Globalization::Calendar::new()?;
        let system_calendar = ::windows::Globalization::Calendar::GetCalendarSystem(&calendar)?;
        let calendar_type: String = system_calendar.to_string();
        Ok(Some(calendar_type))
    }

    fn raw_first_day_of_week() -> Result<Option<String>, HostInfoError> {
        Ok(
            match ::windows::System::UserProfile::GlobalizationPreferences::WeekStartsOn()?.0 {
                0 => Some("sun".to_string()),
                1 => Some("mon".to_string()),
                2 => Some("tue".to_string()),
                3 => Some("wed".to_string()),
                4 => Some("thu".to_string()),
                5 => Some("fri".to_string()),
                6 => Some("sat".to_string()),
                _ => None,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::backends::{windows::WindowsHostInfoBackend, RawHostInfoBackend};
    use crate::locale::WindowsLocale;
    use icu_locale_core::Locale;
    use std::sync::{LazyLock, Mutex};
    use windows::core::{BOOL, PCWSTR};
    use windows::Win32::{
        Foundation::LPARAM,
        Globalization::{EnumSystemLocalesEx, LOCALE_ALL},
    };

    // Since [`EnumSystemLocalesEx`] iterates using a callback with no obvious (safe) way to return data,
    // store them in this static instead. Since this is only a single test with roughly 1,000 items,
    // it shouldn't be much of a concern.
    static LOCALES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

    /// Callback provided to the [`EnumSystemLocalesEx`] to enumerate over locales.
    unsafe extern "system" fn callback(
        locale_name: PCWSTR,
        _locale_flags: u32,
        _callback_parameter: LPARAM,
    ) -> BOOL {
        // SAFETY: caller is the [`EnumSystemLocalesEx`] function, which guarantees a valid null-terminated string
        let locale_name = unsafe { locale_name.to_string() }.unwrap();

        // Skip empty locale 0x007F, marked as "Reserved for invariant locale behavior"
        // Source: MS-LCID version 16.0, page 13 (section 2.2 under "Language ID" table)
        if !locale_name.is_empty() {
            LOCALES.lock().unwrap().push(locale_name);
        }

        // Tell [`EnumSystemLocalesEx`] to continue enumeration
        BOOL::from(true)
    }

    /// Enumerate over all Windows locales, and make sure [`WindowsLocale`] can parse it without any (direct) errors.
    #[test]
    fn system_locales() -> windows::core::Result<()> {
        // Find the list of supported locales, using the [`EnumSystemLocalesEx`] API:
        // https://learn.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-enumsystemlocalesex
        // SAFETY: a valid function pointer is provided and lpReserved is set to NULL/None as required
        unsafe {
            EnumSystemLocalesEx(Some(callback), LOCALE_ALL, LPARAM::default(), None)?;
        }

        // Get the list of locales which the callback has been modifying
        let locales = LOCALES.lock().unwrap();

        // Make sure [`WindowsLocale`] can parse without any obvious issues
        for locale in locales.iter() {
            let windows_locale = WindowsLocale::try_from_str(locale).expect(locale);
            Locale::try_from(windows_locale).expect(locale);
        }

        Ok(())
    }

    #[test]
    fn test_get_raw_requested_locales() {
        let locales = WindowsHostInfoBackend::raw_requested_locales().unwrap();
        for locale in locales {
            assert!(!locale.is_empty(), "Empty locale retrieved");
            assert!(locale.is_ascii(), "Invalid form of locale retrieved");
        }
    }

    #[test]
    fn test_converting_locales() {
        let locales = WindowsHostInfoBackend::raw_requested_locales().unwrap();
        for locale in locales {
            let _converted_locale: Locale = locale.parse().unwrap();
        }
    }

    #[test]
    fn test_calendar() {
        let calendar = WindowsHostInfoBackend::raw_calendar().unwrap().unwrap();
        assert!(!calendar.is_empty(), "Calendar identifier is empty");
        assert!(calendar.is_ascii(), "Calendar identifier form is not valid");
    }
}
