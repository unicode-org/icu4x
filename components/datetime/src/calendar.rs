// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::calendar::*;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::chinese::Chinese;
use icu_calendar::roc::Roc;
use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, dangi::Dangi, ethiopian::Ethiopian, hebrew::Hebrew,
    indian::Indian, islamic::IslamicCivil, islamic::IslamicObservational, islamic::IslamicTabular,
    islamic::IslamicUmmAlQura, japanese::Japanese, japanese::JapaneseExtended, persian::Persian,
    Gregorian,
};
use icu_locid::extensions::unicode::{value, Value};
use icu_provider::prelude::*;
use tinystr::{tinystr, TinyAsciiStr};
/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
pub trait CldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar's skeleton
    /// If multiple BCP 47 identifiers work, this should be
    /// the default one when no others are provided
    ///
    /// If `is_identifier_allowed_for_calendar()` is set, this only is used for loading skeletons data
    const DEFAULT_BCP_47_IDENTIFIER: Value;

    /// The data marker for loading symbols for this calendar.
    type DateSymbolsV1Marker: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>;

    /// The data marker for loading length-patterns for this calendar.
    type DateLengthsV1Marker: KeyedDataMarker<Yokeable = DateLengthsV1<'static>>;

    /// Checks if a given BCP 47 identifier is allowed to be used with this calendar
    ///
    /// By default, just checks against DEFAULT_BCP_47_IDENTIFIER
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == Self::DEFAULT_BCP_47_IDENTIFIER
    }
}

/// Check if the provided value is of the form `islamic-{subcal}`
fn is_islamic_subcal(value: &Value, subcal: TinyAsciiStr<8>) -> bool {
    let slice = value.as_tinystr_slice();
    if slice.len() > 2 {
        return false;
    }
    if let (Some(first), Some(second)) = (slice.get(0), slice.get(1)) {
        return *first == tinystr!(8, "islamic") && *second == subcal;
    }

    false
}
impl CldrCalendar for Buddhist {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("buddhist");
    type DateSymbolsV1Marker = BuddhistDateSymbolsV1Marker;
    type DateLengthsV1Marker = BuddhistDateLengthsV1Marker;
}

impl CldrCalendar for Chinese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("chinese");
    type DateSymbolsV1Marker = ChineseDateSymbolsV1Marker;
    type DateLengthsV1Marker = ChineseDateLengthsV1Marker;
}

impl CldrCalendar for Coptic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("coptic");
    type DateSymbolsV1Marker = CopticDateSymbolsV1Marker;
    type DateLengthsV1Marker = CopticDateLengthsV1Marker;
}

impl CldrCalendar for Dangi {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("dangi");
    type DateSymbolsV1Marker = DangiDateSymbolsV1Marker;
    type DateLengthsV1Marker = DangiDateLengthsV1Marker;
}

impl CldrCalendar for Ethiopian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("ethiopic");
    type DateSymbolsV1Marker = EthiopianDateSymbolsV1Marker;
    type DateLengthsV1Marker = EthiopianDateLengthsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("ethiopic") || *value == value!("ethioaa")
    }
}

impl CldrCalendar for Gregorian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("gregory");
    type DateSymbolsV1Marker = GregorianDateSymbolsV1Marker;
    type DateLengthsV1Marker = GregorianDateLengthsV1Marker;
}

impl CldrCalendar for Hebrew {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("hebrew");
    type DateSymbolsV1Marker = HebrewDateSymbolsV1Marker;
    type DateLengthsV1Marker = HebrewDateLengthsV1Marker;
}

impl CldrCalendar for Indian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("indian");
    type DateSymbolsV1Marker = IndianDateSymbolsV1Marker;
    type DateLengthsV1Marker = IndianDateLengthsV1Marker;
}

impl CldrCalendar for IslamicCivil {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("islamicc") || is_islamic_subcal(value, tinystr!(8, "civil"))
    }
}

impl CldrCalendar for IslamicObservational {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
}

impl CldrCalendar for IslamicTabular {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        is_islamic_subcal(value, tinystr!(8, "tbla"))
    }
}

impl CldrCalendar for IslamicUmmAlQura {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        is_islamic_subcal(value, tinystr!(8, "umalqura"))
    }
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

impl CldrCalendar for Persian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("persian");
    type DateSymbolsV1Marker = PersianDateSymbolsV1Marker;
    type DateLengthsV1Marker = PersianDateLengthsV1Marker;
}

impl CldrCalendar for Roc {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("roc");
    type DateSymbolsV1Marker = RocDateSymbolsV1Marker;
    type DateLengthsV1Marker = RocDateLengthsV1Marker;
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
    P: DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<ChineseDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker>
        + DataProvider<DangiDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker>
        + DataProvider<GregorianDateLengthsV1Marker>
        + DataProvider<HebrewDateLengthsV1Marker>
        + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<IslamicDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker>
        + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<PersianDateLengthsV1Marker>
        + DataProvider<RocDateLengthsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Dangi => {
            DataProvider::<<Dangi as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicUmmAlQura => DataProvider::<
            <IslamicUmmAlQura as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
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
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
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
    P: DataProvider<BuddhistDateSymbolsV1Marker>
        + DataProvider<ChineseDateSymbolsV1Marker>
        + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<DangiDateSymbolsV1Marker>
        + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<HebrewDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker>
        + DataProvider<IslamicDateSymbolsV1Marker>
        + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
        + DataProvider<PersianDateSymbolsV1Marker>
        + DataProvider<RocDateSymbolsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Dangi => {
            DataProvider::<<Dangi as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicUmmAlQura => DataProvider::<
            <IslamicUmmAlQura as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
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
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
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
