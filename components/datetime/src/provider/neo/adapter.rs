// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::calendar::*;
use crate::provider::neo::*;
use alloc::vec;
use icu_calendar::types::MonthCode;
use icu_locid::extensions::private::{subtag, Subtag};
use icu_provider::prelude::*;

mod subtag_consts {
    use super::*;
    pub const STADLN_ABBR: Subtag = subtag!("3s");
    pub const STADLN_WIDE: Subtag = subtag!("4s");
    pub const STADLN_NARW: Subtag = subtag!("5s");
    pub const STADLN_SHRT: Subtag = subtag!("6s");
    pub const FORMAT_ABBR: Subtag = subtag!("3");
    pub const FORMAT_WIDE: Subtag = subtag!("4");
    pub const FORMAT_NARW: Subtag = subtag!("5");
    pub const FORMAT_SHRT: Subtag = subtag!("6");
}

fn single_aux_subtag<M: KeyedDataMarker>(locale: &DataLocale) -> Result<Subtag, DataError> {
    let Some(aux) = locale.get_aux() else {
        return Err(DataError::custom("Expected a single aux key")
            .with_key(M::KEY)
            .with_debug_context(locale));
    };
    let mut iter = aux.iter();
    let Some(subtag) = iter.next() else {
        return Err(DataError::custom("Expected a single aux key")
            .with_key(M::KEY)
            .with_debug_context(locale));
    };
    if iter.next().is_some() {
        return Err(DataError::custom("Expected a single aux key")
            .with_key(M::KEY)
            .with_debug_context(locale));
    }
    Ok(subtag)
}

