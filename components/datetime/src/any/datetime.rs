// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::helpers::size_test;
use crate::provider::{calendar::*, date_time::PatternSelector};
use crate::{calendar, options::DateTimeFormatterOptions, raw, DateFormatter, TimeFormatter};
use crate::{input::DateTimeInput, DateTimeError, FormattedDateTime};
use alloc::string::String;
use icu_calendar::any_calendar::AnyCalendar;
use icu_calendar::provider::{
    ChineseCacheV1Marker, DangiCacheV1Marker, JapaneseErasV1Marker, JapaneseExtendedErasV1Marker,
    WeekDataV1Marker,
};
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;
use icu_provider::DataLocale;
use writeable::Writeable;

size_test!(DateTimeFormatter, date_time_formatter_size, 5208);

/// [`DateTimeFormatter`] is a formatter capable of formatting
/// date/times from any calendar, selected at runtime. For the difference between this and [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter),
/// please read the [crate root docs][crate].
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateTimeFormatter`], and then fast formatting of [`DateTime`](icu_calendar::DateTime) data using the instance.
///
#[doc = date_time_formatter_size!()]
///
/// [`icu_datetime`]: crate
///
/// # Examples
///
/// ```
/// use icu::calendar::DateTime;
/// use icu::datetime::{options::length, DateTimeFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
///
/// let mut options = length::Bag::from_date_time_style(
///     length::Date::Medium,
///     length::Time::Short,
/// );
///
/// let dtf = DateTimeFormatter::try_new(
///     &locale!("en-u-ca-gregory").into(),
///     options.into(),
/// )
/// .expect("Failed to create DateTimeFormatter instance.");
///
/// let datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
/// let any_datetime = datetime.to_any();
///
/// assert_writeable_eq!(
///     dtf.format(&any_datetime).expect("Calendars should match"),
///     "Sep 1, 2020, 12:34 PM"
/// );
/// ```
///
/// Since this works with [`AnyCalendar`], you can use [`DateTime`](icu_calendar::DateTime) with [`AnyCalendar`]
/// to have a date in a runtime-selected calendar:
///
/// ```
/// use icu::calendar::{AnyCalendar, DateTime, Time};
/// use icu::datetime::{options::length, DateTimeFormatter};
/// use icu::locid::locale;
/// use writeable::assert_writeable_eq;
/// # use std::rc::Rc;
///
/// let locale = locale!("en-u-ca-japanese").into(); // English with the Japanese calendar
///
/// let calendar = AnyCalendar::new_for_locale(&locale);
/// let calendar = Rc::new(calendar); // Avoid cloning it
///
///
/// // manually construct a datetime in this calendar
/// let manual_time = Time::try_new(12, 33, 12, 0).expect("failed to construct Time");
/// // construct from era code, year, month code, day, time, and a calendar
/// // This is March 28, 15 Heisei
/// let manual_datetime = DateTime::try_new_from_codes("heisei".parse().unwrap(), 15, "M03".parse().unwrap(), 28,
///                                                manual_time, calendar.clone())
///                     .expect("Failed to construct DateTime manually");
///
///
/// // construct another datetime by converting from ISO
/// let iso_datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct ISO DateTime.");
/// let iso_converted = iso_datetime.to_calendar(calendar);
///
///
/// let options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
///
/// let dtf = DateTimeFormatter::try_new(&locale, options.into())
///     .expect("Failed to create DateTimeFormatter instance.");
///
/// let manual_value = dtf.format(&manual_datetime).expect("Calendars should match");
/// assert_writeable_eq!(manual_value, "Mar 28, 15 Heisei, 12:33 PM");
///
/// let converted_value = dtf.format(&iso_converted).expect("Calendars should match");
/// assert_writeable_eq!(converted_value, "Sep 1, 2 Reiwa, 12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct DateTimeFormatter(pub(crate) raw::DateTimeFormatter, AnyCalendar);

