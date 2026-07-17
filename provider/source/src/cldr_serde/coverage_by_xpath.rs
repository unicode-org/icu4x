// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use litemap::LiteMap;
use serde::Deserialize;
use serde::de::{Deserializer, Error as DeError, SeqAccess, Visitor};
use std::collections::BTreeMap;
use std::fmt;
use zerotrie::ZeroTrieSimpleAscii;

/// Serde wrapper for `coverageByXPath.json` and locale-specific `coverageByXPath/{locale}.json` files.
#[derive(Deserialize, Debug)]
pub(crate) struct CoverageByXPathResource {
    /// Mapping from locale identifier (or `"root"`) to its corresponding coverage levels object.
    #[serde(rename = "coverageByXPath")]
    pub(crate) coverage_by_xpath: BTreeMap<String, CoverageByXPathLevels>,
}

/// Representation of coverage levels (`basic`, `core`, `moderate`, `modern`) containing sets of `XPaths`.
#[derive(Deserialize, Debug)]
pub(crate) struct CoverageByXPathLevels {
    /// `XPaths` classified under the `basic` coverage level.
    #[serde(default, deserialize_with = "set_to_trie")]
    pub(crate) basic: ZeroTrieSimpleAscii<Vec<u8>>,
    /// `XPaths` classified under the `core` coverage level.
    #[serde(default, deserialize_with = "set_to_trie")]
    pub(crate) core: ZeroTrieSimpleAscii<Vec<u8>>,
    /// `XPaths` classified under the `moderate` coverage level.
    #[serde(default, deserialize_with = "set_to_trie")]
    pub(crate) moderate: ZeroTrieSimpleAscii<Vec<u8>>,
    /// `XPaths` classified under the `modern` coverage level.
    #[serde(default, deserialize_with = "set_to_trie")]
    pub(crate) modern: ZeroTrieSimpleAscii<Vec<u8>>,
}

struct ZeroTrieVisitor;

impl<'de> Visitor<'de> for ZeroTrieVisitor {
    type Value = ZeroTrieSimpleAscii<Vec<u8>>;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a sequence of XPaths")
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut litemap = LiteMap::new_vec();
        while let Some(elem) = seq.next_element::<String>()? {
            litemap.insert(elem.into_bytes(), 0);
        }
        ZeroTrieSimpleAscii::try_from(&litemap).map_err(DeError::custom)
    }
}

fn set_to_trie<'de, D>(deserializer: D) -> Result<ZeroTrieSimpleAscii<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(ZeroTrieVisitor)
}
