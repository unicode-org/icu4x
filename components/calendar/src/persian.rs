// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains types and implementations for the Persian calendar.
//!
//! ```rust
//! use icu::calendar::{Date, DateTime};
//!
//! // `PersianDate` type
//! let persian_date = Date::try_new_persian_date(1348, 10, 11)
//!     .expect("Failed to initialize Persian Date instance.");
//!
//! // `PersianDateTime` type
//! let persian_datetime = DateTime::try_new_persian_datetime(1348, 10, 11, 13, 1, 0)
//!     .expect("Failed to initialize Persian DateTime instance.");
//!
//! // `PersianDate` checks
//! assert_eq!(persian_date.year().number, 1348);
//! assert_eq!(persian_date.month().ordinal, 10);
//! assert_eq!(persian_date.day_of_month().0, 11);
//!
//! // `PersianDateTime` checks
//! assert_eq!(persian_datetime.date.year().number, 1348);
//! assert_eq!(persian_datetime.date.month().ordinal, 10);
//! assert_eq!(persian_datetime.date.day_of_month().0, 11);
//! assert_eq!(persian_datetime.time.hour.number(), 13);
//! assert_eq!(persian_datetime.time.minute.number(), 1);
//! assert_eq!(persian_datetime.time.second.number(), 0);
//! ```

use crate::any_calendar::AnyCalendarKind;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::helpers::{div_rem_euclid, div_rem_euclid64, i64_to_i32, I32Result};
use crate::iso::Iso;
use crate::julian::Julian;
use crate::rata_die::RataDie;
use crate::Gregorian;
use crate::{types, Calendar, CalendarError, Date, DateDuration, DateDurationUnit, DateTime};
use ::tinystr::tinystr;
use core::marker::PhantomData;

const FIXED_PERSIAN_EPOCH: RataDie = Julian::fixed_from_julian(ArithmeticDate {
    year: (622),
    month: (3),
    day: (19),
    marker: core::marker::PhantomData,
});
/// The Persian Calendar
///
/// The [Persian Calendar] is a solar calendar used officially by the countries of Iran and Afghanistan and many Persian-speaking regions.
/// It has 12 months and other similarities to the Gregorian Calendar
///
/// This type can be used with [`Date`] or [`DateTime`] to represent dates in this calendar.
///
/// [Persian Calendar]: https://en.wikipedia.org/wiki/Solar_Hijri_calendar
///
/// # Era codes
/// This calendar supports only one era code, which starts from the year of the Hijra, designated as "AH".
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Persian;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]

/// The inner date type used for representing [`Date`]s of [`Persian`]. See [`Date`] and [`Persian`] for more details.
pub struct PersianDateInner(ArithmeticDate<Persian>);

impl CalendarArithmetic for Persian {
    fn month_days(year: i32, month: u8) -> u8 {
        match month {
            1 | 2 | 3 | 4 | 5 | 6 => 31,
            7 | 8 | 9 | 10 | 11 => 30,
            12 if Self::is_leap_year(year) => 30,
            12 => 29,
            _ => 0,
        }
    }

    fn months_for_every_year(_: i32) -> u8 {
        12
    }

    fn is_leap_year(mut p_year: i32) -> bool {
        if 0 < p_year {
            p_year -= 474;
        } else {
            p_year -= 473;
        };
        let d = div_rem_euclid(p_year, 2820);
        let year = d.1 + 474;

        div_rem_euclid((year + 38) * 31, 128).1 < 31
    }

    fn days_in_provided_year(year: i32) -> u32 {
        if Self::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn last_month_day_in_year(year: i32) -> (u8, u8) {
        if Self::is_leap_year(year) {
            (12, 30)
        } else {
            (12, 29)
        }
    }
}

impl Calendar for Persian {
    type DateInner = PersianDateInner;
    fn date_from_codes(
        &self,
        era: types::Era,
        year: i32,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::DateInner, CalendarError> {
        let year = if era.0 == tinystr!(16, "ah") {
            if year <= 0 {
                return Err(CalendarError::OutOfRange);
            }
            year
        } else {
            return Err(CalendarError::UnknownEra(era.0, self.debug_name()));
        };

        ArithmeticDate::new_from_solar(self, year, month_code, day).map(PersianDateInner)
    }

    fn date_from_iso(&self, iso: Date<Iso>) -> PersianDateInner {
        let fixed_iso = Iso::fixed_from_iso(*iso.inner());
        Self::arithmetic_persian_from_fixed(fixed_iso).inner
    }

    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let fixed_persian = Persian::fixed_from_arithmetic_persian(*date);
        Iso::iso_from_fixed(fixed_persian)
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    fn day_of_week(&self, date: &Self::DateInner) -> types::IsoWeekday {
        Iso.day_of_week(self.date_to_iso(date).inner())
    }

    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration<Self>) {
        date.0.offset_date(offset)
    }

    #[allow(clippy::field_reassign_with_default)]
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        _largest_unit: DateDurationUnit,
        _smallest_unit: DateDurationUnit,
    ) -> DateDuration<Self> {
        date1.0.until(date2.0, _largest_unit, _smallest_unit)
    }

    fn year(&self, date: &Self::DateInner) -> types::FormattableYear {
        Self::year_as_persian(date.0.year)
    }

    fn month(&self, date: &Self::DateInner) -> types::FormattableMonth {
        date.0.solar_month()
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let prev_year = date.0.year - 1;
        let next_year = date.0.year + 1;
        types::DayOfYearInfo {
            day_of_year: date.0.day_of_year(),
            days_in_year: date.0.days_in_year(),
            prev_year: Persian::year_as_persian(prev_year),
            days_in_prev_year: Persian::days_in_year_direct(prev_year),
            next_year: Persian::year_as_persian(next_year),
        }
    }

