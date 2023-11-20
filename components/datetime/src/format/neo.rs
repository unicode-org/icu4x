// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::datetime::write_pattern;
use crate::calendar::CldrCalendar;
use crate::error::DateTimeError as Error;
use crate::fields;
use crate::input;
use crate::input::DateInput;
use crate::input::DateTimeInput;
use crate::input::DateTimeInputWithWeekConfig;
use crate::input::ExtractedDateTimeInput;
use crate::input::IsoTimeInput;
use crate::pattern::runtime::Pattern;
use crate::provider::date_time::{DateSymbols, MonthPlaceholderValue, TimeSymbols};
use crate::provider::neo::*;
use core::fmt;
use icu_calendar::types::Era;
use icu_calendar::types::MonthCode;
use icu_calendar::week::WeekCalculator;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::options::GroupingStrategy;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;
use writeable::Writeable;

/// This can be extended in the future to support multiple lengths.
/// For now, this type wraps a symbols object tagged with a single length. See #4337
#[derive(Debug, Copy, Clone)]
enum OptionalNames<S, T> {
    None,
    SingleLength(S, fields::FieldLength, T),
}

enum NamePresence {
    /// The data is not loaded
    NotLoaded,
    /// The data matches and is already loaded
    Loaded,
    /// There is data loaded which does not match
    Mismatched,
}

impl<S, T> OptionalNames<S, T>
where
    S: Copy + PartialEq,
{
    pub(crate) fn check_with_length(
        &self,
        field_symbol: S,
        field_length: fields::FieldLength,
    ) -> NamePresence {
        match self {
            Self::SingleLength(actual_field_symbol, actual_length, _)
                if field_symbol == *actual_field_symbol && field_length == *actual_length =>
            {
                NamePresence::Loaded
            }
            OptionalNames::SingleLength(_, _, _) => NamePresence::Mismatched,
            OptionalNames::None => NamePresence::NotLoaded,
        }
    }
}

impl<S, T> OptionalNames<S, T>
where
    S: Copy + PartialEq,
    T: Copy,
{
    pub(crate) fn get_with_length(
        &self,
        field_symbol: S,
        field_length: fields::FieldLength,
    ) -> Option<T> {
        match self {
            Self::None => None,
            Self::SingleLength(actual_field_symbol, actual_length, t)
                if field_symbol == *actual_field_symbol && field_length == *actual_length =>
            {
                Some(*t)
            }
            _ => None,
        }
    }
}

impl<S, M> OptionalNames<S, DataPayload<M>>
where
    S: Copy,
    M: KeyedDataMarker,
{
    pub(crate) fn as_borrowed(
        &self,
    ) -> OptionalNames<S, &<M::Yokeable as icu_provider::yoke::Yokeable>::Output> {
        match self {
            Self::None => OptionalNames::None,
            Self::SingleLength(field_symbol, field_length, payload) => {
                OptionalNames::SingleLength(*field_symbol, *field_length, payload.get())
            }
        }
    }
}

