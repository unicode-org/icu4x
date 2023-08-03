// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;

use icu_collections::codepointinvliststringlist::{
    CodePointInversionListAndStringList, CodePointInversionListAndStringListULE,
};
use zerovec::*;

// TODO(): Improve the documentation of this datastruct.

/// The data struct representing [UTS #35 transform rules](https://unicode.org/reports/tr35/tr35-general.html#Transforms).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RuleBasedTransliterator<'a> {
    /// Whether this transliterator is accessible directly through the constructor.
    /// Hidden transliterators are intended as dependencies for visible transliterators,
    /// see, e.g., [Devanagari-Latin](https://github.com/unicode-org/cldr/blob/main/common/transforms/Devanagari-Latin.xml)
    pub visibility: bool,
    /// The [`VarTable`] containing any special matchers (variables, UnicodeSets, ...) used by this transliterator.
    pub variable_table: VarTable<'a>,
    /// The filter for this transliterator. If there is none, the set of all code points is used.
    #[serde(borrow)]
    pub filter: CodePointInversionListAndStringList<'a>,
    /// The list of transform rule groups this transliterator uses.
    #[serde(borrow)]
    pub id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIDULE>>,
    /// The list of conversion rule groups this transliterator uses.
    #[serde(borrow)]
    pub rule_group_list: VarZeroVec<'a, VarZeroSlice<RuleULE>>,
}

/// The ID of a transliterator plus an optional filter.
#[derive(Debug, Clone)]
#[make_varule(SimpleIDULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SimpleID<'a> {
    /// The filter for the transliterator. If there is none, the set of all code points is used.
    #[zerovec::varule(CodePointInversionListAndStringListULE)]
    #[serde(borrow)]
    pub filter: CodePointInversionListAndStringList<'a>,
    /// The ID of the transliterator.
    #[serde(borrow)]
    pub id: Cow<'a, str>,
}

/// A conversion rule. The source patterns as well as the replacer use inlined private use characters
/// that refer to elements of the [`VarTable`] for special matchers (variables, UnicodeSets, ...).
#[make_varule(RuleULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Rule<'a> {
    /// If this is true, the rule only matches directly after the start of the input.
    pub anchored_to_start: bool,
    /// If this is true, the rule only matches if the end of the input follows immediately.
    pub anchored_to_end: bool,
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
#[derive(serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone)]
#[make_varule(FunctionCallULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FunctionCall<'a> {
    /// The transliterator that will be called.
    #[zerovec::varule(SimpleIDULE)]
    #[serde(borrow)]
    pub translit: SimpleID<'a>,
    #[serde(borrow)]
    /// The argument to be transliterated given to the transliterator.
    pub arg: Cow<'a, str>,
}
