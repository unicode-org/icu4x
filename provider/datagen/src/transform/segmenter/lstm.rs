// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by LSTM segmentation data.

use icu_locid::locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use ndarray::{Array, Array1, Array2, ArrayBase, Dim, Dimension, OwnedRepr};
use std::collections::HashMap;
use std::fmt::Debug;
use zerovec::{ule::UnvalidatedStr, ZeroVec};

// ndarray data structure in LSTM JSON data.
#[derive(serde::Deserialize, Debug)]
struct RawLstmMatrix {
    #[allow(dead_code)]
    pub v: i16,
    pub dim: Vec<usize>,
    pub data: Vec<f32>,
}

impl RawLstmMatrix {
    fn to_ndarray1(&self) -> Result<Array1<f32>, DataError> {
        if self.dim.len() == 1 {
            Ok(Array::from_vec(self.data.clone()))
        } else {
            Err(DIMENSION_MISMATCH_ERROR)
        }
    }

    fn to_ndarray2(&self) -> Result<Array2<f32>, DataError> {
        let [d0, d1] =
            *<&[usize; 2]>::try_from(self.dim.as_slice()).map_err(|_| DIMENSION_MISMATCH_ERROR)?;
        Array::from_shape_vec((d0, d1), self.data.clone()).map_err(|_| DIMENSION_MISMATCH_ERROR)
    }
}

// LSTM JSON data structure.
#[derive(serde::Deserialize, Debug)]
struct RawLstmData {
    model: String,
    dic: HashMap<String, u16>,
    #[serde(rename = "mat1")]
    embedding: RawLstmMatrix,
    #[serde(rename = "mat2")]
    fw_w: RawLstmMatrix,
    #[serde(rename = "mat3")]
    fw_u: RawLstmMatrix,
    #[serde(rename = "mat4")]
    fw_b: RawLstmMatrix,
    #[serde(rename = "mat5")]
    bw_w: RawLstmMatrix,
    #[serde(rename = "mat6")]
    bw_u: RawLstmMatrix,
    #[serde(rename = "mat7")]
    bw_b: RawLstmMatrix,
    #[serde(rename = "mat8")]
    time_w: RawLstmMatrix,
    #[serde(rename = "mat9")]
    time_b: RawLstmMatrix,
}

impl RawLstmData {
    pub fn try_convert(&self) -> Result<LstmDataV1<'static>, DataError> {
        let embedding = self.embedding.to_ndarray2()?;
        let fw_w = self.fw_w.to_ndarray2()?;
        let fw_u = self.fw_u.to_ndarray2()?;
        let fw_b = self.fw_b.to_ndarray1()?;
        let bw_w = self.bw_w.to_ndarray2()?;
        let bw_u = self.bw_u.to_ndarray2()?;
        let bw_b = self.bw_b.to_ndarray1()?;
        let time_w = self.time_w.to_ndarray2()?;
        let time_b = self.time_b.to_ndarray1()?;
        let embedd_dim = *embedding.shape().get(1).ok_or(DIMENSION_MISMATCH_ERROR)?;
        let hunits = *fw_u.shape().first().ok_or(DIMENSION_MISMATCH_ERROR)?;
        if fw_w.shape() != [embedd_dim, 4 * hunits]
            || fw_u.shape() != [hunits, 4 * hunits]
            || fw_b.shape() != [4 * hunits]
            || bw_w.shape() != [embedd_dim, 4 * hunits]
            || bw_u.shape() != [hunits, 4 * hunits]
            || bw_b.shape() != [4 * hunits]
            || time_w.shape() != [2 * hunits, 4]
            || time_b.shape() != [4]
        {
            return Err(DIMENSION_MISMATCH_ERROR);
        }
        // Unwraps okay: dimensions checked above
        let mut fw_w = fw_w.into_shape((embedd_dim, 4, hunits)).unwrap();
        let mut fw_u = fw_u.into_shape((hunits, 4, hunits)).unwrap();
        let fw_b = fw_b.into_shape((4, hunits)).unwrap();
        let mut bw_w = bw_w.into_shape((embedd_dim, 4, hunits)).unwrap();
        let mut bw_u = bw_u.into_shape((hunits, 4, hunits)).unwrap();
        let bw_b = bw_b.into_shape((4, hunits)).unwrap();
        let mut time_w = time_w.into_shape((2, hunits, 4)).unwrap();
        fw_w.swap_axes(0, 2);
        fw_w.swap_axes(0, 1);
        fw_u.swap_axes(0, 2);
        fw_u.swap_axes(0, 1);
        bw_w.swap_axes(0, 2);
        bw_w.swap_axes(0, 1);
        bw_u.swap_axes(0, 2);
        bw_u.swap_axes(0, 1);
        time_w.swap_axes(1, 2);
        let fw_w = fw_w.as_standard_layout().into_owned();
        let fw_u = fw_u.as_standard_layout().into_owned();
        let fw_b = fw_b.as_standard_layout().into_owned();
        let bw_w = bw_w.as_standard_layout().into_owned();
        let bw_u = bw_u.as_standard_layout().into_owned();
        let bw_b = bw_b.as_standard_layout().into_owned();
        let time_w = time_w.as_standard_layout().into_owned();

