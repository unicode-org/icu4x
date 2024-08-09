// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use windows::{Globalization, System::UserProfile::GlobalizationPreferences};

use crate::RetrievalError;

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

pub fn get_system_timezone() -> Result<String, RetrievalError> {
    let calendar = Globalization::Calendar::new()?;
    let timezone = calendar.GetTimeZone()?;
    Ok(timezone.to_string_lossy())
}
