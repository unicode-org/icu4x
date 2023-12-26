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

/// Marker type so that the loader trait is implemented for [`AnyProvider`]
pub(crate) struct AnyProviderMarker;

/// Marker type so that the loader trait is implemented for [`BufferProvider`]
pub(crate) struct BufferProviderMarker;

/// Marker type so that the loader trait is implemented for [`DataProvider`]
pub(crate) struct DataProviderMarker;

/// Trait for loading a FixedDecimalFormatter. Blanked implemented on closures
/// with the same signature as this trait's only associated function.
///
/// Implemented for compiled data on a [`DataLocale`] 1-tuple.
///
/// For explicit providers, implemented on a 3-tuple including the provider
/// and the appropriate marker type.
pub(crate) trait FixedDecimalFormatterLoader {
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError>;
}

impl FixedDecimalFormatterLoader for (&DataLocale,) {
    #[inline]
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new(self.0, options)
    }
}

impl<P> FixedDecimalFormatterLoader for (&P, &DataLocale, AnyProviderMarker)
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

impl<P> FixedDecimalFormatterLoader for (&P, &DataLocale, BufferProviderMarker)
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

impl<P> FixedDecimalFormatterLoader for (&P, &DataLocale, DataProviderMarker)
where
    P: ?Sized + DataProvider<DecimalSymbolsV1Marker>,
{
    #[inline]
    fn load(
        self,
        options: FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, DecimalError> {
        FixedDecimalFormatter::try_new_unstable(self.0, self.1, options)
    }
}

/// Trait for loading a WeekCalculator. Blanked implemented on closures
/// with the same signature as this trait's only associated function.
///
/// Implemented for compiled data on a [`DataLocale`] 1-tuple.
///
/// For explicit providers, implemented on a 3-tuple including the provider
/// and the appropriate marker type.
pub(crate) trait WeekCalculatorLoader {
    fn load(self) -> Result<WeekCalculator, CalendarError>;
}

impl WeekCalculatorLoader for (&DataLocale,) {
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new(self.0)
    }
}

impl<P> WeekCalculatorLoader for (&P, &DataLocale, AnyProviderMarker)
where
    P: ?Sized + AnyProvider,
{
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_with_any_provider(self.0, self.1)
    }
}

impl<P> WeekCalculatorLoader for (&P, &DataLocale, BufferProviderMarker)
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_with_buffer_provider(self.0, self.1)
    }
}

impl<P> WeekCalculatorLoader for (&P, &DataLocale, DataProviderMarker)
where
    P: ?Sized + DataProvider<WeekDataV2Marker>,
{
    #[inline]
    fn load(self) -> Result<WeekCalculator, CalendarError> {
        WeekCalculator::try_new_unstable(self.0, self.1)
    }
}
