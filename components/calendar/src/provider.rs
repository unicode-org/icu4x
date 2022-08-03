// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

#![allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]

use core::str::FromStr;
use icu_provider::{yoke, zerofrom};
use tinystr::TinyStr16;
use zerovec::ZeroVec;

/// The date at which an era started
///
/// The order of fields in this struct is important!
#[zerovec::make_ule(EraStartDateULE)]
#[derive(
    Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, yoke::Yokeable, zerofrom::ZeroFrom,
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct EraStartDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[icu_provider::data_struct(
    marker(JapaneseErasV1Marker, "calendar/japanese@1"),
    marker(JapaneseExtendedErasV1Marker, "calendar/japanext@1")
)]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_calendar::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct JapaneseErasV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dates_to_eras: ZeroVec<'data, (EraStartDate, TinyStr16)>,
}

impl FromStr for EraStartDate {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, ()> {
        let mut sign = 1;
        #[allow(clippy::indexing_slicing)]
        if s.starts_with('-') {
            // TODO(#1668) Clippy exceptions need docs or fixing.
            s = &s[1..];
            sign = -1;
        }

        let mut split = s.split('-');
        let mut year: i32 = split.next().ok_or(())?.parse().map_err(|_| ())?;
        year *= sign;
        let month: u8 = split.next().ok_or(())?.parse().map_err(|_| ())?;
        let day: u8 = split.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(EraStartDate { year, month, day })
    }
}
