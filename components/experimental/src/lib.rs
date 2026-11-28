// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
    )
)]
#![warn(missing_docs)]

//! 🚧 The experimental development module of the `ICU4X` project.
//!
//! This module is published as its own crate ([`icu_experimental`](https://docs.rs/icu_experimental/latest/icu_experimental/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! It will usually undergo a major `SemVer` bump for every ICU4X release. Components in this
//! crate will eventually stabilize and move to their own top-level components.

#![allow(clippy::module_inception)]

extern crate alloc;

pub mod dimension;
pub mod displaynames;
pub mod duration;
pub mod measure;
mod messageformat;
pub mod personnames;
pub mod relativetime;
pub mod transliterate;
pub mod units;

#[doc(hidden)] // compiled constructors look for the baked provider here
pub mod provider {
    // Provider structs must be stable
    #![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

    #[cfg(feature = "compiled_data")]
    #[derive(Debug)]
    pub struct Baked;

    #[cfg(feature = "compiled_data")]
    #[allow(unused_imports)]
    const _: () = {
        use icu_experimental_data::*;
        pub mod icu {
            pub use crate as experimental;
            pub use icu_collections as collections;
            pub use icu_decimal as decimal;
            pub use icu_plurals as plurals;
        }
        make_provider!(Baked);

        impl_currency_essentials_v1!(Baked);
        impl_currency_symbols_v1!(Baked);
        impl_currency_decimal_symbols_v1!(Baked);
        impl_currency_displayname_v1!(Baked);
        impl_currency_patterns_data_v1!(Baked);
        impl_currency_extended_data_v1!(Baked);
        impl_currency_fractions_v1!(Baked);
        impl_currency_patterns_no_currency_v1!(Baked);
        impl_units_names_area_long_essential_v1!(Baked);
        impl_units_names_area_long_imperial_v1!(Baked);
        impl_units_names_area_long_metric_v1!(Baked);
        impl_units_names_area_long_outlier_v1!(Baked);
        impl_units_names_area_short_essential_v1!(Baked);
        impl_units_names_area_short_imperial_v1!(Baked);
        impl_units_names_area_short_metric_v1!(Baked);
        impl_units_names_area_short_outlier_v1!(Baked);
        impl_units_names_area_narrow_essential_v1!(Baked);
        impl_units_names_area_narrow_imperial_v1!(Baked);
        impl_units_names_area_narrow_metric_v1!(Baked);
        impl_units_names_area_narrow_outlier_v1!(Baked);
        impl_units_names_duration_long_essential_v1!(Baked);
        impl_units_names_duration_long_imperial_v1!(Baked);
        impl_units_names_duration_long_metric_v1!(Baked);
        impl_units_names_duration_long_outlier_v1!(Baked);
        impl_units_names_duration_short_essential_v1!(Baked);
        impl_units_names_duration_short_imperial_v1!(Baked);
        impl_units_names_duration_short_metric_v1!(Baked);
        impl_units_names_duration_short_outlier_v1!(Baked);
        impl_units_names_duration_narrow_essential_v1!(Baked);
        impl_units_names_duration_narrow_imperial_v1!(Baked);
        impl_units_names_duration_narrow_metric_v1!(Baked);
        impl_units_names_duration_narrow_outlier_v1!(Baked);
        impl_units_names_length_long_essential_v1!(Baked);
        impl_units_names_length_long_imperial_v1!(Baked);
        impl_units_names_length_long_metric_v1!(Baked);
        impl_units_names_length_long_outlier_v1!(Baked);
        impl_units_names_length_short_essential_v1!(Baked);
        impl_units_names_length_short_imperial_v1!(Baked);
        impl_units_names_length_short_metric_v1!(Baked);
        impl_units_names_length_short_outlier_v1!(Baked);
        impl_units_names_length_narrow_essential_v1!(Baked);
        impl_units_names_length_narrow_imperial_v1!(Baked);
        impl_units_names_length_narrow_metric_v1!(Baked);
        impl_units_names_length_narrow_outlier_v1!(Baked);
        impl_units_names_mass_long_essential_v1!(Baked);
        impl_units_names_mass_long_imperial_v1!(Baked);
        impl_units_names_mass_long_metric_v1!(Baked);
        impl_units_names_mass_long_outlier_v1!(Baked);
        impl_units_names_mass_short_essential_v1!(Baked);
        impl_units_names_mass_short_imperial_v1!(Baked);
        impl_units_names_mass_short_metric_v1!(Baked);
        impl_units_names_mass_short_outlier_v1!(Baked);
        impl_units_names_mass_narrow_essential_v1!(Baked);
        impl_units_names_mass_narrow_imperial_v1!(Baked);
        impl_units_names_mass_narrow_metric_v1!(Baked);
        impl_units_names_mass_narrow_outlier_v1!(Baked);
        impl_units_names_volume_long_essential_v1!(Baked);
        impl_units_names_volume_long_imperial_v1!(Baked);
        impl_units_names_volume_long_metric_v1!(Baked);
        impl_units_names_volume_long_outlier_v1!(Baked);
        impl_units_names_volume_short_essential_v1!(Baked);
        impl_units_names_volume_short_imperial_v1!(Baked);
        impl_units_names_volume_short_metric_v1!(Baked);
        impl_units_names_volume_short_outlier_v1!(Baked);
        impl_units_names_volume_narrow_essential_v1!(Baked);
        impl_units_names_volume_narrow_imperial_v1!(Baked);
        impl_units_names_volume_narrow_metric_v1!(Baked);
        impl_units_names_volume_narrow_outlier_v1!(Baked);
        impl_units_essentials_v1!(Baked);
        impl_locale_names_language_v0!(Baked);
        impl_units_duration_digital_v1!(Baked);
        impl_locale_names_locale_v0!(Baked);
        impl_locale_names_region_v0!(Baked);
        impl_locale_names_script_v0!(Baked);
        impl_locale_names_variant_v0!(Baked);
        impl_decimal_percent_v1!(Baked);
        impl_person_names_format_v1!(Baked);
        impl_datetime_relative_day_long_v1!(Baked);
        impl_datetime_relative_day_narrow_v1!(Baked);
        impl_datetime_relative_day_short_v1!(Baked);
        impl_datetime_relative_hour_long_v1!(Baked);
        impl_datetime_relative_hour_narrow_v1!(Baked);
        impl_datetime_relative_hour_short_v1!(Baked);
        impl_datetime_relative_minute_long_v1!(Baked);
        impl_datetime_relative_minute_narrow_v1!(Baked);
        impl_datetime_relative_minute_short_v1!(Baked);
        impl_datetime_relative_month_long_v1!(Baked);
        impl_datetime_relative_month_narrow_v1!(Baked);
        impl_datetime_relative_month_short_v1!(Baked);
        impl_datetime_relative_quarter_long_v1!(Baked);
        impl_datetime_relative_quarter_narrow_v1!(Baked);
        impl_datetime_relative_quarter_short_v1!(Baked);
        impl_datetime_relative_second_long_v1!(Baked);
        impl_datetime_relative_second_narrow_v1!(Baked);
        impl_datetime_relative_second_short_v1!(Baked);
        impl_datetime_relative_week_long_v1!(Baked);
        impl_datetime_relative_week_narrow_v1!(Baked);
        impl_datetime_relative_week_short_v1!(Baked);
        impl_datetime_relative_year_long_v1!(Baked);
        impl_datetime_relative_year_narrow_v1!(Baked);
        impl_datetime_relative_year_short_v1!(Baked);
        impl_transliterator_rules_v1!(Baked);
        impl_units_info_v1!(Baked);
        impl_units_id_v1!(Baked);
    };

