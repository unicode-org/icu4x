// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Provider implementation backed by the Chinese AdaBoost segmentation model.

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::segmenter::provider::{AdaboostData, SegmenterAdaboostAutoV1};
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use zerovec::{maps::ZeroMapKV, ZeroMap};

const CHINESE_ADABOOST_ID: &str = "Chinese_adaboost";
const CHINESE_ADABOOST_PATH: &str = "adaboost_cjk_segmenter/model.json";
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
        let bias_x2 = -[
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

        Ok(AdaboostData {
            bias_x2,
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

impl DataProvider<SegmenterAdaboostAutoV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterAdaboostAutoV1>, DataError> {
        if req.id.marker_attributes.as_str() != CHINESE_ADABOOST_ID {
            return Err(DataErrorKind::IdentifierNotFound
                .with_req(SegmenterAdaboostAutoV1::INFO, req));
        }
        self.check_req::<SegmenterAdaboostAutoV1>(req)?;

        let raw = self
            .segmenter_lstm()?
            .read_and_parse_json::<RawAdaboostData>(CHINESE_ADABOOST_PATH)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(raw.try_convert()?),
        })
    }
}

impl IterableDataProviderCached<SegmenterAdaboostAutoV1> for SourceDataProvider {
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
    use icu::segmenter::provider::radical::{SegmenterUnihanRadicalV1, UnihanRadicalsData};

    const MODEL_JSON: &str =
        include_str!("../../tests/data/lstm/adaboost_cjk_segmenter/model.json");

    #[test]
    fn converts_upstream_chinese_model() {
        let raw = serde_json::from_str::<RawAdaboostData>(MODEL_JSON)
            .expect("the upstream AdaBoost model should parse");

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
        assert_eq!(counts, [410, 524, 514, 432, 341, 289, 141, 127]);
        assert_eq!(counts.into_iter().sum::<usize>(), 2778);

        let raw_sum = [
            &raw.uw2,
            &raw.uw3,
            &raw.uw4,
            &raw.uw5,
            &raw.bw2,
            &raw.rad,
            &raw.lsrid,
            &raw.rsrid,
        ]
        .into_iter()
        .flat_map(|weights| weights.values())
        .map(|&weight| i64::from(weight))
        .sum::<i64>();
        assert_eq!(raw_sum, 36);

        let converted = raw
            .try_convert()
            .expect("the upstream AdaBoost model should convert");
        assert_eq!(converted.bias_x2, -36);
        assert_eq!(converted.uw2.len(), counts[0]);
        assert_eq!(converted.uw3.len(), counts[1]);
        assert_eq!(converted.uw4.len(), counts[2]);
        assert_eq!(converted.uw5.len(), counts[3]);
        assert_eq!(converted.bw2.len(), counts[4]);
        assert_eq!(converted.rad.len(), counts[5]);
        assert_eq!(converted.lsrid.len(), counts[6]);
        assert_eq!(converted.rsrid.len(), counts[7]);
    }

    fn parser_scores(
        model: &AdaboostData<'_>,
        radicals: &UnihanRadicalsData<'_>,
        text: &str,
    ) -> Vec<i64> {
        let chars = text.chars().collect::<Vec<_>>();
        let mut chunk_len = 1;
        (1..chars.len())
            .map(|i| {
                let previous = chars[i - 1];
                let current = chars[i];
                let previous_radical = radicals.trie.get(previous);
                let current_radical = radicals.trie.get(current);
                let mut score = i64::from(model.bias_x2) + 2 * 32_i64.pow(chunk_len);
                let mut add = |weight: Option<i16>| {
                    score += 2 * i64::from(weight.unwrap_or(0));
                };

                if current_radical != 0 {
                    add(model.rsrid.get_copied(&(previous, current_radical)));
                }
                if previous_radical != 0 {
                    add(model.lsrid.get_copied(&(previous_radical, current)));
                }
                if previous_radical != 0 && current_radical != 0 {
                    add(model
                        .rad
                        .get_copied(&(previous_radical, current_radical)));
                }
                add(model.bw2.get_copied(&(previous, current)));
                if i > 1 {
                    add(model.uw2.get_copied(&chars[i - 2]));
                }
                add(model.uw3.get_copied(&previous));
                add(model.uw4.get_copied(&current));
                if i + 1 < chars.len() {
                    add(model.uw5.get_copied(&chars[i + 1]));
                }

                chunk_len = if score > 0 { 1 } else { chunk_len + 1 };
                score
            })
            .collect()
    }

    #[test]
    fn exact_scores_match_upstream_parser() {
        let model = serde_json::from_str::<RawAdaboostData>(MODEL_JSON)
            .expect("the upstream AdaBoost model should parse")
            .try_convert()
            .expect("the upstream AdaBoost model should convert");
        let provider = SourceDataProvider::new_testing();
        let radicals: DataResponse<SegmenterUnihanRadicalV1> = provider
            .load(Default::default())
            .expect("the Unihan radical trie should load");
        let radicals = radicals.payload.get();

        assert_eq!(
            parser_scores(&model, radicals, "在香港實施愛國者治港"),
            [5026, -2820, 3424, -828, 2126, 62, 702, 2804, -1710]
        );
        assert_eq!(
            parser_scores(&model, radicals, "根据最新的财报数据显示"),
            [-1340, 6180, -196, 9112, 7230, -2156, 1824, -630, 4712, -1824]
        );
        assert_eq!(
            parser_scores(&model, radicals, "𠀀中國"),
            [2038, -3212]
        );
    }
}
