use super::LiteMap;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde_serialize")]
impl<K: Serialize, V: Serialize> Serialize for LiteMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Many human-readable formats don't support values other
        // than numbers and strings as map keys. For them, we can serialize
        // as a vec of tuples instead
        if serializer.is_human_readable() {
            if let Some(&(ref k, _)) = self.values.get(0) {
                let json = serde_json::json!(k);
                if !json.is_string() && !json.is_number() {
                    return self.values.serialize(serializer);
                }
                // continue to regular serialization
            }
        }
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

/// Modified example from https://serde.rs/deserialize-map.html
struct LiteMapVisitor<K, V> {
    marker: PhantomData<fn() -> LiteMap<K, V>>,
}

impl<K, V> LiteMapVisitor<K, V> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for LiteMapVisitor<K, V>
where
    K: Deserialize<'de> + Ord,
    V: Deserialize<'de>,
{
    type Value = LiteMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map produced by LiteMap")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut map = LiteMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_element()? {
            // Try to append it at the end, hoping for a sorted map.
            // If not sorted, insert as usual.
            // This allows for arbitrary maps (e.g. from user JSON)
            // to be deserialized into LiteMap
            // without impacting performance in the case of deserializing
            // a serialized map that came from another LiteMap
            if let Some((key, value)) = map.try_append(key, value) {
                // Note: this effectively selection sorts the map,
                // which isn't efficient for large maps
                map.insert(key, value);
            }
        }

        Ok(map)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = LiteMap::with_capacity(access.size_hint().unwrap_or(0));

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            // Try to append it at the end, hoping for a sorted map.
            // If not sorted, insert as usual.
            // This allows for arbitrary maps (e.g. from user JSON)
            // to be deserialized into LiteMap
            // without impacting performance in the case of deserializing
            // a serialized map that came from another LiteMap
            if let Some((key, value)) = map.try_append(key, value) {
                // Note: this effectively selection sorts the map,
                // which isn't efficient for large maps
                map.insert(key, value);
            }
        }

        Ok(map)
    }
}

impl<'de, K: Ord + Deserialize<'de>, V: Deserialize<'de>> Deserialize<'de> for LiteMap<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            // deserialize_any only works on self-describing (human-readable)
            // formats
            deserializer.deserialize_any(LiteMapVisitor::new())
        } else {
            deserializer.deserialize_map(LiteMapVisitor::new())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::LiteMap;
    use alloc::borrow::ToOwned;
    use alloc::string::String;
    use alloc::vec;

    fn get_simple_map() -> LiteMap<u32, String> {
        vec![
            (1, "one".to_owned()),
            (2, "two".to_owned()),
            (4, "four".to_owned()),
            (5, "five".to_owned()),
        ]
        .into_iter()
        .collect()
    }

    fn get_tuple_map() -> LiteMap<(u32, String), String> {
        vec![
            ((1, "en".to_owned()), "one".to_owned()),
            ((1, "zh".to_owned()), "一".to_owned()),
            ((2, "en".to_owned()), "two".to_owned()),
            ((2, "zh".to_owned()), "二".to_owned()),
            ((4, "en".to_owned()), "four".to_owned()),
            ((5, "en".to_owned()), "five".to_owned()),
            ((5, "zh".to_owned()), "五".to_owned()),
            ((7, "zh".to_owned()), "七".to_owned()),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_roundtrip_json() {
        let map = get_simple_map();
        let json = serde_json::to_string(&map).unwrap();
        assert_eq!(
            json,
            "{\"1\":\"one\",\"2\":\"two\",\"4\":\"four\",\"5\":\"five\"}"
        );
        let deserialized: LiteMap<u32, String> = serde_json::from_str(&json).unwrap();
        assert_eq!(map, deserialized);

        let map = get_tuple_map();
        let json = serde_json::to_string(&map).unwrap();
        assert_eq!(
            json,
            "[[[1,\"en\"],\"one\"],[[1,\"zh\"],\"一\"],[[2,\"en\"],\"two\"],\
                          [[2,\"zh\"],\"二\"],[[4,\"en\"],\"four\"],[[5,\"en\"],\"five\"],\
                          [[5,\"zh\"],\"五\"],[[7,\"zh\"],\"七\"]]"
        );
        let deserialized: LiteMap<(u32, String), String> = serde_json::from_str(&json).unwrap();
        assert_eq!(map, deserialized);
    }

    #[test]
    fn test_roundtrip_postcard() {
        let map = get_simple_map();
        let postcard = postcard::to_stdvec(&map).unwrap();
        let deserialized: LiteMap<u32, String> = postcard::from_bytes(&postcard).unwrap();
        assert_eq!(map, deserialized);

        let map = get_tuple_map();
        let postcard = postcard::to_stdvec(&map).unwrap();
        let deserialized: LiteMap<(u32, String), String> = postcard::from_bytes(&postcard).unwrap();
        assert_eq!(map, deserialized);
    }
}
