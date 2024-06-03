// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::calendar::*;
use crate::provider::neo::*;
use alloc::vec;
use icu_calendar::types::MonthCode;
use icu_provider::prelude::*;

mod key_attr_consts {
    use tinystr::{tinystr, TinyAsciiStr};
    pub const STADLN_ABBR: TinyAsciiStr<8> = tinystr!(8, "3s");
    pub const STADLN_WIDE: TinyAsciiStr<8> = tinystr!(8, "4s");
    pub const STADLN_NARW: TinyAsciiStr<8> = tinystr!(8, "5s");
    pub const STADLN_SHRT: TinyAsciiStr<8> = tinystr!(8, "6s");
    pub const FORMAT_ABBR: TinyAsciiStr<8> = tinystr!(8, "3");
    pub const FORMAT_WIDE: TinyAsciiStr<8> = tinystr!(8, "4");
    pub const FORMAT_NARW: TinyAsciiStr<8> = tinystr!(8, "5");
    pub const FORMAT_SHRT: TinyAsciiStr<8> = tinystr!(8, "6");
}

fn month_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: DataMarker<Yokeable = MonthNamesV1<'static>>,
{
    let attribute = req
        .marker_attributes
        .single()
        .ok_or_else(|| DataError::custom("TODO"))?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match attribute {
            STADLN_ABBR => payload.months.stand_alone_abbreviated(),
            STADLN_WIDE => payload.months.stand_alone_wide(),
            STADLN_NARW => payload.months.stand_alone_narrow(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match attribute {
            STADLN_ABBR | FORMAT_ABBR => &payload.months.format.abbreviated,
            STADLN_WIDE | FORMAT_WIDE => &payload.months.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.months.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(&attribute))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: Some(new_payload),
        metadata: Default::default(),
    })
}

fn weekday_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: DataMarker<Yokeable = LinearNamesV1<'static>>,
{
    let attribute = req
        .marker_attributes
        .single()
        .ok_or_else(|| DataError::custom("TODO"))?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match attribute {
            STADLN_ABBR => payload.weekdays.stand_alone_abbreviated(),
            STADLN_WIDE => payload.weekdays.stand_alone_wide(),
            STADLN_NARW => payload.weekdays.stand_alone_narrow(),
            STADLN_SHRT => payload.weekdays.stand_alone_short(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match attribute {
            STADLN_SHRT | FORMAT_SHRT => payload.weekdays.format.short.as_ref(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match attribute {
            STADLN_ABBR | FORMAT_ABBR | STADLN_SHRT | FORMAT_SHRT => {
                &payload.weekdays.format.abbreviated
            }
            STADLN_WIDE | FORMAT_WIDE => &payload.weekdays.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.weekdays.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(&attribute))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: Some(new_payload),
        metadata: Default::default(),
    })
}

fn era_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: DataMarker<Yokeable = YearNamesV1<'static>>,
{
    let attribute = req
        .marker_attributes
        .single()
        .ok_or_else(|| DataError::custom("TODO"))?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match attribute {
            FORMAT_ABBR => &payload.eras.abbr,
            FORMAT_WIDE => &payload.eras.names,
            FORMAT_NARW => &payload.eras.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(&attribute))
            }
        };
        Ok(YearNamesV1::Eras(result.clone()))
    })?;
    Ok(DataResponse {
        payload: Some(new_payload),
        metadata: Default::default(),
    })
}

fn dayperiod_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: DataMarker<Yokeable = TimeSymbolsV1<'static>>,
    P: DataMarker<Yokeable = LinearNamesV1<'static>>,
{
    let attribute = req
        .marker_attributes
        .single()
        .ok_or_else(|| DataError::custom("TODO"))?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use key_attr_consts::*;
        let result = match attribute {
            STADLN_ABBR => payload.day_periods.stand_alone_abbreviated(),
            STADLN_WIDE => payload.day_periods.stand_alone_wide(),
            STADLN_NARW => payload.day_periods.stand_alone_narrow(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match attribute {
            STADLN_ABBR | FORMAT_ABBR => &payload.day_periods.format.abbreviated,
            STADLN_WIDE | FORMAT_WIDE => &payload.day_periods.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.day_periods.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown marker attribute")
                    .with_marker(M::INFO)
                    .with_display_context(&attribute))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: Some(new_payload),
        metadata: Default::default(),
    })
}

impl<'a> From<&months::SymbolsV1<'a>> for MonthNamesV1<'a> {
    fn from(other: &months::SymbolsV1<'a>) -> Self {
        match other {
            months::SymbolsV1::SolarTwelve(cow_list) => {
                // Can't zero-copy convert a cow list to a VarZeroVec, so we need to allocate
                // a new VarZeroVec. Since VarZeroVec does not implement `from_iter`, first we
                // make a Vec of string references.
                let vec: alloc::vec::Vec<&str> = cow_list.iter().map(|x| &**x).collect();
                MonthNamesV1::Linear((&vec).into())
            }
            months::SymbolsV1::Other(zero_map) => {
                // Only calendar that uses this is hebrew, we can assume it is 12-month
                let mut vec = vec![""; 24];

                for (k, v) in zero_map.iter() {
                    let Some((number, leap)) = MonthCode(*k).parsed() else {
                        debug_assert!(false, "Found unknown month code {k}");
                        continue;
                    };
                    let offset = if leap { 12 } else { 0 };
                    if let Some(entry) = vec.get_mut((number + offset - 1) as usize) {
                        *entry = v;
                    } else {
                        debug_assert!(false, "Found out of bounds hebrew month code {k}")
                    }
                }
                MonthNamesV1::LeapLinear((&vec).into())
            }
        }
    }
}

