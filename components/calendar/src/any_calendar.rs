// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Module for working with multiple calendars at once

use crate::cal::iso::IsoDateInner;
use crate::cal::*;
use crate::error::DateError;
use crate::options::DateFromFieldsOptions;
use crate::options::{DateAddOptions, DateDifferenceOptions};
use crate::types::{DateFields, YearInfo};
use crate::{types, AsCalendar, Calendar, Date, Ref};

use crate::preferences::{CalendarAlgorithm, HijriCalendarAlgorithm};
use icu_locale_core::preferences::define_preferences;
use icu_locale_core::subtags::region;
use icu_provider::prelude::*;

use core::fmt;

define_preferences!(
    /// The preferences for calendars formatting.
    [Copy]
    CalendarPreferences,
    {
        /// The user's preferred calendar system.
        calendar_algorithm: CalendarAlgorithm
    }
);

/// This is a calendar that encompasses all formattable calendars supported by this crate
///
/// This allows for the construction of [`Date`] objects that have their calendar known at runtime.
///
/// This can be constructed by calling `.into()` on a concrete calendar type if the calendar type is known at
/// compile time. When the type is known at runtime, the [`AnyCalendar::new()`] and sibling methods may be used.
///
/// [`Date`] can also be converted to [`AnyCalendar`]-compatible ones
/// via [`Date::to_any()`](crate::Date::to_any()).
///
/// There are many ways of constructing an AnyCalendar'd date:
/// ```
/// use icu::calendar::{AnyCalendar, AnyCalendarKind, Date, cal::Japanese, types::MonthCode};
/// use icu::locale::locale;
/// use tinystr::tinystr;
/// # use std::rc::Rc;
///
/// let locale = locale!("en-u-ca-japanese"); // English with the Japanese calendar
///
/// let calendar = AnyCalendar::new(AnyCalendarKind::new(locale.into()));
/// let calendar = Rc::new(calendar); // Avoid cloning it each time
///                                   // If everything is a local reference, you may use icu::calendar::Ref instead.
///
/// // construct from era code, year, month code, day, and a calendar
/// // This is March 28, 15 Heisei
/// let manual_date = Date::try_new_from_codes(Some("heisei"), 15, MonthCode(tinystr!(4, "M03")), 28, calendar.clone())
///                     .expect("Failed to construct Date manually");
///
///
/// // construct another date by converting from ISO
/// let iso_date = Date::try_new_iso(2020, 9, 1)
///     .expect("Failed to construct ISO Date.");
/// let iso_converted = iso_date.to_calendar(calendar);
///
/// // Construct a date in the appropriate typed calendar and convert
/// let japanese_calendar = Japanese::new();
/// let japanese_date = Date::try_new_japanese_with_calendar("heisei", 15, 3, 28,
///                                                         japanese_calendar).unwrap();
/// // This is a Date<AnyCalendar>
/// let any_japanese_date = japanese_date.to_any();
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AnyCalendar {
    /// A [`Buddhist`] calendar
    Buddhist(Buddhist),
    /// A [`Chinese`] calendar
    Chinese(LunarChinese<chinese::China>),
    /// A [`Coptic`] calendar
    Coptic(Coptic),
    /// A [`Dangi`] calendar
    Dangi(LunarChinese<chinese::Korea>),
    /// An [`Ethiopian`] calendar
    Ethiopian(Ethiopian),
    /// A [`Gregorian`] calendar
    Gregorian(Gregorian),
    /// A [`Hebrew`] calendar
    Hebrew(Hebrew),
    /// An [`Indian`] calendar
    Indian(Indian),
    /// A [`HijriTabular`] calendar
    HijriTabular(Hijri<hijri::TabularAlgorithm>),
    /// A [`HijriSimulated`] calendar
    HijriSimulated(Hijri<hijri::AstronomicalSimulation>),
    /// A [`HijriUmmAlQura`] calendar
    HijriUmmAlQura(Hijri<hijri::UmmAlQura>),
    /// An [`Iso`] calendar
    Iso(Iso),
    /// A [`Japanese`] calendar
    Japanese(Japanese),
    /// A [`JapaneseExtended`] calendar
    JapaneseExtended(JapaneseExtended),
    /// A [`Persian`] calendar
    Persian(Persian),
    /// A [`Roc`] calendar
    Roc(Roc),
}

// TODO(#3469): Decide on the best way to implement Ord.
/// The inner date type for [`AnyCalendar`]
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
#[non_exhaustive]
pub enum AnyDateInner {
    /// A date for a [`Buddhist`] calendar
    Buddhist(<Buddhist as Calendar>::DateInner),
    /// A date for a [`Chinese`] calendar
    Chinese(<LunarChinese<chinese::China> as Calendar>::DateInner),
    /// A date for a [`Coptic`] calendar
    Coptic(<Coptic as Calendar>::DateInner),
    /// A date for a [`Dangi`] calendar
    Dangi(<LunarChinese<chinese::Korea> as Calendar>::DateInner),
    /// A date for an [`Ethiopian`] calendar
    Ethiopian(<Ethiopian as Calendar>::DateInner),
    /// A date for a [`Gregorian`] calendar
    Gregorian(<Gregorian as Calendar>::DateInner),
    /// A date for a [`Hebrew`] calendar
    Hebrew(<Hebrew as Calendar>::DateInner),
    /// A date for an [`Indian`] calendar
    Indian(<Indian as Calendar>::DateInner),
    /// A date for a [`HijriTabular`] calendar
    HijriTabular(
        <Hijri<hijri::TabularAlgorithm> as Calendar>::DateInner,
        hijri::TabularAlgorithm,
    ),
    /// A date for a [`HijriSimulated`] calendar
    HijriSimulated(<Hijri<hijri::AstronomicalSimulation> as Calendar>::DateInner),
    /// A date for a [`HijriUmmAlQura`] calendar
    HijriUmmAlQura(<Hijri<hijri::UmmAlQura> as Calendar>::DateInner),
    /// A date for an [`Iso`] calendar
    Iso(<Iso as Calendar>::DateInner),
    /// A date for a [`Japanese`] calendar
    Japanese(<Japanese as Calendar>::DateInner),
    /// A date for a [`JapaneseExtended`] calendar
    JapaneseExtended(<JapaneseExtended as Calendar>::DateInner),
    /// A date for a [`Persian`] calendar
    Persian(<Persian as Calendar>::DateInner),
    /// A date for a [`Roc`] calendar
    Roc(<Roc as Calendar>::DateInner),
}

