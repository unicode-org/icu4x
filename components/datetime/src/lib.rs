// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Formatting date and time.
//!
//! This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! [`TypedDateTimeFormatter`] and [`DateTimeFormatter`] are the main types of the component. They accepts a set of arguments which
//! allow it to collect necessary data from the [data provider], and once instantiated, can be
//! used to quickly format any date and time provided. There are variants of these types that can format greater or fewer components,
//! including [`TypedDateFormatter`] & [`DateFormatter`], [`TypedZonedDateTimeFormatter`] & [`ZonedDateTimeFormatter`], [`TimeFormatter`],
//! and [`TimeZoneFormatter`]
//!
//! These formatters work with types from the [`calendar`] module, like [`Date`], [`DateTime`], and [`Time`],
//! and [`timezone::CustomTimeZone`], however other types may be used provided they implement the traits from the [`input`] module.
//!
//! Each instance of a date-related formatter (i.e. not [`TimeFormatter`] or [`TimeZoneFormatter`]
//! is associated with a particular [`Calendar`].
//! The "Typed" vs untyped formatter distinction is to help with this. For example, if you know at compile time that you
//! will only be formatting Gregorian dates, you can use [`TypedDateTimeFormatter<Gregorian>`](TypedDateTimeFormatter) and the
//! APIs will make sure that only Gregorian [`DateTime`]s are used with the calendar. On the other hand, if you want to be able to select
//! the calendar at runtime, you can use [`DateTimeFormatter`] with the calendar specified in the locale, and use it with
//! [`DateTime`],[`AnyCalendar`]. These formatters still require dates associated
//! with the appropriate calendar (though they will convert ISO dates to the calendar if provided), they just do not force the
//! programmer to pick the calendar at compile time.
//!
//!
//! # Examples
//!
//! ```
//! use icu::calendar::{DateTime, Gregorian};
//! use icu::datetime::{
//!     options::length, DateTimeFormatter, DateTimeFormatterOptions,
//!     TypedDateTimeFormatter,
//! };
//! use icu::locid::{locale, Locale};
//! use std::str::FromStr;
//! use writeable::assert_writeable_eq;
//!
//! // See the next code example for a more ergonomic example with .into().
//! let options =
//!     DateTimeFormatterOptions::Length(length::Bag::from_date_time_style(
//!         length::Date::Medium,
//!         length::Time::Short,
//!     ));
//!
//! // You can work with a formatter that can select the calendar at runtime:
//! let locale = Locale::from_str("en-u-ca-gregory").unwrap();
//! let dtf = DateTimeFormatter::try_new(
//!     &locale.into(),
//!     options.clone(),
//! )
//! .expect("Failed to create DateTimeFormatter instance.");
//!
//! // Or one that selects a calendar at compile time:
//! let typed_dtf = TypedDateTimeFormatter::<Gregorian>::try_new(
//!     &locale!("en").into(),
//!     options,
//! )
//! .expect("Failed to create TypedDateTimeFormatter instance.");
//!
//! let typed_date =
//!     DateTime::try_new_gregorian_datetime(2020, 9, 12, 12, 34, 28).unwrap();
//! // prefer using ISO dates with DateTimeFormatter
//! let date = typed_date.to_iso().to_any();
//!
//! let formatted_date = dtf.format(&date).expect("Calendars should match");
//! let typed_formatted_date = typed_dtf.format(&typed_date);
//!
//! assert_writeable_eq!(formatted_date, "Sep 12, 2020, 12:34 PM");
//! assert_writeable_eq!(typed_formatted_date, "Sep 12, 2020, 12:34 PM");
//!
//! let formatted_date_string =
//!     dtf.format_to_string(&date).expect("Calendars should match");
//! let typed_formatted_date_string = typed_dtf.format_to_string(&typed_date);
//!
//! assert_eq!(formatted_date_string, "Sep 12, 2020, 12:34 PM");
//! assert_eq!(typed_formatted_date_string, "Sep 12, 2020, 12:34 PM");
//! ```
//!
//! The options can be created more ergonomically using the `Into` trait to automatically
//! convert a [`options::length::Bag`] into a [`DateTimeFormatterOptions::Length`].
//!
//! ```
//! use icu::calendar::Gregorian;
//! use icu::datetime::{
//!     options::length, DateTimeFormatterOptions, TypedDateTimeFormatter,
//! };
//! use icu::locid::locale;
//! let options = length::Bag::from_date_time_style(
//!     length::Date::Medium,
//!     length::Time::Short,
//! )
//! .into();
//!
//! let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(
//!     &locale!("en").into(),
//!     options,
//! );
//! ```
//!
//! At the moment, the crate provides only options using the [`Length`] bag, but in the future,
//! we expect to add more ways to customize the output, like skeletons, and components.
//!
//! [data provider]: icu_provider
//! [`ICU4X`]: ../icu/index.html
//! [`Length`]: options::length
//! [`DateTime`]: calendar::{DateTime}
//! [`Date`]: calendar::{Date}
//! [`Time`]: calendar::types::{Time}
//! [`Calendar`]: calendar::{Calendar}
//! [`AnyCalendar`]: calendar::any_calendar::{AnyCalendar}
//! [`timezone::CustomTimeZone`]: icu::timezone::{CustomTimeZone}
//! [`TimeZoneFormatter`]: time_zone::TimeZoneFormatter

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod calendar;
mod datetime;
mod error;
pub mod fields;
mod format;
pub mod input;
pub mod options;
#[doc(hidden)]
pub mod pattern;
pub mod provider;
pub(crate) mod raw;
#[doc(hidden)]
#[allow(clippy::exhaustive_structs, clippy::exhaustive_enums)] // private-ish module
#[cfg(any(feature = "datagen", feature = "experimental"))]
pub mod skeleton;
pub mod time_zone;
mod zoned_datetime;

