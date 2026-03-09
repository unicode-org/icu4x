// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Scaffolding traits and types for the datetime crate.
//!
//! Items in this module are mostly for trait bounds. Most users should not need to reference
//! these items in userland code.
//!
//! # Implementing a custom input type
//!
//! One reason you might find yourself in this module is because you have a custom datetime
//! type, and you want to plug it into ICU4X for formatting.
//!
//! To properly equip your type for ICU4X, implement the following traits:
//!
//! - [`UnstableSealed`] must always be implemented to acknowledge that these traits are
//!   subject to change, even across minor releases.
//! - [`GetField<()>`] must almost always be implemented. It is used as a placeholder for input fields that a particular fieldset does not require.
//! - Additional [`GetField`] impls based on what fields your type contains. For a list of all possible fields, see [`AllInputMarkers`].
//! - [`ConvertCalendar`], for [`DateTimeFormatter::format`](crate::DateTimeFormatter::format)
//! - [`InSameCalendar`], for [`DateTimeFormatter::format_same_calendar`](crate::DateTimeFormatter::format_same_calendar)
//! - [`InFixedCalendar`], for [`FixedCalendarDateTimeFormatter::format`](crate::FixedCalendarDateTimeFormatter::format)

mod calendar;
mod dynamic_impls;
mod fieldset_traits;
mod get_field;
mod names_storage;

pub use calendar::CalMarkers;
pub use calendar::CldrCalendar;
pub use calendar::ConvertCalendar;
pub(crate) use calendar::FormattableAnyCalendar;
pub(crate) use calendar::FormattableAnyCalendarNamesLoader;
pub use calendar::FormattableHijriRules;
pub use calendar::FullDataCalMarkers;
pub use calendar::InFixedCalendar;
pub use calendar::InSameCalendar;
pub use calendar::IntoFormattableAnyCalendar;
pub use calendar::NoDataCalMarkers;

pub(crate) use fieldset_traits::datetime_marker_helper;
pub use fieldset_traits::AllAnyCalendarExternalDataMarkers;
pub use fieldset_traits::AllAnyCalendarFormattingDataMarkers;
pub use fieldset_traits::AllAnyCalendarPatternDataMarkers;
pub use fieldset_traits::AllFixedCalendarExternalDataMarkers;
pub use fieldset_traits::AllFixedCalendarFormattingDataMarkers;
pub use fieldset_traits::AllFixedCalendarPatternDataMarkers;
pub use fieldset_traits::AllInputMarkers;
pub use fieldset_traits::DateDataMarkers;
pub use fieldset_traits::DateInputMarkers;
pub use fieldset_traits::DateTimeMarkers;
pub use fieldset_traits::TimeMarkers;
pub use fieldset_traits::TypedDateDataMarkers;
pub use fieldset_traits::ZoneMarkers;

pub use get_field::GetField;

pub use names_storage::DataPayloadWithVariables;
pub use names_storage::DataPayloadWithVariablesBorrowed;
pub use names_storage::DateTimeNamesFrom;
pub use names_storage::DateTimeNamesMarker;
pub use names_storage::MaybePayload;
pub use names_storage::MaybePayloadError;
pub use names_storage::NamesContainer;
pub(crate) use names_storage::OptionalNames;

/// Trait marking other traits that are considered unstable and should not generally be
/// implemented outside of the datetime crate.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break.
/// </div>
pub trait UnstableSealed {}
