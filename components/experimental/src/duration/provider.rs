// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;

#[icu_provider::data_struct(DigitalDurationDataV1Marker = "duration/digital@1")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::duration::provider)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DigitalDurationDataV1 {
    // To represent the patterns from: durationUnit-type-hm
    pub hm: HmVariant,

    // To represent the patterns from: durationUnit-type-hms
    pub hms: HmsVariant,

    // To represent the patterns from: durationUnit-type-ms
    pub ms: MsVariant,
}

#[zerovec::make_ule(HmVariantULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::duration::provider)
)]
#[repr(u8)]
pub enum HmVariant {
    /// hh:mm
    DoubleHourColonSep = 0,
    /// h:mm
    SingleHourColonSep = 1,
    /// h.mm
    SingleHourDotSep = 2,
}

#[zerovec::make_ule(HmsVariantULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::duration::provider)
)]
#[repr(u8)]
pub enum HmsVariant {
    /// hh:mm:ss
    DoubleHourColonSep = 0,
    /// h:mm:ss
    SingleHourColonSep = 1,
    /// h.mm.ss
    SingleHourDotSep = 2,
}

#[zerovec::make_ule(MsVariantULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::duration::provider)
)]
#[repr(u8)]
pub enum MsVariant {
    /// mm:ss
    DoubleMinuteColonSep = 0,
    /// m:ss
    SingleMinuteColonSep = 1,
    /// m.ss
    SingleMinuteDotSep = 2,
}
