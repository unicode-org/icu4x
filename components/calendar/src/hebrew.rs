use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{self, div_rem_euclid, div_rem_euclid64, div_rem_euclid_f64, next};
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::types::Moment;
use crate::{astronomy::*, Iso};
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;

/// Hebrew Arithmetical Calendar
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct Hebrew;
/// Observational Hebrew Calendar
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)]
pub struct ObservationalHebrew;

// Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2206
const FIXED_HEBREW_EPOCH: RataDie = Julian::fixed_from_julian_integers(-3761, 10, 8);

// Hebrew Location
// Lisp code reference:
const HAIFA: Location = Location {
    latitude: 32.82,
    longitude: 35.0,
    elevation: 0.0,
    zone: (1_f64 / 12_f64),
};

// The Hebrew Months
const NISAN: u8 = 1;
const IYYAR: u8 = 2;
const SIVAN: u8 = 3;
const TAMMUZ: u8 = 4;
const AV: u8 = 5;
const ELUL: u8 = 6;
const TISHRI: u8 = 7;
const MARHESHVAN: u8 = 8;
const KISLEV: u8 = 9;
const TEVET: u8 = 10;
const SHEVAT: u8 = 11;
const ADAR: u8 = 12;
const ADARII: u8 = 13;

/// The inner date type used for representing [`Date`]s of [`Hebrew`]. See [`Date`] and [`Hebrew`] for more details.

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct HebrewDateInner(ArithmeticDate<Hebrew>);

impl CalendarArithmetic for Hebrew {
    fn month_days(year: i32, month: u8) -> u8 {
        todo!()
    }

    fn months_for_every_year(year: i32) -> u8 {
        todo!()
    }

    fn days_in_provided_year(_year: i32) -> u32 {
        todo!()
    }

    fn is_leap_year(year: i32) -> bool {
        div_rem_euclid(7 * year + 1, 19).1 < 7
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        todo!()
    }
}

impl Calendar for Hebrew {
    type DateInner = HebrewDateInner;

    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        todo!()
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        todo!()
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        todo!()
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        todo!()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        todo!()
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        todo!()
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        calendar2: &Self,
        largest_unit: DateDurationUnit,
        smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        todo!()
    }

    fn debug_name(&self) -> &'static str {
        todo!()
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        todo!()
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        todo!()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        todo!()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        todo!()
    }
}

impl Hebrew {
    /// Constructs a new Hebrew Calendar
    pub fn new() -> Self {
        Self
    }
    // Hebrew New Moon
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L2244
    pub(crate) fn molad(h_year: i32, h_month: u8) -> Moment {
        let y = if h_month < TISHRI { h_year + 1 } else { h_year };

        let months_elapsed = (h_month as f64 - TISHRI as f64)
            + (libm::floor((1.0 / 19.0) * (235.0 * y as f64 - 234.0)));

        Moment::new(
            FIXED_HEBREW_EPOCH.to_f64_date() - (876.0 / 25920.0)
                + months_elapsed * (29.0 + (1.0 / 2.0) + (793.0 / 25920.0)),
        )
    }

    #[allow(dead_code)]
    pub fn last_month_of_hebrew_year(h_year: i32) -> u8 {
        if Self::is_leap_year(h_year) {
            ADARII
        } else {
            ADAR
        }
    }

    #[allow(dead_code)]
    pub fn hebrew_sabbatical_year(h_year: i32) -> bool {
        div_rem_euclid(h_year, 7).1 == 0
    }