fn month_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: KeyedDataMarker<Yokeable = MonthSymbolsV1<'static>>,
{
    let subtag = single_aux_subtag::<M>(req.locale)?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use subtag_consts::*;
        let result = match subtag {
            STADLN_ABBR => payload.months.stand_alone_abbreviated(),
            STADLN_WIDE => payload.months.stand_alone_wide(),
            STADLN_NARW => payload.months.stand_alone_narrow(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match subtag {
            STADLN_ABBR | FORMAT_ABBR => &payload.months.format.abbreviated,
            STADLN_WIDE | FORMAT_WIDE => &payload.months.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.months.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown aux key")
                    .with_key(M::KEY)
                    .with_display_context(&subtag))
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
    M: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: KeyedDataMarker<Yokeable = LinearSymbolsV1<'static>>,
{
    let subtag = single_aux_subtag::<M>(req.locale)?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use subtag_consts::*;
        let result = match subtag {
            STADLN_ABBR => payload.weekdays.stand_alone_abbreviated(),
            STADLN_WIDE => payload.weekdays.stand_alone_wide(),
            STADLN_NARW => payload.weekdays.stand_alone_narrow(),
            STADLN_SHRT => payload.weekdays.stand_alone_short(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match subtag {
            STADLN_SHRT | FORMAT_SHRT => payload.weekdays.format.short.as_ref(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match subtag {
            STADLN_ABBR | FORMAT_ABBR | STADLN_SHRT | FORMAT_SHRT => {
                &payload.weekdays.format.abbreviated
            }
            STADLN_WIDE | FORMAT_WIDE => &payload.weekdays.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.weekdays.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown aux key")
                    .with_key(M::KEY)
                    .with_display_context(&subtag))
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
    M: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: KeyedDataMarker<Yokeable = YearSymbolsV1<'static>>,
{
    let subtag = single_aux_subtag::<M>(req.locale)?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use subtag_consts::*;
        let result = match subtag {
            FORMAT_ABBR => &payload.eras.abbr,
            FORMAT_WIDE => &payload.eras.names,
            FORMAT_NARW => &payload.eras.narrow,
            _ => {
                return Err(DataError::custom("Unknown aux key")
                    .with_key(M::KEY)
                    .with_display_context(&subtag))
            }
        };
        Ok(YearSymbolsV1::Eras(result.clone()))
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
    M: KeyedDataMarker<Yokeable = TimeSymbolsV1<'static>>,
    P: KeyedDataMarker<Yokeable = LinearSymbolsV1<'static>>,
{
    let subtag = single_aux_subtag::<M>(req.locale)?;
    let new_payload = payload.try_map_project_cloned(|payload, _| {
        use subtag_consts::*;
        let result = match subtag {
            STADLN_ABBR => payload.day_periods.stand_alone_abbreviated(),
            STADLN_WIDE => payload.day_periods.stand_alone_wide(),
            STADLN_NARW => payload.day_periods.stand_alone_narrow(),
            _ => None,
        };
        if let Some(result) = result {
            return Ok(result.into());
        }
        let result = match subtag {
            STADLN_ABBR | FORMAT_ABBR => &payload.day_periods.format.abbreviated,
            STADLN_WIDE | FORMAT_WIDE => &payload.day_periods.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.day_periods.format.narrow,
            _ => {
                return Err(DataError::custom("Unknown aux key")
                    .with_key(M::KEY)
                    .with_display_context(&subtag))
            }
        };
        Ok(result.into())
    })?;
    Ok(DataResponse {
        payload: Some(new_payload),
        metadata: Default::default(),
    })
}

impl<'a> From<&months::SymbolsV1<'a>> for MonthSymbolsV1<'a> {
    fn from(other: &months::SymbolsV1<'a>) -> Self {
        match other {
            months::SymbolsV1::SolarTwelve(cow_list) => {
                // Can't zero-copy convert a cow list to a VarZeroVec, so we need to allocate
                // a new VarZeroVec. Since VarZeroVec does not implement `from_iter`, first we
                // make a Vec of string references.
                let vec: alloc::vec::Vec<&str> = cow_list.iter().map(|x| &**x).collect();
                MonthSymbolsV1::Linear((&vec).into())
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
                MonthSymbolsV1::LeapLinear((&vec).into())
            }
        }
    }
}

impl<'a> From<&weekdays::SymbolsV1<'a>> for LinearSymbolsV1<'a> {
    fn from(other: &weekdays::SymbolsV1<'a>) -> Self {
        // Input is a cow array of length 7. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = other.0.iter().map(|x| &**x).collect();
        LinearSymbolsV1 {
            symbols: (&vec).into(),
        }
    }
}

impl<'a> From<&day_periods::SymbolsV1<'a>> for LinearSymbolsV1<'a> {
    fn from(other: &day_periods::SymbolsV1<'a>) -> Self {
        // Input is a struct with four fields. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = match (other.noon.as_ref(), other.midnight.as_ref()) {
            (Some(noon), Some(midnight)) => vec![&other.am, &other.pm, &noon, &midnight],
            (Some(noon), None) => vec![&other.am, &other.pm, &noon],
            (None, Some(midnight)) => vec![&other.am, &other.pm, "", &midnight],
            (None, None) => vec![&other.am, &other.pm],
        };
        LinearSymbolsV1 {
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
    BuddhistMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1Marker,
    ChineseMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1Marker,
    CopticMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1Marker,
    DangiMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1Marker,
    EthiopianMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1Marker,
    GregorianMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    HebrewMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1Marker,
    IndianMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IslamicDateSymbolsV1Marker,
    IslamicMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1Marker,
    JapaneseMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1Marker,
    JapaneseExtendedMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1Marker,
    PersianMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1Marker,
    RocMonthSymbolsV1Marker,
    month_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    BuddhistDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IslamicDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    BuddhistDateSymbolsV1Marker,
    BuddhistYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    ChineseDateSymbolsV1Marker,
    ChineseYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    CopticDateSymbolsV1Marker,
    CopticYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    DangiDateSymbolsV1Marker,
    DangiYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    EthiopianDateSymbolsV1Marker,
    EthiopianYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    GregorianDateSymbolsV1Marker,
    GregorianYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    HebrewYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IndianDateSymbolsV1Marker,
    IndianYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    IslamicDateSymbolsV1Marker,
    IslamicYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseDateSymbolsV1Marker,
    JapaneseYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    JapaneseExtendedDateSymbolsV1Marker,
    JapaneseExtendedYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    PersianDateSymbolsV1Marker,
    PersianYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    RocDateSymbolsV1Marker,
    RocYearSymbolsV1Marker,
    era_symbols_map_project_cloned
);

impl_data_provider_adapter!(
    TimeSymbolsV1Marker,
    DayPeriodSymbolsV1Marker,
    dayperiod_symbols_map_project_cloned
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;

    #[test]
    fn test_adapter_months_numeric() {
        let symbols: DataPayload<GregorianDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_month_abbreviated: DataPayload<GregorianMonthSymbolsV1Marker> = symbols
            .load(DataRequest {
                locale: &"en-x-3".parse().unwrap(),
                metadata: Default::default(),
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
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_month_abbreviated: DataPayload<HebrewMonthSymbolsV1Marker> = symbols
            .load(DataRequest {
                locale: &"en-x-3".parse().unwrap(),
                metadata: Default::default(),
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
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_weekdays_abbreviated: DataPayload<WeekdaySymbolsV1Marker> = symbols
            .load(DataRequest {
                locale: &"en-x-3".parse().unwrap(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_weekdays_abbreviated:?}"),
            "LinearSymbolsV1 { symbols: [\"Sun\", \"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\", \"Sat\"] }"
        );
    }

    #[test]
    fn test_adapter_weekdays_short() {
        let symbols: DataPayload<HebrewDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_weekdays_short: DataPayload<WeekdaySymbolsV1Marker> = symbols
            .load(DataRequest {
                locale: &"en-x-6s".parse().unwrap(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_weekdays_short:?}"),
            "LinearSymbolsV1 { symbols: [\"Su\", \"Mo\", \"Tu\", \"We\", \"Th\", \"Fr\", \"Sa\"] }"
        );
    }

    #[test]
    fn test_adapter_eras() {
        let symbols: DataPayload<GregorianDateSymbolsV1Marker> = crate::provider::Baked
            .load(DataRequest {
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_eras_wide: DataPayload<GregorianYearSymbolsV1Marker> = symbols
            .load(DataRequest {
                locale: &"en-x-4".parse().unwrap(),
                metadata: Default::default(),
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
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let neo_dayperiods_abbreviated: DataPayload<DayPeriodSymbolsV1Marker> = symbols
            .load(DataRequest {
                locale: &"en-x-3s".parse().unwrap(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_dayperiods_abbreviated:?}"),
            "LinearSymbolsV1 { symbols: [\"AM\", \"PM\", \"noon\", \"midnight\"] }"
        );
    }
}
