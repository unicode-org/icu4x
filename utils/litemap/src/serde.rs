use super::LiteMap;
use core::fmt;
use core::marker::PhantomData;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde_serialize")]
use serde_json;

#[cfg(feature = "serde_serialize")]
impl<K: Serialize, V: Serialize> Serialize for LiteMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(&(ref k, _)) = self.values.get(0) {
            let json = serde_json::json!(k);
            if !json.is_string() && !json.is_number() {
                 //  serialize as a vec of tuples (just serialize `self.values` directly)
            }
           // continue to regular serialization
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
        deserializer.deserialize_map(LiteMapVisitor::new())
    }
}
