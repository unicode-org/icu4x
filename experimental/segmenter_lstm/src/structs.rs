// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::Array1;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 'LstmData' is a struct that store a LSTM model. Its attributes are:
/// `model`: name of the model
/// `dic`: the grapheme cluster dictionary used to train the model
/// `mat1`: the matrix associateed with embedding layer
/// `mat2` - `mat4`: the matrices associated with forward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
/// `mat5` - `mat7`: the matrices associated with backward LSTM layer (embedding to hunits, hunits to hunits, and bias respectively)
/// `mat8` - `mat9`: the matrices associated with output layer (weight and bias term respectiely)
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct LstmData {
    pub model: String,
    pub dic: HashMap<String, i16>,
    pub mat1: Array2<f32>,
    pub mat2: Array2<f32>,
    pub mat3: Array2<f32>,
    pub mat4: Array1<f32>,
    pub mat5: Array2<f32>,
    pub mat6: Array2<f32>,
    pub mat7: Array1<f32>,
    pub mat8: Array2<f32>,
    pub mat9: Array1<f32>,
}
