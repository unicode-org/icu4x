// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Provider implementations backed by built-in `AdaBoost` segmentation models.

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::segmenter::provider::adaboost::{AdaboostData, SegmenterAdaboostV1};
use icu_provider::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
use zerovec::{ZeroMap, maps::ZeroMapKV};

const CHINESE_ADABOOST_ID: &str = "Chinese_adaboost";
const JAPANESE_ADABOOST_ID: &str = "Japanese_adaboost";
const THAI_ADABOOST_ID: &str = "Thai_adaboost";
const CHINESE_MODEL_JSON: &str = include_str!("../../data/segmenter/adaboost/model_chinese.json");
const JAPANESE_MODEL_JSON: &str = include_str!("../../data/segmenter/adaboost/model_japanese.json");
const THAI_MODEL_JSON: &str = include_str!("../../data/segmenter/adaboost/model_thai.json");
const INVALID_FEATURE_KEY: DataError = DataError::custom("Invalid AdaBoost feature key");

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRadicalAdaboostData {
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

impl RawRadicalAdaboostData {
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

        Ok(AdaboostData::Radical {
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
struct RawNgramAdaboostData {
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

impl RawNgramAdaboostData {
    fn try_convert(&self) -> Result<AdaboostData<'static>, DataError> {
        let weight_sum = sum_weights([
            &self.uw1, &self.uw2, &self.uw3, &self.uw4, &self.uw5, &self.uw6, &self.bw1, &self.bw2,
            &self.bw3, &self.tw1, &self.tw2, &self.tw3, &self.tw4,
        ]);

        Ok(AdaboostData::Ngram {
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

fn parse_model<T: DeserializeOwned>(json: &str) -> Result<T, DataError> {
    serde_json::from_str(json).map_err(|e| {
        DataError::custom("Failed to parse built-in AdaBoost model").with_display_context(&e)
    })
}

impl DataProvider<SegmenterAdaboostV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterAdaboostV1>, DataError> {
        self.check_req::<SegmenterAdaboostV1>(req)?;

        let data = match req.id.marker_attributes.as_str() {
            CHINESE_ADABOOST_ID => {
                parse_model::<RawRadicalAdaboostData>(CHINESE_MODEL_JSON)?.try_convert()?
            }
            JAPANESE_ADABOOST_ID => {
                parse_model::<RawNgramAdaboostData>(JAPANESE_MODEL_JSON)?.try_convert()?
            }
            THAI_ADABOOST_ID => {
                parse_model::<RawNgramAdaboostData>(THAI_MODEL_JSON)?.try_convert()?
            }
            _ => {
                return Err(
                    DataErrorKind::IdentifierNotFound.with_req(SegmenterAdaboostV1::INFO, req)
                );
            }
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<SegmenterAdaboostV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(
            [CHINESE_ADABOOST_ID, JAPANESE_ADABOOST_ID, THAI_ADABOOST_ID]
                .into_iter()
                .map(|id| {
                    DataIdentifierCow::from_marker_attributes(
                        DataMarkerAttributes::from_str_or_panic(id),
                    )
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_every_built_in_model() {
        let provider = SourceDataProvider::new_custom();
        let ids = <SourceDataProvider as IterableDataProviderCached<SegmenterAdaboostV1>>::iter_ids_cached(&provider)
            .expect("the built-in model identifiers should be available");
        assert_eq!(ids.len(), 3);

        for id in ids {
            let response: DataResponse<SegmenterAdaboostV1> = provider
                .load(DataRequest {
                    id: id.as_borrowed(),
                    ..Default::default()
                })
                .expect("the built-in AdaBoost model should load");
            let is_radical = matches!(response.payload.get(), AdaboostData::Radical { .. });
            assert_eq!(
                is_radical,
                id.marker_attributes.as_str() == CHINESE_ADABOOST_ID
            );
        }
    }

    #[test]
    fn rejects_unknown_model_attributes() {
        let provider = SourceDataProvider::new_custom();
        let unknown = DataMarkerAttributes::from_str_or_panic("unknown");

        let error = <SourceDataProvider as DataProvider<SegmenterAdaboostV1>>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(unknown),
                ..Default::default()
            },
        )
        .expect_err("unknown model attributes should fail");
        assert_eq!(error.kind, DataErrorKind::IdentifierNotFound);
    }
}
