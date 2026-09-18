// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Provider implementations backed by built-in `AdaBoost` segmentation models.

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::segmenter::provider::adaboost::{
    AdaboostData, CjAdaboostData, JapaneseAdaboostData, SegmenterChineseAutoV1,
    SegmenterCjAutoV1, SegmenterJapaneseAutoV1, SegmenterThaiAutoV1, ThaiAdaboostData,
};
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use zerovec::{ZeroMap, maps::ZeroMapKV};

const CHINESE_ADABOOST_ID: &str = "Chinese_adaboost";
const CJ_ADABOOST_ID: &str = "CJ_adaboost";
const JAPANESE_ADABOOST_ID: &str = "Japanese_adaboost";
const THAI_ADABOOST_ID: &str = "Thai_adaboost";
const CHINESE_MODEL_JSON: &str = include_str!("../../data/segmenter/model.json");
const CJ_MODEL_JSON: &str = include_str!("../../data/segmenter/model_cj.json");
const JAPANESE_MODEL_JSON: &str = include_str!("../../data/segmenter/model_japanese.json");
const THAI_MODEL_JSON: &str = include_str!("../../data/segmenter/model_thai.json");
const INVALID_FEATURE_KEY: DataError = DataError::custom("Invalid AdaBoost feature key");

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawChineseAdaboostData {
    #[serde(rename = "UW2")]
    uw2: HashMap<String, i16>,
    #[serde(rename = "UW3")]
    uw3: HashMap<String, i16>,
    #[serde(rename = "UW4")]
    uw4: HashMap<String, i16>,
    #[serde(rename = "UW5")]
    uw5: HashMap<String, i16>,
    #[serde(rename = "BW2")]
    bw2: HashMap<String, i16>,
    #[serde(rename = "RAD")]
    rad: HashMap<String, i16>,
    #[serde(rename = "LSRID")]
    lsrid: HashMap<String, i16>,
    #[serde(rename = "RSRID")]
    rsrid: HashMap<String, i16>,
}

impl RawChineseAdaboostData {
    fn try_convert(&self) -> Result<AdaboostData<'static>, DataError> {
        let weight_sum = sum_weights([
            &self.uw2,
            &self.uw3,
            &self.uw4,
            &self.uw5,
            &self.bw2,
            &self.rad,
            &self.lsrid,
            &self.rsrid,
        ]);
        if weight_sum % 2 != 0 {
            return Err(DataError::custom("AdaBoost model has a non-integral bias"));
        }

