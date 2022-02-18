// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use litemap::LiteMap;
use ndarray::Array1;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 'LstmData' is a struct that store a LSTM model. Its attributes are:
/// `model`: name of the model
/// `dic`: the grapheme cluster dictionary used to train the model
/// `mat1`: the matrix associateed with embedding layer
/// `mat2` - `mat4`: the matrices associated with forward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
/// `mat5` - `mat7`: the matrices associated with backward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
/// `mat8` - `mat9`: the matrices associated with output layer (weight and bias term respectiely)
#[icu_provider::data_struct(LstmDataMarker)]
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct LstmData<'data> {
    #[serde(borrow)]
    pub model: Cow<'data, str>,
    // TODO: replacing LiteMap with ZeroMap if possible
    #[serde(borrow)]
    #[zerofrom(clone)]
    pub dic: LiteMap<Cow<'data, str>, i16>,
    #[zerofrom(clone)]
    pub mat1: Array2<f32>,
    #[zerofrom(clone)]
    pub mat2: Array2<f32>,
    #[zerofrom(clone)]
    pub mat3: Array2<f32>,
    #[zerofrom(clone)]
    pub mat4: Array1<f32>,
    #[zerofrom(clone)]
    pub mat5: Array2<f32>,
    #[zerofrom(clone)]
    pub mat6: Array2<f32>,
    #[zerofrom(clone)]
    pub mat7: Array1<f32>,
    #[zerofrom(clone)]
    pub mat8: Array2<f32>,
    #[zerofrom(clone)]
    pub mat9: Array1<f32>,
}
