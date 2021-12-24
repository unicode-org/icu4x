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

#[derive(Clone, Debug, Yokeable, ZeroCopyFrom)]
// TODO: Store the actual DFA instead of their serializations. This requires ZCF and Yokeable on them.
pub(crate) enum StringMatcher<'data> {
    // Constructor-created or deserialized from JSON. Always owned, Cow is required for ZCF.
    FromPattern(Cow<'data, str>, Cow<'data, [u8]>),
    // Deserialized from bincode. Always borrowed.
    Precomputed(Cow<'data, [u8]>),
}

impl PartialEq for StringMatcher<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                StringMatcher::FromPattern(pattern1, _),
                StringMatcher::FromPattern(pattern2, _),
            ) => pattern1 == pattern2,
            (StringMatcher::Precomputed(bytes1), StringMatcher::FromPattern(_, bytes2)) => {
                bytes1 == bytes2
            }
            (StringMatcher::FromPattern(_, bytes1), StringMatcher::Precomputed(bytes2)) => {
                bytes1 == bytes2
            }
            (StringMatcher::Precomputed(bytes1), StringMatcher::Precomputed(bytes2)) => {
                bytes1 == bytes2
            }
        }
    }
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
            if cfg!(target_endian = "big") {
                // TODO: Convert LE to BE. For now we just behave like the
                // accept-nothing DFA on BE systems.
                return Ok(StringMatcher::Precomputed(Cow::Borrowed(&[])));
            }

            let bytes = <Cow<'de, [u8]>>::deserialize(deserializer)?;
            // TODO: Validate, see https://github.com/BurntSushi/regex-automata/issues/20
            Ok(StringMatcher::Precomputed(bytes))
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

        let dfa = match self {
            StringMatcher::FromPattern(_, dfa_bytes) => unsafe {
                // This is safe because we created these bytes ourselves
                SparseDFA::<&[u8], u16>::from_bytes(&dfa_bytes)
            },
            StringMatcher::Precomputed(dfa_bytes) => unsafe {
                // TODO: This is not safe
                SparseDFA::<&[u8], u16>::from_bytes(dfa_bytes)
            },
        };

        dfa.find(string.as_bytes()) == Some(string.len())
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

    #[test]
    fn test_postcard_serialization() {
        let matcher = StringMatcher::new("abc*").unwrap();

        let bytes = postcard::to_stdvec(&matcher).unwrap();
        assert_eq!(
            postcard::from_bytes::<StringMatcher>(&bytes).unwrap(),
            matcher
        );
    }

    #[test]
    fn test_json_serialization() {
        let matcher = StringMatcher::new("abc*").unwrap();

        let json = serde_json::to_string(&matcher).unwrap();
        assert_eq!(
            serde_json::from_str::<StringMatcher>(&json).unwrap(),
            matcher
        );
        assert!(serde_json::from_str::<StringMatcher>(&".*[").is_err());
    }
}
