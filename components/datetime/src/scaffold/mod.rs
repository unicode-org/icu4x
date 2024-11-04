// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Scaffolding traits and types for the datetime crate.
//!
//! Items in this module are mostly for trait bounds. Most users should not need to reference
//! these items in userland code.

mod calendar;
mod fieldset_traits;
mod get_field;

pub(crate) use calendar::AnyCalendarProvider;
pub use calendar::CalMarkers;
pub use calendar::CldrCalendar;
pub use calendar::ConvertCalendar;
pub use calendar::FullDataCalMarkers;
pub use calendar::IsAnyCalendarKind;
pub use calendar::IsInCalendar;
pub use calendar::NoDataCalMarkers;

pub(crate) use fieldset_traits::datetime_marker_helper;
pub use fieldset_traits::AllAnyCalendarExternalDataMarkers;
pub use fieldset_traits::AllAnyCalendarFormattingDataMarkers;
pub use fieldset_traits::AllFixedCalendarExternalDataMarkers;
pub use fieldset_traits::AllFixedCalendarFormattingDataMarkers;
pub use fieldset_traits::AllInputMarkers;
pub use fieldset_traits::DateDataMarkers;
pub use fieldset_traits::DateInputMarkers;
pub use fieldset_traits::DateTimeMarkers;
pub use fieldset_traits::HasConstComponents;
pub use fieldset_traits::HasConstDateComponents;
pub use fieldset_traits::HasConstTimeComponents;
pub use fieldset_traits::HasConstZoneComponent;
pub use fieldset_traits::IsRuntimeComponents;
pub use fieldset_traits::NeoNeverMarker;
pub use fieldset_traits::TimeMarkers;
pub use fieldset_traits::TypedDateDataMarkers;
pub use fieldset_traits::ZoneMarkers;

pub(crate) use get_field::impl_get_field;
pub use get_field::GetField;

/// Trait marking other traits that are considered unstable and should not generally be
/// implemented outside of the datetime crate.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland.
/// </div>
pub trait UnstableSealed {}
