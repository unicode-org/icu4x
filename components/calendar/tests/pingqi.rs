// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{
    cal::{
        chinese::{self},
        LunarChinese,
    },
    types::{MonthCode, RataDie},
    Date, Ref,
};

#[derive(Debug, Copy, Clone)]
struct Duration {
    days: u32,
    milliseconds: u32,
}

impl Duration {
    fn to_i64(&self) -> i64 {
        self.days as i64 * MILLISECONDS_IN_DAY as i64 + self.milliseconds as i64
    }
}

/// The mean solar term length indexed according to the Gregorian solar cycle:
///
/// `146097 / 400 / 12`
///
/// 146097 days every 400 years, and 12 major solar terms per year. This resolves to an
/// exact number of seconds.
///
/// This could alternatively be defined in terms of the mean tropical year.
const MEAN_SOLAR_TERM_LENGTH: Duration = Duration {
    days: 30,
    milliseconds: 37746000,
};

/// The mean length of a solstice year, which is 12 times MEAN_SOLAR_TERM_LENGTH.
const MEAN_SOLSTICE_YEAR_LENGTH: Duration = Duration {
    days: 365,
    milliseconds: 20952000,
};

/// The mean synodic length according to Wikipedia.
const MEAN_SYNODIC_MONTH_LENGTH: Duration = Duration {
    days: 29,
    milliseconds: 45842800,
};

/// Number of milliseconds in a day.
const MILLISECONDS_IN_DAY: i64 = 86400000;

/// A RataDie with a number of milliseconds within that day, in local standard time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LocalMoment {
    rata_die: RataDie,
    local_milliseconds: u32,
}

impl LocalMoment {
    /// Adds a specific [`Duration`] to this moment `n` times.
    fn add_duration_times_n(&self, duration: Duration, n: i64) -> LocalMoment {
        let temp = self.local_milliseconds as i64 + (duration.milliseconds as i64 * n);
        let (extra_days, local_milliseconds) = (
            temp.div_euclid(MILLISECONDS_IN_DAY),
            temp.rem_euclid(MILLISECONDS_IN_DAY),
        );
        let rata_die = self.rata_die + extra_days + (duration.days as i64 * n) as i64;
        let local_milliseconds = u32::try_from(local_milliseconds).unwrap();
        Self {
            rata_die,
            local_milliseconds,
        }
    }

    /// Converts this moment to an i64 local timestamp in milliseconds (with Rata Die epoch)
    fn to_i64(&self) -> i64 {
        (self.rata_die.to_i64_date() * MILLISECONDS_IN_DAY) + self.local_milliseconds as i64
    }
}

#[test]
fn test_local_moment_add() {
    let local_moment = LocalMoment {
        rata_die: RataDie::new(1000),
        local_milliseconds: 0,
    };
    let duration = Duration {
        days: 77,
        milliseconds: 25000000,
    };
    assert_eq!(local_moment.add_duration_times_n(duration, 0), local_moment);
    assert_eq!(
        local_moment.add_duration_times_n(duration, 1),
        LocalMoment {
            rata_die: RataDie::new(1077),
            local_milliseconds: 25000000
        }
    );
    assert_eq!(
        local_moment.add_duration_times_n(duration, -1),
        LocalMoment {
            rata_die: RataDie::new(922),
            local_milliseconds: 61400000
        }
    );
    assert_eq!(
        local_moment.add_duration_times_n(duration, -500),
        LocalMoment {
            rata_die: RataDie::new(-37645),
            local_milliseconds: 28000000,
        }
    );
}

/// A fast approximation for the Chinese calendar, inspired by the _píngqì_ (平氣) rule
/// used in the Ming dynasty.
///
/// Stays anchored in the Gregorian calendar, even as the Gregorian calendar drifts
/// from the seasons in the distant future and distant past.
#[derive(Debug, Clone)]
struct FastPingqi {
    pub winter_solstice: LocalMoment,
    pub new_moon: LocalMoment,
}

/// A "solstice year", also known as a _suì_ (歲).
///
/// This is the 12 or 13 month year between two adjacent winter solstices, for internal
/// calculations only. The first month is M11.
struct SolsticeYearInfo {
    /// The first day of M11.
    m11_rd: RataDie,
    /// The lengths of the months, starting with M11 (true = 30 days).
    month_lengths: [bool; 13],
    /// The 0-based index of the leap month, if any.
    leap_month: Option<u8>,
}

