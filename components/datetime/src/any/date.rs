// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::calendar::*;
use crate::{calendar, options::length, raw};
use crate::{input::DateInput, DateTimeError, FormattedDateTime};
use alloc::string::String;
use icu_calendar::any_calendar::{AnyCalendar, AnyCalendarKind};
use icu_calendar::provider::{
    JapaneseErasV1Marker, JapaneseExtendedErasV1Marker, WeekDataV1Marker,
};
use icu_calendar::Date;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;
use icu_provider::DataLocale;
use writeable::Writeable;

/// [`DateFormatter`] is a formatter capable of formatting
/// dates from any calendar, selected at runtime. For the difference between this and [`TypedDateFormatter`](crate::TypedDateFormatter),
/// please read the [crate root docs][crate].
///
/// When constructed, it uses data from the [data provider], selected locale and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateFormatter`], and then fast formatting of [`DateTime`](icu_calendar::DateTime) data using the instance.
///
/// [`icu_datetime`]: crate
///
/// # Examples
///
/// ```
/// use icu::calendar::{any_calendar::AnyCalendar, Date, Gregorian};
/// use icu::datetime::{options::length, DateFormatter};
/// use icu::locid::locale;
/// use std::str::FromStr;
/// use writeable::assert_writeable_eq;
///
/// let length = length::Date::Medium;
///
/// let df = DateFormatter::try_new_with_length(
///     &locale!("en-u-ca-gregory").into(),
///     length,
/// )
/// .expect("Failed to create TypedDateFormatter instance.");
///
/// let date =
///     Date::try_new_iso_date(2020, 9, 1).expect("Failed to construct Date.");
/// let any_date = date.to_any();
///
/// assert_writeable_eq!(
///     df.format(&any_date).expect("Calendars should match"),
///     "Sep 1, 2020"
/// );
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct DateFormatter(pub(crate) raw::DateFormatter, pub(crate) AnyCalendar);

impl DateFormatter {
    /// Construct a new [`DateFormatter`] from compiled data.
    ///
    /// This method will pick the calendar off of the locale; and if unspecified or unknown will fall back to the default
    /// calendar for the locale. See [`AnyCalendarKind`] for a list of supported calendars.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{any_calendar::AnyCalendar, Date, Gregorian};
    /// use icu::datetime::{options::length, DateFormatter};
    /// use icu::locid::locale;
    /// use icu_provider::any::DynamicDataProviderAnyMarkerWrap;
    /// use std::str::FromStr;
    /// use writeable::assert_writeable_eq;
    ///
    /// let length = length::Date::Medium;
    /// let locale = locale!("en-u-ca-gregory");
    ///
    /// let df = DateFormatter::try_new_with_length(
    ///     &locale.into(),
    ///     length,
    /// )
    /// .expect("Failed to create TypedDateFormatter instance.");
    ///
    /// let datetime =
    ///     Date::try_new_iso_date(2020, 9, 1).expect("Failed to construct Date.");
    /// let any_datetime = datetime.to_any();
    ///
    /// assert_writeable_eq!(
    ///     df.format(&any_datetime).expect("Calendars should match"),
    ///     "Sep 1, 2020"
    /// );
    /// ```
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[inline(never)]
    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_length(
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError> {
        let calendar = AnyCalendar::new_for_locale(locale);
        let kind = calendar.kind();

        Ok(Self(
            raw::DateFormatter::try_new(
                calendar::load_lengths_for_any_calendar_kind(
                    &crate::provider::Baked,
                    locale,
                    kind,
                )?,
                || {
                    calendar::load_symbols_for_any_calendar_kind(
                        &crate::provider::Baked,
                        locale,
                        kind,
                    )
                },
                locale,
                length,
            )?,
            calendar,
        ))
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::try_new_with_length)]
    #[inline]
    pub fn try_new_with_length_with_any_provider(
        provider: &impl AnyProvider,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError> {
        let downcasting = provider.as_downcasting();
        Self::try_new_with_length_unstable(&downcasting, locale, length)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::try_new_with_length)]
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_length_with_buffer_provider(
        provider: &impl BufferProvider,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError> {
        let deserializing = provider.as_deserializing();
        Self::try_new_with_length_unstable(&deserializing, locale, length)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_length)]
    #[inline(never)]
    pub fn try_new_with_length_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<GregorianDateLengthsV1Marker>
            + DataProvider<BuddhistDateLengthsV1Marker>
            + DataProvider<ChineseDateLengthsV1Marker>
            + DataProvider<JapaneseDateLengthsV1Marker>
            + DataProvider<JapaneseExtendedDateLengthsV1Marker>
            + DataProvider<CopticDateLengthsV1Marker>
            + DataProvider<IndianDateLengthsV1Marker>
            + DataProvider<EthiopianDateLengthsV1Marker>
            + DataProvider<GregorianDateSymbolsV1Marker>
            + DataProvider<BuddhistDateSymbolsV1Marker>
            + DataProvider<ChineseDateSymbolsV1Marker>
            + DataProvider<JapaneseDateSymbolsV1Marker>
            + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
            + DataProvider<CopticDateSymbolsV1Marker>
            + DataProvider<IndianDateSymbolsV1Marker>
            + DataProvider<EthiopianDateSymbolsV1Marker>
            + DataProvider<JapaneseErasV1Marker>
            + DataProvider<JapaneseExtendedErasV1Marker>
            + ?Sized,
    {
        let calendar = AnyCalendar::try_new_for_locale_unstable(provider, locale)?;
        let kind = calendar.kind();

        Ok(Self(
            raw::DateFormatter::try_new_unstable(
                provider,
                calendar::load_lengths_for_any_calendar_kind(provider, locale, kind)?,
                || calendar::load_symbols_for_any_calendar_kind(provider, locale, kind),
                locale,
                length,
            )?,
            calendar,
        ))
    }

