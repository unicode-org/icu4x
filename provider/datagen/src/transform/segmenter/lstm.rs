// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by LSTM segmentation data.

use icu_locid::locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use ndarray::{Array, Array1, Array2, ArrayBase, Dim, Dimension, OwnedRepr};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use zerovec::ZeroVec;

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
            Err(DataError::custom("LSTM dimension mismatch"))
        }
    }

    fn to_ndarray2(&self) -> Result<Array2<f32>, DataError> {
        let [d0, d1] =
            *<&[usize; 2]>::try_from(self.dim.as_slice()).map_err(|_| DIMENSION_MISMATCH_ERROR)?;
        Array::from_shape_vec((d0, d1), self.data.clone())
            .map_err(|_| DataError::custom("LSTM dimension mismatch"))
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

impl RawLstmData {
    pub fn try_convert(&self) -> Result<LstmDataV1<'static>, DataError> {
        let mat1 = self.mat1.to_ndarray2()?;
        let mat2 = self.mat2.to_ndarray2()?;
        let mat3 = self.mat3.to_ndarray2()?;
        let mat4 = self.mat4.to_ndarray1()?;
        let mat5 = self.mat5.to_ndarray2()?;
        let mat6 = self.mat6.to_ndarray2()?;
        let mat7 = self.mat7.to_ndarray1()?;
        let mat8 = self.mat8.to_ndarray2()?;
        let mat9 = self.mat9.to_ndarray1()?;
        let embedd_dim = *mat1.shape().get(1).ok_or(DIMENSION_MISMATCH_ERROR)?;
        let hunits = *mat3.shape().first().ok_or(DIMENSION_MISMATCH_ERROR)?;
        if mat2.shape() != [embedd_dim, 4 * hunits]
            || mat3.shape() != [hunits, 4 * hunits]
            || mat4.shape() != [4 * hunits]
            || mat5.shape() != [embedd_dim, 4 * hunits]
            || mat6.shape() != [hunits, 4 * hunits]
            || mat7.shape() != [4 * hunits]
            || mat8.shape() != [2 * hunits, 4]
            || mat9.shape() != [4]
        {
            return Err(DIMENSION_MISMATCH_ERROR);
        }
        // Unwraps okay: dimensions checked above
        let mut mat2 = mat2.into_shape((embedd_dim, 4, hunits)).unwrap();
        let mut mat3 = mat3.into_shape((hunits, 4, hunits)).unwrap();
        let mut mat4 = mat4.into_shape((4, hunits)).unwrap();
        let mut mat5 = mat5.into_shape((embedd_dim, 4, hunits)).unwrap();
        let mut mat6 = mat6.into_shape((hunits, 4, hunits)).unwrap();
        let mut mat7 = mat7.into_shape((4, hunits)).unwrap();
        let mut mat8 = mat8.into_shape((2, hunits, 4)).unwrap();
        mat2.swap_axes(0, 2);
        mat3.swap_axes(0, 2);
        mat4.swap_axes(0, 1);
        mat5.swap_axes(0, 2);
        mat6.swap_axes(0, 2);
        mat7.swap_axes(0, 1);
        mat8.swap_axes(1, 2);
        let mat2 = mat2.as_standard_layout().into_owned();
        let mat3 = mat3.as_standard_layout().into_owned();
        let mat4 = mat4.as_standard_layout().into_owned();
        let mat5 = mat5.as_standard_layout().into_owned();
        let mat6 = mat6.as_standard_layout().into_owned();
        let mat7 = mat7.as_standard_layout().into_owned();
        let mat8 = mat8.as_standard_layout().into_owned();

        Ok(LstmDataV1 {
            model: Cow::from(self.model.clone()),
            dic: self.dic.iter().map(|(k, v)| (k.as_str(), v)).collect(),
            mat1: ndarray_to_lstm_matrix(mat1)?,
            mat2: ndarray_to_lstm_matrix(mat2)?,
            mat3: ndarray_to_lstm_matrix(mat3)?,
            mat4: ndarray_to_lstm_matrix(mat4)?,
            mat5: ndarray_to_lstm_matrix(mat5)?,
            mat6: ndarray_to_lstm_matrix(mat6)?,
            mat7: ndarray_to_lstm_matrix(mat7)?,
            mat8: ndarray_to_lstm_matrix(mat8)?,
            mat9: ndarray_to_lstm_matrix(mat9)?,
        })
    }
}

