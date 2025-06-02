// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "compiled_data")]
use crate::measure::{
    category::category,
    measureunit::MeasureUnit,
    provider::{
        si_prefix::{Base, SiPrefix},
        single_unit::SingleUnit,
    },
    single_unit_vec::SingleUnitVec,
};

impl category::Length {
    #[cfg(feature = "compiled_data")]
    /// Returns a [`MeasureUnit`] representing the length of one meter.
    pub fn meter() -> MeasureUnit {
        MeasureUnit {
            single_units: SingleUnitVec::One(SingleUnit {
                power: 1,
                si_prefix: SiPrefix {
                    power: 0,
                    base: Base::Decimal,
                },
                unit_id: crate::provider::Baked::UNIT_IDS_V1_UND_METER,
            }),
            constant_denominator: 0,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use crate::measure::parser::MeasureUnitParser;

    #[test]
    fn test_length_category() {
        let parser = MeasureUnitParser::default();
        let meter = category::Length::meter();
        let meter_parsed = parser.try_from_str("meter").unwrap();
        assert_eq!(meter, meter_parsed);
    }
}
