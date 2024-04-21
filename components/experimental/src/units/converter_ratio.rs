// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::units::ratio::IcuRatio;

// TODO: add Mul & Add for references to avoid cloning.
/// A trait for types that can be converted between two units.
pub trait ConverterRatio: Clone {
    fn reciprocal(&self) -> Self;

    // TODO: remove this method once implement Mul & Add for references to avoid cloning.
    fn add(&self, other: &Self) -> Self;

    // TODO: remove this method once implement Mul & Add for references to avoid cloning.
    fn mul(&self, other: &Self) -> Self;

    // TODO: is there a way to implement this for all types that implement From<IcuRatio>?
    fn from_icu_ratio(value: IcuRatio) -> Option<Self>;
}

impl ConverterRatio for IcuRatio {
    fn reciprocal(&self) -> Self {
        self.recip()
    }

    fn from_icu_ratio(value: IcuRatio) -> Option<Self> {
        Some(value)
    }

    fn mul(&self, other: &Self) -> Self {
        self * other
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }
}

impl ConverterRatio for f64 {
    fn reciprocal(&self) -> Self {
        self.recip()
    }

    fn from_icu_ratio(value: IcuRatio) -> Option<Self> {
        value.to_f64()
    }

    fn mul(&self, other: &Self) -> Self {
        self * other
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }
}