/// Generalized macro that generates a match statement over all AnyCalendar variants.
/// 
/// Parameters:
/// 
/// 1. `$cal`: The variables to be matched over: for example, `cal`, `(cal, date)`, or `(cal, date1, date2)`.
/// 2. `$pattern`: The name of a macro that generates a pattern for the match statement. It is passed as arguments:
///     - The calendar variant ident (e.g. `Buddhist`).
///     - An ident to be used for matching the `AnyCalendar` variant, `c`.
///     - A list of identifiers for `AnyDateInner` variants. For `HijriTabular`, this list also includes identifiers for the algorithm.
/// 3. `$expr`: The name of a macro that generates an expression for the match statement. It is passed as arguments:
///     - The calendar variant ident (e.g. `Buddhist`).
///     - An ident for the `AnyCalendar` variant, `c`.
///     - A list of identifiers for `AnyDateInner` variants.
/// 4. `[$d1, $alg1, $d2, $alg2, ...]`: A list of identifiers that are passed into `$pattern` and `$expr`.
///    There should generally be 2 identifiers per date being matched. Can be omitted if no dates are being matched.
/// 5. `$exhaustive_expr`: The expression for the `_` pattern. Can be omitted if no dates are being matched.
#[rustfmt::skip]
macro_rules! match_cal_general {
    ($cal:expr, $pattern:ident, $expr:ident $(, [$($d:ident, $alg:ident),+], $exhaustive_expr:expr)?) => {
        match $cal {
            $pattern!(Buddhist, c $(,[$($d),*])?) => $expr!(Buddhist, c $(,[$($d),*])?),
            $pattern!(Chinese, c $(,[$($d),*])?) => $expr!(Chinese, c $(,[$($d),*])?),
            $pattern!(Coptic, c $(,[$($d),*])?) => $expr!(Coptic, c $(,[$($d),*])?),
            $pattern!(Dangi, c $(,[$($d),*])?) => $expr!(Dangi, c $(,[$($d),*])?),
            $pattern!(Ethiopian, c $(,[$($d),*])?) => $expr!(Ethiopian, c $(,[$($d),*])?),
            $pattern!(Gregorian, c $(,[$($d),*])?) => $expr!(Gregorian, c $(,[$($d),*])?),
            $pattern!(Hebrew, c $(,[$($d),*])?) => $expr!(Hebrew, c $(,[$($d),*])?),
            $pattern!(Indian, c $(,[$($d),*])?) => $expr!(Indian, c $(,[$($d),*])?),
            $pattern!(HijriSimulated, c $(,[$($d),*])?) => $expr!(HijriSimulated, c $(,[$($d),*])?),
            $pattern!(HijriUmmAlQura, c $(,[$($d),*])?) => $expr!(HijriUmmAlQura, c $(,[$($d),*])?),
            $pattern!(Iso, c $(,[$($d),*])?) => $expr!(Iso, c $(,[$($d),*])?),
            $pattern!(Japanese, c $(,[$($d),*])?) => $expr!(Japanese, c $(,[$($d),*])?),
            $pattern!(JapaneseExtended, c $(,[$($d),*])?) => $expr!(JapaneseExtended, c $(,[$($d),*])?),
            $pattern!(Persian, c $(,[$($d),*])?) => $expr!(Persian, c $(,[$($d),*])?),
            $pattern!(Roc, c $(,[$($d),*])?) => $expr!(Roc, c $(,[$($d),*])?),
            // HijriTabular is special because the DateInner has an extra field
            $pattern!(HijriTabular, c $(,[$($d, $alg),*])?) $(if $(c.0 == $alg)&&*)? => $expr!(HijriTabular, c $(,[$($d),*])?),
            // The exhaustive match is required when matching against DateInner
            $(_ => $exhaustive_expr)?
        }
    };
}

/// Macro that generates a pattern match for a calendar only.
macro_rules! pattern_cal {
    ($cal:ident, $c:ident) => {
        &Self::$cal(ref $c)
    };
}

/// Macro that generates a pattern match for a calendar and date.
macro_rules! pattern_cal_date {
    (HijriTabular, $c:ident, [$d:ident, $alg:ident]) => {
        (&Self::HijriTabular(ref $c), &AnyDateInner::HijriTabular(ref $d, $alg))
    };
    ($cal:ident, $c:ident, [$d:ident]) => {
        (&Self::$cal(ref $c), &AnyDateInner::$cal(ref $d))
    };
}

/// Macro that generates a pattern match for a calendar and two dates.
macro_rules! pattern_cal_date_date {
    (HijriTabular, $c:ident, [$d1:ident, $alg1:ident, $d2:ident, $alg2:ident]) => {
        (&Self::HijriTabular(ref $c), &AnyDateInner::HijriTabular(ref $d1, $alg1), &AnyDateInner::HijriTabular(ref $d2, $alg2))
    };
    ($cal:ident, $c:ident, [$d1:ident, $d2:ident]) => {
        (&Self::$cal(ref $c), &AnyDateInner::$cal(ref $d1), &AnyDateInner::$cal(ref $d2))
    };
}

/// Convenience macro that invokes [`match_cal_general`] with common arguments for
/// matching the calendar only.
macro_rules! match_cal {
    ($cal:expr, c.$($call:tt)+) => {{
        macro_rules! helper {
            (HijriTabular, $c:ident) => {
                AnyDateInner::HijriTabular($c.$($call)+, $c.0)
            };
            ($cal_inner:ident, $c:ident) => {
                AnyDateInner::$cal_inner($c.$($call)+)
            };
        }
        match_cal_general!($cal, pattern_cal, helper)
    }};
}

/// Convenience macro that invokes [`match_cal_general`] with common arguments for
/// matching the calendar and one date.
macro_rules! match_cal_and_date {
    ($cal:expr, c.$f:tt(d $($extra1:tt)*) $($extra2:tt)*) => {{
        macro_rules! helper {
            ($cal_inner:ident, $c:ident, [$d:ident]) => {
                $c.$f($d $($extra1)*) $($extra2)*
            };
        }
        match_cal_general!($cal, pattern_cal_date, helper, [d, alg], {
            unreachable!("AnyCalendar with mismatched date type")
        })
    }};
}

/// Error returned when comparing two [`Date`]s with [`AnyCalendar`].
#[derive(Clone, Copy, PartialEq, Debug)]
#[non_exhaustive]
#[doc(hidden)] // unstable, not yet graduated
pub enum AnyCalendarDifferenceError {
    /// The calendars of the two dates being compared are not equal.
    ///
    /// To compare dates in different calendars, convert them to the same calendar first.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::cal::AnyCalendarDifferenceError;
    ///
    /// let d1 = Date::try_new_gregorian(2000, 1, 1).unwrap().to_any();
    /// let d2 = Date::try_new_hebrew(5780, 1, 1).unwrap().to_any();
    ///
    /// assert!(matches!(
    ///     d1.try_until_with_options(&d2, Default::default()),
    ///     Err(AnyCalendarDifferenceError::MismatchedCalendars),
    /// ));
    ///
    /// // To compare the dates, convert them to the same calendar,
    /// // such as ISO.
    ///
    /// assert!(matches!(
    ///     d1.to_iso().try_until_with_options(&d2.to_iso(), Default::default()),
    ///     Ok(_)
    /// ));
    /// ```
    MismatchedCalendars,
}

