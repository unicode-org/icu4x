// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO(#5613): Even though these markers are no longer exported, we need them in order to export
//! semantic skeleton data markers. This should be refactored to skip the intermediate data struct.

use icu::datetime::provider::calendar::*;
use icu::datetime::provider::neo::*;
use icu_provider::prelude::*;

icu_provider::data_marker!(
    /// `BuddhistDateLengthsV1`
    BuddhistDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `ChineseDateLengthsV1`
    ChineseDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `CopticDateLengthsV1`
    CopticDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `DangiDateLengthsV1`
    DangiDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `EthiopianDateLengthsV1`
    EthiopianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `GregorianDateLengthsV1`
    GregorianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `HebrewDateLengthsV1`
    HebrewDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `IndianDateLengthsV1`
    IndianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `HijriDateLengthsV1`
    HijriDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `JapaneseDateLengthsV1`
    JapaneseDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `JapaneseExtendedDateLengthsV1`
    JapaneseExtendedDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `PersianDateLengthsV1`
    PersianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `RocDateLengthsV1`
    RocDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `TimeLengthsV1`
    TimeLengthsV1,
    TimeLengths<'static>
);
icu_provider::data_marker!(
    /// `BuddhistDateSymbolsV1`
    BuddhistDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `ChineseDateSymbolsV1`
    ChineseDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `CopticDateSymbolsV1`
    CopticDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `DangiDateSymbolsV1`
    DangiDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `EthiopianDateSymbolsV1`
    EthiopianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `GregorianDateSymbolsV1`
    GregorianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `HebrewDateSymbolsV1`
    HebrewDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `IndianDateSymbolsV1`
    IndianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `HijriDateSymbolsV1`
    HijriDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `JapaneseDateSymbolsV1`
    JapaneseDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `JapaneseExtendedDateSymbolsV1`
    JapaneseExtendedDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `PersianDateSymbolsV1`
    PersianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `RocDateSymbolsV1`
    RocDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `TimeSymbolsV1`
    TimeSymbolsV1,
    TimeSymbols<'static>
);

mod key_attr_consts {
    use super::*;

    pub const STADLN_ABBR: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("3s");
    pub const STADLN_WIDE: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("4s");
    pub const STADLN_NARW: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("5s");
    pub const STADLN_SHRT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("6s");
    pub const FORMAT_ABBR: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("3");
    pub const FORMAT_WIDE: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("4");
    pub const FORMAT_NARW: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("5");
    pub const FORMAT_SHRT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("6");

    /// Used for matching
    pub const STADLN_ABBR_STR: &str = STADLN_ABBR.as_str();
    pub const STADLN_WIDE_STR: &str = STADLN_WIDE.as_str();
    pub const STADLN_NARW_STR: &str = STADLN_NARW.as_str();
    pub const STADLN_SHRT_STR: &str = STADLN_SHRT.as_str();
    pub const FORMAT_ABBR_STR: &str = FORMAT_ABBR.as_str();
    pub const FORMAT_WIDE_STR: &str = FORMAT_WIDE.as_str();
    pub const FORMAT_NARW_STR: &str = FORMAT_NARW.as_str();
    pub const FORMAT_SHRT_STR: &str = FORMAT_SHRT.as_str();
}

fn month_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<DataStruct = DateSymbols<'static>>,
    P: DataMarker<DataStruct = MonthNames<'static>>,
{
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match req.id.marker_attributes.as_str() {
            STADLN_ABBR_STR => payload.months.stand_alone_abbreviated(),
            STADLN_WIDE_STR => payload.months.stand_alone_wide(),
            STADLN_NARW_STR => payload.months.stand_alone_narrow(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match req.id.marker_attributes.as_str() {
            STADLN_ABBR_STR | FORMAT_ABBR_STR => &payload.months.format.abbreviated,
            STADLN_WIDE_STR | FORMAT_WIDE_STR => &payload.months.format.wide,
            STADLN_NARW_STR | FORMAT_NARW_STR => &payload.months.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(req.id.marker_attributes.as_str()))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: new_payload,
        metadata: Default::default(),
    })
}

fn weekday_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<DataStruct = DateSymbols<'static>>,
    P: DataMarker<DataStruct = LinearNames<'static>>,
{
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match req.id.marker_attributes.as_str() {
            STADLN_ABBR_STR => payload.weekdays.stand_alone_abbreviated(),
            STADLN_WIDE_STR => payload.weekdays.stand_alone_wide(),
            STADLN_NARW_STR => payload.weekdays.stand_alone_narrow(),
            STADLN_SHRT_STR => payload.weekdays.stand_alone_short(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match req.id.marker_attributes.as_str() {
            STADLN_SHRT_STR | FORMAT_SHRT_STR => payload.weekdays.format.short.as_ref(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match req.id.marker_attributes.as_str() {
            STADLN_ABBR_STR | FORMAT_ABBR_STR | STADLN_SHRT_STR | FORMAT_SHRT_STR => {
                &payload.weekdays.format.abbreviated
            }
            STADLN_WIDE_STR | FORMAT_WIDE_STR => &payload.weekdays.format.wide,
            STADLN_NARW_STR | FORMAT_NARW_STR => &payload.weekdays.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(req.id.marker_attributes.as_str()))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: new_payload,
        metadata: Default::default(),
    })
}

fn dayperiod_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<DataStruct = TimeSymbols<'static>>,
    P: DataMarker<DataStruct = LinearNames<'static>>,
{
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match req.id.marker_attributes.as_str() {
            STADLN_ABBR_STR => payload.day_periods.stand_alone_abbreviated(),
            STADLN_WIDE_STR => payload.day_periods.stand_alone_wide(),
            STADLN_NARW_STR => payload.day_periods.stand_alone_narrow(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match req.id.marker_attributes.as_str() {
            STADLN_ABBR_STR | FORMAT_ABBR_STR => &payload.day_periods.format.abbreviated,
            STADLN_WIDE_STR | FORMAT_WIDE_STR => &payload.day_periods.format.wide,
            STADLN_NARW_STR | FORMAT_NARW_STR => &payload.day_periods.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(req.id.marker_attributes.as_str()))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: new_payload,
        metadata: Default::default(),
    })
}

macro_rules! impl_data_provider_adapter {
    ($old_ty:ty, $new_ty:ty, $cnv:ident) => {
        impl DataProvider<$new_ty> for DataPayload<$old_ty> {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$new_ty>, DataError> {
                $cnv(self, req)
            }
        }
    };
}

impl_data_provider_adapter!(
    BuddhistDateSymbolsV1,
    BuddhistMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1,
    ChineseMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1,
    CopticMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1,
    DangiMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1,
    EthiopianMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1,
    GregorianMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1,
    HebrewMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1,
    IndianMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HijriDateSymbolsV1,
    HijriMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1,
    JapaneseMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1,
    JapaneseExtendedMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1,
    PersianMonthNamesV1,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1,
    RocMonthNamesV1,
    month_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    BuddhistDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HijriDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1,
    WeekdayNamesV1,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    TimeSymbolsV1,
    DayPeriodNamesV1,
    dayperiod_symbols_map_project_cloned
);
