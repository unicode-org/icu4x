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
use icu_locid::subtags::{Language, Region, Script, Variant};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap, ZeroSlice};

// We use raw TinyAsciiStrs for map keys, as we then don't have to
// validate them as subtags on deserialization. Map lookup can be
// done even if they are not valid tags (an invalid key will just
// become inaccessible).
type UnvalidatedLanguage = TinyAsciiStr<3>;
type UnvalidatedScript = TinyAsciiStr<4>;
type UnvalidatedRegion = TinyAsciiStr<3>;
type UnvalidatedVariant = TinyAsciiStr<8>;
type UnvalidatedSubdivision = TinyAsciiStr<7>;

// LanguageIdentifier doesn't have an AsULE implementation, so we have
// to store strs and parse when needed.
type UnvalidatedLanguageIdentifier = str;
type UnvalidatedLanguageIdentifierPair = StrStrPairVarULE;

#[zerovec::make_varule(StrStrPairVarULE)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    zerovec::derive(Serialize),
    databake(path = icu_locid_transform::provider),
)]
/// A pair of strings with a EncodeAsVarULE implementation.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
pub struct StrStrPair<'a>(
    #[cfg_attr(feature = "serde", serde(borrow))] pub Cow<'a, str>,
    #[cfg_attr(feature = "serde", serde(borrow))] pub Cow<'a, str>,
);

#[icu_provider::data_struct(AliasesV1Marker = "locid_transform/aliases@1")]
#[derive(PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_locid_transform::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
/// This alias data is used for locale canonicalization. Each field defines a
/// mapping from an old identifier to a new identifier, based upon the rules in
/// from <http://unicode.org/reports/tr35/#LocaleId_Canonicalization>. The data
/// is stored in sorted order, allowing for binary search to identify rules to
/// apply. It is broken down into smaller vectors based upon some characteristic
/// of the data, to help avoid unnecessary searches. For example, the `sgn_region`
/// field contains aliases for sign language and region, so that it is not
/// necessary to search the data unless the input is a sign language.
///
/// The algorithm in tr35 is not guaranteed to terminate on data other than what
/// is currently in CLDR. For this reason, it is not a good idea to attempt to add
/// or modify aliases for use in this structure.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
// TODO: Use validated types as value types
pub struct AliasesV1<'data> {
    /// `[language(-variant)+\] -> [langid]`
    /// This is not a map as it's searched linearly according to the canonicalization rules.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language_variants: VarZeroVec<'data, UnvalidatedLanguageIdentifierPair>,
    /// `sgn-[region] -> [language]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub sgn_region: ZeroMap<'data, UnvalidatedRegion, Language>,
    /// `[language{2}] -> [langid]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language_len2: ZeroMap<'data, TinyAsciiStr<2>, UnvalidatedLanguageIdentifier>,
    /// `[language{3}] -> [langid]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language_len3: ZeroMap<'data, UnvalidatedLanguage, UnvalidatedLanguageIdentifier>,
    /// `[langid] -> [langid]`
    /// This is not a map as it's searched linearly according to the canonicalization rules.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language: VarZeroVec<'data, UnvalidatedLanguageIdentifierPair>,

    /// `[script] -> [script]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub script: ZeroMap<'data, UnvalidatedScript, Script>,

    /// `[region{2}] -> [region]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub region_alpha: ZeroMap<'data, TinyAsciiStr<2>, Region>,
    /// `[region{3}] -> [region]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub region_num: ZeroMap<'data, UnvalidatedRegion, Region>,

    /// `[region] -> [region]+`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub complex_region: ZeroMap<'data, UnvalidatedRegion, ZeroSlice<Region>>,

    /// `[variant] -> [variant]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub variant: ZeroMap<'data, UnvalidatedVariant, Variant>,

    /// `[value{7}] -> [value{7}]`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub subdivision: ZeroMap<'data, UnvalidatedSubdivision, UnvalidatedSubdivision>,
}

#[icu_provider::data_struct(LikelySubtagsV1Marker = "locid_transform/likelysubtags@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_locid_transform::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
/// This likely subtags data is used for the minimize and maximize operations.
/// Each field defines a mapping from an old identifier to a new identifier,
/// based upon the rules in
/// <https://www.unicode.org/reports/tr35/#Likely_Subtags>.
///
/// The data is stored is broken down into smaller vectors based upon the rules
/// defined for the likely subtags maximize algorithm.
///
/// For efficiency, only the relevant part of the LanguageIdentifier is stored
/// for searching and replacing. E.g., the `language_script` field is used to store
/// rules for `LanguageIdentifier`s that contain a language and a script, but not a
/// region.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[yoke(prove_covariance_manually)]
pub struct LikelySubtagsV1<'data> {
    /// Language and script.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language_script: ZeroMap<'data, (UnvalidatedLanguage, UnvalidatedScript), Region>,
    /// Language and region.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language_region: ZeroMap<'data, (UnvalidatedLanguage, UnvalidatedRegion), Script>,
    /// Just language.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub language: ZeroMap<'data, UnvalidatedLanguage, (Script, Region)>,
    /// Script and region.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub script_region: ZeroMap<'data, (UnvalidatedScript, UnvalidatedRegion), Language>,
    /// Just script.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub script: ZeroMap<'data, UnvalidatedScript, (Language, Region)>,
    /// Just region.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub region: ZeroMap<'data, UnvalidatedRegion, (Language, Script)>,
    /// Undefined.
    pub und: (Language, Script, Region),
}
