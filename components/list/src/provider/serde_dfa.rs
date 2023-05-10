// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use icu_provider::prelude::*;
use regex_automata::dfa::sparse::DFA;

/// A serde-compatible version of [regex_automata::dfa::sparse::DFA]. This does not implement
/// [`serde::Deserialize`] directly, as binary deserialization is not supported in big-endian
/// platforms. `Self::maybe_deserialize` can be used to deserialize to `Option<SerdeDFA>`.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct SerdeDFA<'data> {
    // Safety: These always represent a valid DFA (DFA::from_bytes(dfa_bytes).is_ok())
    dfa_bytes: Cow<'data, [u8]>,
    pattern: Option<Cow<'data, str>>,
}

#[cfg(feature = "datagen")]
impl PartialEq for SerdeDFA<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.dfa_bytes == other.dfa_bytes
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for SerdeDFA<'_> {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_list");
        let le_bytes = databake::Bake::bake(&self.deref().to_bytes_little_endian().as_slice(), env);
        let be_bytes = databake::Bake::bake(&self.deref().to_bytes_big_endian().as_slice(), env);
        // Safe because of `to_bytes_little_endian`/`to_bytes_big_endian`'s invariant.
        databake::quote! {
            unsafe {
                ::icu_list::provider::SerdeDFA::from_dfa_bytes_unchecked(
                    if cfg!(target_endian = "little") {
                        #le_bytes
                    } else {
                        #be_bytes
                    }
                )
            }
        }
    }
}

#[cfg(feature = "datagen")]
impl serde::Serialize for SerdeDFA<'_> {
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
                        "cannot serialize a deserialized bincode SerdeDFA to JSON",
                    ))
                })
        } else {
            self.deref().to_bytes_little_endian().serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'data> SerdeDFA<'data> {
    /// Deserializes to `Option<Self>`. Will return `None` for non-human-readable serialization
    /// formats on big-endian systems, as `regex_automata` serialization is endian-sensitive.
    pub fn maybe_deserialize<'de: 'data, D>(deserializer: D) -> Result<Option<Self>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use icu_provider::serde::borrow_de_utils::CowBytesWrap;
        use serde::Deserialize;

        #[cfg(feature = "serde_human")]
        if deserializer.is_human_readable() {
            #[cfg(not(feature = "std"))]
            use alloc::string::ToString;
            use serde::de::Error;
            return SerdeDFA::new(Cow::<str>::deserialize(deserializer)?)
                .map(Some)
                .map_err(|e| D::Error::custom(e.to_string()));
        }

        let dfa_bytes = <CowBytesWrap<'de>>::deserialize(deserializer)?.0;

        if cfg!(target_endian = "big") {
            return Ok(None);
        }

        // Verify safety invariant
        DFA::from_bytes(&dfa_bytes).map_err(|e| {
            use serde::de::Error;
            D::Error::custom(alloc::format!("Invalid DFA bytes: {e}"))
        })?;

        Ok(Some(SerdeDFA {
            dfa_bytes,
            pattern: None,
        }))
    }
}

impl<'data> SerdeDFA<'data> {
    /// Creates a `SerdeDFA` from raw bytes. Used internally by databake.
    ///
    /// # Safety
    ///
    /// `dfa_bytes` has to be a valid DFA (regex_automata::dfa::sparse::DFA::from_bytes(dfa_bytes).is_ok())
    pub const unsafe fn from_dfa_bytes_unchecked(dfa_bytes: &'data [u8]) -> Self {
        Self {
            dfa_bytes: Cow::Borrowed(dfa_bytes),
            pattern: None,
        }
    }

