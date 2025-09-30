// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use calendrical_calculations::gregorian::DAYS_IN_400_YEAR_CYCLE;
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
    milliseconds: i64,
}

impl Duration {
    const fn new(days: u32, millis_in_day: u32) -> Self {
        Self {
            milliseconds: days as i64 * MILLISECONDS_IN_DAY + millis_in_day as i64,
        }
    }
}

impl core::ops::Mul<i64> for Duration {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            milliseconds: self.milliseconds * rhs,
        }
    }
}

macro_rules! duration_from_day_fraction {
    ($n:tt $(/ $d:tt)+) => {{
        let days = ($n $( / $d)+) as u32;
        let milliseconds = ((MILLISECONDS_IN_DAY as i128 * $n as i128 $( / $d as i128)+) - ($n as i128 $( / $d as i128)+ * MILLISECONDS_IN_DAY as i128)) as u32;
        Duration::new(days, milliseconds)
    }};
    ($n:tt $(/ $d:tt)+, exact) => {{
        let days = ($n $( / $d)+) as u32;
        // This works by using exact rounding in the first term, and intermediate rounding in the second term
        let milliseconds = ((MILLISECONDS_IN_DAY as i128 * $n as i128 $( / $d as i128)+) - ($n as i128 $( / $d as i128)+ * MILLISECONDS_IN_DAY as i128)) as u32;
        // milliseconds needs to be a multiple of (MILLISECONDS_IN_DAY / $denominators) to be exact
        assert!((milliseconds as i128 $(* $d as i128)+) % MILLISECONDS_IN_DAY as i128 == 0, "inexact");
        Duration::new(days, milliseconds)
    }};
}

/// The mean year length according to the Gregorian solar cycle.
const MEAN_GREGORIAN_YEAR_LENGTH: Duration =
    duration_from_day_fraction!(DAYS_IN_400_YEAR_CYCLE / 400, exact);

/// The mean solar term length according to the Gregorian solar cycle
const MEAN_GREGORIAN_SOLAR_TERM_LENGTH: Duration =
    duration_from_day_fraction!(DAYS_IN_400_YEAR_CYCLE / 400 / 12, exact);

/// The mean synodic length on Jan 1 2000 according to the [Astronomical Almanac (1992)].
///
/// [Astronomical Almanac (1992)]: https://archive.org/details/131123ExplanatorySupplementAstronomicalAlmanac/page/n302/mode/1up
const MEAN_SYNODIC_MONTH_LENGTH: Duration =
    duration_from_day_fraction!(295305888531 / 10000000000i64);

/// Number of milliseconds in a day.
const MILLISECONDS_IN_DAY: i64 = 24 * 60 * 60 * 1000;

/// A RataDie with a number of milliseconds within that day, in local standard time.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LocalMoment {
    rata_die: RataDie,
    local_milliseconds: u32,
}

impl core::ops::Add<Duration> for LocalMoment {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        let temp = self.local_milliseconds as i64 + duration.milliseconds;
        Self {
            rata_die: self.rata_die + temp.div_euclid(MILLISECONDS_IN_DAY),
            local_milliseconds: temp.rem_euclid(MILLISECONDS_IN_DAY) as u32,
        }
    }
}

#[test]
fn test_local_moment_add() {
    #![allow(clippy::erasing_op)]
    let local_moment = LocalMoment {
        rata_die: RataDie::new(1000),
        local_milliseconds: 0,
    };
    let duration = Duration::new(77, 25000000);
    assert_eq!(local_moment + duration * 0, local_moment);
    assert_eq!(
        local_moment + duration * 1,
        LocalMoment {
            rata_die: RataDie::new(1077),
            local_milliseconds: 25000000
        }
    );
    assert_eq!(
        local_moment + duration * -1,
        LocalMoment {
            rata_die: RataDie::new(922),
            local_milliseconds: 61400000
        }
    );
    assert_eq!(
        local_moment + duration * -500,
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
    base_moment
        + duration
            * ((rata_die - base_moment.rata_die + 1) * MILLISECONDS_IN_DAY
                - base_moment.local_milliseconds as i64
                - 1)
            .div_euclid(duration.milliseconds)
}

impl icu_calendar::cal::scaffold::UnstableSealed for FastPingqi {}
impl chinese::Rules for FastPingqi {
    fn year_data(&self, related_iso: i32) -> chinese::LunarChineseYearData {
        let mut major_solar_term = periodic_duration_on_or_before(
            calendrical_calculations::gregorian::day_before_year(related_iso),
            self.winter_solstice,
            MEAN_GREGORIAN_YEAR_LENGTH,
        );

        let mut new_moon = periodic_duration_on_or_before(
            major_solar_term.rata_die,
            self.new_moon,
            MEAN_SYNODIC_MONTH_LENGTH,
        );

        // The solstice is in the month of the 10th solar term of the previous year
        let mut solar_term = -2;

        // Skip the months before the year
        while solar_term < 0 {
            let next_new_moon = new_moon + MEAN_SYNODIC_MONTH_LENGTH;

            if major_solar_term.rata_die < next_new_moon.rata_die {
                solar_term += 1;
                major_solar_term = major_solar_term + MEAN_GREGORIAN_SOLAR_TERM_LENGTH;
            }

            new_moon = next_new_moon;
        }

        let start_day = new_moon.rata_die;
        let mut month_lengths = [false; 13];
        let mut month = 0;
        let mut leap_month = None;

        // Iterate over the 12 solar terms, producing potentially 13 months
        while solar_term < 12 {
            let next_new_moon = new_moon + MEAN_SYNODIC_MONTH_LENGTH;

            if major_solar_term.rata_die < next_new_moon.rata_die {
                solar_term += 1;
                major_solar_term = major_solar_term + MEAN_GREGORIAN_SOLAR_TERM_LENGTH;
            } else {
                leap_month = Some(month as u8 + 1);
            }

            month_lengths[month] = next_new_moon.rata_die - new_moon.rata_die == 30;

            month += 1;
            new_moon = next_new_moon;
        }

        chinese::LunarChineseYearData::new(related_iso, start_day, month_lengths, leap_month)
    }

    fn reference_year_from_month_day(
        &self,
        _month_code: icu_calendar::types::MonthCode,
        _day: u8,
    ) -> Result<chinese::LunarChineseYearData, icu_calendar::DateError> {
        todo!()
    }
}

#[test]
fn test_fast_pingqi() {
    // From navy.mil, minute resolution
    let winter_solstice_2024 = LocalMoment {
        rata_die: calendrical_calculations::gregorian::fixed_from_gregorian(2024, 12, 21),
        local_milliseconds: 62400000,
    };
    let new_moon_near_winter_solstice_2024 = LocalMoment {
        rata_die: calendrical_calculations::gregorian::fixed_from_gregorian(2024, 12, 1),
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
