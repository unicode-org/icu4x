// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // so far only used in tests

use crate::provider::calendar::*;
use crate::provider::neo::*;
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

fn month_symbols_map_project_cloned<M, P>(
    payload: &DataPayload<M>,
    req: DataRequest,
) -> Result<DataResponse<P>, DataError>
where
    M: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>,
    P: KeyedDataMarker<Yokeable = MonthSymbolsV1<'static>>,
{
    let subtag = req
        .locale
        .get_aux()
        .and_then(|aux| aux.iter().next())
        .unwrap();
    let new_payload = payload.map_project_cloned(|payload, _| {
        use subtag_consts::*;
        let result = match subtag {
            STADLN_ABBR => payload
                .months
                .stand_alone
                .as_ref()
                .and_then(|x| x.abbreviated.as_ref()),
            STADLN_WIDE => payload
                .months
                .stand_alone
                .as_ref()
                .and_then(|x| x.wide.as_ref()),
            STADLN_NARW => payload
                .months
                .stand_alone
                .as_ref()
                .and_then(|x| x.narrow.as_ref()),
            _ => None,
        };
        if let Some(result) = result {
            return result.into();
        }
        let result = match subtag {
            STADLN_ABBR | FORMAT_ABBR => &payload.months.format.abbreviated,
            STADLN_WIDE | FORMAT_WIDE => &payload.months.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.months.format.narrow,
            _ => panic!("Unknown aux key subtag: {subtag}"),
        };
        return result.into();
    });
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
    let subtag = req
        .locale
        .get_aux()
        .and_then(|aux| aux.iter().next())
        .unwrap();
    let new_payload = payload.map_project_cloned(|payload, _| {
        use subtag_consts::*;
        let result = match subtag {
            STADLN_ABBR => payload
                .weekdays
                .stand_alone
                .as_ref()
                .and_then(|x| x.abbreviated.as_ref()),
            STADLN_WIDE => payload
                .weekdays
                .stand_alone
                .as_ref()
                .and_then(|x| x.wide.as_ref()),
            STADLN_NARW => payload
                .weekdays
                .stand_alone
                .as_ref()
                .and_then(|x| x.narrow.as_ref()),
            STADLN_SHRT => payload
                .weekdays
                .stand_alone
                .as_ref()
                .and_then(|x| x.short.as_ref()),
            _ => None,
        };
        if let Some(result) = result {
            return result.into();
        }
        let result = match subtag {
            STADLN_SHRT | FORMAT_SHRT => payload.weekdays.format.short.as_ref(),
            _ => None,
        };
        if let Some(result) = result {
            return result.into();
        }
        let result = match subtag {
            STADLN_ABBR | FORMAT_ABBR | STADLN_SHRT | FORMAT_SHRT => {
                &payload.weekdays.format.abbreviated
            }
            STADLN_WIDE | FORMAT_WIDE => &payload.weekdays.format.wide,
            STADLN_NARW | FORMAT_NARW => &payload.weekdays.format.narrow,
            _ => panic!("Unknown aux key subtag: {subtag}"),
        };
        return result.into();
    });
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
                MonthSymbolsV1::Numeric((&vec).into())
            }
            months::SymbolsV1::Other(zero_map) => {
                // zero_map has `MonthCode` keys, but we want `UnvalidatedTinyAsciiStr<4>`.
                // We can do this conversion zero-copy. clone() is no-op on a borrowed ZeroMap.
                #[allow(clippy::unwrap_used)] // MonthCode to Unvalidated4 is infallible
                MonthSymbolsV1::Map(zero_map.clone().try_convert_zv_k_unchecked().unwrap())
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
    GregorianDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);
impl_data_provider_adapter!(
    HebrewDateSymbolsV1Marker,
    WeekdaySymbolsV1Marker,
    weekday_symbols_map_project_cloned
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;

    #[test]
    fn test_transform_months_numeric() {
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
            "Numeric([\"Jan\", \"Feb\", \"Mar\", \"Apr\", \"May\", \"Jun\", \"Jul\", \"Aug\", \"Sep\", \"Oct\", \"Nov\", \"Dec\"])"
        );
    }

    #[test]
    fn test_transform_months_map() {
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
            "Map(ZeroMap { keys: ZeroVec([\"M01\", \"M02\", \"M03\", \"M04\", \"M05\", \"M05L\", \"M06\", \"M06L\", \"M07\", \"M08\", \"M09\", \"M10\", \"M11\", \"M12\"]), values: [\"Tishri\", \"Heshvan\", \"Kislev\", \"Tevet\", \"Shevat\", \"Adar I\", \"Adar\", \"Adar II\", \"Nisan\", \"Iyar\", \"Sivan\", \"Tamuz\", \"Av\", \"Elul\"] })"
        );
    }

    #[test]
    fn test_transform_weekdays_abbreviated() {
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
    fn test_transform_weekdays_short() {
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
                locale: &"en-x-6s".parse().unwrap(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            format!("{neo_weekdays_abbreviated:?}"),
            "LinearSymbolsV1 { symbols: [\"Su\", \"Mo\", \"Tu\", \"We\", \"Th\", \"Fr\", \"Sa\"] }"
        );
    }
}
