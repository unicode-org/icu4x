// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;

use writeable::Writeable;

use crate::dimension::provider::units::UnitsDisplayNameV1;
use crate::dimension::units::options::UnitsFormatterOptions;

pub struct FormattedUnit<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) unit: &'l str,
    pub(crate) options: &'l UnitsFormatterOptions,
    // pub(crate) essential: &'l UnitsEssentialsV1<'l>,
    pub(crate) display_name: &'l UnitsDisplayNameV1<'l>,
}

impl<'l> Writeable for FormattedUnit<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        todo!();
    }
}