impl<'a> From<&weekdays::SymbolsV1<'a>> for LinearNamesV1<'a> {
    fn from(other: &weekdays::SymbolsV1<'a>) -> Self {
        // Input is a cow array of length 7. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = other.0.iter().map(|x| &**x).collect();
        LinearNamesV1 {
            symbols: (&vec).into(),
        }
    }
}

impl<'a> From<&day_periods::SymbolsV1<'a>> for LinearNamesV1<'a> {
    fn from(other: &day_periods::SymbolsV1<'a>) -> Self {
        // Input is a struct with four fields. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = match (other.noon.as_ref(), other.midnight.as_ref()) {
            (Some(noon), Some(midnight)) => vec![&other.am, &other.pm, &noon, &midnight],
            (Some(noon), None) => vec![&other.am, &other.pm, &noon],
            (None, Some(midnight)) => vec![&other.am, &other.pm, "", &midnight],
            (None, None) => vec![&other.am, &other.pm],
        };
        LinearNamesV1 {
            symbols: (&vec).into(),
        }
    }
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
    BuddhistDateSymbolsV1Marker,
    BuddhistMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1Marker,
    ChineseMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1Marker,
    CopticMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1Marker,
    DangiMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1Marker,
    EthiopianMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1Marker,
    GregorianMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    HebrewMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1Marker,
    IndianMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IslamicDateSymbolsV1Marker,
    IslamicMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1Marker,
    JapaneseMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1Marker,
    JapaneseExtendedMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1Marker,
    PersianMonthNamesV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1Marker,
    RocMonthNamesV1Marker,
    month_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    BuddhistDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IslamicDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1Marker,
    WeekdayNamesV1Marker,
    weekday_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    BuddhistDateSymbolsV1Marker,
    BuddhistYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1Marker,
    ChineseYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1Marker,
    CopticYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1Marker,
    DangiYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1Marker,
    EthiopianYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1Marker,
    GregorianYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    HebrewYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1Marker,
    IndianYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IslamicDateSymbolsV1Marker,
    IslamicYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1Marker,
    JapaneseYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1Marker,
    JapaneseExtendedYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1Marker,
    PersianYearNamesV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1Marker,
    RocYearNamesV1Marker,
    era_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    TimeSymbolsV1Marker,
    DayPeriodNamesV1Marker,
    dayperiod_symbols_map_project_cloned
);

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use icu_locale_core::langid;

    #[test]
    fn test_adapter_months_numeric() {
        let symbols: DataPayload<GregorianDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_month_abbreviated: DataPayload<GregorianMonthNamesV1Marker> = symbols
            .load(DataRequest {
                locale: &"en".parse().unwrap(),
                marker_attributes: &"3".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "Linear([\"Jan\", \"Feb\", \"Mar\", \"Apr\", \"May\", \"Jun\", \"Jul\", \"Aug\", \"Sep\", \"Oct\", \"Nov\", \"Dec\"])"
        );
    }

    #[test]
    fn test_adapter_months_map() {
        let symbols: DataPayload<HebrewDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_month_abbreviated: DataPayload<HebrewMonthNamesV1Marker> = symbols
            .load(DataRequest {
                locale: &"en".parse().unwrap(),
                marker_attributes: &"3".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "LeapLinear([\"Tishri\", \"Heshvan\", \"Kislev\", \"Tevet\", \"Shevat\", \"Adar\", \"Nisan\", \"Iyar\", \"Sivan\", \"Tamuz\", \"Av\", \"Elul\", \"\", \"\", \"\", \"\", \"Adar I\", \"Adar II\", \"\", \"\", \"\", \"\", \"\", \"\"])"
        );
    }

    #[test]
    fn test_adapter_weekdays_abbreviated() {
        let symbols: DataPayload<HebrewDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_weekdays_abbreviated: DataPayload<WeekdayNamesV1Marker> = symbols
            .load(DataRequest {
                locale: &"en".parse().unwrap(),
                marker_attributes: &"3".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_weekdays_abbreviated:?}"),
            "LinearNamesV1 { symbols: [\"Sun\", \"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\", \"Sat\"] }"
        );
    }

    #[test]
    fn test_adapter_weekdays_short() {
        let symbols: DataPayload<HebrewDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_weekdays_short: DataPayload<WeekdayNamesV1Marker> = symbols
            .load(DataRequest {
                locale: &"en".parse().unwrap(),
                marker_attributes: &"6s".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_weekdays_short:?}"),
            "LinearNamesV1 { symbols: [\"Su\", \"Mo\", \"Tu\", \"We\", \"Th\", \"Fr\", \"Sa\"] }"
        );
    }

    #[test]
    fn test_adapter_eras() {
        let symbols: DataPayload<GregorianDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_eras_wide: DataPayload<GregorianYearNamesV1Marker> = symbols
            .load(DataRequest {
                locale: &"en".parse().unwrap(),
                marker_attributes: &"4".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_eras_wide:?}"),
            "Eras(ZeroMap { keys: [\"bce\", \"ce\"], values: [\"Before Christ\", \"Anno Domini\"] })"
        );
    }

    #[test]
    fn test_adapter_dayperiods() {
        let symbols: DataPayload<TimeSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_dayperiods_abbreviated: DataPayload<DayPeriodNamesV1Marker> = symbols
            .load(DataRequest {
                locale: &"en".parse().unwrap(),
                marker_attributes: &"3s".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_dayperiods_abbreviated:?}"),
            "LinearNamesV1 { symbols: [\"AM\", \"PM\", \"noon\", \"midnight\"] }"
        );
    }
}
