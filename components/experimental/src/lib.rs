// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 The experimental development module of the `ICU4X` project.
//!
//! This module is published as its own crate ([`icu_experimental`](https://docs.rs/icu_experimental/latest/icu_experimental/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! It will usually undergo a major SemVer bump for every ICU4X release. Components in this
//! crate will eventually stabilize and move to their own top-level components.

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc)), no_std)]
// No boilerplate, each module has their own
#![allow(clippy::module_inception)]

extern crate alloc;

pub mod compactdecimal;
pub mod dimension;
pub mod displaynames;
pub mod duration;
pub mod measure;
pub mod personnames;
pub mod relativetime;
pub mod transliterate;
pub mod unicodeset_parse;
pub mod units;

#[doc(hidden)] // compiled constructors look for the baked provider here
pub mod provider {
    #[cfg(feature = "compiled_data")]
    pub struct Baked;

    #[cfg(feature = "compiled_data")]
    #[allow(unused_imports)]
    const _: () = {
        use icu_experimental_data::*;
        pub mod icu {
            pub use crate as experimental;
            pub use icu_collections as collections;
            pub use icu_locale as locale;
            pub use icu_plurals as plurals;
        }
        make_provider!(Baked);

        impl_long_compact_decimal_format_data_v1!(Baked);
        impl_short_compact_decimal_format_data_v1!(Baked);
        impl_short_currency_compact_v1!(Baked);
        impl_currency_essentials_v1!(Baked);
        impl_currency_displayname_v1!(Baked);
        impl_currency_patterns_data_v1!(Baked);
        impl_currency_extended_data_v1!(Baked);
        impl_units_display_name_v1!(Baked);
        impl_units_essentials_v1!(Baked);
        impl_language_display_names_v1!(Baked);
        impl_digital_duration_data_v1!(Baked);
        impl_locale_display_names_v1!(Baked);
        impl_region_display_names_v1!(Baked);
        impl_script_display_names_v1!(Baked);
        impl_variant_display_names_v1!(Baked);
        impl_percent_essentials_v1!(Baked);
        impl_person_names_format_v1!(Baked);
        impl_long_day_relative_v1!(Baked);
        impl_long_hour_relative_v1!(Baked);
        impl_long_minute_relative_v1!(Baked);
        impl_long_month_relative_v1!(Baked);
        impl_long_quarter_relative_v1!(Baked);
        impl_long_second_relative_v1!(Baked);
        impl_long_week_relative_v1!(Baked);
        impl_long_year_relative_v1!(Baked);
        impl_narrow_day_relative_v1!(Baked);
        impl_narrow_hour_relative_v1!(Baked);
        impl_narrow_minute_relative_v1!(Baked);
        impl_narrow_month_relative_v1!(Baked);
        impl_narrow_quarter_relative_v1!(Baked);
        impl_narrow_second_relative_v1!(Baked);
        impl_narrow_week_relative_v1!(Baked);
        impl_narrow_year_relative_v1!(Baked);
        impl_short_day_relative_v1!(Baked);
        impl_short_hour_relative_v1!(Baked);
        impl_short_minute_relative_v1!(Baked);
        impl_short_month_relative_v1!(Baked);
        impl_short_quarter_relative_v1!(Baked);
        impl_short_second_relative_v1!(Baked);
        impl_short_week_relative_v1!(Baked);
        impl_short_year_relative_v1!(Baked);
        impl_transliterator_rules_v1!(Baked);
        impl_units_info_v1!(Baked);
        impl_unit_ids_v1!(Baked);
    };

    #[cfg(feature = "datagen")]
    use icu_provider::prelude::*;

    #[cfg(feature = "datagen")]
    /// The latest minimum set of keys required by this component.
    pub const MARKERS: &[DataMarkerInfo] = &[
        super::compactdecimal::provider::LongCompactDecimalFormatDataV1::INFO,
        super::compactdecimal::provider::ShortCompactDecimalFormatDataV1::INFO,
        super::compactdecimal::provider::LongCompactDecimalFormatDataV1::INFO,
        super::compactdecimal::provider::ShortCompactDecimalFormatDataV1::INFO,
        super::dimension::provider::currency::compact::ShortCurrencyCompactV1::INFO,
        super::dimension::provider::currency::displayname::CurrencyDisplaynameV1::INFO,
        super::dimension::provider::currency::essentials::CurrencyEssentialsV1::INFO,
        super::dimension::provider::currency::patterns::CurrencyPatternsDataV1::INFO,
        super::dimension::provider::currency::extended::CurrencyExtendedDataV1::INFO,
        super::dimension::provider::percent::PercentEssentialsV1::INFO,
        super::dimension::provider::units_essentials::UnitsEssentialsV1::INFO,
        super::dimension::provider::units::UnitsDisplayNameV1::INFO,
        super::displaynames::provider::LanguageDisplayNamesV1::INFO,
        super::duration::provider::DigitalDurationDataV1::INFO,
        super::displaynames::provider::LocaleDisplayNamesV1::INFO,
        super::displaynames::provider::RegionDisplayNamesV1::INFO,
        super::displaynames::provider::ScriptDisplayNamesV1::INFO,
        super::displaynames::provider::VariantDisplayNamesV1::INFO,
        super::measure::provider::UnitIdsV1::INFO,
        super::personnames::provider::PersonNamesFormatV1::INFO,
        super::relativetime::provider::LongDayRelativeV1::INFO,
        super::relativetime::provider::LongHourRelativeV1::INFO,
        super::relativetime::provider::LongMinuteRelativeV1::INFO,
        super::relativetime::provider::LongMonthRelativeV1::INFO,
        super::relativetime::provider::LongQuarterRelativeV1::INFO,
        super::relativetime::provider::LongSecondRelativeV1::INFO,
        super::relativetime::provider::LongWeekRelativeV1::INFO,
        super::relativetime::provider::LongYearRelativeV1::INFO,
        super::relativetime::provider::NarrowDayRelativeV1::INFO,
        super::relativetime::provider::NarrowHourRelativeV1::INFO,
        super::relativetime::provider::NarrowMinuteRelativeV1::INFO,
        super::relativetime::provider::NarrowMonthRelativeV1::INFO,
        super::relativetime::provider::NarrowQuarterRelativeV1::INFO,
        super::relativetime::provider::NarrowSecondRelativeV1::INFO,
        super::relativetime::provider::NarrowWeekRelativeV1::INFO,
        super::relativetime::provider::NarrowYearRelativeV1::INFO,
        super::relativetime::provider::ShortDayRelativeV1::INFO,
        super::relativetime::provider::ShortHourRelativeV1::INFO,
        super::relativetime::provider::ShortMinuteRelativeV1::INFO,
        super::relativetime::provider::ShortMonthRelativeV1::INFO,
        super::relativetime::provider::ShortQuarterRelativeV1::INFO,
        super::relativetime::provider::ShortSecondRelativeV1::INFO,
        super::relativetime::provider::ShortWeekRelativeV1::INFO,
        super::relativetime::provider::ShortYearRelativeV1::INFO,
        super::transliterate::provider::TransliteratorRulesV1::INFO,
        super::units::provider::UnitsInfoV1::INFO,
    ];
}
