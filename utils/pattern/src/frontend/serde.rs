// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::borrow::Cow;
use alloc::vec::Vec;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

#[doc(hidden)]
pub fn deserialize_option_borrowed_cow<'de, D: Deserializer<'de>, B>(
    deserializer: D,
) -> Result<Option<Cow<'de, Pattern<B>>>, D::Error>
where
    B: PatternBackend<Store = str>,
    Pattern<B>: ToOwned,
    B::PlaceholderKeyCow<'de>: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(transparent, bound = "'data: 'de")]
    // Cows fail to borrow in some situations (array, option), but structs of Cows don't.
    struct CowPatternWrap<'data, B>(
        #[serde(borrow, deserialize_with = "deserialize_borrowed_cow")] pub Cow<'data, Pattern<B>>,
    )
    where
        B: PatternBackend<Store = str>,
        Pattern<B>: ToOwned,
        B::PlaceholderKeyCow<'data>: Deserialize<'data>;

    Option::<CowPatternWrap<'de, B>>::deserialize(deserializer)
        .map(|array| array.map(|wrap| wrap.0))
}

type HumanReadablePattern<'a, B> =
    Vec<PatternItemCow<'a, <B as PatternBackend>::PlaceholderKeyCow<'a>>>;

#[derive(Debug, PartialEq)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PatternString<B: PatternBackend>(pub Box<Pattern<B>>);

impl<B: PatternBackend> Clone for PatternString<B>
where
    Box<B::Store>: for<'a> From<&'a B::Store>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<B: PatternBackend> core::ops::Deref for PatternString<B> {
    type Target = Pattern<B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: PatternBackend> Default for PatternString<B>
where
    Box<B::Store>: for<'a> From<&'a B::Store>,
{
    fn default() -> Self {
        Self(Box::<Pattern<B>>::default())
    }
}

#[cfg(feature = "serde")]
impl<'de, B: PatternBackend> serde::Deserialize<'de> for PatternString<B>
where
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

#[doc(hidden)]
pub fn deserialize_borrowed_cow<'de, B, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Cow<'de, Pattern<B>>, D::Error>
where
    B: PatternBackend<Store = str>,
    B::PlaceholderKeyCow<'de>: Deserialize<'de>,
{
    if deserializer.is_human_readable() {
        Box::<Pattern<B>>::deserialize(deserializer).map(Cow::Owned)
    } else {
        <&Pattern<B>>::deserialize(deserializer).map(Cow::Borrowed)
    }
}

impl<'de, 'data, B> Deserialize<'de> for Box<Pattern<B>>
where
    'de: 'data,
    B: PatternBackend<Store = str>,
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

impl<'de, B: PatternBackend> Deserialize<'de> for &'de Pattern<B>
where
    &'de B::Store: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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

impl<B: PatternBackend> Serialize for Pattern<B>
where
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