    fn debug_name(&self) -> &'static str {
        "Persian"
    }
    // Missing any_calendar persian tests, the rest is completed
    fn any_calendar_kind(&self) -> Option<AnyCalendarKind> {
        Some(AnyCalendarKind::Persian)
    }
}

impl Persian {
    /// Constructs a new Persian Calendar
    pub fn new() -> Self {
        Self
    }

    // "Fixed" is a day count representation of calendars staring from Jan 1st of year 1 of the Georgian Calendar.
    // The fixed date algorithms are from
    // Dershowitz, Nachum, and Edward M. Reingold. _Calendrical calculations_. Cambridge University Press, 2008.
    //
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4803
    fn fixed_from_arithmetic_persian(p_date: PersianDateInner) -> RataDie {
        let p_year = i64::from(p_date.0.year);
        let month = i64::from(p_date.0.month);
        let day = i64::from(p_date.0.day);
        let y = if p_year > 0 {
            p_year - 474
        } else {
            p_year - 473
        };
        let x = div_rem_euclid64(y, 2820);
        let year = x.1 + 474;
        let z = div_rem_euclid64(31 * year - 5, 128);

        RataDie::new(
            FIXED_PERSIAN_EPOCH.to_i64_date() - 1
                + 1029983 * x.0
                + 365 * (year - 1)
                + z.0
                + if month <= 7 {
                    31 * (month - 1)
                } else {
                    30 * (month - 1) + 6
                }
                + day,
        )
    }
    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4857
    fn arithmetic_persian_from_fixed(date: RataDie) -> Date<Persian> {
        let year = Self::arithmetic_persian_year_from_fixed(date);
        let year = match i64_to_i32(year) {
            I32Result::BelowMin(_) => {
                return Date::from_raw(PersianDateInner(ArithmeticDate::min_date()), Persian)
            }
            I32Result::AboveMax(_) => {
                return Date::from_raw(PersianDateInner(ArithmeticDate::max_date()), Persian)
            }
            I32Result::WithinRange(y) => y,
        };
        #[allow(clippy::unwrap_used)] // valid month,day
        let day_of_year = (1_i64 + date.to_i64_date()
            - Self::fixed_from_persian_integers(year, 1, 1)
                .unwrap()
                .to_i64_date()) as i32;
        let month = if day_of_year <= 186 {
            libm::ceilf(day_of_year as f32 / 31.0) as u8
        } else {
            libm::ceilf((day_of_year as f32 - 6.0) / 30.0) as u8
        };
        #[allow(clippy::unwrap_used)] // month and day
        let day = (date - Self::fixed_from_persian_integers(year, month, 1).unwrap() + 1) as u8;
        #[allow(clippy::unwrap_used)] // valid month and day
        Date::try_new_persian_date(year, month, day).unwrap()
    }

    // Lisp code reference: https://github.com/EdReingold/calendar-code2/blob/main/calendar.l#L4829
    fn arithmetic_persian_year_from_fixed(date: RataDie) -> i64 {
        #[allow(clippy::unwrap_used)] // valid year,month,day
        let d0 = date - Self::fixed_from_persian_integers(475, 1, 1).unwrap();
        let d = div_rem_euclid64(d0, 1029983);
        let n2820 = d.0;
        let d1 = d.1;
        let y2820 = if d1 == 1029982 {
            2820
        } else {
            div_rem_euclid64(128 * d1 + 46878, 46751).0
        };
        let year = 474 + n2820 * 2820 + y2820;
        if year > 0 {
            year
        } else {
            year - 1
        }
    }

    pub(crate) fn fixed_from_persian_integers(year: i32, month: u8, day: u8) -> Option<RataDie> {
        Date::try_new_persian_date(year, month, day)
            .ok()
            .map(|d| *d.inner())
            .map(Self::fixed_from_arithmetic_persian)
    }

    // Persian New Year occuring in March of Gregorian year (g_year) to fixed date
    fn nowruz(g_year: i32) -> RataDie {
        let iso_from_fixed: Date<Iso> =
            Iso::iso_from_fixed(RataDie::new(FIXED_PERSIAN_EPOCH.to_i64_date()));
        let greg_date_from_fixed: Date<Gregorian> = Date::new_from_iso(iso_from_fixed, Gregorian);
        let persian_year = g_year - greg_date_from_fixed.year().number + 1;
        let _year = if persian_year <= 0 {
            persian_year - 1
        } else {
            persian_year
        };
        #[allow(clippy::unwrap_used)] // valid day and month
        Self::fixed_from_persian_integers(_year, 1, 1).unwrap()
    }

    fn days_in_year_direct(year: i32) -> u32 {
        if Persian::is_leap_year(year) {
            366
        } else {
            365
        }
    }

    fn days_in_provided_year_core(year: i32) -> u32 {
        let fixed_year = Self::fixed_from_persian_integers(year, 1, 1)
            .unwrap()
            .to_i64_date();
        let next_fixed_year = Self::fixed_from_persian_integers(year + 1, 1, 1)
            .unwrap()
            .to_i64_date();

        (next_fixed_year - fixed_year) as u32
    }

    fn year_as_persian(year: i32) -> types::FormattableYear {
        if year > 0 {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "ah")),
                number: year,
                related_iso: None,
            }
        } else {
            types::FormattableYear {
                era: types::Era(tinystr!(16, "bh")),
                number: year,
                related_iso: None,
            }
        }
    }
}

