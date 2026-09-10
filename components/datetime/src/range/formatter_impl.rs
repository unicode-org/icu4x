// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedDateTime;
use crate::format::DateTimeInputUnchecked;
use crate::pattern::RawDateTimeNamesBorrowed;
use crate::provider::fields::FieldSymbol;
use crate::provider::pattern::PatternItem;
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

/// An internal struct containing the fields of `RangeFormatter` types,
/// splitting up the inner `DateTimeFormatter` fields as well.
///
/// This allows shared range formatting infrastructure to work over both
/// `FixedCalendarDateTimeFormatter` and `DateTimeFormatter`.
#[derive(Copy, Clone)]
pub(crate) struct RangeFormatterCore<'a> {
    pub(crate) names: RawDateTimeNamesBorrowed<'a>,
    pub(crate) selection: &'a DateTimeZonePatternSelectionData,
    pub(crate) range_selection: &'a DateTimeZoneRangePatternSelectionData,
}

impl<'a> RangeFormatterCore<'a> {
    /// Formats a full datetime using the core's datetime pattern.
    fn format_datetime(self, input: &DateTimeInputUnchecked) -> FormattedDateTime<'a> {
        FormattedDateTime {
            pattern: self.selection.select(input),
            input: *input,
            names: self.names,
        }
    }

    /// Formats a date-only pattern (time and zone fields removed).
    fn format_date_only(self, input: &DateTimeInputUnchecked) -> FormattedDateTime<'a> {
        let pattern = DateTimeZonePatternDataBorrowed {
            date: self.selection.date.select(input, self.selection.options),
            time: None,
            zone: None,
            glue: None,
        };
        FormattedDateTime {
            pattern,
            input: *input,
            names: self.names,
        }
    }
    /// Formats a time-only pattern (date and zone fields removed).
    fn format_time_only(self, input: &DateTimeInputUnchecked) -> FormattedDateTime<'a> {
        let pattern = DateTimeZonePatternDataBorrowed {
            date: None,
            time: self
                .selection
                .time
                .select(input, self.selection.options, self.selection.prefs),
            zone: None,
            glue: None,
        };
        FormattedDateTime {
            pattern,
            input: *input,
            names: self.names,
        }
    }
    /// Helper function to construct a `FormattedGreatestDifference` with shared parameters.
    fn make_greatest_difference(
        self,
        start: FormattedDateTime<'a>,
        end: FormattedDateTime<'a>,
        pattern_info: RangePatternInfoBorrowed<'a>,
    ) -> FormattedGreatestDifference<'a> {
        FormattedGreatestDifference {
            start,
            end,
            pattern_info,
            glue: self.range_selection.range_glue.get(),
            alignment: self.selection.options.alignment,
        }
    }

    /// Helper to format a greatest difference range when both sides use the full datetime pattern.
    ///
    /// This is used for Case 2 (standard date-only or time-only ranges) where the range
    /// is formatted by displaying the differing field in a range pattern, and other fields
    /// are shared.
    fn format_greatest_difference(
        self,
        start: &DateTimeInputUnchecked,
        end: &DateTimeInputUnchecked,
        diff: Difference,
        use_time: bool,
    ) -> Option<FormattedDateRangeInner<'a>> {
        let pattern_info = if use_time {
            self.range_selection
                .time_range
                .select(start, end, self.selection.options, diff)?
        } else {
            self.range_selection
                .date_range
                .select(start, self.selection.options, diff)?
        };

        let formatted = self.make_greatest_difference(
            self.format_datetime(start),
            self.format_datetime(end),
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
    pub(crate) fn format(
        self,
        start: &DateTimeInputUnchecked,
        end: &DateTimeInputUnchecked,
    ) -> FormattedDateRange<'a> {
        let dayperiods = self.names.dayperiod_names();

        // 1. Resolve difference
        let diff = resolve_difference(start, end, dayperiods);

        let start_pattern = self.selection.select(start);
        let end_pattern = self.selection.select(end);

        // CLDR Step 5: If there is no difference among any of the fields in the pattern,
        // format as a single date using availableFormats, and return.
        if field_is_ignored(start_pattern, end_pattern, diff) {
            return FormattedDateRange(FormattedDateRangeInner::Single(FormattedDateTime {
                pattern: start_pattern,
                input: *start,
                names: self.names,
            }));
        }

        let is_mixed = self.range_selection.date_range.payload.is_payload()
            && self.range_selection.time_range.payload.is_payload();

        // Early fallback for mixed date-time formatter with date difference.
        // UTS 35: If date differs in a mixed skeleton, fall back to range fallback (Case 4).
        if is_mixed && diff.is_date_diff() {
            return FormattedDateRange(self.format_fallback(start, end));
        }

        // 2. Select pattern and format
        let inner = match diff {
            Difference::None => FormattedDateRangeInner::Single(self.format_datetime(start)),
            Difference::Incomparable | Difference::Second => self.format_fallback(start, end),
            diff => {
                if is_mixed {
                    // Case 3: Mixed range, only time differs (date diff was handled by early fallback).
                    let date_formatted = self.format_date_only(start);
                    let time_range_formatted = self
                        .range_selection
                        .time_range
                        .select(start, end, self.selection.options, diff)
                        .map(|pattern_info| {
                            self.make_greatest_difference(
                                self.format_time_only(start),
                                self.format_time_only(end),
                                pattern_info,
                            )
                        });

                    let glue = self.selection.glue.as_ref().map(|g| g.get());
                    if let (Some(time_range), Some(glue)) = (time_range_formatted, glue) {
                        FormattedDateRangeInner::TimeRangeMixed(FormattedTimeRangeMixed {
                            date: date_formatted,
                            time_range,
                            glue,
                        })
                    } else {
                        self.format_fallback(start, end)
                    }
                } else {
                    // Case 2: Time-only or Date-only range.
                    let use_time = diff.is_time_diff();
                    self.format_greatest_difference(start, end, diff, use_time)
                        .unwrap_or_else(|| self.format_fallback(start, end))
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
    fn format_fallback(
        self,
        start: &DateTimeInputUnchecked,
        end: &DateTimeInputUnchecked,
    ) -> FormattedDateRangeInner<'a> {
        let start_formatted = self.format_datetime(start);
        let end_formatted = self.format_datetime(end);
        let glue = self.range_selection.range_glue.get();
        FormattedDateRangeInner::Fallback(FormattedRangeFallback {
            start: start_formatted,
            end: end_formatted,
            glue,
        })
    }
}

fn field_symbol_level(symbol: FieldSymbol) -> Option<u8> {
    match symbol {
        FieldSymbol::Era => Some(0),
        FieldSymbol::Year(_) => Some(1),
        FieldSymbol::Month(_) => Some(2),
        FieldSymbol::Week(_) | FieldSymbol::Day(_) | FieldSymbol::Weekday(_) => Some(3),
        FieldSymbol::DayPeriod(_) => Some(4),
        FieldSymbol::Hour(_) => Some(5),
        FieldSymbol::Minute => Some(6),
        FieldSymbol::Second(_) | FieldSymbol::DecimalSecond(_) => Some(7),
        FieldSymbol::TimeZone(_) => None,
    }
}

fn pattern_has_level(pattern: DateTimeZonePatternDataBorrowed<'_>, min_level: u8) -> bool {
    pattern.iter_items().any(|item| match item {
        PatternItem::Field(field) => {
            field_symbol_level(field.symbol).is_some_and(|lvl| lvl >= min_level)
        }
        _ => false,
    })
}

/// Checks whether the greatest difference is in a field unit that is ignored by both
/// the start and end patterns (i.e. all fields present in both patterns are strictly coarser than `diff`).
///
/// CLDR Step 5: "If there is no difference among any of the fields in the pattern,
/// format as a single date using `availableFormats`, and return."
fn field_is_ignored(
    start_pattern: DateTimeZonePatternDataBorrowed<'_>,
    end_pattern: DateTimeZonePatternDataBorrowed<'_>,
    diff: Difference,
) -> bool {
    let Some(diff_level) = diff.level() else {
        return false;
    };
    !pattern_has_level(start_pattern, diff_level) && !pattern_has_level(end_pattern, diff_level)
}
