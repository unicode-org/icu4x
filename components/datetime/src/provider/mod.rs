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

#[cfg(feature = "serde")]
pub(crate) mod compat;
pub mod fields;
pub mod names;
pub mod packed_pattern;
pub mod pattern;
pub mod semantic_skeletons;
#[cfg(feature = "datagen")]
pub mod skeleton;
pub mod time_zones;

pub use packed_pattern::*;

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

    impl_timezone_names_cities_override_v1!(Baked);
    impl_timezone_names_cities_root_v1!(Baked);
    impl_timezone_names_essentials_v1!(Baked);
    impl_timezone_names_generic_long_v1!(Baked);
    impl_timezone_names_generic_short_v1!(Baked);
    impl_timezone_names_locations_override_v1!(Baked);
    impl_timezone_names_locations_root_v1!(Baked);
    impl_timezone_names_specific_long_v1!(Baked);
    impl_timezone_names_specific_short_v1!(Baked);
    impl_timezone_names_standard_long_v1!(Baked);

    impl_datetime_patterns_glue_v1!(Baked);
    impl_datetime_patterns_time_v1!(Baked);

    impl_datetime_names_weekday_v1!(Baked);
    impl_datetime_names_dayperiod_v1!(Baked);

    impl_datetime_names_year_buddhist_v1!(Baked);
    impl_datetime_names_year_chinese_v1!(Baked);
    impl_datetime_names_year_coptic_v1!(Baked);
    impl_datetime_names_year_dangi_v1!(Baked);
    impl_datetime_names_year_ethiopian_v1!(Baked);
    impl_datetime_names_year_gregorian_v1!(Baked);
    impl_datetime_names_year_hebrew_v1!(Baked);
    impl_datetime_names_year_indian_v1!(Baked);
    impl_datetime_names_year_hijri_v1!(Baked);
    impl_datetime_names_year_japanese_v1!(Baked);
    impl_datetime_names_year_persian_v1!(Baked);
    impl_datetime_names_year_roc_v1!(Baked);

    impl_datetime_names_month_buddhist_v1!(Baked);
    impl_datetime_names_month_chinese_v1!(Baked);
    impl_datetime_names_month_coptic_v1!(Baked);
    impl_datetime_names_month_dangi_v1!(Baked);
    impl_datetime_names_month_ethiopian_v1!(Baked);
    impl_datetime_names_month_gregorian_v1!(Baked);
    impl_datetime_names_month_hebrew_v1!(Baked);
    impl_datetime_names_month_indian_v1!(Baked);
    impl_datetime_names_month_hijri_v1!(Baked);
    impl_datetime_names_month_japanese_v1!(Baked);
    impl_datetime_names_month_persian_v1!(Baked);
    impl_datetime_names_month_roc_v1!(Baked);

    impl_datetime_patterns_date_buddhist_v1!(Baked);
    impl_datetime_patterns_date_chinese_v1!(Baked);
    impl_datetime_patterns_date_coptic_v1!(Baked);
    impl_datetime_patterns_date_dangi_v1!(Baked);
    impl_datetime_patterns_date_ethiopian_v1!(Baked);
    impl_datetime_patterns_date_gregorian_v1!(Baked);
    impl_datetime_patterns_date_hebrew_v1!(Baked);
    impl_datetime_patterns_date_indian_v1!(Baked);
    impl_datetime_patterns_date_hijri_v1!(Baked);
    impl_datetime_patterns_date_japanese_v1!(Baked);
    impl_datetime_patterns_date_persian_v1!(Baked);
    impl_datetime_patterns_date_roc_v1!(Baked);
};

#[cfg(feature = "compiled_data")]
impl DataProvider<icu_time::provider::TimezonePeriodsV1> for Baked {
    #[inline]
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<icu_time::provider::TimezonePeriodsV1>, DataError> {
        icu_time::provider::Baked.load(req)
    }
}

#[cfg(any(feature = "datagen", feature = "compiled_data"))]
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
    time_zones::TimezoneNamesSpecificLongV1::INFO,
    time_zones::TimezoneNamesSpecificShortV1::INFO,
    time_zones::TimezoneNamesEssentialsV1::INFO,
    names::DatetimeNamesWeekdayV1::INFO,
    names::DatetimeNamesDayperiodV1::INFO,
    names::DatetimeNamesYearBuddhistV1::INFO,
    names::DatetimeNamesYearChineseV1::INFO,
    names::DatetimeNamesYearCopticV1::INFO,
    names::DatetimeNamesYearDangiV1::INFO,
    names::DatetimeNamesYearEthiopianV1::INFO,
    names::DatetimeNamesYearGregorianV1::INFO,
    names::DatetimeNamesYearHebrewV1::INFO,
    names::DatetimeNamesYearIndianV1::INFO,
    names::DatetimeNamesYearHijriV1::INFO,
    names::DatetimeNamesYearJapaneseV1::INFO,
    names::DatetimeNamesYearPersianV1::INFO,
    names::DatetimeNamesYearRocV1::INFO,
    names::DatetimeNamesMonthBuddhistV1::INFO,
    names::DatetimeNamesMonthChineseV1::INFO,
    names::DatetimeNamesMonthCopticV1::INFO,
    names::DatetimeNamesMonthDangiV1::INFO,
    names::DatetimeNamesMonthEthiopianV1::INFO,
    names::DatetimeNamesMonthGregorianV1::INFO,
    names::DatetimeNamesMonthHebrewV1::INFO,
    names::DatetimeNamesMonthIndianV1::INFO,
    names::DatetimeNamesMonthHijriV1::INFO,
    names::DatetimeNamesMonthJapaneseV1::INFO,
    names::DatetimeNamesMonthPersianV1::INFO,
    names::DatetimeNamesMonthRocV1::INFO,
    semantic_skeletons::DatetimePatternsGlueV1::INFO,
    semantic_skeletons::DatetimePatternsTimeV1::INFO,
    semantic_skeletons::DatetimePatternsDateBuddhistV1::INFO,
    semantic_skeletons::DatetimePatternsDateChineseV1::INFO,
    semantic_skeletons::DatetimePatternsDateCopticV1::INFO,
    semantic_skeletons::DatetimePatternsDateDangiV1::INFO,
    semantic_skeletons::DatetimePatternsDateEthiopianV1::INFO,
    semantic_skeletons::DatetimePatternsDateGregorianV1::INFO,
    semantic_skeletons::DatetimePatternsDateHebrewV1::INFO,
    semantic_skeletons::DatetimePatternsDateIndianV1::INFO,
    semantic_skeletons::DatetimePatternsDateHijriV1::INFO,
    semantic_skeletons::DatetimePatternsDateJapaneseV1::INFO,
    semantic_skeletons::DatetimePatternsDatePersianV1::INFO,
    semantic_skeletons::DatetimePatternsDateRocV1::INFO,
];