impl Date<Persian> {
    /// Construct new Persian Date.
    ///
    /// Has no negative years, only era is the AH/AP.
    ///
    /// ```rust
    /// use icu::calendar::Date;
    ///
    /// let date_persian = Date::try_new_persian_date(1392, 4, 25)
    ///     .expect("Failed to initialize Persian Date instance.");
    ///
    /// assert_eq!(date_persian.year().number, 1392);
    /// assert_eq!(date_persian.month().ordinal, 4);
    /// assert_eq!(date_persian.day_of_month().0, 25);
    /// ```
    pub fn try_new_persian_date(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Date<Persian>, CalendarError> {
        let inner = ArithmeticDate {
            year,
            month,
            day,
            marker: PhantomData,
        };

        let bound = inner.days_in_month();
        if day > bound {
            return Err(CalendarError::OutOfRange);
        }
        Ok(Date::from_raw(PersianDateInner(inner), Persian))
    }
}

impl DateTime<Persian> {
    /// Construct a new Persian datetime from integers.
    ///
    /// ```rust
    /// use icu::calendar::DateTime;
    ///
    /// let datetime_persian = DateTime::try_new_persian_datetime(474, 10, 11, 13, 1, 0)
    ///     .expect("Failed to initialize Persian DateTime instance.");
    ///
    /// assert_eq!(datetime_persian.date.year().number, 474);
    /// assert_eq!(datetime_persian.date.month().ordinal, 10);
    /// assert_eq!(datetime_persian.date.day_of_month().0, 11);
    /// assert_eq!(datetime_persian.time.hour.number(), 13);
    /// assert_eq!(datetime_persian.time.minute.number(), 1);
    /// assert_eq!(datetime_persian.time.second.number(), 0);
    /// ```
    pub fn try_new_persian_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<DateTime<Persian>, CalendarError> {
        Ok(DateTime {
            date: Date::try_new_persian_date(year, month, day)?,
            time: types::Time::try_new(hour, minute, second, 0)?,
        })
    }
}
mod tests {

    #[cfg(test)]
    use super::*;
    #[derive(Debug)]
    struct DateCase {
        year: i32,
        month: u8,
        day: u8,
    }

    static FIXED_DATE: [i64; 33] = [
        -214193, -61387, 25469, 49217, 171307, 210155, 253427, 369740, 400085, 434355, 452605,
        470160, 473837, 507850, 524156, 544676, 567118, 569477, 601716, 613424, 626596, 645554,
        664224, 671401, 694799, 704424, 708842, 709409, 709580, 727274, 728714, 744313, 764652,
    ];

    static CASES: [DateCase; 33] = [
        DateCase {
            year: -1208,
            month: 5,
            day: 1,
        },
        DateCase {
            year: -790,
            month: 9,
            day: 14,
        },
        DateCase {
            year: -552,
            month: 7,
            day: 2,
        },
        DateCase {
            year: -487,
            month: 7,
            day: 9,
        },
        DateCase {
            year: -153,
            month: 10,
            day: 18,
        },
        DateCase {
            year: -46,
            month: 2,
            day: 30,
        },
        DateCase {
            year: 73,
            month: 8,
            day: 19,
        },
        DateCase {
            year: 392,
            month: 2,
            day: 5,
        },
        DateCase {
            year: 475,
            month: 3,
            day: 3,
        },
        DateCase {
            year: 569,
            month: 1,
            day: 3,
        },
        DateCase {
            year: 618,
            month: 12,
            day: 20,
        },
        DateCase {
            year: 667,
            month: 1,
            day: 14,
        },
        DateCase {
            year: 677,
            month: 2,
            day: 8,
        },
        DateCase {
            year: 770,
            month: 3,
            day: 22,
        },
        DateCase {
            year: 814,
            month: 11,
            day: 13,
        },
        DateCase {
            year: 871,
            month: 1,
            day: 21,
        },
        DateCase {
            year: 932,
            month: 6,
            day: 28,
        },
        DateCase {
            year: 938,
            month: 12,
            day: 14,
        },
        DateCase {
            year: 1027,
            month: 3,
            day: 21,
        },
        DateCase {
            year: 1059,
            month: 4,
            day: 10,
        },
        DateCase {
            year: 1095,
            month: 5,
            day: 2,
        },
        DateCase {
            year: 1147,
            month: 3,
            day: 30,
        },
        DateCase {
            year: 1198,
            month: 5,
            day: 10,
        },
        DateCase {
            year: 1218,
            month: 1,
            day: 7,
        },
        DateCase {
            year: 1282,
            month: 1,
            day: 29,
        },
        DateCase {
            year: 1308,
            month: 6,
            day: 3,
        },
        DateCase {
            year: 1320,
            month: 7,
            day: 7,
        },
        DateCase {
            year: 1322,
            month: 1,
            day: 29,
        },
        DateCase {
            year: 1322,
            month: 7,
            day: 14,
        },
        DateCase {
            year: 1370,
            month: 12,
            day: 27,
        },
        DateCase {
            year: 1374,
            month: 12,
            day: 6,
        },
        DateCase {
            year: 1417,
            month: 8,
            day: 19,
        },
        DateCase {
            year: 1473,
            month: 4,
            day: 28,
        },
    ];

