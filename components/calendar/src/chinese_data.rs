//! Compiled and compressed data for Chinese calendar years; includes `RataDie` types for the beginning of years
//! TODO: More docs

/// The minimum year present in CHINESE_DATA_ARRAY
pub(crate) const MIN_YEAR: i32 = 4560;

/// The maximum year present in CHINESE_DATA_ARRAY
pub(crate) const MAX_YEAR: i32 = 4760;

/// The array of year data for Chinese years between MIN_YEAR and MAX_YEAR
/// 
/// Each data entry is an i32 storing 24 bits of information with 5 bits available for later use.
/// The first 5 bits are currently unused;
/// the next 6 bits represent which day of the ISO year marks the Chinese New Year (ex. for 4660 (ISO 2023), New Year was Jan. 22, so the value would be 22);
/// the next 12 bits correspond to the month lengths of the first 12 ordinal months in the year, 0 meaning 29 days long, and 1 meaning 30;
/// the next 2 bits are empty bits, followed by 1 bit representing the length of the 13th month in a similar way;
/// the final 6 bits store which ordinal month is a leap month (2..=13), or equals zero if there is no leap month.
pub(crate) const CHINESE_DATA_ARRAY: [i32; 200] = [

];