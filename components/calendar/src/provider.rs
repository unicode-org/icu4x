// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use core::str::FromStr;
use icu_provider::yoke::{self, *};
use litemap::LiteMap;
use tinystr::TinyStr16;

pub mod key {
    //! Resource keys for [`icu_decimal`](crate).
    use icu_provider::{resource_key, ResourceKey};

    /// Resource key: Japanese era data ([`JapaneseErasV1`](super::JapaneseErasV1))
    pub const JAPANESE_ERAS_V1: ResourceKey = resource_key!(Calendar, "japanese", 1);
}

/// The date at which an era started
///
/// The order of fields in this struct is important!
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct EraStartDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
// TODO (#1393) Make this zero-copy
pub struct JapaneseErasV1 {
    pub dates_to_historical_eras: LiteMap<EraStartDate, TinyStr16>,
    pub dates_to_eras: LiteMap<EraStartDate, TinyStr16>,
}

impl FromStr for EraStartDate {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, ()> {
        let mut sign = 1;
        if s.starts_with('-') {
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
