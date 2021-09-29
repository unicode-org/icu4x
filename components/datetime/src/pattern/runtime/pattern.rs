// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{reference, PatternItem, TimeGranularity};
use alloc::vec;
use zerovec::ZeroVec;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Pattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub items: ZeroVec<'data, PatternItem>,
    pub(crate) time_granularity: TimeGranularity,
}

impl<'data> From<&'data reference::Pattern> for Pattern<'data> {
    fn from(input: &'data reference::Pattern) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&input.items),
            time_granularity: input.time_granularity,
        }
    }
}

impl From<reference::Pattern> for Pattern<'_> {
    fn from(input: reference::Pattern) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&input.items),
            time_granularity: input.time_granularity,
        }
    }
}

impl Default for Pattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::Owned(vec![]),
            time_granularity: TimeGranularity::default(),
        }
    }
}
