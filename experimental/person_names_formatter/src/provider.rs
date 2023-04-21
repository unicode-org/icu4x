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

use bitmask_enum::bitmask;

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
///
#[icu_provider::data_struct(
    PersonNamesFormattingDefinitionV1Marker = "person_names/person_names@1"
)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "datagen", derive(Serialize))]
pub struct PersonNamesFormattingDefinitionV1<'data> {
    /// <nameOrderLocales order="surnameFirst">ko vi yue zh</nameOrderLocales>
    pub surname_first_locales: VarZeroVec<'data, str>,
    /// <nameOrderLocales order="givenFirst">und en</nameOrderLocales>
    pub given_first_locales: VarZeroVec<'data, str>,
    /// foreignSpaceReplacement element.
    pub foreign_space_replacement: Option<Cow<'data, str>>,

    /// Equivalent of initialPattern tag + initial
    /// ```xml
    /// <initialPattern type="initial">{0}.</initialPattern>
    /// ```
    pub initial_pattern: Option<Cow<'data, str>>,

    ///
    /// Equivalent of initialPattern tag + initialSequence
    /// ```xml
    /// <initialPattern type="initialSequence">{0} {1}</initialPattern>
    /// ```
    pub initial_pattern_sequence: Option<Cow<'data, str>>,
    ///
    /// Equivalent of PersonNames
    /// ```xml
    /// <personName>...</personName>
    /// ```
    pub person_names_patterns: VarZeroVec<'data, PersonNamesFormattingDataVARULE>,
}

///
/// Person Name Attributes.
/// {order=givenFirst, length=long, usage=referring, formality=formal}
/// see https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element
#[bitmask(u16)]
#[zerovec::make_ule(PersonNamesFormattingAttributesULE)]
pub enum PersonNamesFormattingAttributes {
    // Order
    GivenFirst,
    SurnameFirst,
    Sorting,
    // Length
    Short,
    Medium,
    Long,
    // Usage
    Addressing,
    Referring,
    Monogram,
    // Formality
    Formal,
    Informal,
}

///
/// PersonName Formatting data.
///
/// https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element
#[zerovec::make_varule(PersonNamesFormattingDataVARULE)]
#[zerovec::skip_derive(ZeroMapKV, Ord)]
pub struct PersonNamesFormattingData<'data> {
    /// Attributes
    /// https://www.unicode.org/reports/tr35/tr35-personNames.html#personname-element
    pub attributes: PersonNamesFormattingAttributes,
    /// https://www.unicode.org/reports/tr35/tr35-personNames.html#namepattern-syntax
    pub patterns: VarZeroVec<'data, str>,
}
