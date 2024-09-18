// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_calendar::any_calendar::AnyCalendarKind;

/// An error from mixing calendar types in a formatter.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[displaydoc("DateTimeFormatter for {this_kind} calendar was given a {date_kind:?} calendar")]
#[non_exhaustive]
pub struct MismatchedCalendarError {
    /// The calendar kind of the target object (formatter).
    pub this_kind: AnyCalendarKind,
    /// The calendar kind of the input object (date being formatted).
    /// Can be `None` if the input calendar was not specified.
    pub date_kind: Option<AnyCalendarKind>,
}
