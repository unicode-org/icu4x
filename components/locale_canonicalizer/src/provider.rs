// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::vec::Vec;
use icu_locid::{
    subtags::{Language, Region, Script},
    LanguageIdentifier,
};
use icu_provider::prelude::*;
use litemap::LiteMap;
use tinystr::TinyAsciiStr;
use zerovec::{ZeroMap, ZeroSlice};

type UnvalidatedLanguage = TinyAsciiStr<3>;
type UnvalidatedScript = TinyAsciiStr<4>;
type UnvalidatedRegion = TinyAsciiStr<3>;
type UnvalidatedVariant = TinyAsciiStr<8>;
type UnvalidatedValue = TinyAsciiStr<7>;

#[icu_provider::data_struct(AliasesV1Marker = "locale_canonicalizer/aliases@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
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
// TODO: Use validated types as value types
pub struct AliasesV1<'data> {
    /// Language data not covered by other rules, normally this will be empty.
    /// This is not a map as it's searched linearly according to the canonicalization rules.
    #[zerofrom(clone)]
    pub language: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    /// Language and variant.
    /// This is not a map as it's searched linearly according to the canonicalization rules.
    #[zerofrom(clone)]
    pub language_variants: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    /// Sign language and region data.
    #[zerofrom(clone)]
    pub sgn_region: LiteMap<UnvalidatedLanguage, LanguageIdentifier>,
    /// Two character language codes.
    #[zerofrom(clone)]
    pub language_len2: LiteMap<TinyAsciiStr<2>, LanguageIdentifier>,
    /// Three character language codes.
    #[zerofrom(clone)]
    pub language_len3: LiteMap<UnvalidatedLanguage, LanguageIdentifier>,
    /// Scripts.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub script: ZeroMap<'data, UnvalidatedScript, UnvalidatedScript>,
    /// Alphabetical region codes.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub region_alpha: ZeroMap<'data, TinyAsciiStr<2>, UnvalidatedRegion>,
    /// Numeric region codes.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub region_num: ZeroMap<'data, UnvalidatedRegion, UnvalidatedRegion>,
    /// Old regions which map to more than one new region.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub complex_region: ZeroMap<'data, UnvalidatedRegion, ZeroSlice<UnvalidatedRegion>>,
    /// Variants.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub variant: ZeroMap<'data, UnvalidatedVariant, UnvalidatedVariant>,
    /// Subdivisions.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub subdivision: ZeroMap<'data, UnvalidatedValue, UnvalidatedValue>,
}

#[icu_provider::data_struct(LikelySubtagsV1Marker = "locale_canonicalizer/likelysubtags@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
/// This likely subtags data is used for the minimize and maximize operations.
/// Each field defines a mapping from an old identifier to a new identifier,
/// based upon the rules in
/// <https://www.unicode.org/reports/tr35/#Likely_Subtags>.
///
/// The data is stored in sorted order, allowing for binary search to identify
/// rules to apply. It is broken down into smaller vectors based upon the rules
/// defined for the likely subtags maximize algorithm.
///
/// For efficiency, only the relevant part of the LanguageIdentifier is stored
/// for searching and replacing. E.g., the `language_script` field is used to store
/// rules for `LanguageIdentifier`s that contain a language and a script, but not a
/// region.
#[yoke(prove_covariance_manually)]
pub struct LikelySubtagsV1<'data> {
    /// Language and script.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub language_script: ZeroMap<'data, (UnvalidatedLanguage, UnvalidatedScript), Region>,
    /// Language and region.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub language_region: ZeroMap<'data, (UnvalidatedLanguage, UnvalidatedRegion), Script>,
    /// Just language.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub language: ZeroMap<'data, UnvalidatedLanguage, (Script, Region)>,
    /// Script and region.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub script_region: ZeroMap<'data, (UnvalidatedScript, UnvalidatedRegion), Language>,
    /// Just script.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub script: ZeroMap<'data, UnvalidatedScript, (Language, Region)>,
    /// Just region.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub region: ZeroMap<'data, UnvalidatedRegion, (Language, Script)>,
    /// Undefined.
    pub und: (Language, Script, Region),
}
