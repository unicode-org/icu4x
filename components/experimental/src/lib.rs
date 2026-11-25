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
        impl_units_names_area_long_tiny_v1!(Baked);
        impl_units_names_area_long_light_v1!(Baked);
        impl_units_names_area_long_heavy_v1!(Baked);
        impl_units_names_area_short_tiny_v1!(Baked);
        impl_units_names_area_short_light_v1!(Baked);
        impl_units_names_area_short_heavy_v1!(Baked);
        impl_units_names_area_narrow_tiny_v1!(Baked);
        impl_units_names_area_narrow_light_v1!(Baked);
        impl_units_names_area_narrow_heavy_v1!(Baked);
        impl_units_names_duration_long_tiny_v1!(Baked);
        impl_units_names_duration_long_light_v1!(Baked);
        impl_units_names_duration_long_heavy_v1!(Baked);
        impl_units_names_duration_short_tiny_v1!(Baked);
        impl_units_names_duration_short_light_v1!(Baked);
        impl_units_names_duration_short_heavy_v1!(Baked);
        impl_units_names_duration_narrow_tiny_v1!(Baked);
        impl_units_names_duration_narrow_light_v1!(Baked);
        impl_units_names_duration_narrow_heavy_v1!(Baked);
        impl_units_names_length_long_tiny_v1!(Baked);
        impl_units_names_length_long_light_v1!(Baked);
        impl_units_names_length_long_heavy_v1!(Baked);
        impl_units_names_length_short_tiny_v1!(Baked);
        impl_units_names_length_short_light_v1!(Baked);
        impl_units_names_length_short_heavy_v1!(Baked);
        impl_units_names_length_narrow_tiny_v1!(Baked);
        impl_units_names_length_narrow_light_v1!(Baked);
        impl_units_names_length_narrow_heavy_v1!(Baked);
        impl_units_names_mass_long_tiny_v1!(Baked);
        impl_units_names_mass_long_light_v1!(Baked);
        impl_units_names_mass_long_heavy_v1!(Baked);
        impl_units_names_mass_short_tiny_v1!(Baked);
        impl_units_names_mass_short_light_v1!(Baked);
        impl_units_names_mass_short_heavy_v1!(Baked);
        impl_units_names_mass_narrow_tiny_v1!(Baked);
        impl_units_names_mass_narrow_light_v1!(Baked);
        impl_units_names_mass_narrow_heavy_v1!(Baked);
        impl_units_names_volume_long_tiny_v1!(Baked);
        impl_units_names_volume_long_light_v1!(Baked);
        impl_units_names_volume_long_heavy_v1!(Baked);
        impl_units_names_volume_short_tiny_v1!(Baked);
        impl_units_names_volume_short_light_v1!(Baked);
        impl_units_names_volume_short_heavy_v1!(Baked);
        impl_units_names_volume_narrow_tiny_v1!(Baked);
        impl_units_names_volume_narrow_light_v1!(Baked);
        impl_units_names_volume_narrow_heavy_v1!(Baked);
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
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaLongHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaShortHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesAreaNarrowHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationLongHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationShortHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesDurationNarrowHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthLongHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthShortHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesLengthNarrowHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassLongHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassShortHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesMassNarrowHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeLongHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeShortHeavyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowTinyV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowLightV1::INFO,
        super::dimension::provider::units::categorized_display_names::UnitsNamesVolumeNarrowHeavyV1::INFO,
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
