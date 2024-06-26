// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Internal traits and structs for loading data from other crates.

use icu_calendar::week::WeekCalculator;
use icu_calendar::AnyCalendar;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;

/// Trait for loading a FixedDecimalFormatter.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait FixedDecimalFormatterLoader {
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DataError>;
}

/// Trait for loading a WeekCalculator.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait WeekCalculatorLoader {
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, DataError>;
}

/// Trait for loading an AnyCalendar.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait AnyCalendarLoader {
    fn load(&self, locale: &DataLocale) -> Result<AnyCalendar, DataError>;
}

/// Loader for types from other crates using compiled data.
#[cfg(feature = "compiled_data")]
pub(crate) struct ExternalLoaderCompiledData;

#[cfg(feature = "compiled_data")]
impl FixedDecimalFormatterLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DataError> {
        FixedDecimalFormatter::try_new(locale, options)
    }
}

#[cfg(feature = "compiled_data")]
impl WeekCalculatorLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, DataError> {
        WeekCalculator::try_new(locale)
    }
}

#[cfg(feature = "compiled_data")]
impl AnyCalendarLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<AnyCalendar, DataError> {
        Ok(AnyCalendar::new_for_locale(locale))
    }
}

/// Loader for types from other crates using [`AnyProvider`].
pub(crate) struct ExternalLoaderAny<'a, P: ?Sized>(pub &'a P);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderAny<'_, P>
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DataError> {
        FixedDecimalFormatter::try_new_with_any_provider(self.0, locale, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderAny<'_, P>
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, DataError> {
        WeekCalculator::try_new_with_any_provider(self.0, locale)
    }
}

impl<P> AnyCalendarLoader for ExternalLoaderAny<'_, P>
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<AnyCalendar, DataError> {
        AnyCalendar::try_new_for_locale_with_any_provider(self.0, locale)
    }
}

/// Loader for types from other crates using [`BufferProvider`].
#[cfg(feature = "serde")]
pub(crate) struct ExternalLoaderBuffer<'a, P: ?Sized>(pub &'a P);

#[cfg(feature = "serde")]
impl<P> FixedDecimalFormatterLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DataError> {
        FixedDecimalFormatter::try_new_with_buffer_provider(self.0, locale, options)
    }
}

#[cfg(feature = "serde")]
impl<P> WeekCalculatorLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, DataError> {
        WeekCalculator::try_new_with_buffer_provider(self.0, locale)
    }
}

#[cfg(feature = "serde")]
impl<P> AnyCalendarLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<AnyCalendar, DataError> {
        AnyCalendar::try_new_for_locale_with_buffer_provider(self.0, locale)
    }
}

/// Loader for types from other crates using [`DataProvider`].
pub(crate) struct ExternalLoaderUnstable<'a, P: ?Sized>(pub &'a P);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>,
{
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DataError> {
        FixedDecimalFormatter::try_new_unstable(self.0, locale, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized + DataProvider<icu_calendar::provider::WeekDataV2Marker>,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, DataError> {
        WeekCalculator::try_new_unstable(self.0, locale)
    }
}

impl<P> AnyCalendarLoader for ExternalLoaderUnstable<'_, P>
where
    P: DataProvider<icu_calendar::provider::JapaneseErasV1Marker>
        + DataProvider<icu_calendar::provider::JapaneseExtendedErasV1Marker>
        + DataProvider<icu_calendar::provider::ChineseCacheV1Marker>
        + DataProvider<icu_calendar::provider::DangiCacheV1Marker>
        + DataProvider<icu_calendar::provider::IslamicObservationalCacheV1Marker>
        + DataProvider<icu_calendar::provider::IslamicUmmAlQuraCacheV1Marker>
        + ?Sized,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<AnyCalendar, DataError> {
        AnyCalendar::try_new_for_locale_unstable(self.0, locale)
    }
}
