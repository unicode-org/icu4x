// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

pub fn unstable() -> UnstableDataProvider {
    UnstableDataProvider
}

pub struct UnstableDataProvider;

include!("../data/baked/macros/decimal_symbols_v1.data.rs");
__impl_decimal_symbols_v1!(UnstableDataProvider);

include!("../data/baked/macros/list_and_v1.data.rs");
__impl_list_and_v1!(UnstableDataProvider);

#[cfg(feature = "icu_displaynames")]
include!("../data/baked/macros/displaynames_regions_v1.data.rs");
#[cfg(feature = "icu_displaynames")]
__impl_displaynames_regions_v1!(UnstableDataProvider);
