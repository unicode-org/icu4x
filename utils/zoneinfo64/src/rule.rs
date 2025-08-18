// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{Offset, PossibleOffset, Transition};
use calendrical_calculations::iso;
use calendrical_calculations::rata_die::RataDie;
use core::cmp::Ordering;
use icu_time::zone::UtcOffset;

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
    additional_offset_secs: i32,
    /// The yearly start date of the rule
    start: TzRuleDate,
    /// The yearly end date of the rule
    end: TzRuleDate,
}

#[derive(Debug)]
struct TzRuleDate {
    /// A 1-indexed day number
    day: u8,
    /// A day of the week (1 = Sunday)
    day_of_week: u8,
    /// A 1-indexed month number
    month: u8,
    /// The time in the day (in seconds) that the transition occurs
    transition_time: u32,
    /// How to interpret transition_time
    time_mode: TimeMode,
    /// How to interpret day, day_of_week, and month
    mode: RuleMode,
}

#[derive(Debug, Copy, Clone)]
enum TimeMode {
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
enum RuleMode {
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

    fn is_inverted(&self) -> bool {
        (self.start.month, self.start.day) > (self.end.month, self.end.day)
    }
}

impl TzRuleDate {
    fn new(
        mut day: i8,
        mut day_of_week: i8,
        zero_based_month: u8,
        transition_time: u32,
        time_mode: i8,
    ) -> Option<Self> {
        let month = zero_based_month + 1;

        if day == 0 {
            return None;
        }
        if month > 12 {
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
        } else if day < 1
            || day
                > if month == 2 {
                    29
                } else {
                    30 | month ^ (month >> 3)
                } as i8
        {
            return None;
        }

        Some(Self {
            day: u8::try_from(day).unwrap_or_default(),
            day_of_week: u8::try_from(day_of_week).unwrap_or_default(),
            month: zero_based_month + 1,
            transition_time,
            time_mode,
            mode,
        })
    }

    /// Given a year, return the 1-indexed day number in that year for this transition
    pub(crate) fn day_in_year(&self, year: i32, day_before_year: RataDie) -> u16 {
        let days_before_month = iso::days_before_month(year, self.month);

        if let RuleMode::DOM = self.mode {
            return days_before_month + u16::from(self.day);
        }

        fn weekday(rd: RataDie) -> u8 {
            const MONDAY: RataDie = iso::const_fixed_from_iso(1, 1, 1);
            (rd.since(MONDAY) % 7) as u8
        }

        let weekday_before_month = weekday(day_before_year + days_before_month as i64 + 1);

        // Turn this into a zero-indexed day of week
        let day_of_week_0idx = self.day_of_week - 1;
        let day_of_month = match self.mode {
            RuleMode::DOM | // unreachable
            RuleMode::DOW_IN_MONTH => {
                // First we calculate the first {day_of_week} of the month
                let first_weekday = day_of_week_0idx - weekday_before_month + if day_of_week_0idx > weekday(day_before_year) {
                    0
                } else {
                    7
                };

                // Then we add additional weeks to it if desired
                first_weekday + (self.day - 1) * 7
            }
            // These two compute after/before an "anchor" day in the month
            RuleMode::DOW_GEQ_DOM => {
                let weekday_of_anchor = (weekday_before_month + self.day) % 7;
                let days_to_add = day_of_week_0idx - weekday_of_anchor + if day_of_week_0idx >= weekday_of_anchor {
                    0
                } else {
                    7
                };
                self.day + days_to_add
            }
            RuleMode::DOW_LEQ_DOM => {
                let weekday_of_anchor = (weekday_before_month + self.day) % 7;
                let days_to_subtract = weekday_of_anchor - day_of_week_0idx + if day_of_week_0idx <= weekday_of_anchor {
                    0
                } else {
                    7
                };
                self.day - days_to_subtract
            }
        };
        // Subtract one so we get a 0-indexed value (Jan 1 = day 0)
        days_before_month + u16::from(day_of_month)
    }

    fn timestamp_for_year(
        &self,
        year: i32,
        day_before_year: RataDie,
        standard_offset_seconds: i32,
        additional_offset_seconds: i32,
    ) -> i64 {
        let day = day_before_year + self.day_in_year(year, day_before_year) as i64;
        let start_seconds =
            self.transition_time_to_utc(standard_offset_seconds, additional_offset_seconds);
        day.since(super::EPOCH) * SECONDS_IN_UTC_DAY + i64::from(start_seconds)
    }

    /// Converts the {transition_time} into a time in the UTC day, in seconds
    fn transition_time_to_utc(
        &self,
        standard_offset_seconds: i32,
        additional_offset_seconds: i32,
    ) -> i32 {
        let seconds_of_day = self.transition_time as i32;
        match self.time_mode {
            TimeMode::Utc => seconds_of_day,
            TimeMode::Standard => seconds_of_day - standard_offset_seconds,
            TimeMode::Wall => {
                seconds_of_day - (standard_offset_seconds + additional_offset_seconds)
            }
        }
    }