    #[cfg(feature = "datagen")]
    use icu_provider::prelude::*;

    #[cfg(feature = "datagen")]
    /// The latest minimum set of keys required by this component.
    pub const MARKERS: &[DataMarkerInfo] = &[
        super::dimension::provider::currency::displayname::CurrencyDisplaynameV1::INFO,
        super::dimension::provider::currency::essentials::CurrencyEssentialsV1::INFO,
        super::dimension::provider::currency::symbols::CurrencySymbolsV1::INFO,
        super::dimension::provider::currency::symbols::CurrencyDecimalSymbolsV1::INFO,
        super::dimension::provider::currency::patterns::CurrencyPatternsDataV1::INFO,
        super::dimension::provider::currency::extended::CurrencyExtendedDataV1::INFO,
        super::dimension::provider::currency::fractions::CurrencyFractionsV1::INFO,
        super::dimension::provider::currency::no_currency::CurrencyPatternsNoCurrencyV1::INFO,
        super::dimension::provider::percent::DecimalPercentV1::INFO,
        super::dimension::provider::units::essentials::UnitsEssentialsV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortOutlierV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowEssentialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowImperialV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowMetricV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowOutlierV1::INFO,
        super::displaynames::provider::LocaleNamesLanguageV0::INFO,
        super::duration::provider::UnitsDurationDigitalV1::INFO,
        super::displaynames::provider::LocaleNamesLocaleV0::INFO,
        super::displaynames::provider::LocaleNamesRegionV0::INFO,
        super::displaynames::provider::LocaleNamesScriptV0::INFO,
        super::displaynames::provider::LocaleNamesVariantV0::INFO,
        super::measure::provider::UnitsIdV1::INFO,
        super::personnames::provider::PersonNamesFormatV1::INFO,
        super::relativetime::provider::DatetimeRelativeDayLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeHourLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeMinuteLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeMonthLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeQuarterLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeSecondLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeWeekLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeYearLongV1::INFO,
        super::relativetime::provider::DatetimeRelativeDayNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeHourNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeMinuteNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeMonthNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeQuarterNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeSecondNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeWeekNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeYearNarrowV1::INFO,
        super::relativetime::provider::DatetimeRelativeDayShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeHourShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeMinuteShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeMonthShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeQuarterShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeSecondShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeWeekShortV1::INFO,
        super::relativetime::provider::DatetimeRelativeYearShortV1::INFO,
        super::transliterate::provider::TransliteratorRulesV1::INFO,
        super::units::provider::UnitsInfoV1::INFO,
    ];
}
