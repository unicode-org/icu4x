// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{reference, PatternError, PatternItem, TimeGranularity};
use alloc::{fmt, vec::Vec};
use core::str::FromStr;
use icu_provider::{yoke, zerofrom};
use zerovec::ZeroVec;

#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[allow(clippy::exhaustive_structs)] // part of data struct
pub struct Pattern<'data> {
    pub items: ZeroVec<'data, PatternItem>,
    /// This field should contain the smallest time unit from the `items` vec.
    /// If it doesn't, unexpected results for day periods may be encountered.
    pub time_granularity_unchecked: TimeGranularity,
}

impl<'data> Pattern<'data> {
    pub fn into_owned(self) -> Pattern<'static> {
        Pattern {
            items: self.items.into_owned(),
            time_granularity_unchecked: self.time_granularity_unchecked,
        }
    }
}

impl From<Vec<PatternItem>> for Pattern<'_> {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            time_granularity_unchecked: items.iter().map(Into::into).max().unwrap_or_default(),
            items: ZeroVec::alloc_from_slice(&items),
        }
    }
}

impl From<&reference::Pattern> for Pattern<'_> {
    fn from(input: &reference::Pattern) -> Self {
        Self {
            items: ZeroVec::alloc_from_slice(&input.items),
            time_granularity_unchecked: input.time_granularity,
        }
    }
}

impl From<&Pattern<'_>> for reference::Pattern {
    fn from(input: &Pattern<'_>) -> Self {
        Self {
            items: input.items.to_vec(),
            time_granularity: input.time_granularity_unchecked,
        }
    }
}

impl FromStr for Pattern<'_> {
    type Err = PatternError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reference = crate::pattern::reference::Pattern::from_str(input)?;
        Ok(Self::from(&reference))
    }
}

impl Default for Pattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::Owned(Vec::new()),
            time_granularity_unchecked: TimeGranularity::default(),
        }
    }
}

impl fmt::Display for Pattern<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let reference = crate::pattern::reference::Pattern::from(self);
        reference.fmt(formatter)
    }
}
