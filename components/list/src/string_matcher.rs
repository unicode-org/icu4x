// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
#[cfg(any(feature = "serde_human", feature = "datagen"))]
use alloc::string::ToString;
use icu_provider::{yoke, zerofrom};
use regex_automata::dfa::sparse::DFA;
use regex_automata::dfa::Automaton;

#[derive(Clone, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct StringMatcher<'data> {
    // Safety: These always represent a valid DFA (DFA::from_bytes(dfa_bytes).is_ok())
    dfa_bytes: Cow<'data, [u8]>,
    pattern: Option<Cow<'data, str>>,
}

impl PartialEq for StringMatcher<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.dfa_bytes == other.dfa_bytes
    }
}

#[cfg(feature = "datagen")]
impl serde::Serialize for StringMatcher<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        if serializer.is_human_readable() {
            self.pattern
                .as_ref()
                .map(|pattern| pattern.serialize(serializer))
                .unwrap_or_else(|| {
                    use serde::ser::Error;
                    Err(S::Error::custom(
                        "cannot serialize a deserialized bincode StringMatcher to JSON",
                    ))
                })
        } else {
            self.dfa_bytes.serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de: 'data, 'data> serde::Deserialize<'de> for StringMatcher<'data> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use icu_provider::serde::borrow_de_utils::CowBytesWrap;

        #[cfg(feature = "serde_human")]
        if deserializer.is_human_readable() {
            use serde::de::Error;
            return StringMatcher::new(<&str>::deserialize(deserializer)?)
                .map_err(|e| D::Error::custom(e.to_string()));
        }

        if cfg!(target_endian = "big") {
            // TODO: Convert LE to BE. For now we just behave like the
            // accept-nothing DFA on BE systems.
            return Ok(StringMatcher {
                dfa_bytes: Cow::Borrowed(&[]),
                pattern: None,
            });
        }

        let dfa_bytes = <CowBytesWrap<'de>>::deserialize(deserializer)?.0;

        // Verify safety invariant
        DFA::from_bytes(&dfa_bytes).map_err(|e| {
            use serde::de::Error;
            D::Error::custom(alloc::format!("Invalid DFA bytes: {}", e))
        })?;

        Ok(StringMatcher {
            dfa_bytes,
            pattern: None,
        })
    }
}

impl<'data> StringMatcher<'data> {
    #[cfg(any(feature = "datagen", feature = "serde_human",))]
    pub fn new(pattern: &str) -> Result<Self, icu_provider::DataError> {
        use regex_automata::{
            dfa::dense::{Builder, Config},
            SyntaxConfig,
        };

        let mut builder = Builder::new();
        let dfa = builder
            .syntax(SyntaxConfig::new().case_insensitive(true))
            .configure(Config::new().anchored(true).minimize(true))
            .build(pattern)
            .map_err(|_| {
                icu_provider::DataError::custom("Cannot build DFA").with_display_context(&pattern)
            })?
            .to_sparse()
            .map_err(|_| {
                icu_provider::DataError::custom("Cannot sparsify DFA")
                    .with_display_context(&pattern)
            })?;

        Ok(Self {
            dfa_bytes: dfa.to_bytes_little_endian().into(),
            pattern: Some(pattern.to_string().into()),
        })
    }

    pub fn test(&self, string: &str) -> bool {
        cfg!(target_endian = "little")
            && matches!(
                // Safe due to struct invariant.
                unsafe { DFA::from_bytes_unchecked(&self.dfa_bytes).unwrap().0 }
                    .find_earliest_fwd(string.as_bytes()),
                Ok(Some(_))
            )
    }
}

#[cfg(all(test, feature = "datagen"))]
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

        let mut bytes = postcard::to_stdvec(&matcher).unwrap();
        assert_eq!(
            postcard::from_bytes::<StringMatcher>(&bytes).unwrap(),
            matcher
        );

        // A corrupted byte leads to an error
        bytes[17] ^= 255;
        assert!(postcard::from_bytes::<StringMatcher>(&bytes).is_err());
        bytes[17] ^= 255;

        // An extra byte leads to an error
        bytes.insert(123, 40);
        assert!(postcard::from_bytes::<StringMatcher>(&bytes).is_err());
        bytes.remove(123);

        // Missing bytes lead to an error
        assert!(postcard::from_bytes::<StringMatcher>(&bytes[0..bytes.len() - 5]).is_err());
    }

    #[test]
    #[cfg(feature = "serde_human")]
    fn test_json_serialization() {
        let matcher = StringMatcher::new("abc*").unwrap();

        let json = serde_json::to_string(&matcher).unwrap();
        assert_eq!(
            serde_json::from_str::<StringMatcher>(&json).unwrap(),
            matcher
        );
        assert!(serde_json::from_str::<StringMatcher>(".*[").is_err());
    }
}
