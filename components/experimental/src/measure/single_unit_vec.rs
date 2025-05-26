// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::provider::single_unit::SingleUnit;

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec::Vec};

// The SingleUnitVec enum is used to represent a collection of SingleUnit instances.
// It can represent zero, one, two, or multiple units, depending on the variant.
// The iter method provides an iterator over the contained SingleUnit instances.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum SingleUnitVec {
    Zero,
    One(SingleUnit),
    Two(SingleUnit, SingleUnit),

    #[cfg(feature = "alloc")]
    Multi(Vec<SingleUnit>),
}

impl SingleUnitVec {
    #[cfg(feature = "alloc")]
    pub fn iter(&self) -> Box<dyn Iterator<Item = &SingleUnit> + '_> {
        match self {
            SingleUnitVec::Zero => Box::new(core::iter::empty()),
            SingleUnitVec::One(unit) => Box::new(core::iter::once(unit)),
            SingleUnitVec::Two(unit1, unit2) => {
                Box::new(core::iter::once(unit1).chain(core::iter::once(unit2)))
            }
            SingleUnitVec::Multi(units) => Box::new(units.iter()),
        }
    }
}
