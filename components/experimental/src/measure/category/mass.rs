// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::measure::measureunit::MeasureUnit;

use super::category;

impl category::Mass {
    pub fn kilogram() -> &'static MeasureUnit {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_category() {
        let _kilogram = category::Mass::kilogram();
    }
}
