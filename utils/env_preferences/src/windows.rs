// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use windows::{Globalization, System::UserProfile::GlobalizationPreferences};

use crate::RetrievalError;

/// Retrieves languages preffered by the user , it consumes [`GlobalizationPreferences::Languages`](https://learn.microsoft.com/en-us/uwp/api/windows.system.userprofile.globalizationpreferences.languages?view=winrt-26100)
/// 
/// ### Example 
/// 
/// ```rust
/// use env_preferences::get_locales;
/// let locales: Vec<String> = get_locales().unwrap();
/// ```
pub fn get_locales() -> Result<Vec<String>, RetrievalError> {
    let mut locale_vec_str: Vec<String> = Vec::new();
    let locale = GlobalizationPreferences::Languages()?;

    for i in 0..locale.Size()? {
        let hstring = locale.GetAt(i)?;
        let string = hstring.to_string_lossy();
        locale_vec_str.push(string);
    }
    Ok(locale_vec_str)
}

/// Gets the list calendar type and it's corresponding locale. It returns a Vec<(String, String)>
/// The first element is the locale of the calendar, second is the calendar identifier
/// 
/// ### Example
/// 
/// ```rust
/// use env_preferences::get_system_calendars;
/// let system_calendars = get_system_calendars().unwrap();
/// for system_calendar in system_calendars {
///     let locale = system_calendar.0;
///     let identifier = system_calendar.1;
/// }
/// ```
pub fn get_system_calendars() -> Result<Vec<(String, String)>, RetrievalError> {
    let calendar = Globalization::Calendar::new()?;
    let system_calendar = Globalization::Calendar::GetCalendarSystem(&calendar)?;
    let calendar_type: String = system_calendar.to_string();
    let locale_list: Vec<String> = get_locales()?;

    let result: Vec<(String, String)> = locale_list
        .into_iter()
        .map(|locale| (locale, calendar_type.clone()))
        .collect();

    Ok(result)
}

/// Get the current time zone of the system
/// ### Example 
/// ```rust
/// use env_preferences::get_system_timezone;
/// let time_zone = get_system_timezone().unwrap();
/// ```
pub fn get_system_timezone() -> Result<String, RetrievalError> {
    let calendar = Globalization::Calendar::new()?;
    let timezone = calendar.GetTimeZone()?;
    Ok(timezone.to_string_lossy())
}
