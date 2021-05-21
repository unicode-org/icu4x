// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{ZeroMap, ZeroMapKV, ZeroVecLike};
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'a, K, V> Serialize for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K::Container: Serialize,
    V::Container: Serialize,
    K::SerializeType: Serialize,
    V::SerializeType: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in self.iter() {
                K::with_ser(k, |k| map.serialize_key(k))?;
                V::with_ser(v, |v| map.serialize_value(v))?;
            }
            map.end()
        } else {
            (&self.keys, &self.values).serialize(serializer)
        }
    }
}

/// Modified example from https://serde.rs/deserialize-map.html
struct ZeroMapMapVisitor<K, V> {
    marker: PhantomData<fn() -> (K, V)>,
}

impl<K, V> ZeroMapMapVisitor<K, V> {
    fn new() -> Self {
        ZeroMapMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for ZeroMapMapVisitor<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
    K: ZeroMapKV<'de>,
    V: ZeroMapKV<'de>,
{
    type Value = ZeroMap<'de, K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map produced by ZeroMap")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = ZeroMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            // Try to append it at the end, hoping for a sorted map.
            // If not sorted, return an error
            // a serialized map that came from another ZeroMap
            if map.try_append(key, value).is_some() {
                return Err(de::Error::custom(
                    "ZeroMap's keys must be sorted while deserializing",
                ));
            }
        }

        Ok(map)
    }
}

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'de, K, V> Deserialize<'de> for ZeroMap<'de, K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
    K::Container: Deserialize<'de>,
    V::Container: Deserialize<'de>,
    K: ZeroMapKV<'de>,
    V: ZeroMapKV<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_map(ZeroMapMapVisitor::new())
        } else {
            let (keys, values): (K::Container, V::Container) =
                Deserialize::deserialize(deserializer)?;
            if keys.len() != values.len() {
                return Err(de::Error::custom(
                    "Mismatched key and value sizes in ZeroMap",
                ));
            }
            if !keys.is_ascending() {
                return Err(de::Error::custom("ZeroMap deserializing keys out of order"));
            }
            Ok(Self { keys, values })
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    const JSON_STR: &str = "{\"1\":\"uno\",\"2\":\"dos\",\"3\":\"tres\"}";
    const BINCODE_BYTES: &[u8] = &[
        12, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 117, 110, 111, 100, 111, 115, 116, 114, 101, 115,
    ];

    fn make_map() -> ZeroMap<'static, u32, String> {
        let mut map = ZeroMap::new();
        map.insert(1, "uno".to_owned());
        map.insert(2, "dos".to_owned());
        map.insert(3, "tres".to_owned());
        map
    }
    #[test]
    fn test_serde_json() {
        let map = make_map();
        let json_str = serde_json::to_string(&map).expect("serialize");
        assert_eq!(JSON_STR, json_str);
        let new_map: ZeroMap<u32, String> = serde_json::from_str(&json_str).expect("deserialize");
        assert_eq!(
            new_map.iter().collect::<Vec<_>>(),
            map.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_bincode() {
        let map = make_map();
        let bincode_bytes = bincode::serialize(&map).expect("serialize");
        assert_eq!(BINCODE_BYTES, bincode_bytes);
        let new_map: ZeroMap<u32, String> =
            bincode::deserialize(&bincode_bytes).expect("deserialize");
        assert_eq!(
            new_map.iter().collect::<Vec<_>>(),
            map.iter().collect::<Vec<_>>()
        );
    }
}
