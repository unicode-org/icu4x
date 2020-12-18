// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_provider::structs::likelysubtags::*;

pub mod locale_canonicalizer;

#[derive(Debug)]
pub enum LocaleCanonicalizerError {
    NotMatched,
}

pub struct LocaleCanonicalizer {
    likely_subtags: LikelySubtagsV1,
}
