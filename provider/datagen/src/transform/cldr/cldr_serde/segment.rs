// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON suppressions.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-segments-full/segments/und/suppressions.json>

use serde::{de::Deserializer, Deserialize};
use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Debug, Deserialize)]
struct RawSuppression {
    suppression: String,
}

#[derive(PartialEq, Debug, Deserialize)]
struct RawSegmentation {
    #[serde(default)]
    variables: Vec<HashMap<String, String>>,
    #[serde(default)]
    #[serde(rename = "segmentRules")]
    pub segment_rules: HashMap<String, String>,
    #[serde(default)]
    standard: Vec<RawSuppression>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Segmentations {
    #[serde(rename = "GraphemeClusterBreak")]
    #[serde(default)]
    pub grapheme_cluster_break: Segmentation,
    #[serde(rename = "LineBreak")]
    #[serde(default)]
    pub line_break: Segmentation,
    #[serde(rename = "SentenceBreak")]
    #[serde(default)]
    pub sentence_break: Segmentation,
    #[serde(rename = "WordBreak")]
    #[serde(default)]
    pub word_break: Segmentation,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Segments {
    pub segmentations: Segmentations,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub segments: Segments,
}

#[derive(PartialEq, Debug, Default)]
pub struct Segmentation {
    pub variables: BTreeMap<String, Vec<String>>,
    pub sorted_rules: Vec<String>,
    pub suppressions: Vec<String>,
}

impl<'de> Deserialize<'de> for Segmentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawSegmentation::deserialize(deserializer)?;

        let mut variables = BTreeMap::<String, Vec<String>>::new();

        for kvs in raw.variables {
            for (key, value) in kvs {
                // https://unicode-org.atlassian.net/browse/CLDR-16662
                let value = value.replace("\\\\p", "\\p");
                variables.entry(key).or_default().push(value);
            }
        }

        let mut rules = raw.segment_rules.into_iter().collect::<Vec<(_, _)>>();

        rules.sort_by(|(a, _), (b, _)| {
            a.split('.')
                .map(|x| x.parse::<usize>().unwrap())
                .cmp(b.split('.').map(|x| x.parse::<usize>().unwrap()))
        });

        Ok(Self {
            variables,
            sorted_rules: rules.into_iter().map(|(_, value)| value).collect(),
            suppressions: raw.standard.into_iter().map(|s| s.suppression).collect(),
        })
    }
}
