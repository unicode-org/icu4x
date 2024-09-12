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

#[cfg(feature = "experimental")]
include!("../../tests/data/date_skeleton_patterns_v1_marker.rs.data");

#[cfg(feature = "compiled_data")]
#[allow(unused_imports)]
const _: () = {
    use icu_datetime_data::*;
    pub mod icu {
        pub use crate as datetime;
        pub use icu_datetime_data::icu_locale as locale;
        pub use icu_timezone as timezone;
    }
    make_provider!(Baked);
    impl_buddhist_date_lengths_v1_marker!(Baked);
    impl_buddhist_date_symbols_v1_marker!(Baked);
    impl_chinese_date_lengths_v1_marker!(Baked);
    impl_chinese_date_symbols_v1_marker!(Baked);
    impl_coptic_date_lengths_v1_marker!(Baked);
    impl_coptic_date_symbols_v1_marker!(Baked);
    impl_dangi_date_lengths_v1_marker!(Baked);
    impl_dangi_date_symbols_v1_marker!(Baked);
    impl_ethiopian_date_lengths_v1_marker!(Baked);
    impl_ethiopian_date_symbols_v1_marker!(Baked);
    impl_gregorian_date_lengths_v1_marker!(Baked);
    impl_gregorian_date_symbols_v1_marker!(Baked);
    impl_hebrew_date_lengths_v1_marker!(Baked);
    impl_hebrew_date_symbols_v1_marker!(Baked);
    impl_indian_date_lengths_v1_marker!(Baked);
    impl_indian_date_symbols_v1_marker!(Baked);
    impl_islamic_date_lengths_v1_marker!(Baked);
    impl_islamic_date_symbols_v1_marker!(Baked);
    impl_japanese_date_lengths_v1_marker!(Baked);
    impl_japanese_date_symbols_v1_marker!(Baked);
    impl_japanese_extended_date_lengths_v1_marker!(Baked);
    impl_japanese_extended_date_symbols_v1_marker!(Baked);
    impl_persian_date_lengths_v1_marker!(Baked);
    impl_persian_date_symbols_v1_marker!(Baked);
    impl_roc_date_lengths_v1_marker!(Baked);
    impl_roc_date_symbols_v1_marker!(Baked);
    impl_time_lengths_v1_marker!(Baked);
    impl_time_symbols_v1_marker!(Baked);
    impl_exemplar_cities_v1_marker!(Baked);
    impl_metazone_generic_names_long_v1_marker!(Baked);
    impl_metazone_generic_names_short_v1_marker!(Baked);
    impl_metazone_specific_names_long_v1_marker!(Baked);
    impl_metazone_specific_names_short_v1_marker!(Baked);
    icu_timezone_data::impl_metazone_period_v1_marker!(Baked);
    impl_time_zone_formats_v1_marker!(Baked);

    #[cfg(feature = "experimental")]
    impl_date_skeleton_patterns_v1_marker!(Baked);

    #[cfg(feature = "experimental")]
    impl_weekday_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_day_period_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_glue_pattern_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_time_neo_skeleton_patterns_v1_marker!(Baked);

    #[cfg(feature = "experimental")]
    impl_buddhist_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_chinese_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_coptic_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_dangi_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_ethiopian_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_gregorian_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_hebrew_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_indian_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_islamic_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_japanese_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_japanese_extended_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_persian_year_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_roc_year_names_v1_marker!(Baked);

    #[cfg(feature = "experimental")]
    impl_buddhist_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_chinese_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_coptic_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_dangi_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_ethiopian_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_gregorian_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_hebrew_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_indian_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_islamic_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_japanese_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_japanese_extended_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_persian_month_names_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_roc_month_names_v1_marker!(Baked);

    #[cfg(feature = "experimental")]
    impl_buddhist_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_chinese_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_coptic_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_dangi_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_ethiopian_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_gregorian_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_hebrew_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_indian_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_islamic_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_japanese_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_japanese_extended_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_persian_date_neo_skeleton_patterns_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_roc_date_neo_skeleton_patterns_v1_marker!(Baked);
};

#[cfg(feature = "datagen")]
use icu_provider::prelude::*;

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    calendar::BuddhistDateLengthsV1Marker::INFO,
    calendar::BuddhistDateSymbolsV1Marker::INFO,
    calendar::ChineseDateLengthsV1Marker::INFO,
    calendar::ChineseDateSymbolsV1Marker::INFO,
    calendar::CopticDateLengthsV1Marker::INFO,
    calendar::CopticDateSymbolsV1Marker::INFO,
    calendar::DangiDateLengthsV1Marker::INFO,
    calendar::DangiDateSymbolsV1Marker::INFO,
    calendar::EthiopianDateLengthsV1Marker::INFO,
    calendar::EthiopianDateSymbolsV1Marker::INFO,
    calendar::GregorianDateLengthsV1Marker::INFO,
    calendar::GregorianDateSymbolsV1Marker::INFO,
    calendar::HebrewDateLengthsV1Marker::INFO,
    calendar::HebrewDateSymbolsV1Marker::INFO,
    calendar::IndianDateLengthsV1Marker::INFO,
    calendar::IndianDateSymbolsV1Marker::INFO,
    calendar::IslamicDateLengthsV1Marker::INFO,
    calendar::IslamicDateSymbolsV1Marker::INFO,
    calendar::JapaneseDateLengthsV1Marker::INFO,
    calendar::JapaneseDateSymbolsV1Marker::INFO,
    calendar::JapaneseExtendedDateLengthsV1Marker::INFO,
    calendar::JapaneseExtendedDateSymbolsV1Marker::INFO,
    calendar::PersianDateLengthsV1Marker::INFO,
    calendar::PersianDateSymbolsV1Marker::INFO,
    calendar::RocDateLengthsV1Marker::INFO,
    calendar::RocDateSymbolsV1Marker::INFO,
    calendar::TimeLengthsV1Marker::INFO,
    calendar::TimeSymbolsV1Marker::INFO,
    time_zones::ExemplarCitiesV1Marker::INFO,
    time_zones::MetazoneGenericNamesLongV1Marker::INFO,
    time_zones::MetazoneGenericNamesShortV1Marker::INFO,
    time_zones::MetazoneSpecificNamesLongV1Marker::INFO,
    time_zones::MetazoneSpecificNamesShortV1Marker::INFO,
    time_zones::TimeZoneFormatsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::WeekdayNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DayPeriodNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GluePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::TimeNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocDateNeoSkeletonPatternsV1Marker::INFO,
];
