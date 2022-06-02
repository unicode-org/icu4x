// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains the untyped [`AnyCalendar`]-based `DateTimeFormat` APIs that are
//! capable of formatting dates from any calendar

use crate::{
    options::{components, DateTimeFormatOptions},
    provider::calendar::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
    provider::week_data::WeekDataV1Marker,
    raw,
};
use alloc::string::String;
use core::marker::PhantomData;
use icu_locid::{unicode_ext_key, Locale};
use icu_plurals::provider::OrdinalV1Marker;
use icu_provider::prelude::*;

use crate::{date::DateTimeInput, CldrCalendar, DateTimeFormatError, FormattedDateTime};

use icu_calendar::any_calendar::{AnyCalendar, AnyCalendarKind};
use icu_calendar::{AsCalendar, DateTime, Ref};

/// [`AnyDateTimeFormat`] is a [`DateTimeFormat`](crate::DateTimeFormat) capable of formatting
/// dates from any calendar, selected at runtime.
///
/// When constructed, it uses data from the [data provider], selected [`Locale`] and provided options to
/// collect all data necessary to format any dates into that locale.
///
/// For that reason, one should think of the process of formatting a date in two steps - first, a computational
/// heavy construction of [`DateTimeFormat`], and then fast formatting of [`DateTime`] data using the instance.
///
/// [`icu_datetime`]: crate
/// [`DateTimeFormat`]: crate::datetime::DateTimeFormat
///
/// # Examples
///
/// ```
/// use icu::calendar::{any_calendar::AnyCalendar, DateTime, Gregorian};
/// use icu::datetime::{options::length, any::AnyDateTimeFormat};
/// use icu::locid::Locale;
/// use icu_provider::any::DynProviderAnyMarkerWrap;
/// use std::str::FromStr;
///
/// let provider = icu_testdata::get_provider();
/// // let provider = DynProviderAnyMarkerWrap(&provider);
///
/// let mut options = length::Bag::from_date_time_style(length::Date::Medium, length::Time::Short);
///
/// let dtf = AnyDateTimeFormat::try_new_with_buffer_provider(Locale::from_str("en-u-ca-gregory").unwrap(), &provider, &options.into())
///     .expect("Failed to create DateTimeFormat instance.");
///
/// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28, 0)
///     .expect("Failed to construct DateTime.");
/// let any_datetime = datetime.to_any();
///
/// let value = dtf.format_to_string(&any_datetime);
/// assert_eq!(value, "Sep 1, 2020, 12:34 PM");
/// ```
///
/// This model replicates that of `ICU` and `ECMA402`.
///
/// [data provider]: icu_provider
pub struct AnyDateTimeFormat(pub(super) raw::DateTimeFormat, AnyCalendar);

impl AnyDateTimeFormat {
    ///
    #[inline]
    pub fn try_new_with_any_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        P: AnyProvider,
    {
        let locale = locale.into();

        let kind = AnyCalendarKind::from_locale(&locale).unwrap(); // XXXManishearth unwrap

        let calendar = AnyCalendar::try_new_with_any_provider(kind, data_provider)?;

        let downcasting = data_provider.as_downcasting();
        Ok(Self(
            raw::DateTimeFormat::try_new(locale, &downcasting, options)?,
            calendar,
        ))
    }

    /// TBD
    #[inline]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        P: BufferProvider,
    {
        let locale = locale.into();

        let kind = AnyCalendarKind::from_locale(&locale).unwrap(); // XXXManishearth unwrap

        let calendar = AnyCalendar::try_new_with_buffer_provider(kind, data_provider)?;

        let deserializing = data_provider.as_deserializing();
        Ok(Self(
            raw::DateTimeFormat::try_new(locale, &deserializing, options)?,
            calendar,
        ))
    }

    ///  ...
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<(), DateTimeFormatError> {
        self.check_calendars(value)?;
        self.0.format_to_write(w, value)?;
        Ok(())
    }

    /// ..
    #[inline]
    pub fn format_to_string(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<String, DateTimeFormatError> {
        self.check_calendars(value)?;
        Ok(self.0.format_to_string(value))
    }

    /// ...
    pub fn resolve_components(&self) -> components::Bag {
        self.0.resolve_components()
    }

    /// Checks if a date is constructed with the same calendar
    fn check_calendars(
        &self,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> Result<(), DateTimeFormatError> {
        let this_calendar = self.1.kind();
        let date_calendar = value.any_calendar_kind();
        if Some(this_calendar) != date_calendar {
            return Err(DateTimeFormatError::MismatchedAnyCalendar(
                this_calendar,
                date_calendar,
            ));
        }

        Ok(())
    }
}