    /// Takes a [`DateInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> Result<FormattedDateTime<'l>, DateTimeError>
    where
        T: DateInput<Calendar = AnyCalendar>,
    {
        if let Some(converted) = self.convert_if_necessary(value)? {
            Ok(self.0.format(&converted))
        } else {
            Ok(self.0.format(value))
        }
    }

    /// Takes a [`DateInput`] implementer and returns it formatted as a string.
    ///
    /// This function will fail if the date passed in uses a different calendar than that of the
    /// AnyCalendar. Please convert dates before passing them in if necessary. This function
    /// will automatically convert and format dates that are associated with the ISO calendar.
    #[inline]
    pub fn format_to_string(
        &self,
        value: &impl DateInput<Calendar = AnyCalendar>,
    ) -> Result<String, DateTimeError> {
        Ok(self.format(value)?.write_to_string().into_owned())
    }

    /// Converts a date to the correct calendar if necessary
    ///
    /// Returns Err if the date is not ISO or compatible with the current calendar, returns Ok(None)
    /// if the date is compatible with the current calendar and doesn't need conversion
    fn convert_if_necessary<'a>(
        &'a self,
        value: &impl DateInput<Calendar = AnyCalendar>,
    ) -> Result<Option<Date<icu_calendar::Ref<'a, AnyCalendar>>>, DateTimeError> {
        let this_calendar = self.1.kind();
        let date_calendar = value.any_calendar_kind();

        if Some(this_calendar) != date_calendar {
            if date_calendar != Some(AnyCalendarKind::Iso) {
                return Err(DateTimeError::MismatchedAnyCalendar(
                    this_calendar,
                    date_calendar,
                ));
            }
            let date = value.to_iso().to_any();
            let converted = self.1.convert_any_date(&date);
            Ok(Some(converted))
        } else {
            Ok(None)
        }
    }
}

#[test]
#[cfg(feature = "serde")]
fn serde_constructor() {
    use icu::calendar::Date;
    use icu::datetime::{options::length, DateFormatter};
    use icu::locid::locale;
    use writeable::assert_writeable_eq;

    let provider = icu_provider_blob::BlobDataProvider::try_new_from_static_blob(include_bytes!(
        "../../tests/data/blob.postcard"
    ))
    .unwrap();

    let df = DateFormatter::try_new_with_length_with_buffer_provider(
        &provider,
        &locale!("en").into(),
        length::Date::Medium,
    )
    .unwrap();

    assert_writeable_eq!(
        df.format(&Date::try_new_iso_date(2020, 9, 1).unwrap().to_any())
            .unwrap(),
        "Sep 1, 2020"
    );
}
