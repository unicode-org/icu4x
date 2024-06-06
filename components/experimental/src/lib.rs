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
#![cfg_attr(not(any(test, feature = "std")), no_std)]
// No boilerplate, each module has their own
#![allow(clippy::module_inception)]

extern crate alloc;

pub mod compactdecimal;
pub mod dimension;
pub mod displaynames;
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
    const _: () = {
        pub mod icu {
            pub use crate as experimental;
            #[allow(unused_imports)] // baked data may or may not need this
            pub use icu_locale as locale;
        }
        icu_experimental_data::make_provider!(Baked);
        icu_experimental_data::impl_compactdecimal_long_v1!(Baked);
        icu_experimental_data::impl_compactdecimal_short_v1!(Baked);
        icu_experimental_data::impl_currency_essentials_v1!(Baked);
        icu_experimental_data::impl_displaynames_languages_v1!(Baked);
        icu_experimental_data::impl_displaynames_locales_v1!(Baked);
        icu_experimental_data::impl_displaynames_regions_v1!(Baked);
        icu_experimental_data::impl_displaynames_scripts_v1!(Baked);
        icu_experimental_data::impl_displaynames_variants_v1!(Baked);
        icu_experimental_data::impl_percent_essentials_v1!(Baked);
        icu_experimental_data::impl_personnames_personnames_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_day_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_hour_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_minute_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_month_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_quarter_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_second_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_week_v1!(Baked);
        icu_experimental_data::impl_relativetime_long_year_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_day_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_hour_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_minute_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_month_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_quarter_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_second_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_week_v1!(Baked);
        icu_experimental_data::impl_relativetime_narrow_year_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_day_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_hour_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_minute_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_month_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_quarter_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_second_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_week_v1!(Baked);
        icu_experimental_data::impl_relativetime_short_year_v1!(Baked);
        icu_experimental_data::impl_units_info_v1!(Baked);
    };

    #[cfg(feature = "datagen")]
    use icu_provider::prelude::*;

    #[cfg(feature = "datagen")]
    /// The latest minimum set of keys required by this component.
    pub const MARKERS: &[DataMarkerInfo] = &[
        super::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker::INFO,
        super::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker::INFO,
        super::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker::INFO,
        super::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker::INFO,
        super::dimension::provider::currency::CurrencyEssentialsV1Marker::INFO,
        super::dimension::provider::percent::PercentEssentialsV1Marker::INFO,
        super::displaynames::provider::LanguageDisplayNamesV1Marker::INFO,
        super::displaynames::provider::LocaleDisplayNamesV1Marker::INFO,
        super::displaynames::provider::RegionDisplayNamesV1Marker::INFO,
        super::displaynames::provider::ScriptDisplayNamesV1Marker::INFO,
        super::displaynames::provider::VariantDisplayNamesV1Marker::INFO,
        super::personnames::provider::PersonNamesFormatV1Marker::INFO,
        super::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker::INFO,
        super::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker::INFO,
        super::units::provider::UnitsInfoV1Marker::INFO,
    ];
}
