// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON BCP47 alias files.
//!
//! These files contain alias data for Unicode extension key-value pairs,
//! used for locale canonicalization per UTS #35 Section 3.3.1.
//!
//! Sample files:
//! - <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/calendar.json>
//! - <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-bcp47/bcp47/collation.json>

use serde::Deserialize;
use std::collections::HashMap;

/// A single value entry within a BCP47 key definition.
///
/// Contains alias/deprecation information for a specific key value.
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47ValueData {
    #[serde(rename = "_description", default)]
    pub(crate) description: Option<String>,
    #[serde(rename = "_alias", default)]
    pub(crate) alias: Option<String>,
    #[serde(rename = "_deprecated", default)]
    pub(crate) deprecated: Option<bool>,
    #[serde(rename = "_preferred", default)]
    pub(crate) preferred: Option<String>,
    #[serde(rename = "_since", default)]
    pub(crate) since: Option<String>,
}

/// A BCP47 key definition containing its values and metadata.
///
/// For example, the `ca` key in `calendar.json` contains calendar algorithm
/// values like `islamicc` (deprecated, preferred: `islamic-civil`).
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47KeyData {
    #[serde(rename = "_description", default)]
    pub(crate) description: Option<String>,
    #[serde(rename = "_alias", default)]
    pub(crate) alias: Option<String>,
    #[serde(rename = "_deprecated", default)]
    pub(crate) deprecated: Option<bool>,
    #[serde(rename = "_preferred", default)]
    pub(crate) preferred: Option<String>,
    #[serde(rename = "_valueType", default)]
    pub(crate) value_type: Option<String>,
    #[serde(rename = "_since", default)]
    pub(crate) since: Option<String>,
    #[serde(flatten)]
    pub(crate) values: HashMap<String, Bcp47ValueData>,
}

/// The `keyword.u` section of a BCP47 JSON file.
///
/// Contains all Unicode extension keys defined in this file.
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47UnicodeKeyword {
    #[serde(flatten)]
    pub(crate) keys: HashMap<String, Bcp47KeyData>,
}

/// The `keyword.t` section of a BCP47 JSON file.
///
/// Contains all Transform extension keys defined in this file.
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47TransformKeyword {
    #[serde(flatten)]
    pub(crate) keys: HashMap<String, Bcp47KeyData>,
}

/// Top-level `keyword` section containing extension-specific data.
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47Keyword {
    #[serde(rename = "u", default)]
    pub(crate) unicode: Option<Bcp47UnicodeKeyword>,
    #[serde(rename = "t", default)]
    pub(crate) transform: Option<Bcp47TransformKeyword>,
}

/// Top-level BCP47 JSON resource structure.
#[derive(PartialEq, Debug, Clone, Deserialize)]
pub(crate) struct Bcp47Resource {
    pub(crate) keyword: Bcp47Keyword,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calendar_json() {
        let data: Bcp47Resource = serde_json::from_str(include_str!(
            "../../tests/data/cldr/cldr-bcp47/bcp47/calendar.json"
        ))
        .unwrap();
        let u = data.keyword.unicode.unwrap();
        let ca = u.keys.get("ca").unwrap();
        // islamicc should be deprecated with preferred = islamic-civil
        let islamicc = ca.values.get("islamicc").unwrap();
        assert_eq!(islamicc.deprecated, Some(true));
        assert_eq!(islamicc.preferred.as_deref(), Some("islamic-civil"));
        // ethioaa should have alias = ethiopic-amete-alem
        let ethioaa = ca.values.get("ethioaa").unwrap();
        assert_eq!(ethioaa.alias.as_deref(), Some("ethiopic-amete-alem"));
    }

    #[test]
    fn test_parse_collation_json() {
        let data: Bcp47Resource = serde_json::from_str(include_str!(
            "../../tests/data/cldr/cldr-bcp47/bcp47/collation.json"
        ))
        .unwrap();
        let u = data.keyword.unicode.unwrap();
        // ks should have level1 with alias = primary
        let ks = u.keys.get("ks").unwrap();
        let level1 = ks.values.get("level1").unwrap();
        assert_eq!(level1.alias.as_deref(), Some("primary"));
        // kb should have true with alias = yes
        let kb = u.keys.get("kb").unwrap();
        let true_val = kb.values.get("true").unwrap();
        assert_eq!(true_val.alias.as_deref(), Some("yes"));
    }

    #[test]
    fn test_parse_measure_json() {
        let data: Bcp47Resource = serde_json::from_str(include_str!(
            "../../tests/data/cldr/cldr-bcp47/bcp47/measure.json"
        ))
        .unwrap();
        let u = data.keyword.unicode.unwrap();
        let ms = u.keys.get("ms").unwrap();
        let uksystem = ms.values.get("uksystem").unwrap();
        assert_eq!(uksystem.alias.as_deref(), Some("imperial"));
    }

    #[test]
    fn test_parse_transform_json() {
        let data: Bcp47Resource = serde_json::from_str(include_str!(
            "../../tests/data/cldr/cldr-bcp47/bcp47/transform.json"
        ))
        .unwrap();
        let t = data.keyword.transform.unwrap();
        let m0 = t.keys.get("m0").unwrap();
        let prprname = m0.values.get("prprname").unwrap();
        assert_eq!(prprname.alias.as_deref(), Some("names"));
    }
}
