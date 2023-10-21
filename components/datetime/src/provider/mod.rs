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

/// Module for experimental new DateSymbols design
/// <https://github.com/unicode-org/icu4x/issues/3865>
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
//
//
// WHEN THIS GRADUATES; be sure to update the check for "neo" in baked_exporter!
#[cfg(any(feature = "datagen", feature = "experimental"))]
pub mod neo;

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

    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_weekdays_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_dayperiods_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_buddhist_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_buddhist_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_chinese_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_chinese_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_coptic_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_coptic_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_dangi_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_dangi_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_ethiopic_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_ethiopic_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_gregory_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_gregory_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_hebrew_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_hebrew_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_indian_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_indian_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_islamic_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_islamic_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_japanese_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_japanese_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_japanext_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_japanext_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_persian_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_persian_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_roc_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_symbols_roc_years_v1!(Baked);

    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_datetime_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_buddhist_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_chinese_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_coptic_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_dangi_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_ethiopic_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_gregory_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_hebrew_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_indian_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_islamic_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_japanese_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_japanext_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_persian_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_patterns_roc_date_v1!(Baked);
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
    #[cfg(feature = "experimental")]
    neo::WeekdaySymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::DayPeriodSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::BuddhistYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::ChineseYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::CopticYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::DangiYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::EthiopianYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::GregorianYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::HebrewYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::IndianYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::IslamicYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::JapaneseYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::PersianYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::RocYearSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::BuddhistMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::ChineseMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::CopticMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::DangiMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::EthiopianMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::GregorianMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::HebrewMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::IndianMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::IslamicMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::JapaneseMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::PersianMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::RocMonthSymbolsV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::DateTimePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::BuddhistDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::ChineseDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::CopticDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::DangiDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::EthiopianDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::GregorianDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::HebrewDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::IndianDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::IslamicDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::JapaneseDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::PersianDatePatternV1Marker::KEY,
    #[cfg(feature = "experimental")]
    neo::RocDatePatternV1Marker::KEY,
];
