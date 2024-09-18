// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

/// The time zone offset was invalid. Must be within ±18:00:00.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct InvalidOffsetError;

/// An error when the time zone was invalid or unknown.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct UnknownTimeZoneError;