mod any;

pub use any::{DateFormatter, DateTimeFormatter, ZonedDateTimeFormatter};
pub use calendar::CldrCalendar;
pub use datetime::{TimeFormatter, TypedDateFormatter, TypedDateTimeFormatter};
pub use error::DateTimeError;
pub use format::datetime::FormattedDateTime;
pub use format::time_zone::FormattedTimeZone;
pub use format::zoned_datetime::FormattedZonedDateTime;
pub use options::DateTimeFormatterOptions;
pub use zoned_datetime::TypedZonedDateTimeFormatter;

#[doc(no_inline)]
pub use DateTimeError as Error;

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;
    use icu_calendar::provider::WeekDataV1Marker;
    use icu_calendar::Gregorian;
    use icu_decimal::FixedDecimalFormatter;
    use icu_plurals::PluralRules;
    use icu_provider::prelude::*;
    use icu_timezone::CustomTimeZone;
    use provider::calendar::patterns::GenericPatternV1Marker;
    use provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
    use provider::calendar::ErasedDateSymbolsV1Marker;
    use provider::calendar::TimeSymbolsV1Marker;
    use provider::time_zones::ExemplarCitiesV1Marker;
    use provider::time_zones::MetazoneGenericNamesLongV1Marker;
    use provider::time_zones::MetazoneGenericNamesShortV1Marker;
    use provider::time_zones::MetazoneSpecificNamesLongV1Marker;
    use provider::time_zones::MetazoneSpecificNamesShortV1Marker;
    use provider::time_zones::TimeZoneFormatsV1Marker;
    use time_zone::TimeZoneDataPayloads;
    use time_zone::TimeZoneFormatter;
    use time_zone::TimeZoneFormatterUnit;

    /// Checks that the size of the type is one of the given sizes.
    /// The size might differ across Rust versions or channels.
    macro_rules! check_size_of {
        ($sizes:pat, $type:path) => {
            assert!(
                matches!(size_of::<$type>(), $sizes),
                concat!(stringify!($type), " is of size {}"),
                size_of::<$type>()
            );
        };
    }

    #[test]
    fn check_sizes() {
        check_size_of!(5800 | 4632, DateFormatter);
        check_size_of!(6792 | 5504, DateTimeFormatter);
        check_size_of!(7904 | 6552, ZonedDateTimeFormatter);
        check_size_of!(1496 | 1344, TimeFormatter);
        check_size_of!(1112 | 1048, TimeZoneFormatter);
        check_size_of!(5752 | 4584, TypedDateFormatter::<Gregorian>);
        check_size_of!(6744 | 5456, TypedDateTimeFormatter::<Gregorian>);

        check_size_of!(88, DateTimeError);
        check_size_of!(200, FormattedDateTime);
        check_size_of!(16, FormattedTimeZone::<CustomTimeZone>);
        check_size_of!(184, FormattedZonedDateTime);

        if cfg!(feature = "experimental") {
            check_size_of!(13, DateTimeFormatterOptions);
        }

        type DP<M> = DataPayload<M>;
        check_size_of!(216, DP::<PatternPluralsFromPatternsV1Marker>);
        check_size_of!(1032 | 912, DP::<TimeSymbolsV1Marker>);
        check_size_of!(40, DP::<GenericPatternV1Marker>);
        check_size_of!(216, DP::<PatternPluralsFromPatternsV1Marker>);
        check_size_of!(5064 | 3912, DP::<ErasedDateSymbolsV1Marker>);
        check_size_of!(24, DP::<WeekDataV1Marker>);
        check_size_of!(232 | 240, DP::<TimeZoneFormatsV1Marker>);
        check_size_of!(64, DP::<ExemplarCitiesV1Marker>);
        check_size_of!(120 | 112, DP::<MetazoneGenericNamesLongV1Marker>);
        check_size_of!(120 | 112, DP::<MetazoneGenericNamesShortV1Marker>);
        check_size_of!(216 | 208, DP::<MetazoneSpecificNamesLongV1Marker>);
        check_size_of!(216 | 208, DP::<MetazoneSpecificNamesShortV1Marker>);
        check_size_of!(176, PluralRules);
        check_size_of!(256 | 216, FixedDecimalFormatter);
        check_size_of!(1024 | 936, TimeZoneDataPayloads);
        check_size_of!(3, TimeZoneFormatterUnit);
    }
}
