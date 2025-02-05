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
            pub use icu_experimental_data::icu_locale as locale;
            pub use icu_plurals as plurals;
        }
        make_provider!(Baked);

        impl_long_compact_decimal_format_data_v1_marker!(Baked);
        impl_short_compact_decimal_format_data_v1_marker!(Baked);
        impl_short_currency_compact_v1_marker!(Baked);
        impl_currency_essentials_v1_marker!(Baked);
        impl_currency_displayname_v1_marker!(Baked);
        impl_currency_patterns_data_v1_marker!(Baked);
        impl_currency_extended_data_v1_marker!(Baked);
        impl_units_display_name_v1_marker!(Baked);
        impl_units_essentials_v1_marker!(Baked);
        impl_language_display_names_v1_marker!(Baked);
        impl_digital_duration_data_v1_marker!(Baked);
        impl_locale_display_names_v1_marker!(Baked);
        impl_region_display_names_v1_marker!(Baked);
        impl_script_display_names_v1_marker!(Baked);
        impl_variant_display_names_v1_marker!(Baked);
        impl_percent_essentials_v1_marker!(Baked);
        impl_person_names_format_v1_marker!(Baked);
        impl_long_day_relative_time_format_data_v1_marker!(Baked);
        impl_long_hour_relative_time_format_data_v1_marker!(Baked);
        impl_long_minute_relative_time_format_data_v1_marker!(Baked);
        impl_long_month_relative_time_format_data_v1_marker!(Baked);
        impl_long_quarter_relative_time_format_data_v1_marker!(Baked);
        impl_long_second_relative_time_format_data_v1_marker!(Baked);
        impl_long_week_relative_time_format_data_v1_marker!(Baked);
        impl_long_year_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_day_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_hour_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_minute_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_month_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_quarter_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_second_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_week_relative_time_format_data_v1_marker!(Baked);
        impl_narrow_year_relative_time_format_data_v1_marker!(Baked);
        impl_short_day_relative_time_format_data_v1_marker!(Baked);
        impl_short_hour_relative_time_format_data_v1_marker!(Baked);
        impl_short_minute_relative_time_format_data_v1_marker!(Baked);
        impl_short_month_relative_time_format_data_v1_marker!(Baked);
        impl_short_quarter_relative_time_format_data_v1_marker!(Baked);
        impl_short_second_relative_time_format_data_v1_marker!(Baked);
        impl_short_week_relative_time_format_data_v1_marker!(Baked);
        impl_short_year_relative_time_format_data_v1_marker!(Baked);
        impl_transliterator_rules_v1_marker!(Baked);
        impl_units_info_v1_marker!(Baked);
        impl_units_trie_v1_marker!(Baked);
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
        super::dimension::provider::currency_compact::ShortCurrencyCompactV1::INFO,
        super::dimension::provider::currency_displayname::CurrencyDisplaynameV1::INFO,
        super::dimension::provider::currency::CurrencyEssentialsV1::INFO,
        super::dimension::provider::currency_patterns::CurrencyPatternsDataV1::INFO,
        super::dimension::provider::extended_currency::CurrencyExtendedDataV1::INFO,
        super::dimension::provider::percent::PercentEssentialsV1::INFO,
        super::dimension::provider::units_essentials::UnitsEssentialsV1::INFO,
        super::dimension::provider::units::UnitsDisplayNameV1::INFO,
        super::displaynames::provider::LanguageDisplayNamesV1::INFO,
        super::duration::provider::DigitalDurationDataV1::INFO,
        super::displaynames::provider::LocaleDisplayNamesV1::INFO,
        super::displaynames::provider::RegionDisplayNamesV1::INFO,
        super::displaynames::provider::ScriptDisplayNamesV1::INFO,
        super::displaynames::provider::VariantDisplayNamesV1::INFO,
        super::measure::provider::trie::UnitsTrieV1::INFO,
        super::personnames::provider::PersonNamesFormatV1::INFO,
        super::relativetime::provider::LongDayRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongHourRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongMinuteRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongMonthRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongQuarterRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongSecondRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongWeekRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::LongYearRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowDayRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowHourRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::NarrowYearRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortDayRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortHourRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortMonthRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortSecondRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortWeekRelativeTimeFormatDataV1::INFO,
        super::relativetime::provider::ShortYearRelativeTimeFormatDataV1::INFO,
        super::transliterate::provider::TransliteratorRulesV1::INFO,
        super::units::provider::UnitsInfoV1::INFO,
    ];
}