    #[allow(dead_code)]
    pub fn hebrew_calendar_elapsed_days(h_year: i32) -> i32 {
        let months_elapsed = libm::floor((1.0 / 19.0) * (235.0 * h_year as f64 - 234.0));
        let parts_elapsed = 12084.0 + 13753.0 * months_elapsed;
        let days = 29.0 * months_elapsed + libm::floor(parts_elapsed / 25920.0);

        if div_rem_euclid_f64(3.0 * (days + 1.0), 7.0).1 < 3.0 {
            days as i32 + 1
        } else {
            days as i32
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[derive(Debug)]
    struct DateCase {
        year: i32,
        month: u8,
        day: u8,
    }

    static TEST_FIXED_DATE: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];

    static HEBREW_DATES: [DateCase; 33] = [
        DateCase {
            year: 3174,
            month: 5,
            day: 10,
        },
        DateCase {
            year: 3593,
            month: 9,
            day: 25,
        },
        DateCase {
            year: 3831,
            month: 7,
            day: 3,
        },
        DateCase {
            year: 3896,
            month: 7,
            day: 9,
        },
        DateCase {
            year: 4230,
            month: 10,
            day: 18,
        },
        DateCase {
            year: 4336,
            month: 3,
            day: 4,
        },
        DateCase {
            year: 4455,
            month: 8,
            day: 13,
        },
        DateCase {
            year: 4773,
            month: 2,
            day: 6,
        },
        DateCase {
            year: 4856,
            month: 2,
            day: 23,
        },
        DateCase {
            year: 4950,
            month: 1,
            day: 7,
        },
        DateCase {
            year: 5000,
            month: 13,
            day: 8,
        },
        DateCase {
            year: 5048,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 5058,
            month: 2,
            day: 7,
        },
        DateCase {
            year: 5151,
            month: 4,
            day: 1,
        },
        DateCase {
            year: 5196,
            month: 11,
            day: 7,
        },
        DateCase {
            year: 5252,
            month: 1,
            day: 3,
        },
        DateCase {
            year: 5314,
            month: 7,
            day: 1,
        },
        DateCase {
            year: 5320,
            month: 12,
            day: 27,
        },
        DateCase {
            year: 5408,
            month: 3,
            day: 20,
        },
        DateCase {
            year: 5440,
            month: 4,
            day: 3,
        },
        DateCase {
            year: 5476,
            month: 5,
            day: 5,
        },
        DateCase {
            year: 5528,
            month: 4,
            day: 4,
        },
        DateCase {
            year: 5579,
            month: 5,
            day: 11,
        },
        DateCase {
            year: 5599,
            month: 1,
            day: 12,
        },
        DateCase {
            year: 5663,
            month: 1,
            day: 22,
        },
        DateCase {
            year: 5689,
            month: 5,
            day: 19,
        },
        DateCase {
            year: 5702,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 5703,
            month: 1,
            day: 14,
        },
        DateCase {
            year: 5704,
            month: 7,
            day: 8,
        },
        DateCase {
            year: 5752,
            month: 13,
            day: 12,
        },
        DateCase {
            year: 5756,
            month: 12,
            day: 5,
        },
        DateCase {
            year: 5799,
            month: 8,
            day: 12,
        },
        DateCase {
            year: 5854,
            month: 5,
            day: 5,
        },
    ];

    static EXPECTED_MOLAD_DATES: [f64; 33] = [
        -1850718767f64 / 8640f64,
        -1591805959f64 / 25920f64,
        660097927f64 / 25920f64,
        1275506059f64 / 25920f64,
        4439806081f64 / 25920f64,
        605235101f64 / 2880f64,
        3284237627f64 / 12960f64,
        9583515841f64 / 25920f64,
        2592403883f64 / 6480f64,
        2251656649f64 / 5184f64,
        11731320839f64 / 25920f64,
        12185988041f64 / 25920f64,
        6140833583f64 / 12960f64,
        6581722991f64 / 12960f64,
        6792982499f64 / 12960f64,
        4705980311f64 / 8640f64,
        14699670013f64 / 25920f64,
        738006961f64 / 1296f64,
        1949499007f64 / 3240f64,
        5299956319f64 / 8640f64,
        3248250415f64 / 5184f64,
        16732660061f64 / 25920f64,
        17216413717f64 / 25920f64,
        1087650871f64 / 1620f64,
        2251079609f64 / 3240f64,
        608605601f64 / 864f64,
        306216383f64 / 432f64,
        18387526207f64 / 25920f64,
        3678423761f64 / 5184f64,
        1570884431f64 / 2160f64,
        18888119389f64 / 25920f64,
        19292268013f64 / 25920f64,
        660655045f64 / 864f64,
    ];

    static EXPECTED_LAST_HEBREW_MONTH: [u8; 33] = [
        12, 12, 12, 12, 12, 12, 12, 12, 13, 12, 13, 12, 12, 12, 12, 13, 12, 13, 12, 13, 12, 12, 12,
        12, 12, 13, 12, 13, 12, 13, 12, 12, 12,
    ];

    static EXPECTED_HEBREW_SABBATICAL_YEAR: [bool; 33] = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, true, false, false, false, false, true, false, true, false,
        false, false, false, false, false, false, false,
    ];

    #[test]
    fn test_hebrew_epoch() {
        // page 119 of the Calendrical Calculations book
        let fixed_hebrew_date = -1373427.0;
        assert_eq!(FIXED_HEBREW_EPOCH.to_f64_date(), fixed_hebrew_date);
    }

    #[test]
    fn test_hebrew_molad() {
        let precision = 1_00000f64;
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_MOLAD_DATES.iter()) {
            let molad =
                (Hebrew::molad(case.year, case.month).inner() * precision).round() / precision;
            let final_expected = (expected * precision).round() / precision;
            assert_eq!(molad, final_expected, "{case:?}");
        }
    }

    #[test]
    fn test_last_hebrew_month() {
        for (case, expected) in HEBREW_DATES.iter().zip(EXPECTED_LAST_HEBREW_MONTH.iter()) {
            let last_month = Hebrew::last_month_of_hebrew_year(case.year);
            assert_eq!(last_month, *expected);
        }
    }

    #[test]
    fn test_hebrew_sabbatical_year() {
        for (case, expected) in HEBREW_DATES
            .iter()
            .zip(EXPECTED_HEBREW_SABBATICAL_YEAR.iter())
        {
            let boolean = Hebrew::hebrew_sabbatical_year(case.year);
            assert_eq!(boolean, *expected);
        }
    }
}
