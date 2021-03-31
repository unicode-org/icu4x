// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use writeable::Writeable;
use crate::options::*;

pub struct FormattedFixedDecimal<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) options: &'l FixedDecimalFormatOptions,
    pub(crate) symbols: &'l crate::provider::DecimalSymbolsV1,
}

impl<'l> Writeable for FormattedFixedDecimal<'l> {
    fn write_to<W>(&self, sink: &mut W) -> std::result::Result<(), std::fmt::Error>
    where
        W: std::fmt::Write + ?Sized,
    {
        self.value.write_to(sink)
    }

    // fn write_len(&self) -> writeable::LengthHint {
    //     todo!()
    // }
}
