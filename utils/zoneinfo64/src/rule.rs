// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Debug)]
pub(crate) struct TzRule {
    additional_offset_secs: i32,
    start: TzRuleDate,
    end: TzRuleDate,
}

#[derive(Debug)]
struct TzRuleDate {
    day: i8,
    day_of_week: i8,
    month: u8,
    millis_of_day: u32,
    time_mode: TimeMode,
    mode: RuleMode,
}

#[derive(Debug)]
enum TimeMode {
    Wall = 0,
    Standard = 1,
    Utc = 2,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
enum RuleMode {
    DOW_IN_MONTH,
    DOM,
    DOW_GE_DOM,
    DOW_LE_DOM,
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
        millis_of_day: u32,
        time_mode: i8,
    ) -> Option<Self> {
        const GREGORIAN_MONTHS: [i8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if day == 0 {
            return None;
        }
        if month > 11 {
            return None;
        }
        if millis_of_day > 24 * 60 * 60 * 1000 {
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
                    mode = RuleMode::DOW_GE_DOM;
                } else {
                    day = -day;
                    mode = RuleMode::DOW_LE_DOM;
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
            millis_of_day,
            time_mode,
            mode,
        })
    }
}

impl TzRule {
    pub(crate) fn additional_offset_since(
        &self,
        seconds_since_epoch: i64,
        _start_year: u32,
    ) -> (i32, i64) {
        let _ = self.start.month;
        let _ = self.start.day;
        let _ = self.start.day_of_week;
        let _ = self.start.millis_of_day;
        let _ = self.start.time_mode;
        let _ = self.start.mode;

        let _ = self.end.month;
        let _ = self.end.day;
        let _ = self.end.day_of_week;
        let _ = self.end.millis_of_day;
        let _ = self.end.time_mode;
        let _ = self.end.mode;

        (self.additional_offset_secs, seconds_since_epoch)
    }
}
