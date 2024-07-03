// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::exhaustive_structs)] // part of data struct and internal API

use super::super::{reference, PatternError, PatternItem, TimeGranularity};
use alloc::vec::Vec;
use core::str::FromStr;
use icu_provider::prelude::*;
use zerovec::{ule::AsULE, ZeroSlice, ZeroVec};

#[derive(Debug, PartialEq, Eq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_datetime::pattern::runtime),
)]
#[zerovec::make_varule(PatternULE)]
#[zerovec::derive(Debug)]
#[zerovec::skip_derive(Ord)]
#[cfg_attr(feature = "serde", zerovec::derive(Deserialize))]
#[cfg_attr(feature = "datagen", zerovec::derive(Serialize))]
pub struct Pattern<'data> {
    pub items: ZeroVec<'data, PatternItem>,
    /// This field should contain the smallest time unit from the `items` vec.
    /// If it doesn't, unexpected results for day periods may be encountered.
    pub metadata: PatternMetadata,
}

#[derive(Debug, Copy, Clone)]
pub struct PatternBorrowed<'data> {
    pub items: &'data ZeroSlice<PatternItem>,
    pub metadata: PatternMetadata,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[zerovec::make_ule(PatternMetadataULE)]
#[zerovec::skip_derive(Ord)]
pub struct PatternMetadata(u8);

impl PatternMetadata {
    pub(crate) const DEFAULT: PatternMetadata = Self::from_time_granularity(TimeGranularity::None);

    #[inline]
    pub(crate) fn time_granularity(self) -> TimeGranularity {
        TimeGranularity::from_ordinal(self.0)
    }

    pub(crate) fn from_items(items: &[PatternItem]) -> Self {
        let time_granularity: TimeGranularity =
            items.iter().map(Into::into).max().unwrap_or_default();
        Self::from_time_granularity(time_granularity)
    }

    /// Merges the metadata from a date pattern and a time pattern into one.
    #[cfg(feature = "experimental")]
    #[inline]
    pub(crate) fn merge_date_and_time_metadata(
        _date: PatternMetadata,
        time: PatternMetadata,
    ) -> PatternMetadata {
        // Currently we only have time granularity so we ignore the date metadata.
        time
    }

    #[inline]
    #[doc(hidden)] // databake
    pub const fn from_time_granularity(time_granularity: TimeGranularity) -> Self {
        Self(time_granularity.ordinal())
    }

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    #[inline]
    pub(crate) fn set_time_granularity(&mut self, time_granularity: TimeGranularity) {
        self.0 = time_granularity.ordinal();
    }
}

impl Default for PatternMetadata {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<'data> Pattern<'data> {
    pub fn into_owned(self) -> Pattern<'static> {
        Pattern {
            items: self.items.into_owned(),
            metadata: self.metadata,
        }
    }

    pub fn as_borrowed(&self) -> PatternBorrowed {
        PatternBorrowed {
            items: &self.items,
            metadata: self.metadata,
        }
    }
}

impl PatternULE {
    pub fn as_borrowed(&self) -> PatternBorrowed {
        PatternBorrowed {
            items: &self.items,
            metadata: PatternMetadata::from_unaligned(self.metadata),
        }
    }
}

impl<'data> PatternBorrowed<'data> {
    #[cfg(feature = "experimental")]
    pub(crate) const DEFAULT: PatternBorrowed<'static> = PatternBorrowed {
        items: ZeroSlice::new_empty(),
        metadata: PatternMetadata::DEFAULT,
    };

    pub fn as_pattern(&self) -> Pattern<'data> {
        Pattern {
            items: self.items.as_zerovec(),
            metadata: self.metadata,
        }
    }
}

impl From<Vec<PatternItem>> for Pattern<'_> {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            metadata: PatternMetadata::from_items(&items),
            items: ZeroVec::alloc_from_slice(&items),
        }
    }
}

impl From<&reference::Pattern> for Pattern<'_> {
    fn from(input: &reference::Pattern) -> Self {
        Self {
            items: ZeroVec::alloc_from_slice(&input.items),
            metadata: PatternMetadata::from_time_granularity(input.time_granularity),
        }
    }
}

impl From<&Pattern<'_>> for reference::Pattern {
    fn from(input: &Pattern<'_>) -> Self {
        Self {
            items: input.items.to_vec(),
            time_granularity: input.metadata.time_granularity(),
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
            items: ZeroVec::new(),
            metadata: PatternMetadata::default(),
        }
    }
}

#[cfg(feature = "datagen")]
impl core::fmt::Display for Pattern<'_> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        let reference = crate::pattern::reference::Pattern::from(self);
        reference.fmt(formatter)
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for PatternMetadata {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("icu_datetime");
        let time_granularity = databake::Bake::bake(&self.time_granularity(), ctx);
        databake::quote! {
            icu_datetime::pattern::runtime::PatternMetadata::from_time_granularity(#time_granularity)
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::BakeSize for PatternMetadata {
    fn borrows_size(&self) -> usize {
        0
    }
}

#[test]
#[cfg(feature = "datagen")]
fn databake() {
    databake::test_bake!(
        PatternMetadata,
        const,
        crate::pattern::runtime::PatternMetadata::from_time_granularity(
            crate::pattern::TimeGranularity::Hours
        ),
        icu_datetime,
    );
}
