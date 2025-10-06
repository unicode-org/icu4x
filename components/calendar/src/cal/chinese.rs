// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cal::iso::{Iso, IsoDateInner};
use crate::calendar_arithmetic::{ArithmeticDate, ArithmeticDateBuilder, CalendarArithmetic};
use crate::calendar_arithmetic::{DateFieldsResolver, PrecomputedDataSource};
use crate::error::DateError;
use crate::options::{DateFromFieldsOptions, Overflow};
use crate::provider::chinese_based::PackedChineseBasedYearInfo;
use crate::types::{MonthCode, MonthInfo};
use crate::AsCalendar;
use crate::{types, Calendar, Date, DateDuration, DateDurationUnit};
use calendrical_calculations::chinese_based::{
    self, ChineseBased, YearBounds, WELL_BEHAVED_ASTRONOMICAL_RANGE,
};
use calendrical_calculations::rata_die::RataDie;
use icu_locale_core::preferences::extensions::unicode::keywords::CalendarAlgorithm;
use icu_provider::prelude::*;
use tinystr::tinystr;

#[path = "chinese/china_data.rs"]
mod china_data;
#[path = "chinese/korea_data.rs"]
mod korea_data;
#[path = "chinese/qing_data.rs"]
mod qing_data;

/// The [Chinese Calendar](https://en.wikipedia.org/wiki/Chinese_calendar)
///
/// The Chinese Calendar is a lunisolar calendar used traditionally in China as well as in other
/// countries particularly in East Asia. It is often used today to track important
/// cultural events and holidays like the Chinese Lunar New Year.
///
/// This type can be used with [`Date`] to represent dates in the Chinese calendar.
///
/// # Months
///
/// The Chinese calendar is an astronomical calendar which uses the phases of the moon to track months.
/// Each month starts on the date of the new moon, meaning that months last 29 or 30 days.
///
/// One year in the Chinese calendar is typically 12 lunar months; however, because 12 lunar months does
/// not line up to one solar year, the Chinese calendar will add an intercalary leap month approximately
/// every three years to keep Chinese calendar months in line with the solar year.
///
/// Leap months can happen after any month; the month in which a leap month occurs is based on the alignment
/// of months with 24 solar terms into which the solar year is divided.
///
/// # Year and Era codes
///
/// Unlike the Gregorian calendar, the Chinese calendar does not traditionally count years in an infinitely
/// increasing sequence. Instead, 10 "celestial stems" and 12 "terrestrial branches" are combined to form a
/// cycle of year names which repeats every 60 years. However, for the purposes of calendar calculations and
/// conversions, this calendar also counts years based on the Gregorian (ISO) calendar. This "related ISO year"
/// marks the Gregorian year in which a Chinese year begins.
///
/// Because the Chinese calendar does not traditionally count years, era codes are not used in this calendar.
///
/// For more information, suggested reading materials include:
/// * _Calendrical Calculations_ by Reingold & Dershowitz
/// * _The Mathematics of the Chinese Calendar_ by Helmer Aslaksen <https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.139.9311&rep=rep1&type=pdf>
/// * Wikipedia: <https://en.wikipedia.org/wiki/Chinese_calendar>
///
/// # Month codes
///
/// This calendar is a lunisolar calendar. It supports regular month codes `"M01" - "M12"` as well
/// as leap month codes `"M01L" - "M12L"`.
///
/// This calendar is currently in a preview state: formatting for this calendar is not
/// going to be perfect.
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct LunarChinese<X>(pub X);

/// The rules for the [`LunarChinese`] calendar.
///
/// The calendar depends on both astronomical calculations and local time.
/// The rules for how to perform these calculations, as well as how local
/// time is determined differ between countries and have changed over time.
///
/// This crate currently provides [`Rules`] for [`China`] and [`Korea`].
///
/// <div class="stab unstable">
/// 🚫 This trait is sealed; it should not be implemented by user code. If an API requests an item that implements this
/// trait, please consider using a type from the implementors listed below.
///
/// It is still possible to implement this trait in userland (since `UnstableSealed` is public),
/// do not do so unless you are prepared for things to occasionally break.
/// </div>
pub trait Rules: Clone + core::fmt::Debug + crate::cal::scaffold::UnstableSealed {
    /// Returns data about the given year.
    fn year_data(&self, related_iso: i32) -> LunarChineseYearData;

    /// Returns an ECMA reference year that contains the given month-day combination.
    ///
    /// If the day is out of range, it will return a year that contains the given month
    /// and the maximum day possible for that month. See [the spec][spec] for the
    /// precise algorithm used.
    ///
    /// This API only matters when using [`MissingFieldsStrategy::Ecma`] to compute
    /// a date without providing a year in [`Date::try_from_fields()`]. The default impl
    /// will just error, and custom calendars who do not care about ECMA/Temporal
    /// reference years do not need to override this.
    ///
    /// [spec]: https://tc39.es/proposal-temporal/#sec-temporal-nonisomonthdaytoisoreferencedate
    /// [`MissingFieldsStrategy::Ecma`]: crate::options::MissingFieldsStrategy::Ecma
    fn ecma_reference_year(
        &self,
        _month_code: types::MonthCode,
        _day: u8,
    ) -> Result<i32, DateError> {
        Err(DateError::NotEnoughFields)
    }

    /// The debug name for the calendar defined by these [`Rules`].
    fn debug_name(&self) -> &'static str {
        "Chinese (custom)"
    }

    /// The BCP-47 [`CalendarAlgorithm`] for the calendar defined by these [`Rules`], if defined.
    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        None
    }
}

/// The [`Rules`] used in China.
///
/// This type agrees with the official data published by the
/// [Purple Mountain Observatory for the years 1900-2025], as well as with
/// the data published by the [Hong Kong Observatory for the years 1901-2100].
///
/// For years since 1912, this uses the [GB/T 33661-2017] rules.
/// As accurate computation is computationally expensive, years until
/// 2100 are precomputed. If performance beyond 2100 is an issue, clients
/// can implement their own [`Rules`] type containing more precomputed data.
/// We note that the calendar is inherently uncertain for some future dates.
///
/// Before 1912 [different rules](https://ytliu.epizy.com/Shixian/Shixian_summary.html)
/// were used. This type produces correct data for the years 1900-1912, and
/// falls back to [GB/T 33661-2017] proleptically before 1900. If accuracy
/// is required before 1900, clients can implement their own [`Rules`] type
/// using data such as from the excellent compilation by [Yuk Tung Liu].
///
/// [Purple Mountain Observatory for the years 1900-2025]: http://www.pmo.cas.cn/xwdt2019/kpdt2019/202203/P020250414456381274062.pdf
/// [Hong Kong Observatory for the years 1901-2100]: https://www.hko.gov.hk/en/gts/time/conversion.htm
/// [GB/T 33661-2017]: China::gb_t_33661_2017
/// [Yuk Tung Liu]: https://ytliu0.github.io/ChineseCalendar/table.html
#[derive(Copy, Clone, Debug, Default)]
#[non_exhaustive]
pub struct China;

impl China {
    /// Computes [`LunarChineseYearData`] according to [GB/T 33661-2017],
    /// as implemented by [`calendrical_calculations::chinese_based::Chinese`].
    ///
    /// The rules specified in [GB/T 33661-2017] have only been used
    /// since 1912, applying them proleptically to years before 1912 will not
    /// necessarily match historical calendars.
    ///
    /// Note that for future years there is a small degree of uncertainty, as
    /// [GB/T 33661-2017] depends on the uncertain future [difference between UT1
    /// and UTC](https://en.wikipedia.org/wiki/Leap_second#Future).
    /// As noted by
    /// [Yuk Tung Liu](https://ytliu0.github.io/ChineseCalendar/computation.html#modern),
    /// years as early as 2057, 2089, and 2097 have lunar events very close to
    /// local midnight, which might affect the start of a (single) month if additional
    /// leap seconds are introduced.
    ///
    /// [GB/T 33661-2017]: https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=E107EA4DE9725EDF819F33C60A44B296
    pub fn gb_t_33661_2017(related_iso: i32) -> LunarChineseYearData {
        LunarChineseYearData::calendrical_calculations::<chinese_based::Chinese>(related_iso)
    }
}

impl crate::cal::scaffold::UnstableSealed for China {}
impl Rules for China {
    fn year_data(&self, related_iso: i32) -> LunarChineseYearData {
        if let Some(data) = china_data::DATA.get(related_iso) {
            data
        } else if related_iso > china_data::DATA.first_related_iso_year {
            Self::gb_t_33661_2017(related_iso)
        } else {
            qing_data::DATA
                .get(related_iso)
                .unwrap_or_else(|| Self::gb_t_33661_2017(related_iso))
        }
    }

