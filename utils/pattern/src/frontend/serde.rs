// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::borrow::Cow;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn deserialize_borrowed_cow<'de, E, B, D>(
    deserializer: D,
) -> Result<Cow<'de, Pattern<B>>, D::Error>
where
    D: Deserializer<'de>,
    B: PatternBackend<StoreFromBytesError = E, Store = str>,
    B::PlaceholderKeyCow<'de>: FromStr,
    <B::PlaceholderKeyCow<'de> as FromStr>::Err: fmt::Debug,
{
    if deserializer.is_human_readable() {
        Box::<Pattern<B>>::deserialize(deserializer).map(Cow::Owned)
    } else {
        <&Pattern<B>>::deserialize(deserializer).map(Cow::Borrowed)
    }
}

impl<'de, 'data, B, E> Deserialize<'de> for Box<Pattern<B>>
where
    'de: 'data,
    B: PatternBackend<StoreFromBytesError = E, Store = str>,
    B::PlaceholderKeyCow<'data>: FromStr,
    <B::PlaceholderKeyCow<'data> as FromStr>::Err: fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Pattern::<B>::try_from_str(&Cow::<str>::deserialize(deserializer)?, Default::default())
        } else {
            let store = Box::<B::Store>::deserialize(deserializer)?;
            B::validate_store(&store).map(|()| Pattern::<B>::from_boxed_store_unchecked(store))
        }
        .map_err(<D::Error as ::serde::de::Error>::custom)
    }
}

impl<'data, B, E> Deserialize<'data> for &'data Pattern<B>
where
    B: PatternBackend<StoreFromBytesError = E>,
    for<'a> &'a B::Store: Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'data>,
    {
        if deserializer.is_human_readable() {
            Err(<D::Error as ::serde::de::Error>::custom(
                "human readable format cannot be borrowed",
            ))
        } else {
            let store = <&B::Store>::deserialize(deserializer)?;
            B::validate_store(store).map_err(<D::Error as ::serde::de::Error>::custom)?;
            Ok(Pattern::from_ref_store_unchecked(store))
        }
    }
}

impl<B> Serialize for Pattern<B>
where
    B: PatternBackend,
    B::Store: Serialize,
    for<'a> B::PlaceholderKeyCow<'a>: Serialize + From<B::PlaceholderKey<'a>>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut pattern = String::new();
            for item in B::iter_items(&self.store) {
                match item {
                    PatternItem::Literal(s) => pattern.push_str(s),
                    PatternItem::Placeholder(p) => B::escape_placeholder(&mut pattern, p),
                }
            }
            pattern.serialize(serializer)
        } else {
            self.store.serialize(serializer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SinglePlaceholderPattern;
    use alloc::borrow::Cow;

    #[test]
    fn test_json() {
        let pattern_owned =
            SinglePlaceholderPattern::try_from_str("Hello, {0}!", Default::default()).unwrap();
        let pattern_cow: Cow<SinglePlaceholderPattern> = Cow::Owned(pattern_owned);
        let pattern_json = serde_json::to_string(&pattern_cow).unwrap();
        assert_eq!(pattern_json, r#""Hello, {0}!""#);
        let pattern_deserialized: Cow<SinglePlaceholderPattern> =
            deserialize_borrowed_cow(&mut serde_json::Deserializer::from_str(&pattern_json))
                .unwrap();
        assert_eq!(pattern_cow, pattern_deserialized);
        assert!(matches!(pattern_deserialized, Cow::Owned(_)));
    }

    #[test]
    fn test_postcard() {
        let pattern_owned =
            SinglePlaceholderPattern::try_from_str("Hello, {0}!", Default::default()).unwrap();
        let pattern_cow: Cow<SinglePlaceholderPattern> = Cow::Owned(pattern_owned);
        let pattern_postcard = postcard::to_stdvec(&pattern_cow).unwrap();
        assert_eq!(pattern_postcard, b"\x09\x08Hello, !");
        let pattern_deserialized =
            deserialize_borrowed_cow(&mut postcard::Deserializer::from_bytes(&pattern_postcard))
                .unwrap();
        assert_eq!(pattern_cow, pattern_deserialized);
        assert!(matches!(pattern_deserialized, Cow::Borrowed(_)));
    }

    #[test]
    fn test_rmp() {
        let pattern_owned =
            SinglePlaceholderPattern::try_from_str("Hello, {0}!", Default::default()).unwrap();
        let pattern_cow: Cow<SinglePlaceholderPattern> = Cow::Owned(pattern_owned);
        let pattern_rmp = rmp_serde::to_vec(&pattern_cow).unwrap();
        assert_eq!(pattern_rmp, b"\xA9\x08Hello, !");
        let pattern_deserialized =
            deserialize_borrowed_cow(&mut rmp_serde::Deserializer::from_read_ref(&pattern_rmp))
                .unwrap();
        assert_eq!(pattern_cow, pattern_deserialized);
        assert!(matches!(pattern_deserialized, Cow::Borrowed(_)));
    }
}
