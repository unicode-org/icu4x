// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{Offset, PossibleOffset};
use calendrical_calculations::iso;
use calendrical_calculations::rata_die::RataDie;
use icu_time::zone::UtcOffset;
use std::ops::Range;

const EPOCH: RataDie = calendrical_calculations::iso::const_fixed_from_iso(1970, 1, 1);
const SECONDS_IN_UTC_DAY: i64 = 86400;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rule<'a> {
    /// The year the rule starts applying
    pub(crate) start_year: u32,
    /// The offset of standard time
    pub(crate) standard_offset_seconds: i32,
    pub(crate) inner: &'a TzRule,
}

#[derive(Debug)]
pub(crate) struct TzRule {
    /// The amount of seconds to add to standard_offset_seconds
    /// to get the rule offset
    pub(crate) additional_offset_secs: i32,
    /// The yearly start date of the rule
    pub(crate) start: TzRuleDate,
    /// The yearly end date of the rule
    pub(crate) end: TzRuleDate,
}

#[derive(Debug)]
pub(crate) struct TzRuleDate {
    /// A 1-indexed day number
    pub(crate) day: u8,
    /// A 1-indexed day of the week (1 = Sunday)
    pub(crate) day_of_week: u8,
    /// A 0-indexed month number
    pub(crate) month: u8,
    /// The time in the day (in seconds) that the transition occurs
    pub(crate) transition_time: u32,
    /// How to interpret transition_time
    pub(crate) time_mode: TimeMode,
    /// How to interpret day, day_of_week, and month
    pub(crate) mode: RuleMode,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimeMode {
    /// {transition_time} is local wall clock time in the time zone
    /// *before* the transition
    ///
    /// e.g. if the transition between LST and LDT is to happen at 02:00,
    /// the time that *would be* 02:00 LST would be the first time of LDT.
    ///
    /// This means that `{local_wall_clock_time}` may never actually be the
    /// wall clock time! The America/Los_Angeles transition occurs at Wall 02:00,
    /// however the transition from PST to PDT is
    /// `2025-03-09T01:59:59-08:00[America/Los_Angeles]` to
    /// 2025-03-09T03:00:00-07:00[America/Los_Angeles],
    /// so 2025-03-09T02:00:00 never occurs.
    ///
    /// This can be turned into Standard by subtracting the offset-from-standard
    /// of the time zone *before* this transition
    Wall = 0,
    /// {transition_time} is local standard time
    ///
    /// Will produce different results from Wall=0 for DST-to-STD transitions
    ///
    /// This can be turned into Wall by adding the offset-from-standard of the time zone
    /// *before* this transition.
    Standard = 1,
    /// {transition_time} is UTC time
    ///
    /// This is UTC time *on the UTC day* identified by this rule; which may
    /// end up on a different local day.
    ///
    /// For example, America/Santiago transitions to STD on the first Sunday after April 2.
    /// at UTC 03:00:00, which is `2025-04-06T03:00:00+00:00[UTC]`. This ends up being
    /// a transition from`2025-04-05T23:59:59-03:00[America/Santiago]` to
    /// `2025-04-05T23:00:00-04:00[America/Santiago]`).
    ///
    /// This can be turned into Standard by subtracting the standard-offset-from-UTC of the
    /// time zone. It can be turned into Wall by subtracting the offset-from-UTC of the time zone
    /// before this transition.
    Utc = 2,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
/// How to interpret `{day}` `{day_of_week}` and `{month}`
pub(crate) enum RuleMode {
    /// The {day}th {day_of_week} in {month}
    ///
    /// Current zoneinfo64 does not use this, instead
    /// choosing to represent this as DOW_GEQ_DOM with day = 1/8/15/22
    DOW_IN_MONTH,
    /// {month} {day}
    ///
    /// Current zoneinfo64 does not use this
    DOM,
    /// The first {day_of_week} on or after {month} {day}
    DOW_GEQ_DOM,
    /// The first {day_of_week} on or before {month} {day}
    ///
    /// Typically, this represents rules like "Last Sunday in March" (Europe/London)
    DOW_LEQ_DOM,
}

/// The weekday before this year started (0 = Sun, 6 -=Sat)
fn weekday_before_year(year: i32) -> u8 {
    const SUNDAY: RataDie = iso::const_fixed_from_iso(2000, 1, 2);
    (iso::fixed_from_iso(year - 1, 12, 31) - SUNDAY).rem_euclid(7) as u8
}

/// Represent the year as a number of days since the start of the 1970 epoch
///
/// i.e. days_since_epoch(1970) = 0
fn days_since_epoch(year: i32) -> i64 {
    let rd = iso::fixed_from_iso(year, 1, 1);
    rd - EPOCH
}

impl TzRuleDate {
    /// Given a year, return the 0-indexed day number in that year for this transition
    pub(crate) fn day_in_year(&self, year: i32) -> u16 {
        let weekday_before_year = weekday_before_year(year);
        let days_before_month = iso::days_before_month(year, self.month + 1);
        let weekday_before_month =
            (weekday_before_year + days_before_month.rem_euclid(7) as u8).rem_euclid(7);

        // Turn this into a zero-indexed day of week
        let day_of_week_0idx = self.day_of_week - 1;
        let day_of_month = match self.mode {
            RuleMode::DOM => self.day,
            RuleMode::DOW_IN_MONTH => {
                // First we calculate the first {day_of_week} of the month
                let first_weekday = if day_of_week_0idx > weekday_before_year {
                    day_of_week_0idx - weekday_before_month
                } else {
                    7 + day_of_week_0idx - weekday_before_month
                };

                // Then we add additional weeks to it if desired
                first_weekday + (self.day - 1) * 7
            }
            // These two compute after/before an "anchor" day in the month
            RuleMode::DOW_GEQ_DOM => {
                let weekday_of_anchor = (weekday_before_month + self.day).rem_euclid(7);
                let days_to_add = if day_of_week_0idx >= weekday_of_anchor {
                    day_of_week_0idx - weekday_of_anchor
                } else {
                    7 + day_of_week_0idx - weekday_of_anchor
                };
                self.day + days_to_add
            }
            RuleMode::DOW_LEQ_DOM => {
                let weekday_of_anchor = (weekday_before_month + self.day).rem_euclid(7);
                let days_to_subtract = if day_of_week_0idx <= weekday_of_anchor {
                    weekday_of_anchor - day_of_week_0idx
                } else {
                    7 - day_of_week_0idx + weekday_of_anchor
                };
                self.day - days_to_subtract
            }
        };
        // Subtract one so we get a 0-indexed value (Jan 1 = day 0)
        days_before_month + u16::from(day_of_month) - 1
    }
}

impl TzRule {
    pub(crate) fn from_raw(value: &[i32; 11]) -> Self {
        Self {
            additional_offset_secs: value[10],
            start: TzRuleDate::new(
                value[1] as i8,
                value[2] as i8,
                value[0] as u8,
                value[3] as u32,
                value[4] as i8,
            )
            .unwrap(),
            end: TzRuleDate::new(
                value[6] as i8,
                value[7] as i8,
                value[5] as u8,
                value[8] as u32,
                value[9] as i8,
            )
            .unwrap(),
        }
    }
}

impl TzRuleDate {
    fn new(
        mut day: i8,
        mut day_of_week: i8,
        month: u8,
        transition_time: u32,
        time_mode: i8,
    ) -> Option<Self> {
        const GREGORIAN_MONTHS: [i8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if day == 0 {
            return None;
        }
        if month > 11 {
            return None;
        }
        if i64::from(transition_time) > SECONDS_IN_UTC_DAY {
            return None;
        }

        let time_mode = match time_mode {
            0 => TimeMode::Wall,
            1 => TimeMode::Standard,
            2 => TimeMode::Utc,
            _ => return None,
        };

        let mode;

        if day_of_week == 0 {
            mode = RuleMode::DOM;
        } else {
            if day_of_week > 0 {
                mode = RuleMode::DOW_IN_MONTH
            } else {
                day_of_week = -day_of_week;
                if day > 0 {
                    mode = RuleMode::DOW_GEQ_DOM;
                } else {
                    day = -day;
                    mode = RuleMode::DOW_LEQ_DOM;
                }
            }
            if day_of_week > 7 {
                return None;
            }
        }

        if mode == RuleMode::DOW_IN_MONTH {
            if !(-5..=5).contains(&day) {
                return None;
            }
        } else if day < 1 || day > GREGORIAN_MONTHS[month as usize] {
            return None;
        }

        debug_assert!(day >= 0);
        debug_assert!(day_of_week >= 0);

        Some(Self {
            day: u8::try_from(day).unwrap_or_default(),
            day_of_week: u8::try_from(day_of_week).unwrap_or_default(),
            month,
            transition_time,
            time_mode,
            mode,
        })
    }
}

/// A reference point for a transition start time
#[derive(Copy, Clone, Debug)]
enum WallReference {
    /// The time is in wall time pre-transition (note that this time may not occur!)
    Prev,
    /// The time is in wall time post-transition
    Next,
}

#[derive(Copy, Clone, Debug)]
struct TransitionsForYear {
    /// The transition start time as a number of seconds since the epoch
    start_epoch_seconds: i64,
    /// The transition end time as a number of seconds since the epoch
    end_epoch_seconds: i64,
}

impl TransitionsForYear {
    /// For a UTC TransitionsForYear, convert it to the given WallReference mode
    fn to_wall(self, rule: &Rule, mode_to: WallReference) -> Self {
        let standard = i64::from(rule.standard_offset_seconds);
        let rule = standard + i64::from(rule.inner.additional_offset_secs);

        match mode_to {
            WallReference::Prev => TransitionsForYear {
                start_epoch_seconds: self.start_epoch_seconds + standard,
                end_epoch_seconds: self.end_epoch_seconds + rule,
            },
            WallReference::Next => TransitionsForYear {
                start_epoch_seconds: self.start_epoch_seconds + rule,
                end_epoch_seconds: self.end_epoch_seconds + standard,
            },
        }
    }

    /// Returns the range between offsets in this year
    /// This may cover DST or standard time, whichever starts first
    pub(crate) fn range(&self) -> Range<i64> {
        if self.range_is_standard() {
            self.end_epoch_seconds..self.start_epoch_seconds
        } else {
            self.start_epoch_seconds..self.end_epoch_seconds
        }
    }

    /// Whether range() contains standard time
    pub(crate) fn range_is_standard(&self) -> bool {
        self.start_epoch_seconds > self.end_epoch_seconds
    }
}

impl Rule<'_> {
    /// Get the transitions for a year, either as Utc or Wall
    fn transitions_for_year(&self, year: i32) -> TransitionsForYear {
        let days_since_epoch = days_since_epoch(year);

        let start = &self.inner.start;
        let start_day_in_year = start.day_in_year(year);
        let start_seconds = self.transition_time_to_utc(start, true);
        let start_epoch_seconds = (days_since_epoch + i64::from(start_day_in_year))
            * SECONDS_IN_UTC_DAY
            + i64::from(start_seconds);

        let end = &self.inner.end;
        let end_day_in_year = end.day_in_year(year);
        let end_seconds = self.transition_time_to_utc(end, false);
        let end_epoch_seconds = (days_since_epoch + i64::from(end_day_in_year))
            * SECONDS_IN_UTC_DAY
            + i64::from(end_seconds);

        TransitionsForYear {
            start_epoch_seconds,
            end_epoch_seconds,
        }
    }

    /// Converts the {transition_time} into a time in the UTC day, in seconds
    /// for either the start or end transition
    fn transition_time_to_utc(&self, date: &TzRuleDate, is_start: bool) -> i32 {
        let seconds_of_day = date.transition_time as i32;
        let standard = self.standard_offset_seconds;
        let rule = standard + self.inner.additional_offset_secs;
        match date.time_mode {
            TimeMode::Utc => seconds_of_day,
            TimeMode::Standard => seconds_of_day - standard,
            // Tz before this transition was standard
            TimeMode::Wall if is_start => seconds_of_day - standard,
            // Tz before this transition was rule
            TimeMode::Wall => seconds_of_day - rule,
        }
    }
    /// Get the possible offsets matching to a timestamp given in *local* (wall) time
    ///
    /// seconds_since_local_epoch is the seconds-since-epoch of the datetime interpreted as
    /// a UTC timestamp. This is *not* the seconds since `1970-01-01[tz]`, since that time may have
    /// its own offset, it is `y-m-dThh:mm:ss` interpreted as a UTC offset.
    ///
    /// `local_year` is the year of the local (wall) time.
    ///
    /// Returns None when the rule doesn't apply (this is different from `PossibleOffset::None`,
    /// which means the rule does apply, but the datetime occurred during a gap transition and is thus
    /// invalid).
    pub(crate) fn resolve_local(
        &self,
        local_year: i32,
        seconds_since_local_epoch: i64,
    ) -> Option<PossibleOffset> {
        // If we're out of range for the rule, return None.
        if local_year < self.start_year as i32 {
            return None;
        }

        // We assume that transitions do not cross year boundaries (Invariant: rule-stays-inside-year)
        let transitions = self.transitions_for_year(local_year);
        let transitions_prev = transitions.to_wall(self, WallReference::Prev);
        let transitions_next = transitions.to_wall(self, WallReference::Next);
        let range_is_standard = transitions.range_is_standard();
        let range_prev = transitions_prev.range();
        let range_next = transitions_next.range();

        let standard = self.standard_offset_seconds;
        let standard_offset = Offset {
            offset: UtcOffset::from_seconds_unchecked(standard),
            rule_applies: false,
        };
        let additional = standard + self.inner.additional_offset_secs;
        let additional_offset = Offset {
            offset: UtcOffset::from_seconds_unchecked(additional),
            rule_applies: true,
        };
        let (in_range_offset, out_of_range_offset) = if range_is_standard {
            (standard_offset, additional_offset)
        } else {
            (additional_offset, standard_offset)
        };

        if local_year == self.start_year as i32 {
            // If we're before the first rule transition in the year, we should just
            // use the previous transition
            if seconds_since_local_epoch < range_prev.start
                && seconds_since_local_epoch < range_next.start
            {
                return None;
            }

            // This does not handle the case where we are in an overlap offset at the beginning of the year.
            //
            // This will only happen during an inverted rule (e.g. when daylight savings ends first, and then starts)
            //
            // It's not 100% clear how to interpret the data in that case, and at the moment in zoneinfo64
            // all transitions smoothly flow into the rules that come after them (invariant: last-transition-smoothly-into-rule)
            // so it doesn't matter if you attempt to resolve this via transitions or via rules.
        }

        // Prev is in the time zone *before* the transition, Next is in the time zone of and after the transition
        // This means that when something is >= prev but < next there is no match, but if something is
        // >= next and < prev there is an ambiguous match

        if seconds_since_local_epoch < range_prev.start
            && seconds_since_local_epoch < range_next.start
        {
            // We're before any of the transitions
            Some(PossibleOffset::Single(out_of_range_offset))
        } else if seconds_since_local_epoch >= range_prev.start
            && seconds_since_local_epoch < range_next.start
        {
            // We're in the gap of the first transition
            Some(PossibleOffset::None)
        } else if seconds_since_local_epoch < range_prev.start
            && seconds_since_local_epoch >= range_next.start
        {
            // We're in the ambiguous part of the first transition
            // This happens when prev.start > next.start, which happens if the new (in range) offset is smaller
            debug_assert!(in_range_offset.offset <= out_of_range_offset.offset);
            Some(PossibleOffset::Ambiguous(
                out_of_range_offset,
                in_range_offset,
            ))
        } else if seconds_since_local_epoch < range_prev.end
            && seconds_since_local_epoch < range_next.end
        {
            // We're between the two transitions
            Some(PossibleOffset::Single(in_range_offset))
        } else if seconds_since_local_epoch >= range_prev.end
            && seconds_since_local_epoch < range_next.end
        {
            // We're in the gap of the second transition
            Some(PossibleOffset::None)
        } else if seconds_since_local_epoch < range_prev.end
            && seconds_since_local_epoch >= range_next.end
        {
            // We're in the ambiguous part of the second transition
            // This happens when prev.end > next.end, which happens if the new (out of range) offset is smaller
            debug_assert!(out_of_range_offset.offset <= in_range_offset.offset);
            Some(PossibleOffset::Ambiguous(
                in_range_offset,
                out_of_range_offset,
            ))
        } else {
            // We're out of range
            Some(PossibleOffset::Single(out_of_range_offset))
        }
    }

    /// Get the offset matching to a timestamp given in UTC time.
    ///
    /// Returns None if the seconds_since_epoch is not in range of the Rule.
    ///
    /// seconds_since_epoch must resolve to a year that is in-range for i32
    pub(crate) fn resolve_utc(&self, seconds_since_utc_epoch: i64) -> Option<Offset> {
        let rd = rd_for_seconds(seconds_since_utc_epoch);
        let ymd = iso::iso_from_fixed(rd);
        debug_assert!(ymd.is_ok());
        let Ok((year, _, _)) = ymd else {
            // GIGO behavior for out of range dates
            return Default::default();
        };

        if year < self.start_year as i32 {
            // We are before the rule year, return None
            return None;
        }

        // We assume that transitions do not cross year boundaries (Invariant: rule-stays-inside-year)
        let transitions = self.transitions_for_year(year);

        let range_is_standard = transitions.range_is_standard();
        let range = transitions.range();

        let standard = self.standard_offset_seconds;

        if year == self.start_year as i32 {
            // The invariant last-transition-smoothly-into-rule ensures that
            // the last transition smoothly transitions into the rule offset,
            // however we might as well explicitly handle the case where the
            // last transition produces an offset that is not the same as the
            // rule in case that invariant doesn't uphold.
            if seconds_since_utc_epoch < range.start {
                return None;
            }
        }

        if range.contains(&seconds_since_utc_epoch) ^ range_is_standard {
            Some(Offset {
                rule_applies: true,
                offset: UtcOffset::from_seconds_unchecked(
                    standard + self.inner.additional_offset_secs,
                ),
            })
        } else {
            Some(Offset {
                rule_applies: false,
                offset: UtcOffset::from_seconds_unchecked(standard),
            })
        }
    }
}

/// Convert a value of seconds since the epoch to a Rata Die.
fn rd_for_seconds(seconds_since_epoch: i64) -> RataDie {
    let days = seconds_since_epoch / SECONDS_IN_UTC_DAY;
    EPOCH + days
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TZDB;
    use chrono::Datelike;

    /// This tests invariants we rely on in our code
    ///
    /// These invariants not being upheld should never cause a panic, but can produce garbage behavior.
    #[test]
    fn test_rule_not_at_year_boundary() {
        for chrono in chrono_tz::TZ_VARIANTS {
            let iana = chrono.name();

            let zoneinfo64 = TZDB.get(iana).unwrap();

            if let Some(rule) = zoneinfo64.final_rule {
                let final_offset = zoneinfo64.transition_offset_idx(i64::MAX);
                let offset = zoneinfo64.transition_offset_at(final_offset);
                let utc_datetime = chrono::DateTime::from_timestamp(offset.since, 0)
                    .unwrap()
                    .naive_utc();

                assert!(
                    utc_datetime.year() < rule.start_year as i32,
                    "{iana}: Expected last transition to not be in rule year {} < {} \
                    (invariant: last-transition-not-in-rule-year)",
                    utc_datetime.year(),
                    rule.start_year
                );

                // The first offset at the beginning of the year
                let rule_offset_at_beginning_of_year =
                    if rule.inner.start.month > rule.inner.end.month {
                        rule.standard_offset_seconds + rule.inner.additional_offset_secs
                    } else {
                        rule.standard_offset_seconds
                    };

                assert_eq!(rule_offset_at_beginning_of_year, offset.offset.to_seconds(),
                           "{iana}: Expect that the offset at the beginning of the year is the same as the \
                           offset after the last transition. \
                           (invariant: last-transition-smoothly-into-rule)");

                let max_delta = (rule.standard_offset_seconds.abs()
                    + rule.inner.additional_offset_secs.abs())
                    as u32;
                for date in [&rule.inner.start, &rule.inner.end] {
                    let seconds_of_day = date.transition_time;
                    if date.month == 0 && date.day == 1 {
                        assert!(
                            seconds_of_day > max_delta,
                            "{iana}: Rule at beginning should not cross year boundary \
                                 {seconds_of_day} > Δ{max_delta} \
                                 (invariant: rule-stays-inside-year)"
                        );
                    }
                    if date.month == 11 && date.day == 31 {
                        assert!(
                            seconds_of_day + max_delta < SECONDS_IN_UTC_DAY as u32,
                            "{iana}: Rule at end of year should not cross year boundary \
                                 {seconds_of_day} + Δ{max_delta} < 24h \
                                 (invariant: rule-stays-inside-year)"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_weekday_before_year() {
        // Dec 31 1999 was a Friday
        assert_eq!(weekday_before_year(2000), 5);
        // Dec 31 2024 was a Tuesday
        assert_eq!(weekday_before_year(2025), 2);
        // Dec 31 1969 was a Wednesday
        assert_eq!(weekday_before_year(1970), 3);
        // Dec 32 3029 will be a Thursday
        assert_eq!(weekday_before_year(3030), 4);
        // Dec 32 0999 (proleptic) was a Tuesday
        assert_eq!(weekday_before_year(1000), 2);
    }

    fn test_single_year(
        tz: &str,
        year: i32,
        (start_month, start_day, (start_hours_prev, start_hours_next)): (u8, u8, (i8, i8)),
        (end_month, end_day, (end_hours_prev, end_hours_next)): (u8, u8, (i8, i8)),
    ) {
        let rule = TZDB.get(tz).unwrap().final_rule.unwrap();
        let epoch_start = iso::fixed_from_iso(year, start_month, start_day) - EPOCH;
        let epoch_end = iso::fixed_from_iso(year, end_month, end_day) - EPOCH;

        let seconds_start_prev =
            epoch_start * SECONDS_IN_UTC_DAY + i64::from(start_hours_prev) * 3600;
        let seconds_start_next =
            epoch_start * SECONDS_IN_UTC_DAY + i64::from(start_hours_next) * 3600;
        let seconds_end_prev = epoch_end * SECONDS_IN_UTC_DAY + i64::from(end_hours_prev) * 3600;
        let seconds_end_next = epoch_end * SECONDS_IN_UTC_DAY + i64::from(end_hours_next) * 3600;
        let transitions = rule.transitions_for_year(year);

        let transitions_prev = transitions.to_wall(&rule, WallReference::Prev);
        let transitions_next = transitions.to_wall(&rule, WallReference::Next);

        assert_eq!(
            seconds_start_prev,
            transitions_prev.start_epoch_seconds,
            "{tz}: {year}-{start_month}-{start_day} at {start_hours_prev} (before transition). Delta: {}h",
            (seconds_start_prev - transitions_prev.start_epoch_seconds) as f64 / 3600.
        );
        assert_eq!(
            seconds_start_next,
            transitions_next.start_epoch_seconds,
            "{tz}: {year}-{start_month}-{start_day} at {start_hours_next} (after transition). Delta: {}h",
            (seconds_start_next - transitions_next.start_epoch_seconds) as f64 / 3600.
        );

        assert_eq!(
            seconds_end_prev,
            transitions_prev.end_epoch_seconds,
            "{tz}: {year}-{end_month}-{end_day} at {end_hours_prev} (before transition). Delta: {}h",
            (seconds_end_prev - transitions_prev.end_epoch_seconds) as f64 / 3600.
        );
        assert_eq!(
            seconds_end_next,
            transitions_next.end_epoch_seconds,
            "{tz}: {year}-{end_month}-{end_day} at {end_hours_next} (after transition). Delta: {}h",
            (seconds_end_next - transitions_next.end_epoch_seconds) as f64 / 3600.
        );
    }

    #[test]
    fn test_transitions_for_year() {
        // This is a Wall rule
        // so the transition happens at the same time in the
        // previous timezone
        test_single_year(
            "America/Los_Angeles",
            2025,
            // The transition happens at 02:00 in the previous offset
            // and 03:00/01:00 in the next
            (3, 9, (2, 3)),
            (11, 2, (2, 1)),
        );

        // This is a Standard rule, so the transition happens
        // at the same time in the standard timezone
        test_single_year("Europe/London", 2017, (3, 26, (1, 2)), (10, 29, (2, 1)));

        // This is a Utc rule, so the transition happens
        // at the same time in UTC
        test_single_year(
            "America/Santiago",
            2025,
            // Note: this is in the southern hemisphere,
            // the transition start is later in the year
            (9, 7, (0, 1)),
            // The transition day is April 6, but the backwards
            // transition briefly puts us back in April 5, so we get a -1
            (4, 6, (0, -1)),
        );
    }
}
