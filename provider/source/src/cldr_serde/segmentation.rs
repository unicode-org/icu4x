// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON tailorings.json files.
//!
//! Sample file:
//! icu4x/provider/source/data/segmenter/neo/cldr-json/cldr-segments-full/segments/el/tailorings.json

use icu::locale::{
    Locale,
    extensions::unicode::{key, value},
};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize)]
pub struct Resource {
    pub segments: Segments,
}

#[derive(Debug, Deserialize)]
pub struct Segments {
    pub segmentations: Segmentation,
}

#[derive(Debug)]
pub struct Segmentation(pub BTreeMap<String, HashMap<Locale, Vec<String>>>);

impl<'de> Deserialize<'de> for Segmentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = BTreeMap::<String, HashMap<Locale, Vec<String>>>::deserialize(deserializer)?;

        for map in map.values_mut() {
            // Remove comments
            for lines in map.values_mut() {
                for line in &mut *lines {
                    *line = line
                        .split_once('#')
                        .map(|(l, _)| l)
                        .unwrap_or(line)
                        .trim()
                        .into();
                }
                lines.retain(|s| !s.is_empty());
            }

            // xx-u-lb-normal includes rules from xx
            if let Some(locale_strict) = map.keys().find(|l| l.extensions.is_empty())
                && let Some(lines_strict) = map.get(locale_strict).cloned()
                && let Some(lines_normal) = map.get_mut(&{
                    let mut locale_normal = locale_strict.clone();
                    locale_normal
                        .extensions
                        .unicode
                        .keywords
                        .set(key!("lb"), value!("normal"));
                    locale_normal
                })
            {
                *lines_normal = lines_strict
                    .into_iter()
                    .chain(core::mem::take(lines_normal))
                    .collect();
            }

            // xx-u-lb-loose includes rules from xx-u-lb-normal
            if let Some(locale_normal) = map
                .keys()
                .find(|l| l.extensions.unicode.keywords.get(&key!("lb")) == Some(&value!("normal")))
                && let Some(lines_normal) = map.get(locale_normal).cloned()
                && let Some(lines_loose) = map.get_mut(&{
                    let mut locale_loose = locale_normal.clone();
                    locale_loose
                        .extensions
                        .unicode
                        .keywords
                        .set(key!("lb"), value!("loose"));
                    locale_loose
                })
            {
                *lines_loose = lines_normal
                    .into_iter()
                    .chain(core::mem::take(lines_loose))
                    .collect();
            }
        }

        Ok(Segmentation(map))
    }
}
