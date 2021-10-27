// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(any(test, feature = "provider_transform_internals"))]
use crate::error::Error;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
use regex_automata::{SparseDFA, DFA};

#[derive(Clone, Debug, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Deserialize, serde::Serialize)
)]
pub(crate) struct StringMatcher<'data>(
    #[cfg_attr(feature = "provider_serde", serde(borrow))] Cow<'data, [u8]>,
);

impl<'data> StringMatcher<'data> {
    #[cfg(any(test, feature = "provider_transform_internals"))]
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

        // We have to decide on an endianness here. For now we use LE, and ignore
        // conditional patterns on BE systems. In the future the regex_automata
        // crate will make it easier to deserialize on different-endian systems:
        // https://github.com/BurntSushi/regex-automata/issues/20
        let bytes = sparse_dfa.to_bytes_little_endian();

        // We can unwrap because the u16 state size does not produce an error
        Ok(Self(Cow::Owned(bytes.unwrap())))
    }

    pub(crate) fn test(&self, string: &str) -> bool {
        cfg!(target_endian = "little")
            && unsafe { SparseDFA::<&[u8], u16>::from_bytes(&*self.0) }.find(string.as_bytes())
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
