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
    impl_datetime_names_year_japanext_v1!(Baked);
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
    impl_datetime_names_month_japanext_v1!(Baked);
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
    impl_datetime_patterns_date_japanext_v1!(Baked);
    impl_datetime_patterns_date_persian_v1!(Baked);
    impl_datetime_patterns_date_roc_v1!(Baked);
};

#[cfg(feature = "compiled_data")]
impl icu_provider::DataProvider<icu_time::provider::TimezonePeriodsV1> for Baked {
    #[inline]
    fn load(
        &self,
        req: icu_provider::DataRequest,
    ) -> Result<
        icu_provider::DataResponse<icu_time::provider::TimezonePeriodsV1>,
        icu_provider::DataError,
    > {
        icu_time::provider::Baked.load(req)
    }
}

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
    neo::DatetimeNamesYearJapanextV1::INFO,
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
    neo::DatetimeNamesMonthJapanextV1::INFO,
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
    DatetimePatternsDateJapaneseV1::INFO,
    DatetimePatternsDateJapanextV1::INFO,
    DatetimePatternsDatePersianV1::INFO,
    DatetimePatternsDateRocV1::INFO,
];
