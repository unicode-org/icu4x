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
pub mod fields;
pub mod neo;
pub(crate) mod packed_pattern;
pub mod pattern;
#[cfg(feature = "datagen")]
pub mod skeleton;
pub mod time_zones;

pub use packed_pattern::*;

pub(crate) type ErasedPackedPatterns = icu_provider::marker::ErasedMarker<PackedPatterns<'static>>;

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
#[allow(unused_imports)]
const _: () = {
    use icu_datetime_data::*;
    pub mod icu {
        pub use crate as datetime;
        pub use icu_locale as locale;
    }
    make_provider!(Baked);

    impl_locations_v1!(Baked);
    impl_locations_root_v1!(Baked);
    impl_exemplar_cities_v1!(Baked);
    impl_exemplar_cities_root_v1!(Baked);
    impl_metazone_generic_names_long_v1!(Baked);
    impl_metazone_standard_names_long_v1!(Baked);
    impl_metazone_generic_names_short_v1!(Baked);
    impl_metazone_period_v1!(Baked);
    impl_metazone_specific_names_long_v1!(Baked);
    impl_metazone_specific_names_short_v1!(Baked);
    impl_time_zone_essentials_v1!(Baked);

    impl_weekday_names_v1!(Baked);
    impl_day_period_names_v1!(Baked);
    impl_glue_pattern_v1!(Baked);
    impl_time_neo_skeleton_patterns_v1!(Baked);

    impl_buddhist_year_names_v1!(Baked);
    impl_chinese_year_names_v1!(Baked);
    impl_coptic_year_names_v1!(Baked);
    impl_dangi_year_names_v1!(Baked);
    impl_ethiopian_year_names_v1!(Baked);
    impl_gregorian_year_names_v1!(Baked);
    impl_hebrew_year_names_v1!(Baked);
    impl_indian_year_names_v1!(Baked);
    impl_hijri_year_names_v1!(Baked);
    impl_japanese_year_names_v1!(Baked);
    impl_japanese_extended_year_names_v1!(Baked);
    impl_persian_year_names_v1!(Baked);
    impl_roc_year_names_v1!(Baked);

    impl_buddhist_month_names_v1!(Baked);
    impl_chinese_month_names_v1!(Baked);
    impl_coptic_month_names_v1!(Baked);
    impl_dangi_month_names_v1!(Baked);
    impl_ethiopian_month_names_v1!(Baked);
    impl_gregorian_month_names_v1!(Baked);
    impl_hebrew_month_names_v1!(Baked);
    impl_indian_month_names_v1!(Baked);
    impl_hijri_month_names_v1!(Baked);
    impl_japanese_month_names_v1!(Baked);
    impl_japanese_extended_month_names_v1!(Baked);
    impl_persian_month_names_v1!(Baked);
    impl_roc_month_names_v1!(Baked);

    impl_buddhist_date_neo_skeleton_patterns_v1!(Baked);
    impl_chinese_date_neo_skeleton_patterns_v1!(Baked);
    impl_coptic_date_neo_skeleton_patterns_v1!(Baked);
    impl_dangi_date_neo_skeleton_patterns_v1!(Baked);
    impl_ethiopian_date_neo_skeleton_patterns_v1!(Baked);
    impl_gregorian_date_neo_skeleton_patterns_v1!(Baked);
    impl_hebrew_date_neo_skeleton_patterns_v1!(Baked);
    impl_indian_date_neo_skeleton_patterns_v1!(Baked);
    impl_hijri_date_neo_skeleton_patterns_v1!(Baked);
    impl_japanese_date_neo_skeleton_patterns_v1!(Baked);
    impl_japanese_extended_date_neo_skeleton_patterns_v1!(Baked);
    impl_persian_date_neo_skeleton_patterns_v1!(Baked);
    impl_roc_date_neo_skeleton_patterns_v1!(Baked);
};

#[cfg(feature = "datagen")]
use icu_provider::prelude::*;

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    time_zones::TimezoneNamesLocationsOverrideV1::INFO,
    time_zones::TimezoneNamesLocationsRootV1::INFO,
    time_zones::TimezoneNamesCitiesOverrideV1::INFO,
    time_zones::TimezoneNamesCitiesRootV1::INFO,
    time_zones::TimezoneNamesGenericLongV1::INFO,
    time_zones::TimezoneNamesStandardLongV1::INFO,
    time_zones::TimezoneNamesGenericShortV1::INFO,
    time_zones::TimezoneMetazonePeriodsV1::INFO,
    time_zones::TimezoneNamesSpecificLongV1::INFO,
    time_zones::TimezoneNamesSpecificShortV1::INFO,
    time_zones::TimezoneNamesEssentialsV1::INFO,
    neo::DatetimeNamesWeekdayV1::INFO,
    neo::DatetimeNamesDayperiodV1::INFO,
    neo::DatetimePatternsGlueV1::INFO,
    DatetimePatternsTimeV1::INFO,
    neo::DatetimeNamesYearBuddhistV1::INFO,
    neo::DatetimeNamesYearChineseV1::INFO,
    neo::DatetimeNamesYearCopticV1::INFO,
    neo::DatetimeNamesYearDangiV1::INFO,
    neo::DatetimeNamesYearEthiopianV1::INFO,
    neo::DatetimeNamesYearGregorianV1::INFO,
    neo::DatetimeNamesYearHebrewV1::INFO,
    neo::DatetimeNamesYearIndianV1::INFO,
    neo::DatetimeNamesYearHijriV1::INFO,
    neo::DatetimeNamesYearJapaneseV1::INFO,
    neo::DatetimeNamesYearJapaneseExtendedV1::INFO,
    neo::DatetimeNamesYearPersianV1::INFO,
    neo::DatetimeNamesYearRocV1::INFO,
    neo::DatetimeNamesMonthBuddhistV1::INFO,
    neo::DatetimeNamesMonthChineseV1::INFO,
    neo::DatetimeNamesMonthCopticV1::INFO,
    neo::DatetimeNamesMonthDangiV1::INFO,
    neo::DatetimeNamesMonthEthiopianV1::INFO,
    neo::DatetimeNamesMonthGregorianV1::INFO,
    neo::DatetimeNamesMonthHebrewV1::INFO,
    neo::DatetimeNamesMonthIndianV1::INFO,
    neo::DatetimeNamesMonthHijriV1::INFO,
    neo::DatetimeNamesMonthJapaneseV1::INFO,
    neo::DatetimeNamesMonthJapaneseExtendedV1::INFO,
    neo::DatetimeNamesMonthPersianV1::INFO,
    neo::DatetimeNamesMonthRocV1::INFO,
    DatetimePatternsDateBuddhistV1::INFO,
    DatetimePatternsDateChineseV1::INFO,
    DatetimePatternsDateCopticV1::INFO,
    DatetimePatternsDateDangiV1::INFO,
    DatetimePatternsDateEthiopianV1::INFO,
    DatetimePatternsDateGregorianV1::INFO,
    DatetimePatternsDateHebrewV1::INFO,
    DatetimePatternsDateIndianV1::INFO,
    DatetimePatternsDateHijriV1::INFO,
    DatetimePatternsDateJapaneseModernV1::INFO,
    DatetimePatternsDateJapaneseExtendedV1::INFO,
    DatetimePatternsDatePersianV1::INFO,
    DatetimePatternsDateRocV1::INFO,
];
