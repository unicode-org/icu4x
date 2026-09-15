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

use crate::dimension::currency::CurrencyType;

icu_provider::data_marker!(
    /// Currency symbol data needed for short and narrow currency formatting.
    CurrencySymbolsV1,
    CurrencySymbol<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "currency",
);

icu_provider::data_marker!(
    /// Currency-specific decimal symbols override data.
    CurrencyDecimalSymbolsV1,
    "currency/decimal/symbols/v1",
    icu_decimal::provider::DecimalSymbols<'static>,
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

/// The width of a currency symbol.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[non_exhaustive]
pub enum CurrencySymbolWidth {
    /// Standard or short currency symbol (e.g. `"$"` or `"CA$"`).
    Short,
    /// Narrow currency symbol (e.g. `"$"`).
    Narrow,
}

impl CurrencySymbolWidth {
    /// Returns the 1-byte ASCII character code used in data marker attributes.
    pub const fn as_tinystr(self) -> TinyAsciiStr<1> {
        match self {
            Self::Short => tinystr!(1, "s"),
            Self::Narrow => tinystr!(1, "n"),
        }
    }

    /// Returns the character code as a string slice.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Short => "s",
            Self::Narrow => "n",
        }
    }
}

impl CurrencySymbolsV1 {
    pub const SHORT: CurrencySymbolWidth = CurrencySymbolWidth::Short;
    pub const NARROW: CurrencySymbolWidth = CurrencySymbolWidth::Narrow;

    pub fn make_attributes(
        currency: CurrencyType,
        width: CurrencySymbolWidth,
        buffer: &mut TinyAsciiStr<5>,
    ) -> &DataMarkerAttributes {
        *buffer = width
            .as_tinystr()
            .concat::<1, 2>(tinystr!(1, "/"))
            .concat::<3, 5>(currency.iso_code());
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
