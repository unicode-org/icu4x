// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by LSTM segmentation data.

use icu_locid::locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use zerovec::ZeroVec;

// ndarray data structure in LSTM JSON data.
#[derive(serde::Deserialize, Debug)]
struct RawLstmMatrix {
    #[allow(dead_code)]
    pub v: i16,
    pub dim: Vec<i16>,
    pub data: Vec<f32>,
}

impl RawLstmMatrix {
    fn as_lstm_matrix(&self) -> LstmMatrix<'static> {
        LstmMatrix {
            dim: ZeroVec::alloc_from_slice(&self.dim),
            data: ZeroVec::alloc_from_slice(&self.data),
        }
    }
}

// LSTM JSON data structure.
#[derive(serde::Deserialize, Debug)]
struct RawLstmData {
    model: String,
    dic: HashMap<String, i16>,
    mat1: RawLstmMatrix,
    mat2: RawLstmMatrix,
    mat3: RawLstmMatrix,
    mat4: RawLstmMatrix,
    mat5: RawLstmMatrix,
    mat6: RawLstmMatrix,
    mat7: RawLstmMatrix,
    mat8: RawLstmMatrix,
    mat9: RawLstmMatrix,
}

impl DataProvider<LstmDataV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LstmDataV1Marker>, DataError> {
        let lstm_data = self
            .source
            .segmenter_lstm()?
            .read_and_parse_json::<RawLstmData>(&format!("lstm_{}.json", req.locale))
            .map_err(|_| DataErrorKind::MissingLocale.into_error())?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(LstmDataV1 {
                model: Cow::from(lstm_data.model.clone()),
                dic: lstm_data.dic.iter().map(|(k, v)| (k.as_str(), v)).collect(),
                mat1: lstm_data.mat1.as_lstm_matrix(),
                mat2: lstm_data.mat2.as_lstm_matrix(),
                mat3: lstm_data.mat3.as_lstm_matrix(),
                mat4: lstm_data.mat4.as_lstm_matrix(),
                mat5: lstm_data.mat5.as_lstm_matrix(),
                mat6: lstm_data.mat6.as_lstm_matrix(),
                mat7: lstm_data.mat7.as_lstm_matrix(),
                mat8: lstm_data.mat8.as_lstm_matrix(),
                mat9: lstm_data.mat9.as_lstm_matrix(),
            })),
        })
    }
}

impl IterableDataProvider<LstmDataV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![
            locale!("km").into(),
            locale!("lo").into(),
            locale!("my").into(),
            locale!("th").into(),
        ])
    }
}
