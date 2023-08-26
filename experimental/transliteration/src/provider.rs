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

use alloc::borrow::Cow;

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListULE},
    codepointinvliststringlist::CodePointInversionListAndStringListULE,
};
use zerovec::*;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    mod icu {
        pub use crate as transliteration;
        pub use icu_collections as collections;
    }
    icu_transliteration_data::impl_transliterator_rules_v1!(Baked);
};

// TODO(#3776): Improve the documentation of this datastruct.

/// The data struct representing [UTS #35 transform rules](https://unicode.org/reports/tr35/tr35-general.html#Transforms).
#[icu_provider::data_struct(TransliteratorRulesV1Marker = "transliterator/rules@1")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_transliteration::provider))]
pub struct RuleBasedTransliterator<'a> {
    /// Whether this transliterator is accessible directly through the constructor.
    /// Hidden transliterators are intended as dependencies for visible transliterators,
    /// see, e.g., [Devanagari-Latin](https://github.com/unicode-org/cldr/blob/main/common/transforms/Devanagari-Latin.xml)
    pub visibility: bool,
    /// The [`VarTable`] containing any special matchers (variables, UnicodeSets, ...) used by this transliterator.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub variable_table: VarTable<'a>,
    /// The filter for this transliterator. If there is none, the set of all code points is used.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub filter: CodePointInversionList<'a>,
    /// The list of transform rule groups this transliterator uses.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIdULE>>,
    /// The list of conversion rule groups this transliterator uses.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub rule_group_list: VarZeroVec<'a, VarZeroSlice<RuleULE>>,
    /// The direct dependencies of this transliterator.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dependencies: VarZeroVec<'a, str>,
}

/// The ID of a transliterator plus an optional filter.
#[derive(Debug, Clone)]
#[make_varule(SimpleIdULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct SimpleId<'a> {
    /// The filter for the transliterator. If there is none, the set of all code points is used.
    #[zerovec::varule(CodePointInversionListULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub filter: CodePointInversionList<'a>,
    /// The ID of the transliterator.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub id: Cow<'a, str>,
}

/// A conversion rule. The source patterns as well as the replacer use inlined private use characters
/// that refer to elements of the [`VarTable`] for special matchers (variables, UnicodeSets, ...).
#[derive(Debug, Clone)]
#[make_varule(RuleULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct Rule<'a> {
    /// The pattern for the ante context. This is not replaced.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ante: Cow<'a, str>,
    /// The pattern for the key. This is what gets replaced.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub key: Cow<'a, str>,
    /// The pattern for the post context. This is not replaced.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub post: Cow<'a, str>,
    /// The replacer. The key gets replaced with this.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub replacer: Cow<'a, str>,
}

/// The special matchers and replacers used by this transliterator.
#[derive(Debug, Clone, zerofrom::ZeroFrom, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_transliteration::provider))]
pub struct VarTable<'a> {
    /// Variable definitions.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub compounds: VarZeroVec<'a, str>,
    /// Zero or one quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_opt: VarZeroVec<'a, str>,
    /// Zero or more quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_kleene: VarZeroVec<'a, str>,
    /// One or more quantifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub quantifiers_kleene_plus: VarZeroVec<'a, str>,
    /// Segments.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub segments: VarZeroVec<'a, SegmentULE>,
    /// UnicodeSets. These are represented as a [`CodePointInversionListAndStringList`](icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub unicode_sets: VarZeroVec<'a, CodePointInversionListAndStringListULE>,
    /// Function calls.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub function_calls: VarZeroVec<'a, FunctionCallULE>,
    /// The maximum number of _left_ placeholders (`rest @@@ |`) in any rule.
    pub max_left_placeholder_count: u16,
    /// The maximum number of _right_ placeholders (`| @@@ rest`) in any rule.
    pub max_right_placeholder_count: u16,
}

/// Segments store matched parts of the input dynamically and can be referred to by back references
/// in the replacer.
#[derive(Debug, Clone)]
#[make_varule(SegmentULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct Segment<'a> {
    /// The 0-based index of this segment.
    pub idx: u16,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The content of the segment.
    pub content: Cow<'a, str>,
}

/// An inline recursive call to a transliterator with an arbitrary argument.
#[derive(Debug, Clone)]
#[make_varule(FunctionCallULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
pub struct FunctionCall<'a> {
    /// The transliterator that will be called.
    #[zerovec::varule(SimpleIdULE)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub translit: SimpleId<'a>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The argument to be transliterated given to the transliterator.
    pub arg: Cow<'a, str>,
}
