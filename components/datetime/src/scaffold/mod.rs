// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Scaffolding traits and types for the datetime crate.
//!
//! Items in this module are mostly for trait bounds. Most users should not need to reference
//! these items in userland code.

mod calendar;

pub(crate) use calendar::AnyCalendarProvider;
pub use calendar::CalMarkers;
pub use calendar::CldrCalendar;
pub use calendar::FullDataCalMarkers;
pub use calendar::NoDataCalMarkers;

/// Trait marking other traits that are considered unstable and should not generally be
/// implemented outside of the datetime crate.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland.
/// </div>
pub trait UnstableSealed {}
