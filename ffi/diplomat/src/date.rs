// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::AnyCalendar;
    use icu_calendar::{Date, Iso};

    use crate::calendar::ffi::ICU4XCalendar;
    use crate::errors::ffi::ICU4XError;

    #[diplomat::opaque]
    /// An ICU4X Date object capable of containing a ISO-8601 date
    #[diplomat::rust_link(icu::calendar::Date, Struct)]
    pub struct ICU4XIsoDate(pub Date<Iso>);

    impl ICU4XIsoDate {
        /// Creates a new [`ICU4XIsoDate`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::Date::new_gregorian_date, FnInStruct)]
        pub fn try_new(
            year: i32,
            month: u8,
            day: u8,
        ) -> DiplomatResult<Box<ICU4XIsoDate>, ICU4XError> {
            Date::new_iso_date(year, month, day)
                .map(|dt| Box::new(ICU4XIsoDate(dt)))
                .map_err(Into::into)
                .into()
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Date object capable of containing a date and time for any calendar.
    #[diplomat::rust_link(icu::calendar::Date, Struct)]
    pub struct ICU4XDate(pub Date<Arc<AnyCalendar>>);

    impl ICU4XDate {
        /// Creates a new [`ICU4XDate`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::calendar::Date::new_iso_date, FnInStruct)]
        pub fn try_new_from_iso_in_calendar(
            year: i32,
            month: u8,
            day: u8,
            calendar: &ICU4XCalendar,
        ) -> DiplomatResult<Box<ICU4XDate>, ICU4XError> {
            let cal = calendar.0.clone();
            Date::new_iso_date(year, month, day)
                .map(|dt| Box::new(ICU4XDate(dt.to_calendar(cal))))
                .map_err(Into::into)
                .into()
        }
    }
}