        Ok(AdaboostData {
            bias: -weight_sum / 2,
            uw2: convert_map(&self.uw2, parse_char)?,
            uw3: convert_map(&self.uw3, parse_char)?,
            uw4: convert_map(&self.uw4, parse_char)?,
            uw5: convert_map(&self.uw5, parse_char)?,
            bw2: convert_map(&self.bw2, parse_char_pair)?,
            rad: convert_map(&self.rad, parse_radical_pair)?,
            lsrid: convert_map(&self.lsrid, parse_left_radical)?,
            rsrid: convert_map(&self.rsrid, parse_right_radical)?,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCjAdaboostData {
    #[serde(rename = "UW1")]
    uw1: HashMap<String, i16>,
    #[serde(rename = "UW2")]
    uw2: HashMap<String, i16>,
    #[serde(rename = "UW3")]
    uw3: HashMap<String, i16>,
    #[serde(rename = "UW4")]
    uw4: HashMap<String, i16>,
    #[serde(rename = "UW5")]
    uw5: HashMap<String, i16>,
    #[serde(rename = "UW6")]
    uw6: HashMap<String, i16>,
    #[serde(rename = "BW1")]
    bw1: HashMap<String, i16>,
    #[serde(rename = "BW2")]
    bw2: HashMap<String, i16>,
    #[serde(rename = "BW3")]
    bw3: HashMap<String, i16>,
    #[serde(rename = "TW1")]
    tw1: HashMap<String, i16>,
    #[serde(rename = "TW2")]
    tw2: HashMap<String, i16>,
    #[serde(rename = "TW3")]
    tw3: HashMap<String, i16>,
    #[serde(rename = "TW4")]
    tw4: HashMap<String, i16>,
    #[serde(rename = "RAD")]
    rad: HashMap<String, i16>,
    #[serde(rename = "LSRID")]
    lsrid: HashMap<String, i16>,
    #[serde(rename = "RSRID")]
    rsrid: HashMap<String, i16>,
}

impl RawCjAdaboostData {
    fn try_convert(&self) -> Result<CjAdaboostData<'static>, DataError> {
        let weight_sum = sum_weights([
            &self.uw1,
            &self.uw2,
            &self.uw3,
            &self.uw4,
            &self.uw5,
            &self.uw6,
            &self.bw1,
            &self.bw2,
            &self.bw3,
            &self.tw1,
            &self.tw2,
            &self.tw3,
            &self.tw4,
            &self.rad,
            &self.lsrid,
            &self.rsrid,
        ]);
        if weight_sum % 2 != 0 {
            return Err(DataError::custom("AdaBoost model has a non-integral bias"));
        }

        Ok(CjAdaboostData {
            bias: -weight_sum / 2,
            uw1: convert_map(&self.uw1, parse_char)?,
            uw2: convert_map(&self.uw2, parse_char)?,
            uw3: convert_map(&self.uw3, parse_char)?,
            uw4: convert_map(&self.uw4, parse_char)?,
            uw5: convert_map(&self.uw5, parse_char)?,
            uw6: convert_map(&self.uw6, parse_char)?,
            bw1: convert_map(&self.bw1, parse_char_pair)?,
            bw2: convert_map(&self.bw2, parse_char_pair)?,
            bw3: convert_map(&self.bw3, parse_char_pair)?,
            tw1: convert_string_map(&self.tw1),
            tw2: convert_string_map(&self.tw2),
            tw3: convert_string_map(&self.tw3),
            tw4: convert_string_map(&self.tw4),
            rad: convert_map(&self.rad, parse_radical_pair)?,
            lsrid: convert_map(&self.lsrid, parse_left_radical)?,
            rsrid: convert_map(&self.rsrid, parse_right_radical)?,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawUnigramAdaboostData {
    #[serde(rename = "UW1")]
    uw1: HashMap<String, i16>,
    #[serde(rename = "UW2")]
    uw2: HashMap<String, i16>,
    #[serde(rename = "UW3")]
    uw3: HashMap<String, i16>,
    #[serde(rename = "UW4")]
    uw4: HashMap<String, i16>,
    #[serde(rename = "UW5")]
    uw5: HashMap<String, i16>,
    #[serde(rename = "UW6")]
    uw6: HashMap<String, i16>,
    #[serde(rename = "BW1")]
    bw1: HashMap<String, i16>,
    #[serde(rename = "BW2")]
    bw2: HashMap<String, i16>,
    #[serde(rename = "BW3")]
    bw3: HashMap<String, i16>,
    #[serde(rename = "TW1")]
    tw1: HashMap<String, i16>,
    #[serde(rename = "TW2")]
    tw2: HashMap<String, i16>,
    #[serde(rename = "TW3")]
    tw3: HashMap<String, i16>,
    #[serde(rename = "TW4")]
    tw4: HashMap<String, i16>,
}

impl RawUnigramAdaboostData {
    fn try_convert_thai(&self) -> Result<ThaiAdaboostData<'static>, DataError> {
        let weight_sum = sum_weights([
            &self.uw1, &self.uw2, &self.uw3, &self.uw4, &self.uw5, &self.uw6, &self.bw1, &self.bw2,
            &self.bw3, &self.tw1, &self.tw2, &self.tw3, &self.tw4,
        ]);

        Ok(ThaiAdaboostData {
            bias: -weight_sum,
            uw1: convert_map(&self.uw1, parse_char)?,
            uw2: convert_map(&self.uw2, parse_char)?,
            uw3: convert_map(&self.uw3, parse_char)?,
            uw4: convert_map(&self.uw4, parse_char)?,
            uw5: convert_map(&self.uw5, parse_char)?,
            uw6: convert_map(&self.uw6, parse_char)?,
            bw1: convert_map(&self.bw1, parse_char_pair)?,
            bw2: convert_map(&self.bw2, parse_char_pair)?,
            bw3: convert_map(&self.bw3, parse_char_pair)?,
            tw1: convert_string_map(&self.tw1),
            tw2: convert_string_map(&self.tw2),
            tw3: convert_string_map(&self.tw3),
            tw4: convert_string_map(&self.tw4),
        })
    }

    fn try_convert_japanese(&self) -> Result<JapaneseAdaboostData<'static>, DataError> {
        let weight_sum = sum_weights([
            &self.uw1, &self.uw2, &self.uw3, &self.uw4, &self.uw5, &self.uw6, &self.bw1, &self.bw2,
            &self.bw3, &self.tw1, &self.tw2, &self.tw3, &self.tw4,
        ]);

        Ok(JapaneseAdaboostData {
            bias: -weight_sum,
            uw1: convert_map(&self.uw1, parse_char)?,
            uw2: convert_map(&self.uw2, parse_char)?,
            uw3: convert_map(&self.uw3, parse_char)?,
            uw4: convert_map(&self.uw4, parse_char)?,
            uw5: convert_map(&self.uw5, parse_char)?,
            uw6: convert_map(&self.uw6, parse_char)?,
            bw1: convert_map(&self.bw1, parse_char_pair)?,
            bw2: convert_map(&self.bw2, parse_char_pair)?,
            bw3: convert_map(&self.bw3, parse_char_pair)?,
            tw1: convert_string_map(&self.tw1),
            tw2: convert_string_map(&self.tw2),
            tw3: convert_string_map(&self.tw3),
            tw4: convert_string_map(&self.tw4),
        })
    }
}

fn sum_weights<'a>(maps: impl IntoIterator<Item = &'a HashMap<String, i16>>) -> i32 {
    maps.into_iter()
        .flat_map(|weights| weights.values())
        .map(|&weight| i32::from(weight))
        .sum()
}

fn convert_map<K>(
    raw: &HashMap<String, i16>,
    parse_key: fn(&str) -> Result<K, DataError>,
) -> Result<ZeroMap<'static, K, i16>, DataError>
where
    K: ZeroMapKV<'static> + Ord,
{
    raw.iter()
        .map(|(key, &weight)| Ok((parse_key(key)?, weight)))
        .collect()
}

fn convert_string_map(raw: &HashMap<String, i16>) -> ZeroMap<'static, str, i16> {
    raw.iter()
        .map(|(key, &weight)| (key.as_str(), weight))
        .collect()
}

fn parse_char(key: &str) -> Result<char, DataError> {
    let mut chars = key.chars();
    let ch = chars.next().ok_or(INVALID_FEATURE_KEY)?;
    if chars.next().is_some() {
        return Err(INVALID_FEATURE_KEY);
    }
    Ok(ch)
}

fn parse_char_pair(key: &str) -> Result<(char, char), DataError> {
    let mut chars = key.chars();
    let left = chars.next().ok_or(INVALID_FEATURE_KEY)?;
    let right = chars.next().ok_or(INVALID_FEATURE_KEY)?;
    if chars.next().is_some() {
        return Err(INVALID_FEATURE_KEY);
    }
    Ok((left, right))
}

fn parse_radical(key: &str) -> Result<u8, DataError> {
    key.parse().map_err(|_| INVALID_FEATURE_KEY)
}

fn parse_radical_pair(key: &str) -> Result<(u8, u8), DataError> {
    let (left, right) = key.split_once(':').ok_or(INVALID_FEATURE_KEY)?;
    Ok((parse_radical(left)?, parse_radical(right)?))
}

fn parse_left_radical(key: &str) -> Result<(u8, char), DataError> {
    let (left, right) = key.split_once(':').ok_or(INVALID_FEATURE_KEY)?;
    Ok((parse_radical(left)?, parse_char(right)?))
}

fn parse_right_radical(key: &str) -> Result<(char, u8), DataError> {
    let (left, right) = key.rsplit_once(':').ok_or(INVALID_FEATURE_KEY)?;
    Ok((parse_char(left)?, parse_radical(right)?))
}

impl DataProvider<SegmenterChineseAutoV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterChineseAutoV1>, DataError> {
        if req.id.marker_attributes.as_str() != CHINESE_ADABOOST_ID {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(SegmenterChineseAutoV1::INFO, req)
            );
        }
        self.check_req::<SegmenterChineseAutoV1>(req)?;

        let raw =
            serde_json::from_str::<RawChineseAdaboostData>(CHINESE_MODEL_JSON).map_err(|e| {
                DataError::custom("Failed to parse built-in Chinese AdaBoost model")
                    .with_display_context(&e)
            })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(raw.try_convert()?),
        })
    }
}

