// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::fmt::Display;

use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

type HumanReadablePattern<'a, B> =
    Vec<PatternItemCow<'a, <B as PatternBackend>::PlaceholderKeyCow<'a>>>;

impl<'de, 'data, B, Store> Deserialize<'de> for Pattern<B, Store>
where
    'de: 'data,
    B: PatternBackend,
    B::Store: ToOwned + 'de,
    &'de B::Store: Deserialize<'de>,
    B::PlaceholderKeyCow<'data>: Deserialize<'de>,
    Store: TryFrom<Cow<'data, B::Store>> + AsRef<B::Store>,
    Store::Error: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let pattern_items = <HumanReadablePattern<B>>::deserialize(deserializer)?;
            let pattern_owned: Pattern<B, <B::Store as ToOwned>::Owned> =
                Pattern::try_from_items(pattern_items.into_iter())
                    .map_err(<D::Error as ::serde::de::Error>::custom)?;
            let pattern: Pattern<B, Store> = Pattern::from_store_unchecked(
                Cow::<B::Store>::Owned(pattern_owned.take_store())
                    .try_into()
                    .map_err(<D::Error as ::serde::de::Error>::custom)?,
            );
            Ok(pattern)
        } else {
            let store = <&B::Store>::deserialize(deserializer)?;
            let pattern = Self::try_from_store(
                Cow::Borrowed(store)
                    .try_into()
                    .map_err(<D::Error as ::serde::de::Error>::custom)?,
            )
            .map_err(<D::Error as ::serde::de::Error>::custom)?;
            Ok(pattern)
        }
    }
}

impl<B, Store> Serialize for Pattern<B, Store>
where
    B: PatternBackend,
    B::Store: Serialize,
    for<'a> B::PlaceholderKeyCow<'a>: Serialize + From<B::PlaceholderKey<'a>>,
    Store: AsRef<B::Store>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let pattern_items: HumanReadablePattern<B> = B::iter_items(self.store.as_ref())
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
        assert!(matches!(pattern_deserialized.take_store(), Cow::Owned(_)));
    }

    #[test]
    fn test_postcard() {
        let pattern_owned = SinglePlaceholderPattern::try_from_str("Hello, {0}!").unwrap();
        let pattern_cow: SinglePlaceholderPattern<Cow<str>> =
            SinglePlaceholderPattern::from_store_unchecked(Cow::Owned(pattern_owned.take_store()));
        let pattern_postcard = postcard::to_stdvec(&pattern_cow).unwrap();
        assert_eq!(pattern_postcard, b"\x09\x08Hello, !");
        let pattern_deserialized: SinglePlaceholderPattern<Cow<str>> =
            postcard::from_bytes(&pattern_postcard).unwrap();
        assert_eq!(pattern_cow, pattern_deserialized);
        assert!(matches!(
            pattern_deserialized.take_store(),
            Cow::Borrowed(_)
        ));
    }

    macro_rules! check_store {
        ($store:expr, $ty:ty) => {
            check_store!(@borrow, $store, $ty);
            let json = serde_json::to_string::<SinglePlaceholderPattern<$ty>>(
                &SinglePlaceholderPattern::from_store_unchecked($store.clone()),
            )
            .unwrap();
            let de_json = serde_json::from_str::<SinglePlaceholderPattern<$ty>>(&json).unwrap();
            assert_eq!(de_json.take_store(), $store);
        };
        (@borrow, $store:expr, $ty:ty) => {
            let postcard = postcard::to_stdvec::<SinglePlaceholderPattern<$ty>>(
                &SinglePlaceholderPattern::from_store_unchecked($store.clone()),
            )
            .unwrap();
            let de_postcard = postcard::from_bytes::<SinglePlaceholderPattern<$ty>>(&postcard).unwrap();
            assert_eq!(de_postcard.take_store(), $store);
        };
    }

    #[test]
    fn test_serde_stores() {
        let store = SinglePlaceholderPattern::try_from_str("Hello, {0}!")
            .unwrap()
            .take_store();

        check_store!(Cow::Borrowed(store.as_str()), Cow<str>);
        check_store!(Cow::<str>::Owned(store.clone()), Cow<str>);
        check_store!(store.clone(), String);

        /// A type implementing TryFrom<Cow<str>> that returns an error if the Cow is Owned
        #[derive(Debug, Clone, PartialEq, displaydoc::Display)]
        struct MyStr<'a>(&'a str);
        impl<'a> TryFrom<Cow<'a, str>> for MyStr<'a> {
            type Error = &'static str;
            fn try_from(input: Cow<'a, str>) -> Result<MyStr<'a>, Self::Error> {
                match input {
                    Cow::Borrowed(s) => Ok(MyStr(s)),
                    Cow::Owned(_) => Err("cannot borrow from a Cow with needed lifetime"),
                }
            }
        }
        impl AsRef<str> for MyStr<'_> {
            fn as_ref(&self) -> &str {
                self.0
            }
        }

        check_store!(@borrow, MyStr(store.as_str()), MyStr);
    }
}