        assert_eq!(fw_w.shape(), [4, hunits, embedd_dim]);
        assert_eq!(fw_u.shape(), [4, hunits, hunits]);
        assert_eq!(fw_b.shape(), [4, hunits]);
        assert_eq!(bw_w.shape(), [4, hunits, embedd_dim]);
        assert_eq!(bw_u.shape(), [4, hunits, hunits]);
        assert_eq!(bw_b.shape(), [4, hunits]);
        assert_eq!(time_w.shape(), [2, 4, hunits]);
        assert_eq!(time_b.shape(), [4]);

        let model = if self.model.contains("_codepoints") {
            ModelType::Codepoints
        } else if self.model.contains("_graphclust_") {
            ModelType::GraphemeClusters
        } else {
            return Err(DataError::custom("Unknown model type"));
        };

        let lstm_data_float32 = LstmDataFloat32::try_from_parts(
            model,
            self.dic
                .iter()
                .map(|(k, &v)| (UnvalidatedStr::from_str(k), v))
                .collect(),
            ndarray_to_lstm_matrix2(embedding)?,
            ndarray_to_lstm_matrix3(fw_w)?,
            ndarray_to_lstm_matrix3(fw_u)?,
            ndarray_to_lstm_matrix2(fw_b)?,
            ndarray_to_lstm_matrix3(bw_w)?,
            ndarray_to_lstm_matrix3(bw_u)?,
            ndarray_to_lstm_matrix2(bw_b)?,
            ndarray_to_lstm_matrix3(time_w)?,
            ndarray_to_lstm_matrix1(time_b)?,
        )
        .map_err(|_| DataError::custom("Just checked the shapes"))?;
        Ok(LstmDataV1::Float32(lstm_data_float32))
    }
}

const DIMENSION_MISMATCH_ERROR: DataError = DataError::custom("LSTM dimension mismatch");

macro_rules! convert {
    ($fn_name:ident, $matrix_name:ident, $generic:literal) => {
        fn $fn_name(
            nd: ArrayBase<OwnedRepr<f32>, Dim<[usize; $generic]>>,
        ) -> Result<$matrix_name<'static>, DataError>
        where
            Dim<[usize; $generic]>: Dimension,
        {
            let dims = <[u16; $generic]>::try_from(
                nd.shape()
                    .iter()
                    .copied()
                    .map(u16::try_from)
                    .collect::<Result<Vec<u16>, _>>()
                    .map_err(|_| DataError::custom("LSTM bounds too big for u16"))?,
            )
            .map_err(|_| DIMENSION_MISMATCH_ERROR)?;
            let data = nd
                .as_slice_memory_order()
                .ok_or_else(|| DataError::custom("ndarray matrix not in memory order"))?;
            $matrix_name::from_parts(dims, ZeroVec::alloc_from_slice(data))
        }
    };
}

convert!(ndarray_to_lstm_matrix1, LstmMatrix1, 1);
convert!(ndarray_to_lstm_matrix2, LstmMatrix2, 2);
convert!(ndarray_to_lstm_matrix3, LstmMatrix3, 3);

impl DataProvider<LstmForWordLineAutoV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LstmForWordLineAutoV1Marker>, DataError> {
        let lstm_data = self
            .source
            .segmenter_lstm()?
            .read_and_parse_json::<RawLstmData>(&format!("lstm_{}.json", req.locale))
            .map_err(|_| DataErrorKind::MissingLocale.into_error())?;

        let data = lstm_data.try_convert()?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl IterableDataProvider<LstmForWordLineAutoV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![
            locale!("km").into(),
            locale!("lo").into(),
            locale!("my").into(),
            locale!("th").into(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_provider_adapters::any_payload::AnyPayloadProvider;
    use icu_provider_adapters::fork::ForkByKeyProvider;
    use icu_segmenter::LineSegmenter;

    fn load_lstm_data_directly(filename: &str) -> DataPayload<LstmForWordLineAutoV1Marker> {
        let json_str = std::fs::read(filename).unwrap();
        let raw_data: RawLstmData = serde_json::from_slice(&json_str).unwrap();
        let data = raw_data.try_convert().unwrap();
        DataPayload::from_owned(data)
    }

    #[test]
    fn thai_word_break_with_grapheme_model() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";
        // The keys of Lstm JSON data has to be sorted. So this JSON is generated by converter.py in data directory.
        let filename =
            "tests/data/segmenter/Thai_graphclust_exclusive_model4_heavy/converted_weights.json";
        let lstm_data = load_lstm_data_directly(filename);
        let provider = ForkByKeyProvider::new(
            AnyPayloadProvider::from_payload(lstm_data),
            crate::DatagenProvider::for_test(),
        );
        let segmenter = LineSegmenter::try_new_lstm_with_any_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        assert_eq!(
            breaks,
            [6, 12, 21, 27, 33, TEST_STR.len()],
            "Thai test with grapheme model"
        );
    }
}
