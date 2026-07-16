// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for currency symbols.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::ule::vartuple::VarTupleULE;
use zerovec::{VarZeroVec, ZeroMap};

use crate::dimension::currency::CurrencyCode;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

icu_provider::data_marker!(
    /// Currency symbol data needed for short and narrow currency formatting.
    CurrencySymbolsV1,
    CurrencySymbols<'static>
);

/// This type contains the symbol mappings for short and narrow currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[yoke(prove_covariance_manually)]
pub struct CurrencySymbols<'data> {
    /// A mapping from 3-letter currency ISO codes to their [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern_config_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>,

    /// A list of symbols, including short (`symbol`) and narrow (`symbol-narrow`)
    /// currency symbols (such as `$`, `€`, `US$`), referenced by index.
    ///
    /// These values are retrieved using [`CurrencySymbol::Index`] stored in [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub symbols: VarZeroVec<'data, VarTupleULE<u8, str>>,
}

icu_provider::data_struct!(CurrencySymbols<'_>, #[cfg(feature = "datagen")]);

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum CurrencySymbolIndex {
    /// The index of the symbol in the symbols list.
    /// NOTE: the maximum value is `MAX_SYMBOL_INDEX` which is 2045 (`0b0111_1111_1101`).
    Index(u16),

    /// The symbol is the ISO code.
    ISO,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct CurrencySymbol<'a> {
    pub starts_with_letter: bool,
    pub ends_with_letter: bool,
    pub symbol: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub enum Width {
    /// Format the currency with the standard (short) currency symbol.
    ///
    /// For example, 1 USD formats as "$1.00" in en-US and "US$1" in most other locales.
    Short,

    /// Format the currency with the narrow currency symbol.
    ///
    /// The narrow symbol may be ambiguous, so it should be evident from context which
    /// currency is being represented.
    ///
    /// For example, 1 USD formats as "$1.00" in most locales.
    Narrow,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Copy, Debug, Clone, Default, PartialEq, Eq)]
pub struct CurrencyPatternConfig {
    /// The symbol for short currency formatting.
    /// If the value is `None`, this means that the short pattern does not have a symbol.
    pub short_symbol: Option<CurrencySymbolIndex>,

    /// The symbol for narrow currency formatting.
    /// If the value is `None`, this means that the narrow pattern does not have a symbol.
    pub narrow_symbol: Option<CurrencySymbolIndex>,
}

impl<'a> CurrencySymbols<'a> {
    /// Returns the formatted currency name/symbol,
    /// the currency pattern for the given width and currency,
    /// and the pattern selection.
    pub fn get(&'a self, width: Width, currency: &'a CurrencyCode) -> CurrencySymbol<'a> {
        let config = self
            .pattern_config_map
            .get_copied(&currency.0.to_unvalidated())
            .unwrap_or(CurrencyPatternConfig {
                short_symbol: None,
                narrow_symbol: None,
            });

        let symbol = match width {
            Width::Short => config.short_symbol,
            Width::Narrow => config.narrow_symbol,
        };

        match symbol {
            Some(CurrencySymbolIndex::Index(index)) => {
                self.symbols.get(index.into()).map(|vt| CurrencySymbol {
                    symbol: &vt.variable,
                    starts_with_letter: vt.sized & 0b10 != 0,
                    ends_with_letter: vt.sized & 0b01 != 0,
                })
            }
            Some(CurrencySymbolIndex::ISO) | None => None,
        }
        .unwrap_or(CurrencySymbol {
            symbol: currency.0.as_str(),
            starts_with_letter: true,
            ends_with_letter: true,
        })
    }
}