impl crate::cal::scaffold::UnstableSealed for AnyCalendar {}
impl Calendar for AnyCalendar {
    type DateInner = AnyDateInner;
    type Year = YearInfo;
    type DifferenceError = AnyCalendarDifferenceError;

    fn from_fields(
        &self,
        fields: DateFields,
        options: DateFromFieldsOptions,
    ) -> Result<Self::DateInner, DateError> {
        Ok(match_cal!(self, c.from_fields(fields, options)?))
    }

    fn from_iso(&self, iso: IsoDateInner) -> AnyDateInner {
        match_cal!(self, c.from_iso(iso))
    }

    fn from_rata_die(&self, rd: calendrical_calculations::rata_die::RataDie) -> Self::DateInner {
        match_cal!(self, c.from_rata_die(rd))
    }

    fn to_rata_die(&self, date: &Self::DateInner) -> calendrical_calculations::rata_die::RataDie {
        match_cal_and_date!((self, date), c.to_rata_die(d))
    }

    fn to_iso(&self, date: &Self::DateInner) -> IsoDateInner {
        match_cal_and_date!((self, date), c.to_iso(d))
    }

    fn months_in_year(&self, date: &Self::DateInner) -> u8 {
        match_cal_and_date!((self, date), c.months_in_year(d))
    }

    fn days_in_year(&self, date: &Self::DateInner) -> u16 {
        match_cal_and_date!((self, date), c.days_in_year(d))
    }

    fn days_in_month(&self, date: &Self::DateInner) -> u8 {
        match_cal_and_date!((self, date), c.days_in_month(d))
    }

    fn add(
        &self,
        date: &Self::DateInner,
        duration: types::DateDuration,
        options: DateAddOptions,
    ) -> Result<Self::DateInner, DateError> {
        macro_rules! expr_add {
            (HijriTabular, $c:ident, [$d:ident]) => {
                Some(AnyDateInner::HijriTabular(
                    $c.add($d, duration, options)?,
                    $c.0,
                ))
            };
            ($cal:ident, $c:ident, [$d:ident]) => {
                Some(AnyDateInner::$cal($c.add($d, duration, options)?))
            };
        }
        let result = match_cal_general!((self, date), pattern_cal_date, expr_add, [d, alg], None);
        match result {
            Some(result) => Ok(result),
            None => {
                unreachable!("AnyCalendar with mismatched date type")
            }
        }
    }

    fn until(
        &self,
        date1: &Self::DateInner,
        date2: &Self::DateInner,
        options: DateDifferenceOptions,
    ) -> Result<types::DateDuration, Self::DifferenceError> {
        macro_rules! expr_until {
            ($cal:ident, $c:ident, [$d1:ident, $d2:ident]) => {
                Some($c.until($d1, $d2, options))
            };
        }
        let result = match_cal_general!(
            (self, date1, date2),
            pattern_cal_date_date,
            expr_until,
            [d1, alg1, d2, alg2],
            None
        );
        match result {
            Some(Ok(result)) => Ok(result),
            None => Err(AnyCalendarDifferenceError::MismatchedCalendars),
        }
    }

    fn year_info(&self, date: &Self::DateInner) -> types::YearInfo {
        match_cal_and_date!((self, date), c.year_info(d).into())
    }

    /// The calendar-specific check if `date` is in a leap year
    fn is_in_leap_year(&self, date: &Self::DateInner) -> bool {
        match_cal_and_date!((self, date), c.is_in_leap_year(d))
    }

    /// The calendar-specific month represented by `date`
    fn month(&self, date: &Self::DateInner) -> types::MonthInfo {
        match_cal_and_date!((self, date), c.month(d))
    }

    /// The calendar-specific day-of-month represented by `date`
    fn day_of_month(&self, date: &Self::DateInner) -> types::DayOfMonth {
        match_cal_and_date!((self, date), c.day_of_month(d))
    }

    /// Information of the day of the year
    fn day_of_year(&self, date: &Self::DateInner) -> types::DayOfYear {
        match_cal_and_date!((self, date), c.day_of_year(d))
    }

    fn debug_name(&self) -> &'static str {
        match self.kind() {
            AnyCalendarKind::Buddhist => "AnyCalendar (Buddhist)",
            AnyCalendarKind::Chinese => "AnyCalendar (Chinese)",
            AnyCalendarKind::Coptic => "AnyCalendar (Coptic)",
            AnyCalendarKind::Dangi => "AnyCalendar (Dangi)",
            AnyCalendarKind::Ethiopian => "AnyCalendar (Ethiopian, Amete Miret)",
            AnyCalendarKind::EthiopianAmeteAlem => "AnyCalendar (Ethiopian, Amete Alem)",
            AnyCalendarKind::Gregorian => "AnyCalendar (Gregorian)",
            AnyCalendarKind::Hebrew => "AnyCalendar (Hebrew)",
            AnyCalendarKind::Indian => "AnyCalendar (Indian)",
            AnyCalendarKind::HijriTabularTypeIIFriday => {
                "AnyCalendar (Hijri, tabular, type II leap years, Friday epoch)"
            }
            AnyCalendarKind::HijriTabularTypeIIThursday => {
                "AnyCalendar (Hijri, tabular, type II leap years, Thursday epoch)"
            }
            AnyCalendarKind::HijriSimulatedMecca => "AnyCalendar (Hijri, simulated Mecca)",
            AnyCalendarKind::HijriUmmAlQura => "AnyCalendar (Hijri, Umm al-Qura)",
            AnyCalendarKind::Iso => "AnyCalendar (Iso)",
            AnyCalendarKind::Japanese => "AnyCalendar (Japanese)",
            AnyCalendarKind::JapaneseExtended => "AnyCalendar (Japanese, historical era data)",
            AnyCalendarKind::Persian => "AnyCalendar (Persian)",
            AnyCalendarKind::Roc => "AnyCalendar (Roc)",
        }
    }

    fn calendar_algorithm(&self) -> Option<CalendarAlgorithm> {
        match self {
            Self::Buddhist(ref c) => c.calendar_algorithm(),
            Self::Chinese(ref c) => c.calendar_algorithm(),
            Self::Coptic(ref c) => c.calendar_algorithm(),
            Self::Dangi(ref c) => c.calendar_algorithm(),
            Self::Ethiopian(ref c) => c.calendar_algorithm(),
            Self::Gregorian(ref c) => c.calendar_algorithm(),
            Self::Hebrew(ref c) => c.calendar_algorithm(),
            Self::Indian(ref c) => c.calendar_algorithm(),
            Self::HijriSimulated(ref c) => c.calendar_algorithm(),
            Self::HijriTabular(ref c) => c.calendar_algorithm(),
            Self::HijriUmmAlQura(ref c) => c.calendar_algorithm(),
            Self::Iso(ref c) => c.calendar_algorithm(),
            Self::Japanese(ref c) => c.calendar_algorithm(),
            Self::JapaneseExtended(ref c) => c.calendar_algorithm(),
            Self::Persian(ref c) => c.calendar_algorithm(),
            Self::Roc(ref c) => c.calendar_algorithm(),
        }
    }
}

