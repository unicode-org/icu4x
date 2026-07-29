// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::DateTimeFormatter;
use crate::FixedCalendarDateTimeFormatter;
use crate::FormattedDateTime;
use crate::format::DateTimeInputUnchecked;
use crate::pattern::RawDateTimeNamesBorrowed;
use crate::provider::range_patterns::RangePatternInfoBorrowed;
use crate::provider::semantic_skeletons::GluePattern;
use crate::range::difference::{Difference, resolve_difference};
use crate::range::write::{
    FormattedDateRange, FormattedDateRangeInner, FormattedGreatestDifference,
    FormattedRangeFallback, FormattedTimeRangeMixed,
};
use crate::raw::neo::{
    DateTimeZonePatternDataBorrowed, DateTimeZoneRangePatternSelectionData, RawOptions,
};
use crate::scaffold::{CldrCalendar, DateTimeNamesMarker};

/// An internal trait that abstracts the core formatting capabilities needed for range formatting.
///
/// This allows shared range formatting infrastructure to work over both
/// `FixedCalendarDateTimeFormatter` and `DateTimeFormatter`.
pub(crate) trait RangeFormatterCore {
    /// Returns the borrowed datetime names.
    fn names(&self) -> RawDateTimeNamesBorrowed<'_>;
    /// Selects the pattern for a full datetime.
    fn select_datetime(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_>;
    /// Selects the pattern for date-only fields.
    fn select_date_only(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_>;
    /// Selects the pattern for time-only fields.
    fn select_time_only(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_>;
    /// Returns the raw formatting options.
    fn options(&self) -> RawOptions;
    /// Returns the shared datetime-time glue pattern, if available.
    fn glue(&self) -> Option<&GluePattern<'_>>;
}

impl<C: CldrCalendar, FSet: DateTimeNamesMarker> RangeFormatterCore
    for FixedCalendarDateTimeFormatter<C, FSet>
{
    fn names(&self) -> RawDateTimeNamesBorrowed<'_> {
        self.names.as_borrowed()
    }
    fn select_datetime(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        self.selection.select(input)
    }
    fn select_date_only(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        DateTimeZonePatternDataBorrowed {
            date: self.selection.date.select(input, self.selection.options),
            time: None,
            zone: None,
            glue: None,
        }
    }
    fn select_time_only(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        DateTimeZonePatternDataBorrowed {
            date: None,
            time: self
                .selection
                .time
                .select(input, self.selection.options, self.selection.prefs),
            zone: None,
            glue: None,
        }
    }
    fn options(&self) -> RawOptions {
        self.selection.options
    }
    fn glue(&self) -> Option<&GluePattern<'_>> {
        self.selection.glue.as_ref().map(|g| g.get())
    }
}

impl<FSet: DateTimeNamesMarker> RangeFormatterCore for DateTimeFormatter<FSet> {
    fn names(&self) -> RawDateTimeNamesBorrowed<'_> {
        self.names.as_borrowed()
    }
    fn select_datetime(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        self.selection.select(input)
    }
    fn select_date_only(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        DateTimeZonePatternDataBorrowed {
            date: self.selection.date.select(input, self.selection.options),
            time: None,
            zone: None,
            glue: None,
        }
    }
    fn select_time_only(
        &self,
        input: &DateTimeInputUnchecked,
    ) -> DateTimeZonePatternDataBorrowed<'_> {
        DateTimeZonePatternDataBorrowed {
            date: None,
            time: self
                .selection
                .time
                .select(input, self.selection.options, self.selection.prefs),
            zone: None,
            glue: None,
        }
    }
    fn options(&self) -> RawOptions {
        self.selection.options
    }
    fn glue(&self) -> Option<&GluePattern<'_>> {
        self.selection.glue.as_ref().map(|g| g.get())
    }
}

/// Formats a full datetime using the core's datetime pattern.
fn format_datetime<'a>(
    core: &'a impl RangeFormatterCore,
    input: &DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
) -> FormattedDateTime<'a> {
    FormattedDateTime {
        pattern: core.select_datetime(input),
        input: *input,
        names,
    }
}

/// Formats a date-only pattern (time and zone fields removed).
fn format_date_only<'a>(
    core: &'a impl RangeFormatterCore,
    input: &DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
) -> FormattedDateTime<'a> {
    FormattedDateTime {
        pattern: core.select_date_only(input),
        input: *input,
        names,
    }
}

