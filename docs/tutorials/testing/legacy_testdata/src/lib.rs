// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

pub fn unstable() -> UnstableDataProvider {
    UnstableDataProvider
}

pub struct UnstableDataProvider;

include!("../data/baked/macros/calendar_japanese_v1.data.rs");
__impl_calendar_japanese_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_indian_datelengths_v1.data.rs");
__impl_datetime_indian_datelengths_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_indian_datesymbols_v1.data.rs");
__impl_datetime_indian_datesymbols_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_japanese_datelengths_v1.data.rs");
__impl_datetime_japanese_datelengths_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_japanese_datesymbols_v1.data.rs");
__impl_datetime_japanese_datesymbols_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_timelengths_v1.data.rs");
__impl_datetime_timelengths_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_timesymbols_v1.data.rs");
__impl_datetime_timesymbols_v1!(UnstableDataProvider);

include!("../data/baked/macros/datetime_week_data_v1.data.rs");
__impl_datetime_week_data_v1!(UnstableDataProvider);

include!("../data/baked/macros/decimal_symbols_v1.data.rs");
__impl_decimal_symbols_v1!(UnstableDataProvider);

#[cfg(feature = "icu_displaynames")]
include!("../data/baked/macros/displaynames_regions_v1.data.rs");
#[cfg(feature = "icu_displaynames")]
__impl_displaynames_regions_v1!(UnstableDataProvider);

include!("../data/baked/macros/fallback_likelysubtags_v1.data.rs");
__impl_fallback_likelysubtags_v1!(UnstableDataProvider);

include!("../data/baked/macros/fallback_parents_v1.data.rs");
__impl_fallback_parents_v1!(UnstableDataProvider);

include!("../data/baked/macros/fallback_supplement_co_v1.data.rs");
__impl_fallback_supplement_co_v1!(UnstableDataProvider);

include!("../data/baked/macros/list_and_v1.data.rs");
__impl_list_and_v1!(UnstableDataProvider);

include!("../data/baked/macros/plurals_ordinal_v1.data.rs");
__impl_plurals_ordinal_v1!(UnstableDataProvider);
