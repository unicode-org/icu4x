// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for radicals

use icu_collections::codepointtrie::CodePointTrie;
use icu_provider::prelude::*;

/// Data for Unihan IRG sources (Radicals).
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UnihanIrgData<'data> {
    /// Trie mapping code points to their IRG source radical ID (u8).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u8>,
}

icu_provider::data_struct!(
    UnihanIrgData<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `SegmenterUnihanIrgV1`
    SegmenterUnihanIrgV1,
    "segmenter/unihan/irg/v1",
    UnihanIrgData<'static>,
    is_singleton = true
);
