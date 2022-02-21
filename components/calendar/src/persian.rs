use crate::{types, Calendar, Date, DateDuration, DateDurationUnit, Iso};
use tinystr::tinystr;

#[derive(Copy, Clone, Debug, Default)]
/// The Persian Calendar
pub struct Persian;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PersianDateInner {
    year: i32,
    month: u8,
    day: u8,
}

impl Persian {
    fn is_leap_year(year: i32) -> bool {
        (year * 25 + 11) % 33 < 8
    }

    fn day_of_year(date: &PersianDateInner) -> i32 {
        Persian::MONTH_PREFIX_SUM[date.month as usize - 1] + date.day as i32 - 1
    }

    const EPOCH: i32 = 226895;
    const MONTH_PREFIX_SUM: [i32; 12] = [0, 31, 62, 93, 124, 155, 186, 216, 246, 276, 306, 336];
}

impl Calendar for Persian {
    type DateInner = PersianDateInner;

    // Source: https://github.com/hhstechgroup/icu4j/blob/master/main/classes/core/src/com/ibm/icu/util/PersianCalendar.java
    fn date_from_iso(&self, iso: Date<Iso>) -> Self::DateInner {
        let days_since_epoch = Iso::fixed_from_iso(*iso.inner()) - Persian::EPOCH;
        let year = 1 + (33 * days_since_epoch + 3) / 12053;
        let farvardin1 = 365 * (year - 1) + (8 * year + 21) / 33;
        let day_of_year = days_since_epoch - farvardin1;
        let month = if day_of_year < 216 {
            day_of_year / 31
        } else {
            (day_of_year - 6) / 30
        } as u8;
        let day = day_of_year - Persian::MONTH_PREFIX_SUM[month as usize];
        let day = day as u8 + 1;
        let month = month + 1;
        PersianDateInner { day, month, year }
    }

    // Source: https://github.com/hhstechgroup/icu4j/blob/master/main/classes/core/src/com/ibm/icu/util/PersianCalendar.java
    fn date_to_iso(&self, date: &Self::DateInner) -> Date<Iso> {
        let day_of_year = Persian::day_of_year(date);
        let farvardin1 = 365 * (date.year - 1) + (8 * date.year + 21) / 33;
        Iso::iso_from_fixed(farvardin1 + day_of_year + Persian::EPOCH)
    }

    fn months_in_year(&self, _: &Self::DateInner) -> u8 {
        12
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u32 {
        if Persian::is_leap_year(date.year) {
            366
        } else {
            365
        }
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        match date.month {
            1..=6 => 31,
            7..=11 => 30,
            12 if Persian::is_leap_year(date.year) => 30,
            12 => 29,
            _ => 0, // invalid
        }
    }

    fn offset_date(&self, _: &mut Self::DateInner, _: DateDuration<Self>) {
        todo!()
    }

    fn until(
        &self,
        _: &Self::DateInner,
        _: &Self::DateInner,
        _: DateDurationUnit,
        _: DateDurationUnit,
    ) -> DateDuration<Self> {
        todo!()
    }

    fn debug_name() -> &'static str {
        "persian"
    }

    fn year(&self, date: &Self::DateInner) -> types::Year {
        let (era, number) = if date.year < 1 {
            (tinystr!(16, "bh"), -date.year + 1)
        } else {
            (tinystr!(16, "ah"), date.year)
        };
        let related_iso = date.year + 621;
        types::Year {
            era: types::Era(era),
            number,
            related_iso,
        }
    }

    fn month(&self, date: &Self::DateInner) -> types::Month {
        types::Month {
            number: date.month as u32,
            // TODO(#486): Implement month codes
            code: types::MonthCode(tinystr!(8, "TODO")),
        }
    }

    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        types::DayOfMonth(date.day as u32)
    }

    fn day_of_year_info(&self, date: &Self::DateInner) -> types::DayOfYearInfo {
        let day_of_year = Persian::day_of_year(date) as u32;
        let days_in_year = self.days_in_year(date);
        types::DayOfYearInfo {
            day_of_year,
            days_in_year,
            prev_year: self.year(&PersianDateInner {
                day: 1,
                month: 1,
                year: date.year - 1,
            }),
            days_in_prev_year: self.days_in_year(&PersianDateInner {
                day: 1,
                month: 1,
                year: date.year - 1,
            }),
            next_year: self.year(&PersianDateInner {
                day: 1,
                month: 1,
                year: date.year + 1,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Date, Iso};

    use super::{Persian, PersianDateInner};

    fn check_equiv(persian: u32, iso: u32) {
        let p_year = (persian / 10000) as i32;
        let p_month = ((persian / 100) % 100) as u8;
        let p_day = (persian % 100) as u8;

        let i_year = (iso / 10000) as i32;
        let i_month = ((iso / 100) % 100) as u8;
        let i_day = (iso % 100) as u8;

        let iso = Date::new_iso_date_from_integers(i_year, i_month, i_day).unwrap();
        let persian = Date::from_raw(
            PersianDateInner {
                day: p_day,
                month: p_month,
                year: p_year,
            },
            Persian,
        );
        assert_eq!(iso.to_calendar(Persian), persian);
        assert_eq!(iso, persian.to_calendar(Iso));
    }

    #[test]
    fn basic() {
        check_equiv(14001202, 20220221);
    }

    #[test]
    fn leaps() {
        check_equiv(14031230, 20250320);
        check_equiv(14081230, 20300320);
        check_equiv(13781210, 20000229);
        check_equiv(13941210, 20160229);
    }
}