    fn ecma_reference_year(&self, month_code: types::MonthCode, day: u8) -> Result<i32, DateError> {
        let Some((number, is_leap)) = month_code.parsed() else {
            return Err(DateError::UnknownMonthCode(month_code));
        };
        // Computed by `generate_reference_years`
        Ok(match (number, is_leap, day > 29) {
            (1, false, false) => 1972,
            (1, false, true) => 1970,
            (1, true, false) => 1651,
            (1, true, true) => 1461,
            (2, false, false) => 1972,
            (2, false, true) => 1972,
            (2, true, false) => 1947,
            (2, true, true) => 1765,
            (3, false, false) => 1972,
            (3, false, true) => 1966,
            (3, true, false) => 1966,
            (3, true, true) => 1955,
            (4, false, false) => 1972,
            (4, false, true) => 1970,
            (4, true, false) => 1963,
            (4, true, true) => 1944,
            (5, false, false) => 1972,
            (5, false, true) => 1972,
            (5, true, false) => 1971,
            (5, true, true) => 1952,
            (6, false, false) => 1972,
            (6, false, true) => 1971,
            (6, true, false) => 1960,
            (6, true, true) => 1941,
            (7, false, false) => 1972,
            (7, false, true) => 1972,
            (7, true, false) => 1968,
            (7, true, true) => 1938,
            (8, false, false) => 1972,
            (8, false, true) => 1971,
            (8, true, false) => 1957,
            (8, true, true) => 1718,
            (9, false, false) => 1972,
            (9, false, true) => 1972,
            (9, true, false) => 2014,
            (9, true, true) => -5738,
            (10, false, false) => 1972,
            (10, false, true) => 1972,
            (10, true, false) => 1984,
            (10, true, true) => -4098,
            // Dec 31, 1972 is 1972-M11-26, dates after that
            // are in the next year
            (11, false, false) if day > 26 => 1971,
            (11, false, false) => 1972,
            (11, false, true) => 1969,
            (11, true, false) => 2033,
            (11, true, true) => -2173,
            (12, false, false) => 1971,
            (12, false, true) => 1971,
            (12, true, false) => 1403,
            (12, true, true) => -180,
            _ => return Err(DateError::UnknownMonthCode(month_code)),
        })
    }

    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        Some(CalendarAlgorithm::Chinese)
    }

    fn debug_name(&self) -> &'static str {
        "Chinese"
    }
}

/// The [`Rules`] used in [Korea](https://en.wikipedia.org/wiki/Korean_calendar).
///
/// This type agrees with the official data published by the
/// [Korea Astronomy and Space Science Institute for the years 1900-2050].
///
/// For years since 1912, this uses [adapted GB/T 33661-2017] rules,
/// using Korea time instead of Beijing Time.
/// As accurate computation is computationally expensive, years until
/// 2100 are precomputed. If performance beyond 2100 is an issue, clients
/// can implement their own [`Rules`] type containing more precomputed data.
/// We note that the calendar is inherently uncertain for some future dates.
///
/// Before 1912 [different rules](https://ytliu.epizy.com/Shixian/Shixian_summary.html)
/// were used (those of Qing-dynasty China). This type produces correct data
/// for the years 1900-1912, and falls back to [GB/T 33661-2017] proleptically
/// before 1900. If accuracy is required before 1900, clients can implement
/// their own [`Rules`] type using data such as from the excellent compilation
/// by [Yuk Tung Liu].
///
/// [Korea Astronomy and Space Science Institute for the years 1900-2050]: https://astro.kasi.re.kr/life/pageView/5
/// [adapted GB/T 33661-2017]: Korea::adapted_gb_t_33661_2017
/// [GB/T 33661-2017]: China::gb_t_33661_2017
/// [Yuk Tung Liu]: https://ytliu0.github.io/ChineseCalendar/table.html
///
/// ```rust
/// use icu::calendar::cal::LunarChinese;
/// use icu::calendar::Date;
/// use tinystr::tinystr;
///
/// let iso_a = Date::try_new_iso(2012, 4, 23).unwrap();
/// let korean_a = iso_a.to_calendar(LunarChinese::new_korea());
/// let chinese_a = iso_a.to_calendar(LunarChinese::new_china());
///
/// assert_eq!(korean_a.month().standard_code.0, tinystr!(4, "M03L"));
/// assert_eq!(chinese_a.month().standard_code.0, tinystr!(4, "M04"));
///
/// let iso_b = Date::try_new_iso(2012, 5, 23).unwrap();
/// let korean_b = iso_b.to_calendar(LunarChinese::new_korea());
/// let chinese_b = iso_b.to_calendar(LunarChinese::new_china());
///
/// assert_eq!(korean_b.month().standard_code.0, tinystr!(4, "M04"));
/// assert_eq!(chinese_b.month().standard_code.0, tinystr!(4, "M04L"));
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct Korea;

impl Korea {
    /// A version of [`China::gb_t_33661_2017`] adapted for Korean time.
    ///
    /// See [`China::gb_t_33661_2017`] for caveats.
    pub fn adapted_gb_t_33661_2017(related_iso: i32) -> LunarChineseYearData {
        LunarChineseYearData::calendrical_calculations::<chinese_based::Dangi>(related_iso)
    }
}

impl LunarChinese<Korea> {
    /// Creates a new calendar using [`Korea`] rules.
    pub const fn new_korea() -> Self {
        Self(Korea)
    }

    /// Use [`Self::new_korea`].
    #[deprecated(since = "2.1.0", note = "use `Self::new_korea()")]
    pub const fn new() -> Self {
        Self::new_korea()
    }

    /// Use [`Self::new_korea`].
    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::new)]
    #[deprecated(since = "2.1.0", note = "use `Self::new_korea()")]
    pub fn try_new_with_buffer_provider(
        _provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Ok(Self::new_korea())
    }

    /// Use [`Self::new_korea`].
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    #[deprecated(since = "2.1.0", note = "use `Self::new_korea()")]
    pub fn try_new_unstable<D: ?Sized>(_provider: &D) -> Result<Self, DataError> {
        Ok(Self::new_korea())
    }

    /// Use [`Self::new_korea`].
    #[deprecated(since = "2.1.0", note = "use `Self::new_korea()")]
    pub fn new_always_calculating() -> Self {
        Self::new_korea()
    }
}

impl crate::cal::scaffold::UnstableSealed for Korea {}
impl Rules for Korea {
    fn year_data(&self, related_iso: i32) -> LunarChineseYearData {
        if let Some(data) = korea_data::DATA.get(related_iso) {
            data
        } else if related_iso > korea_data::DATA.first_related_iso_year {
            Self::adapted_gb_t_33661_2017(related_iso)
        } else {
            // Korea used Qing-dynasty rules before 1912
            // https://github.com/unicode-org/icu4x/issues/6455#issuecomment-3282175550
            qing_data::DATA
                .get(related_iso)
                .unwrap_or_else(|| China::gb_t_33661_2017(related_iso))
        }
    }

    fn ecma_reference_year(&self, month_code: types::MonthCode, day: u8) -> Result<i32, DateError> {
        let Some((number, is_leap)) = month_code.parsed() else {
            return Err(DateError::UnknownMonthCode(month_code));
        };
        // Computed by `generate_reference_years`
        Ok(match (number, is_leap, day > 29) {
            (1, false, false) => 1972,
            (1, false, true) => 1970,
            (1, true, false) => 1651,
            (1, true, true) => 1461,
            (2, false, false) => 1972,
            (2, false, true) => 1972,
            (2, true, false) => 1947,
            (2, true, true) => 1765,
            (3, false, false) => 1972,
            (3, false, true) => 1968,
            (3, true, false) => 1966,
            (3, true, true) => 1955,
            (4, false, false) => 1972,
            (4, false, true) => 1970,
            (4, true, false) => 1963,
            (4, true, true) => 1944,
            (5, false, false) => 1972,
            (5, false, true) => 1972,
            (5, true, false) => 1971,
            (5, true, true) => 1952,
            (6, false, false) => 1972,
            (6, false, true) => 1971,
            (6, true, false) => 1960,
            (6, true, true) => 1941,
            (7, false, false) => 1972,
            (7, false, true) => 1972,
            (7, true, false) => 1968,
            (7, true, true) => 1938,
            (8, false, false) => 1972,
            (8, false, true) => 1971,
            (8, true, false) => 1957,
            (8, true, true) => 1718,
            (9, false, false) => 1972,
            (9, false, true) => 1972,
            (9, true, false) => 2014,
            (9, true, true) => -5738,
            (10, false, false) => 1972,
            (10, false, true) => 1972,
            (10, true, false) => 1984,
            (10, true, true) => -4098,
            // Dec 31, 1972 is 1972-M11-26, dates after that
            // are in the next year
            (11, false, false) if day > 26 => 1971,
            (11, false, false) => 1972,
            (11, false, true) => 1969,
            (11, true, false) => 2033,
            (11, true, true) => -2173,
            (12, false, false) => 1971,
            (12, false, true) => 1971,
            (12, true, false) => 1403,
            (12, true, true) => -180,
            _ => return Err(DateError::UnknownMonthCode(month_code)),
        })
    }

    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        Some(CalendarAlgorithm::Dangi)
    }
    fn debug_name(&self) -> &'static str {
        "Korean"
    }
}

