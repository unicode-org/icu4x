// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// An implementor of [`crate::core::ParseInput`] that reads bytes from files.
pub mod file;
/// An implementor of [`crate::core::ParseInput`] that reads bytes from slices.
pub mod slice;