/// A low-level type that formats datetime patterns with localized symbols.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
///
/// # Examples
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::DateTime;
/// use icu::datetime::TypedDateTimePatternInterpolator;
/// use icu::datetime::fields::FieldLength;
/// use icu::datetime::fields;
/// use icu::datetime::pattern;
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// // Create an interpolator that can format abbreviated month, weekday, and day period names:
/// let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
///     TypedDateTimePatternInterpolator::try_new(&locale!("uk").into()).unwrap();
/// interpolator
///     .include_month_names(fields::Month::Format, FieldLength::Abbreviated)
///     .unwrap()
///     .include_weekday_names(fields::Weekday::Format, FieldLength::Abbreviated)
///     .unwrap()
///     .include_day_period_names(FieldLength::Abbreviated)
///     .unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "E MMM d y -- h:mm a";
/// let reference_pattern: pattern::reference::Pattern = pattern_str.parse().unwrap();
/// let pattern: pattern::runtime::Pattern = (&reference_pattern).into();
///
/// // Test it:
/// let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 20, 11, 35, 3).unwrap();
/// assert_writeable_eq!(interpolator.format(&pattern, &datetime), "пн лист. 20 2023 -- 11:35 дп");
/// ```
///
/// If the correct data is not loaded, and error will occur:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::DateTime;
/// use icu::datetime::TypedDateTimePatternInterpolator;
/// use icu::datetime::fields::FieldLength;
/// use icu::datetime::fields;
/// use icu::datetime::pattern;
/// use icu::locid::locale;
/// use writeable::Writeable;
///
/// // Create an interpolator that can format abbreviated month, weekday, and day period names:
/// let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
///     TypedDateTimePatternInterpolator::try_new(&locale!("en").into()).unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "'It is:' E MMM d y 'at' h:mm a";
/// let reference_pattern: pattern::reference::Pattern = pattern_str.parse().unwrap();
/// let pattern: pattern::runtime::Pattern = (&reference_pattern).into();
///
/// // The pattern string contains lots of symbols including "E", "MMM", and "a", but we did not load any data!
/// let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 20, 11, 35, 3).unwrap();
/// let mut buffer = String::new();
/// assert!(interpolator.format(&pattern, &datetime).write_to(&mut buffer).is_err());
/// ```
#[derive(Debug)]
pub struct TypedDateTimePatternInterpolator<C: CldrCalendar> {
    locale: DataLocale,
    /// `year_symbols` is different because it could be either era or cyclic year.
    year_symbols: OptionalNames<(), DataPayload<C::YearSymbolsV1Marker>>,
    month_symbols: OptionalNames<fields::Month, DataPayload<C::MonthSymbolsV1Marker>>,
    weekday_symbols: OptionalNames<fields::Weekday, DataPayload<WeekdaySymbolsV1Marker>>,
    dayperiod_symbols: OptionalNames<(), DataPayload<DayPeriodSymbolsV1Marker>>,
    // TODO: Make the FixedDecimalFormatter optional?
    fixed_decimal_formatter: FixedDecimalFormatter,
    week_calculator: Option<WeekCalculator>,
}