    #[test]
    fn test_persian_leap_year() {
        let mut leap_years: [i32; 33] = [0; 33];
        let expected_values = [
            false, true, false, false, false, false, false, true, false, true, false, false, true,
            false, false, true, false, false, false, false, false, false, false, true, false,
            false, false, false, false, true, false, false, false,
        ];

        let leap_year_cycle: [i32; 2820] = [
            474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490,
            491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504, 505, 506, 507,
            508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 521, 522, 523, 524,
            525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537, 538, 539, 540, 541,
            542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558,
            559, 560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575,
            576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592,
            593, 594, 595, 596, 597, 598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609,
            610, 611, 612, 613, 614, 615, 616, 617, 618, 619, 620, 621, 622, 623, 624, 625, 626,
            627, 628, 629, 630, 631, 632, 633, 634, 635, 636, 637, 638, 639, 640, 641, 642, 643,
            644, 645, 646, 647, 648, 649, 650, 651, 652, 653, 654, 655, 656, 657, 658, 659, 660,
            661, 662, 663, 664, 665, 666, 667, 668, 669, 670, 671, 672, 673, 674, 675, 676, 677,
            678, 679, 680, 681, 682, 683, 684, 685, 686, 687, 688, 689, 690, 691, 692, 693, 694,
            695, 696, 697, 698, 699, 700, 701, 702, 703, 704, 705, 706, 707, 708, 709, 710, 711,
            712, 713, 714, 715, 716, 717, 718, 719, 720, 721, 722, 723, 724, 725, 726, 727, 728,
            729, 730, 731, 732, 733, 734, 735, 736, 737, 738, 739, 740, 741, 742, 743, 744, 745,
            746, 747, 748, 749, 750, 751, 752, 753, 754, 755, 756, 757, 758, 759, 760, 761, 762,
            763, 764, 765, 766, 767, 768, 769, 770, 771, 772, 773, 774, 775, 776, 777, 778, 779,
            780, 781, 782, 783, 784, 785, 786, 787, 788, 789, 790, 791, 792, 793, 794, 795, 796,
            797, 798, 799, 800, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 811, 812, 813,
            814, 815, 816, 817, 818, 819, 820, 821, 822, 823, 824, 825, 826, 827, 828, 829, 830,
            831, 832, 833, 834, 835, 836, 837, 838, 839, 840, 841, 842, 843, 844, 845, 846, 847,
            848, 849, 850, 851, 852, 853, 854, 855, 856, 857, 858, 859, 860, 861, 862, 863, 864,
            865, 866, 867, 868, 869, 870, 871, 872, 873, 874, 875, 876, 877, 878, 879, 880, 881,
            882, 883, 884, 885, 886, 887, 888, 889, 890, 891, 892, 893, 894, 895, 896, 897, 898,
            899, 900, 901, 902, 903, 904, 905, 906, 907, 908, 909, 910, 911, 912, 913, 914, 915,
            916, 917, 918, 919, 920, 921, 922, 923, 924, 925, 926, 927, 928, 929, 930, 931, 932,
            933, 934, 935, 936, 937, 938, 939, 940, 941, 942, 943, 944, 945, 946, 947, 948, 949,
            950, 951, 952, 953, 954, 955, 956, 957, 958, 959, 960, 961, 962, 963, 964, 965, 966,
            967, 968, 969, 970, 971, 972, 973, 974, 975, 976, 977, 978, 979, 980, 981, 982, 983,
            984, 985, 986, 987, 988, 989, 990, 991, 992, 993, 994, 995, 996, 997, 998, 999, 1000,
            1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 1013, 1014,
            1015, 1016, 1017, 1018, 1019, 1020, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028,
            1029, 1030, 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042,
            1043, 1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056,
            1057, 1058, 1059, 1060, 1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070,
            1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084,
            1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098,
            1099, 1100, 1101, 1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1110, 1111, 1112,
            1113, 1114, 1115, 1116, 1117, 1118, 1119, 1120, 1121, 1122, 1123, 1124, 1125, 1126,
            1127, 1128, 1129, 1130, 1131, 1132, 1133, 1134, 1135, 1136, 1137, 1138, 1139, 1140,
            1141, 1142, 1143, 1144, 1145, 1146, 1147, 1148, 1149, 1150, 1151, 1152, 1153, 1154,
            1155, 1156, 1157, 1158, 1159, 1160, 1161, 1162, 1163, 1164, 1165, 1166, 1167, 1168,
            1169, 1170, 1171, 1172, 1173, 1174, 1175, 1176, 1177, 1178, 1179, 1180, 1181, 1182,
            1183, 1184, 1185, 1186, 1187, 1188, 1189, 1190, 1191, 1192, 1193, 1194, 1195, 1196,
            1197, 1198, 1199, 1200, 1201, 1202, 1203, 1204, 1205, 1206, 1207, 1208, 1209, 1210,
            1211, 1212, 1213, 1214, 1215, 1216, 1217, 1218, 1219, 1220, 1221, 1222, 1223, 1224,
            1225, 1226, 1227, 1228, 1229, 1230, 1231, 1232, 1233, 1234, 1235, 1236, 1237, 1238,
            1239, 1240, 1241, 1242, 1243, 1244, 1245, 1246, 1247, 1248, 1249, 1250, 1251, 1252,
            1253, 1254, 1255, 1256, 1257, 1258, 1259, 1260, 1261, 1262, 1263, 1264, 1265, 1266,
            1267, 1268, 1269, 1270, 1271, 1272, 1273, 1274, 1275, 1276, 1277, 1278, 1279, 1280,
            1281, 1282, 1283, 1284, 1285, 1286, 1287, 1288, 1289, 1290, 1291, 1292, 1293, 1294,
            1295, 1296, 1297, 1298, 1299, 1300, 1301, 1302, 1303, 1304, 1305, 1306, 1307, 1308,
            1309, 1310, 1311, 1312, 1313, 1314, 1315, 1316, 1317, 1318, 1319, 1320, 1321, 1322,
            1323, 1324, 1325, 1326, 1327, 1328, 1329, 1330, 1331, 1332, 1333, 1334, 1335, 1336,
            1337, 1338, 1339, 1340, 1341, 1342, 1343, 1344, 1345, 1346, 1347, 1348, 1349, 1350,
            1351, 1352, 1353, 1354, 1355, 1356, 1357, 1358, 1359, 1360, 1361, 1362, 1363, 1364,
            1365, 1366, 1367, 1368, 1369, 1370, 1371, 1372, 1373, 1374, 1375, 1376, 1377, 1378,
            1379, 1380, 1381, 1382, 1383, 1384, 1385, 1386, 1387, 1388, 1389, 1390, 1391, 1392,
            1393, 1394, 1395, 1396, 1397, 1398, 1399, 1400, 1401, 1402, 1403, 1404, 1405, 1406,
            1407, 1408, 1409, 1410, 1411, 1412, 1413, 1414, 1415, 1416, 1417, 1418, 1419, 1420,
            1421, 1422, 1423, 1424, 1425, 1426, 1427, 1428, 1429, 1430, 1431, 1432, 1433, 1434,
            1435, 1436, 1437, 1438, 1439, 1440, 1441, 1442, 1443, 1444, 1445, 1446, 1447, 1448,
            1449, 1450, 1451, 1452, 1453, 1454, 1455, 1456, 1457, 1458, 1459, 1460, 1461, 1462,
            1463, 1464, 1465, 1466, 1467, 1468, 1469, 1470, 1471, 1472, 1473, 1474, 1475, 1476,
            1477, 1478, 1479, 1480, 1481, 1482, 1483, 1484, 1485, 1486, 1487, 1488, 1489, 1490,
            1491, 1492, 1493, 1494, 1495, 1496, 1497, 1498, 1499, 1500, 1501, 1502, 1503, 1504,
            1505, 1506, 1507, 1508, 1509, 1510, 1511, 1512, 1513, 1514, 1515, 1516, 1517, 1518,
            1519, 1520, 1521, 1522, 1523, 1524, 1525, 1526, 1527, 1528, 1529, 1530, 1531, 1532,
            1533, 1534, 1535, 1536, 1537, 1538, 1539, 1540, 1541, 1542, 1543, 1544, 1545, 1546,
            1547, 1548, 1549, 1550, 1551, 1552, 1553, 1554, 1555, 1556, 1557, 1558, 1559, 1560,
            1561, 1562, 1563, 1564, 1565, 1566, 1567, 1568, 1569, 1570, 1571, 1572, 1573, 1574,
            1575, 1576, 1577, 1578, 1579, 1580, 1581, 1582, 1583, 1584, 1585, 1586, 1587, 1588,
            1589, 1590, 1591, 1592, 1593, 1594, 1595, 1596, 1597, 1598, 1599, 1600, 1601, 1602,
            1603, 1604, 1605, 1606, 1607, 1608, 1609, 1610, 1611, 1612, 1613, 1614, 1615, 1616,
            1617, 1618, 1619, 1620, 1621, 1622, 1623, 1624, 1625, 1626, 1627, 1628, 1629, 1630,
            1631, 1632, 1633, 1634, 1635, 1636, 1637, 1638, 1639, 1640, 1641, 1642, 1643, 1644,
            1645, 1646, 1647, 1648, 1649, 1650, 1651, 1652, 1653, 1654, 1655, 1656, 1657, 1658,
            1659, 1660, 1661, 1662, 1663, 1664, 1665, 1666, 1667, 1668, 1669, 1670, 1671, 1672,
            1673, 1674, 1675, 1676, 1677, 1678, 1679, 1680, 1681, 1682, 1683, 1684, 1685, 1686,
            1687, 1688, 1689, 1690, 1691, 1692, 1693, 1694, 1695, 1696, 1697, 1698, 1699, 1700,
            1701, 1702, 1703, 1704, 1705, 1706, 1707, 1708, 1709, 1710, 1711, 1712, 1713, 1714,
            1715, 1716, 1717, 1718, 1719, 1720, 1721, 1722, 1723, 1724, 1725, 1726, 1727, 1728,
            1729, 1730, 1731, 1732, 1733, 1734, 1735, 1736, 1737, 1738, 1739, 1740, 1741, 1742,
            1743, 1744, 1745, 1746, 1747, 1748, 1749, 1750, 1751, 1752, 1753, 1754, 1755, 1756,
            1757, 1758, 1759, 1760, 1761, 1762, 1763, 1764, 1765, 1766, 1767, 1768, 1769, 1770,
            1771, 1772, 1773, 1774, 1775, 1776, 1777, 1778, 1779, 1780, 1781, 1782, 1783, 1784,
            1785, 1786, 1787, 1788, 1789, 1790, 1791, 1792, 1793, 1794, 1795, 1796, 1797, 1798,
            1799, 1800, 1801, 1802, 1803, 1804, 1805, 1806, 1807, 1808, 1809, 1810, 1811, 1812,
            1813, 1814, 1815, 1816, 1817, 1818, 1819, 1820, 1821, 1822, 1823, 1824, 1825, 1826,
            1827, 1828, 1829, 1830, 1831, 1832, 1833, 1834, 1835, 1836, 1837, 1838, 1839, 1840,
            1841, 1842, 1843, 1844, 1845, 1846, 1847, 1848, 1849, 1850, 1851, 1852, 1853, 1854,
            1855, 1856, 1857, 1858, 1859, 1860, 1861, 1862, 1863, 1864, 1865, 1866, 1867, 1868,
            1869, 1870, 1871, 1872, 1873, 1874, 1875, 1876, 1877, 1878, 1879, 1880, 1881, 1882,
            1883, 1884, 1885, 1886, 1887, 1888, 1889, 1890, 1891, 1892, 1893, 1894, 1895, 1896,
            1897, 1898, 1899, 1900, 1901, 1902, 1903, 1904, 1905, 1906, 1907, 1908, 1909, 1910,
            1911, 1912, 1913, 1914, 1915, 1916, 1917, 1918, 1919, 1920, 1921, 1922, 1923, 1924,
            1925, 1926, 1927, 1928, 1929, 1930, 1931, 1932, 1933, 1934, 1935, 1936, 1937, 1938,
            1939, 1940, 1941, 1942, 1943, 1944, 1945, 1946, 1947, 1948, 1949, 1950, 1951, 1952,
            1953, 1954, 1955, 1956, 1957, 1958, 1959, 1960, 1961, 1962, 1963, 1964, 1965, 1966,
            1967, 1968, 1969, 1970, 1971, 1972, 1973, 1974, 1975, 1976, 1977, 1978, 1979, 1980,
            1981, 1982, 1983, 1984, 1985, 1986, 1987, 1988, 1989, 1990, 1991, 1992, 1993, 1994,
            1995, 1996, 1997, 1998, 1999, 2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008,
            2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022,
            2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036,
            2037, 2038, 2039, 2040, 2041, 2042, 2043, 2044, 2045, 2046, 2047, 2048, 2049, 2050,
            2051, 2052, 2053, 2054, 2055, 2056, 2057, 2058, 2059, 2060, 2061, 2062, 2063, 2064,
            2065, 2066, 2067, 2068, 2069, 2070, 2071, 2072, 2073, 2074, 2075, 2076, 2077, 2078,
            2079, 2080, 2081, 2082, 2083, 2084, 2085, 2086, 2087, 2088, 2089, 2090, 2091, 2092,
            2093, 2094, 2095, 2096, 2097, 2098, 2099, 2100, 2101, 2102, 2103, 2104, 2105, 2106,
            2107, 2108, 2109, 2110, 2111, 2112, 2113, 2114, 2115, 2116, 2117, 2118, 2119, 2120,
            2121, 2122, 2123, 2124, 2125, 2126, 2127, 2128, 2129, 2130, 2131, 2132, 2133, 2134,
            2135, 2136, 2137, 2138, 2139, 2140, 2141, 2142, 2143, 2144, 2145, 2146, 2147, 2148,
            2149, 2150, 2151, 2152, 2153, 2154, 2155, 2156, 2157, 2158, 2159, 2160, 2161, 2162,
            2163, 2164, 2165, 2166, 2167, 2168, 2169, 2170, 2171, 2172, 2173, 2174, 2175, 2176,
            2177, 2178, 2179, 2180, 2181, 2182, 2183, 2184, 2185, 2186, 2187, 2188, 2189, 2190,
            2191, 2192, 2193, 2194, 2195, 2196, 2197, 2198, 2199, 2200, 2201, 2202, 2203, 2204,
            2205, 2206, 2207, 2208, 2209, 2210, 2211, 2212, 2213, 2214, 2215, 2216, 2217, 2218,
            2219, 2220, 2221, 2222, 2223, 2224, 2225, 2226, 2227, 2228, 2229, 2230, 2231, 2232,
            2233, 2234, 2235, 2236, 2237, 2238, 2239, 2240, 2241, 2242, 2243, 2244, 2245, 2246,
            2247, 2248, 2249, 2250, 2251, 2252, 2253, 2254, 2255, 2256, 2257, 2258, 2259, 2260,
            2261, 2262, 2263, 2264, 2265, 2266, 2267, 2268, 2269, 2270, 2271, 2272, 2273, 2274,
            2275, 2276, 2277, 2278, 2279, 2280, 2281, 2282, 2283, 2284, 2285, 2286, 2287, 2288,
            2289, 2290, 2291, 2292, 2293, 2294, 2295, 2296, 2297, 2298, 2299, 2300, 2301, 2302,
            2303, 2304, 2305, 2306, 2307, 2308, 2309, 2310, 2311, 2312, 2313, 2314, 2315, 2316,
            2317, 2318, 2319, 2320, 2321, 2322, 2323, 2324, 2325, 2326, 2327, 2328, 2329, 2330,
            2331, 2332, 2333, 2334, 2335, 2336, 2337, 2338, 2339, 2340, 2341, 2342, 2343, 2344,
            2345, 2346, 2347, 2348, 2349, 2350, 2351, 2352, 2353, 2354, 2355, 2356, 2357, 2358,
            2359, 2360, 2361, 2362, 2363, 2364, 2365, 2366, 2367, 2368, 2369, 2370, 2371, 2372,
            2373, 2374, 2375, 2376, 2377, 2378, 2379, 2380, 2381, 2382, 2383, 2384, 2385, 2386,
            2387, 2388, 2389, 2390, 2391, 2392, 2393, 2394, 2395, 2396, 2397, 2398, 2399, 2400,
            2401, 2402, 2403, 2404, 2405, 2406, 2407, 2408, 2409, 2410, 2411, 2412, 2413, 2414,
            2415, 2416, 2417, 2418, 2419, 2420, 2421, 2422, 2423, 2424, 2425, 2426, 2427, 2428,
            2429, 2430, 2431, 2432, 2433, 2434, 2435, 2436, 2437, 2438, 2439, 2440, 2441, 2442,
            2443, 2444, 2445, 2446, 2447, 2448, 2449, 2450, 2451, 2452, 2453, 2454, 2455, 2456,
            2457, 2458, 2459, 2460, 2461, 2462, 2463, 2464, 2465, 2466, 2467, 2468, 2469, 2470,
            2471, 2472, 2473, 2474, 2475, 2476, 2477, 2478, 2479, 2480, 2481, 2482, 2483, 2484,
            2485, 2486, 2487, 2488, 2489, 2490, 2491, 2492, 2493, 2494, 2495, 2496, 2497, 2498,
            2499, 2500, 2501, 2502, 2503, 2504, 2505, 2506, 2507, 2508, 2509, 2510, 2511, 2512,
            2513, 2514, 2515, 2516, 2517, 2518, 2519, 2520, 2521, 2522, 2523, 2524, 2525, 2526,
            2527, 2528, 2529, 2530, 2531, 2532, 2533, 2534, 2535, 2536, 2537, 2538, 2539, 2540,
            2541, 2542, 2543, 2544, 2545, 2546, 2547, 2548, 2549, 2550, 2551, 2552, 2553, 2554,
            2555, 2556, 2557, 2558, 2559, 2560, 2561, 2562, 2563, 2564, 2565, 2566, 2567, 2568,
            2569, 2570, 2571, 2572, 2573, 2574, 2575, 2576, 2577, 2578, 2579, 2580, 2581, 2582,
            2583, 2584, 2585, 2586, 2587, 2588, 2589, 2590, 2591, 2592, 2593, 2594, 2595, 2596,
            2597, 2598, 2599, 2600, 2601, 2602, 2603, 2604, 2605, 2606, 2607, 2608, 2609, 2610,
            2611, 2612, 2613, 2614, 2615, 2616, 2617, 2618, 2619, 2620, 2621, 2622, 2623, 2624,
            2625, 2626, 2627, 2628, 2629, 2630, 2631, 2632, 2633, 2634, 2635, 2636, 2637, 2638,
            2639, 2640, 2641, 2642, 2643, 2644, 2645, 2646, 2647, 2648, 2649, 2650, 2651, 2652,
            2653, 2654, 2655, 2656, 2657, 2658, 2659, 2660, 2661, 2662, 2663, 2664, 2665, 2666,
            2667, 2668, 2669, 2670, 2671, 2672, 2673, 2674, 2675, 2676, 2677, 2678, 2679, 2680,
            2681, 2682, 2683, 2684, 2685, 2686, 2687, 2688, 2689, 2690, 2691, 2692, 2693, 2694,
            2695, 2696, 2697, 2698, 2699, 2700, 2701, 2702, 2703, 2704, 2705, 2706, 2707, 2708,
            2709, 2710, 2711, 2712, 2713, 2714, 2715, 2716, 2717, 2718, 2719, 2720, 2721, 2722,
            2723, 2724, 2725, 2726, 2727, 2728, 2729, 2730, 2731, 2732, 2733, 2734, 2735, 2736,
            2737, 2738, 2739, 2740, 2741, 2742, 2743, 2744, 2745, 2746, 2747, 2748, 2749, 2750,
            2751, 2752, 2753, 2754, 2755, 2756, 2757, 2758, 2759, 2760, 2761, 2762, 2763, 2764,
            2765, 2766, 2767, 2768, 2769, 2770, 2771, 2772, 2773, 2774, 2775, 2776, 2777, 2778,
            2779, 2780, 2781, 2782, 2783, 2784, 2785, 2786, 2787, 2788, 2789, 2790, 2791, 2792,
            2793, 2794, 2795, 2796, 2797, 2798, 2799, 2800, 2801, 2802, 2803, 2804, 2805, 2806,
            2807, 2808, 2809, 2810, 2811, 2812, 2813, 2814, 2815, 2816, 2817, 2818, 2819, 2820,
            2821, 2822, 2823, 2824, 2825, 2826, 2827, 2828, 2829, 2830, 2831, 2832, 2833, 2834,
            2835, 2836, 2837, 2838, 2839, 2840, 2841, 2842, 2843, 2844, 2845, 2846, 2847, 2848,
            2849, 2850, 2851, 2852, 2853, 2854, 2855, 2856, 2857, 2858, 2859, 2860, 2861, 2862,
            2863, 2864, 2865, 2866, 2867, 2868, 2869, 2870, 2871, 2872, 2873, 2874, 2875, 2876,
            2877, 2878, 2879, 2880, 2881, 2882, 2883, 2884, 2885, 2886, 2887, 2888, 2889, 2890,
            2891, 2892, 2893, 2894, 2895, 2896, 2897, 2898, 2899, 2900, 2901, 2902, 2903, 2904,
            2905, 2906, 2907, 2908, 2909, 2910, 2911, 2912, 2913, 2914, 2915, 2916, 2917, 2918,
            2919, 2920, 2921, 2922, 2923, 2924, 2925, 2926, 2927, 2928, 2929, 2930, 2931, 2932,
            2933, 2934, 2935, 2936, 2937, 2938, 2939, 2940, 2941, 2942, 2943, 2944, 2945, 2946,
            2947, 2948, 2949, 2950, 2951, 2952, 2953, 2954, 2955, 2956, 2957, 2958, 2959, 2960,
            2961, 2962, 2963, 2964, 2965, 2966, 2967, 2968, 2969, 2970, 2971, 2972, 2973, 2974,
            2975, 2976, 2977, 2978, 2979, 2980, 2981, 2982, 2983, 2984, 2985, 2986, 2987, 2988,
            2989, 2990, 2991, 2992, 2993, 2994, 2995, 2996, 2997, 2998, 2999, 3000, 3001, 3002,
            3003, 3004, 3005, 3006, 3007, 3008, 3009, 3010, 3011, 3012, 3013, 3014, 3015, 3016,
            3017, 3018, 3019, 3020, 3021, 3022, 3023, 3024, 3025, 3026, 3027, 3028, 3029, 3030,
            3031, 3032, 3033, 3034, 3035, 3036, 3037, 3038, 3039, 3040, 3041, 3042, 3043, 3044,
            3045, 3046, 3047, 3048, 3049, 3050, 3051, 3052, 3053, 3054, 3055, 3056, 3057, 3058,
            3059, 3060, 3061, 3062, 3063, 3064, 3065, 3066, 3067, 3068, 3069, 3070, 3071, 3072,
            3073, 3074, 3075, 3076, 3077, 3078, 3079, 3080, 3081, 3082, 3083, 3084, 3085, 3086,
            3087, 3088, 3089, 3090, 3091, 3092, 3093, 3094, 3095, 3096, 3097, 3098, 3099, 3100,
            3101, 3102, 3103, 3104, 3105, 3106, 3107, 3108, 3109, 3110, 3111, 3112, 3113, 3114,
            3115, 3116, 3117, 3118, 3119, 3120, 3121, 3122, 3123, 3124, 3125, 3126, 3127, 3128,
            3129, 3130, 3131, 3132, 3133, 3134, 3135, 3136, 3137, 3138, 3139, 3140, 3141, 3142,
            3143, 3144, 3145, 3146, 3147, 3148, 3149, 3150, 3151, 3152, 3153, 3154, 3155, 3156,
            3157, 3158, 3159, 3160, 3161, 3162, 3163, 3164, 3165, 3166, 3167, 3168, 3169, 3170,
            3171, 3172, 3173, 3174, 3175, 3176, 3177, 3178, 3179, 3180, 3181, 3182, 3183, 3184,
            3185, 3186, 3187, 3188, 3189, 3190, 3191, 3192, 3193, 3194, 3195, 3196, 3197, 3198,
            3199, 3200, 3201, 3202, 3203, 3204, 3205, 3206, 3207, 3208, 3209, 3210, 3211, 3212,
            3213, 3214, 3215, 3216, 3217, 3218, 3219, 3220, 3221, 3222, 3223, 3224, 3225, 3226,
            3227, 3228, 3229, 3230, 3231, 3232, 3233, 3234, 3235, 3236, 3237, 3238, 3239, 3240,
            3241, 3242, 3243, 3244, 3245, 3246, 3247, 3248, 3249, 3250, 3251, 3252, 3253, 3254,
            3255, 3256, 3257, 3258, 3259, 3260, 3261, 3262, 3263, 3264, 3265, 3266, 3267, 3268,
            3269, 3270, 3271, 3272, 3273, 3274, 3275, 3276, 3277, 3278, 3279, 3280, 3281, 3282,
            3283, 3284, 3285, 3286, 3287, 3288, 3289, 3290, 3291, 3292, 3293,
        ];

        let mut leap_year_results: Vec<bool> = Vec::new();
        for year in leap_year_cycle {
            let r = Persian::is_leap_year(year);
            if r {
                leap_year_results.push(r);
            }
        }
        // 683 is the amount of leap years in a 2820 Persian year cycle
        assert_eq!(leap_year_results.len(), 683);

        let mut index = 0;
        for case in CASES.iter() {
            leap_years[index] = case.year;
            index += 1;
        }
        for (year, bool) in leap_years.iter().zip(expected_values.iter()) {
            assert_eq!(Persian::is_leap_year(*year), *bool);
        }
    }

