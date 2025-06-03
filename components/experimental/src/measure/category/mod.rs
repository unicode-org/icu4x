// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::measure::measureunit::MeasureUnit;

pub mod area;
pub mod length;
pub mod mass;
pub mod volume;

pub trait MeasureUnitCategory {}

/// A [`MeasureUnit`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedMeasureUnit<T: MeasureUnitCategory> {
    _category: PhantomData<T>,
    pub unit: MeasureUnit,
}

/// The categories of [`MeasureUnit`]s.
pub mod category {
    /// A [`super::MeasureUnit`] that is related to the length category.
    pub struct Length;

    /// A [`super::MeasureUnit`] that is related to the area category.
    pub struct Area;

    /// A [`super::MeasureUnit`] that is related to the volume category.
    pub struct Volume;

    /// A [`super::MeasureUnit`] that is related to the mass category.
    pub struct Mass;
}

impl MeasureUnitCategory for category::Length {}
impl MeasureUnitCategory for category::Area {}
impl MeasureUnitCategory for category::Volume {}
impl MeasureUnitCategory for category::Mass {}