impl<A: AsCalendar<Calendar = LunarChinese<Korea>>> Date<A> {
    /// Use [`Date::try_new_chinese_with_calendar`]
    #[deprecated(since = "2.1.0", note = "use `Date::try_new_chinese_with_calendar`")]
    pub fn try_new_dangi_with_calendar(
        related_iso_year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, DateError> {
        Date::try_new_chinese_with_calendar(related_iso_year, month, day, calendar)
    }
}

/// The inner date type used for representing [`Date`]s of [`LunarChinese`].
#[derive(Debug, Clone)]
pub struct ChineseDateInner<R: Rules>(ArithmeticDate<LunarChinese<R>>);

impl<S: Rules> Copy for ChineseDateInner<S> {}
impl<S: Rules> PartialEq for ChineseDateInner<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<S: Rules> Eq for ChineseDateInner<S> {}

impl LunarChinese<China> {
    /// Creates a new calendar using [`China`] rules.
    pub const fn new_china() -> Self {
        LunarChinese(China)
    }

    /// Use [`Self::new_china()`].
    #[deprecated(since = "2.1.0", note = "use `Self::new_china()")]
    pub const fn new() -> Self {
        Self::new_china()
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::new)]
    #[deprecated(since = "2.1.0", note = "use `Self::new_china()")]
    pub fn try_new_with_buffer_provider(
        _provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Ok(Self::new_china())
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    #[deprecated(since = "2.1.0", note = "use `Self::new_china()")]
    pub fn try_new_unstable<D: ?Sized>(_provider: &D) -> Result<Self, DataError> {
        Ok(Self::new_china())
    }

    /// Use [`Self::new_china()`].
    #[deprecated(since = "2.1.0", note = "use `Self::new_china()")]
    pub fn new_always_calculating() -> Self {
        Self::new_china()
    }
}

impl<R: Rules> CalendarArithmetic for LunarChinese<R> {
    type YearInfo = LunarChineseYearData;

    fn days_in_provided_month(year: LunarChineseYearData, month: u8) -> u8 {
        year.days_in_month(month)
    }

    /// Returns the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    fn months_in_provided_year(year: LunarChineseYearData) -> u8 {
        year.months_in_year()
    }

    /// Returns true if the given year is a leap year, and false if not.
    fn provided_year_is_leap(year: LunarChineseYearData) -> bool {
        year.leap_month().is_some()
    }

    /// Returns the (month, day) of the last day in a Chinese year (the day before Chinese New Year).
    /// The last month in a year will always be 12 in a common year or 13 in a leap year. The day is
    /// determined by finding the day immediately before the next new year and calculating the number
    /// of days since the last new moon (beginning of the last month in the year).
    fn last_month_day_in_provided_year(year: LunarChineseYearData) -> (u8, u8) {
        if year.leap_month().is_some() {
            (13, year.days_in_month(13))
        } else {
            (12, year.days_in_month(12))
        }
    }

    fn days_in_provided_year(year: LunarChineseYearData) -> u16 {
        year.days_in_year()
    }
}

impl<R: Rules> PrecomputedDataSource<LunarChineseYearData> for LunarChinese<R> {
    fn load_or_compute_info(&self, related_iso: i32) -> LunarChineseYearData {
        self.0.year_data(related_iso)
    }
}

impl<R: Rules> DateFieldsResolver for LunarChinese<R> {
    type YearInfo = LunarChineseYearData;

    #[inline]
    fn year_info_from_era(&self, _era: &str, _era_year: i32) -> Result<Self::YearInfo, DateError> {
        // This calendar has no era codes
        Err(DateError::UnknownEra)
    }

    #[inline]
    fn year_info_from_extended(&self, extended_year: i32) -> Self::YearInfo {
        self.load_or_compute_info(extended_year)
    }

    #[inline]
    fn reference_year_from_month_day(
        &self,
        month_code: types::MonthCode,
        day: u8,
    ) -> Result<Self::YearInfo, DateError> {
        Ok(self
            .0
            .year_data(self.0.ecma_reference_year(month_code, day)?))
    }

    fn ordinal_month_from_code(
        &self,
        year: &Self::YearInfo,
        month_code: types::MonthCode,
        options: DateFromFieldsOptions,
    ) -> Result<u8, DateError> {
        match year.parse_month_code(month_code) {
            ComputedOrdinalMonth::Exact(val) => Ok(val),
            ComputedOrdinalMonth::LeapToNormal(val)
                if options.overflow == Some(Overflow::Constrain) =>
            {
                Ok(val)
            }
            _ => Err(DateError::UnknownMonthCode(month_code)),
        }
    }
}

impl<R: Rules> crate::cal::scaffold::UnstableSealed for LunarChinese<R> {}
impl<R: Rules> Calendar for LunarChinese<R> {
    type DateInner = ChineseDateInner<R>;
    type Year = types::CyclicYear;

    fn from_fields(
        &self,
        fields: types::DateFields,
        options: DateFromFieldsOptions,
    ) -> Result<Self::DateInner, DateError> {
        let builder = ArithmeticDateBuilder::try_from_fields(fields, self, options)?;
        Ok(ChineseDateInner(ArithmeticDate::try_from_builder(
            builder, options,
        )?))
    }

    fn from_rata_die(&self, rd: RataDie) -> Self::DateInner {
        let iso = Iso.from_rata_die(rd);
        let y = {
            let actual_iso = iso.0.extended_year();
            let candidate = self.0.year_data(actual_iso);

            if rd >= candidate.new_year() {
                candidate
            } else {
                self.0.year_data(actual_iso - 1)
            }
        };
        let (m, d) = y.md_from_rd(rd);
        ChineseDateInner(ArithmeticDate::new_unchecked(y, m, d))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> RataDie {
        date.0.year.rd_from_md(date.0.month, date.0.day)
    }

    fn from_iso(&self, iso: IsoDateInner) -> Self::DateInner {
        let rd = Iso.to_rata_die(&iso);
        let y = {
            let actual_iso = iso.0.extended_year();
            let candidate = self.0.year_data(actual_iso);

            if rd >= candidate.new_year() {
                candidate
            } else {
                self.0.year_data(actual_iso - 1)
            }
        };
        let (m, d) = y.md_from_rd(rd);
        ChineseDateInner(ArithmeticDate::new_unchecked(y, m, d))
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        Iso.from_rata_die(self.to_rata_die(date))
    }

    // Count the number of months in a given year, specified by providing a date
    // from that year
    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        date.0.days_in_year()
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        date.0.days_in_month()
    }

    #[doc(hidden)] // unstable
    fn offset_date(&self, date: &mut Self::DateInner, offset: DateDuration) {
        date.0.offset_date(offset, self);
    }

    #[doc(hidden)] // unstable
    /// Calculate `date2 - date` as a duration
    ///
    /// `calendar2` is the calendar object associated with `date2`. In case the specific calendar objects
    /// differ on date, the date for the first calendar is used, and `date2` may be converted if necessary.
    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        _calendar2: &Self,
        _largest_unit: DateDurationUnit,
        _smallest_unit: DateDurationUnit,
    ) -> DateDuration {
        date1.0.until(date2.0, _largest_unit, _smallest_unit)
    }

    /// Obtain a name for the calendar for debug printing
    fn debug_name(&self) -> &'static str {
        self.0.debug_name()
    }

    fn year_info(&self, date: &Self::DateInner) -> Self::Year {
        let year = date.0.year;
        types::CyclicYear {
            year: (year.related_iso - 4).rem_euclid(60) as u8 + 1,
            related_iso: year.related_iso,
        }
    }

    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        Self::provided_year_is_leap(date.0.year)
    }

    /// The calendar-specific month code represented by `date`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal months 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        date.0.year.month(date.0.month)
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        date.0.day_of_month()
    }

    /// Information of the day of the year
    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        types::DayOfYear(date.0.year.day_of_year(date.0.month, date.0.day))
    }

    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        self.0.calendar_algorithm()
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        date.0.months_in_year()
    }
}

impl<R: Rules, A: AsCalendar<Calendar = LunarChinese<R>>> Date<A> {
    /// Construct a new Chinese date from a `year`, `month`, and `day`.
    /// `year` represents the [ISO](crate::Iso) year that roughly matches the Chinese year;
    /// `month` represents the month of the year ordinally (ex. if it is a leap year, the last month will be 13, not 12);
    /// `day` indicates the day of month
    ///
    /// This date will not use any precomputed calendrical calculations,
    /// one that loads such data from a provider will be added in the future (#3933)
    ///
    /// ```rust
    /// use icu::calendar::{cal::Chinese, Date};
    ///
    /// let chinese = Chinese::new_china();
    ///
    /// let date_chinese =
    ///     Date::try_new_chinese_with_calendar(2023, 6, 11, chinese)
    ///         .expect("Failed to initialize Chinese Date instance.");
    ///
    /// assert_eq!(date_chinese.cyclic_year().related_iso, 2023);
    /// assert_eq!(date_chinese.cyclic_year().year, 40);
    /// assert_eq!(date_chinese.month().ordinal, 6);
    /// assert_eq!(date_chinese.day_of_month().0, 11);
    /// ```
    pub fn try_new_chinese_with_calendar(
        related_iso_year: i32,
        month: u8,
        day: u8,
        calendar: A,
    ) -> Result<Date<A>, DateError> {
        let year = calendar.as_calendar().0.year_data(related_iso_year);
        year.validate_md(month, day)?;
        Ok(Date::from_raw(
            ChineseDateInner(ArithmeticDate::new_unchecked(year, month, day)),
            calendar,
        ))
    }
}

