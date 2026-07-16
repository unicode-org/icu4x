// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for currency symbols.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::{TinyAsciiStr, tinystr};
use zerovec::VarZeroCow;
use zerovec::ule::vartuple::{VarTuple, VarTupleULE};

use crate::dimension::currency::CurrencyCode;

icu_provider::data_marker!(
    /// Currency symbol data needed for short and narrow currency formatting.
    CurrencySymbolsV1,
    CurrencySymbol<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "currency",
);

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Debug, Clone, PartialEq, Eq, zerofrom::ZeroFrom, yoke::Yokeable)]

pub struct CurrencySymbol<'a>(
    #[doc(hidden)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub VarZeroCow<'a, VarTupleULE<u8, str>>,
);

impl CurrencySymbol<'_> {
    pub fn new(symbol: &str, starts_with_letter: bool, ends_with_letter: bool) -> Self {
        let sized = (starts_with_letter as u8) << 1 | (ends_with_letter as u8);
        let variable = VarZeroCow::from(symbol);
        Self(VarZeroCow::from_encodeable(&VarTuple { sized, variable }))
    }

    /// Returns true if the symbol starts with a letter.
    pub fn starts_with_letter(&self) -> bool {
        self.0.sized & 0b10 != 0
    }

    /// Returns true if the symbol ends with a letter.
    pub fn ends_with_letter(&self) -> bool {
        self.0.sized & 0b01 != 0
    }

    /// Returns the symbol as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0.variable
    }
}

impl CurrencySymbolsV1 {
    pub const SHORT: TinyAsciiStr<1> = tinystr!(1, "s");
    pub const NARROW: TinyAsciiStr<1> = tinystr!(1, "n");

    pub fn make_attributes(
        currency: CurrencyCode,
        width: TinyAsciiStr<1>,
        buffer: &mut TinyAsciiStr<5>,
    ) -> &DataMarkerAttributes {
        *buffer = width
            .concat::<1, 2>(tinystr!(1, "/"))
            .concat::<3, 5>(currency.0);
        // All valid
        DataMarkerAttributes::try_from_str(buffer).unwrap()
    }
}

icu_provider::data_struct!(
    CurrencySymbol<'_>,
    varule: VarTupleULE<u8, str>,
    #[cfg(feature = "datagen")]
    encode_as_varule: |v: &CurrencySymbol<'_>| &v.0
);

impl<'zf> zerofrom::ZeroFrom<'zf, VarTupleULE<u8, str>> for CurrencySymbol<'zf> {
    fn zero_from(source: &'zf VarTupleULE<u8, str>) -> Self {
        Self(VarZeroCow::zero_from(source))
    }
}
