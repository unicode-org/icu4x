// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_provider::structs::likelysubtags::*;
use std::borrow::Cow;

pub mod locale_canonicalizer;

#[derive(Debug, PartialEq)]
pub enum CanonicalizationResult {
    Modified,
    Unmodified,
}

pub struct LocaleCanonicalizer<'a> {
    likely_subtags: Cow<'a, LikelySubtagsV1>,
}
