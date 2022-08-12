// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DateTimeFormatterError;
use crate::provider::calendar::*;
use alloc::string::ToString;
use core::any;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, ethiopic::Ethiopic, indian::Indian, japanese::Japanese,
    japanese::JapaneseExtended, Gregorian,
};
use icu_locid::extensions::unicode::Value;
use icu_locid::extensions_unicode_key as key;
use icu_locid::extensions_unicode_value as value;
use icu_provider::prelude::*;
use tinystr::{tinystr, TinyStr16};

/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar
    /// If multiple BCP 47 identifiers work, this should be
    /// the default one when no others are provided
    const DEFAULT_BCP_47_IDENTIFIER: Value;

    /// The data marker for loading symbols for this calendar.
    type DateSymbolsV1Marker: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>> + 'static;

    /// The data marker for loading length-patterns for this calendar.
    type DateLengthsV1Marker: KeyedDataMarker<Yokeable = DateLengthsV1<'static>> + 'static;

    /// Checks if a given BCP 47 identifier is allowed to be used with this calendar
    ///
    /// By default, just checks against DEFAULT_BCP_47_IDENTIFIER
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == Self::DEFAULT_BCP_47_IDENTIFIER
    }
}

impl CldrCalendar for Gregorian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("gregory");
    type DateSymbolsV1Marker = GregorianDateSymbolsV1Marker;
    type DateLengthsV1Marker = GregorianDateLengthsV1Marker;
}

impl CldrCalendar for Buddhist {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("buddhist");
    type DateSymbolsV1Marker = BuddhistDateSymbolsV1Marker;
    type DateLengthsV1Marker = BuddhistDateLengthsV1Marker;
}

impl CldrCalendar for Japanese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanese");
    type DateSymbolsV1Marker = JapaneseDateSymbolsV1Marker;
    type DateLengthsV1Marker = JapaneseDateLengthsV1Marker;
}

impl CldrCalendar for JapaneseExtended {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanext");
    type DateSymbolsV1Marker = JapaneseExtendedDateSymbolsV1Marker;
    type DateLengthsV1Marker = JapaneseExtendedDateLengthsV1Marker;
}

impl CldrCalendar for Coptic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("coptic");
    type DateSymbolsV1Marker = CopticDateSymbolsV1Marker;
    type DateLengthsV1Marker = CopticDateLengthsV1Marker;
}

impl CldrCalendar for Indian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("indian");
    type DateSymbolsV1Marker = IndianDateSymbolsV1Marker;
    type DateLengthsV1Marker = IndianDateLengthsV1Marker;
}

impl CldrCalendar for Ethiopic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("ethiopic");
    type DateSymbolsV1Marker = EthiopicDateSymbolsV1Marker;
    type DateLengthsV1Marker = EthiopicDateLengthsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("ethiopic") || *value == value!("ethioaa")
    }
}

pub(crate) fn potentially_fixup_calendar<C: CldrCalendar>(
    locale: &mut DataLocale,
) -> Result<(), DateTimeFormatterError> {
    let cal = locale.get_unicode_ext(&key!("ca"));

    if let Some(cal) = cal {
        if !C::is_identifier_allowed_for_calendar(&cal) {
            let mut string = cal.to_string();
            string.truncate(16);
            let tiny = TinyStr16::from_str(&string).unwrap_or(tinystr!(16, "unknown"));
            return Err(DateTimeFormatterError::MismatchedCalendarLocale(
                any::type_name::<C>(),
                tiny,
            ));
        }
    } else {
        locale.set_unicode_ext(key!("ca"), C::DEFAULT_BCP_47_IDENTIFIER);
    }

    if locale.get_unicode_ext(&key!("ca")) == Some(value!("ethioaa")) {
        locale.set_unicode_ext(key!("ca"), value!("ethiopic"));
    }

    Ok(())
}

pub(crate) fn load_lengths_for_cldr_calendar<C, P>(
    provider: &P,
    locale: &DataLocale,
) -> Result<DataPayload<ErasedDateLengthsV1Marker>, DataError>
where
    C: CldrCalendar,
    P: DataProvider<<C as CldrCalendar>::DateLengthsV1Marker> + ?Sized,
{
    let payload = provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(payload.cast())
}

pub(crate) fn load_symbols_for_cldr_calendar<C, P>(
    provider: &P,
    locale: &DataLocale,
) -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>
where
    C: CldrCalendar,
    P: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker> + ?Sized,
{
    let payload = provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(payload.cast())
}

pub(crate) fn load_lengths_for_any_calendar_kind<P>(
    provider: &P,
    locale: &DataLocale,
    kind: AnyCalendarKind,
) -> Result<DataPayload<ErasedDateLengthsV1Marker>, DataError>
where
    P: DataProvider<GregorianDateLengthsV1Marker>
        + DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker>
        + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker>
        + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<EthiopicDateLengthsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopic => {
            DataProvider::<<Ethiopic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethioaa => {
            DataProvider::<<Ethiopic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        _ => {
            return Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&kind),
            )
        }
    };
    Ok(payload)
}

pub(crate) fn load_symbols_for_any_calendar_kind<P>(
    provider: &P,
    locale: &DataLocale,
    kind: AnyCalendarKind,
) -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>
where
    P: DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<BuddhistDateSymbolsV1Marker>
        + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
        + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker>
        + DataProvider<EthiopicDateSymbolsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopic => {
            DataProvider::<<Ethiopic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethioaa => {
            DataProvider::<<Ethiopic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        _ => {
            return Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&kind),
            )
        }
    };
    Ok(payload)
}
