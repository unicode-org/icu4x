// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::export::DeduplicationStrategy;
use crate::prelude::*;
use alloc::borrow::Cow;

use crate as icu_provider;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
#[crate::data_struct(marker(SourceInfoMarker, "_sourceinfo@1", singleton))]
#[cfg_attr(feature = "export", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "export", databake(path = icu_provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum SourceInfo<'data> {
    V001 {
        deduplication: DeduplicationStrategy,
        lifetime_holder: Cow<'data, ()>,
    },
}

impl SourceInfo<'_> {
    /// Get the deduplication strategy that the provider was exported with.
    pub fn deduplication(&self) -> DeduplicationStrategy {
        let Self::V001 { deduplication, .. } = self;
        *deduplication
    }
}
