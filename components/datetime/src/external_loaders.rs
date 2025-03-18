// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Internal traits and structs for loading data from other crates.

use crate::DateTimeFormatterLoadError;
use icu_calendar::preferences::{CalendarAlgorithm, HijriCalendarAlgorithm};
use icu_calendar::AnyCalendar;
use icu_decimal::options::DecimalFormatterOptions;
use icu_decimal::{DecimalFormatter, DecimalFormatterPreferences};
use icu_provider::prelude::*;

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
pub(crate) trait AnyCalendarLoader {
    fn load(&self, kind: CalendarAlgorithm) -> Result<AnyCalendar, DateTimeFormatterLoadError>;
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
impl AnyCalendarLoader for ExternalLoaderCompiledData {
    #[inline]
    fn load(&self, cal: CalendarAlgorithm) -> Result<AnyCalendar, DateTimeFormatterLoadError> {
        use icu_calendar::cal::*;
        Ok(match cal {
            CalendarAlgorithm::Buddhist => AnyCalendar::Buddhist(Buddhist),
            CalendarAlgorithm::Chinese => AnyCalendar::Chinese(Chinese::new()),
            CalendarAlgorithm::Coptic => AnyCalendar::Coptic(Coptic),
            CalendarAlgorithm::Dangi => AnyCalendar::Dangi(Dangi::new()),
            CalendarAlgorithm::Ethiopic => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            CalendarAlgorithm::Ethioaa => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
            CalendarAlgorithm::Gregory => AnyCalendar::Gregorian(Gregorian),
            CalendarAlgorithm::Hebrew => AnyCalendar::Hebrew(Hebrew),
            CalendarAlgorithm::Indian => AnyCalendar::Indian(Indian),
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil)) => {
                AnyCalendar::HijriCivil(HijriCivil)
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Rgsa)) => {
                AnyCalendar::HijriObservational(HijriObservational::new())
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                AnyCalendar::HijriTabular(HijriTabular)
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => {
                AnyCalendar::HijriUmmAlQura(HijriUmmAlQura::new())
            }
            CalendarAlgorithm::Japanese => AnyCalendar::Japanese(Japanese::new()),
            CalendarAlgorithm::Persian => AnyCalendar::Persian(Persian),
            CalendarAlgorithm::Roc => AnyCalendar::Roc(Roc),
            c => return Err(DateTimeFormatterLoadError::UnsupportedCalendar(c)),
        })
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
impl<P> AnyCalendarLoader for ExternalLoaderBuffer<'_, P>
where
    P: ?Sized + BufferProvider,
{
    #[inline]
    fn load(&self, cal: CalendarAlgorithm) -> Result<AnyCalendar, DateTimeFormatterLoadError> {
        use icu_calendar::cal::*;
        Ok(match cal {
            CalendarAlgorithm::Buddhist => AnyCalendar::Buddhist(Buddhist),
            CalendarAlgorithm::Chinese => AnyCalendar::Chinese(
                Chinese::try_new_with_buffer_provider(self.0)
                    .map_err(DateTimeFormatterLoadError::Data)?,
            ),
            CalendarAlgorithm::Coptic => AnyCalendar::Coptic(Coptic),
            CalendarAlgorithm::Dangi => AnyCalendar::Dangi(
                Dangi::try_new_with_buffer_provider(self.0)
                    .map_err(DateTimeFormatterLoadError::Data)?,
            ),
            CalendarAlgorithm::Ethiopic => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            CalendarAlgorithm::Ethioaa => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
            CalendarAlgorithm::Gregory => AnyCalendar::Gregorian(Gregorian),
            CalendarAlgorithm::Hebrew => AnyCalendar::Hebrew(Hebrew),
            CalendarAlgorithm::Indian => AnyCalendar::Indian(Indian),
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil)) => {
                AnyCalendar::HijriCivil(HijriCivil)
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Rgsa)) => {
                AnyCalendar::HijriObservational(
                    HijriObservational::try_new_with_buffer_provider(self.0)
                        .map_err(DateTimeFormatterLoadError::Data)?,
                )
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                AnyCalendar::HijriTabular(HijriTabular)
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => {
                AnyCalendar::HijriUmmAlQura(
                    HijriUmmAlQura::try_new_with_buffer_provider(self.0)
                        .map_err(DateTimeFormatterLoadError::Data)?,
                )
            }
            CalendarAlgorithm::Japanese => AnyCalendar::Japanese(
                Japanese::try_new_with_buffer_provider(self.0)
                    .map_err(DateTimeFormatterLoadError::Data)?,
            ),
            CalendarAlgorithm::Persian => AnyCalendar::Persian(Persian),
            CalendarAlgorithm::Roc => AnyCalendar::Roc(Roc),
            c => return Err(DateTimeFormatterLoadError::UnsupportedCalendar(c)),
        })
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

impl<P> AnyCalendarLoader for ExternalLoaderUnstable<'_, P>
where
    P: DataProvider<icu_calendar::provider::CalendarJapaneseModernV1>
        + DataProvider<icu_calendar::provider::CalendarJapaneseExtendedV1>
        + DataProvider<icu_calendar::provider::CalendarChineseV1>
        + DataProvider<icu_calendar::provider::CalendarDangiV1>
        + DataProvider<icu_calendar::provider::CalendarHijriObservationalV1>
        + DataProvider<icu_calendar::provider::CalendarHijriUmmalquraV1>
        + ?Sized,
{
    #[inline]
    fn load(&self, kind: CalendarAlgorithm) -> Result<AnyCalendar, DateTimeFormatterLoadError> {
        use icu_calendar::cal::*;
        Ok(match kind {
            CalendarAlgorithm::Buddhist => AnyCalendar::Buddhist(Buddhist),
            CalendarAlgorithm::Chinese => AnyCalendar::Chinese(
                Chinese::try_new_unstable(self.0).map_err(DateTimeFormatterLoadError::Data)?,
            ),
            CalendarAlgorithm::Coptic => AnyCalendar::Coptic(Coptic),
            CalendarAlgorithm::Dangi => AnyCalendar::Dangi(
                Dangi::try_new_unstable(self.0).map_err(DateTimeFormatterLoadError::Data)?,
            ),
            CalendarAlgorithm::Ethiopic => AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(
                EthiopianEraStyle::AmeteMihret,
            )),
            CalendarAlgorithm::Ethioaa => {
                AnyCalendar::Ethiopian(Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem))
            }
            CalendarAlgorithm::Gregory => AnyCalendar::Gregorian(Gregorian),
            CalendarAlgorithm::Hebrew => AnyCalendar::Hebrew(Hebrew),
            CalendarAlgorithm::Indian => AnyCalendar::Indian(Indian),
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil)) => {
                AnyCalendar::HijriCivil(HijriCivil)
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Rgsa)) => {
                AnyCalendar::HijriObservational(
                    HijriObservational::try_new_unstable(self.0)
                        .map_err(DateTimeFormatterLoadError::Data)?,
                )
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                AnyCalendar::HijriTabular(HijriTabular)
            }
            CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => {
                AnyCalendar::HijriUmmAlQura(
                    HijriUmmAlQura::try_new_unstable(self.0)
                        .map_err(DateTimeFormatterLoadError::Data)?,
                )
            }
            CalendarAlgorithm::Japanese => {
                AnyCalendar::Japanese(Japanese::try_new_unstable(self.0)?)
            }
            CalendarAlgorithm::Persian => AnyCalendar::Persian(Persian),
            CalendarAlgorithm::Roc => AnyCalendar::Roc(Roc),
            c => return Err(DateTimeFormatterLoadError::UnsupportedCalendar(c)),
        })
    }
}
