// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Provider structs for digital data.

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use core::str::FromStr;

use displaydoc::Display;
use icu_provider::prelude::*;

#[icu_provider::data_struct(DigitalDurationDataV1Marker = "duration/digital@1")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::duration::provider)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]

/// A struct containing digital duration data (durationUnit-type-* patterns).
pub struct DigitalDurationDataV1<'data> {
    /// The separator between the hour, minute, and second fields.
    #[cfg_attr(feature = "serde", serde(borrow))]
    separator: Cow<str, 'data>,

    /// The number of digits to pad the hour field with.
    hour_padding: u8,
}

/// Unknown time pattern: {0}
#[derive(Debug, Display)]
pub struct UnknownPatternError(String);

impl<'data> FromStr for DigitalDurationDataV1<'data> {
    type Err = UnknownPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hh:mm:ss" => Ok(DigitalDurationDataV1 {
                separator: ":".into(),
                hour_padding: 2,
            }),
            "h:mm:ss" => Ok(DigitalDurationDataV1 {
                separator: ":".into(),
                hour_padding: 1,
            }),
            "h.mm.ss" => Ok(DigitalDurationDataV1 {
                separator: ".".into(),
                hour_padding: 1,
            }),
            "h,mm,ss" => Ok(DigitalDurationDataV1 {
                separator: ",".into(),
                hour_padding: 1,
            }),
            _ => Err(UnknownPatternError(s.to_string())),
        }
    }
}
