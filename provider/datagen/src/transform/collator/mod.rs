// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module transforms collation-related TOML files created by
//! `genrb -X` in the ICU4C repo to ICU4X-internal data structures.

mod transform;

pub use transform::CollationDataDataProvider;
pub use transform::CollationDiacriticsDataProvider;
pub use transform::CollationJamoDataProvider;
pub use transform::CollationMetadataDataProvider;
pub use transform::CollationReorderingDataProvider;
pub use transform::CollationSpecialPrimariesDataProvider;
pub use transform::ALL_KEYS;