impl IterableDataProviderCached<SegmenterChineseAutoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        singleton_id(CHINESE_ADABOOST_ID)
    }
}

impl DataProvider<SegmenterCjAutoV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterCjAutoV1>, DataError> {
        if req.id.marker_attributes.as_str() != CJ_ADABOOST_ID {
            return Err(DataErrorKind::IdentifierNotFound
                .with_req(SegmenterCjAutoV1::INFO, req));
        }
        self.check_req::<SegmenterCjAutoV1>(req)?;

        let raw = serde_json::from_str::<RawCjAdaboostData>(CJ_MODEL_JSON).map_err(|e| {
            DataError::custom("Failed to parse built-in Chinese/Japanese AdaBoost model")
                .with_display_context(&e)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(raw.try_convert()?),
        })
    }
}

impl IterableDataProviderCached<SegmenterCjAutoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        singleton_id(CJ_ADABOOST_ID)
    }
}

impl DataProvider<SegmenterJapaneseAutoV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterJapaneseAutoV1>, DataError> {
        if req.id.marker_attributes.as_str() != JAPANESE_ADABOOST_ID {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(SegmenterJapaneseAutoV1::INFO, req)
            );
        }
        self.check_req::<SegmenterJapaneseAutoV1>(req)?;

        let raw =
            serde_json::from_str::<RawUnigramAdaboostData>(JAPANESE_MODEL_JSON).map_err(|e| {
                DataError::custom("Failed to parse built-in Japanese AdaBoost model")
                    .with_display_context(&e)
            })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(raw.try_convert_japanese()?),
        })
    }
}