impl SolsticeYearInfo {
    /// Returns the index of the lunar new year (M01) within month_lengths.
    fn lunar_new_year_index(&self) -> usize {
        match self.leap_month {
            None => 2,                 // common year
            Some(0) => unreachable!(), // the first month contains the winter solstice
            Some(1) => 3,              // M11L
            Some(2) => 3,              // M12L
            Some(_) => 2,              // all other leap months
        }
    }

    /// Returns the month_lengths in this solstice year, either 12 or 13.
    fn valid_month_lengths(&self) -> &[bool] {
        match self.leap_month {
            None => &self.month_lengths[0..12],
            Some(_) => &self.month_lengths,
        }
    }

    /// Returns the [`RataDie`] of the linar new year (first day of M01).
    fn lunar_new_year_rd(&self) -> RataDie {
        let mut result =
            self.m11_rd + 58 + self.month_lengths[0] as i64 + self.month_lengths[1] as i64;
        if self.lunar_new_year_index() == 3 {
            result += 29 + self.month_lengths[2] as i64;
        }
        result
    }
}

/// Calculates the last moment that occurs on or before the given rata die that is an
/// exact multiple of the given duration relative to the base moment.
///
/// For example, given these inputs:
///
/// - Rata Die: 900
/// - Base Moment: 1000.0 (RD 1000, 0 milliseconds)
/// - Duration: 30.3 (30 days, 25920 milliseconds)
///
/// The result is the moment 878.8 (RD 878, 69120 milliseconds), which is 4 periods before
/// the base moment, and the last period before RD 900.
///
/// This is the heart of FastPingqi.
fn periodic_duration_on_or_before(
    rata_die: RataDie,
    base_moment: LocalMoment,
    duration: Duration,
) -> LocalMoment {
    // For now, do math as i64 milliseconds, which covers 600 million years.
    let upper_bound = LocalMoment {
        rata_die: rata_die + 1,
        local_milliseconds: 0,
    }
    .to_i64();
    let num_periods = (upper_bound - base_moment.to_i64() - 1).div_euclid(duration.to_i64());
    base_moment.add_duration_times_n(duration, num_periods)
}

impl FastPingqi {
    /// Calculates the moment of the nearest winter solstice on or before the given rata die
    fn winter_solstice_on_or_before(&self, rata_die: RataDie) -> LocalMoment {
        periodic_duration_on_or_before(rata_die, self.winter_solstice, MEAN_SOLSTICE_YEAR_LENGTH)
    }

    /// Calculates the moment of the nearest new moon on or before the given rata die
    fn new_moon_on_or_before(&self, rata_die: RataDie) -> LocalMoment {
        periodic_duration_on_or_before(rata_die, self.new_moon, MEAN_SYNODIC_MONTH_LENGTH)
    }

    /// Calculates the [`SolsticeYear`] containing the specified ISO new year, i.e.,
    /// the one starting in November or December of the previous ISO year.
    fn solstice_year_for_iso_year(&self, related_iso: i32) -> SolsticeYearInfo {
        let iso_new_year_rd = calendrical_calculations::iso::fixed_from_iso(related_iso, 1, 1);
        let prev_winter_solstice = self.winter_solstice_on_or_before(iso_new_year_rd);
        let next_winter_solstice =
            prev_winter_solstice.add_duration_times_n(MEAN_SOLAR_TERM_LENGTH, 12);
        let m11_moment = self.new_moon_on_or_before(prev_winter_solstice.rata_die);
        let mut new_moon = m11_moment;
        let mut major_solar_term = prev_winter_solstice;
        let mut month_lengths: [bool; 13] = Default::default();
        let mut month_lengths_iter = month_lengths.iter_mut();
        let mut potential_leap_month = None;
        // Loop over the months to calculate month lengths and identify a leap month:
        loop {
            let next_new_moon = new_moon.add_duration_times_n(MEAN_SYNODIC_MONTH_LENGTH, 1);
            if next_new_moon.rata_die > next_winter_solstice.rata_die {
                break;
            }
            if next_new_moon.rata_die - new_moon.rata_die == 30 {
                *month_lengths_iter.next().unwrap() = true;
            } else {
                debug_assert_eq!(next_new_moon.rata_die - new_moon.rata_die, 29);
                month_lengths_iter.next();
            }
            dbg!(new_moon, major_solar_term);
            if major_solar_term.rata_die >= next_new_moon.rata_die {
                assert!(potential_leap_month.is_none());
                potential_leap_month =
                    Some(u8::try_from(12 - month_lengths_iter.as_slice().len()).unwrap());
            } else {
                major_solar_term = major_solar_term.add_duration_times_n(MEAN_SOLAR_TERM_LENGTH, 1);
            }
            new_moon = next_new_moon;
        }
        match month_lengths_iter.into_slice().len() {
            1 => SolsticeYearInfo {
                m11_rd: m11_moment.rata_die,
                month_lengths,
                leap_month: None,
            },
            0 => SolsticeYearInfo {
                m11_rd: m11_moment.rata_die,
                month_lengths,
                leap_month: Some(potential_leap_month.unwrap()),
            },
            _ => unreachable!(),
        }
    }
}

