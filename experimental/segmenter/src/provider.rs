// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::boxed::Box;
use core::ops::Deref;
use icu_provider::prelude::*;
use zerovec::{ZeroSlice, ZeroVec};

pub(crate) mod line_data {
    include!(concat!(env!("OUT_DIR"), "/generated_line_table.rs"));
}

/// Pre-processed Unicode data in the form of tables to be used for line breaking.
#[icu_provider::data_struct(LineBreakDataV1Marker = "segmenter/line@1")]
pub struct LineBreakDataV1<'data> {
    /// Property table for line breaking.
    pub property_table: LineBreakPropertyTable<'data>,

    /// Break state table for line breaking.
    pub break_state_table: LineBreakStateTable<'data>,

    /// Number of properties; should be the square root of the length of
    /// [`Self::break_state_table`].
    pub property_count: u8,

    pub last_codepoint_property: i8,
    pub sot_property: u8,
    pub eot_property: u8,
    pub complex_property: u8,
}

impl Default for LineBreakDataV1<'static> {
    fn default() -> Self {
        Self {
            property_table: LineBreakPropertyTable::Borrowed(&line_data::PROPERTY_TABLE),
            break_state_table: LineBreakStateTable(
                ZeroSlice::from_ule_slice(&line_data::BREAK_STATE_MACHINE_TABLE).as_zerovec(),
            ),
            property_count: line_data::PROPERTY_COUNT,
            last_codepoint_property: line_data::LAST_CODEPOINT_PROPERTY,
            sot_property: line_data::PROP_SOT,
            eot_property: line_data::PROP_EOT,
            complex_property: line_data::PROP_COMPLEX,
        }
    }
}

/// Property table for line breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable)]
pub enum LineBreakPropertyTable<'data> {
    Borrowed(&'data [[u8; 1024]; 128]),
    Owned(Box<[[u8; 1024]; 128]>),
}

impl Deref for LineBreakPropertyTable<'_> {
    type Target = [[u8; 1024]; 128];
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => v,
        }
    }
}

impl<'zf> zerofrom::ZeroFrom<'zf, LineBreakPropertyTable<'_>> for LineBreakPropertyTable<'zf> {
    fn zero_from(cart: &'zf LineBreakPropertyTable<'_>) -> Self {
        LineBreakPropertyTable::Borrowed(&*cart)
    }
}

/// Break state table for line breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct LineBreakStateTable<'data>(pub ZeroVec<'data, i8>);

/// Pre-processed Unicode data in the form of tables to be used for rule-based breaking.
#[icu_provider::data_struct(
    WordBreakDataV1Marker = "segmenter/word@1",
    GraphemeClusterBreakDataV1Marker = "segmenter/grapheme@1",
    SentenceBreakDataV1Marker = "segmenter/sentence@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RuleBreakDataV1<'data> {
    /// Property table for rule-based breaking.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub property_table: RuleBreakPropertyTable<'data>,

    /// Break state table for rule-based breaking.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub break_state_table: RuleBreakStateTable<'data>,

    /// Number of properties; should be the square root of the length of [`Self::break_state_table`].
    pub property_count: u8,

    pub last_codepoint_property: i8,
    pub sot_property: u8,
    pub eot_property: u8,
    pub complex_property: u8,
}

/// Property table for rule-based breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RuleBreakPropertyTable<'data>(#[serde(borrow)] pub ZeroVec<'data, u8>);

/// Break state table for rule-based breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RuleBreakStateTable<'data>(#[serde(borrow)] pub ZeroVec<'data, i8>);

pub const ALL_KEYS: [ResourceKey; 3] = [
    GraphemeClusterBreakDataV1Marker::KEY,
    WordBreakDataV1Marker::KEY,
    SentenceBreakDataV1Marker::KEY,
];