    /// Creates a `SerdeDFA` from a regex.
    #[cfg(any(feature = "datagen", feature = "serde_human",))]
    pub fn new(pattern: Cow<'data, str>) -> Result<Self, icu_provider::DataError> {
        use regex_automata::{
            dfa::dense::{Builder, Config},
            SyntaxConfig,
        };

        let mut builder = Builder::new();
        let dfa = builder
            .syntax(SyntaxConfig::new().case_insensitive(true))
            .configure(Config::new().anchored(true).minimize(true))
            .build(&pattern)
            .map_err(|_| {
                icu_provider::DataError::custom("Cannot build DFA").with_display_context(&pattern)
            })?
            .to_sparse()
            .map_err(|_| {
                icu_provider::DataError::custom("Cannot sparsify DFA")
                    .with_display_context(&pattern)
            })?;

        Ok(Self {
            dfa_bytes: dfa.to_bytes_native_endian().into(),
            pattern: Some(pattern),
        })
    }

    /// Returns the represented [`DFA`]
    #[allow(clippy::unwrap_used)] // by invariant
    pub fn deref(&'data self) -> DFA<&'data [u8]> {
        // Safe due to struct invariant.
        unsafe { DFA::from_bytes_unchecked(&self.dfa_bytes).unwrap().0 }
    }
}

#[cfg(all(test, feature = "datagen"))]
mod test {
    use super::*;

    #[test]
    fn test_serde_dfa() {
        use regex_automata::dfa::Automaton;

        let matcher = SerdeDFA::new(Cow::Borrowed("abc")).unwrap();

        assert!(matcher.deref().find_earliest_fwd(b"ab").unwrap().is_none());
        assert!(matcher.deref().find_earliest_fwd(b"abc").unwrap().is_some());
        assert!(matcher
            .deref()
            .find_earliest_fwd(b"abcde")
            .unwrap()
            .is_some());
        assert!(matcher
            .deref()
            .find_earliest_fwd(b" abcde")
            .unwrap()
            .is_none());
    }

    #[derive(serde::Deserialize)]
    struct OptionSerdeDFA<'data>(
        #[serde(borrow, deserialize_with = "SerdeDFA::maybe_deserialize")] Option<SerdeDFA<'data>>,
    );

    #[test]
    #[cfg(target_endian = "little")]
    fn test_postcard_serialization() {
        let matcher = SerdeDFA::new(Cow::Borrowed("abc*")).unwrap();

        let mut bytes = postcard::to_stdvec(&matcher).unwrap();
        assert_eq!(
            postcard::from_bytes::<OptionSerdeDFA>(&bytes).unwrap().0,
            Some(matcher)
        );

        // A corrupted byte leads to an error
        bytes[17] ^= 255;
        assert!(postcard::from_bytes::<OptionSerdeDFA>(&bytes).is_err());
        bytes[17] ^= 255;

        // An extra byte leads to an error
        bytes.insert(123, 40);
        assert!(postcard::from_bytes::<OptionSerdeDFA>(&bytes).is_err());
        bytes.remove(123);

        // Missing bytes lead to an error
        assert!(postcard::from_bytes::<OptionSerdeDFA>(&bytes[0..bytes.len() - 5]).is_err());
    }

    #[test]
    #[cfg(feature = "serde_human")]
    fn test_json_serialization() {
        let matcher = SerdeDFA::new(Cow::Borrowed("abc*")).unwrap();

        let json = serde_json::to_string(&matcher).unwrap();
        assert_eq!(
            serde_json::from_str::<OptionSerdeDFA>(&json).unwrap().0,
            Some(matcher)
        );
        assert!(serde_json::from_str::<OptionSerdeDFA>(".*[").is_err());
    }

    #[test]
    #[ignore] // https://github.com/rust-lang/rust/issues/98906
    fn databake() {
        databake::test_bake!(
            SerdeDFA,
            const: unsafe { crate::provider::SerdeDFA::from_dfa_bytes_unchecked(if cfg!(target_endian = "little") {
                b"foo" // TODO: set this when activating the test
            } else {
                b"bar" // TODO: set this when activating the test
            })},
            icu_list
        );
    }
}
