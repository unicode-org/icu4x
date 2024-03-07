// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::borrow::Cow;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

type HumanReadablePatternForSerde<'a, T> = Vec<PatternItemCow<'a, T>>;

impl<'de, 'data, B> Deserialize<'de> for Pattern<B, Cow<'data, B::Store>>
where
    'de: 'data,
    B: PatternBackend,
    B::Store: ToOwned,
    <B::Store as ToOwned>::Owned: Deserialize<'de>,
    B::PlaceholderKey: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let pattern_items =
                <HumanReadablePatternForSerde<B::PlaceholderKey>>::deserialize(deserializer)?;
            let pattern_owned: Pattern<B, <B::Store as ToOwned>::Owned> =
                Pattern::try_from_items(pattern_items.into_iter())
                    .map_err(|e| <D::Error as ::serde::de::Error>::custom(e))?;
            let pattern_cow: Pattern<B, Cow<B::Store>> =
                Pattern::from_store_unchecked(Cow::Owned(pattern_owned.take_store()));
            Ok(pattern_cow)
        } else {
            let store = Cow::<B::Store>::deserialize(deserializer)?;
            let pattern = Self::try_from_store(store)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e))?;
            Ok(pattern)
        }
    }
}

impl<B, Store> Serialize for Pattern<B, Store>
where
    B: PatternBackend,
    B::Store: Serialize,
    B::PlaceholderKey: Serialize,
    Store: AsRef<B::Store>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let pattern_items: HumanReadablePatternForSerde<B::PlaceholderKey> =
                B::iter_items(self.store.as_ref())
                    .map(|x| x.into())
                    .collect();
            pattern_items.serialize(serializer)
        } else {
            let bytes = self.store.as_ref();
            bytes.serialize(serializer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SinglePlaceholderPattern;

    #[test]
    fn test_json() {
        let pattern_owned = SinglePlaceholderPattern::try_from_str("Hello, {0}!").unwrap();
        let pattern_cow: SinglePlaceholderPattern<Cow<str>> =
            SinglePlaceholderPattern::from_store_unchecked(Cow::Owned(pattern_owned.take_store()));
        let pattern_json = serde_json::to_string(&pattern_cow).unwrap();
        assert_eq!(
            pattern_json,
            r#"[{"Literal":"Hello, "},{"Placeholder":"Singleton"},{"Literal":"!"}]"#
        );
        let pattern_deserialized: SinglePlaceholderPattern<Cow<str>> =
            serde_json::from_str(&pattern_json).unwrap();
        assert_eq!(pattern_cow, pattern_deserialized);
    }

    #[test]
    fn test_postcard() {
        let pattern_owned = SinglePlaceholderPattern::try_from_str("Hello, {0}!").unwrap();
        let pattern_cow: SinglePlaceholderPattern<Cow<str>> =
            SinglePlaceholderPattern::from_store_unchecked(Cow::Owned(pattern_owned.take_store()));
        let pattern_postcard = postcard::to_stdvec(&pattern_cow).unwrap();
        assert_eq!(
            pattern_postcard,
            b"\x09\x08Hello, !"
        );
        let pattern_deserialized: SinglePlaceholderPattern<Cow<str>> =
            postcard::from_bytes(&pattern_postcard).unwrap();
        assert_eq!(pattern_cow, pattern_deserialized);
    }
}
