// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
/// TODO
#[doc(hidden)]
macro_rules! __struct_keyword {
    ($(#[$doc:meta])* $name:ident, $ext_key:literal, $value:ty, $try_from:expr, $into:expr) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        #[allow(clippy::exhaustive_structs)] // TODO
        $(#[$doc])*
        pub struct $name(pub $value);

        impl TryFrom<icu_locale_core::extensions::unicode::Value> for $name {
            type Error = $crate::extensions::unicode::errors::PreferencesParseError;

            fn try_from(
                input: icu_locale_core::extensions::unicode::Value,
            ) -> Result<Self, Self::Error> {
                $try_from(input)
            }
        }

        impl From<$name> for icu_locale_core::extensions::unicode::Value {
            fn from(input: $name) -> icu_locale_core::extensions::unicode::Value {
                $into(input)
            }
        }

        impl $crate::PreferenceKey for $name {
            fn unicode_extension_key() -> Option<icu_locale_core::extensions::unicode::Key> {
                Some(icu_locale_core::extensions::unicode::key!($ext_key))
            }

            fn unicode_extension_value(
                &self,
            ) -> Option<icu_locale_core::extensions::unicode::Value> {
                Some(self.clone().into())
            }
        }
    };
}
pub use __struct_keyword as struct_keyword;

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;
    use icu_locale_core::{
        extensions::unicode,
        subtags::{subtag, Subtag},
    };

    #[test]
    fn struct_keywords_test() {
        struct_keyword!(
            /// TODO
            DummyKeyword,
            "dk",
            Subtag,
            |input: unicode::Value| {
                if let Some(subtag) = input.into_single_subtag() {
                    if subtag.len() == 3 {
                        return Ok(DummyKeyword(subtag));
                    }
                }
                Err(crate::extensions::unicode::errors::PreferencesParseError::InvalidKeywordValue)
            },
            |input: DummyKeyword| { unicode::Value::from_subtag(Some(input.0)) }
        );

        let v = unicode::Value::from_str("foo").unwrap();
        let dk: DummyKeyword = v.clone().try_into().unwrap();
        assert_eq!(dk, DummyKeyword(subtag!("foo")));
        assert_eq!(unicode::Value::from(dk), v);

        let v = unicode::Value::from_str("foobar").unwrap();
        let dk: Result<DummyKeyword, _> = v.clone().try_into();
        assert!(dk.is_err());
    }
}
