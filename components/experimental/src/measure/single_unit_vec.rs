// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::provider::single_unit::SingleUnit;

use alloc::vec::Vec;

// The SingleUnitVec enum is used to represent a collection of SingleUnit instances.
// It can represent zero, one, two, or multiple units, depending on the variant.
// The iter method provides an iterator over the contained SingleUnit instances.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum SingleUnitVec {
    Zero,
    One(SingleUnit),
    Two([SingleUnit; 2]),
    Multi(Vec<SingleUnit>),
}

impl SingleUnitVec {
    /// Returns a slice of the [`SingleUnit`] instances contained
    /// within the [`SingleUnitVec`].
    pub fn as_slice(&self) -> &[SingleUnit] {
        match self {
            SingleUnitVec::Zero => &[],
            SingleUnitVec::One(unit) => core::slice::from_ref(unit),
            SingleUnitVec::Two(units) => &units[..],
            SingleUnitVec::Multi(units) => &units[..],
        }
    }
}

impl FromIterator<SingleUnit> for SingleUnitVec {
    fn from_iter<I: IntoIterator<Item = SingleUnit>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        if let Some(unit1) = iter.next() {
            if let Some(unit2) = iter.next() {
                if let Some(unit3) = iter.next() {
                    let mut units = Vec::with_capacity(3);
                    units.push(unit1);
                    units.push(unit2);
                    units.push(unit3);
                    units.extend(iter);
                    return SingleUnitVec::Multi(units);
                }
                return SingleUnitVec::Two([unit1, unit2]);
            }
            return SingleUnitVec::One(unit1);
        }
        SingleUnitVec::Zero
    }
}
