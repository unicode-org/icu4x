// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for currency symbols.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::{TinyAsciiStr, tinystr};
use zerovec::VarZeroCow;

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
pub struct CurrencySymbol<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub symbol: VarZeroCow<'data, str>,
    pub starts_with_letter: bool,
    pub ends_with_letter: bool,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub decimal_separator: Option<VarZeroCow<'data, str>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub grouping_separator: Option<VarZeroCow<'data, str>>,
}

impl CurrencySymbol<'_> {
    pub fn new(
        symbol: &str,
        starts_with_letter: bool,
        ends_with_letter: bool,
        decimal_separator: Option<&str>,
        grouping_separator: Option<&str>,
    ) -> CurrencySymbol<'static> {
        CurrencySymbol {
            symbol: VarZeroCow::from(alloc::boxed::Box::<str>::from(symbol)),
            starts_with_letter,
            ends_with_letter,
            decimal_separator: decimal_separator
                .map(|s| VarZeroCow::from(alloc::boxed::Box::<str>::from(s))),
            grouping_separator: grouping_separator
                .map(|s| VarZeroCow::from(alloc::boxed::Box::<str>::from(s))),
        }
    }

    /// Returns true if the symbol starts with a letter.
    pub fn starts_with_letter(&self) -> bool {
        self.starts_with_letter
    }

    /// Returns true if the symbol ends with a letter.
    pub fn ends_with_letter(&self) -> bool {
        self.ends_with_letter
    }

    pub(crate) fn decimal_separator_char(&self) -> Option<char> {
        self.decimal_separator
            .as_deref()
            .and_then(|s| s.chars().next())
    }

    pub(crate) fn grouping_separator_char(&self) -> Option<char> {
        self.grouping_separator
            .as_deref()
            .and_then(|s| s.chars().next())
    }

    /// Returns the symbol as a string slice.
    pub fn as_str(&self) -> &str {
        &self.symbol
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

icu_provider::data_struct!(CurrencySymbol<'_>, #[cfg(feature = "datagen")]);
