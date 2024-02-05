// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{char::DecodeUtf16Error, collections::HashMap, fmt::Debug, marker::PhantomData};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

use zerovec::ZeroVec;

/// A zero-copy representation of a little-endian UTF-16 string.
///
/// Unlike `String`, the contents are not required to be valid UTF-16. Consumers
/// are expected to validate the contents or use `try_into::<String>()`. No zero
/// terminator is included.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct ZeroUTF16String<'a> {
    #[serde(borrow)]
    units: ZeroVec<'a, u16>,
}

impl ZeroUTF16String<'_> {
    /// Gets whether the UTF-16 string is empty.
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// Gets the count of units in the string.
    ///
    /// This value does not necessarily equal the length of the string in
    /// characters, as characters outside the Basic Multilingual Plane are
    /// represented by 2 units.
    pub fn len(&self) -> usize {
        self.units.len()
    }

    /// Gets an iterator for the units of the string.
    ///
    /// See `len` for details on why this does not correspond to characters.
    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        self.units.iter()
    }
}

impl TryFrom<ZeroUTF16String<'_>> for String {
    type Error = DecodeUtf16Error;

    fn try_from(value: ZeroUTF16String<'_>) -> Result<Self, Self::Error> {
        char::decode_utf16(value.iter()).collect::<Result<String, _>>()
    }
}

impl Debug for ZeroUTF16String<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decoded = char::decode_utf16(self.iter())
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<String>();
        write!(f, "{}", decoded)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TzDataRuleData<'a> {
    #[serde(borrow)]
    type_offsets: ZeroVec<'a, i32>,
    #[serde(borrow)]
    trans: Option<ZeroVec<'a, i32>>,
    #[serde(borrow)]
    trans_pre32: Option<ZeroVec<'a, i32>>,
    #[serde(borrow)]
    trans_post32: Option<ZeroVec<'a, i32>>,
    type_map: Option<&'a [u8]>,
    #[serde(borrow)]
    final_rule: Option<ZeroUTF16String<'a>>,
    final_raw: Option<i32>,
    final_year: Option<u32>,
    #[serde(borrow)]
    links: Option<ZeroVec<'a, u32>>,
}

#[derive(Debug)]
pub enum TzDataRule<'a> {
    // The rule data is boxed here due to the large size difference between the
    // `TzDataRuleData` struct and `u32`. It's not strictly necessary.
    Table(Box<TzDataRuleData<'a>>),
    Int(u32),
}

impl<'de: 'a, 'a> Deserialize<'de> for TzDataRule<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(TzDataRuleEnumVisitor {
            phantom: PhantomData,
        })
    }
}

struct TzDataRuleEnumVisitor<'a> {
    phantom: PhantomData<TzDataRule<'a>>,
}

impl<'de: 'a, 'a> Visitor<'de> for TzDataRuleEnumVisitor<'a> {
    type Value = TzDataRule<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned 32-bit integer or a table of rule data")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TzDataRule::Int(v))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let value = TzDataRuleData::deserialize(de::value::MapAccessDeserializer::new(map))?;

        Ok(TzDataRule::Table(Box::new(value)))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "zoneinfo64")]
#[serde(rename_all = "PascalCase")]
pub struct ZoneInfo64<'a> {
    #[serde(borrow)]
    pub zones: Vec<TzDataRule<'a>>,
    #[serde(borrow)]
    pub names: Vec<ZeroUTF16String<'a>>,
    #[serde(borrow)]
    pub rules: HashMap<&'a str, ZeroVec<'a, i32>>,
    #[serde(borrow)]
    pub regions: Vec<ZeroUTF16String<'a>>,
}

fn main() {
    let in_bytes = include_bytes!("data/zoneinfo64.res");

    resb::binary::from_bytes::<ZoneInfo64>(in_bytes)
        .expect("Error processing resource bundle file");
}
