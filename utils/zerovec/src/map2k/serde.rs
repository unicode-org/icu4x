// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{ZeroMap2k, ZeroMap2kBorrowed};
use crate::map::{MutableZeroVecLike, ZeroMapKV, ZeroVecLike};
use crate::ZeroVec;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'a, K0, K1, V> Serialize for ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Serialize + ?Sized,
    K1: ZeroMapKV<'a> + Serialize + ?Sized,
    V: ZeroMapKV<'a> + Serialize + ?Sized,
    K0::Container: Serialize,
    K1::Container: Serialize,
    V::Container: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            todo!()
            // let mut map = serializer.serialize_map(Some(self.len()))?;
            // for (k0, K1, v) in self.iter() {
            //     K::Container::t_with_ser(k, |k| map.serialize_key(k))?;
            //     V::Container::t_with_ser(v, |v| map.serialize_value(v))?;
            // }
            // map.end()
        } else {
            (&self.keys0, &self.joiner, &self.keys1, &self.values).serialize(serializer)
        }
    }
}

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'a, K0, K1, V> Serialize for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Serialize + ?Sized,
    K1: ZeroMapKV<'a> + Serialize + ?Sized,
    V: ZeroMapKV<'a> + Serialize + ?Sized,
    K0::Container: Serialize,
    K1::Container: Serialize,
    V::Container: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ZeroMap2k::<K0, K1, V>::from(*self).serialize(serializer)
    }
}

/// Modified example from https://serde.rs/deserialize-map.html
struct ZeroMap2kMapVisitor<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    #[allow(clippy::type_complexity)] // it's a marker type, complexity doesn't matter
    marker: PhantomData<fn() -> (&'a K0::OwnedType, &'a K1::OwnedType, &'a V::OwnedType)>,
}

impl<'a, K0, K1, V> ZeroMap2kMapVisitor<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
{
    fn new() -> Self {
        ZeroMap2kMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'a, 'de, K0, K1, V> Visitor<'de> for ZeroMap2kMapVisitor<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Ord + ?Sized,
    K1: ZeroMapKV<'a> + Ord + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    K0::OwnedType: Deserialize<'de>,
    K1::OwnedType: Deserialize<'de>,
    V::OwnedType: Deserialize<'de>,
{
    type Value = ZeroMap2k<'a, K0, K1, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map produced by ZeroMap2k")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        todo!()
        // let mut map = ZeroMap2k::with_capacity(access.size_hint().unwrap_or(0));

        // // While there are entries remaining in the input, add them
        // // into our map.
        // while let Some((key, value)) = access.next_entry::<K::OwnedType, V::OwnedType>()? {
        //     // Try to append it at the end, hoping for a sorted map.
        //     // If not sorted, return an error
        //     // a serialized map that came from another ZeroMap2k
        //     if map
        //         .try_append(
        //             K::Container::owned_as_t(&key),
        //             V::Container::owned_as_t(&value),
        //         )
        //         .is_some()
        //     {
        //         return Err(de::Error::custom(
        //             "ZeroMap2k's keys must be sorted while deserializing",
        //         ));
        //     }
        // }

        // Ok(map)
    }
}

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'de, 'a, K0, K1, V> Deserialize<'de> for ZeroMap2k<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Ord + ?Sized,
    K1: ZeroMapKV<'a> + Ord + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    K0::Container: Deserialize<'de>,
    K1::Container: Deserialize<'de>,
    V::Container: Deserialize<'de>,
    K0::OwnedType: Deserialize<'de>,
    K1::OwnedType: Deserialize<'de>,
    V::OwnedType: Deserialize<'de>,
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_map(ZeroMap2kMapVisitor::<'a, K0, K1, V>::new())
        } else {
            let (keys0, joiner, keys1, values): (
                K0::Container,
                ZeroVec<u32>,
                K1::Container,
                V::Container,
            ) = Deserialize::deserialize(deserializer)?;
            if keys0.zvl_len() != joiner.len() {
                return Err(de::Error::custom(
                    "Mismatched keys0 and joiner sizes in ZeroMap2k",
                ));
            }
            if keys1.zvl_len() != values.zvl_len() {
                return Err(de::Error::custom(
                    "Mismatched keys1 and value sizes in ZeroMap2k",
                ));
            }
            if !keys0.zvl_is_ascending() {
                return Err(de::Error::custom(
                    "ZeroMap2k deserializing keys0 out of order",
                ));
            }
            if !joiner.zvl_is_ascending() {
                return Err(de::Error::custom(
                    "ZeroMap2k deserializing joiner array out of order",
                ));
            }
            if joiner.last().is_some() && joiner.last().map(|x| x as usize) != Some(keys1.zvl_len())
            {
                return Err(de::Error::custom(
                    "ZeroMap2k deserializing joiner array malformed",
                ));
            }
            // TODO: Check the following additional invariants:
            // - keys1 ascending in ranges
            Ok(Self {
                keys0,
                joiner,
                keys1,
                values,
            })
        }
    }
}

