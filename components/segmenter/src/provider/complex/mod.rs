// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod lstm;
pub use lstm::*;
mod dictionary;
pub use dictionary::*;
#[cfg(feature = "unstable")]
mod radical;
#[cfg(feature = "unstable")]
pub use radical::*;

icu_provider::data_marker!(
    /// `SegmenterLstmWordLineAutoV1`
    SegmenterLstmAutoV1,
    "segmenter/lstm/auto/v1",
    LstmData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);

icu_provider::data_marker!(
    /// `SegmenterDictionaryWordAutoV1`
    SegmenterDictionaryAutoV1,
    "segmenter/dictionary/auto/v1",
    UCharDictionaryBreakData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);

icu_provider::data_marker!(
    /// `SegmenterDictionaryExtendedV1`
    SegmenterDictionaryExtendedV1,
    "segmenter/dictionary/extended/v1",
    UCharDictionaryBreakData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);

#[cfg(feature = "unstable")]
icu_provider::data_marker!(
    /// Marker for the singleton trie mapping code points to their Unihan IRG source radical IDs.
    SegmenterUnihanRadicalV1,
    "segmenter/unihan/radical/v1",
    UnihanRadicalsData<'static>,
    is_singleton = true
);
