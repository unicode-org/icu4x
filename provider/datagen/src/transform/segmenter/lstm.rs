// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by built-in segmentation data.

use crate::SourceData;
use icu_locid::{langid, locale};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use zerovec::ZeroMap;
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

/// A data provider reading from segmenter lstm files.
#[derive(Debug)]
pub struct SegmenterLstmProvider {
    source: SourceData,
}

impl From<&SourceData> for SegmenterLstmProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

impl SegmenterLstmProvider {
    // Generate LSTM Data for DataProvider from LSTM JSON.
    fn generate_data(&self, options: &DataOptions) -> Result<LstmDataV1<'static>, DataError> {
        let lstm_data: &RawLstmData = self
            .source
            .segmenter_lstm()?
            .segmenter_lstm()
            .read_and_parse::<RawLstmData>(
                Self::get_json_filename(options)
                    .ok_or_else(|| DataErrorKind::MissingDataOptions.into_error())?,
            )?;
        // The map of "dic" may not be sorted, so we cannot use ZeroMap directly.
        let mut dic: ZeroMap<'static, str, i16> = ZeroMap::new();
        lstm_data.dic.iter().for_each(|(k, v)| {
            dic.insert(k, v);
        });
        let mat1 = lstm_data.mat1.as_lstm_matrix();
        let mat2 = lstm_data.mat2.as_lstm_matrix();
        let mat3 = lstm_data.mat3.as_lstm_matrix();
        let mat4 = lstm_data.mat4.as_lstm_matrix();
        let mat5 = lstm_data.mat5.as_lstm_matrix();
        let mat6 = lstm_data.mat6.as_lstm_matrix();
        let mat7 = lstm_data.mat7.as_lstm_matrix();
        let mat8 = lstm_data.mat8.as_lstm_matrix();
        let mat9 = lstm_data.mat9.as_lstm_matrix();

        Ok(LstmDataV1 {
            model: Cow::from(lstm_data.model.clone()),
            dic,
            mat1,
            mat2,
            mat3,
            mat4,
            mat5,
            mat6,
            mat7,
            mat8,
            mat9,
        })
    }

    fn get_json_filename(options: &DataOptions) -> Option<&'static str> {
        if options.get_langid() == langid!("km") {
            Some("lstm_km.json")
        } else if options.get_langid() == langid!("lo") {
            Some("lstm_lo.json")
        } else if options.get_langid() == langid!("my") {
            Some("lstm_my.json")
        } else if options.get_langid() == langid!("th") {
            Some("lstm_th.json")
        } else {
            None
        }
    }
}

impl DataProvider<LstmDataV1Marker> for SegmenterLstmProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<LstmDataV1Marker>, DataError> {
        let lstm_data = self.generate_data(&req.options)?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(lstm_data)),
        })
    }
}

icu_provider::make_exportable_provider!(SegmenterLstmProvider, [LstmDataV1Marker,]);

impl IterableDataProvider<LstmDataV1Marker> for SegmenterLstmProvider {
    fn supported_options(&self) -> Result<Vec<DataOptions>, DataError> {
        Ok(vec![
            locale!("km").into(),
            locale!("lo").into(),
            locale!("my").into(),
            locale!("th").into(),
        ])
    }
}
