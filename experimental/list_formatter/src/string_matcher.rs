// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "provider_serde")]
use crate::error::Error;
use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use icu_provider::yoke::{self, *};
use regex_automata::{SparseDFA, DFA};

#[derive(Clone, Debug, PartialEq, Yokeable, ZeroCopyFrom)]
// TODO: Store the actual DFA instead of their serializations. This requires ZCF and Yokeable on them.
pub(crate) enum StringMatcher<'data> {
    // Constructor-created or deserialized from JSON. Always owned, Cow is required for ZCF.
    FromPattern(Cow<'data, str>, Cow<'data, [u8]>),
    // Deserialized from bincode. Always borrowed.
    Precomputed(Cow<'data, [u8]>),
}

#[cfg(feature = "provider_serde")]
impl serde::Serialize for StringMatcher<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            StringMatcher::FromPattern(regex, _) if serializer.is_human_readable() => {
                regex.serialize(serializer)
            }
            StringMatcher::FromPattern(_, dfa_bytes) => dfa_bytes.serialize(serializer),
            StringMatcher::Precomputed(dfa_bytes) if !serializer.is_human_readable() => {
                dfa_bytes.serialize(serializer)
            }
            _ => {
                use serde::ser::Error;
                Err(S::Error::custom(
                    "Cannot serialize a deserialized bincode StringMatcher to JSON.",
                ))
            }
        }
    }
}

#[cfg(feature = "provider_serde")]
impl<'de: 'data, 'data> serde::Deserialize<'de> for StringMatcher<'data> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            StringMatcher::new(<&str>::deserialize(deserializer)?).map_err(|e| {
                use alloc::string::ToString;
                use serde::de::Error;
                D::Error::custom(e.to_string())
            })
        } else {
            // TODO: Validate, https://github.com/BurntSushi/regex-automata/issues/20
            Ok(StringMatcher::Precomputed(<Cow<'de, [u8]>>::deserialize(
                deserializer,
            )?))
        }
    }
}

impl<'data> StringMatcher<'data> {
    #[cfg(feature = "provider_serde")]
    pub(crate) fn new(pattern: &str) -> Result<Self, Error> {
        let mut builder = regex_automata::dense::Builder::new();
        let dfa: regex_automata::DenseDFA<Vec<u16>, u16> = builder
            .anchored(true)
            .case_insensitive(true)
            .minimize(true)
            .build_with_size(pattern)
            .map_err(Error::IllegalCondition)?;

        let sparse_dfa = dfa
            .to_sparse_sized::<u16>()
            .map_err(Error::IllegalCondition)?;

        Ok(Self::FromPattern(
            Cow::Owned(String::from(pattern)),
            // We can unwrap because the u16 state size does not produce an error
            Cow::Owned(sparse_dfa.to_bytes_little_endian().unwrap()),
        ))
    }

    pub(crate) fn test(&self, string: &str) -> bool {
        #[cfg(target_endian = "big")]
        return false;

        let dfa_bytes: &[u8] = match self {
            StringMatcher::FromPattern(_, dfa_bytes) => &dfa_bytes,
            StringMatcher::Precomputed(dfa_bytes) => dfa_bytes,
        };

        // TODO: We've handled endianness, but there could still be other corruptions,
        // so this is unsafe.
        unsafe { SparseDFA::<&[u8], u16>::from_bytes(dfa_bytes) }.find(string.as_bytes())
            == Some(string.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_matcher() {
        let matcher = StringMatcher::new("abc.*").unwrap();
        assert!(!matcher.test("ab"));
        assert!(matcher.test("abc"));
        assert!(matcher.test("abcde"));
    }
}
