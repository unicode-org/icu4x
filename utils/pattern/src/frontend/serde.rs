// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::borrow::Cow;
use alloc::vec::Vec;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

type HumanReadablePattern<'a, B> =
    Vec<PatternItemCow<'a, <B as PatternBackend>::PlaceholderKeyCow<'a>>>;

#[derive(Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PatternString<P: PatternBackend>(pub Box<Pattern<P>>);

impl<B: PatternBackend> Clone for PatternString<B>
where
    Box<B::Store>: for<'a> From<&'a B::Store>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P: PatternBackend> core::ops::Deref for PatternString<P> {
    type Target = Pattern<P>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: PatternBackend> Default for PatternString<B>
where
    B: PatternBackend,
    for<'a> B::PlaceholderKeyCow<'a>: core::str::FromStr,
    for<'a> <B::PlaceholderKeyCow<'a> as core::str::FromStr>::Err: core::fmt::Debug,
{
    fn default() -> Self {
        #[allow(clippy::unwrap_used)] // todo does this work?
        Self(Pattern::<B>::try_from_str("", Default::default()).unwrap())
    }
}

#[cfg(feature = "serde")]
impl<'de, B: PatternBackend> serde::Deserialize<'de> for PatternString<B>
where
    B: PatternBackend,
    B::PlaceholderKeyCow<'de>: core::str::FromStr,
    <B::PlaceholderKeyCow<'de> as core::str::FromStr>::Err: core::fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(
            Pattern::<B>::try_from_str(&<Cow<str>>::deserialize(deserializer)?, Default::default())
                .map_err(<D::Error as ::serde::de::Error>::custom)?,
        ))
    }
}

pub fn deserialize_borrowed_cow<'de, E, B, D>(
    deserializer: D,
) -> Result<Cow<'de, Pattern<B>>, D::Error>
where
    D: Deserializer<'de>,
    B: PatternBackend<StoreFromBytesError = E, Store = str>,
    B::PlaceholderKeyCow<'de>: Deserialize<'de>,
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
    B::PlaceholderKeyCow<'data>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Pattern::<B>::try_from_items(
                <HumanReadablePattern<B>>::deserialize(deserializer)?.into_iter(),
            )
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
            B::iter_items(&self.store)
                .map(|x| x.into())
                .collect::<HumanReadablePattern<B>>()
                .serialize(serializer)
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
        assert_eq!(
            pattern_json,
            r#"[{"Literal":"Hello, "},{"Placeholder":"Singleton"},{"Literal":"!"}]"#
        );
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
