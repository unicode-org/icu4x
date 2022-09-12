// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{super::MixedPatternItem, super::PatternError, Parser};
use alloc::vec::Vec;
use core::str::FromStr;

#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct MixedPattern {
    pub items: Vec<MixedPatternItem>,
}

impl From<Vec<MixedPatternItem>> for MixedPattern {
    fn from(items: Vec<MixedPatternItem>) -> Self {
        Self { items }
    }
}

impl FromStr for MixedPattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(s).parse_mixed().map(Self::from)
    }
}
