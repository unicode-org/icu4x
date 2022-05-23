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

/// [`AnyDateTimeFormat`] is a [`DateTimeFormat`](crate::DateTimeFormat) capable of formatting
/// dates from any calendar, selected at runtime.
pub struct AnyDateTimeFormat(pub(super) raw::DateTimeFormat, AnyCalendar);

impl AnyDateTimeFormat {
    /// TBD
    #[inline]
    pub fn try_new_with_any_provider<T: Into<Locale>, P>(
        locale: T,
        data_provider: &P,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        P: AnyProvider
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
        P: BufferProvider
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

    /// ...
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l, T>
    where
        T: DateTimeInput<Calendar = AnyCalendar>,
    {
        self.0.format(value)
    }

    ///  ...
    #[inline]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput<Calendar = AnyCalendar>,
    ) -> core::fmt::Result {
        self.0.format_to_write(w, value)
    }

    /// ..
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput<Calendar = AnyCalendar>) -> String {
        self.0.format_to_string(value)
    }

    /// ...
    pub fn resolve_components(&self) -> components::Bag {
        self.0.resolve_components()
    }
}
