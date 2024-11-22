use crate::iso::is_leap_year;

type IsoWeekday = u8;

/// Prev algo from: `components\calendar\src\iso.rs`
// In the code removed: `date.0.`
// Next line WAS: `fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {`
pub fn day_of_week(year: i32, month: u8, day: u8) -> IsoWeekday {
    // For the purposes of the calculation here, Monday is 0, Sunday is 6
    // ISO has Monday=1, Sunday=7, which we transform in the last step

    // The days of the week are the same every 400 years
    // so we normalize to the nearest multiple of 400
    let years_since_400 = year.rem_euclid(400);
    debug_assert!(years_since_400 >= 0); // rem_euclid returns positive numbers
    let years_since_400 = years_since_400 as u32;
    let leap_years_since_400 = years_since_400 / 4 - years_since_400 / 100;
    // The number of days to the current year
    // Can never cause an overflow because years_since_400 has a maximum value of 399.
    let days_to_current_year = 365 * years_since_400 + leap_years_since_400;
    // The weekday offset from January 1 this year and January 1 2000
    let year_offset = days_to_current_year % 7;

    // Corresponding months from
    // https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Corresponding_months
    let month_offset = if is_leap_year(year) {
        match month {
            10 => 0,
            5 => 1,
            2 | 8 => 2,
            3 | 11 => 3,
            6 => 4,
            9 | 12 => 5,
            1 | 4 | 7 => 6,
            _ => unreachable!(),
        }
    } else {
        match month {
            1 | 10 => 0,
            5 => 1,
            8 => 2,
            2 | 3 | 11 => 3,
            6 => 4,
            9 | 12 => 5,
            4 | 7 => 6,
            _ => unreachable!(),
        }
    };
    let january_1_2000 = 5; // Saturday
    let day_offset = (january_1_2000 + year_offset + month_offset + day as u32) % 7;

    // We calculated in a zero-indexed fashion, but ISO specifies one-indexed
    (day_offset + 1) as u8
}

/// Count the number of days in a given month/year combo
const fn days_in_month(year: i32, month: u8) -> u8 {
    // see comment to `<impl CalendarArithmetic for Iso>::month_days`
    match month {
        2 => 28 | (is_leap_year(year) as u8),
        _ => 30 | (month ^ (month >> 3)),
    }
}

/// Prev algo from: `components\calendar\src\iso.rs::Iso`
///
/// Return `(day, month)` for the given `year & `day_of_year`
pub fn iso_from_year_day(year: i32, year_day: u16) -> (u8, u8) {
    let mut month = 1;
    let mut day = year_day as i32;
    while month <= 12 {
        let month_days = days_in_month(year, month) as i32;
        if day <= month_days {
            break;
        } else {
            debug_assert!(month < 12); // don't try going to month 13
            day -= month_days;
            month += 1;
        }
    }
    let day = day as u8; // day <= month_days < u8::MAX

    (month, day)
}
