// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
use crate::property_table::UAX14_PROPERTY_TABLE;
use crate::rule_table::UAX14_RULE_TABLE;
use zerovec::ZeroVec;
use zerovec::ZeroSlice;
use alloc::boxed::Box;
use core::ops::Deref;

pub mod key {
    //! Resource keys for [`icu_decimal`](crate).
    use icu_provider::{resource_key, ResourceKey};

    /// Resource key: symbols used for basic decimal formatting.
    pub const LINE_BREAK_DATA_V1: ResourceKey = resource_key!(Segmenter, "line", 1);
}

/// Pre-processed Unicode data in the form of tables to be used for line breaking.
#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LineBreakDataV1<'data> {
    /// String to prepend before the decimal number.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub property_table: LineBreakPropertyTable<'data>,

    /// String to append after the decimal number.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub rule_table: ZeroVec<'data, i8>,
}

#[derive(Debug, PartialEq, Clone, Yokeable)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub(crate) enum LineBreakPropertyTableInner<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    Borrowed(&'data [[u8; 1024]; 128]),
    Owned(Box<[[u8; 1024]; 128]>),
}

impl Deref for LineBreakPropertyTableInner<'_> {
    type Target = [[u8; 1024]; 128];
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => &v,
        }
    }
}

impl<'a> ZeroCopyFrom<LineBreakPropertyTableInner<'a>> for LineBreakPropertyTableInner<'static> {
    fn zero_copy_from<'b>(cart: &'b LineBreakPropertyTableInner<'a>) -> <Self as Yokeable<'b>>::Output {
        LineBreakPropertyTableInner::Borrowed(&*cart)
    }
}

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LineBreakPropertyTable<'data> {
    pub(crate) table: LineBreakPropertyTableInner<'data>,
}

impl Default for LineBreakDataV1<'static> {
    fn default() -> Self {
        let property_table = LineBreakPropertyTable {
            table: LineBreakPropertyTableInner::Borrowed(&UAX14_PROPERTY_TABLE)
        };
        let rule_table = ZeroSlice::from_ule_slice(&UAX14_RULE_TABLE).as_zerovec();
        Self {
            property_table,
            rule_table,
        }
    }
}
