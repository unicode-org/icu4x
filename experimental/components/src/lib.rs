// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 `icu_experimental` is the development crate of the `ICU4X` project.
//!
//! It will usually undergo a major SemVer bump for every ICU4X release. Components in this
//! crate will eventually stabilize and move to the `icu` crate.

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
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

#[doc(hidden)]
// Compiled constructors look for the baked provider at crate::provider::Baked,
// which is why we have to hook it up here.
pub mod provider {
    #[cfg(feature = "compiled_data")]
    pub struct Baked;

    #[cfg(feature = "compiled_data")]
    const _: () = {
        pub mod icu {
            pub use crate as experimental;
            #[allow(unused_imports)] // baked data may or may not need this
            pub use icu_locid_transform as locid_transform;
        }
        icu_experimental_data::make_provider!(Baked);
        icu_experimental_data::impl_displaynames_languages_v1!(Baked);
        icu_experimental_data::impl_displaynames_locales_v1!(Baked);
        icu_experimental_data::impl_displaynames_regions_v1!(Baked);
        icu_experimental_data::impl_displaynames_scripts_v1!(Baked);
        icu_experimental_data::impl_displaynames_variants_v1!(Baked);
        icu_experimental_data::impl_compactdecimal_long_v1!(Baked);
        icu_experimental_data::impl_compactdecimal_short_v1!(Baked);
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
    };

    #[cfg(feature = "datagen")]
    /// The latest minimum set of keys required by this component.
    pub const KEYS: &[DataKey] = &[
        super::dimension::provider::CurrencyEssentialsV1Marker::KEY,
        super::dimension::provider::PercentEssentialsV1Marker::KEY,
        super::displaynames::provider::LanguageDisplayNamesV1Marker::KEY,
        super::displaynames::provider::LocaleDisplayNamesV1Marker::KEY,
        super::displaynames::provider::RegionDisplayNamesV1Marker::KEY,
        super::displaynames::provider::ScriptDisplayNamesV1Marker::KEY,
        super::displaynames::provider::VariantDisplayNamesV1Marker::KEY,
        super::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker::KEY,
        super::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker::KEY,
        super::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker::KEY,
        super::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker::KEY,
    ];
}