    /// Converts the {transition_time} into a time in the local day, in seconds
    fn transition_time_to_wall(
        &self,
        standard_offset_seconds: i32,
        additional_offset_seconds: i32,
    ) -> i32 {
        let seconds_of_day = self.transition_time as i32;
        match self.time_mode {
            TimeMode::Utc => seconds_of_day + standard_offset_seconds + additional_offset_seconds,
            TimeMode::Standard => seconds_of_day + additional_offset_seconds,
            TimeMode::Wall => seconds_of_day,
        }
    }
}

impl Rule<'_> {
    /// Get the possible offsets matching to a timestamp given in *local* (wall) time
    ///
    /// Returns None when the rule doesn't apply (this is different from `PossibleOffset::None`,
    /// which means the rule does apply, but the datetime occurred during a gap transition and is thus
    /// invalid).
    pub(crate) fn resolve_local(
        &self,
        year: i32,
        day_before_year: RataDie,
        day_in_year: u16,
        local_time_of_day: i32,
    ) -> Option<PossibleOffset> {
        if year < self.start_year as i32 {
            return None;
        }

        struct DateTime {
            day_in_year: u16,
            local_time_of_day: i32,
        }

        let datetime = DateTime {
            day_in_year,
            local_time_of_day,
        };

        struct LazyRuleEval<'a> {
            year: i32,
            day_before_year: RataDie,
            rule: &'a TzRuleDate,
            standard_offset_seconds: i32,
            additional_offset_seconds: i32,
            correction: i32,
        }

