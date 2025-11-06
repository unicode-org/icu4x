// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Structs for date / time patterns that store data corresponding to pattern lengths
//! and/or plural forms.

use icu_provider::prelude::*;

use crate::provider::pattern::runtime;

/// An enum containing four lengths (full, long, medium, short) for interfacing
/// with [`GenericLengthPatterns`]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug)]
pub enum FullLongMediumShort {
    /// "full" length
    Full,
    /// "long" length
    Long,
    /// "medium" length
    Medium,
    /// "short" length
    Short,
}

/// A struct containing `dateTimePatterns` aka "glue patterns".
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct GenericLengthPatterns<'data> {
    /// A full length glue pattern of other formatted elements.
    pub full: runtime::GenericPattern<'data>,
    /// A long length glue pattern of other formatted elements.
    pub long: runtime::GenericPattern<'data>,
    /// A medium length glue pattern of other formatted elements.
    pub medium: runtime::GenericPattern<'data>,
    /// A short length glue pattern of other formatted elements.
    pub short: runtime::GenericPattern<'data>,
}
