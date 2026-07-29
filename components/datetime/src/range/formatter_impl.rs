// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedDateTime;
use crate::format::DateTimeInputUnchecked;
use crate::pattern::RawDateTimeNamesBorrowed;
use crate::provider::range_patterns::RangePatternInfoBorrowed;
use crate::range::difference::{Difference, resolve_difference};
use crate::range::write::{
    FormattedDateRange, FormattedDateRangeInner, FormattedGreatestDifference,
    FormattedRangeFallback, FormattedTimeRangeMixed,
};
use crate::raw::neo::{
    DateTimeZonePatternDataBorrowed, DateTimeZonePatternSelectionData,
    DateTimeZoneRangePatternSelectionData,
};

/// An internal struct containing the types from "core" DateTimeFormatter/FixedCalendarDateTimeFormatter
/// needed for range formatting.
///
/// This allows shared range formatting infrastructure to work over both
/// `FixedCalendarDateTimeFormatter` and `DateTimeFormatter`.
#[derive(Copy, Clone)]
pub(crate) struct RangeFormatterCore<'a> {
    pub(crate) names: RawDateTimeNamesBorrowed<'a>,
    pub(crate) selection: &'a DateTimeZonePatternSelectionData,
}

/// Formats a full datetime using the core's datetime pattern.
fn format_datetime<'a>(
    core: RangeFormatterCore<'a>,
    input: &DateTimeInputUnchecked,
) -> FormattedDateTime<'a> {
    FormattedDateTime {
        pattern: core.selection.select(input),
        input: *input,
        names: core.names,
    }
}

/// Formats a date-only pattern (time and zone fields removed).
fn format_date_only<'a>(
    core: RangeFormatterCore<'a>,
    input: &DateTimeInputUnchecked,
) -> FormattedDateTime<'a> {
    let pattern = DateTimeZonePatternDataBorrowed {
        date: core.selection.date.select(input, core.selection.options),
        time: None,
        zone: None,
        glue: None,
    };
    FormattedDateTime {
        pattern,
        input: *input,
        names: core.names,
    }
}

/// Formats a time-only pattern (date and zone fields removed).
fn format_time_only<'a>(
    core: RangeFormatterCore<'a>,
    input: &DateTimeInputUnchecked,
) -> FormattedDateTime<'a> {
    let pattern = DateTimeZonePatternDataBorrowed {
        date: None,
        time: core
            .selection
            .time
            .select(input, core.selection.options, core.selection.prefs),
        zone: None,
        glue: None,
    };
    FormattedDateTime {
        pattern,
        input: *input,
        names: core.names,
    }
}

/// Helper function to construct a `FormattedGreatestDifference` with shared parameters.
fn make_greatest_difference<'a>(
    core: RangeFormatterCore<'a>,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: FormattedDateTime<'a>,
    end: FormattedDateTime<'a>,
    pattern_info: RangePatternInfoBorrowed<'a>,
) -> FormattedGreatestDifference<'a> {
    FormattedGreatestDifference {
        start,
        end,
        pattern_info,
        glue: range_selection.range_glue.get(),
        alignment: core.selection.options.alignment,
    }
}

/// Helper to format a greatest difference range when both sides use the full datetime pattern.
///
/// This is used for Case 2 (standard date-only or time-only ranges) where the range
/// is formatted by displaying the differing field in a range pattern, and other fields
/// are shared.
fn format_greatest_difference<'a>(
    core: RangeFormatterCore<'a>,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: &DateTimeInputUnchecked,
    end: &DateTimeInputUnchecked,
    diff: Difference,
    use_time: bool,
) -> Option<FormattedDateRangeInner<'a>> {
    let pattern_info = if use_time {
        range_selection
            .time_range
            .select(start, end, core.selection.options, diff)?
    } else {
        range_selection
            .date_range
            .select(start, core.selection.options, diff)?
    };

    let formatted = make_greatest_difference(
        core,
        range_selection,
        format_datetime(core, start),
        format_datetime(core, end),
        pattern_info,
    );
    Some(FormattedDateRangeInner::GreatestDifference(formatted))
}

/// The shared implementation of range formatting, used by both `DateTimeFormatter`
/// and `FixedCalendarDateTimeFormatter`.
///
/// This function orchestrates the formatting by:
/// 1. Resolving the greatest difference between the start and end dates.
/// 2. Selecting the appropriate pattern (date range, time range, or fallback).
/// 3. Formatting the sides and wrapping them in the appropriate result type.
pub(crate) fn format_impl<'a>(
    core: RangeFormatterCore<'a>,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: &DateTimeInputUnchecked,
    end: &DateTimeInputUnchecked,
) -> FormattedDateRange<'a> {
    let dayperiods = core.names.dayperiod_names();

    // 1. Resolve difference
    let diff = resolve_difference(start, end, dayperiods);

    let is_mixed = range_selection.date_range.payload.is_payload()
        && range_selection.time_range.payload.is_payload();

    // Early fallback for mixed date-time formatter with date difference.
    // UTS 35: If date differs in a mixed skeleton, fall back to range fallback (Case 4).
    if is_mixed && diff.is_date_diff() {
        return FormattedDateRange(format_fallback(core, range_selection, start, end));
    }

    // 2. Select pattern and format
    let inner = match diff {
        Difference::None => FormattedDateRangeInner::Single(format_datetime(core, start)),
        Difference::Incomparable | Difference::Second => {
            format_fallback(core, range_selection, start, end)
        }
        diff => {
            if is_mixed {
                // Case 3: Mixed range, only time differs (date diff was handled by early fallback).
                let date_formatted = format_date_only(core, start);
                let time_range_formatted = range_selection
                    .time_range
                    .select(start, end, core.selection.options, diff)
                    .map(|pattern_info| {
                        make_greatest_difference(
                            core,
                            range_selection,
                            format_time_only(core, start),
                            format_time_only(core, end),
                            pattern_info,
                        )
                    });

                let core_glue = core.selection.glue.as_ref().map(|g| g.get());
                if let (Some(time_range), Some(glue)) = (time_range_formatted, core_glue) {
                    FormattedDateRangeInner::TimeRangeMixed(FormattedTimeRangeMixed {
                        date: date_formatted,
                        time_range,
                        glue,
                    })
                } else {
                    format_fallback(core, range_selection, start, end)
                }
            } else {
                // Case 2: Time-only or Date-only range.
                let use_time = diff.is_time_diff();
                format_greatest_difference(core, range_selection, start, end, diff, use_time)
                    .unwrap_or_else(|| format_fallback(core, range_selection, start, end))
            }
        }
    };
    FormattedDateRange(inner)
}

/// Formats the range using the fallback range pattern (gluing the fully formatted
/// start and end datetimes together).
///
/// This is used as a final fallback when no specific greatest difference pattern
/// is available, or when the difference requires a full fallback (Case 4).
fn format_fallback<'a>(
    core: RangeFormatterCore<'a>,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: &DateTimeInputUnchecked,
    end: &DateTimeInputUnchecked,
) -> FormattedDateRangeInner<'a> {
    let start_formatted = format_datetime(core, start);
    let end_formatted = format_datetime(core, end);
    let glue = range_selection.range_glue.get();
    FormattedDateRangeInner::Fallback(FormattedRangeFallback {
        start: start_formatted,
        end: end_formatted,
        glue,
    })
}
