// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Provider implementation backed by the Chinese `AdaBoost` segmentation model.

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::segmenter::provider::adaboost::{AdaboostData, SegmenterChineseAutoV1};
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use zerovec::{ZeroMap, maps::ZeroMapKV};

const CHINESE_ADABOOST_ID: &str = "Chinese_adaboost";
const MODEL_JSON: &str = include_str!("../../data/segmenter/model.json");
const INVALID_FEATURE_KEY: DataError = DataError::custom("Invalid AdaBoost feature key");

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAdaboostData {
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

impl RawAdaboostData {
    fn try_convert(&self) -> Result<AdaboostData<'static>, DataError> {
        let weight_sum = [
            &self.uw2,
            &self.uw3,
            &self.uw4,
            &self.uw5,
            &self.bw2,
            &self.rad,
            &self.lsrid,
            &self.rsrid,
        ]
        .into_iter()
        .flat_map(|weights| weights.values())
        .map(|&weight| i32::from(weight))
        .sum::<i32>();
        if weight_sum % 2 != 0 {
            return Err(DataError::custom("AdaBoost model has a non-integral bias"));
        }
        let bias = -weight_sum / 2;

        Ok(AdaboostData {
            bias,
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

        let raw = serde_json::from_str::<RawAdaboostData>(MODEL_JSON).map_err(|e| {
            DataError::custom("Failed to parse built-in AdaBoost model").with_display_context(&e)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(raw.try_convert()?),
        })
    }
}

impl IterableDataProviderCached<SegmenterChineseAutoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let attributes = DataMarkerAttributes::try_from_string(CHINESE_ADABOOST_ID.to_owned())
            .map_err(|_| DataError::custom("Invalid built-in AdaBoost model identifier"))?;
        Ok(HashSet::from([
            DataIdentifierCow::from_marker_attributes_owned(attributes),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MODEL_JSON: &str = include_str!("../../data/segmenter/model.json");

    #[test]
    fn converts_built_in_chinese_model() {
        let raw = serde_json::from_str::<RawAdaboostData>(MODEL_JSON)
            .expect("the built-in AdaBoost model should parse");

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
        assert_eq!(counts.into_iter().sum::<usize>(), 5291);

        let raw_sum = [
            &raw.uw2, &raw.uw3, &raw.uw4, &raw.uw5, &raw.bw2, &raw.rad, &raw.lsrid, &raw.rsrid,
        ]
        .into_iter()
        .flat_map(|weights| weights.values())
        .map(|&weight| i64::from(weight))
        .sum::<i64>();
        assert_eq!(raw_sum, -8);

        let converted = raw
            .try_convert()
            .expect("the built-in AdaBoost model should convert");
        assert_eq!(converted.bias, 4);
        assert_eq!(converted.uw2.len(), counts[0]);
        assert_eq!(converted.uw3.len(), counts[1]);
        assert_eq!(converted.uw4.len(), counts[2]);
        assert_eq!(converted.uw5.len(), counts[3]);
        assert_eq!(converted.bw2.len(), counts[4]);
        assert_eq!(converted.rad.len(), counts[5]);
        assert_eq!(converted.lsrid.len(), counts[6]);
        assert_eq!(converted.rsrid.len(), counts[7]);
    }

    #[test]
    fn loads_built_in_chinese_model() {
        let provider = SourceDataProvider::new_testing();
        let ids = <SourceDataProvider as IterableDataProviderCached<
            SegmenterChineseAutoV1,
        >>::iter_ids_cached(&provider)
        .expect("the built-in model identifier should be available");
        assert_eq!(ids.len(), 1);

        let id = ids.into_iter().next().unwrap();
        assert_eq!(id.marker_attributes.as_str(), CHINESE_ADABOOST_ID);
        let response: DataResponse<SegmenterChineseAutoV1> = provider
            .load(DataRequest {
                id: id.as_borrowed(),
                ..Default::default()
            })
            .expect("the built-in AdaBoost model should load");
        let data = response.payload.get();
        assert_eq!(data.bias, 4);
        assert_eq!(
            data.uw2.len()
                + data.uw3.len()
                + data.uw4.len()
                + data.uw5.len()
                + data.bw2.len()
                + data.rad.len()
                + data.lsrid.len()
                + data.rsrid.len(),
            5291
        );
    }
}
