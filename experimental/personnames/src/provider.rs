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

    /// ```xml
    /// <namePattern>{given} «{given2}» {surname}</namePattern>
    /// <namePattern alt="2">«{given2}» {surname}</namePattern>
    /// ```
    /// We consider that the person name has been choosen by the data provider through the options.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name_patterns: VarZeroVec<'data, str>,
}