    #[test]
    fn days_in_provided_year_test() {
        for case in CASES.iter() {
            assert_eq!(
                Persian::days_in_provided_year_core(case.year),
                Persian::days_in_provided_year(case.year)
            );
        }
    }

    #[test]
    fn test_persian_year_from_fixed() {
        for (case, f_date) in CASES.iter().zip(FIXED_DATE.iter()) {
            let date = PersianDateInner(ArithmeticDate {
                year: (case.year),
                month: (case.month),
                day: (case.day),
                marker: (PhantomData),
            });
            assert_eq!(
                date.0.year as i64,
                Persian::arithmetic_persian_year_from_fixed(RataDie::new(*f_date)),
                "{case:?}"
            );
        }
    }
    #[test]
    fn test_fixed_from_persian() {
        for (case, f_date) in CASES.iter().zip(FIXED_DATE.iter()) {
            let date = PersianDateInner(ArithmeticDate {
                year: (case.year),
                month: (case.month),
                day: (case.day),
                marker: (PhantomData),
            });

            assert_eq!(
                Persian::fixed_from_arithmetic_persian(date).to_i64_date(),
                *f_date,
                "{case:?}"
            );
        }
    }
    #[test]
    fn test_persian_from_fixed() {
        for (case, f_date) in CASES.iter().zip(FIXED_DATE.iter()) {
            let date = Date::try_new_persian_date(case.year, case.month, case.day).unwrap();
            assert_eq!(
                Persian::arithmetic_persian_from_fixed(RataDie::new(*f_date)),
                date,
                "{case:?}"
            );
        }
    }
    #[test]
    fn test_persian_epoch() {
        let epoch = FIXED_PERSIAN_EPOCH.to_i64_date();
        // Iso year of Persian Epoch
        let epoch_year_from_fixed = Iso::iso_from_fixed(RataDie::new(epoch)).inner.0.year;
        // 622 is the correct ISO year for the Persian Epoch
        assert_eq!(epoch_year_from_fixed, 622);
    }

