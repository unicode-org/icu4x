// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    fields::FieldSymbol,
    format::datetime,
    options::DateTimeFormatOptions,
    pattern::PatternItem,
    provider::{gregory::DatesV1, helpers::DateTimePatterns},
};
use icu_locid::Locale;
use icu_provider::{DataProvider, DataRequest, ResourceOptions, ResourcePath};
use std::borrow::Cow;

use crate::{
    date::DateTimeInput, pattern::Pattern, provider, DateTimeFormatError, FormattedDateTime,
};

/// `DateTimeFormat` is the main structure of the `icu_datetime` component.
/// When constructed, it uses data from the `DataProvider`, selected `Locale` and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of `DateTimeFormat`, and then fast formatting of `DateTime` data using the instance.
///
/// # Examples
///
/// ```
/// use icu_locid::Locale;
/// use icu_locid_macros::langid;
/// use icu_datetime::{DateTimeFormat, options::style};
/// use icu_datetime::mock::datetime::MockDateTime;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let locale: Locale = langid!("en").into();
///
/// let provider = InvariantDataProvider;
///
/// let options = style::Bag {
///     date: Some(style::Date::Medium),
///     time: Some(style::Time::Short),
///     ..Default::default()
/// };
/// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
///
///
/// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let value = dtf.format_to_string(&date_time);
/// ```
///
/// This model replicates that of `ICU` and `ECMA402` and in the future will get even more pronounce when we introduce
/// asynchronous `DataProvider` and corresponding asynchronous constructor.
pub struct DateTimeFormat<'d> {
    pub(super) locale: Locale,
    pub(super) pattern: Pattern,
    pub(super) symbols: Cow<'d, provider::gregory::DateSymbolsV1>,
}

impl<'d> DateTimeFormat<'d> {
    /// `DateTimeFormat` constructor which takes a selected `Locale`, reference to a `DataProvider` and
    /// a list of options and collects all data necessary to format date and time values into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use icu_locid_macros::langid;
    /// use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// use icu_datetime::mock::datetime::MockDateTime;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let locale: Locale = langid!("en").into();
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let options = DateTimeFormatOptions::default();
    ///
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options);
    ///
    /// assert_eq!(dtf.is_ok(), true);
    /// ```
    pub fn try_new<T: Into<Locale>, D: DataProvider<'d, provider::gregory::DatesV1> + ?Sized>(
        locale: T,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError> {
        let locale = locale.into();
        let data = data_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.clone().into()),
                    },
                },
            })?
            .payload
            .take()?;

        let pattern = data
            .patterns
            .get_pattern_for_options(options)?
            .unwrap_or_default();

        let time_zone_field = pattern
            .items()
            .iter()
            .filter_map(|p| match p {
                PatternItem::Field(field) => Some(field),
                _ => None,
            })
            .find(|field| matches!(field.symbol, FieldSymbol::TimeZone(_)));

        if let Some(field) = time_zone_field {
            return Err(DateTimeFormatError::UnsupportedField(field.symbol));
        }

        Ok(Self::new(locale, pattern, data))
    }

    /// Creates a new `DateTimeFormat` regardless of whether there are time-zone symbols in the pattern.
    ///
    /// By contrast, the public `DateTimeFormat::try_new` function will return an `Err` if there are
    /// time-zone symbols in the pattern.
    ///
    /// This function is only `pub(super)` (not `pub`) because it is needed by `ZonedDateTimeFormat`
    /// to create a `DateTimeFormat` for use internally. `ZonedDateTimeFormat` maintains
    /// the invariant that `DateTimeFormat` will not be used to format the time zone.
    ///
    /// Creating a `DateTimeFormat` with time-zone symbols should always be an error
    /// in public contexts.
    pub(super) fn new<T: Into<Locale>>(
        locale: T,
        pattern: Pattern,
        data: Cow<'d, DatesV1>,
    ) -> Self {
        let locale = locale.into();

        let symbols = match data {
            Cow::Borrowed(data) => Cow::Borrowed(&data.symbols),
            Cow::Owned(data) => Cow::Owned(data.symbols),
        };

        Self {
            locale,
            pattern,
            symbols,
        }
    }

    /// `format` takes a `DateTime` value and returns an instance of a `FormattedDateTime` object
    /// which contains all information necessary to display a formatted date and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locid::Locale;
    /// # use icu_locid_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::mock::datetime::MockDateTime;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options)
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let formatted_date = dtf.format(&date_time);
    ///
    /// let _ = format!("Date: {}", formatted_date);
    /// ```
    ///
    /// At the moment, there's little value in using that over one of the other `format` methods,
    /// but `FormattedDateTime` will grow with methods for iterating over fields, extracting information
    /// about formatted date and so on.
    pub fn format<'s, T>(&'s self, value: &'s T) -> FormattedDateTime<'s, T>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            pattern: &self.pattern,
            symbols: &self.symbols,
            date_time: value,
            locale: &self.locale,
        }
    }

    /// `format_to_write` takes a mutable reference to anything that implements `Write` trait
    /// and a `DateTime` value and populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locid::Locale;
    /// # use icu_locid_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::mock::datetime::MockDateTime;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let mut buffer = String::new();
    /// dtf.format_to_write(&mut buffer, &date_time)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Date: {}", buffer);
    /// ```
    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl DateTimeInput,
    ) -> std::fmt::Result {
        datetime::write_pattern(&self.pattern, &self.symbols, value, &self.locale, w)
            .map_err(|_| std::fmt::Error)
    }

    /// `format_to_string` takes a `DateTime` value and returns it formatted
    /// as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use icu_locid::Locale;
    /// # use icu_locid_macros::langid;
    /// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
    /// # use icu_datetime::mock::datetime::MockDateTime;
    /// # use icu_provider::inv::InvariantDataProvider;
    /// # let locale: Locale = langid!("en").into();
    /// # let provider = InvariantDataProvider;
    /// # let options = DateTimeFormatOptions::default();
    /// let dtf = DateTimeFormat::try_new(locale, &provider, &options.into())
    ///     .expect("Failed to create DateTimeFormat instance.");
    ///
    /// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
    ///     .expect("Failed to construct DateTime.");
    ///
    /// let _ = dtf.format_to_string(&date_time);
    /// ```
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructing_datetime_format_with_zones_is_err() {
        use crate::options::style::{Bag, Time};
        use crate::{DateTimeFormat, DateTimeFormatOptions};
        use icu_locid::Locale;
        use icu_locid_macros::langid;

        let options = DateTimeFormatOptions::Style(Bag {
            time: Some(Time::Full),
            ..Default::default()
        });

        let locale: Locale = langid!("en").into();
        let provider = icu_testdata::get_provider();
        let result = DateTimeFormat::try_new(locale, &provider, &options.into());
        assert!(matches!(
            result,
            Err(DateTimeFormatError::UnsupportedField(
                FieldSymbol::TimeZone(_)
            ))
        ));
    }
}
