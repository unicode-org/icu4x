// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::convert::TryInto;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::AnyCalendar;
    use icu_calendar::{DateTime, Gregorian};

    use crate::errors::ffi::ICU4XError;
    use crate::locale::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;

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

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::calendar::AnyCalendar, Enum)]
    pub struct ICU4XCalendar(pub Arc<AnyCalendar>);

    impl ICU4XCalendar {
        /// Creates a new [`ICU4XGregorianDateTime`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> DiplomatResult<Box<ICU4XCalendar>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let kind = match (&locale.0).try_into() {
                Ok(k) => k,
                Err(e) => return Err(ICU4XError::from(e)).into(),
            };

            AnyCalendar::try_new_unstable(&provider, kind)
                .map(|df| Box::new(ICU4XCalendar(Arc::new(df))))
                .map_err(Into::into)
                .into()
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateTime object capable of containing a Gregorian date and time.
    #[diplomat::rust_link(icu::calendar::DateTime, Struct)]
    pub struct ICU4XDateTime(pub DateTime<Arc<AnyCalendar>>);

    impl ICU4XDateTime {
        /// Creates a new [`ICU4XDateTime`] representing the ISO date and time
        /// given but in a given calendar
        #[diplomat::rust_link(icu::calendar::DateTime::new_iso_datetime, FnInStruct)]
        pub fn try_new_from_iso_in_calendar(
            year: i32,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: u8,
            calendar: &ICU4XCalendar,
        ) -> DiplomatResult<Box<ICU4XDateTime>, ICU4XError> {
            let cal = calendar.0.clone();
            DateTime::new_iso_datetime(year, month, day, hour, minute, second)
                .map(|dt| Box::new(ICU4XDateTime(dt.to_calendar(cal))))
                .map_err(Into::into)
                .into()
        }
    }
}