// /// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'de, 'a, K0, K1, V> Deserialize<'de> for ZeroMap2kBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + Ord + ?Sized,
    K1: ZeroMapKV<'a> + Ord + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    K0::Container: Deserialize<'de>,
    K1::Container: Deserialize<'de>,
    V::Container: Deserialize<'de>,
    K0::OwnedType: Deserialize<'de>,
    K1::OwnedType: Deserialize<'de>,
    V::OwnedType: Deserialize<'de>,
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Err(de::Error::custom(
                "ZeroMap2kBorrowed cannot be deserialized from human-readable formats",
            ))
        } else {
            let deserialized: ZeroMap2k<'a, K0, K1, V> = ZeroMap2k::deserialize(deserializer)?;
            let keys0 = if let Some(keys0) = deserialized.keys0.zvl_as_borrowed_inner() {
                keys0
            } else {
                return Err(de::Error::custom(
                    "ZeroMap2kBorrowed can only deserialize in zero-copy ways",
                ));
            };
            let joiner = if let Some(joiner) = deserialized.joiner.zvl_as_borrowed_inner() {
                joiner
            } else {
                return Err(de::Error::custom(
                    "ZeroMap2kBorrowed can only deserialize in zero-copy ways",
                ));
            };
            let keys1 = if let Some(keys1) = deserialized.keys1.zvl_as_borrowed_inner() {
                keys1
            } else {
                return Err(de::Error::custom(
                    "ZeroMap2kBorrowed can only deserialize in zero-copy ways",
                ));
            };
            let values = if let Some(values) = deserialized.values.zvl_as_borrowed_inner() {
                values
            } else {
                return Err(de::Error::custom(
                    "ZeroMap2kBorrowed can only deserialize in zero-copy ways",
                ));
            };
            Ok(Self {
                keys0,
                joiner,
                keys1,
                values,
            })
        }
    }
}

#[cfg(test)]
#[allow(non_camel_case_types)]
mod test {
    use super::super::*;

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct DeriveTest_ZeroMap2k<'data> {
        #[serde(borrow)]
        _data: ZeroMap2k<'data, u16, str, [u8]>,
    }

    #[derive(::serde::Serialize, ::serde::Deserialize)]
    struct DeriveTest_ZeroMap2kBorrowed<'data> {
        #[serde(borrow)]
        _data: ZeroMap2kBorrowed<'data, u16, str, [u8]>,
    }

    const JSON_STR: &str = "{\"1\":\"uno\",\"2\":\"dos\",\"3\":\"tres\"}";
    const BINCODE_BYTES: &[u8] = &[
        8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0,
        0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0, 26, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
        0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 117, 110, 111, 100, 111, 115, 116, 114, 101, 115,
    ];

    fn make_map() -> ZeroMap2k<'static, u32, u16, str> {
        let mut map = ZeroMap2k::new();
        map.insert(&1, &1, "uno");
        map.insert(&2, &2, "dos");
        map.insert(&2, &3, "tres");
        map
    }

    #[test]
    fn test_serde_json() {
        let map = make_map();
        let json_str = serde_json::to_string(&map).expect("serialize");
        assert_eq!(JSON_STR, json_str);
        let new_map: ZeroMap2k<u32, u16, str> =
            serde_json::from_str(&json_str).expect("deserialize");
        assert_eq!(format!("{:?}", new_map), format!("{:?}", map));
    }

    #[test]
    fn test_bincode() {
        let map = make_map();
        let bincode_bytes = bincode::serialize(&map).expect("serialize");
        assert_eq!(BINCODE_BYTES, bincode_bytes);
        let new_map: ZeroMap2k<u32, u16, str> =
            bincode::deserialize(&bincode_bytes).expect("deserialize");
        assert_eq!(
            format!("{:?}", new_map),
            format!("{:?}", map).replace("Owned", "Borrowed"),
        );

        let new_map: ZeroMap2kBorrowed<u32, u16, str> =
            bincode::deserialize(&bincode_bytes).expect("deserialize");
        assert_eq!(
            format!("{:?}", new_map),
            format!("{:?}", map)
                .replace("Owned", "Borrowed")
                .replace("ZeroMap2k", "ZeroMap2kBorrowed")
        );
    }
}