const DIMENSION_MISMATCH_ERROR: DataError = DataError::custom("LSTM dimension mismatch");

fn ndarray_to_lstm_matrix<const D: usize>(
    nd: ArrayBase<OwnedRepr<f32>, Dim<[usize; D]>>,
) -> Result<LstmMatrix<'static>, DataError>
where
    Dim<[usize; D]>: Dimension,
{
    let dims: Vec<u16> = nd
        .shape()
        .iter()
        .copied()
        .map(u16::try_from)
        .collect::<Result<Vec<u16>, _>>()
        .map_err(|_| DataError::custom("LSTM bounds too big for u16"))?;
    let data = nd
        .as_slice_memory_order()
        .ok_or_else(|| DataError::custom("ndarray matrix not in memory order"))?;
    Ok(LstmMatrix {
        data: ZeroVec::alloc_from_slice(data),
        dims: ZeroVec::alloc_from_slice(&dims),
    })
}

impl DataProvider<LstmDataV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LstmDataV1Marker>, DataError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use icu_provider_adapters::any_payload::AnyPayloadProvider;
    use icu_provider_adapters::fork::ForkByKeyProvider;
    use icu_segmenter::LineSegmenter;

    fn load_lstm_data_directly(filename: &str) -> DataPayload<LstmDataV1Marker> {
        let json_str = std::fs::read(filename).unwrap();
        let raw_data: RawLstmData = serde_json::from_slice(&json_str).unwrap();
        let data = raw_data.try_convert().unwrap();
        DataPayload::from_owned(data)
    }

    #[test]
    fn burmese_word_break() {
        // "Burmese Language" in Burmese
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        let provider = crate::DatagenProvider::for_test();
        let segmenter = LineSegmenter::try_new_lstm_unstable(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // LSTM model breaks more characters, but it is better to return [30].
        assert_eq!(breaks, [12, 18, 30, TEST_STR.len()], "Burmese test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        // LSTM model breaks more characters, but it is better to return [10].
        assert_eq!(breaks, [4, 6, 10, utf16.len()], "Burmese utf-16 test");
    }

    #[test]
    fn khmer_word_break() {
        const TEST_STR: &str = "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស";

        let provider = crate::DatagenProvider::for_test();
        let segmenter = LineSegmenter::try_new_lstm_unstable(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: This small sample matches the ICU dictionary segmenter
        assert_eq!(breaks, [39, 48, 54, 72, TEST_STR.len()], "Khmer test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [13, 16, 18, 24, utf16.len()], "Khmer utf-16 test");
    }

    #[test]
    fn lao_word_break() {
        const TEST_STR: &str = "ກ່ຽວກັບສິດຂອງມະນຸດ";

        let provider = crate::DatagenProvider::for_test();
        let segmenter = LineSegmenter::try_new_lstm_unstable(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: LSTM finds a break at '12' that the dictionary does not find
        assert_eq!(breaks, [12, 21, 30, 39, TEST_STR.len()], "Lao test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 10, 13, utf16.len()], "Lao utf-16 test");
    }

    #[test]
    fn thai_word_break_with_grapheme_model() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";
        // The keys of Lstm JSON data has to be sorted. So this JSON is generated by converter.py in data directory.
        let filename =
            "tests/data/segmenter/Thai_graphclust_exclusive_model4_heavy/converted_weights.json";
        let lstm_data = load_lstm_data_directly(filename);
        assert_eq!(
            lstm_data.get().model,
            "Thai_graphclust_exclusive_model4_heavy"
        );
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