impl AnyCalendar {
    /// Constructs an AnyCalendar for a given calendar kind from compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new(kind: AnyCalendarKind) -> Self {
        match kind {
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Chinese => AnyCalendar::Chinese(LunarChinese::new_china()),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Dangi => AnyCalendar::Dangi(LunarChinese::new_korea()),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Hebrew => AnyCalendar::Hebrew(Hebrew),
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::HijriTabularTypeIIFriday => {
                AnyCalendar::HijriTabular(Hijri::new_tabular(
                    hijri::TabularAlgorithmLeapYears::TypeII,
                    hijri::TabularAlgorithmEpoch::Friday,
                ))
            }
            AnyCalendarKind::HijriSimulatedMecca => {
                AnyCalendar::HijriSimulated(Hijri::new_simulated_mecca())
            }
            AnyCalendarKind::HijriTabularTypeIIThursday => {
                AnyCalendar::HijriTabular(Hijri::new_tabular(
                    hijri::TabularAlgorithmLeapYears::TypeII,
                    hijri::TabularAlgorithmEpoch::Thursday,
                ))
            }
            AnyCalendarKind::HijriUmmAlQura => {
                AnyCalendar::HijriUmmAlQura(Hijri::new_umm_al_qura())
            }
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Japanese => AnyCalendar::Japanese(Japanese::new()),
            AnyCalendarKind::JapaneseExtended => {
                AnyCalendar::JapaneseExtended(JapaneseExtended::new())
            }
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Roc => AnyCalendar::Roc(Roc),
        }
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER, Self::new)]
    pub fn try_new_with_buffer_provider<P>(
        provider: &P,
        kind: AnyCalendarKind,
    ) -> Result<Self, DataError>
    where
        P: BufferProvider + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Chinese => AnyCalendar::Chinese(LunarChinese::new_china()),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Dangi => AnyCalendar::Dangi(LunarChinese::new_korea()),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Hebrew => AnyCalendar::Hebrew(Hebrew),
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::HijriTabularTypeIIFriday => {
                AnyCalendar::HijriTabular(Hijri::new_tabular(
                    hijri::TabularAlgorithmLeapYears::TypeII,
                    hijri::TabularAlgorithmEpoch::Friday,
                ))
            }
            AnyCalendarKind::HijriSimulatedMecca => {
                AnyCalendar::HijriSimulated(Hijri::new_simulated_mecca())
            }
            AnyCalendarKind::HijriTabularTypeIIThursday => {
                AnyCalendar::HijriTabular(Hijri::new_tabular(
                    hijri::TabularAlgorithmLeapYears::TypeII,
                    hijri::TabularAlgorithmEpoch::Thursday,
                ))
            }
            AnyCalendarKind::HijriUmmAlQura => {
                AnyCalendar::HijriUmmAlQura(Hijri::new_umm_al_qura())
            }
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Japanese => {
                AnyCalendar::Japanese(Japanese::try_new_with_buffer_provider(provider)?)
            }
            AnyCalendarKind::JapaneseExtended => AnyCalendar::JapaneseExtended(
                JapaneseExtended::try_new_with_buffer_provider(provider)?,
            ),
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Roc => AnyCalendar::Roc(Roc),
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P, kind: AnyCalendarKind) -> Result<Self, DataError>
    where
        P: DataProvider<crate::provider::CalendarJapaneseModernV1>
            + DataProvider<crate::provider::CalendarJapaneseExtendedV1>
            + ?Sized,
    {
        Ok(match kind {
            AnyCalendarKind::Buddhist => AnyCalendar::Buddhist(Buddhist),
            AnyCalendarKind::Chinese => AnyCalendar::Chinese(LunarChinese::new_china()),
            AnyCalendarKind::Coptic => AnyCalendar::Coptic(Coptic),
            AnyCalendarKind::Dangi => AnyCalendar::Dangi(LunarChinese::new_korea()),
            AnyCalendarKind::Ethiopian => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            AnyCalendarKind::EthiopianAmeteAlem => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
            AnyCalendarKind::Gregorian => AnyCalendar::Gregorian(Gregorian),
            AnyCalendarKind::Hebrew => AnyCalendar::Hebrew(Hebrew),
            AnyCalendarKind::Indian => AnyCalendar::Indian(Indian),
            AnyCalendarKind::HijriTabularTypeIIFriday => {
                AnyCalendar::HijriTabular(Hijri::new_tabular(
                    hijri::TabularAlgorithmLeapYears::TypeII,
                    hijri::TabularAlgorithmEpoch::Friday,
                ))
            }
            AnyCalendarKind::HijriSimulatedMecca => {
                AnyCalendar::HijriSimulated(Hijri::new_simulated_mecca())
            }
            AnyCalendarKind::HijriTabularTypeIIThursday => {
                AnyCalendar::HijriTabular(Hijri::new_tabular(
                    hijri::TabularAlgorithmLeapYears::TypeII,
                    hijri::TabularAlgorithmEpoch::Thursday,
                ))
            }
            AnyCalendarKind::HijriUmmAlQura => {
                AnyCalendar::HijriUmmAlQura(Hijri::new_umm_al_qura())
            }
            AnyCalendarKind::Iso => AnyCalendar::Iso(Iso),
            AnyCalendarKind::Japanese => {
                AnyCalendar::Japanese(Japanese::try_new_unstable(provider)?)
            }
            AnyCalendarKind::JapaneseExtended => {
                AnyCalendar::JapaneseExtended(JapaneseExtended::try_new_unstable(provider)?)
            }
            AnyCalendarKind::Persian => AnyCalendar::Persian(Persian),
            AnyCalendarKind::Roc => AnyCalendar::Roc(Roc),
        })
    }

    /// The [`AnyCalendarKind`] corresponding to the calendar this contains
    pub fn kind(&self) -> AnyCalendarKind {
        match *self {
            Self::Buddhist(_) => AnyCalendarKind::Buddhist,
            Self::Chinese(_) => AnyCalendarKind::Chinese,
            Self::Coptic(_) => AnyCalendarKind::Coptic,
            Self::Dangi(_) => AnyCalendarKind::Dangi,
            Self::Ethiopian(ref e) => IntoAnyCalendar::kind(e),
            Self::Gregorian(_) => AnyCalendarKind::Gregorian,
            Self::Hebrew(_) => AnyCalendarKind::Hebrew,
            Self::Indian(_) => AnyCalendarKind::Indian,
            Self::HijriTabular(ref h) => IntoAnyCalendar::kind(h),
            Self::HijriSimulated(ref h) => IntoAnyCalendar::kind(h),
            Self::HijriUmmAlQura(_) => AnyCalendarKind::HijriUmmAlQura,
            Self::Iso(_) => AnyCalendarKind::Iso,
            Self::Japanese(_) => AnyCalendarKind::Japanese,
            Self::JapaneseExtended(_) => AnyCalendarKind::JapaneseExtended,
            Self::Persian(_) => AnyCalendarKind::Persian,
            Self::Roc(_) => AnyCalendarKind::Roc,
        }
    }
}

