// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

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
    /// The year the era started in
    pub year: i32,
    /// The month the era started in
    pub month: u8,
    /// The day the era started in
    pub day: u8,
}

/// A data structure containing the necessary era data for constructing a
/// [`Japanese`](crate::japanese::Japanese) calendar object
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
    /// A map from era start dates to their era codes
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub dates_to_eras: ZeroVec<'data, (EraStartDate, TinyStr16)>,
}

impl FromStr for EraStartDate {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, ()> {
        let sign = if let Some(suffix) = s.strip_prefix('-') {
            s = suffix;
            -1
        } else {
            1
        };

        let mut split = s.split('-');
        let year = split.next().ok_or(())?.parse::<i32>().map_err(|_| ())? * sign;
        let month = split.next().ok_or(())?.parse().map_err(|_| ())?;
        let day = split.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(EraStartDate { year, month, day })
    }
}
