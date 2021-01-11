// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_provider::structs::likelysubtags::*;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Display};

pub mod locale_canonicalizer;

#[derive(Debug)]
pub enum LocaleCanonicalizerError {
    NotMatched,
}

impl Error for LocaleCanonicalizerError {}

impl Display for LocaleCanonicalizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::NotMatched => "Not matched",
        };
        f.write_str(value)
    }
}

pub struct LocaleCanonicalizer<'a> {
    likely_subtags: Cow<'a, LikelySubtagsV1>,
}