/// Information about a [`LunarChinese`] year.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// TODO(#3933): potentially make this smaller
pub struct LunarChineseYearData {
    /// Contains:
    /// - length of each month in the year
    /// - whether or not there is a leap month, and which month it is
    /// - the date of Chinese New Year in the related ISO year
    pub(crate) packed: PackedChineseBasedYearInfo,
    pub(crate) related_iso: i32,
}

impl From<LunarChineseYearData> for i32 {
    fn from(value: LunarChineseYearData) -> Self {
        value.related_iso
    }
}

impl LunarChineseYearData {
    /// Creates [`LunarChineseYearData`] from the given parts.
    ///
    /// `start_day` is the date for the first day of the year, see [`Date::to_rata_die`]
    /// to obtain a [`RataDie`] from a [`Date`] in an arbitrary calendar.
    ///
    /// `leap_month` is the ordinal number of the leap month, for example if a year
    /// has months 1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, the `leap_month`
    /// would be `Some(4)`.
    ///
    /// `month_lengths[n - 1]` is true if the nth month has 30 days, and false otherwise.
    /// The leap month does not necessarily have the same number of days as the previous
    /// month, which is why this has length 13. In non-leap years, the last value is ignored.
    pub fn new(
        related_iso: i32,
        start_day: RataDie,
        month_lengths: [bool; 13],
        leap_month: Option<u8>,
    ) -> Self {
        Self {
            packed: PackedChineseBasedYearInfo::new(
                related_iso,
                month_lengths,
                leap_month,
                start_day,
            ),
            related_iso,
        }
    }

    fn calendrical_calculations<CB: ChineseBased>(related_iso: i32) -> LunarChineseYearData {
        let mid_year = calendrical_calculations::gregorian::fixed_from_gregorian(related_iso, 7, 1);
        let year_bounds = YearBounds::compute::<CB>(mid_year);

        let YearBounds {
            new_year,
            next_new_year,
            ..
        } = year_bounds;
        let (month_lengths, leap_month) =
            chinese_based::month_structure_for_year::<CB>(new_year, next_new_year);

        LunarChineseYearData {
            packed: PackedChineseBasedYearInfo::new(
                related_iso,
                month_lengths,
                leap_month,
                new_year,
            ),
            related_iso,
        }
    }

    /// Get the new year R.D.    
    pub(crate) fn new_year(self) -> RataDie {
        self.packed.new_year(self.related_iso)
    }

    /// Get the next new year R.D.
    pub(crate) fn next_new_year(self) -> RataDie {
        self.new_year() + i64::from(self.days_in_year())
    }

    /// Get which month is the leap month. This produces the month *number*
    /// that is the leap month (not the ordinal month). In other words, for
    /// a year with an M05L, this will return Some(5). Note that the regular month precedes
    /// the leap month.
    pub(crate) fn leap_month(self) -> Option<u8> {
        self.packed.leap_month()
    }

    /// The last day of year in the previous month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    pub(crate) fn last_day_of_previous_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        // Get the last day of the previous month.
        // Since `month` is 1-indexed, this needs to check if the month is 1 for the zero case
        if month == 1 {
            0
        } else {
            self.packed.last_day_of_month(month - 1)
        }
    }

    pub(crate) fn days_in_year(self) -> u16 {
        self.last_day_of_month(self.months_in_year())
    }

    /// Return the number of months in a given year, which is 13 in a leap year, and 12 in a common year.
    pub(crate) fn months_in_year(self) -> u8 {
        if self.leap_month().is_some() {
            13
        } else {
            12
        }
    }

    /// The last day of year in the current month.
    /// `month` is 1-indexed, and the returned value is also
    /// a 1-indexed day of year
    ///
    /// Will be zero for the first month as the last day of the previous month
    /// is not in this year
    pub(crate) fn last_day_of_month(self, month: u8) -> u16 {
        debug_assert!((1..=13).contains(&month), "Month out of bounds!");
        self.packed.last_day_of_month(month)
    }

    pub(crate) fn days_in_month(self, month: u8) -> u8 {
        if self.packed.month_has_30_days(month) {
            30
        } else {
            29
        }
    }

    pub(crate) fn md_from_rd(self, rd: RataDie) -> (u8, u8) {
        debug_assert!(
            rd < self.next_new_year() || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Stored date {rd:?} out of bounds!"
        );
        // 1-indexed day of year
        let day_of_year = u16::try_from(rd - self.new_year() + 1);
        debug_assert!(
            day_of_year.is_ok() || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Somehow got a very large year in data"
        );
        let day_of_year = day_of_year.unwrap_or(1);
        let mut month = 1;
        // TODO(#3933) perhaps use a binary search
        for iter_month in 1..=13 {
            month = iter_month;
            if self.last_day_of_month(iter_month) >= day_of_year {
                break;
            }
        }

        debug_assert!((1..=13).contains(&month), "Month out of bounds!");

        debug_assert!(
            month < 13
                || self.leap_month().is_some()
                || !WELL_BEHAVED_ASTRONOMICAL_RANGE.contains(&rd),
            "Cannot have 13 months in a non-leap year!"
        );
        let day_before_month_start = self.last_day_of_previous_month(month);
        let day_of_month = day_of_year - day_before_month_start;
        let day_of_month = u8::try_from(day_of_month);
        debug_assert!(day_of_month.is_ok(), "Month too big!");
        let day_of_month = day_of_month.unwrap_or(1);

        (month, day_of_month)
    }

    pub(crate) fn rd_from_md(self, month: u8, day: u8) -> RataDie {
        self.new_year() + self.day_of_year(month, day) as i64 - 1
    }

    /// Calculate the number of days in the year so far for a ChineseBasedDate;
    /// similar to `CalendarArithmetic::day_of_year`
    pub(crate) fn day_of_year(self, month: u8, day: u8) -> u16 {
        self.last_day_of_previous_month(month) + day as u16
    }

    /// The calendar-specific month code represented by `month`;
    /// since the Chinese calendar has leap months, an "L" is appended to the month code for
    /// leap months. For example, in a year where an intercalary month is added after the second
    /// month, the month codes for ordinal months 1, 2, 3, 4, 5 would be "M01", "M02", "M02L", "M03", "M04".
    pub(crate) fn month(self, month: u8) -> MonthInfo {
        // 1 indexed leap month name. This is also the ordinal for the leap month
        // in the year (e.g. in `M01, M01L, M02, ..`, the leap month is for month 1, and it is also
        // ordinally `month 2`, zero-indexed)
        // 14 is a sentinel value
        let leap_month = self.leap_month().unwrap_or(14);
        let code_inner = if leap_month == month {
            // Month cannot be 1 because a year cannot have a leap month before the first actual month,
            // and the maximum num of months ina leap year is 13.
            debug_assert!((2..=13).contains(&month));
            match month {
                2 => tinystr!(4, "M01L"),
                3 => tinystr!(4, "M02L"),
                4 => tinystr!(4, "M03L"),
                5 => tinystr!(4, "M04L"),
                6 => tinystr!(4, "M05L"),
                7 => tinystr!(4, "M06L"),
                8 => tinystr!(4, "M07L"),
                9 => tinystr!(4, "M08L"),
                10 => tinystr!(4, "M09L"),
                11 => tinystr!(4, "M10L"),
                12 => tinystr!(4, "M11L"),
                13 => tinystr!(4, "M12L"),
                _ => tinystr!(4, "und"),
            }
        } else {
            let mut adjusted_ordinal = month;
            if month > leap_month {
                // Before adjusting for leap month, if ordinal > leap_month,
                // the month cannot be 1 because this implies the leap month is < 1, which is impossible;
                // cannot be 2 because that implies the leap month is = 1, which is impossible,
                // and cannot be more than 13 because max number of months in a year is 13.
                debug_assert!((2..=13).contains(&month));
                adjusted_ordinal -= 1;
            }
            debug_assert!((1..=12).contains(&adjusted_ordinal));
            match adjusted_ordinal {
                1 => tinystr!(4, "M01"),
                2 => tinystr!(4, "M02"),
                3 => tinystr!(4, "M03"),
                4 => tinystr!(4, "M04"),
                5 => tinystr!(4, "M05"),
                6 => tinystr!(4, "M06"),
                7 => tinystr!(4, "M07"),
                8 => tinystr!(4, "M08"),
                9 => tinystr!(4, "M09"),
                10 => tinystr!(4, "M10"),
                11 => tinystr!(4, "M11"),
                12 => tinystr!(4, "M12"),
                _ => tinystr!(4, "und"),
            }
        };
        let code = MonthCode(code_inner);
        MonthInfo {
            ordinal: month,
            standard_code: code,
            formatting_code: code,
        }
    }

    /// Create a new arithmetic date from a year, month ordinal, and day with bounds checking; returns the
    /// result of creating this arithmetic date, as well as a ChineseBasedYearInfo - either the one passed in
    /// optionally as an argument, or a new ChineseBasedYearInfo for the given year, month, and day args.
    pub(crate) fn validate_md(self, month: u8, day: u8) -> Result<(), DateError> {
        let max_month = self.months_in_year();
        if month == 0 || !(1..=max_month).contains(&month) {
            return Err(DateError::Range {
                field: "month",
                value: month as i32,
                min: 1,
                max: max_month as i32,
            });
        }

        let max_day = self.days_in_month(month);
        if day == 0 || day > max_day {
            return Err(DateError::Range {
                field: "day",
                value: day as i32,
                min: 1,
                max: max_day as i32,
            });
        }
        Ok(())
    }

    /// Get the ordinal lunar month from a code for chinese-based calendars.
    fn parse_month_code(self, code: MonthCode) -> ComputedOrdinalMonth {
        // 14 is a sentinel value, greater than all other months, for the purpose of computation only;
        // it is impossible to actually have 14 months in a year.
        let leap_month = self.leap_month().unwrap_or(14);

        let Some((unadjusted @ 1..13, leap)) = code.parsed() else {
            return ComputedOrdinalMonth::NotFound;
        };

        if leap && unadjusted + 1 == leap_month {
            return ComputedOrdinalMonth::Exact(leap_month);
        }

        let adjusted = unadjusted + (unadjusted + 1 > leap_month) as u8;

        if leap {
            ComputedOrdinalMonth::LeapToNormal(adjusted)
        } else {
            ComputedOrdinalMonth::Exact(adjusted)
        }
    }
}