    #[test]
    fn test_nowruz() {
        let fixed_date = Persian::nowruz(622).to_i64_date();
        assert_eq!(fixed_date, FIXED_PERSIAN_EPOCH.to_i64_date());

        let years = [
            2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013,
            2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024, 2025, 2026, 2027,
            2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036, 2037, 2038, 2039, 2040, 2041,
            2042, 2043, 2044, 2045, 2046, 2047, 2048, 2049, 2050, 2051, 2052, 2053, 2054, 2055,
            2056, 2057, 2058, 2059, 2060, 2061, 2062, 2063, 2064, 2065, 2066, 2067, 2068, 2069,
            2070, 2071, 2072, 2073, 2074, 2075, 2076, 2077, 2078, 2079, 2080, 2081, 2082, 2083,
            2084, 2085, 2086, 2087, 2088, 2089, 2090, 2091, 2092, 2093, 2094, 2095, 2096, 2097,
            2098, 2099, 2100, 2101, 2101, 2102, 2103,
        ];

        for year in years {
            let two_thousand_eight_to_fixed = Persian::nowruz(year).to_i64_date();
            let iso_date = Date::try_new_iso_date(year, 3, 21).unwrap();
            let persian_year =
                Persian::arithmetic_persian_year_from_fixed(Iso::fixed_from_iso(iso_date.inner));
            assert_eq!(
                Persian::arithmetic_persian_year_from_fixed(RataDie::new(
                    two_thousand_eight_to_fixed
                )),
                persian_year
            );
        }
    }
}
