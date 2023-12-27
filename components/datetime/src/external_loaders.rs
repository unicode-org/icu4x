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
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError>;
}

/// Trait for loading a WeekCalculator.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait WeekCalculatorLoader {
    fn load(self) -> Result<WeekCalculator, CalendarError>;
}

/// Loader for types from other crates using compiled data.
pub(crate) struct ExternalLoaderCompiledData<'a>(pub &'a DataLocale);

impl FixedDecimalFormatterLoader for ExternalLoaderCompiledData<'_> {
    #[inline]
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new(self.0, options)
    }
}

impl WeekCalculatorLoader for ExternalLoaderCompiledData<'_> {
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new(self.0)
    }
}

/// Loader for types from other crates using [`AnyProvider`].
pub(crate) struct ExternalLoaderAny<'a, P: ?Sized>(pub &'a P, pub &'a DataLocale);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderAny<'_, P>
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_with_any_provider(self.0, self.1, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderAny<'_, P>
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_with_any_provider(self.0, self.1)
    }
}

/// Loader for types from other crates using [`BufferProvider`].
pub(crate) struct ExternalLoaderBuffer<'a, P: ?Sized>(pub &'a P, pub &'a DataLocale);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_with_buffer_provider(self.0, self.1, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_with_buffer_provider(self.0, self.1)
    }
}

/// Loader for types from other crates using [`DataProvider`].
pub(crate) struct ExternalLoaderUnstable<'a, P: ?Sized>(pub &'a P, pub &'a DataLocale);

impl<P> FixedDecimalFormatterLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized + DataProvider<DecimalSymbolsV1Marker>,
{
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_unstable(self.0, self.1, options)
    }
}

impl<P> WeekCalculatorLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized + DataProvider<WeekDataV2Marker>,
{
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_unstable(self.0, self.1)
    }
}
