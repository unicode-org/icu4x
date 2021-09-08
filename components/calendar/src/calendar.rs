// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Date, DateDuration, DurationUnit, Iso};

/// A calendar implementation
///
/// Only implementors of [`Calendar`] should care about these methods, in general users of
/// these calendars should use the methods on [`Date`] instead.
///
/// Individual [`Calendar`] implementations may have inherent utility methods
/// allowing for direct construction, etc.
pub trait Calendar {
    type DateInner: PartialEq + Eq + Clone;
    /// Construct the date from an ISO date
    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner;
    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso>;
    // fn validate_date(&self, e: Era, y: Year, m: MonthCode, d: Day) -> bool;
    // // similar validators for YearMonth, etc

    // fn is_leap<A: AsCalendar<Calendar = Self>>(&self, date: &Date<A>) -> bool;
    fn months_in_year(&self, date: &Self::DateInner) -> u8;
    fn days_in_year(&self, date: &Self::DateInner) -> u32;
    fn days_in_month(&self, date: &Self::DateInner) -> u8;
    /// Monday is 1, Sunday is 7, according to ISO
    fn day_of_week(&self, date: &Self::DateInner) -> u8 {
        self.date_to_iso(date).day_of_week()
    }
    // fn week_of_year(&self, date: &Self::DateInner) -> u8;

    /// Add `offset` to `date`
    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>);

    /// Calculate `date2 - date` as a duration
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        largest_unit: DurationUnit,
        smallest_unit: DurationUnit,
    ) -> DateDuration<Self>;

    fn debug_name() -> &'static str;
    // fn since(&self, from: &Date<Self>, to: &Date<Self>) -> Duration<Self>, Error;
}