impl IterableDataProviderCached<SegmenterJapaneseAutoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        singleton_id(JAPANESE_ADABOOST_ID)
    }
}

impl DataProvider<SegmenterThaiAutoV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterThaiAutoV1>, DataError> {
        if req.id.marker_attributes.as_str() != THAI_ADABOOST_ID {
            return Err(DataErrorKind::IdentifierNotFound.with_req(SegmenterThaiAutoV1::INFO, req));
        }
        self.check_req::<SegmenterThaiAutoV1>(req)?;

        let raw = serde_json::from_str::<RawUnigramAdaboostData>(THAI_MODEL_JSON).map_err(|e| {
            DataError::custom("Failed to parse built-in Thai AdaBoost model")
                .with_display_context(&e)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(raw.try_convert_thai()?),
        })
    }
}

impl IterableDataProviderCached<SegmenterThaiAutoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        singleton_id(THAI_ADABOOST_ID)
    }
}

fn singleton_id(id: &str) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
    let attributes = DataMarkerAttributes::try_from_string(id.to_owned())
        .map_err(|_| DataError::custom("Invalid built-in AdaBoost model identifier"))?;
    Ok(HashSet::from([
        DataIdentifierCow::from_marker_attributes_owned(attributes),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_built_in_chinese_model() {
        let raw = serde_json::from_str::<RawChineseAdaboostData>(CHINESE_MODEL_JSON)
            .expect("the built-in Chinese AdaBoost model should parse");

        let counts = [
            raw.uw2.len(),
            raw.uw3.len(),
            raw.uw4.len(),
            raw.uw5.len(),
            raw.bw2.len(),
            raw.rad.len(),
            raw.lsrid.len(),
            raw.rsrid.len(),
        ];
        assert_eq!(counts, [730, 802, 850, 746, 644, 734, 384, 401]);

        let converted = raw
            .try_convert()
            .expect("the built-in Chinese AdaBoost model should convert");
        assert_eq!(converted.bias, 4);
        assert_eq!(counts.into_iter().sum::<usize>(), 5291);
    }

    #[test]
    fn converts_built_in_thai_model() {
        let raw = serde_json::from_str::<RawUnigramAdaboostData>(THAI_MODEL_JSON)
            .expect("the built-in Thai AdaBoost model should parse");

        let counts = [
            raw.uw1.len(),
            raw.uw2.len(),
            raw.uw3.len(),
            raw.uw4.len(),
            raw.uw5.len(),
            raw.uw6.len(),
            raw.bw1.len(),
            raw.bw2.len(),
            raw.bw3.len(),
            raw.tw1.len(),
            raw.tw2.len(),
            raw.tw3.len(),
            raw.tw4.len(),
        ];
        assert_eq!(
            counts,
            [57, 62, 54, 71, 62, 54, 391, 443, 415, 673, 495, 536, 588]
        );

        let converted = raw
            .try_convert_thai()
            .expect("the built-in Thai AdaBoost model should convert");
        assert_eq!(converted.bias, -3755);
        assert_eq!(counts.into_iter().sum::<usize>(), 3901);
        assert_eq!(converted.tw4.get_copied("ละ"), Some(870));
    }

    #[test]
    fn converts_built_in_cj_model() {
        let raw = serde_json::from_str::<RawCjAdaboostData>(CJ_MODEL_JSON)
            .expect("the built-in Chinese/Japanese AdaBoost model should parse");

        let counts = [
            raw.uw1.len(),
            raw.uw2.len(),
            raw.uw3.len(),
            raw.uw4.len(),
            raw.uw5.len(),
            raw.uw6.len(),
            raw.bw1.len(),
            raw.bw2.len(),
            raw.bw3.len(),
            raw.tw1.len(),
            raw.tw2.len(),
            raw.tw3.len(),
            raw.tw4.len(),
            raw.rad.len(),
            raw.lsrid.len(),
            raw.rsrid.len(),
        ];
        assert_eq!(
            counts,
            [
                383, 585, 787, 836, 612, 386, 481, 544, 436, 35, 40, 36, 37, 441, 265, 251
            ]
        );

        let converted = raw
            .try_convert()
            .expect("the built-in Chinese/Japanese AdaBoost model should convert");
        assert_eq!(converted.bias, -15);
        assert_eq!(counts.into_iter().sum::<usize>(), 6155);
        assert_eq!(converted.tw4.get_copied("かなり"), Some(2015));
    }

    #[test]
    fn converts_built_in_japanese_model() {
        let raw = serde_json::from_str::<RawUnigramAdaboostData>(JAPANESE_MODEL_JSON)
            .expect("the built-in Japanese AdaBoost model should parse");

        let counts = [
            raw.uw1.len(),
            raw.uw2.len(),
            raw.uw3.len(),
            raw.uw4.len(),
            raw.uw5.len(),
            raw.uw6.len(),
            raw.bw1.len(),
            raw.bw2.len(),
            raw.bw3.len(),
            raw.tw1.len(),
            raw.tw2.len(),
            raw.tw3.len(),
            raw.tw4.len(),
        ];
        assert_eq!(
            counts,
            [120, 167, 192, 244, 117, 108, 208, 153, 156, 78, 37, 43, 87]
        );

        let converted = raw
            .try_convert_japanese()
            .expect("the built-in Japanese AdaBoost model should convert");
        assert_eq!(converted.bias, -3331);
        assert_eq!(counts.into_iter().sum::<usize>(), 1710);
        assert_eq!(converted.tw3.get_copied("という"), Some(377));
    }

    #[test]
    fn loads_built_in_chinese_model() {
        let provider = SourceDataProvider::new_testing();
        let ids = <SourceDataProvider as IterableDataProviderCached<
            SegmenterChineseAutoV1,
        >>::iter_ids_cached(&provider)
        .expect("the built-in Chinese model identifier should be available");
        assert_eq!(ids.len(), 1);

        let id = ids.into_iter().next().unwrap();
        assert_eq!(id.marker_attributes.as_str(), CHINESE_ADABOOST_ID);
        let response: DataResponse<SegmenterChineseAutoV1> = provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .expect("the built-in Chinese AdaBoost model should load");
        assert_eq!(response.payload.get().bias, 4);
    }

    #[test]
    fn loads_built_in_thai_model() {
        let provider = SourceDataProvider::new_testing();
        let ids = <SourceDataProvider as IterableDataProviderCached<
            SegmenterThaiAutoV1,
        >>::iter_ids_cached(&provider)
        .expect("the built-in Thai model identifier should be available");
        assert_eq!(ids.len(), 1);

        let id = ids.into_iter().next().unwrap();
        assert_eq!(id.marker_attributes.as_str(), THAI_ADABOOST_ID);
        let response: DataResponse<SegmenterThaiAutoV1> = provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .expect("the built-in Thai AdaBoost model should load");
        assert_eq!(response.payload.get().bias, -3755);
    }

    #[test]
    fn loads_built_in_cj_model() {
        let provider = SourceDataProvider::new_testing();
        let ids = <SourceDataProvider as IterableDataProviderCached<
            SegmenterCjAutoV1,
        >>::iter_ids_cached(&provider)
        .expect("the built-in Chinese/Japanese model identifier should be available");
        assert_eq!(ids.len(), 1);

        let id = ids.into_iter().next().unwrap();
        assert_eq!(
            id.marker_attributes.as_str(),
            CJ_ADABOOST_ID
        );
        let response: DataResponse<SegmenterCjAutoV1> = provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .expect("the built-in Chinese/Japanese AdaBoost model should load");
        assert_eq!(response.payload.get().bias, -15);
    }

    #[test]
    fn loads_built_in_japanese_model() {
        let provider = SourceDataProvider::new_testing();
        let ids = <SourceDataProvider as IterableDataProviderCached<
            SegmenterJapaneseAutoV1,
        >>::iter_ids_cached(&provider)
        .expect("the built-in Japanese model identifier should be available");
        assert_eq!(ids.len(), 1);

        let id = ids.into_iter().next().unwrap();
        assert_eq!(id.marker_attributes.as_str(), JAPANESE_ADABOOST_ID);
        let response: DataResponse<SegmenterJapaneseAutoV1> = provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .expect("the built-in Japanese AdaBoost model should load");
        assert_eq!(response.payload.get().bias, -3331);
    }

    #[test]
    fn rejects_unknown_model_attributes() {
        let provider = SourceDataProvider::new_testing();
        let unknown = DataMarkerAttributes::from_str_or_panic("unknown");

        let chinese = <SourceDataProvider as DataProvider<SegmenterChineseAutoV1>>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(unknown),
                ..Default::default()
            },
        )
        .expect_err("unknown Chinese model attributes should fail");
        assert_eq!(chinese.kind, DataErrorKind::IdentifierNotFound);

        let thai = <SourceDataProvider as DataProvider<SegmenterThaiAutoV1>>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(unknown),
                ..Default::default()
            },
        )
        .expect_err("unknown Thai model attributes should fail");
        assert_eq!(thai.kind, DataErrorKind::IdentifierNotFound);

        let chinese_or_japanese =
            <SourceDataProvider as DataProvider<SegmenterCjAutoV1>>::load(
                &provider,
                DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes(unknown),
                    ..Default::default()
                },
            )
            .expect_err("unknown Chinese/Japanese model attributes should fail");
        assert_eq!(chinese_or_japanese.kind, DataErrorKind::IdentifierNotFound);

        let japanese = <SourceDataProvider as DataProvider<SegmenterJapaneseAutoV1>>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(unknown),
                ..Default::default()
            },
        )
        .expect_err("unknown Japanese model attributes should fail");
        assert_eq!(japanese.kind, DataErrorKind::IdentifierNotFound);
    }
}
