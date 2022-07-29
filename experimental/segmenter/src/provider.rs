// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_codepointtrie::CodePointTrie;
use icu_provider::prelude::*;
use zerovec::{ZeroMap, ZeroVec};

#[cfg(feature = "lstm")]
use crate::lstm_error::Error;
#[cfg(feature = "lstm")]
use ndarray::{Array, Array1, Array2};

/// Pre-processed Unicode data in the form of tables to be used for rule-based breaking.
#[icu_provider::data_struct(
    LineBreakDataV1Marker = "segmenter/line@1",
    WordBreakDataV1Marker = "segmenter/word@1",
    GraphemeClusterBreakDataV1Marker = "segmenter/grapheme@1",
    SentenceBreakDataV1Marker = "segmenter/sentence@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RuleBreakDataV1<'data> {
    /// Property table for rule-based breaking.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub property_table: RuleBreakPropertyTable<'data>,

    /// Break state table for rule-based breaking.
    #[cfg_attr(feature = "serde", serde(borrow))]
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
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RuleBreakPropertyTable<'data>(
    #[cfg_attr(feature = "serde", serde(borrow))] pub CodePointTrie<'data, u8>,
);

/// Break state table for rule-based breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RuleBreakStateTable<'data>(
    #[cfg_attr(feature = "serde", serde(borrow))] pub ZeroVec<'data, i8>,
);

/// char16trie data for dictionary break
#[icu_provider::data_struct(UCharDictionaryBreakDataV1Marker = "segmenter/dictionary@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UCharDictionaryBreakDataV1<'data> {
    /// Dictionary data of char16trie.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie_data: ZeroVec<'data, u16>,
}

/// The struct that stores a LSTM's matrix.
#[derive(PartialEq, Debug, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LstmMatrix<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dim: ZeroVec<'data, i16>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub data: ZeroVec<'data, f32>,
}

#[cfg(feature = "lstm")]
impl<'data> LstmMatrix<'data> {
    pub fn as_ndarray1(&self) -> Result<Array1<f32>, Error> {
        if self.dim.len() == 1 {
            Ok(Array::from_vec(self.data.to_vec()))
        } else {
            Err(Error::DimensionMismatch)
        }
    }

    pub fn as_ndarray2(&self) -> Result<Array2<f32>, Error> {
        if self.dim.len() == 2 {
            Array::from_shape_vec(
                (
                    self.dim.get(0).unwrap() as usize,
                    self.dim.get(1).unwrap() as usize,
                ),
                self.data.to_vec(),
            )
            .map_err(|_| Error::DimensionMismatch)
        } else {
            Err(Error::DimensionMismatch)
        }
    }
}

/// The struct that stores a LSTM model.
#[icu_provider::data_struct(LstmDataV1Marker = "segmenter/lstm@1")]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LstmDataV1<'data> {
    /// Name of the model
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub model: Cow<'data, str>,
    /// The grapheme cluster dictionary used to train the model
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dic: ZeroMap<'data, str, i16>,
    /// The matrix associateed with embedding layer
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat1: LstmMatrix<'data>,
    /// The matrices associated with forward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat2: LstmMatrix<'data>,
    /// The matrices associated with forward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat3: LstmMatrix<'data>,
    /// The matrices associated with forward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat4: LstmMatrix<'data>,
    /// The matrices associated with backward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat5: LstmMatrix<'data>,
    /// The matrices associated with backward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat6: LstmMatrix<'data>,
    /// The matrices associated with backward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat7: LstmMatrix<'data>,
    /// The matrices associated with output layer (weight and bias term respectiely)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat8: LstmMatrix<'data>,
    /// The matrices associated with output layer (weight and bias term respectiely)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat9: LstmMatrix<'data>,
}