/// Formats a time-only pattern (date and zone fields removed).
fn format_time_only<'a>(
    core: &'a impl RangeFormatterCore,
    input: &DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
) -> FormattedDateTime<'a> {
    FormattedDateTime {
        pattern: core.select_time_only(input),
        input: *input,
        names,
    }
}

/// Helper function to construct a `FormattedGreatestDifference` with shared parameters.
fn make_greatest_difference<'a>(
    core: &'a impl RangeFormatterCore,
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
        alignment: core.options().alignment,
    }
}

/// Helper to format a greatest difference range when both sides use the full datetime pattern.
///
/// This is used for Case 2 (standard date-only or time-only ranges) where the range
/// is formatted by displaying the differing field in a range pattern, and other fields
/// are shared.
fn format_greatest_difference<'a>(
    core: &'a impl RangeFormatterCore,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: &DateTimeInputUnchecked,
    end: &DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
    diff: Difference,
    use_time: bool,
) -> Option<FormattedDateRangeInner<'a>> {
    let pattern_info = if use_time {
        range_selection
            .time_range
            .select(start, end, core.options(), diff)?
    } else {
        range_selection
            .date_range
            .select(start, core.options(), diff)?
    };

    let formatted = make_greatest_difference(
        core,
        range_selection,
        format_datetime(core, start, names),
        format_datetime(core, end, names),
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
pub(crate) fn format_impl_shared<'a>(
    core: &'a impl RangeFormatterCore,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: &DateTimeInputUnchecked,
    end: &DateTimeInputUnchecked,
) -> FormattedDateRange<'a> {
    let names = core.names();
    let dayperiods = names.dayperiod_names();

    // 1. Resolve difference
    let diff = resolve_difference(start, end, dayperiods);

    let is_mixed = range_selection.date_range.payload.is_payload()
        && range_selection.time_range.payload.is_payload();

    // Early fallback for mixed date-time formatter with date difference.
    // UTS 35: If date differs in a mixed skeleton, fall back to range fallback (Case 4).
    if is_mixed && diff.is_date_diff() {
        return FormattedDateRange(fallback_format_shared(
            core,
            range_selection,
            start,
            end,
            names,
        ));
    }

    // 2. Select pattern and format
    let inner = match diff {
        Difference::None => FormattedDateRangeInner::Single(format_datetime(core, start, names)),
        Difference::Incomparable | Difference::Second => {
            fallback_format_shared(core, range_selection, start, end, names)
        }
        diff => {
            if is_mixed {
                // Case 3: Mixed range, only time differs (date diff was handled by early fallback).
                let date_formatted = format_date_only(core, start, names);
                let time_range_formatted = range_selection
                    .time_range
                    .select(start, end, core.options(), diff)
                    .map(|pattern_info| {
                        make_greatest_difference(
                            core,
                            range_selection,
                            format_time_only(core, start, names),
                            format_time_only(core, end, names),
                            pattern_info,
                        )
                    });

                if let (Some(time_range), Some(glue)) = (time_range_formatted, core.glue()) {
                    FormattedDateRangeInner::TimeRangeMixed(FormattedTimeRangeMixed {
                        date: date_formatted,
                        time_range,
                        glue,
                    })
                } else {
                    fallback_format_shared(core, range_selection, start, end, names)
                }
            } else {
                // Case 2: Time-only or Date-only range.
                let use_time = diff.is_time_diff();
                format_greatest_difference(core, range_selection, start, end, names, diff, use_time)
                    .unwrap_or_else(|| {
                        fallback_format_shared(core, range_selection, start, end, names)
                    })
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
fn fallback_format_shared<'a>(
    core: &'a impl RangeFormatterCore,
    range_selection: &'a DateTimeZoneRangePatternSelectionData,
    start: &DateTimeInputUnchecked,
    end: &DateTimeInputUnchecked,
    names: RawDateTimeNamesBorrowed<'a>,
) -> FormattedDateRangeInner<'a> {
    let start_formatted = format_datetime(core, start, names);
    let end_formatted = format_datetime(core, end, names);
    let glue = range_selection.range_glue.get();
    FormattedDateRangeInner::Fallback(FormattedRangeFallback {
        start: start_formatted,
        end: end_formatted,
        glue,
    })
}
