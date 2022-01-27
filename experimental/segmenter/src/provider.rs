// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::property_table::UAX14_PROPERTY_TABLE;
use crate::rule_table::UAX14_RULE_TABLE;
use alloc::boxed::Box;
use core::ops::Deref;
use icu_provider::yoke::{self, *};
use zerovec::ZeroSlice;
use zerovec::ZeroVec;

pub mod key {
    //! Resource keys for [`icu_segmenter`](crate).
    use icu_provider::{resource_key, ResourceKey};

    /// Resource key: data for line breaking.
    pub const LINE_BREAK_DATA_V1: ResourceKey = resource_key!("segmenter/line@1");
}

/// Pre-processed Unicode data in the form of tables to be used for line breaking.
#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LineBreakDataV1<'data> {
    /// Property table for line breaking.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub property_table: LineBreakPropertyTable<'data>,

    /// Rule table for line breaking.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub rule_table: LineBreakRuleTable<'data>,
}

/// Property table for line breaking.
#[derive(Debug, PartialEq, Clone, Yokeable)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum LineBreakPropertyTable<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    Borrowed(&'data [[u8; 1024]; 128]),
    Owned(Box<[[u8; 1024]; 128]>),
}

impl Deref for LineBreakPropertyTable<'_> {
    type Target = [[u8; 1024]; 128];
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(v) => v,
            Self::Owned(v) => v,
        }
    }
}

impl<'zcf> ZeroCopyFrom<'zcf, LineBreakPropertyTable<'_>> for LineBreakPropertyTable<'zcf> {
    fn zero_copy_from(cart: &'zcf LineBreakPropertyTable<'_>) -> Self {
        LineBreakPropertyTable::Borrowed(&*cart)
    }
}

impl Default for LineBreakPropertyTable<'static> {
    fn default() -> Self {
        LineBreakPropertyTable::Borrowed(&UAX14_PROPERTY_TABLE)
    }
}

/// Rule table for line breaking.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LineBreakRuleTable<'data> {
    /// Matrix of rules.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub table_data: ZeroVec<'data, i8>,
    /// Number of properties; should be the square root of the length of [`Self::table_data`].
    pub property_count: u8,
}

impl Default for LineBreakRuleTable<'static> {
    fn default() -> Self {
        Self {
            table_data: ZeroSlice::from_ule_slice(&UAX14_RULE_TABLE).as_zerovec(),
            property_count: crate::lb_define::PROP_COUNT as u8,
        }
    }
}
