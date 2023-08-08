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

// TODO(#3776): Improve the documentation of this datastruct.

/// The data struct representing [UTS #35 transform rules](https://unicode.org/reports/tr35/tr35-general.html#Transforms).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuleBasedTransliterator<'a> {
    /// Whether this transliterator is accessible directly through the constructor.
    /// Hidden transliterators are intended as dependencies for visible transliterators,
    /// see, e.g., [Devanagari-Latin](https://github.com/unicode-org/cldr/blob/main/common/transforms/Devanagari-Latin.xml)
    pub visibility: bool,
    /// The [`VarTable`] containing any special matchers (variables, UnicodeSets, ...) used by this transliterator.
    #[serde(borrow)]
    pub variable_table: VarTable<'a>,
    /// The filter for this transliterator. If there is none, the set of all code points is used.
    #[serde(borrow)]
    pub filter: CodePointInversionList<'a>,
    /// The list of transform rule groups this transliterator uses.
    #[serde(borrow)]
    pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIdULE>>,
    /// The list of conversion rule groups this transliterator uses.
    #[serde(borrow)]
    pub rule_group_list: VarZeroVec<'a, VarZeroSlice<RuleULE>>,
}

/// The ID of a transliterator plus an optional filter.
#[derive(Debug, Clone)]
#[make_varule(SimpleIdULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimpleId<'a> {
    /// The filter for the transliterator. If there is none, the set of all code points is used.
    #[zerovec::varule(CodePointInversionListULE)]
    #[serde(borrow)]
    pub filter: CodePointInversionList<'a>,
    /// The ID of the transliterator.
    #[serde(borrow)]
    pub id: Cow<'a, str>,
}

/// A conversion rule. The source patterns as well as the replacer use inlined private use characters
/// that refer to elements of the [`VarTable`] for special matchers (variables, UnicodeSets, ...).
#[make_varule(RuleULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rule<'a> {
    /// The pattern for the ante context. This is not replaced.
    #[serde(borrow)]
    pub ante: Cow<'a, str>,
    /// The pattern for the key. This is what gets replaced.
    #[serde(borrow)]
    pub key: Cow<'a, str>,
    /// The pattern for the post context. This is not replaced.
    #[serde(borrow)]
    pub post: Cow<'a, str>,
    /// The replacer. The key gets replaced with this.
    #[serde(borrow)]
    pub replacer: Cow<'a, str>,
    /// The offset of the cursor after this rule, if the rule matches.
    /// The end of the key/replacer is 0, i.e., a no-op.
    pub cursor_offset: i32,
}

/// The special matchers and replacers used by this transliterator.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VarTable<'a> {
    /// Variable definitions.
    #[serde(borrow)]
    pub compounds: VarZeroVec<'a, str>,
    /// Zero or one quantifiers.
    #[serde(borrow)]
    pub quantifiers_opt: VarZeroVec<'a, str>,
    /// Zero or more quantifiers.
    #[serde(borrow)]
    pub quantifiers_kleene: VarZeroVec<'a, str>,
    /// One or more quantifiers.
    #[serde(borrow)]
    pub quantifiers_kleene_plus: VarZeroVec<'a, str>,
    /// Segments.
    #[serde(borrow)]
    pub segments: VarZeroVec<'a, str>,
    /// UnicodeSets. These are represented as a [`CodePointInversionListAndStringList`](icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList)
    #[serde(borrow)]
    pub unicode_sets: VarZeroVec<'a, CodePointInversionListAndStringListULE>,
    /// Function calls.
    #[serde(borrow)]
    pub function_calls: VarZeroVec<'a, FunctionCallULE>,
}

/// An inline recursive call to a transliterator with an arbitrary argument.
#[make_varule(FunctionCallULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Debug)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionCall<'a> {
    /// The transliterator that will be called.
    #[zerovec::varule(SimpleIdULE)]
    #[serde(borrow)]
    pub translit: SimpleId<'a>,
    #[serde(borrow)]
    /// The argument to be transliterated given to the transliterator.
    pub arg: Cow<'a, str>,
}
