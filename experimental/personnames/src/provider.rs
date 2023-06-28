// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
#![warn(unused_imports)]
//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;

use icu_provider::prelude::*;
use zerovec::VarZeroVec;

/// This is the equivalent of
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#personnames-element
///
/// There are some liberties taken from the DTD definition,
/// as we enforce uniqueness on some tags where the documentation doesn't specify any of these
/// constraints.
///
/// e.g. : initialPattern has no upper bound, DTD allows for the element to be specified any number
/// of times, while in this implementation we are restraining it to the 2 documented types
/// (`initial`, `initialSequence`).
#[icu_provider::data_struct(PersonNamesFormattingDefinitionV1Marker = "personnames/personnames@1")]
#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "datagen",
derive(serde::Serialize, databake::Bake),
databake(path = icu_personnames::provider))
]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PersonNamesFormattingDefinitionV1<'data> {
    /// <nameOrderLocales order="surnameFirst">ko vi yue zh</nameOrderLocales>
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub surname_first_locales: VarZeroVec<'data, str>,

    /// <nameOrderLocales order="givenFirst">und en</nameOrderLocales>
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub given_first_locales: VarZeroVec<'data, str>,

    /// foreignSpaceReplacement element.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub foreign_space_replacement: Option<Cow<'data, str>>,

    /// Equivalent of initialPattern tag + initial
    /// ```xml
    /// <initialPattern type="initial">{0}.</initialPattern>
    /// ```
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub initial_pattern: Option<Cow<'data, str>>,

    /// Equivalent of initialPattern tag + initialSequence
    /// ```xml
    /// <initialPattern type="initialSequence">{0} {1}</initialPattern>
    /// ```
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub initial_pattern_sequence: Option<Cow<'data, str>>,

    /// Equivalent of PersonNames
    /// ```xml
    /// <personName>...</personName>
    /// ```
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub person_names_patterns: VarZeroVec<'data, PersonNamesFormattingDataVarULE>,
}

/// Person Name Attributes.
/// {order=givenFirst, length=long, usage=referring, formality=formal}
/// see https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element
#[repr(u32)]
pub enum PersonNamesFormattingAttributes {
    // Order
    GivenFirst = 1 << 0,
    SurnameFirst = 1 << 1,
    Sorting = 1 << 2,
    // Length
    Short = 1 << 3,
    Medium = 1 << 4,
    Long = 1 << 5,
    // Usage
    Addressing = 1 << 6,
    Referring = 1 << 7,
    Monogram = 1 << 8,
    // Formality
    Formal = 1 << 9,
    Informal = 1 << 10,
}

type PersonNamesFormattingAttributesMask = u32;

/// PersonName Formatting data.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element
#[zerovec::make_varule(PersonNamesFormattingDataVarULE)]
#[zerovec::skip_derive(ZeroMapKV, Ord)]
#[cfg_attr(feature = "datagen",
derive(serde::Serialize, databake::Bake),
databake(path = icu_personnames::provider),
zerovec::derive(Serialize))
]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
pub struct PersonNamesFormattingData<'data> {
    /// Attributes
    /// https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element
    pub attributes: PersonNamesFormattingAttributesMask,
    /// https://www.unicode.org/reports/tr35/tr35-personNames.html#namepattern-syntax
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, str>,
}
