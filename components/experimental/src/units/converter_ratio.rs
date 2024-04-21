// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::{Add, Mul};

use crate::units::ratio::IcuRatio;

// TODO: add Mul & Add for references to avoid cloning.
/// A trait for types that can be converted between two units.
pub trait ConverterRatio: Mul<Output = Self> + Add<Output = Self> + Clone {
    fn recip(&self) -> Self;

    // TODO: is there a way to implement this for all types that implement From<IcuRatio>?
    fn from_icu_ratio(value: IcuRatio) -> Option<Self>;
}
impl ConverterRatio for IcuRatio {
    fn recip(&self) -> Self {
        self.recip()
    }

    fn from_icu_ratio(value: IcuRatio) -> Option<Self> {
        Some(value)
    }
}

impl ConverterRatio for f64 {
    fn recip(&self) -> Self {
        self.recip()
    }

    fn from_icu_ratio(value: IcuRatio) -> Option<Self> {
        value.to_f64()
    }
}
