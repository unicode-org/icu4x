// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "lstm")]
use crate::lstm_error::Error;

use alloc::borrow::Cow;
use icu_provider::prelude::*;
#[cfg(feature = "lstm")]
use ndarray::{Array, Array1, Array2};
use zerovec::{ZeroMap, ZeroVec};

#[derive(PartialEq, Debug, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter),
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

/// 'LstmDataV1' is a struct that store a LSTM model. Its attributes are:
/// `model`: name of the model
/// `dic`: the grapheme cluster dictionary used to train the model
/// `mat1`: the matrix associateed with embedding layer
/// `mat2` - `mat4`: the matrices associated with forward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
/// `mat5` - `mat7`: the matrices associated with backward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
/// `mat8` - `mat9`: the matrices associated with output layer (weight and bias term respectiely)
#[icu_provider::data_struct(LstmDataV1Marker = "segmenter/lstm@1")]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LstmDataV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub model: Cow<'data, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dic: ZeroMap<'data, str, i16>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat1: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat2: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat3: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat4: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat5: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat6: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat7: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat8: LstmMatrix<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub mat9: LstmMatrix<'data>,
}