impl<C: CldrCalendar> TypedDateTimePatternInterpolator<C> {
    /// Constructor that takes a selected locale and creates an empty pattern interpolator.
    ///
    /// For an example, see [`TypedDateTimePatternInterpolator`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale) -> Result<Self, Error> {
        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, fixed_decimal_format_options)?;
        Ok(Self::new_internal(locale.clone(), fixed_decimal_formatter))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(provider: &P, locale: &DataLocale) -> Result<Self, Error>
    where
        P: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            fixed_decimal_format_options,
        )?;
        Ok(Self::new_internal(locale.clone(), fixed_decimal_formatter))
    }

    fn new_internal(locale: DataLocale, fixed_decimal_formatter: FixedDecimalFormatter) -> Self {
        TypedDateTimePatternInterpolator {
            locale,
            year_symbols: OptionalNames::None,
            month_symbols: OptionalNames::None,
            weekday_symbols: OptionalNames::None,
            dayperiod_symbols: OptionalNames::None,
            fixed_decimal_formatter,
            week_calculator: None,
        }
    }

    /// Loads year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_year_names<P>(
        &mut self,
        provider: &P,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<C::YearSymbolsV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: fields::FieldSymbol::Era,
            length: field_length,
        };
        // UTS 35 says that "G..GGG" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match self.year_symbols.check_with_length((), field_length) {
            NamePresence::Loaded => return Ok(self),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = self.locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::subtag_for(
            aux::Context::Format,
            match field_length {
                fields::FieldLength::Abbreviated => aux::Length::Abbr,
                fields::FieldLength::Narrow => aux::Length::Narrow,
                fields::FieldLength::Wide => aux::Length::Wide,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        self.year_symbols = OptionalNames::SingleLength((), field_length, payload);
        Ok(self)
    }

    /// Includes year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::DateTimeError;
    /// use icu::datetime::TypedDateTimePatternInterpolator;
    /// use icu::locid::locale;
    ///
    /// let mut interpolator =
    ///     TypedDateTimePatternInterpolator::<Gregorian>::try_new(
    ///         &locale!("und").into(),
    ///     )
    ///     .unwrap();
    ///
    /// // First length is successful:
    /// interpolator.include_year_names(FieldLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// interpolator.include_year_names(FieldLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     interpolator.include_year_names(FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_year_names(
        &mut self,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked:
            icu_provider::DataProvider<<C as CldrCalendar>::YearSymbolsV1Marker>,
    {
        self.load_year_names(&crate::provider::Baked, field_length)
    }

    /// Loads month names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_month_names<P>(
        &mut self,
        provider: &P,
        field_symbol: fields::Month,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<C::MonthSymbolsV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: fields::FieldSymbol::Month(field_symbol),
            length: field_length,
        };
        // Note: UTS 35 says that "M..MM" and "L..LL" are numeric
        match self
            .month_symbols
            .check_with_length(field_symbol, field_length)
        {
            NamePresence::Loaded => return Ok(self),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = self.locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::subtag_for(
            match field_symbol {
                fields::Month::Format => aux::Context::Format,
                fields::Month::StandAlone => aux::Context::Standalone,
            },
            match field_length {
                fields::FieldLength::Abbreviated => aux::Length::Abbr,
                fields::FieldLength::Narrow => aux::Length::Narrow,
                fields::FieldLength::Wide => aux::Length::Wide,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        self.month_symbols = OptionalNames::SingleLength(field_symbol, field_length, payload);
        Ok(self)
    }

    /// Includes month names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::DateTimeError;
    /// use icu::datetime::TypedDateTimePatternInterpolator;
    /// use icu::locid::locale;
    ///
    /// let mut interpolator =
    ///     TypedDateTimePatternInterpolator::<Gregorian>::try_new(
    ///         &locale!("und").into(),
    ///     )
    ///     .unwrap();
    /// let field_symbol = icu::datetime::fields::Month::Format;
    /// let alt_field_symbol = icu::datetime::fields::Month::StandAlone;
    ///
    /// // First length is successful:
    /// interpolator
    ///     .include_month_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// interpolator
    ///     .include_month_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     interpolator.include_month_names(alt_field_symbol, FieldLength::Wide),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// assert!(matches!(
    ///     interpolator
    ///         .include_month_names(field_symbol, FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_month_names(
        &mut self,
        field_symbol: fields::Month,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked:
            icu_provider::DataProvider<<C as CldrCalendar>::MonthSymbolsV1Marker>,
    {
        self.load_month_names(&crate::provider::Baked, field_symbol, field_length)
    }

    /// Loads day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_day_period_names<P>(
        &mut self,
        provider: &P,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<DayPeriodSymbolsV1Marker> + ?Sized,
    {
        let field = fields::Field {
            // Names for 'a' and 'b' are stored in the same data key
            symbol: fields::FieldSymbol::DayPeriod(fields::DayPeriod::NoonMidnight),
            length: field_length,
        };
        // UTS 35 says that "a..aaa" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match self.dayperiod_symbols.check_with_length((), field_length) {
            NamePresence::Loaded => return Ok(self),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = self.locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::subtag_for(
            aux::Context::Format,
            match field_length {
                fields::FieldLength::Abbreviated => aux::Length::Abbr,
                fields::FieldLength::Narrow => aux::Length::Narrow,
                fields::FieldLength::Wide => aux::Length::Wide,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        self.dayperiod_symbols = OptionalNames::SingleLength((), field_length, payload);
        Ok(self)
    }

    /// Includes day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::DateTimeError;
    /// use icu::datetime::TypedDateTimePatternInterpolator;
    /// use icu::locid::locale;
    ///
    /// let mut interpolator =
    ///     TypedDateTimePatternInterpolator::<Gregorian>::try_new(
    ///         &locale!("und").into(),
    ///     )
    ///     .unwrap();
    ///
    /// // First length is successful:
    /// interpolator
    ///     .include_day_period_names(FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// interpolator
    ///     .include_day_period_names(FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     interpolator.include_day_period_names(FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_day_period_names(
        &mut self,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked: icu_provider::DataProvider<DayPeriodSymbolsV1Marker>,
    {
        self.load_day_period_names(&crate::provider::Baked, field_length)
    }

    /// Loads weekday names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_weekday_names<P>(
        &mut self,
        provider: &P,
        field_symbol: fields::Weekday,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<WeekdaySymbolsV1Marker> + ?Sized,
    {
        let field = fields::Field {
            symbol: fields::FieldSymbol::Weekday(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "E..EEE" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match self
            .weekday_symbols
            .check_with_length(field_symbol, field_length)
        {
            NamePresence::Loaded => return Ok(self),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = self.locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::subtag_for(
            match field_symbol {
                // UTS 35 says that "e" and "E" have the same non-numeric names
                fields::Weekday::Format | fields::Weekday::Local => aux::Context::Format,
                fields::Weekday::StandAlone => aux::Context::Standalone,
            },
            match field_length {
                fields::FieldLength::Abbreviated => aux::Length::Abbr,
                fields::FieldLength::Narrow => aux::Length::Narrow,
                fields::FieldLength::Wide => aux::Length::Wide,
                fields::FieldLength::Six => aux::Length::Short,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        self.weekday_symbols = OptionalNames::SingleLength(field_symbol, field_length, payload);
        Ok(self)
    }

    /// Includes weekday names for the specified symbol and length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::DateTimeError;
    /// use icu::datetime::TypedDateTimePatternInterpolator;
    /// use icu::locid::locale;
    ///
    /// let mut interpolator =
    ///     TypedDateTimePatternInterpolator::<Gregorian>::try_new(
    ///         &locale!("und").into(),
    ///     )
    ///     .unwrap();
    /// let field_symbol = icu::datetime::fields::Weekday::Format;
    /// let alt_field_symbol = icu::datetime::fields::Weekday::StandAlone;
    ///
    /// // First length is successful:
    /// interpolator
    ///     .include_weekday_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// interpolator
    ///     .include_weekday_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     interpolator.include_weekday_names(alt_field_symbol, FieldLength::Wide),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// assert!(matches!(
    ///     interpolator
    ///         .include_weekday_names(field_symbol, FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_weekday_names(
        &mut self,
        field_symbol: fields::Weekday,
        field_length: fields::FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked: icu_provider::DataProvider<WeekdaySymbolsV1Marker>,
    {
        self.load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
    }
}

#[derive(Debug, Copy, Clone)]
struct RawDateTimePatternInterpolatorBorrowed<'l> {
    /// `year_symbols` is different because it could be either era or cyclic year.
    year_symbols: OptionalNames<(), &'l YearSymbolsV1<'l>>,
    month_symbols: OptionalNames<fields::Month, &'l MonthSymbolsV1<'l>>,
    weekday_symbols: OptionalNames<fields::Weekday, &'l LinearSymbolsV1<'l>>,
    dayperiod_symbols: OptionalNames<(), &'l LinearSymbolsV1<'l>>,
    fixed_decimal_formatter: &'l FixedDecimalFormatter,
    week_calculator: Option<&'l WeekCalculator>,
}

impl<C: CldrCalendar> TypedDateTimePatternInterpolator<C> {
    fn as_borrowed(&self) -> RawDateTimePatternInterpolatorBorrowed {
        RawDateTimePatternInterpolatorBorrowed {
            year_symbols: self.year_symbols.as_borrowed(),
            month_symbols: self.month_symbols.as_borrowed(),
            weekday_symbols: self.weekday_symbols.as_borrowed(),
            dayperiod_symbols: self.dayperiod_symbols.as_borrowed(),
            fixed_decimal_formatter: &self.fixed_decimal_formatter,
            week_calculator: self.week_calculator.as_ref(),
        }
    }

    /// Formats a date and time of day.
    ///
    /// For an example, see [`TypedDateTimePatternInterpolator`].
    pub fn format<'l, T>(
        &'l self,
        pattern: &'l Pattern<'l>,
        datetime: &T,
    ) -> FormattedDateTimePattern<'l>
    where
        T: DateTimeInput<Calendar = C>,
    {
        FormattedDateTimePattern {
            pattern,
            datetime: ExtractedDateTimeInput::extract_from(datetime),
            interpolator: self.as_borrowed(),
        }
    }

    /// Formats a date without a time of day.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::pattern;
    /// use icu::datetime::TypedDateTimePatternInterpolator;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create an interpolator that can format wide month and era names:
    /// let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
    ///     TypedDateTimePatternInterpolator::try_new(&locale!("en-GB").into())
    ///         .unwrap();
    /// interpolator
    ///     .include_month_names(fields::Month::Format, FieldLength::Wide)
    ///     .unwrap()
    ///     .include_year_names(FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The date is:' MMMM d, y GGGG";
    /// let reference_pattern: pattern::reference::Pattern =
    ///     pattern_str.parse().unwrap();
    /// let pattern: pattern::runtime::Pattern = (&reference_pattern).into();
    ///
    /// // Test it with some different dates:
    /// // Note: extended year -50 is year 51 BCE
    /// let date_bce = Date::try_new_gregorian_date(-50, 3, 15).unwrap();
    /// let date_ce = Date::try_new_gregorian_date(1700, 11, 20).unwrap();
    /// assert_writeable_eq!(
    ///     interpolator.format_date(&pattern, &date_bce),
    ///     "The date is: March 15, 51 Before Christ"
    /// );
    /// assert_writeable_eq!(
    ///     interpolator.format_date(&pattern, &date_ce),
    ///     "The date is: November 20, 1700 Anno Domini"
    /// );
    /// ```
    pub fn format_date<'l, T>(
        &'l self,
        pattern: &'l Pattern<'l>,
        datetime: &T,
    ) -> FormattedDateTimePattern<'l>
    where
        T: DateInput<Calendar = C>,
    {
        FormattedDateTimePattern {
            pattern,
            datetime: ExtractedDateTimeInput::extract_from_date(datetime),
            interpolator: self.as_borrowed(),
        }
    }

    /// Formats a time of day without a date.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::types::Time;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::fields::FieldLength;
    /// use icu::datetime::pattern;
    /// use icu::datetime::TypedDateTimePatternInterpolator;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create an interpolator that can format abbreviated day periods:
    /// let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
    ///     TypedDateTimePatternInterpolator::try_new(&locale!("en-US").into())
    ///         .unwrap();
    /// interpolator
    ///     .include_day_period_names(FieldLength::Abbreviated)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The time is:' h:mm b";
    /// let reference_pattern: pattern::reference::Pattern =
    ///     pattern_str.parse().unwrap();
    /// let pattern: pattern::runtime::Pattern = (&reference_pattern).into();
    ///
    /// // Test it with different times of day:
    /// let time_am = Time::try_new(11, 4, 14, 0).unwrap();
    /// let time_pm = Time::try_new(13, 41, 28, 0).unwrap();
    /// let time_noon = Time::try_new(12, 0, 0, 0).unwrap();
    /// let time_midnight = Time::try_new(0, 0, 0, 0).unwrap();
    /// assert_writeable_eq!(
    ///     interpolator.format_time(&pattern, &time_am),
    ///     "The time is: 11:04 AM"
    /// );
    /// assert_writeable_eq!(
    ///     interpolator.format_time(&pattern, &time_pm),
    ///     "The time is: 1:41 PM"
    /// );
    /// assert_writeable_eq!(
    ///     interpolator.format_time(&pattern, &time_noon),
    ///     "The time is: 12:00 noon"
    /// );
    /// assert_writeable_eq!(
    ///     interpolator.format_time(&pattern, &time_midnight),
    ///     "The time is: 12:00 midnight"
    /// );
    /// ```
    pub fn format_time<'l, T>(
        &'l self,
        pattern: &'l Pattern<'l>,
        datetime: &T,
    ) -> FormattedDateTimePattern<'l>
    where
        T: IsoTimeInput,
    {
        FormattedDateTimePattern {
            pattern,
            datetime: ExtractedDateTimeInput::extract_from_time(datetime),
            interpolator: self.as_borrowed(),
        }
    }
}

/// A pattern that has been interpolated and implements [`Writeable`].
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/1317">#1317</a>
/// </div>
#[derive(Debug)]
pub struct FormattedDateTimePattern<'l> {
    pattern: &'l Pattern<'l>,
    datetime: ExtractedDateTimeInput,
    interpolator: RawDateTimePatternInterpolatorBorrowed<'l>,
}

impl<'l> Writeable for FormattedDateTimePattern<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let loc_datetime =
            DateTimeInputWithWeekConfig::new(&self.datetime, self.interpolator.week_calculator);
        write_pattern(
            self.pattern,
            Some(&self.interpolator),
            Some(&self.interpolator),
            &loc_datetime,
            self.interpolator.fixed_decimal_formatter,
            sink,
        )
        .map_err(|_e| {
            icu_provider::_internal::log::warn!("{_e:?}");
            core::fmt::Error
        })
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l> fmt::Display for FormattedDateTimePattern<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<'data> DateSymbols<'data> for RawDateTimePatternInterpolatorBorrowed<'data> {
    fn get_symbol_for_month(
        &self,
        field_symbol: fields::Month,
        field_length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, Error> {
        let field = fields::Field {
            symbol: fields::FieldSymbol::Month(field_symbol),
            length: field_length,
        };
        let month_symbols = self
            .month_symbols
            .get_with_length(field_symbol, field_length)
            .ok_or_else(|| Error::MissingNames(field))?;
        let Some((month_number, is_leap)) = code.parsed() else {
            return Err(Error::MissingMonthSymbol(code));
        };
        let Some(month_index) = month_number.checked_sub(1) else {
            return Err(Error::MissingMonthSymbol(code));
        };
        let month_index = usize::from(month_index);
        let symbol = match month_symbols {
            MonthSymbolsV1::Linear(linear) => {
                if is_leap {
                    None
                } else {
                    linear.get(month_index)
                }
            }
            MonthSymbolsV1::LeapLinear(leap_linear) => {
                let num_months = leap_linear.len() / 2;
                if is_leap {
                    leap_linear.get(month_index + num_months)
                } else if month_index < num_months {
                    leap_linear.get(month_index)
                } else {
                    None
                }
            }
            MonthSymbolsV1::LeapNumeric(leap_numeric) => {
                if is_leap {
                    return Ok(MonthPlaceholderValue::NumericPattern(leap_numeric));
                } else {
                    return Ok(MonthPlaceholderValue::Numeric);
                }
            }
        };
        // Note: Always return `false` for the second argument since neo MonthSymbols
        // knows how to handle leap months and we don't need the fallback logic
        symbol
            .map(|s| MonthPlaceholderValue::PlainString(s))
            .ok_or_else(|| Error::MissingMonthSymbol(code))
    }

    fn get_symbol_for_weekday(
        &self,
        field_symbol: fields::Weekday,
        field_length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, Error> {
        let field = fields::Field {
            symbol: fields::FieldSymbol::Weekday(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "e" and "E" have the same non-numeric names
        let field_symbol = field_symbol.to_format_symbol();
        // UTS 35 says that "E..EEE" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let weekday_symbols = self
            .weekday_symbols
            .get_with_length(field_symbol, field_length)
            .ok_or_else(|| Error::MissingNames(field))?;
        let day_usize = day as usize;
        weekday_symbols
            .symbols
            .get(day_usize)
            .ok_or_else(|| Error::MissingWeekdaySymbol(day_usize))
    }

    fn get_symbol_for_era<'a>(
        &'a self,
        field_length: fields::FieldLength,
        era_code: &'a Era,
    ) -> Result<Option<&str>, Error> {
        let field = fields::Field {
            symbol: fields::FieldSymbol::Era,
            length: field_length,
        };
        // UTS 35 says that "G..GGG" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let year_symbols = self
            .year_symbols
            .get_with_length((), field_length)
            .ok_or_else(|| Error::MissingNames(field))?;
        let YearSymbolsV1::Eras(era_symbols) = year_symbols else {
            return Err(Error::MissingNames(field));
        };
        Ok(era_symbols.get(era_code.0.as_str().into()))
    }
}

impl<'data> TimeSymbols for RawDateTimePatternInterpolatorBorrowed<'data> {
    fn get_symbol_for_day_period(
        &self,
        field_symbol: fields::DayPeriod,
        field_length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, Error> {
        use fields::DayPeriod::NoonMidnight;
        let field = fields::Field {
            symbol: fields::FieldSymbol::DayPeriod(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "a..aaa" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let dayperiod_symbols = self
            .dayperiod_symbols
            .get_with_length((), field_length)
            .ok_or_else(|| Error::MissingNames(field))?;
        let option_value: Option<&str> = match (field_symbol, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => dayperiod_symbols
                .midnight()
                .or_else(|| dayperiod_symbols.am()),
            (NoonMidnight, 12, true) => dayperiod_symbols.noon().or_else(|| dayperiod_symbols.pm()),
            (_, hour, _) if hour < 12 => dayperiod_symbols.am(),
            _ => dayperiod_symbols.pm(),
        };
        option_value.ok_or_else(|| Error::MissingNames(field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::reference;
    use icu_calendar::{DateTime, Gregorian};
    use icu_locid::locale;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic_pattern_interpolator() {
        let locale = locale!("en").into();
        let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
            TypedDateTimePatternInterpolator::try_new(&locale).unwrap();
        interpolator
            .load_month_names(
                &crate::provider::Baked,
                fields::Month::Format,
                fields::FieldLength::Abbreviated,
            )
            .unwrap()
            .load_weekday_names(
                &crate::provider::Baked,
                fields::Weekday::Format,
                fields::FieldLength::Wide,
            )
            .unwrap()
            .load_year_names(&crate::provider::Baked, fields::FieldLength::Narrow)
            .unwrap()
            .load_day_period_names(&crate::provider::Baked, fields::FieldLength::Abbreviated)
            .unwrap();
        let reference_pattern: reference::Pattern = "'It is' EEEE, MMM d, y GGGGG 'at' hh:mm a'!'"
            .parse()
            .unwrap();
        let pattern: Pattern = (&reference_pattern).into();
        let datetime = DateTime::try_new_gregorian_datetime(2023, 10, 25, 15, 0, 55).unwrap();
        let formatted_pattern = interpolator.format(&pattern, &datetime);

        assert_writeable_eq!(
            formatted_pattern,
            "It is Wednesday, Oct 25, 2023 A at 03:00 PM!"
        );
    }

    #[test]
    fn test_era_coverage() {
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_length: fields::FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<G>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GG>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGG>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGGG>",
                field_length: fields::FieldLength::Wide,
                expected: "<нашої ери>",
            },
            TestCase {
                pattern: "<GGGGG>",
                field_length: fields::FieldLength::Narrow,
                expected: "<н.е.>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
                TypedDateTimePatternInterpolator::try_new(&locale).unwrap();
            interpolator
                .load_year_names(&crate::provider::Baked, field_length)
                .unwrap();
            let reference_pattern: reference::Pattern = pattern.parse().unwrap();
            let pattern: Pattern = (&reference_pattern).into();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = interpolator.format(&pattern, &datetime);

            assert_writeable_eq!(formatted_pattern, expected, "{cas:?}");
        }
    }

    #[test]
    fn test_month_coverage() {
        // Ukrainian has different values for format and standalone
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_symbol: fields::Month,
            field_length: fields::FieldLength,
            expected: &'static str,
        }
        let cases = [
            // 'M' and 'MM' are numeric
            TestCase {
                pattern: "<MMM>",
                field_symbol: fields::Month::Format,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<MMMM>",
                field_symbol: fields::Month::Format,
                field_length: fields::FieldLength::Wide,
                expected: "<листопада>",
            },
            TestCase {
                pattern: "<MMMMM>",
                field_symbol: fields::Month::Format,
                field_length: fields::FieldLength::Narrow,
                expected: "<л>",
            },
            // 'L' and 'LL' are numeric
            TestCase {
                pattern: "<LLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<LLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: fields::FieldLength::Wide,
                expected: "<листопад>",
            },
            TestCase {
                pattern: "<LLLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: fields::FieldLength::Narrow,
                expected: "<Л>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_symbol,
                field_length,
                expected,
            } = cas;
            let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
                TypedDateTimePatternInterpolator::try_new(&locale).unwrap();
            interpolator
                .load_month_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let reference_pattern: reference::Pattern = pattern.parse().unwrap();
            let pattern: Pattern = (&reference_pattern).into();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = interpolator.format(&pattern, &datetime);

            assert_writeable_eq!(formatted_pattern, expected, "{cas:?}");
        }
    }

    #[test]
    fn test_weekday_coverage() {
        let locale = locale!("uk").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_symbol: fields::Weekday,
            field_length: fields::FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<E>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EE>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEE>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Wide,
                expected: "<пʼятницю>",
            },
            TestCase {
                pattern: "<EEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<EEEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Six,
                expected: "<пт>",
            },
            // 'e' and 'ee' are numeric
            TestCase {
                pattern: "<eee>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<eeee>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Wide,
                expected: "<пʼятницю>",
            },
            TestCase {
                pattern: "<eeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<eeeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: fields::FieldLength::Six,
                expected: "<пт>",
            },
            // 'c' and 'cc' are numeric
            TestCase {
                pattern: "<ccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: fields::FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<cccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: fields::FieldLength::Wide,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<ccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: fields::FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<cccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: fields::FieldLength::Six,
                expected: "<пт>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_symbol,
                field_length,
                expected,
            } = cas;
            let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
                TypedDateTimePatternInterpolator::try_new(&locale).unwrap();
            interpolator
                .load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let reference_pattern: reference::Pattern = pattern.parse().unwrap();
            let pattern: Pattern = (&reference_pattern).into();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = interpolator.format(&pattern, &datetime);

            assert_writeable_eq!(formatted_pattern, expected, "{cas:?}");
        }
    }

    #[test]
    fn test_dayperiod_coverage() {
        // Thai has different values for different lengths of day periods
        // TODO(#487): Support flexible day periods, too
        let locale = locale!("th").into();
        #[derive(Debug)]
        struct TestCase {
            pattern: &'static str,
            field_length: fields::FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<a>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aa>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaa>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaaa>",
                field_length: fields::FieldLength::Wide,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<aaaaa>",
                field_length: fields::FieldLength::Narrow,
                expected: "<p>",
            },
            TestCase {
                pattern: "<b>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bb>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbb>",
                field_length: fields::FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbbb>",
                field_length: fields::FieldLength::Wide,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<bbbbb>",
                field_length: fields::FieldLength::Narrow,
                expected: "<p>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut interpolator: TypedDateTimePatternInterpolator<Gregorian> =
                TypedDateTimePatternInterpolator::try_new(&locale).unwrap();
            interpolator
                .load_day_period_names(&crate::provider::Baked, field_length)
                .unwrap();
            let reference_pattern: reference::Pattern = pattern.parse().unwrap();
            let pattern: Pattern = (&reference_pattern).into();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = interpolator.format(&pattern, &datetime);

            assert_writeable_eq!(formatted_pattern, expected, "{cas:?}");
        }
    }
}
