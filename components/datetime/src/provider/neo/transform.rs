// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // so far only used in tests

use crate::provider::calendar::*;
use crate::provider::neo::*;

pub enum Length {
    Abbreviated,
    Narrow,
    Short,
    Wide,
}

impl<'a> DateSymbolsV1<'a> {
    pub(crate) fn neo_months_abbreviated(&self) -> MonthSymbolsV1<'a> {
        (&self.months.format.abbreviated).into()
    }
    pub(crate) fn neo_months_narrow(&self) -> MonthSymbolsV1<'a> {
        (&self.months.format.narrow).into()
    }
    pub(crate) fn neo_months_short(&self) -> Option<MonthSymbolsV1<'a>> {
        self.months.format.short.as_ref().map(Into::into)
    }
    pub(crate) fn neo_months_wide(&self) -> MonthSymbolsV1<'a> {
        (&self.months.format.wide).into()
    }

    pub(crate) fn neo_months_standalone_abbreviated(&self) -> Option<MonthSymbolsV1<'a>> {
        self.months
            .stand_alone
            .as_ref()
            .and_then(|x| x.abbreviated.as_ref())
            .map(Into::into)
    }
    pub(crate) fn neo_months_standalone_narrow(&self) -> Option<MonthSymbolsV1<'a>> {
        self.months
            .stand_alone
            .as_ref()
            .and_then(|x| x.narrow.as_ref())
            .map(Into::into)
    }
    pub(crate) fn neo_months_standalone_short(&self) -> Option<MonthSymbolsV1<'a>> {
        self.months
            .stand_alone
            .as_ref()
            .and_then(|x| x.short.as_ref())
            .map(Into::into)
    }
    pub(crate) fn neo_months_standalone_wide(&self) -> Option<MonthSymbolsV1<'a>> {
        self.months
            .stand_alone
            .as_ref()
            .and_then(|x| x.wide.as_ref())
            .map(Into::into)
    }
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
                MonthSymbolsV1::Map(zero_map.clone().try_convert_zv_k().unwrap())
            }
        }
    }
}

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
        let neo_month_abbreviated: DataPayload<GregorianMonthSymbolsV1Marker> =
            symbols.map_project(|symbols, _| symbols.neo_months_abbreviated());

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
        let neo_month_abbreviated: DataPayload<HebrewMonthSymbolsV1Marker> =
            symbols.map_project(|symbols, _| symbols.neo_months_abbreviated());

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "Map(ZeroMap { keys: ZeroVec([\"M01\", \"M02\", \"M03\", \"M04\", \"M05\", \"M05L\", \"M06\", \"M06L\", \"M07\", \"M08\", \"M09\", \"M10\", \"M11\", \"M12\"]), values: [\"Tishri\", \"Heshvan\", \"Kislev\", \"Tevet\", \"Shevat\", \"Adar I\", \"Adar\", \"Adar II\", \"Nisan\", \"Iyar\", \"Sivan\", \"Tamuz\", \"Av\", \"Elul\"] })"
        );
    }
}
