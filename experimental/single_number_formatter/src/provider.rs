// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]


// trait Currency {
//     const LONG_KEY: DataKey;
//     const SHORT_KEY: DataKey;
//     const NARROW_KEY: DataKey;
// }

// // Currencies
// struct USD;
// struct EUR;
// struct CHF;
// struct GBP;

// impl Currency for USD {
//     // NOTE: The data keys should come from data_struct trait
//     const LONG_KEY = data_key!("currency/usd/long");
//     const SHORT_KEY = data_key!("currency/usd/short");
//     const NARROW_KEY = data_key!("currency/usd/narrow"); // Pattern "foo {} bar" => "foo  bar", 4

// }

use alloc::borrow::Cow;
use icu_provider::{yoke, zerofrom};

#[icu_provider::data_struct(
    UsdLongV1Marker = "currency/usd/long@1",
    UsdShortV1Marker = "currency/usd/short@1",
    UsdNarrowV1Marker = "currency/usd/narrow@1"
)]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CurrencyData<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: CurrencyPattern<'data>
}




#[derive(Default, Clone, PartialEq, Debug, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CurrencyPattern<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: Cow<'data, str>,

    pub index: u8,
}


// https://github.com/unicode-org/icu/tree/main/icu4c/source/data/curr
// https://github.com/unicode-org/icu/tree/main/icu4c/source/data/unit 

// 1- Where is the formatting options (short, narrow, full)
// 2- where are the rounding preferences? where are the spacing rules?
// 3- why the data is duplicated