impl<C: AsCalendar<Calendar = AnyCalendar>> Date<C> {
    /// Convert this `Date<AnyCalendar>` to another `AnyCalendar`, if conversion is needed
    pub fn convert_any<'a>(&self, calendar: &'a AnyCalendar) -> Date<Ref<'a, AnyCalendar>> {
        if calendar.kind() != self.calendar.as_calendar().kind() {
            Date::new_from_iso(self.to_iso(), Ref(calendar))
        } else {
            Date {
                inner: self.inner,
                calendar: Ref(calendar),
            }
        }
    }
}

/// Convenient type for selecting the kind of AnyCalendar to construct
#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AnyCalendarKind {
    /// The kind of a [`Buddhist`] calendar
    ///
    /// This corresponds to the `"buddhist"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Buddhist,
    /// The kind of a [`Chinese`] calendar
    ///
    /// This corresponds to the `"chinese"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Chinese,
    /// The kind of a [`Coptic`] calendar
    ///
    /// This corresponds to the `"coptic"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Coptic,
    /// The kind of a [`Dangi`] calendar
    ///
    /// This corresponds to the `"dangi"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Dangi,
    /// The kind of an [`Ethiopian`] calendar, with Amete Mihret era
    ///
    /// This corresponds to the `"ethiopic"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Ethiopian,
    /// The kind of an [`Ethiopian`] calendar, with Amete Alem era
    ///
    /// This corresponds to the `"ethioaa"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    EthiopianAmeteAlem,
    /// The kind of a [`Gregorian`] calendar
    ///
    /// This corresponds to the `"gregory"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Gregorian,
    /// The kind of a [`Hebrew`] calendar
    ///
    /// This corresponds to the `"hebrew"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Hebrew,
    /// The kind of a [`Indian`] calendar
    ///
    /// This corresponds to the `"indian"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Indian,
    /// The kind of an [`HijriTabular`] calendar using [`HijriTabularLeapYears::TypeII`] and [`HijriTabularEpoch::Friday`]
    ///
    /// This corresponds to the `"islamic-civil"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    HijriTabularTypeIIFriday,
    /// The kind of an [`HijriSimulated`], Mecca calendar
    ///
    /// This corresponds to the `"islamic-rgsa"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    HijriSimulatedMecca,
    /// The kind of an [`HijriTabular`] calendar using [`HijriTabularLeapYears::TypeII`] and [`HijriTabularEpoch::Thursday`]
    ///
    /// This corresponds to the `"islamic-tbla"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    HijriTabularTypeIIThursday,
    /// The kind of an [`HijriUmmAlQura`] calendar
    ///
    /// This corresponds to the `"islamic-umalqura"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    HijriUmmAlQura,
    /// The kind of an [`Iso`] calendar
    ///
    /// This corresponds to the `"iso8601"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Iso,
    /// The kind of a [`Japanese`] calendar
    ///
    /// This corresponds to the `"japanese"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Japanese,
    /// The kind of a [`JapaneseExtended`] calendar
    ///
    /// This corresponds to the `"japanext"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    JapaneseExtended,
    /// The kind of a [`Persian`] calendar
    ///
    /// This corresponds to the `"persian"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Persian,
    /// The kind of a [`Roc`] calendar
    ///
    /// This corresponds to the `"roc"` [CLDR calendar](https://unicode.org/reports/tr35/#UnicodeCalendarIdentifier).
    Roc,
}

impl AnyCalendarKind {
    /// Selects the [`AnyCalendarKind`] appropriate for the given [`CalendarPreferences`].
    pub fn new(prefs: CalendarPreferences) -> Self {
        let algo = prefs.calendar_algorithm;
        let region = prefs.locale_preferences.region();
        if let Some(kind) = algo.and_then(|a| a.try_into().ok()) {
            return kind;
        }
        if region == Some(region!("TH")) {
            AnyCalendarKind::Buddhist
        } else if region == Some(region!("AF")) || region == Some(region!("IR")) {
            AnyCalendarKind::Persian
        } else if region == Some(region!("SA")) && algo == Some(CalendarAlgorithm::Hijri(None)) {
            AnyCalendarKind::HijriUmmAlQura
        } else {
            AnyCalendarKind::Gregorian
        }
    }
}

impl TryFrom<CalendarAlgorithm> for AnyCalendarKind {
    type Error = ();
    fn try_from(v: CalendarAlgorithm) -> Result<Self, Self::Error> {
        use CalendarAlgorithm::*;
        match v {
            Buddhist => Ok(AnyCalendarKind::Buddhist),
            Chinese => Ok(AnyCalendarKind::Chinese),
            Coptic => Ok(AnyCalendarKind::Coptic),
            Dangi => Ok(AnyCalendarKind::Dangi),
            Ethioaa => Ok(AnyCalendarKind::EthiopianAmeteAlem),
            Ethiopic => Ok(AnyCalendarKind::Ethiopian),
            Gregory => Ok(AnyCalendarKind::Gregorian),
            Hebrew => Ok(AnyCalendarKind::Hebrew),
            Indian => Ok(AnyCalendarKind::Indian),
            Hijri(None) => Err(()),
            Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => Ok(AnyCalendarKind::HijriUmmAlQura),
            Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                Ok(AnyCalendarKind::HijriTabularTypeIIThursday)
            }
            Hijri(Some(HijriCalendarAlgorithm::Civil)) => {
                Ok(AnyCalendarKind::HijriTabularTypeIIFriday)
            }
            Hijri(Some(HijriCalendarAlgorithm::Rgsa)) => Ok(AnyCalendarKind::HijriSimulatedMecca),
            Iso8601 => Ok(AnyCalendarKind::Iso),
            Japanese => Ok(AnyCalendarKind::Japanese),
            Persian => Ok(AnyCalendarKind::Persian),
            Roc => Ok(AnyCalendarKind::Roc),
            _ => {
                debug_assert!(false, "unknown calendar algorithm {v:?}");
                Err(())
            }
        }
    }
}

impl fmt::Display for AnyCalendarKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Trait for calendars that may be converted to [`AnyCalendar`]
pub trait IntoAnyCalendar: Calendar + Sized {
    /// Convert this calendar into an [`AnyCalendar`], moving it
    ///
    /// You should not need to call this method directly
    fn to_any(self) -> AnyCalendar;

    /// The [`AnyCalendarKind`] enum variant associated with this calendar
    fn kind(&self) -> AnyCalendarKind;

