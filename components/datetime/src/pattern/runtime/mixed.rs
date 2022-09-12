// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{fmt, str::FromStr};

use super::{
    super::MixedPatternItem,
    super::{reference, PatternError},
};
use icu_provider::{yoke, zerofrom};
use zerovec::ZeroVec;

#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_datetime::pattern::runtime),
)]
pub struct MixedPattern<'data> {
    pub items: ZeroVec<'data, MixedPatternItem>,
}

impl Default for MixedPattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::new(),
        }
    }
}

impl From<&reference::MixedPattern> for MixedPattern<'_> {
    fn from(input: &reference::MixedPattern) -> Self {
        Self {
            items: ZeroVec::alloc_from_slice(&input.items),
        }
    }
}

impl From<&MixedPattern<'_>> for reference::MixedPattern {
    fn from(input: &MixedPattern<'_>) -> Self {
        Self {
            items: input.items.to_vec(),
        }
    }
}

impl fmt::Display for MixedPattern<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let reference = reference::MixedPattern::from(self);
        reference.fmt(formatter)
    }
}

impl FromStr for MixedPattern<'_> {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reference = reference::MixedPattern::from_str(s)?;
        Ok(Self::from(&reference))
    }
}
