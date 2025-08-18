// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{Offset, PossibleOffset, SECONDS_IN_UTC_DAY};

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
    pub(crate) day: i8,
    /// A 1-indexed day of the week (1 = Sunday)
    pub(crate) day_of_week: i8,
    /// A 0-indexed month number
    pub(crate) month: u8,
    /// The time in the day (in seconds) that the transition occurs
    pub(crate) transition_time: u32,
    /// How to interpret transition_time
    pub(crate) time_mode: TimeMode,
    /// How to interpret day, day_of_week, and month
    pub(crate) mode: RuleMode,
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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

        Some(Self {
            day,
            day_of_week,
            month,
            transition_time,
            time_mode,
            mode,
        })
    }
}

impl Rule<'_> {
    #[allow(
        clippy::too_many_arguments,
        reason = "We're passing around datetimes, they need many arguments"
    )]
    pub(crate) fn resolve_local(
        &self,
        _year: i32,
        _month: u8,
        _day: u8,
        _hour: u8,
        _minute: u8,
        _second: u8,
    ) -> PossibleOffset {
        // Unimplemented
        let _ = self.standard_offset_seconds;
        let inner = self.inner;
        let _ = inner.additional_offset_secs;
        let _ = inner.start.month;
        let _ = inner.start.day;
        let _ = inner.start.day_of_week;
        let _ = inner.start.transition_time;
        let _ = inner.start.time_mode;
        let _ = inner.start.mode;
        let _ = inner.end.month;
        let _ = inner.end.day;
        let _ = inner.end.day_of_week;
        let _ = inner.end.transition_time;
        let _ = inner.end.time_mode;
        let _ = inner.end.mode;

        PossibleOffset::None
    }

    pub(crate) fn resolve_utc(&self, _year: i32, _seconds_since_epoch: i64) -> Offset {
        // Unimplemented
        Default::default()
    }
}
