// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Internal traits and structs for loading data from other crates.

use icu_calendar::provider::WeekDataV2Marker;
use icu_calendar::week::WeekCalculator;
use icu_calendar::CalendarError;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::{DecimalError, FixedDecimalFormatter};
use icu_provider::prelude::*;

/// Trait for loading a FixedDecimalFormatter.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait FixedDecimalFormatterLoader {
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError>;
}

/// Trait for loading a WeekCalculator.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait WeekCalculatorLoader {
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, CalendarError>;
}

/// Helper for type resolution with optional loader arguments
pub(crate) struct PhantomLoader {
    _not_constructible: core::convert::Infallible,
}

impl FixedDecimalFormatterLoader for PhantomLoader {
    fn load(
        &self,
        _locale: &DataLocale,
        _options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        unreachable!() // not constructible
    }
}

impl WeekCalculatorLoader for PhantomLoader {
    #[inline]
    fn load(&self, _locale: &DataLocale) -> Result<WeekCalculator, CalendarError> {
        unreachable!() // not constructible
    }
}

/// Loader for types from other crates using compiled data.
pub(crate) struct ExternalLoaderCompiledData;

impl FixedDecimalFormatterLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new(locale, options)
    }
}

impl WeekCalculatorLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new(locale)
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
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_with_any_provider(self.0, locale, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderAny<'_, P>
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_with_any_provider(self.0, locale)
    }
}

/// Loader for types from other crates using [`BufferProvider`].
pub(crate) struct ExternalLoaderBuffer<'a, P: ?Sized>(pub &'a P);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_with_buffer_provider(self.0, locale, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_with_buffer_provider(self.0, locale)
    }
}

/// Loader for types from other crates using [`DataProvider`].
pub(crate) struct ExternalLoaderUnstable<'a, P: ?Sized>(pub &'a P);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized + DataProvider<DecimalSymbolsV1Marker>,
{
    #[inline]
    fn load(
        &self,
        locale: &DataLocale,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_unstable(self.0, locale, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized + DataProvider<WeekDataV2Marker>,
{
    #[inline]
    fn load(&self, locale: &DataLocale) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_unstable(self.0, locale)
    }
}
