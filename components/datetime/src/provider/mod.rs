// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

pub mod calendar;
pub(crate) mod date_time;
pub mod time_zones;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as datetime;
        pub use icu_locid_transform as locid_transform;
    }
    icu_datetime_data::make_provider!(Baked);
    icu_datetime_data::impl_datetime_buddhist_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_buddhist_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_chinese_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_chinese_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_coptic_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_coptic_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_dangi_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_dangi_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_ethiopic_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_ethiopic_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_gregory_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_gregory_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_hebrew_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_hebrew_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_indian_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_indian_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_islamic_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_islamic_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_japanese_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_japanese_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_japanext_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_japanext_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_persian_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_persian_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_roc_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_roc_datesymbols_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_skeletons_v1!(Baked);
    icu_datetime_data::impl_datetime_timelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_timesymbols_v1!(Baked);
    icu_datetime_data::impl_time_zone_exemplar_cities_v1!(Baked);
    icu_datetime_data::impl_time_zone_formats_v1!(Baked);
    icu_datetime_data::impl_time_zone_generic_long_v1!(Baked);
    icu_datetime_data::impl_time_zone_generic_short_v1!(Baked);
    icu_datetime_data::impl_time_zone_specific_long_v1!(Baked);
    icu_datetime_data::impl_time_zone_specific_short_v1!(Baked);
};

#[cfg(feature = "datagen")]
use icu_provider::prelude::*;

#[cfg(feature = "datagen")]
/// The latest minimum set of keys required by this component.
pub const KEYS: &[DataKey] = &[
    calendar::BuddhistDateLengthsV1Marker::KEY,
    calendar::BuddhistDateSymbolsV1Marker::KEY,
    calendar::ChineseDateLengthsV1Marker::KEY,
    calendar::ChineseDateSymbolsV1Marker::KEY,
    calendar::CopticDateLengthsV1Marker::KEY,
    calendar::CopticDateSymbolsV1Marker::KEY,
    calendar::DangiDateLengthsV1Marker::KEY,
    calendar::DangiDateSymbolsV1Marker::KEY,
    calendar::EthiopianDateLengthsV1Marker::KEY,
    calendar::EthiopianDateSymbolsV1Marker::KEY,
    calendar::GregorianDateLengthsV1Marker::KEY,
    calendar::GregorianDateSymbolsV1Marker::KEY,
    calendar::HebrewDateLengthsV1Marker::KEY,
    calendar::HebrewDateSymbolsV1Marker::KEY,
    calendar::IndianDateLengthsV1Marker::KEY,
    calendar::IndianDateSymbolsV1Marker::KEY,
    calendar::IslamicDateLengthsV1Marker::KEY,
    calendar::IslamicDateSymbolsV1Marker::KEY,
    calendar::JapaneseDateLengthsV1Marker::KEY,
    calendar::JapaneseDateSymbolsV1Marker::KEY,
    calendar::JapaneseExtendedDateLengthsV1Marker::KEY,
    calendar::JapaneseExtendedDateSymbolsV1Marker::KEY,
    calendar::PersianDateLengthsV1Marker::KEY,
    calendar::PersianDateSymbolsV1Marker::KEY,
    calendar::RocDateLengthsV1Marker::KEY,
    calendar::RocDateSymbolsV1Marker::KEY,
    calendar::TimeLengthsV1Marker::KEY,
    calendar::TimeSymbolsV1Marker::KEY,
    time_zones::ExemplarCitiesV1Marker::KEY,
    time_zones::MetazoneGenericNamesLongV1Marker::KEY,
    time_zones::MetazoneGenericNamesShortV1Marker::KEY,
    time_zones::MetazoneSpecificNamesLongV1Marker::KEY,
    time_zones::MetazoneSpecificNamesShortV1Marker::KEY,
    time_zones::TimeZoneFormatsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    calendar::DateSkeletonPatternsV1Marker::KEY,
];
