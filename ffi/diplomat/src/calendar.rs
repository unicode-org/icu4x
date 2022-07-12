// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::{DateTime, Gregorian};

    use crate::errors::ffi::ICU4XError;

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a Gregorian date and time.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XGregorianDateTime(pub DateTime<Gregorian>);

    impl ICU4XGregorianDateTime {
        /// Creates a new [`ICU4XGregorianDateTime`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::DateTime::new_gregorian_datetime, FnInStruct)]
        pub fn try_new(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
        ) -> DiplomatResult<Box<ICU4XGregorianDateTime>, ICU4XError> {
            DateTime::new_gregorian_datetime(year, month, day, hour, minute, second)
                .map(|dt| Box::new(ICU4XGregorianDateTime(dt)))
                .map_err(Into::into)
                .into()
        }
    }
}
