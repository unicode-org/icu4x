// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::boxed::Box;
use core::ops::Deref;
use icu_provider::prelude::*;
use zerovec::ZeroSlice;
use zerovec::ZeroVec;

pub(crate) mod line_data {
    include!(concat!(env!("OUT_DIR"), "/generated_line_table.rs"));
}

/// Pre-processed Unicode data in the form of tables to be used for line breaking.
#[icu_provider::data_struct(LineBreakDataV1Marker = "segmenter/line@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LineBreakDataV1<'data> {
    /// Property table for line breaking.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub property_table: LineBreakPropertyTable<'data>,

    /// Break state table for line breaking.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
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
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum LineBreakPropertyTable<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
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
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
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
#[derive(Debug, PartialEq, Clone, yoke::Yokeable)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum RuleBreakPropertyTable<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    Borrowed(&'data [[u8; 1024]; 897]),
    Owned(Box<[[u8; 1024]; 897]>),
}

impl<'data> Deref for RuleBreakPropertyTable<'data> {
    type Target = [[u8; 1024]; 897];
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => v,
        }
    }
}

impl<'zf> zerofrom::ZeroFrom<'zf, RuleBreakPropertyTable<'_>> for RuleBreakPropertyTable<'zf> {
    fn zero_from(cart: &'zf RuleBreakPropertyTable<'_>) -> Self {
        RuleBreakPropertyTable::Borrowed(&*cart)
    }
}

/// Break state table for rule-based breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RuleBreakStateTable<'data>(pub ZeroVec<'data, i8>);

/// A provider to load static rule-based breaking data generated by build.rs.
pub struct RuleBreakDataProvider;

mod grapheme_data {
    include!(concat!(env!("OUT_DIR"), "/generated_grapheme_table.rs"));
}

impl ResourceProvider<GraphemeClusterBreakDataV1Marker> for RuleBreakDataProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<GraphemeClusterBreakDataV1Marker>, DataError> {
        let metadata = DataResponseMetadata::default();
        let data = RuleBreakDataV1 {
            property_table: RuleBreakPropertyTable::Borrowed(&grapheme_data::PROPERTY_TABLE),
            break_state_table: RuleBreakStateTable(
                ZeroSlice::from_ule_slice(&grapheme_data::BREAK_STATE_MACHINE_TABLE).as_zerovec(),
            ),
            property_count: grapheme_data::PROPERTY_COUNT,
            last_codepoint_property: grapheme_data::LAST_CODEPOINT_PROPERTY,
            sot_property: grapheme_data::PROP_SOT,
            eot_property: grapheme_data::PROP_EOT,
            complex_property: grapheme_data::PROP_COMPLEX,
        };

        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

mod word_data {
    include!(concat!(env!("OUT_DIR"), "/generated_word_table.rs"));
}

impl ResourceProvider<WordBreakDataV1Marker> for RuleBreakDataProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<WordBreakDataV1Marker>, DataError> {
        let metadata = DataResponseMetadata::default();
        let data = RuleBreakDataV1 {
            property_table: RuleBreakPropertyTable::Borrowed(&word_data::PROPERTY_TABLE),
            break_state_table: RuleBreakStateTable(
                ZeroSlice::from_ule_slice(&word_data::BREAK_STATE_MACHINE_TABLE).as_zerovec(),
            ),
            property_count: word_data::PROPERTY_COUNT,
            last_codepoint_property: word_data::LAST_CODEPOINT_PROPERTY,
            sot_property: word_data::PROP_SOT,
            eot_property: word_data::PROP_EOT,
            complex_property: word_data::PROP_COMPLEX,
        };

        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

mod sentence_data {
    include!(concat!(env!("OUT_DIR"), "/generated_sentence_table.rs"));
}

impl ResourceProvider<SentenceBreakDataV1Marker> for RuleBreakDataProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<SentenceBreakDataV1Marker>, DataError> {
        let metadata = DataResponseMetadata::default();
        let data = RuleBreakDataV1 {
            property_table: RuleBreakPropertyTable::Borrowed(&sentence_data::PROPERTY_TABLE),
            break_state_table: RuleBreakStateTable(
                ZeroSlice::from_ule_slice(&sentence_data::BREAK_STATE_MACHINE_TABLE).as_zerovec(),
            ),
            property_count: sentence_data::PROPERTY_COUNT,
            last_codepoint_property: sentence_data::LAST_CODEPOINT_PROPERTY,
            sot_property: sentence_data::PROP_SOT,
            eot_property: sentence_data::PROP_EOT,
            complex_property: sentence_data::PROP_COMPLEX,
        };

        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}
