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

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as datetime;
        pub use icu_locid_transform as locid_transform;
    }
    icu_datetime_data::impl_datetime_buddhist_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_buddhist_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_chinese_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_chinese_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_coptic_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_coptic_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_dangi_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_dangi_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_ethiopic_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_ethiopic_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_gregory_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_gregory_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_hebrew_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_hebrew_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_indian_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_indian_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_islamic_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_islamic_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_japanese_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_japanese_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_japanext_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_japanext_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_persian_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_persian_datesymbols_v1!(Baked);
    icu_datetime_data::impl_datetime_roc_datelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_roc_datesymbols_v1!(Baked);
    #[cfg(feature = "experimental")]
    icu_datetime_data::impl_datetime_skeletons_v1!(Baked);
    icu_datetime_data::impl_datetime_timelengths_v1!(Baked);
    icu_datetime_data::impl_datetime_timesymbols_v1!(Baked);
    icu_datetime_data::impl_time_zone_exemplar_cities_v1!(Baked);
    icu_datetime_data::impl_time_zone_formats_v1!(Baked);
    icu_datetime_data::impl_time_zone_generic_long_v1!(Baked);
    icu_datetime_data::impl_time_zone_generic_short_v1!(Baked);
    icu_datetime_data::impl_time_zone_specific_long_v1!(Baked);
    icu_datetime_data::impl_time_zone_specific_short_v1!(Baked);
};