        impl PartialEq<LazyRuleEval<'_>> for DateTime {
            fn eq(&self, other: &LazyRuleEval) -> bool {
                self.partial_cmp(other) == Some(Ordering::Equal)
            }
        }

        impl PartialOrd<LazyRuleEval<'_>> for DateTime {
            fn partial_cmp(&self, rule: &LazyRuleEval) -> Option<Ordering> {
                let mut rule_day_in_year = rule.rule.day_in_year(rule.year, rule.day_before_year);

                // Fast paths, don't need to look at time
                if self.day_in_year > rule_day_in_year + 1 {
                    return Some(Ordering::Greater);
                } else if self.day_in_year < rule_day_in_year - 1 {
                    return Some(Ordering::Less);
                }

                let mut rule_timestamp = rule.rule.transition_time_to_wall(
                    rule.standard_offset_seconds,
                    rule.additional_offset_seconds,
                ) + rule.correction;
                if rule_timestamp >= SECONDS_IN_UTC_DAY as i32 {
                    rule_timestamp -= SECONDS_IN_UTC_DAY as i32;
                    rule_day_in_year += 1;
                }
                if rule_timestamp < 0 {
                    rule_timestamp += SECONDS_IN_UTC_DAY as i32;
                    rule_day_in_year -= 1;
                }

                (self.day_in_year, self.local_time_of_day)
                    .partial_cmp(&(rule_day_in_year, rule_timestamp))
            }
        }

        let before_start = LazyRuleEval {
            year,
            day_before_year,
            rule: &self.inner.start,
            standard_offset_seconds: self.standard_offset_seconds,
            additional_offset_seconds: 0,
            correction: 0,
        };

        let after_start = LazyRuleEval {
            year,
            day_before_year,
            rule: &self.inner.start,
            standard_offset_seconds: self.standard_offset_seconds,
            additional_offset_seconds: 0,
            correction: self.inner.additional_offset_secs,
        };

        let before_end = LazyRuleEval {
            year,
            day_before_year,
            rule: &self.inner.end,
            standard_offset_seconds: self.standard_offset_seconds,
            additional_offset_seconds: self.inner.additional_offset_secs,
            correction: -self.inner.additional_offset_secs,
        };

        let after_end = LazyRuleEval {
            year,
            day_before_year,
            rule: &self.inner.end,
            standard_offset_seconds: self.standard_offset_seconds,
            additional_offset_seconds: self.inner.additional_offset_secs,
            correction: 0,
        };

        // The order of periods depends on both whether the rule is inverted (end before start),
        // and whether the rule offset is positive or negative. Currently zoneinfo64 only contains
        // positive rules.

        let standard_offset = Offset {
            offset: UtcOffset::from_seconds_unchecked(self.standard_offset_seconds),
            rule_applies: false,
        };
        let rule_offset = Offset {
            offset: UtcOffset::from_seconds_unchecked(
                self.standard_offset_seconds + self.inner.additional_offset_secs,
            ),
            rule_applies: true,
        };

        #[allow(clippy::collapsible_else_if)] // symmetry
        if !self.inner.is_inverted() {
            if datetime < before_start {
                if year == self.start_year as i32 {
                    return None;
                }
                Some(PossibleOffset::Single(standard_offset))
            } else if datetime < after_start {
                Some(PossibleOffset::None)
            } else if datetime < before_end {
                Some(PossibleOffset::Single(rule_offset))
            } else if datetime < after_end {
                Some(PossibleOffset::Ambiguous(rule_offset, standard_offset))
            } else {
                Some(PossibleOffset::Single(standard_offset))
            }
        } else {
            if datetime < before_end {
                // Here the rule_offset is fine even if year == start_year, as inverted rules seem
                // to be valid from the start of the year before. This makes sense as TZDB defines
                // rules in terms of start+end, not end+start, so inverted rules always start in
                // the second half of a year (and zoneinfo64 apparently sets the start year to
                // the next, first full year).
                Some(PossibleOffset::Single(rule_offset))
            } else if datetime < after_end {
                Some(PossibleOffset::Ambiguous(rule_offset, standard_offset))
            } else if datetime < before_start {
                Some(PossibleOffset::Single(standard_offset))
            } else if datetime < after_start {
                Some(PossibleOffset::None)
            } else {
                Some(PossibleOffset::Single(rule_offset))
            }
        }
    }

    /// Get the offset matching to a timestamp given in UTC time.
    ///
    /// Returns None if the seconds_since_epoch is not in range of the Rule.
    ///
    /// seconds_since_epoch must resolve to a year that is in-range for i32
    pub(crate) fn resolve_utc(&self, seconds_since_epoch: i64) -> Option<Offset> {
        let year =
            iso::iso_year_from_fixed(super::EPOCH + (seconds_since_epoch / SECONDS_IN_UTC_DAY))
                .unwrap();

        // No transition happens in a different UTC year, this is verified
        // in `test_rule_not_at_year_boundary`
        let local_year = year;

        if local_year < self.start_year as i32 {
            return None;
        }

        let day_before_year = iso::day_before_year(year);

        let start = Transition {
            since: self.inner.start.timestamp_for_year(
                year,
                day_before_year,
                self.standard_offset_seconds,
                0,
            ),
            offset: UtcOffset::from_seconds_unchecked(
                self.standard_offset_seconds + self.inner.additional_offset_secs,
            ),
            rule_applies: true,
        };
        let end = Transition {
            since: self.inner.end.timestamp_for_year(
                year,
                day_before_year,
                self.standard_offset_seconds,
                self.inner.additional_offset_secs,
            ),
            offset: UtcOffset::from_seconds_unchecked(self.standard_offset_seconds),
            rule_applies: false,
        };

        let (first, second) = if self.inner.is_inverted() {
            (end, start)
        } else {
            (start, end)
        };

        if seconds_since_epoch < first.since {
            if !self.inner.is_inverted() && local_year == self.start_year as i32 {
                return None;
            }
            Some(second.into())
        } else if seconds_since_epoch < second.since {
            Some(first.into())
        } else {
            Some(second.into())
        }
    }
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

    #[track_caller]
    fn test_single_year(
        tz: &str,
        year: i32,
        (start_month, start_day, (start_before, start_after)): (u8, u8, (i8, i8)),
        (end_month, end_day, (end_before, end_after)): (u8, u8, (i8, i8)),
    ) {
        let zone = TZDB.get(tz).unwrap();

        // start_before doesn't actually happen
        assert_eq!(
            zone.for_date_time(
                year,
                start_month,
                start_day - start_before.div_euclid(24).unsigned_abs(),
                start_before.rem_euclid(24).unsigned_abs(),
                0,
                0
            ),
            PossibleOffset::None,
        );

        // start_after happens exactly once
        assert!(matches!(
            zone.for_date_time(
                year,
                start_month,
                start_day - start_after.div_euclid(24).unsigned_abs(),
                start_after.rem_euclid(24).unsigned_abs(),
                0,
                0
            ),
            PossibleOffset::Single(_)
        ));

        // end_before happens exactly once
        assert!(matches!(
            zone.for_date_time(
                year,
                end_month,
                end_day - end_before.div_euclid(24).unsigned_abs(),
                end_before.rem_euclid(24).unsigned_abs(),
                0,
                0
            ),
            PossibleOffset::Single(_)
        ));

        // end_after happens again after falling back
        assert!(matches!(
            zone.for_date_time(
                year,
                end_month,
                end_day - end_after.div_euclid(24).unsigned_abs(),
                end_after.rem_euclid(24).unsigned_abs(),
                0,
                0
            ),
            PossibleOffset::Ambiguous(_, _),
        ));
    }

    #[test]
    fn test_los_angeles() {
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
    }

    #[test]
    fn test_london() {
        // This is a Standard rule, so the transition happens
        // at the same time in the standard timezone
        test_single_year("Europe/London", 2017, (3, 26, (1, 2)), (10, 29, (2, 1)));
    }

    #[test]
    fn test_santiago() {
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
