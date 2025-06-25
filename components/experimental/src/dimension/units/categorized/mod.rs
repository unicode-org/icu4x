// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use icu_provider::DataProvider;

use crate::{dimension::units::formatter::UnitsFormatter, measure::category::{Area, MeasureUnitCategory}};



/// A [`MeasureUnit`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedFormatter<T: FormatterCategory> {
    _category: PhantomData<T>,
    pub formatter: UnitsFormatter,
}

impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
    /// Creates a new [`CategorizedFormatter`] for area units.
    pub fn try_new_unstable<P: ?Sized>(provider: P, formatter: UnitsFormatter) -> Self
     {
        Self {
            _category: PhantomData,
            formatter,
        }
    }

    pub fn format(&self) -> String {
        C::format(&self)
    }
}
