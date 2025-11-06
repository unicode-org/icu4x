// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Internal traits and structs for loading data from other crates.

use icu_decimal::options::DecimalFormatterOptions;
use icu_decimal::{DecimalFormatter, DecimalFormatterPreferences};
use icu_provider::prelude::*;

use crate::scaffold::{FormattableAnyCalendar, FormattableAnyCalendarKind};

/// Trait for loading a DecimalFormatter.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait DecimalFormatterLoader {
    fn load(
        &self,
        prefs: DecimalFormatterPreferences,
        options: DecimalFormatterOptions,
    ) -> Result<DecimalFormatter, DataError>;
}

/// Trait for loading an AnyCalendar.
///
/// Implemented on the provider-specific loader types in this module.
pub(crate) trait FormattableAnyCalendarLoader {
    fn load(&self, kind: FormattableAnyCalendarKind) -> Result<FormattableAnyCalendar, DataError>;
}

/// Loader for types from other crates using compiled data.
#[cfg(feature = "compiled_data")]
pub(crate) struct ExternalLoaderCompiledData;

#[cfg(feature = "compiled_data")]
impl DecimalFormatterLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(
        &self,
        prefs: DecimalFormatterPreferences,
        options: DecimalFormatterOptions,
    ) -> Result<DecimalFormatter, DataError> {
        DecimalFormatter::try_new(prefs, options)
    }
}

#[cfg(feature = "compiled_data")]
impl FormattableAnyCalendarLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(&self, kind: FormattableAnyCalendarKind) -> Result<FormattableAnyCalendar, DataError> {
        FormattableAnyCalendar::try_new(kind)
    }
}

/// Loader for types from other crates using [`BufferProvider`].
#[cfg(feature = "serde")]
pub(crate) struct ExternalLoaderBuffer<'a, P: ?Sized>(pub &'a P);

#[cfg(feature = "serde")]
impl<P> DecimalFormatterLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(
        &self,
        prefs: DecimalFormatterPreferences,
        options: DecimalFormatterOptions,
    ) -> Result<DecimalFormatter, DataError> {
        DecimalFormatter::try_new_with_buffer_provider(self.0, prefs, options)
    }
}

#[cfg(feature = "serde")]
impl<P> FormattableAnyCalendarLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(&self, kind: FormattableAnyCalendarKind) -> Result<FormattableAnyCalendar, DataError> {
        FormattableAnyCalendar::try_new_with_buffer_provider(self.0, kind)
    }
}

/// Loader for types from other crates using [`DataProvider`].
pub(crate) struct ExternalLoaderUnstable<'a, P: ?Sized>(pub &'a P);

impl<P> DecimalFormatterLoader for ExternalLoaderUnstable<'_, P>
where
    P: ?Sized
        + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
        + DataProvider<icu_decimal::provider::DecimalDigitsV1>,
{
    #[inline]
    fn load(
        &self,
        prefs: DecimalFormatterPreferences,
        options: DecimalFormatterOptions,
    ) -> Result<DecimalFormatter, DataError> {
        DecimalFormatter::try_new_unstable(self.0, prefs, options)
    }
}

impl<P> FormattableAnyCalendarLoader for ExternalLoaderUnstable<'_, P>
where
    P: DataProvider<icu_calendar::provider::CalendarJapaneseModernV1> + ?Sized,
{
    #[inline]
    fn load(&self, kind: FormattableAnyCalendarKind) -> Result<FormattableAnyCalendar, DataError> {
        FormattableAnyCalendar::try_new_unstable(self.0, kind)
    }
}
