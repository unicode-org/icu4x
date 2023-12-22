// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::datetime::write_pattern;
use crate::calendar::CldrCalendar;
use crate::error::DateTimeError as Error;
use crate::fields::{self, FieldLength, FieldSymbol};
use crate::input;
use crate::input::DateInput;
use crate::input::DateTimeInput;
use crate::input::DateTimeInputWithWeekConfig;
use crate::input::ExtractedDateTimeInput;
use crate::input::IsoTimeInput;
use crate::neo_pattern::{DateTimePattern, DateTimePatternBorrowed};
use crate::pattern::PatternItem;
use crate::provider::date_time::{DateSymbols, MonthPlaceholderValue, TimeSymbols};
use crate::provider::neo::*;
use core::fmt;
use core::marker::PhantomData;
use icu_calendar::provider::WeekDataV1Marker;
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
    SingleLength(S, FieldLength, T),
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
        field_length: FieldLength,
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
    pub(crate) fn get_with_length(&self, field_symbol: S, field_length: FieldLength) -> Option<T> {
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
    M: DataMarker,
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

/// Helper for type resolution with optional DataProvider arguments
pub(crate) struct PhantomProvider {
    _not_constructible: core::convert::Infallible,
}

impl<M: KeyedDataMarker> DataProvider<M> for PhantomProvider {
    #[inline]
    fn load(&self, _req: DataRequest) -> Result<DataResponse<M>, DataError> {
        unreachable!() // not constructible
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
/// use icu::datetime::TypedDateTimeNames;
/// use icu::datetime::fields::FieldLength;
/// use icu::datetime::fields;
/// use icu::datetime::neo_pattern::DateTimePattern;
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// // Create an instance that can format abbreviated month, weekday, and day period names:
/// let mut names: TypedDateTimeNames<Gregorian> =
///     TypedDateTimeNames::try_new(&locale!("uk").into()).unwrap();
/// names
///     .include_month_names(fields::Month::Format, FieldLength::Abbreviated)
///     .unwrap()
///     .include_weekday_names(fields::Weekday::Format, FieldLength::Abbreviated)
///     .unwrap()
///     .include_day_period_names(FieldLength::Abbreviated)
///     .unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "E MMM d y -- h:mm a";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // Test it:
/// let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 20, 11, 35, 3).unwrap();
/// assert_writeable_eq!(names.with_pattern(&pattern).format(&datetime), "пн лист. 20 2023 -- 11:35 дп");
/// ```
///
/// If the correct data is not loaded, and error will occur:
///
/// ```
/// use icu::calendar::Gregorian;
/// use icu::calendar::DateTime;
/// use icu::datetime::TypedDateTimeNames;
/// use icu::datetime::fields::FieldLength;
/// use icu::datetime::fields;
/// use icu::datetime::neo_pattern::DateTimePattern;
/// use icu::locid::locale;
/// use writeable::Writeable;
///
/// // Create an instance that can format abbreviated month, weekday, and day period names:
/// let mut names: TypedDateTimeNames<Gregorian> =
///     TypedDateTimeNames::try_new(&locale!("en").into()).unwrap();
///
/// // Create a pattern from a pattern string:
/// let pattern_str = "'It is:' E MMM d y 'at' h:mm a";
/// let pattern: DateTimePattern = pattern_str.parse().unwrap();
///
/// // The pattern string contains lots of symbols including "E", "MMM", and "a", but we did not load any data!
/// let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 20, 11, 35, 3).unwrap();
/// let mut buffer = String::new();
/// assert!(names.with_pattern(&pattern).format(&datetime).write_to(&mut buffer).is_err());
/// ```
#[derive(Debug)]
pub struct TypedDateTimeNames<C: CldrCalendar> {
    locale: DataLocale,
    inner: RawDateTimeNames,
    _calendar: PhantomData<C>,
}

#[derive(Debug)]
pub(crate) struct RawDateTimeNames {
    year_symbols: OptionalNames<(), DataPayload<ErasedYearNamesV1Marker>>,
    month_symbols: OptionalNames<fields::Month, DataPayload<ErasedMonthNamesV1Marker>>,
    weekday_symbols: OptionalNames<fields::Weekday, DataPayload<WeekdayNamesV1Marker>>,
    dayperiod_symbols: OptionalNames<(), DataPayload<DayPeriodNamesV1Marker>>,
    // TODO(#4340): Make the FixedDecimalFormatter optional
    fixed_decimal_formatter: Option<FixedDecimalFormatter>,
    week_calculator: Option<WeekCalculator>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawDateTimeNamesBorrowed<'l> {
    year_names: OptionalNames<(), &'l YearNamesV1<'l>>,
    month_names: OptionalNames<fields::Month, &'l MonthNamesV1<'l>>,
    weekday_names: OptionalNames<fields::Weekday, &'l LinearNamesV1<'l>>,
    dayperiod_names: OptionalNames<(), &'l LinearNamesV1<'l>>,
    pub(crate) fixed_decimal_formatter: Option<&'l FixedDecimalFormatter>,
    pub(crate) week_calculator: Option<&'l WeekCalculator>,
}

impl<C: CldrCalendar> TypedDateTimeNames<C> {
    /// Constructor that takes a selected locale and creates an empty instance.
    ///
    /// For an example, see [`TypedDateTimeNames`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale) -> Result<Self, Error> {
        let mut names = Self {
            locale: locale.clone(),
            inner: RawDateTimeNames::new_without_fixed_decimal_formatter(),
            _calendar: PhantomData,
        };
        names.include_fixed_decimal_formatter()?;
        Ok(names)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(provider: &P, locale: &DataLocale) -> Result<Self, Error>
    where
        P: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        let mut names = Self {
            locale: locale.clone(),
            inner: RawDateTimeNames::new_without_fixed_decimal_formatter(),
            _calendar: PhantomData,
        };
        names.load_fixed_decimal_formatter(provider)?;
        Ok(names)
    }

    /// Loads year (era or cycle) names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_year_names<P>(
        &mut self,
        provider: &P,
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<C::YearNamesV1Marker> + ?Sized,
    {
        self.inner
            .load_year_names(provider, &self.locale, field_length)?;
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
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names.include_year_names(FieldLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names.include_year_names(FieldLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     names.include_year_names(FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_year_names(&mut self, field_length: FieldLength) -> Result<&mut Self, Error>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::YearNamesV1Marker>,
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
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<C::MonthNamesV1Marker> + ?Sized,
    {
        self.inner
            .load_month_names(provider, &self.locale, field_symbol, field_length)?;
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
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    /// let field_symbol = icu::datetime::fields::Month::Format;
    /// let alt_field_symbol = icu::datetime::fields::Month::StandAlone;
    ///
    /// // First length is successful:
    /// names
    ///     .include_month_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names
    ///     .include_month_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     names.include_month_names(alt_field_symbol, FieldLength::Wide),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// assert!(matches!(
    ///     names.include_month_names(field_symbol, FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_month_names(
        &mut self,
        field_symbol: fields::Month,
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked: icu_provider::DataProvider<<C as CldrCalendar>::MonthNamesV1Marker>,
    {
        self.load_month_names(&crate::provider::Baked, field_symbol, field_length)
    }

    /// Loads day period names for the specified length.
    ///
    /// Does not support multiple field symbols or lengths. See #4337
    pub fn load_day_period_names<P>(
        &mut self,
        provider: &P,
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<DayPeriodNamesV1Marker> + ?Sized,
    {
        self.inner
            .load_day_period_names(provider, &self.locale, field_length)?;
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
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    ///
    /// // First length is successful:
    /// names.include_day_period_names(FieldLength::Wide).unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names.include_day_period_names(FieldLength::Wide).unwrap();
    ///
    /// // But loading a new length fails:
    /// assert!(matches!(
    ///     names.include_day_period_names(FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_day_period_names(
        &mut self,
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked: icu_provider::DataProvider<DayPeriodNamesV1Marker>,
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
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        P: DataProvider<WeekdayNamesV1Marker> + ?Sized,
    {
        self.inner
            .load_weekday_names(provider, &self.locale, field_symbol, field_length)?;
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
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("und").into())
    ///         .unwrap();
    /// let field_symbol = icu::datetime::fields::Weekday::Format;
    /// let alt_field_symbol = icu::datetime::fields::Weekday::StandAlone;
    ///
    /// // First length is successful:
    /// names
    ///     .include_weekday_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Attempting to load the first length a second time will succeed:
    /// names
    ///     .include_weekday_names(field_symbol, FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // But loading a new symbol or length fails:
    /// assert!(matches!(
    ///     names.include_weekday_names(alt_field_symbol, FieldLength::Wide),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// assert!(matches!(
    ///     names.include_weekday_names(field_symbol, FieldLength::Abbreviated),
    ///     Err(DateTimeError::DuplicateField(_))
    /// ));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_weekday_names(
        &mut self,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
    ) -> Result<&mut Self, Error>
    where
        crate::provider::Baked: icu_provider::DataProvider<WeekdayNamesV1Marker>,
    {
        self.load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
    }

    /// Sets the week calculator to use with patterns requiring week numbering.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::week::WeekCalculator;
    /// use icu::calendar::Date;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("en").into())
    ///         .unwrap();
    ///
    /// // Load the week calculator and set it here:
    /// let mut week_calculator =
    ///     WeekCalculator::try_new(&locale!("en").into()).unwrap();
    /// names.set_week_calculator(week_calculator);
    ///
    /// // Format a pattern needing week data:
    /// let pattern_str = "'Week' w 'of' Y";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    /// let date = Date::try_new_gregorian_date(2023, 12, 5).unwrap();
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date),
    ///     "Week 49 of 2023"
    /// );
    /// ```
    #[inline]
    pub fn set_week_calculator(&mut self, week_calculator: WeekCalculator) -> &mut Self {
        self.inner.set_week_calculator(week_calculator);
        self
    }

    // TODO(#4340): Make this fn public when FixedDecimalFormatter is fully optional
    #[inline]
    fn load_fixed_decimal_formatter<P>(&mut self, provider: &P) -> Result<&mut Self, Error>
    where
        P: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        self.inner.load_fixed_decimal_formatter(|options| {
            FixedDecimalFormatter::try_new_unstable(provider, &self.locale, options)
        })?;
        Ok(self)
    }

    // TODO(#4340): Make this fn public when FixedDecimalFormatter is fully optional
    #[cfg(feature = "compiled_data")]
    #[inline]
    fn include_fixed_decimal_formatter(&mut self) -> Result<&mut Self, Error> {
        self.inner.load_fixed_decimal_formatter(|options| {
            FixedDecimalFormatter::try_new(&self.locale, options)
        })?;
        Ok(self)
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// without loading additional data for that pattern.
    #[inline]
    pub fn with_pattern<'l>(
        &'l self,
        pattern: &'l DateTimePattern,
    ) -> DateTimePatternFormatter<'l, C> {
        DateTimePatternFormatter {
            inner: self.inner.with_pattern(pattern.as_borrowed()),
            _calendar: PhantomData,
        }
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// and loads all data required for that pattern.
    ///
    /// Does not duplicate textual field symbols. See #4337
    pub fn load_for_pattern<'l, P>(
        &'l mut self,
        provider: &P,
        pattern: &'l DateTimePattern,
    ) -> Result<DateTimePatternFormatter<'l, C>, Error>
    where
        P: DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let locale = &self.locale;
        self.inner
            .load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
                Some(provider),
                Some(provider),
                Some(provider),
                Some(provider),
                locale,
                pattern.iter_items(),
                |options| FixedDecimalFormatter::try_new_unstable(provider, locale, options),
                || WeekCalculator::try_new_unstable(provider, locale),
            )?;
        Ok(DateTimePatternFormatter {
            inner: self.inner.with_pattern(pattern.as_borrowed()),
            _calendar: PhantomData,
        })
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// and includes all data required for that pattern.
    ///
    /// Does not support duplicate textual field symbols. See #4337
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut names =
    ///     TypedDateTimeNames::<Gregorian>::try_new(&locale!("en").into())
    ///         .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "EEEE 'on week' w 'of' Y G (MMM d) 'at' h:mm a";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Load data for the pattern and format:
    /// let datetime =
    ///     DateTime::try_new_gregorian_datetime(2023, 12, 5, 17, 43, 12).unwrap();
    /// assert_writeable_eq!(
    ///     names
    ///         .include_for_pattern(&pattern)
    ///         .unwrap()
    ///         .format(&datetime),
    ///     "Tuesday on week 49 of 2023 AD (Dec 5) at 5:43 PM"
    /// );
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn include_for_pattern<'l>(
        &'l mut self,
        pattern: &'l DateTimePattern,
    ) -> Result<DateTimePatternFormatter<'l, C>, Error>
    where
        crate::provider::Baked: DataProvider<C::YearNamesV1Marker>
            + DataProvider<C::MonthNamesV1Marker>
            + DataProvider<WeekdayNamesV1Marker>
            + DataProvider<DayPeriodNamesV1Marker>,
    {
        let locale = &self.locale;
        self.inner
            .load_for_pattern::<C::YearNamesV1Marker, C::MonthNamesV1Marker>(
                Some(&crate::provider::Baked),
                Some(&crate::provider::Baked),
                Some(&crate::provider::Baked),
                Some(&crate::provider::Baked),
                locale,
                pattern.iter_items(),
                |options| FixedDecimalFormatter::try_new(locale, options),
                || WeekCalculator::try_new(locale),
            )?;
        Ok(DateTimePatternFormatter {
            inner: self.inner.with_pattern(pattern.as_borrowed()),
            _calendar: PhantomData,
        })
    }
}

impl RawDateTimeNames {
    pub(crate) fn new_without_fixed_decimal_formatter() -> Self {
        Self {
            year_symbols: OptionalNames::None,
            month_symbols: OptionalNames::None,
            weekday_symbols: OptionalNames::None,
            dayperiod_symbols: OptionalNames::None,
            fixed_decimal_formatter: None,
            week_calculator: None,
        }
    }

    pub(crate) fn as_borrowed(&self) -> RawDateTimeNamesBorrowed {
        RawDateTimeNamesBorrowed {
            year_names: self.year_symbols.as_borrowed(),
            month_names: self.month_symbols.as_borrowed(),
            weekday_names: self.weekday_symbols.as_borrowed(),
            dayperiod_names: self.dayperiod_symbols.as_borrowed(),
            fixed_decimal_formatter: self.fixed_decimal_formatter.as_ref(),
            week_calculator: self.week_calculator.as_ref(),
        }
    }

    pub(crate) fn load_year_names<P, M>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_length: FieldLength,
    ) -> Result<(), Error>
    where
        P: DataProvider<M> + ?Sized,
        M: KeyedDataMarker<Yokeable = YearNamesV1<'static>>,
    {
        let field = fields::Field {
            symbol: FieldSymbol::Era,
            length: field_length,
        };
        // UTS 35 says that "G..GGG" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match self.year_symbols.check_with_length((), field_length) {
            NamePresence::Loaded => return Ok(()),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::symbol_subtag_for(
            aux::Context::Format,
            match field_length {
                FieldLength::Abbreviated => aux::Length::Abbr,
                FieldLength::Narrow => aux::Length::Narrow,
                FieldLength::Wide => aux::Length::Wide,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?
            .cast();
        self.year_symbols = OptionalNames::SingleLength((), field_length, payload);
        Ok(())
    }

    pub(crate) fn load_month_names<P, M>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_symbol: fields::Month,
        field_length: FieldLength,
    ) -> Result<(), Error>
    where
        P: DataProvider<M> + ?Sized,
        M: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>,
    {
        let field = fields::Field {
            symbol: FieldSymbol::Month(field_symbol),
            length: field_length,
        };
        // Note: UTS 35 says that "M..MM" and "L..LL" are numeric
        match self
            .month_symbols
            .check_with_length(field_symbol, field_length)
        {
            NamePresence::Loaded => return Ok(()),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::symbol_subtag_for(
            match field_symbol {
                fields::Month::Format => aux::Context::Format,
                fields::Month::StandAlone => aux::Context::Standalone,
            },
            match field_length {
                FieldLength::Abbreviated => aux::Length::Abbr,
                FieldLength::Narrow => aux::Length::Narrow,
                FieldLength::Wide => aux::Length::Wide,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?
            .cast();
        self.month_symbols = OptionalNames::SingleLength(field_symbol, field_length, payload);
        Ok(())
    }

    pub(crate) fn load_day_period_names<P, M>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_length: FieldLength,
    ) -> Result<(), Error>
    where
        P: DataProvider<M> + ?Sized,
        M: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>,
    {
        let field = fields::Field {
            // Names for 'a' and 'b' are stored in the same data key
            symbol: FieldSymbol::DayPeriod(fields::DayPeriod::NoonMidnight),
            length: field_length,
        };
        // UTS 35 says that "a..aaa" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        match self.dayperiod_symbols.check_with_length((), field_length) {
            NamePresence::Loaded => return Ok(()),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::symbol_subtag_for(
            aux::Context::Format,
            match field_length {
                FieldLength::Abbreviated => aux::Length::Abbr,
                FieldLength::Narrow => aux::Length::Narrow,
                FieldLength::Wide => aux::Length::Wide,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?
            .cast();
        self.dayperiod_symbols = OptionalNames::SingleLength((), field_length, payload);
        Ok(())
    }

    pub(crate) fn load_weekday_names<P, M>(
        &mut self,
        provider: &P,
        locale: &DataLocale,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
    ) -> Result<(), Error>
    where
        P: DataProvider<M> + ?Sized,
        M: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>,
    {
        let field = fields::Field {
            symbol: FieldSymbol::Weekday(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "E..EEE" are all Abbreviated
        // However, this doesn't apply to "e" and "c".
        let field_length = if matches!(field_symbol, fields::Weekday::Format) {
            field_length.numeric_to_abbr()
        } else {
            field_length
        };
        match self
            .weekday_symbols
            .check_with_length(field_symbol, field_length)
        {
            NamePresence::Loaded => return Ok(()),
            NamePresence::NotLoaded => (),
            NamePresence::Mismatched => return Err(Error::DuplicateField(field)),
        };
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::symbol_subtag_for(
            match field_symbol {
                // UTS 35 says that "e" and "E" have the same non-numeric names
                fields::Weekday::Format | fields::Weekday::Local => aux::Context::Format,
                fields::Weekday::StandAlone => aux::Context::Standalone,
            },
            match field_length {
                FieldLength::Abbreviated => aux::Length::Abbr,
                FieldLength::Narrow => aux::Length::Narrow,
                FieldLength::Wide => aux::Length::Wide,
                FieldLength::Six => aux::Length::Short,
                _ => return Err(Error::UnsupportedFormattingField(field)),
            },
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?
            .cast();
        self.weekday_symbols = OptionalNames::SingleLength(field_symbol, field_length, payload);
        Ok(())
    }

    #[inline]
    pub(crate) fn set_week_calculator(&mut self, week_calculator: WeekCalculator) {
        self.week_calculator = Some(week_calculator);
    }

    pub(crate) fn load_fixed_decimal_formatter(
        &mut self,
        loader: impl FnOnce(
            FixedDecimalFormatterOptions,
        ) -> Result<FixedDecimalFormatter, icu_decimal::DecimalError>,
    ) -> Result<(), Error> {
        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        self.fixed_decimal_formatter = Some(loader(fixed_decimal_format_options)?);
        Ok(())
    }

    /// Associates this [`TypedDateTimeNames`] with a pattern
    /// without loading additional data for that pattern.
    #[inline]
    pub(crate) fn with_pattern<'l>(
        &'l self,
        pattern: DateTimePatternBorrowed<'l>,
    ) -> RawDateTimePatternFormatter<'l> {
        RawDateTimePatternFormatter {
            pattern,
            names: self.as_borrowed(),
        }
    }

    pub(crate) fn load_for_pattern<'l, YearMarker, MonthMarker>(
        &'l mut self,
        year_provider: Option<&(impl DataProvider<YearMarker> + ?Sized)>,
        month_provider: Option<&(impl DataProvider<MonthMarker> + ?Sized)>,
        weekday_provider: Option<&(impl DataProvider<WeekdayNamesV1Marker> + ?Sized)>,
        dayperiod_provider: Option<&(impl DataProvider<DayPeriodNamesV1Marker> + ?Sized)>,
        locale: &DataLocale,
        pattern_items: impl Iterator<Item = PatternItem>,
        fixed_decimal_formatter_loader: impl FnOnce(
            FixedDecimalFormatterOptions,
        ) -> Result<
            FixedDecimalFormatter,
            icu_decimal::DecimalError,
        >,
        week_calculator_loader: impl FnOnce() -> Result<WeekCalculator, icu_calendar::CalendarError>,
    ) -> Result<(), Error>
    where
        YearMarker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>,
        MonthMarker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>,
    {
        let fields = pattern_items.filter_map(|p| match p {
            PatternItem::Field(field) => Some(field),
            _ => None,
        });

        let mut has_numeric = false;
        let mut has_weeks = false;
        for field in fields {
            match field.symbol {
                ///// Textual symbols /////
                FieldSymbol::Era => {
                    self.load_year_names(
                        year_provider.ok_or(Error::MissingNames(field))?,
                        locale,
                        field.length,
                    )?;
                }
                FieldSymbol::Month(symbol) => match field.length {
                    FieldLength::One => has_numeric = true,
                    FieldLength::TwoDigit => has_numeric = true,
                    _ => {
                        self.load_month_names(
                            month_provider.ok_or(Error::MissingNames(field))?,
                            locale,
                            symbol,
                            field.length,
                        )?;
                    }
                },
                // 'E' is always text
                // 'e' and 'c' are either numeric or text
                FieldSymbol::Weekday(symbol) => match field.length {
                    FieldLength::One | FieldLength::TwoDigit
                        if !matches!(symbol, fields::Weekday::Format) =>
                    {
                        has_numeric = true
                    }
                    _ => {
                        self.load_weekday_names(
                            weekday_provider.ok_or(Error::MissingNames(field))?,
                            locale,
                            symbol,
                            field.length,
                        )?;
                    }
                },
                FieldSymbol::DayPeriod(_) => {
                    self.load_day_period_names(
                        dayperiod_provider.ok_or(Error::MissingNames(field))?,
                        locale,
                        field.length,
                    )?;
                }

                ///// Numeric symbols /////
                FieldSymbol::Year(fields::Year::WeekOf) => has_weeks = true,
                FieldSymbol::Year(_) => has_numeric = true,
                FieldSymbol::Week(_) => has_weeks = true,
                FieldSymbol::Day(_) => has_numeric = true,
                FieldSymbol::Hour(_) => has_numeric = true,
                FieldSymbol::Minute => has_numeric = true,
                FieldSymbol::Second(_) => has_numeric = true,
                FieldSymbol::TimeZone(_) => {
                    // TODO: Consider whether time zones are supported here.
                    return Err(Error::UnsupportedField(field.symbol));
                }
            };
        }

        if has_weeks {
            self.set_week_calculator(week_calculator_loader()?);
        }

        if has_numeric || has_weeks {
            self.load_fixed_decimal_formatter(fixed_decimal_formatter_loader)?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DateTimePatternFormatter<'a, C: CldrCalendar> {
    inner: RawDateTimePatternFormatter<'a>,
    _calendar: PhantomData<C>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawDateTimePatternFormatter<'a> {
    pattern: DateTimePatternBorrowed<'a>,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a, C: CldrCalendar> DateTimePatternFormatter<'a, C> {
    /// Formats a date and time of day.
    ///
    /// For an example, see [`TypedDateTimeNames`].
    pub fn format<T>(&self, datetime: &'a T) -> FormattedDateTimePattern<'a>
    where
        T: DateTimeInput<Calendar = C>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            datetime: ExtractedDateTimeInput::extract_from(datetime),
            names: self.inner.names,
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
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create an instance that can format wide month and era names:
    /// let mut names: TypedDateTimeNames<Gregorian> =
    ///     TypedDateTimeNames::try_new(&locale!("en-GB").into()).unwrap();
    /// names
    ///     .include_month_names(fields::Month::Format, FieldLength::Wide)
    ///     .unwrap()
    ///     .include_year_names(FieldLength::Wide)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The date is:' MMMM d, y GGGG";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Test it with some different dates:
    /// // Note: extended year -50 is year 51 BCE
    /// let date_bce = Date::try_new_gregorian_date(-50, 3, 15).unwrap();
    /// let date_ce = Date::try_new_gregorian_date(1700, 11, 20).unwrap();
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date_bce),
    ///     "The date is: March 15, 51 Before Christ"
    /// );
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_date(&date_ce),
    ///     "The date is: November 20, 1700 Anno Domini"
    /// );
    /// ```
    pub fn format_date<T>(&self, datetime: &'a T) -> FormattedDateTimePattern<'a>
    where
        T: DateInput<Calendar = C>,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            datetime: ExtractedDateTimeInput::extract_from_date(datetime),
            names: self.inner.names,
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
    /// use icu::datetime::neo_pattern::DateTimePattern;
    /// use icu::datetime::TypedDateTimeNames;
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // Create an instance that can format abbreviated day periods:
    /// let mut names: TypedDateTimeNames<Gregorian> =
    ///     TypedDateTimeNames::try_new(&locale!("en-US").into()).unwrap();
    /// names
    ///     .include_day_period_names(FieldLength::Abbreviated)
    ///     .unwrap();
    ///
    /// // Create a pattern from a pattern string:
    /// let pattern_str = "'The time is:' h:mm b";
    /// let pattern: DateTimePattern = pattern_str.parse().unwrap();
    ///
    /// // Test it with different times of day:
    /// let time_am = Time::try_new(11, 4, 14, 0).unwrap();
    /// let time_pm = Time::try_new(13, 41, 28, 0).unwrap();
    /// let time_noon = Time::try_new(12, 0, 0, 0).unwrap();
    /// let time_midnight = Time::try_new(0, 0, 0, 0).unwrap();
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_am),
    ///     "The time is: 11:04 AM"
    /// );
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_pm),
    ///     "The time is: 1:41 PM"
    /// );
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_noon),
    ///     "The time is: 12:00 noon"
    /// );
    /// assert_writeable_eq!(
    ///     names.with_pattern(&pattern).format_time(&time_midnight),
    ///     "The time is: 12:00 midnight"
    /// );
    /// ```
    pub fn format_time<T>(&self, datetime: &'a T) -> FormattedDateTimePattern<'a>
    where
        T: IsoTimeInput,
    {
        FormattedDateTimePattern {
            pattern: self.inner.pattern,
            datetime: ExtractedDateTimeInput::extract_from_time(datetime),
            names: self.inner.names,
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
pub struct FormattedDateTimePattern<'a> {
    pattern: DateTimePatternBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

impl<'a> Writeable for FormattedDateTimePattern<'a> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let loc_datetime =
            DateTimeInputWithWeekConfig::new(&self.datetime, self.names.week_calculator);
        let Some(fixed_decimal_formatter) = self.names.fixed_decimal_formatter else {
            // TODO(#4340): Make the FixedDecimalFormatter optional
            icu_provider::_internal::log::warn!("FixedDecimalFormatter not loaded");
            return Err(core::fmt::Error);
        };
        write_pattern(
            self.pattern.0.items.iter(),
            self.pattern.0.metadata,
            Some(&self.names),
            Some(&self.names),
            &loc_datetime,
            fixed_decimal_formatter,
            sink,
        )
        .map_err(|_e| {
            icu_provider::_internal::log::warn!("{_e:?}");
            core::fmt::Error
        })
    }

    // TODO(#489): Implement writeable_length_hint
}

writeable::impl_display_with_writeable!(FormattedDateTimePattern<'_>);

impl<'data> DateSymbols<'data> for RawDateTimeNamesBorrowed<'data> {
    fn get_symbol_for_month(
        &self,
        field_symbol: fields::Month,
        field_length: FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, Error> {
        let field = fields::Field {
            symbol: FieldSymbol::Month(field_symbol),
            length: field_length,
        };
        let month_symbols = self
            .month_names
            .get_with_length(field_symbol, field_length)
            .ok_or(Error::MissingNames(field))?;
        let Some((month_number, is_leap)) = code.parsed() else {
            return Err(Error::MissingMonthSymbol(code));
        };
        let Some(month_index) = month_number.checked_sub(1) else {
            return Err(Error::MissingMonthSymbol(code));
        };
        let month_index = usize::from(month_index);
        let symbol = match month_symbols {
            MonthNamesV1::Linear(linear) => {
                if is_leap {
                    None
                } else {
                    linear.get(month_index)
                }
            }
            MonthNamesV1::LeapLinear(leap_linear) => {
                let num_months = leap_linear.len() / 2;
                if is_leap {
                    leap_linear.get(month_index + num_months)
                } else if month_index < num_months {
                    leap_linear.get(month_index)
                } else {
                    None
                }
            }
            MonthNamesV1::LeapNumeric(leap_numeric) => {
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
            .map(MonthPlaceholderValue::PlainString)
            .ok_or(Error::MissingMonthSymbol(code))
    }

    fn get_symbol_for_weekday(
        &self,
        field_symbol: fields::Weekday,
        field_length: FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, Error> {
        let field = fields::Field {
            symbol: FieldSymbol::Weekday(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "e" and "E" have the same non-numeric names
        let field_symbol = field_symbol.to_format_symbol();
        // UTS 35 says that "E..EEE" are all Abbreviated
        // However, this doesn't apply to "e" and "c".
        let field_length = if matches!(field_symbol, fields::Weekday::Format) {
            field_length.numeric_to_abbr()
        } else {
            field_length
        };
        let weekday_symbols = self
            .weekday_names
            .get_with_length(field_symbol, field_length)
            .ok_or(Error::MissingNames(field))?;
        let day_usize = day as usize;
        weekday_symbols
            .symbols
            .get(day_usize)
            .ok_or(Error::MissingWeekdaySymbol(day_usize))
    }

    fn get_symbol_for_era<'a>(
        &'a self,
        field_length: FieldLength,
        era_code: &'a Era,
    ) -> Result<Option<&str>, Error> {
        let field = fields::Field {
            symbol: FieldSymbol::Era,
            length: field_length,
        };
        // UTS 35 says that "G..GGG" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let year_symbols = self
            .year_names
            .get_with_length((), field_length)
            .ok_or(Error::MissingNames(field))?;
        let YearNamesV1::Eras(era_symbols) = year_symbols else {
            return Err(Error::MissingNames(field));
        };
        Ok(era_symbols.get(era_code.0.as_str().into()))
    }
}

impl<'data> TimeSymbols for RawDateTimeNamesBorrowed<'data> {
    fn get_symbol_for_day_period(
        &self,
        field_symbol: fields::DayPeriod,
        field_length: FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, Error> {
        use fields::DayPeriod::NoonMidnight;
        let field = fields::Field {
            symbol: FieldSymbol::DayPeriod(field_symbol),
            length: field_length,
        };
        // UTS 35 says that "a..aaa" are all Abbreviated
        let field_length = field_length.numeric_to_abbr();
        let dayperiod_symbols = self
            .dayperiod_names
            .get_with_length((), field_length)
            .ok_or(Error::MissingNames(field))?;
        let option_value: Option<&str> = match (field_symbol, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => dayperiod_symbols
                .midnight()
                .or_else(|| dayperiod_symbols.am()),
            (NoonMidnight, 12, true) => dayperiod_symbols.noon().or_else(|| dayperiod_symbols.pm()),
            (_, hour, _) if hour < 12 => dayperiod_symbols.am(),
            _ => dayperiod_symbols.pm(),
        };
        option_value.ok_or(Error::MissingNames(field))
    }
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use icu_calendar::{DateTime, Gregorian};
    use icu_locid::locale;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic_pattern_formatting() {
        let locale = locale!("en").into();
        let mut names: TypedDateTimeNames<Gregorian> =
            TypedDateTimeNames::try_new(&locale).unwrap();
        names
            .load_month_names(
                &crate::provider::Baked,
                fields::Month::Format,
                FieldLength::Abbreviated,
            )
            .unwrap()
            .load_weekday_names(
                &crate::provider::Baked,
                fields::Weekday::Format,
                FieldLength::Wide,
            )
            .unwrap()
            .load_year_names(&crate::provider::Baked, FieldLength::Narrow)
            .unwrap()
            .load_day_period_names(&crate::provider::Baked, FieldLength::Abbreviated)
            .unwrap();
        let pattern: DateTimePattern = "'It is' EEEE, MMM d, y GGGGG 'at' hh:mm a'!'"
            .parse()
            .unwrap();
        let datetime = DateTime::try_new_gregorian_datetime(2023, 10, 25, 15, 0, 55).unwrap();
        let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

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
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<G>",
                field_length: FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GG>",
                field_length: FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGG>",
                field_length: FieldLength::Abbreviated,
                expected: "<н. е.>",
            },
            TestCase {
                pattern: "<GGGG>",
                field_length: FieldLength::Wide,
                expected: "<нашої ери>",
            },
            TestCase {
                pattern: "<GGGGG>",
                field_length: FieldLength::Narrow,
                expected: "<н.е.>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_year_names(&crate::provider::Baked, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

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
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            // 'M' and 'MM' are numeric
            TestCase {
                pattern: "<MMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<MMMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Wide,
                expected: "<листопада>",
            },
            TestCase {
                pattern: "<MMMMM>",
                field_symbol: fields::Month::Format,
                field_length: FieldLength::Narrow,
                expected: "<л>",
            },
            // 'L' and 'LL' are numeric
            TestCase {
                pattern: "<LLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Abbreviated,
                expected: "<лист.>",
            },
            TestCase {
                pattern: "<LLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Wide,
                expected: "<листопад>",
            },
            TestCase {
                pattern: "<LLLLL>",
                field_symbol: fields::Month::StandAlone,
                field_length: FieldLength::Narrow,
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
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_month_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

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
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<E>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<EEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Wide,
                expected: "<пʼятницю>",
            },
            TestCase {
                pattern: "<EEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<EEEEEE>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
            // 'e' and 'ee' are numeric
            TestCase {
                pattern: "<eee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<eeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Wide,
                expected: "<пʼятницю>",
            },
            TestCase {
                pattern: "<eeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<eeeeee>",
                field_symbol: fields::Weekday::Format,
                field_length: FieldLength::Six,
                expected: "<пт>",
            },
            // 'c' and 'cc' are numeric
            TestCase {
                pattern: "<ccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Abbreviated,
                expected: "<пт>",
            },
            TestCase {
                pattern: "<cccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Wide,
                expected: "<пʼятниця>",
            },
            TestCase {
                pattern: "<ccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Narrow,
                expected: "<П>",
            },
            TestCase {
                pattern: "<cccccc>",
                field_symbol: fields::Weekday::StandAlone,
                field_length: FieldLength::Six,
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
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_weekday_names(&crate::provider::Baked, field_symbol, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

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
            field_length: FieldLength,
            expected: &'static str,
        }
        let cases = [
            TestCase {
                pattern: "<a>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aa>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaa>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<aaaa>",
                field_length: FieldLength::Wide,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<aaaaa>",
                field_length: FieldLength::Narrow,
                expected: "<p>",
            },
            TestCase {
                pattern: "<b>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bb>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbb>",
                field_length: FieldLength::Abbreviated,
                expected: "<PM>",
            },
            TestCase {
                pattern: "<bbbb>",
                field_length: FieldLength::Wide,
                expected: "<หลังเที่ยง>",
            },
            TestCase {
                pattern: "<bbbbb>",
                field_length: FieldLength::Narrow,
                expected: "<p>",
            },
        ];
        for cas in cases {
            let TestCase {
                pattern,
                field_length,
                expected,
            } = cas;
            let mut names: TypedDateTimeNames<Gregorian> =
                TypedDateTimeNames::try_new(&locale).unwrap();
            names
                .load_day_period_names(&crate::provider::Baked, field_length)
                .unwrap();
            let pattern: DateTimePattern = pattern.parse().unwrap();
            let datetime = DateTime::try_new_gregorian_datetime(2023, 11, 17, 13, 41, 28).unwrap();
            let formatted_pattern = names.with_pattern(&pattern).format(&datetime);

            assert_writeable_eq!(formatted_pattern, expected, "{cas:?}");
        }
    }
}
