// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedDateTime;
use crate::provider::pattern::PatternItem;
use crate::provider::semantic_skeletons::GluePattern;
use core::fmt;
use writeable::{PartsWrite, Writeable};
use zerovec::ule::AsULE;

/// The formatting result of a date/time range.
#[derive(Debug)]
pub struct FormattedDateRange<'l> {
    pub(crate) inner: FormattedDateRangeInner<'l>,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)] // Short-lived formatting result, avoid heap allocation
#[allow(dead_code, reason = "https://github.com/unicode-org/icu4x/issues/5448")]
pub(crate) enum FormattedDateRangeInner<'l> {
    /// The range resolved to a single date/time (start and end are equal).
    Single(FormattedDateTime<'l>),
    /// The range is formatted using a greatest difference pattern.
    GreatestDifference(FormattedGreatestDifference<'l>),
    /// The range has a single date but a range of times.
    TimeRangeMixed(FormattedTimeRangeMixed<'l>),
    /// The range is formatted using the fallback range pattern.
    Fallback(FormattedRangeFallback<'l>),
}

impl Writeable for FormattedDateRange<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        match &self.inner {
            FormattedDateRangeInner::Single(x) => x.write_to_parts(sink),
            FormattedDateRangeInner::GreatestDifference(x) => x.write_to_parts(sink),
            FormattedDateRangeInner::TimeRangeMixed(x) => x.write_to_parts(sink),
            FormattedDateRangeInner::Fallback(x) => x.write_to_parts(sink),
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        match &self.inner {
            FormattedDateRangeInner::Single(x) => x.writeable_length_hint(),
            FormattedDateRangeInner::GreatestDifference(x) => x.writeable_length_hint(),
            FormattedDateRangeInner::TimeRangeMixed(x) => x.writeable_length_hint(),
            FormattedDateRangeInner::Fallback(x) => x.writeable_length_hint(),
        }
    }
}

writeable::impl_display_with_writeable!(FormattedDateRange<'_>);

/// The formatting result of a date/time range where the start and end dates
/// differ by a specific field (e.g., day, month, year) and are formatted
/// using a greatest-difference pattern.
#[derive(Debug)]
pub(crate) struct FormattedGreatestDifference<'l> {
    pub(crate) start: FormattedDateTime<'l>,
    #[allow(dead_code, reason = "https://github.com/unicode-org/icu4x/issues/5448")]
    pub(crate) end: FormattedDateTime<'l>,
}

impl Writeable for FormattedGreatestDifference<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        // TODO: Support greatest difference formatting (currently just writes the start date)
        self.start.write_to_parts(sink)
    }
}

/// The formatting result of a date/time range where the date is the same
/// but the time differs, formatted by joining the time range and the date
/// using a glue pattern.
#[derive(Debug)]
pub(crate) struct FormattedTimeRangeMixed<'l> {
    pub(crate) date: FormattedDateTime<'l>,
    pub(crate) time_range: FormattedGreatestDifference<'l>,
    pub(crate) glue: &'l GluePattern<'l>,
}

impl Writeable for FormattedTimeRangeMixed<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        write_glue_pattern(sink, self.glue, &self.time_range, &self.date)
    }
}

/// The formatting result of a date/time range formatted using the fallback
/// range pattern (gluing the fully formatted start and end datetimes).
#[derive(Debug)]
pub(crate) struct FormattedRangeFallback<'l> {
    pub(crate) start: FormattedDateTime<'l>,
    pub(crate) end: FormattedDateTime<'l>,
    pub(crate) glue: &'l GluePattern<'l>,
}

impl Writeable for FormattedRangeFallback<'_> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> Result<(), fmt::Error> {
        write_glue_pattern(sink, self.glue, &self.start, &self.end)
    }
}

pub(crate) fn write_glue_pattern<W, S, E>(
    sink: &mut W,
    glue: &GluePattern<'_>,
    start: &S,
    end: &E,
) -> fmt::Result
where
    W: PartsWrite + ?Sized,
    S: Writeable + ?Sized,
    E: Writeable + ?Sized,
{
    for generic_item_ule in glue.pattern.items.as_ule_slice().iter() {
        match generic_item_ule.as_pattern_item_ule() {
            Ok(pattern_item_ule) => {
                let pattern_item = <PatternItem as AsULE>::from_unaligned(*pattern_item_ule);
                if let PatternItem::Literal(ch) = pattern_item {
                    sink.write_char(ch)?;
                } else {
                    debug_assert!(false, "Expected only literals in glue pattern");
                }
            }
            Err(0) => {
                start.write_to_parts(sink)?;
            }
            Err(1) => {
                end.write_to_parts(sink)?;
            }
            Err(_) => {
                debug_assert!(false, "Unexpected placeholder index in glue pattern");
            }
        }
    }
    Ok(())
}
