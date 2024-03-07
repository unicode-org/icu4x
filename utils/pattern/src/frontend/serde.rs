// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;

use ::serde::{Serialize, Deserialize, Serializer, Deserializer};

type HumanReadablePatternForSerde<T> = Vec<PatternItemCow<'static, T>>;

impl<'de, 'data, B, Store> Deserialize<'de> for Pattern<B, Store> where 'de: 'data, B: PatternBackend, B::Store: ToOwned, B::PlaceholderKey: Deserialize<'de>, Store: Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let pattern_items = <HumanReadablePatternForSerde<B::PlaceholderKey>>::deserialize(deserializer)?;
            let pattern = Self::try_from_items(pattern_items)?;
            Ok(pattern)
        } else {
            let store = Store::deserialize(deserializer)?;
            let pattern = Self::try_from_store(store)?;
            Ok(pattern)
        }
    }
}

impl<B, Store> Serialize for Pattern<B, Store> where B: PatternBackend, B::Store: Serialize, Store: AsRef<B::Store> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.store.as_ref();
        bytes.serialize(serializer)
        /*
        if serializer.is_human_readable() {
            match core::str::from_utf8(bytes) {
                Ok(s) => serializer.serialize_str(s),
                Err(_) => serializer.serialize_bytes(bytes),
            }
        } else {
            serializer.serialize_bytes(bytes)
        }
        */
    }
}
