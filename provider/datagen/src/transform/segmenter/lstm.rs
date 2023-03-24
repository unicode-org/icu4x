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
    embedding: RawLstmMatrix,
    fw_w: RawLstmMatrix,
    fw_u: RawLstmMatrix,
    fw_b: RawLstmMatrix,
    bw_w: RawLstmMatrix,
    bw_u: RawLstmMatrix,
    bw_b: RawLstmMatrix,
    time_w: RawLstmMatrix,
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
        let mut fw_b = fw_b.into_shape((4, hunits)).unwrap();
        let mut bw_w = bw_w.into_shape((embedd_dim, 4, hunits)).unwrap();
        let mut bw_u = bw_u.into_shape((hunits, 4, hunits)).unwrap();
        let mut bw_b = bw_b.into_shape((4, hunits)).unwrap();
        let mut time_w = time_w.into_shape((2, hunits, 4)).unwrap();
        fw_w.swap_axes(0, 2);
        fw_u.swap_axes(0, 2);
        fw_b.swap_axes(0, 1);
        bw_w.swap_axes(0, 2);
        bw_u.swap_axes(0, 2);
        bw_b.swap_axes(0, 1);
        time_w.swap_axes(1, 2);
        let fw_w = fw_w.as_standard_layout().into_owned();
        let fw_u = fw_u.as_standard_layout().into_owned();
        let fw_b = fw_b.as_standard_layout().into_owned();
        let bw_w = bw_w.as_standard_layout().into_owned();
        let bw_u = bw_u.as_standard_layout().into_owned();
        let bw_b = bw_b.as_standard_layout().into_owned();
        let time_w = time_w.as_standard_layout().into_owned();

        Ok(LstmDataV1 {
            model: Cow::from(self.model.clone()),
            dic: self.dic.iter().map(|(k, v)| (k.as_str(), v)).collect(),
            embedding: ndarray_to_lstm_matrix(embedding)?,
            fw_w: ndarray_to_lstm_matrix(fw_w)?,
            fw_u: ndarray_to_lstm_matrix(fw_u)?,
            fw_b: ndarray_to_lstm_matrix(fw_b)?,
            bw_w: ndarray_to_lstm_matrix(bw_w)?,
            bw_u: ndarray_to_lstm_matrix(bw_u)?,
            bw_b: ndarray_to_lstm_matrix(bw_b)?,
            time_w: ndarray_to_lstm_matrix(time_w)?,
            time_b: ndarray_to_lstm_matrix(time_b)?,
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