/// An ordinal month computed from a month code
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ComputedOrdinalMonth {
    /// The exact code was found
    Exact(u8),
    /// The exact code was not found, but the corresponding
    /// non-leap ordinal month exists
    LeapToNormal(u8),
    /// The month code was not found at all, either as leap or non-leap
    NotFound,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::options::{DateFromFieldsOptions, Overflow};
    use crate::types::DateFields;
    use crate::types::MonthCode;
    use calendrical_calculations::{gregorian::fixed_from_gregorian, rata_die::RataDie};
    use std::collections::BTreeMap;
    use tinystr::tinystr;

    #[test]
    fn test_chinese_from_rd() {
        #[derive(Debug)]
        struct TestCase {
            rd: i64,
            expected_year: i32,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                rd: -964192,
                expected_year: -2639,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: -963838,
                expected_year: -2638,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: -963129,
                expected_year: -2637,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                rd: -963100,
                expected_year: -2637,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                rd: -963099,
                expected_year: -2636,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: 738700,
                expected_year: 2023,
                expected_month: 6,
                expected_day: 12,
            },
            TestCase {
                rd: fixed_from_gregorian(2319, 2, 20).to_i64_date(),
                expected_year: 2318,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                rd: fixed_from_gregorian(2319, 2, 21).to_i64_date(),
                expected_year: 2319,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                rd: 738718,
                expected_year: 2023,
                expected_month: 6,
                expected_day: 30,
            },
            TestCase {
                rd: 738747,
                expected_year: 2023,
                expected_month: 7,
                expected_day: 29,
            },
            TestCase {
                rd: 738748,
                expected_year: 2023,
                expected_month: 8,
                expected_day: 1,
            },
            TestCase {
                rd: 738865,
                expected_year: 2023,
                expected_month: 11,
                expected_day: 29,
            },
            TestCase {
                rd: 738895,
                expected_year: 2023,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                rd: 738925,
                expected_year: 2023,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                rd: 0,
                expected_year: 0,
                expected_month: 12,
                expected_day: 20,
            },
            TestCase {
                rd: -1,
                expected_year: 0,
                expected_month: 12,
                expected_day: 19,
            },
            TestCase {
                rd: -365,
                expected_year: -1,
                expected_month: 12,
                expected_day: 9,
            },
            TestCase {
                rd: 100,
                expected_year: 1,
                expected_month: 3,
                expected_day: 1,
            },
        ];

        for case in cases {
            let rata_die = RataDie::new(case.rd);

            let chinese = Date::from_rata_die(rata_die, LunarChinese::new_china());
            assert_eq!(
                case.expected_year,
                chinese.extended_year(),
                "Chinese from RD failed, case: {case:?}"
            );
            assert_eq!(
                case.expected_month,
                chinese.month().ordinal,
                "Chinese from RD failed, case: {case:?}"
            );
            assert_eq!(
                case.expected_day,
                chinese.day_of_month().0,
                "Chinese from RD failed, case: {case:?}"
            );
        }
    }

    #[test]
    fn test_rd_from_chinese() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            expected: i64,
        }

        let cases = [
            TestCase {
                year: 2023,
                month: 6,
                day: 6,
                // June 23 2023
                expected: 738694,
            },
            TestCase {
                year: -2636,
                month: 1,
                day: 1,
                expected: -963099,
            },
        ];

        for case in cases {
            let date = Date::try_new_chinese_with_calendar(
                case.year,
                case.month,
                case.day,
                LunarChinese::new_china(),
            )
            .unwrap();
            let rd = date.to_rata_die().to_i64_date();
            let expected = case.expected;
            assert_eq!(rd, expected, "RD from Chinese failed, with expected: {expected} and calculated: {rd}, for test case: {case:?}");
        }
    }

    #[test]
    fn test_rd_chinese_roundtrip() {
        let mut rd = -1963020;
        let max_rd = 1963020;
        let mut iters = 0;
        let max_iters = 560;
        while rd < max_rd && iters < max_iters {
            let rata_die = RataDie::new(rd);

            let chinese = Date::from_rata_die(rata_die, LunarChinese::new_china());
            let result = chinese.to_rata_die();
            assert_eq!(result, rata_die, "Failed roundtrip RD -> Chinese -> RD for RD: {rata_die:?}, with calculated: {result:?} from Chinese date:\n{chinese:?}");

            rd += 7043;
            iters += 1;
        }
    }

    #[test]
    fn test_chinese_epoch() {
        let iso = Date::try_new_iso(-2636, 2, 15).unwrap();

        let chinese = iso.to_calendar(LunarChinese::new_china());

        assert_eq!(chinese.cyclic_year().related_iso, -2636);
        assert_eq!(chinese.month().ordinal, 1);
        assert_eq!(chinese.month().standard_code.0, "M01");
        assert_eq!(chinese.day_of_month().0, 1);
        assert_eq!(chinese.cyclic_year().year, 1);
        assert_eq!(chinese.cyclic_year().related_iso, -2636);
    }

    #[test]
    fn test_iso_to_chinese_negative_years() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_year: i32,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                iso_year: -2636,
                iso_month: 2,
                iso_day: 14,
                expected_year: -2637,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                iso_year: -2636,
                iso_month: 1,
                iso_day: 15,
                expected_year: -2637,
                expected_month: 12,
                expected_day: 30,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.iso_year, case.iso_month, case.iso_day).unwrap();

            let chinese = iso.to_calendar(LunarChinese::new_china());
            assert_eq!(
                case.expected_year,
                chinese.cyclic_year().related_iso,
                "ISO to Chinese failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_month,
                chinese.month().ordinal,
                "ISO to Chinese failed for case: {case:?}"
            );
            assert_eq!(
                case.expected_day,
                chinese.day_of_month().0,
                "ISO to Chinese failed for case: {case:?}"
            );
        }
    }

    #[test]
    fn test_chinese_leap_months() {
        let expected = [
            (1933, 6),
            (1938, 8),
            (1984, 11),
            (2009, 6),
            (2017, 7),
            (2028, 6),
        ];

        for case in expected {
            let year = case.0;
            let expected_month = case.1;
            let iso = Date::try_new_iso(year, 6, 1).unwrap();

            let chinese_date = iso.to_calendar(LunarChinese::new_china());
            assert!(
                chinese_date.is_in_leap_year(),
                "{year} should be a leap year"
            );
            let new_year = chinese_date.inner.0.year.new_year();
            assert_eq!(
                expected_month,
                chinese_based::get_leap_month_from_new_year::<chinese_based::Chinese>(new_year),
                "{year} have leap month {expected_month}"
            );
        }
    }

    #[test]
    fn test_month_days() {
        let year = LunarChinese::new_china().0.year_data(2023);
        let cases = [
            (1, 29),
            (2, 30),
            (3, 29),
            (4, 29),
            (5, 30),
            (6, 30),
            (7, 29),
            (8, 30),
            (9, 30),
            (10, 29),
            (11, 30),
            (12, 29),
            (13, 30),
        ];
        for case in cases {
            let days_in_month = year.days_in_month(case.0);
            assert_eq!(
                case.1, days_in_month,
                "month_days test failed for case: {case:?}"
            );
        }
    }

    #[test]
    fn test_ordinal_to_month_code() {
        #[derive(Debug)]
        struct TestCase {
            year: i32,
            month: u8,
            day: u8,
            expected_code: &'static str,
        }

        let cases = [
            TestCase {
                year: 2023,
                month: 1,
                day: 9,
                expected_code: "M12",
            },
            TestCase {
                year: 2023,
                month: 2,
                day: 9,
                expected_code: "M01",
            },
            TestCase {
                year: 2023,
                month: 3,
                day: 9,
                expected_code: "M02",
            },
            TestCase {
                year: 2023,
                month: 4,
                day: 9,
                expected_code: "M02L",
            },
            TestCase {
                year: 2023,
                month: 5,
                day: 9,
                expected_code: "M03",
            },
            TestCase {
                year: 2023,
                month: 6,
                day: 9,
                expected_code: "M04",
            },
            TestCase {
                year: 2023,
                month: 7,
                day: 9,
                expected_code: "M05",
            },
            TestCase {
                year: 2023,
                month: 8,
                day: 9,
                expected_code: "M06",
            },
            TestCase {
                year: 2023,
                month: 9,
                day: 9,
                expected_code: "M07",
            },
            TestCase {
                year: 2023,
                month: 10,
                day: 9,
                expected_code: "M08",
            },
            TestCase {
                year: 2023,
                month: 11,
                day: 9,
                expected_code: "M09",
            },
            TestCase {
                year: 2023,
                month: 12,
                day: 9,
                expected_code: "M10",
            },
            TestCase {
                year: 2024,
                month: 1,
                day: 9,
                expected_code: "M11",
            },
            TestCase {
                year: 2024,
                month: 2,
                day: 9,
                expected_code: "M12",
            },
            TestCase {
                year: 2024,
                month: 2,
                day: 10,
                expected_code: "M01",
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.year, case.month, case.day).unwrap();
            let chinese = iso.to_calendar(LunarChinese::new_china());
            let result_code = chinese.month().standard_code.0;
            let expected_code = case.expected_code.to_string();
            assert_eq!(
                expected_code, result_code,
                "Month codes did not match for test case: {case:?}"
            );
        }
    }

    #[test]
    fn test_month_code_to_ordinal() {
        // construct using ::default() to force recomputation
        let year = LunarChinese::new_china().0.year_data(2023);
        let codes = [
            (1, tinystr!(4, "M01")),
            (2, tinystr!(4, "M02")),
            (3, tinystr!(4, "M02L")),
            (4, tinystr!(4, "M03")),
            (5, tinystr!(4, "M04")),
            (6, tinystr!(4, "M05")),
            (7, tinystr!(4, "M06")),
            (8, tinystr!(4, "M07")),
            (9, tinystr!(4, "M08")),
            (10, tinystr!(4, "M09")),
            (11, tinystr!(4, "M10")),
            (12, tinystr!(4, "M11")),
            (13, tinystr!(4, "M12")),
        ];
        for ordinal_code_pair in codes {
            let code = MonthCode(ordinal_code_pair.1);
            let ordinal = year.parse_month_code(code);
            assert_eq!(
                ordinal,
                ComputedOrdinalMonth::Exact(ordinal_code_pair.0),
                "Code to ordinal failed for year: {}, code: {code}",
                year.related_iso
            );
        }
    }

    #[test]
    fn check_invalid_month_code_to_ordinal() {
        let non_leap_year = 4659;
        let leap_year = 4660;
        let invalid_codes = [
            (non_leap_year, tinystr!(4, "M2")),
            (leap_year, tinystr!(4, "M0")),
            (non_leap_year, tinystr!(4, "J01")),
            (leap_year, tinystr!(4, "3M")),
            (non_leap_year, tinystr!(4, "M04L")),
            (leap_year, tinystr!(4, "M04L")),
            (non_leap_year, tinystr!(4, "M13")),
            (leap_year, tinystr!(4, "M13")),
        ];
        for (year, code) in invalid_codes {
            // construct using ::default() to force recomputation
            let year = LunarChinese::new_china().0.year_data(year);
            let code = MonthCode(code);
            let ordinal = year.parse_month_code(code);
            assert!(
                !matches!(ordinal, ComputedOrdinalMonth::Exact(_)),
                "Invalid month code failed for year: {}, code: {code}",
                year.related_iso
            );
        }
    }

    #[test]
    fn test_iso_chinese_roundtrip() {
        for i in -1000..=1000 {
            let year = i;
            let month = i as u8 % 12 + 1;
            let day = i as u8 % 28 + 1;
            let iso = Date::try_new_iso(year, month, day).unwrap();
            let chinese = iso.to_calendar(LunarChinese::new_china());
            let result = chinese.to_calendar(Iso);
            assert_eq!(iso, result, "ISO to Chinese roundtrip failed!\nIso: {iso:?}\nChinese: {chinese:?}\nResult: {result:?}");
        }
    }

    #[test]
    fn test_consistent_with_icu() {
        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_rel_iso: i32,
            expected_cyclic: u8,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                iso_year: -2332,
                iso_month: 3,
                iso_day: 1,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 16,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 2,
                iso_day: 15,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                // This test case fails to match ICU
                iso_year: -2332,
                iso_month: 2,
                iso_day: 14,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                // This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 17,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 2,
            },
            TestCase {
                // This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 15,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 1,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 15,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2334,
                expected_cyclic: 3,
                expected_month: 12,
                expected_day: 19,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let chinese = iso.to_calendar(LunarChinese::new_china());
            let chinese_rel_iso = chinese.cyclic_year().related_iso;
            let chinese_cyclic = chinese.cyclic_year().year;
            let chinese_month = chinese.month().ordinal;
            let chinese_day = chinese.day_of_month().0;

            assert_eq!(
                chinese_rel_iso, case.expected_rel_iso,
                "Related ISO failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_cyclic, case.expected_cyclic,
                "Cyclic year failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_month, case.expected_month,
                "Month failed for test case: {case:?}"
            );
            assert_eq!(
                chinese_day, case.expected_day,
                "Day failed for test case: {case:?}"
            );
        }
    }

    fn check_cyclic_and_rel_iso(year: i32) {
        let iso = Date::try_new_iso(year, 6, 6).unwrap();
        let chinese = iso.to_calendar(LunarChinese::new_china());
        let korean = iso.to_calendar(LunarChinese::new_korea());
        let chinese_year = chinese.cyclic_year();
        let korean_year = korean.cyclic_year();
        assert_eq!(
            chinese_year, korean_year,
            "Cyclic year failed for year: {year}"
        );
        let chinese_rel_iso = chinese_year.related_iso;
        let korean_rel_iso = korean_year.related_iso;
        assert_eq!(
            chinese_rel_iso, korean_rel_iso,
            "Rel. ISO year equality failed for year: {year}"
        );
        assert_eq!(korean_rel_iso, year, "Korean Rel. ISO failed!");
    }

    #[test]
    fn test_cyclic_same_as_chinese_near_present_day() {
        for year in 1923..=2123 {
            check_cyclic_and_rel_iso(year);
        }
    }

    #[test]
    fn test_cyclic_same_as_chinese_near_rd_zero() {
        for year in -100..=100 {
            check_cyclic_and_rel_iso(year);
        }
    }

    #[test]
    fn test_iso_to_korean_roundtrip() {
        let mut rd = -1963020;
        let max_rd = 1963020;
        let mut iters = 0;
        let max_iters = 560;
        while rd < max_rd && iters < max_iters {
            let rata_die = RataDie::new(rd);
            let iso = Date::from_rata_die(rata_die, Iso);
            let korean = iso.to_calendar(LunarChinese::new_korea());
            let result = korean.to_calendar(Iso);
            assert_eq!(
                iso, result,
                "Failed roundtrip ISO -> Korean -> ISO for RD: {rd}"
            );

            rd += 7043;
            iters += 1;
        }
    }

    #[test]
    fn test_from_fields_constrain() {
        let fields = DateFields {
            day: core::num::NonZero::new(31),
            month_code: Some(MonthCode("M01".parse().unwrap())),
            extended_year: Some(1972),
            ..Default::default()
        };
        let options = DateFromFieldsOptions {
            overflow: Some(Overflow::Constrain),
            ..Default::default()
        };

        let cal = LunarChinese::new_china();
        let date = Date::try_from_fields(fields, options, cal).unwrap();
        assert_eq!(
            date.day_of_month().0,
            29,
            "Day was successfully constrained"
        );

        // 2022 did not have M01L, the month should be constrained back down
        let fields = DateFields {
            day: core::num::NonZero::new(1),
            month_code: Some(MonthCode("M01L".parse().unwrap())),
            extended_year: Some(2022),
            ..Default::default()
        };
        let date = Date::try_from_fields(fields, options, cal).unwrap();
        assert_eq!(
            date.month().standard_code.0,
            "M01",
            "Month was successfully constrained"
        );
    }

    #[test]
    fn test_korean_consistent_with_icu() {
        // Test cases for this test are derived from existing ICU Intl.DateTimeFormat. If there is a bug in ICU,
        // these test cases may be affected, and this calendar's output may not be entirely valid.

        // There are a number of test cases which do not match ICU for dates very far in the past or future,
        // see #3709.

        #[derive(Debug)]
        struct TestCase {
            iso_year: i32,
            iso_month: u8,
            iso_day: u8,
            expected_rel_iso: i32,
            expected_cyclic: u8,
            expected_month: u8,
            expected_day: u8,
        }

        let cases = [
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: 4321,
                iso_month: 1,
                iso_day: 23,
                expected_rel_iso: 4320,
                expected_cyclic: 57,
                expected_month: 13,
                expected_day: 12,
            },
            TestCase {
                iso_year: 3649,
                iso_month: 9,
                iso_day: 20,
                expected_rel_iso: 3649,
                expected_cyclic: 46,
                expected_month: 9,
                expected_day: 1,
            },
            TestCase {
                iso_year: 3333,
                iso_month: 3,
                iso_day: 3,
                expected_rel_iso: 3333,
                expected_cyclic: 30,
                expected_month: 1,
                expected_day: 25,
            },
            TestCase {
                iso_year: 3000,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: 3000,
                expected_cyclic: 57,
                expected_month: 3,
                expected_day: 3,
            },
            TestCase {
                iso_year: 2772,
                iso_month: 7,
                iso_day: 27,
                expected_rel_iso: 2772,
                expected_cyclic: 9,
                expected_month: 7,
                expected_day: 5,
            },
            TestCase {
                iso_year: 2525,
                iso_month: 2,
                iso_day: 25,
                expected_rel_iso: 2525,
                expected_cyclic: 2,
                expected_month: 2,
                expected_day: 3,
            },
            TestCase {
                iso_year: 2345,
                iso_month: 3,
                iso_day: 21,
                expected_rel_iso: 2345,
                expected_cyclic: 2,
                expected_month: 2,
                expected_day: 17,
            },
            TestCase {
                iso_year: 2222,
                iso_month: 2,
                iso_day: 22,
                expected_rel_iso: 2222,
                expected_cyclic: 59,
                expected_month: 1,
                expected_day: 11,
            },
            TestCase {
                iso_year: 2167,
                iso_month: 6,
                iso_day: 22,
                expected_rel_iso: 2167,
                expected_cyclic: 4,
                expected_month: 5,
                expected_day: 6,
            },
            TestCase {
                iso_year: 2121,
                iso_month: 2,
                iso_day: 12,
                expected_rel_iso: 2120,
                expected_cyclic: 17,
                expected_month: 13,
                expected_day: 25,
            },
            TestCase {
                iso_year: 2080,
                iso_month: 12,
                iso_day: 31,
                expected_rel_iso: 2080,
                expected_cyclic: 37,
                expected_month: 12,
                expected_day: 21,
            },
            TestCase {
                iso_year: 2030,
                iso_month: 3,
                iso_day: 20,
                expected_rel_iso: 2030,
                expected_cyclic: 47,
                expected_month: 2,
                expected_day: 17,
            },
            TestCase {
                iso_year: 2027,
                iso_month: 2,
                iso_day: 7,
                expected_rel_iso: 2027,
                expected_cyclic: 44,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                iso_year: 2023,
                iso_month: 7,
                iso_day: 1,
                expected_rel_iso: 2023,
                expected_cyclic: 40,
                expected_month: 6,
                expected_day: 14,
            },
            TestCase {
                iso_year: 2022,
                iso_month: 3,
                iso_day: 1,
                expected_rel_iso: 2022,
                expected_cyclic: 39,
                expected_month: 1,
                expected_day: 29,
            },
            TestCase {
                iso_year: 2021,
                iso_month: 2,
                iso_day: 1,
                expected_rel_iso: 2020,
                expected_cyclic: 37,
                expected_month: 13,
                expected_day: 20,
            },
            TestCase {
                iso_year: 2016,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: 2016,
                expected_cyclic: 33,
                expected_month: 2,
                expected_day: 22,
            },
            TestCase {
                iso_year: 2016,
                iso_month: 7,
                iso_day: 30,
                expected_rel_iso: 2016,
                expected_cyclic: 33,
                expected_month: 6,
                expected_day: 27,
            },
            TestCase {
                iso_year: 2015,
                iso_month: 9,
                iso_day: 22,
                expected_rel_iso: 2015,
                expected_cyclic: 32,
                expected_month: 8,
                expected_day: 10,
            },
            TestCase {
                iso_year: 2013,
                iso_month: 10,
                iso_day: 1,
                expected_rel_iso: 2013,
                expected_cyclic: 30,
                expected_month: 8,
                expected_day: 27,
            },
            TestCase {
                iso_year: 2010,
                iso_month: 2,
                iso_day: 1,
                expected_rel_iso: 2009,
                expected_cyclic: 26,
                expected_month: 13,
                expected_day: 18,
            },
            TestCase {
                iso_year: 2000,
                iso_month: 8,
                iso_day: 30,
                expected_rel_iso: 2000,
                expected_cyclic: 17,
                expected_month: 8,
                expected_day: 2,
            },
            TestCase {
                iso_year: 1990,
                iso_month: 11,
                iso_day: 11,
                expected_rel_iso: 1990,
                expected_cyclic: 7,
                expected_month: 10,
                expected_day: 24,
            },
            TestCase {
                iso_year: 1970,
                iso_month: 6,
                iso_day: 10,
                expected_rel_iso: 1970,
                expected_cyclic: 47,
                expected_month: 5,
                expected_day: 7,
            },
            TestCase {
                iso_year: 1970,
                iso_month: 1,
                iso_day: 1,
                expected_rel_iso: 1969,
                expected_cyclic: 46,
                expected_month: 11,
                expected_day: 24,
            },
            TestCase {
                iso_year: 1941,
                iso_month: 12,
                iso_day: 7,
                expected_rel_iso: 1941,
                expected_cyclic: 18,
                expected_month: 11,
                expected_day: 19,
            },
            TestCase {
                iso_year: 1812,
                iso_month: 5,
                iso_day: 4,
                expected_rel_iso: 1812,
                expected_cyclic: 9,
                expected_month: 3,
                expected_day: 24,
            },
            TestCase {
                iso_year: 1655,
                iso_month: 6,
                iso_day: 15,
                expected_rel_iso: 1655,
                expected_cyclic: 32,
                expected_month: 5,
                expected_day: 12,
            },
            TestCase {
                iso_year: 1333,
                iso_month: 3,
                iso_day: 10,
                expected_rel_iso: 1333,
                expected_cyclic: 10,
                expected_month: 2,
                expected_day: 16,
            },
            TestCase {
                iso_year: 1000,
                iso_month: 10,
                iso_day: 10,
                expected_rel_iso: 1000,
                expected_cyclic: 37,
                expected_month: 9,
                expected_day: 5,
            },
            TestCase {
                iso_year: 842,
                iso_month: 2,
                iso_day: 15,
                expected_rel_iso: 841,
                expected_cyclic: 58,
                expected_month: 13,
                expected_day: 28,
            },
            TestCase {
                iso_year: 101,
                iso_month: 1,
                iso_day: 10,
                expected_rel_iso: 100,
                expected_cyclic: 37,
                expected_month: 12,
                expected_day: 24,
            },
            TestCase {
                iso_year: -1,
                iso_month: 3,
                iso_day: 28,
                expected_rel_iso: -1,
                expected_cyclic: 56,
                expected_month: 2,
                expected_day: 25,
            },
            TestCase {
                iso_year: -3,
                iso_month: 2,
                iso_day: 28,
                expected_rel_iso: -3,
                expected_cyclic: 54,
                expected_month: 2,
                expected_day: 5,
            },
            TestCase {
                iso_year: -365,
                iso_month: 7,
                iso_day: 24,
                expected_rel_iso: -365,
                expected_cyclic: 52,
                expected_month: 6,
                expected_day: 24,
            },
            TestCase {
                iso_year: -999,
                iso_month: 9,
                iso_day: 9,
                expected_rel_iso: -999,
                expected_cyclic: 18,
                expected_month: 7,
                expected_day: 27,
            },
            TestCase {
                iso_year: -1500,
                iso_month: 1,
                iso_day: 5,
                expected_rel_iso: -1501,
                expected_cyclic: 56,
                expected_month: 12,
                expected_day: 2,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 3,
                iso_day: 1,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 16,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 2,
                iso_day: 15,
                expected_rel_iso: -2332,
                expected_cyclic: 5,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -2332,
                iso_month: 2,
                iso_day: 14,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 30,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 17,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 2,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -2332,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 13,
                expected_day: 1,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 15,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                iso_year: -2332,
                iso_month: 1,
                iso_day: 1,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 12,
                expected_day: 15,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 16,
                expected_rel_iso: -2334,
                expected_cyclic: 3,
                expected_month: 12,
                expected_day: 19,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 27,
                expected_rel_iso: -2333,
                expected_cyclic: 4,
                expected_month: 1,
                expected_day: 1,
            },
            TestCase {
                iso_year: -2333,
                iso_month: 1,
                iso_day: 26,
                expected_rel_iso: -2334,
                expected_cyclic: 3,
                expected_month: 12,
                expected_day: 29,
            },
            TestCase {
                iso_year: -2600,
                iso_month: 9,
                iso_day: 16,
                expected_rel_iso: -2600,
                expected_cyclic: 37,
                expected_month: 8,
                expected_day: 16,
            },
            TestCase {
                iso_year: -2855,
                iso_month: 2,
                iso_day: 3,
                expected_rel_iso: -2856,
                expected_cyclic: 21,
                expected_month: 12,
                expected_day: 30,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3000,
                iso_month: 5,
                iso_day: 15,
                expected_rel_iso: -3000,
                expected_cyclic: 57,
                expected_month: 4,
                expected_day: 1,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3649,
                iso_month: 9,
                iso_day: 20,
                expected_rel_iso: -3649,
                expected_cyclic: 8,
                expected_month: 8,
                expected_day: 10,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3649,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: -3649,
                expected_cyclic: 8,
                expected_month: 2,
                expected_day: 14,
            },
            TestCase {
                // #3709: This test case fails to match ICU
                iso_year: -3650,
                iso_month: 3,
                iso_day: 30,
                expected_rel_iso: -3650,
                expected_cyclic: 7,
                expected_month: 3,
                expected_day: 3,
            },
        ];

        for case in cases {
            let iso = Date::try_new_iso(case.iso_year, case.iso_month, case.iso_day).unwrap();
            let korean = iso.to_calendar(LunarChinese::new_korea());
            let korean_cyclic = korean.cyclic_year();
            let korean_month = korean.month().ordinal;
            let korean_day = korean.day_of_month().0;

            assert_eq!(
                korean_cyclic.related_iso, case.expected_rel_iso,
                "Related ISO failed for test case: {case:?}"
            );
            assert_eq!(
                korean_cyclic.year, case.expected_cyclic,
                "Cyclic year failed for test case: {case:?}"
            );
            assert_eq!(
                korean_month, case.expected_month,
                "Month failed for test case: {case:?}"
            );
            assert_eq!(
                korean_day, case.expected_day,
                "Day failed for test case: {case:?}"
            );
        }
    }

    #[test]
    #[ignore] // slow, network
    fn test_against_hong_kong_observatory_data() {
        use crate::{
            cal::{Gregorian, LunarChinese},
            types::MonthCode,
            Date,
        };

        let mut related_iso = 1900;
        let mut lunar_month = MonthCode::new_normal(11).unwrap();

        for year in 1901..=2100 {
            println!("Validating year {year}...");

            for line in ureq::get(&format!(
                "https://www.hko.gov.hk/en/gts/time/calendar/text/files/T{year}e.txt"
            ))
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap()
            .split('\n')
            {
                if !line.starts_with(['1', '2']) {
                    // comments or blank lines
                    continue;
                }

                let mut fields = line.split_ascii_whitespace();

                let mut gregorian = fields.next().unwrap().split('/');
                let gregorian = Date::try_new_gregorian(
                    gregorian.next().unwrap().parse().unwrap(),
                    gregorian.next().unwrap().parse().unwrap(),
                    gregorian.next().unwrap().parse().unwrap(),
                )
                .unwrap();

                let day_or_lunar_month = fields.next().unwrap();

                let lunar_day = if fields.next().is_some_and(|s| s.contains("Lunar")) {
                    let new_lunar_month = day_or_lunar_month
                        // 1st, 2nd, 3rd, nth
                        .split_once(['s', 'n', 'r', 't'])
                        .unwrap()
                        .0
                        .parse()
                        .unwrap();
                    if new_lunar_month == lunar_month.parsed().unwrap().0 {
                        lunar_month = MonthCode::new_leap(new_lunar_month).unwrap();
                    } else {
                        lunar_month = MonthCode::new_normal(new_lunar_month).unwrap();
                    }
                    if new_lunar_month == 1 {
                        related_iso += 1;
                    }
                    1
                } else {
                    day_or_lunar_month.parse().unwrap()
                };

                let chinese = Date::try_new_from_codes(
                    None,
                    related_iso,
                    lunar_month,
                    lunar_day,
                    LunarChinese::new_china(),
                )
                .unwrap();

                assert_eq!(
                    gregorian,
                    chinese.to_calendar(Gregorian),
                    "{line}, {chinese:?}"
                );
            }
        }
    }

    #[test]
    #[ignore] // network
    fn test_against_kasi_data() {
        use crate::{
            cal::{Gregorian, LunarChinese},
            types::MonthCode,
            Date,
        };

        // TODO: query KASI directly
        let uri = "https://gist.githubusercontent.com/Manishearth/d8c94a7df22a9eacefc4472a5805322e/raw/e1ea3b0aa52428686bb3a9cd0f262878515e16c1/resolved.json";
        
        #[derive(serde::Deserialize)]
        struct Golden(BTreeMap<i32, BTreeMap<MonthCode, MonthData>>);

        #[derive(serde::Deserialize)]
        struct MonthData {
            start_date: String,
        }

        let json = ureq::get(uri)
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap();

        let golden = serde_json::from_str::<Golden>(&json).unwrap();

        for (&year, months) in &golden.0 {
            if year == 1899 || year == 2050 {
                continue;
            }
            for (&month, month_data) in months {
                let mut gregorian = month_data.start_date.split('-');
                let gregorian = Date::try_new_gregorian(
                    gregorian.next().unwrap().parse().unwrap(),
                    gregorian.next().unwrap().parse().unwrap(),
                    gregorian.next().unwrap().parse().unwrap(),
                )
                .unwrap();

                assert_eq!(
                    Date::try_new_from_codes(None, year, month, 1, LunarChinese::new_korea())
                        .unwrap()
                        .to_calendar(Gregorian),
                    gregorian
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn generate_reference_years() {
        generate_reference_years_for(LunarChinese::new_china());
        generate_reference_years_for(LunarChinese::new_korea());
        fn generate_reference_years_for<R: Rules + Copy>(calendar: LunarChinese<R>) {
            use crate::Date;

            println!("Reference years for {calendar:?}:");
            let reference_year_end = Date::from_rata_die(
                crate::cal::abstract_gregorian::LAST_DAY_OF_REFERENCE_YEAR,
                calendar,
            );
            let year_1900_start = Date::try_new_gregorian(1900, 1, 1)
                .unwrap()
                .to_calendar(calendar);
            let year_2035_end = Date::try_new_gregorian(2035, 12, 31)
                .unwrap()
                .to_calendar(calendar);
            for month in 1..=12 {
                for leap in [false, true] {
                    'outer: for long in [false, true] {
                        for (start_year, start_month, end_year, end_month, by) in [
                            (
                                reference_year_end.extended_year(),
                                reference_year_end.month().month_number(),
                                year_1900_start.extended_year(),
                                year_1900_start.month().month_number(),
                                -1,
                            ),
                            (
                                reference_year_end.extended_year(),
                                reference_year_end.month().month_number(),
                                year_2035_end.extended_year(),
                                year_2035_end.month().month_number(),
                                1,
                            ),
                            (
                                year_1900_start.extended_year(),
                                year_1900_start.month().month_number(),
                                -10000,
                                1,
                                -1,
                            ),
                        ] {
                            let mut year = start_year;
                            while year * by < end_year * by {
                                if year == start_year
                                    && month as i32 * by <= start_month as i32 * by
                                    || year == end_year
                                        && month as i32 * by >= end_month as i32 * by
                                {
                                    year += by;
                                    continue;
                                }
                                let data = calendar.0.year_data(year);
                                let leap_month = data.leap_month().unwrap_or(15);
                                let days_in_month = data.days_in_month({
                                    if leap && month + 1 == leap_month {
                                        month + 1
                                    } else {
                                        month + (month + 1 > leap_month) as u8
                                    }
                                });
                                if (!long || (days_in_month == 30))
                                    && (!leap || month + 1 == leap_month)
                                {
                                    println!("({month}, {leap:?}, {long:?}) => {year},");
                                    continue 'outer;
                                }
                                year += by;
                            }
                        }
                        println!("({month}, {leap:?}, {long:?}) => todo!(),")
                    }
                }
            }
        }
    }
}