impl chinese::Rules for FastPingqi {
    fn year_data(&self, related_iso: i32) -> chinese::LunarChineseYearData {
        // Calculate the two solstice years that occur during the lunar year.
        let solstice_year_0 = self.solstice_year_for_iso_year(related_iso);
        let solstice_year_1 = self.solstice_year_for_iso_year(related_iso + 1);
        // The lunar new year occurs in the first solstice year.
        let start_day = solstice_year_0.lunar_new_year_rd();
        // We need to pull 10-11 month lengths from the first solstice year
        // and 2-3 month lengths from the second solstice year.
        //
        // First, we need to figure out if there is a leap month and which solstice year
        // it comes from.
        let ny0 = solstice_year_0.lunar_new_year_index();
        let ny1 = solstice_year_1.lunar_new_year_index();
        let month_lengths_0 = &solstice_year_0.valid_month_lengths()[ny0..];
        let month_lengths_1 = &solstice_year_1.month_lengths[..ny1];
        let num_months = month_lengths_0.len() + month_lengths_1.len();
        let leap_month = match num_months {
            12 => None,
            13 => {
                match (solstice_year_0.leap_month, solstice_year_1.leap_month) {
                    (Some(x), None) if x >= 3 => {
                        // 3 => M01L, 4 => M02L, ...
                        Some(x - 2)
                    }
                    (None, Some(x)) if x <= 2 => {
                        // 1 => M11L, 2 => M12L
                        Some(x + 10)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        // Now copy over the month lengths.
        let mut month_lengths: [bool; 13] = Default::default();
        month_lengths[0..month_lengths_0.len()].copy_from_slice(month_lengths_0);
        month_lengths[month_lengths_0.len()..num_months].copy_from_slice(month_lengths_1);
        // All done.
        chinese::LunarChineseYearData::new(related_iso, start_day, month_lengths, leap_month)
    }

    fn reference_year_from_month_day(
        &self,
        month_code: icu_calendar::types::MonthCode,
        day: u8,
    ) -> Result<chinese::LunarChineseYearData, icu_calendar::DateError> {
        todo!()
    }
}

#[test]
fn test_fast_pingqi() {
    // From navy.mil, minute resolution
    let winter_solstice_2024 = LocalMoment {
        rata_die: calendrical_calculations::iso::fixed_from_iso(2024, 12, 21),
        local_milliseconds: 62400000,
    };
    let new_moon_near_winter_solstice_2024 = LocalMoment {
        rata_die: calendrical_calculations::iso::fixed_from_iso(2024, 12, 1),
        local_milliseconds: 51660000,
    };
    let rules = FastPingqi {
        winter_solstice: winter_solstice_2024,
        new_moon: new_moon_near_winter_solstice_2024,
    };
    let cal = LunarChinese(rules);
    let date =
        Date::try_new_from_codes(None, 2025, MonthCode::new_normal(1).unwrap(), 1, Ref(&cal))
            .unwrap();
    assert_eq!(date.to_iso(), Date::try_new_iso(2025, 1, 29).unwrap());
}