    /// Move an [`AnyCalendar`] into a `Self`, or returning it as an error
    /// if the types do not match.
    ///
    /// You should not need to call this method directly
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar>;

    /// Convert an [`AnyCalendar`] reference into a `Self` reference.
    ///
    /// You should not need to call this method directly
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self>;

    /// Convert a date for this calendar into an `AnyDateInner`
    ///
    /// You should not need to call this method directly
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner;
}

impl IntoAnyCalendar for AnyCalendar {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        self
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        self.kind()
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        Ok(any)
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        Some(any)
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        *d
    }
}

impl IntoAnyCalendar for Buddhist {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Buddhist(Buddhist)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Buddhist
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Buddhist(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Buddhist(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Buddhist(*d)
    }
}

impl From<Buddhist> for AnyCalendar {
    fn from(value: Buddhist) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for LunarChinese<chinese::China> {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Chinese(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Chinese
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Chinese(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Chinese(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Chinese(*d)
    }
}

impl From<LunarChinese<chinese::China>> for AnyCalendar {
    fn from(value: LunarChinese<chinese::China>) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Coptic {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Coptic(Coptic)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Coptic
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Coptic(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Coptic(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Coptic(*d)
    }
}

impl From<Coptic> for AnyCalendar {
    fn from(value: Coptic) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for LunarChinese<chinese::Korea> {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Dangi(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Dangi
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Dangi(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Dangi(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Dangi(*d)
    }
}

impl From<LunarChinese<chinese::Korea>> for AnyCalendar {
    fn from(value: LunarChinese<chinese::Korea>) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Ethiopian {
    // Amete Mihret calendars are the default
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Ethiopian(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        match self.era_style() {
            EthiopianEraStyle::AmeteAlem => AnyCalendarKind::EthiopianAmeteAlem,
            EthiopianEraStyle::AmeteMihret => AnyCalendarKind::Ethiopian,
        }
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Ethiopian(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Ethiopian(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Ethiopian(*d)
    }
}

impl From<Ethiopian> for AnyCalendar {
    fn from(value: Ethiopian) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Gregorian {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Gregorian(Gregorian)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Gregorian
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Gregorian(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Gregorian(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Gregorian(*d)
    }
}

impl From<Gregorian> for AnyCalendar {
    fn from(value: Gregorian) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Hebrew {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Hebrew(Hebrew)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Hebrew
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Hebrew(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Hebrew(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Hebrew(*d)
    }
}

impl From<Hebrew> for AnyCalendar {
    fn from(value: Hebrew) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Indian {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Indian(Indian)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Indian
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Indian(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Indian(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Indian(*d)
    }
}

impl From<Indian> for AnyCalendar {
    fn from(value: Indian) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Hijri<hijri::TabularAlgorithm> {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::HijriTabular(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        match self.0 {
            hijri::TabularAlgorithm {
                leap_years: hijri::TabularAlgorithmLeapYears::TypeII,
                epoch: hijri::TabularAlgorithmEpoch::Friday,
            } => AnyCalendarKind::HijriTabularTypeIIFriday,
            hijri::TabularAlgorithm {
                leap_years: hijri::TabularAlgorithmLeapYears::TypeII,
                epoch: hijri::TabularAlgorithmEpoch::Thursday,
            } => AnyCalendarKind::HijriTabularTypeIIThursday,
        }
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::HijriTabular(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::HijriTabular(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::HijriTabular(*d, self.0)
    }
}

impl From<Hijri<hijri::TabularAlgorithm>> for AnyCalendar {
    fn from(value: Hijri<hijri::TabularAlgorithm>) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Hijri<hijri::AstronomicalSimulation> {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::HijriSimulated(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        match self.0.location {
            crate::cal::hijri_internal::SimulatedLocation::Mecca => {
                AnyCalendarKind::HijriSimulatedMecca
            }
        }
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::HijriSimulated(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::HijriSimulated(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::HijriSimulated(*d)
    }
}

impl From<Hijri<hijri::AstronomicalSimulation>> for AnyCalendar {
    fn from(value: Hijri<hijri::AstronomicalSimulation>) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Hijri<hijri::UmmAlQura> {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::HijriUmmAlQura(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::HijriUmmAlQura
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::HijriUmmAlQura(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::HijriUmmAlQura(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::HijriUmmAlQura(*d)
    }
}

impl From<Hijri<hijri::UmmAlQura>> for AnyCalendar {
    fn from(value: Hijri<hijri::UmmAlQura>) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Iso {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Iso(Iso)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Iso
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Iso(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Iso(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Iso(*d)
    }
}

impl From<Iso> for AnyCalendar {
    fn from(value: Iso) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Japanese {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Japanese(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Japanese
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Japanese(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Japanese(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Japanese(*d)
    }
}

impl From<Japanese> for AnyCalendar {
    fn from(value: Japanese) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for JapaneseExtended {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::JapaneseExtended(self)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::JapaneseExtended
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::JapaneseExtended(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::JapaneseExtended(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::JapaneseExtended(*d)
    }
}

impl From<JapaneseExtended> for AnyCalendar {
    fn from(value: JapaneseExtended) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Persian {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Persian(Persian)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Persian
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Persian(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Persian(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Persian(*d)
    }
}

impl From<Persian> for AnyCalendar {
    fn from(value: Persian) -> AnyCalendar {
        value.to_any()
    }
}

impl IntoAnyCalendar for Roc {
    #[inline]
    fn to_any(self) -> AnyCalendar {
        AnyCalendar::Roc(Roc)
    }
    #[inline]
    fn kind(&self) -> AnyCalendarKind {
        AnyCalendarKind::Roc
    }
    #[inline]
    fn from_any(any: AnyCalendar) -> Result<Self, AnyCalendar> {
        if let AnyCalendar::Roc(cal) = any {
            Ok(cal)
        } else {
            Err(any)
        }
    }
    #[inline]
    fn from_any_ref(any: &AnyCalendar) -> Option<&Self> {
        if let AnyCalendar::Roc(cal) = any {
            Some(cal)
        } else {
            None
        }
    }
    #[inline]
    fn date_to_any(&self, d: &Self::DateInner) -> AnyDateInner {
        AnyDateInner::Roc(*d)
    }
}

impl From<Roc> for AnyCalendar {
    fn from(value: Roc) -> AnyCalendar {
        value.to_any()
    }
}

#[cfg(test)]
mod tests {
    use tinystr::tinystr;
    use types::MonthCode;

    use super::*;
    use crate::Ref;

    #[track_caller]
    fn single_test_roundtrip(
        calendar: Ref<AnyCalendar>,
        era: Option<(&str, Option<u8>)>,
        year: i32,
        month_code: &str,
        day: u8,
    ) {
        let month = types::MonthCode(month_code.parse().expect("month code must parse"));

        let date = Date::try_new_from_codes(era.map(|x| x.0), year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to construct date for {} with {era:?}, {year}, {month}, {day}: {e:?}",
                    calendar.debug_name(),
                )
            });

        let roundtrip_year = date.year();
        let roundtrip_month = date.month().standard_code;
        let roundtrip_day = date.day_of_month().0;

        assert_eq!(
            (month, day),
            (roundtrip_month, roundtrip_day),
            "Failed to roundtrip for calendar {}",
            calendar.debug_name()
        );

        if let Some((era_code, era_index)) = era {
            let roundtrip_era_year = date.year().era().expect("year type should be era");

            let roundtrip_year = roundtrip_year.era_year_or_related_iso();
            assert_eq!(
                (era_code, era_index, year),
                (
                    roundtrip_era_year.era.as_str(),
                    roundtrip_era_year.era_index,
                    roundtrip_year
                ),
                "Failed to roundtrip era for calendar {}",
                calendar.debug_name()
            )
        } else {
            assert_eq!(
                year,
                date.extended_year(),
                "Failed to roundtrip year for calendar {}",
                calendar.debug_name()
            );
        }

        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            date, reconstructed,
            "Failed to roundtrip via iso with {era:?}, {year}, {month}, {day}"
        )
    }

    #[track_caller]
    fn single_test_error(
        calendar: Ref<AnyCalendar>,
        era: Option<(&str, Option<u8>)>,
        year: i32,
        month_code: &str,
        day: u8,
        error: DateError,
    ) {
        let month = types::MonthCode(month_code.parse().expect("month code must parse"));

        let date = Date::try_new_from_codes(era.map(|x| x.0), year, month, day, calendar);
        assert_eq!(
            date,
            Err(error),
            "Construction with {era:?}, {year}, {month}, {day} did not return {error:?}"
        )
    }

    #[test]
    fn test_any_construction() {
        let buddhist = AnyCalendar::new(AnyCalendarKind::Buddhist);
        let chinese = AnyCalendar::new(AnyCalendarKind::Chinese);
        let coptic = AnyCalendar::new(AnyCalendarKind::Coptic);
        let dangi = AnyCalendar::new(AnyCalendarKind::Dangi);
        let ethioaa = AnyCalendar::new(AnyCalendarKind::EthiopianAmeteAlem);
        let ethiopian = AnyCalendar::new(AnyCalendarKind::Ethiopian);
        let gregorian = AnyCalendar::new(AnyCalendarKind::Gregorian);
        let hebrew = AnyCalendar::new(AnyCalendarKind::Hebrew);
        let indian = AnyCalendar::new(AnyCalendarKind::Indian);
        let hijri_civil: AnyCalendar = AnyCalendar::new(AnyCalendarKind::HijriTabularTypeIIFriday);
        let hijri_simulated: AnyCalendar = AnyCalendar::new(AnyCalendarKind::HijriSimulatedMecca);
        let hijri_astronomical: AnyCalendar =
            AnyCalendar::new(AnyCalendarKind::HijriTabularTypeIIThursday);
        let hijri_umm_al_qura: AnyCalendar = AnyCalendar::new(AnyCalendarKind::HijriUmmAlQura);
        let japanese = AnyCalendar::new(AnyCalendarKind::Japanese);
        let japanext = AnyCalendar::new(AnyCalendarKind::JapaneseExtended);
        let persian = AnyCalendar::new(AnyCalendarKind::Persian);
        let roc = AnyCalendar::new(AnyCalendarKind::Roc);
        let buddhist = Ref(&buddhist);
        let chinese = Ref(&chinese);
        let coptic = Ref(&coptic);
        let dangi = Ref(&dangi);
        let ethioaa = Ref(&ethioaa);
        let ethiopian = Ref(&ethiopian);
        let gregorian = Ref(&gregorian);
        let hebrew = Ref(&hebrew);
        let indian = Ref(&indian);
        let hijri_civil = Ref(&hijri_civil);
        let hijri_simulated = Ref(&hijri_simulated);
        let hijri_astronomical = Ref(&hijri_astronomical);
        let hijri_umm_al_qura = Ref(&hijri_umm_al_qura);
        let japanese = Ref(&japanese);
        let japanext = Ref(&japanext);
        let persian = Ref(&persian);
        let roc = Ref(&roc);

        single_test_roundtrip(buddhist, Some(("be", Some(0))), 100, "M03", 1);
        single_test_roundtrip(buddhist, None, 100, "M03", 1);
        single_test_roundtrip(buddhist, None, -100, "M03", 1);
        single_test_roundtrip(buddhist, Some(("be", Some(0))), -100, "M03", 1);
        single_test_error(
            buddhist,
            Some(("be", Some(0))),
            100,
            "M13",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M13"))),
        );

        single_test_roundtrip(coptic, Some(("am", Some(0))), 100, "M03", 1);
        single_test_roundtrip(coptic, None, 2000, "M03", 1);
        single_test_roundtrip(coptic, None, -100, "M03", 1);
        single_test_roundtrip(coptic, Some(("am", Some(0))), -99, "M03", 1);
        single_test_roundtrip(coptic, Some(("am", Some(0))), 100, "M13", 1);
        single_test_error(
            coptic,
            Some(("am", Some(0))),
            100,
            "M14",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M14"))),
        );

        single_test_roundtrip(ethiopian, Some(("am", Some(1))), 100, "M03", 1);
        single_test_roundtrip(ethiopian, None, 2000, "M03", 1);
        single_test_roundtrip(ethiopian, None, -100, "M03", 1);
        single_test_roundtrip(ethiopian, Some(("am", Some(1))), 2000, "M13", 1);
        single_test_roundtrip(ethiopian, Some(("aa", Some(0))), 5400, "M03", 1);
        // Since #6910, the era range is not enforced in try_from_codes
        /*
        single_test_error(
            ethiopian,
            Some(("am", Some(0))),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        single_test_error(
            ethiopian,
            Some(("aa", Some(0))),
            5600,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 5600,
                min: i32::MIN,
                max: 5500,
            },
        );
        */
        single_test_error(
            ethiopian,
            Some(("am", Some(0))),
            100,
            "M14",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M14"))),
        );

        single_test_roundtrip(ethioaa, Some(("aa", Some(0))), 7000, "M13", 1);
        single_test_roundtrip(ethioaa, None, 7000, "M13", 1);
        single_test_roundtrip(ethioaa, None, -100, "M13", 1);
        single_test_roundtrip(ethioaa, Some(("aa", Some(0))), 100, "M03", 1);
        single_test_error(
            ethiopian,
            Some(("aa", Some(0))),
            100,
            "M14",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M14"))),
        );

        single_test_roundtrip(gregorian, Some(("ce", Some(1))), 100, "M03", 1);
        single_test_roundtrip(gregorian, None, 2000, "M03", 1);
        single_test_roundtrip(gregorian, None, -100, "M03", 1);
        single_test_roundtrip(gregorian, Some(("bce", Some(0))), 100, "M03", 1);
        // Since #6910, the era range is not enforced in try_from_codes
        /*
        single_test_error(
            gregorian,
            Some(("ce", Some(1))),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        single_test_error(
            gregorian,
            Some(("bce", Some(0))),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        */
        single_test_error(
            gregorian,
            Some(("bce", Some(0))),
            100,
            "M13",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M13"))),
        );

        single_test_roundtrip(indian, Some(("shaka", Some(0))), 100, "M03", 1);
        single_test_roundtrip(indian, None, 2000, "M12", 1);
        single_test_roundtrip(indian, None, -100, "M03", 1);
        single_test_roundtrip(indian, Some(("shaka", Some(0))), 0, "M03", 1);
        single_test_error(
            indian,
            Some(("shaka", Some(0))),
            100,
            "M13",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M13"))),
        );

        single_test_roundtrip(chinese, None, 400, "M02", 5);
        single_test_roundtrip(chinese, None, 4660, "M07", 29);
        single_test_roundtrip(chinese, None, -100, "M11", 12);
        single_test_error(
            chinese,
            None,
            4658,
            "M13",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M13"))),
        );

        single_test_roundtrip(dangi, None, 400, "M02", 5);
        single_test_roundtrip(dangi, None, 4660, "M08", 29);
        single_test_roundtrip(dangi, None, -1300, "M11", 12);
        single_test_error(
            dangi,
            None,
            10393,
            "M00L",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M00L"))),
        );

        single_test_roundtrip(japanese, Some(("reiwa", None)), 3, "M03", 1);
        single_test_roundtrip(japanese, Some(("heisei", None)), 6, "M12", 1);
        single_test_roundtrip(japanese, Some(("meiji", None)), 10, "M03", 1);
        single_test_roundtrip(japanese, Some(("ce", None)), 1000, "M03", 1);
        single_test_roundtrip(japanese, None, 1000, "M03", 1);
        single_test_roundtrip(japanese, None, -100, "M03", 1);
        single_test_roundtrip(japanese, None, 2024, "M03", 1);
        single_test_roundtrip(japanese, Some(("bce", None)), 10, "M03", 1);
        // Since #6910, the era range is not enforced in try_from_codes
        /*
        single_test_error(
            japanese,
            Some(("ce", None)),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        single_test_error(
            japanese,
            Some(("bce", Some(0))),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        */
        single_test_error(
            japanese,
            Some(("reiwa", None)),
            2,
            "M13",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M13"))),
        );

        single_test_roundtrip(japanext, Some(("reiwa", None)), 3, "M03", 1);
        single_test_roundtrip(japanext, Some(("heisei", None)), 6, "M12", 1);
        single_test_roundtrip(japanext, Some(("meiji", None)), 10, "M03", 1);
        single_test_roundtrip(japanext, Some(("tenpyokampo-749", None)), 1, "M04", 20);
        single_test_roundtrip(japanext, Some(("ce", None)), 100, "M03", 1);
        single_test_roundtrip(japanext, Some(("bce", None)), 10, "M03", 1);
        // Since #6910, the era range is not enforced in try_from_codes
        /*
        single_test_error(
            japanext,
            Some(("ce", None)),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        single_test_error(
            japanext,
            Some(("bce", Some(0))),
            0,
            "M03",
            1,
            DateError::Range {
                field: "year",
                value: 0,
                min: 1,
                max: i32::MAX,
            },
        );
        */
        single_test_error(
            japanext,
            Some(("reiwa", None)),
            2,
            "M13",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M13"))),
        );

        single_test_roundtrip(persian, Some(("ap", Some(0))), 477, "M03", 1);
        single_test_roundtrip(persian, None, 2083, "M07", 21);
        single_test_roundtrip(persian, None, -100, "M07", 21);
        single_test_roundtrip(persian, Some(("ap", Some(0))), 1600, "M12", 20);
        single_test_error(
            persian,
            Some(("ap", Some(0))),
            100,
            "M9",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M9"))),
        );

        single_test_roundtrip(hebrew, Some(("am", Some(0))), 5773, "M03", 1);
        single_test_roundtrip(hebrew, None, 4993, "M07", 21);
        single_test_roundtrip(hebrew, None, -100, "M07", 21);
        single_test_roundtrip(hebrew, Some(("am", Some(0))), 5012, "M12", 20);
        single_test_error(
            hebrew,
            Some(("am", Some(0))),
            100,
            "M9",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M9"))),
        );

        single_test_roundtrip(roc, Some(("roc", Some(1))), 10, "M05", 3);
        single_test_roundtrip(roc, Some(("broc", Some(0))), 15, "M01", 10);
        single_test_roundtrip(roc, None, 100, "M10", 30);
        single_test_roundtrip(roc, None, -100, "M10", 30);

        single_test_roundtrip(hijri_simulated, Some(("ah", Some(0))), 477, "M03", 1);
        single_test_roundtrip(hijri_simulated, None, 2083, "M07", 21);
        single_test_roundtrip(hijri_simulated, None, -100, "M07", 21);
        single_test_roundtrip(hijri_simulated, Some(("ah", Some(0))), 1600, "M12", 20);
        single_test_error(
            hijri_simulated,
            Some(("ah", Some(0))),
            100,
            "M9",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M9"))),
        );

        single_test_roundtrip(hijri_civil, Some(("ah", Some(0))), 477, "M03", 1);
        single_test_roundtrip(hijri_civil, None, 2083, "M07", 21);
        single_test_roundtrip(hijri_civil, None, -100, "M07", 21);
        single_test_roundtrip(hijri_civil, Some(("ah", Some(0))), 1600, "M12", 20);
        single_test_error(
            hijri_civil,
            Some(("ah", Some(0))),
            100,
            "M9",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M9"))),
        );

        single_test_roundtrip(hijri_umm_al_qura, Some(("ah", Some(0))), 477, "M03", 1);
        single_test_roundtrip(hijri_umm_al_qura, None, 2083, "M07", 21);
        single_test_roundtrip(hijri_umm_al_qura, None, -100, "M07", 21);
        single_test_roundtrip(hijri_umm_al_qura, Some(("ah", Some(0))), 1600, "M12", 20);
        single_test_error(
            hijri_umm_al_qura,
            Some(("ah", Some(0))),
            100,
            "M9",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M9"))),
        );

        single_test_roundtrip(hijri_astronomical, Some(("ah", Some(0))), 477, "M03", 1);
        single_test_roundtrip(hijri_astronomical, None, 2083, "M07", 21);
        single_test_roundtrip(hijri_astronomical, None, -100, "M07", 21);
        single_test_roundtrip(hijri_astronomical, Some(("ah", Some(0))), 1600, "M12", 20);
        single_test_error(
            hijri_astronomical,
            Some(("ah", Some(0))),
            100,
            "M9",
            1,
            DateError::UnknownMonthCode(MonthCode(tinystr!(4, "M9"))),
        );
    }
}
