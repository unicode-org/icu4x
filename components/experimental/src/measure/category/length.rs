// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


use super::category;
use crate::measure::measureunit::MeasureUnit;
// use crate::measure::provider::si_prefix::{Base, SiPrefix};
// use crate::measure::provider::single_unit::SingleUnit;
// use smallvec::smallvec;
// use alloc::vec;


impl category::Length {
    pub fn meter() -> &'static MeasureUnit {
        // static METER: MeasureUnit = MeasureUnit {
        //     single_units: vec![SingleUnit {
        //         power: 1,
        //         si_prefix: SiPrefix { power: 0, base: Base::Decimal },
        //         unit_id: 0,
        //     }],
        //     constant_denominator: 0,
        // };
        // &METER

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_category() {
        let _meter = category::Length::meter();
    }
}