impl DateTimeFormatter {
    /// Construct a new [`DateTimeFormatter`] from compiled data.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`](icu_calendar::AnyCalendarKind) for a list of supported calendars.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::length, DateTimeFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let options = length::Bag::from_date_time_style(
    ///     length::Date::Medium,
    ///     length::Time::Short,
    /// );
    /// let locale = locale!("en-u-ca-gregory");
    ///
    /// let dtf = DateTimeFormatter::try_new(&locale.into(), options.into())
    ///     .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     dtf.format(&any_datetime).expect("Calendars should match"),
    ///     "Sep 1, 2020, 12:34 PM"
    /// );
    /// ```
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub fn try_new(
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let calendar = AnyCalendar::new_for_locale(locale);
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options(
            &crate::provider::Baked,
            calendar::load_lengths_for_any_calendar_kind(&crate::provider::Baked, locale, kind)?,
            locale,
            &options,
        )?;

        Ok(Self(
            raw::DateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_any_calendar_kind(
                        &crate::provider::Baked,
                        locale,
                        kind,
                    )
                },
                locale,
            )?,
            calendar,
        ))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::try_new)]
    #[inline]
    pub fn try_new_with_any_provider(
        provider: &impl AnyProvider,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let downcasting = provider.as_downcasting();
        Self::try_new_unstable(&downcasting, locale, options)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::try_new)]
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider(
        provider: &impl BufferProvider,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let deserializing = provider.as_deserializing();
        Self::try_new_unstable(&deserializing, locale, options)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_experimental)]
    #[cfg(feature = "experimental")]
    #[inline(never)]
    pub fn try_new_experimental_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<crate::provider::calendar::DateSkeletonPatternsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<ChineseDateLengthsV1Marker>
            + DataProvider<ChineseDateSymbolsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<DangiDateLengthsV1Marker>
            + DataProvider<DangiDateSymbolsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<HebrewDateLengthsV1Marker>
            + DataProvider<HebrewDateSymbolsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<IslamicDateLengthsV1Marker>
            + DataProvider<IslamicDateSymbolsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + DataProvider<PersianDateLengthsV1Marker>
            + DataProvider<PersianDateSymbolsV1Marker>
            + DataProvider<RocDateLengthsV1Marker>
            + DataProvider<RocDateSymbolsV1Marker>
            + ?Sized,
    {
        let calendar = AnyCalendar::try_new_for_locale_unstable(provider, locale)?;
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options_experimental(
            provider,
            calendar::load_lengths_for_any_calendar_kind(provider, locale, kind)?,
            locale,
            &kind.as_bcp47_value(),
            &options,
        )?;

        Ok(Self(
            raw::DateTimeFormatter::try_new_unstable(
                provider,
                patterns,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
            )?,
            calendar,
        ))
    }

    /// Constructor that supports experimental options with compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` and `experimental` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
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
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{options::components, DateTimeFormatter};
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let mut options = components::Bag::default();
    /// options.year = Some(components::Year::Numeric);
    /// options.month = Some(components::Month::Long);
    ///
    /// let dtf = DateTimeFormatter::try_new_experimental(
    ///     &locale!("en-u-ca-gregory").into(),
    ///     options.into(),
    /// )
    /// .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     dtf.format(&any_datetime).expect("Calendars should match"),
    ///     "September 2020"
    /// );
    /// ```
    #[cfg(feature = "experimental")]
    #[cfg(feature = "compiled_data")]
    #[inline(never)]
    pub fn try_new_experimental(
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError> {
        let calendar = AnyCalendar::new_for_locale(locale);
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options_experimental(
            &crate::provider::Baked,
            calendar::load_lengths_for_any_calendar_kind(&crate::provider::Baked, locale, kind)?,
            locale,
            &kind.as_bcp47_value(),
            &options,
        )?;

        Ok(Self(
            raw::DateTimeFormatter::try_new(
                patterns,
                || {
                    calendar::load_symbols_for_any_calendar_kind(
                        &crate::provider::Baked,
                        locale,
                        kind,
                    )
                },
                locale,
            )?,
            calendar,
        ))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    #[inline(never)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        options: DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<ChineseCacheV1Marker>
            + DataProvider<ChineseDateLengthsV1Marker>
            + DataProvider<ChineseDateSymbolsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<DangiCacheV1Marker>
            + DataProvider<DangiDateLengthsV1Marker>
            + DataProvider<DangiDateSymbolsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<HebrewDateLengthsV1Marker>
            + DataProvider<HebrewDateSymbolsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<IslamicDateLengthsV1Marker>
            + DataProvider<IslamicDateSymbolsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + DataProvider<PersianDateLengthsV1Marker>
            + DataProvider<PersianDateSymbolsV1Marker>
            + DataProvider<RocDateLengthsV1Marker>
            + DataProvider<RocDateSymbolsV1Marker>
            + ?Sized,
    {
        let calendar = AnyCalendar::try_new_for_locale_unstable(provider, locale)?;
        let kind = calendar.kind();

        let patterns = PatternSelector::for_options(
            provider,
            calendar::load_lengths_for_any_calendar_kind(provider, locale, kind)?,
            locale,
            &options,
        )?;

        Ok(Self(
            raw::DateTimeFormatter::try_new_unstable(
                provider,
                patterns,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
            )?,
            calendar,
        ))
    }

    /// Constructor that takes a [`TimeFormatter`] and [`DateFormatter`] and combines them into a [`DateTimeFormatter`].
    /// Prefer using one of the other constructors if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::datetime::{
    ///     options::length, DateFormatter, DateTimeFormatter, TimeFormatter,
    /// };
    /// use icu::locid::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let length = length::Date::Medium;
    /// let locale = locale!("en-u-ca-gregory");
    ///
    /// let df = DateFormatter::try_new_with_length(&locale.into(), length)
    ///     .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let tf = TimeFormatter::try_new_with_length(
    ///     &locale!("en").into(),
    ///     length::Time::Short,
    /// )
    /// .expect("Failed to create TimeFormatter instance.");
    ///
    /// let dtf = DateTimeFormatter::try_from_date_and_time(df, tf).unwrap();
    ///
    /// let datetime = DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     dtf.format(&any_datetime).expect("Calendars should match"),
    ///     "Sep 1, 2020, 12:34 PM"
    /// );
    /// ```
    ///
    /// [data provider]: icu_provider
    #[inline]
    pub fn try_from_date_and_time(
        date: DateFormatter,
        time: TimeFormatter,
    ) -> Result<Self, DateTimeError>
where {
        Ok(Self(
            raw::DateTimeFormatter::try_from_date_and_time(date.0, time.0)?,
            date.1,
        ))
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> Result<FormattedDateTime<'l>, DateTimeError>
    where
        T: DateTimeInput<Calendar = AnyCalendar>,
    {
        if let Some(converted) = calendar::convert_datetime_if_necessary(&self.1, value)? {
            Ok(self.0.format(&converted))
        } else {
            Ok(self.0.format(value))
        }
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_string(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<String, DateTimeError> {
        Ok(self.format(value)?.write_to_string().into_owned())
    }

    /// Returns a [`components::Bag`](crate::options::components::Bag) that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormatter`]. The developer may request
    /// a certain set of options for a [`DateTimeFormatter`] but the locale and resolution
    /// algorithm may change certain details of what actually gets resolved.
    ///
    /// ✨ *Enabled with the `experimental` Cargo feature.*
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
    /// use icu::datetime::{
    ///     options::{components, length},
    ///     DateTimeFormatter,
    /// };
    /// use icu::locid::locale;
    ///
    /// let options = length::Bag::from_date_style(length::Date::Medium).into();
    ///
    /// let dtf =
    ///     DateTimeFormatter::try_new(&locale!("en-u-ca-gregory").into(), options)
    ///         .expect("Failed to create TypedDateTimeFormatter instance.");
    ///
    /// let mut expected_components_bag = components::Bag::default();
    /// expected_components_bag.year = Some(components::Year::Numeric);
    /// expected_components_bag.month = Some(components::Month::Short);
    /// expected_components_bag.day = Some(components::Day::NumericDayOfMonth);
    ///
    /// assert_eq!(dtf.resolve_components(), expected_components_bag);
    /// ```
    #[cfg(feature = "experimental")]
    pub fn resolve_components(&self) -> crate::options::components::Bag {
        self.0.resolve_components()
    }
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use icu::calendar::{AnyCalendar, DateTime};
    use icu::datetime::{options::length, DateTimeFormatter};
    use icu::locid::{locale, Locale};

    fn test_format(datetime: &DateTime<AnyCalendar>, locale: Locale, expected: &str) {
        let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Short);

        let dtf = DateTimeFormatter::try_new(&locale.into(), options.into()).unwrap();
        writeable::assert_writeable_eq!(
            dtf.format(datetime).expect("Calendars should match"),
            expected
        );
    }

    #[test]
    fn test_fallback() {
        // We can rely on the code's ability to convert ISO datetimes
        let datetime = DateTime::try_new_iso_datetime(2022, 4, 5, 12, 33, 44).unwrap();
        let datetime = datetime.to_any();
        // fr with unspecified and nonsense calendars falls back to gregorian
        test_format(&datetime, locale!("fr"), "5 avril 2022, 12:33");
        test_format(
            &datetime,
            locale!("fr-u-ca-blahblah"),
            "5 avril 2022, 12:33",
        );
        // thai falls back to buddhist
        test_format(
            &datetime,
            locale!("th-u-ca-buddhist"),
            "5 เมษายน 2565 12:33",
        );
        test_format(&datetime, locale!("th"), "5 เมษายน 2565 12:33");
        // except when overridden
        test_format(
            &datetime,
            locale!("th-u-ca-gregory"),
            "5 เมษายน ค.ศ. 2022 12:33",
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn works_with_default_options() {
        assert_eq!(
            DateTimeFormatter::try_new(Default::default(), Default::default(),)
                .unwrap()
                .format_to_string(
                    &DateTime::try_new_iso_datetime(2022, 9, 20, 0, 0, 0)
                        .unwrap()
                        .to_any()
                )
                .unwrap(),
            "2022 M09 20 00:00:00"
        );
    }
}

#[test]
#[cfg(feature = "serde")]
fn buffer_constructor() {
    use icu::calendar::DateTime;
    use icu::datetime::{options::length, DateTimeFormatter};
    use icu::locid::locale;
    use writeable::assert_writeable_eq;

    let provider = icu_provider_blob::BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "../../tests/data/blob.postcard"
    ))
    .unwrap();

    let dtf = DateTimeFormatter::try_new_with_buffer_provider(
        &provider,
        &locale!("en").into(),
        length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short).into(),
    )
    .unwrap();

    assert_writeable_eq!(
        dtf.format(
            &DateTime::try_new_iso_datetime(2020, 9, 1, 12, 34, 28)
                .unwrap()
                .to_any()
        )
        .expect("Calendars should match"),
        "Sep 1, 2020, 12:34 PM"
    );
}
