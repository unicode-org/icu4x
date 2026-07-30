// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for no-currency patterns.

use icu_pattern::DoublePlaceholderPattern;
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

icu_provider::data_marker!(
    /// `CurrencyNoCurrencyPatternsV1`
    CurrencyNoCurrencyPatternsV1,
    CurrencyNoCurrencyPatterns<'static>,
);

/// Currency `NoCurrency` patterns data struct.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(
    feature = "datagen",
    databake(path = icu_experimental::dimension::provider::currency::no_currency)
)]
#[yoke(prove_covariance_manually)]
pub struct CurrencyNoCurrencyPatterns<'data> {
    /// A packed list of distinct no-currency patterns referenced by [`NoCurrencyPatternIndices`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, DoublePlaceholderPattern>,

    /// Indices into `patterns` for each formatting variant.
    pub indices: NoCurrencyPatternIndices,
}

/// Indices into `patterns` in [`CurrencyNoCurrencyPatterns`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(
    feature = "datagen",
    databake(path = icu_experimental::dimension::provider::currency::no_currency)
)]
pub struct NoCurrencyPatternIndices {
    /// Standard positive pattern index (required).
    pub standard: u8,
    /// Standard negative pattern index (optional).
    pub standard_negative: Option<u8>,
    /// Accounting positive pattern index (defaults to standard positive).
    pub accounting_positive: u8,
    /// Accounting negative pattern index (optional).
    pub accounting_negative: Option<u8>,
}

icu_provider::data_struct!(CurrencyNoCurrencyPatterns<'_>, #[cfg(feature = "datagen")]);

impl<'a> CurrencyNoCurrencyPatterns<'a> {
    /// Gets the standard positive no-currency pattern.
    pub fn get_positive(&'a self) -> &'a DoublePlaceholderPattern {
        self.patterns
            .get(self.indices.standard as usize)
            .unwrap_or_else(|| {
                debug_assert!(false, "Standard pattern index is out of bounds");
                <&DoublePlaceholderPattern>::default()
            })
    }

    /// Gets the standard negative no-currency pattern.
    pub fn get_negative(&'a self) -> Option<&'a DoublePlaceholderPattern> {
        let idx = self.indices.standard_negative?;
        self.patterns.get(idx as usize)
    }

    /// Gets the accounting positive no-currency pattern.
    pub fn get_positive_accounting(&'a self) -> &'a DoublePlaceholderPattern {
        self.patterns
            .get(self.indices.accounting_positive as usize)
            .unwrap_or_else(|| {
                debug_assert!(false, "Accounting positive pattern index is out of bounds");
                <&DoublePlaceholderPattern>::default()
            })
    }

    /// Gets the accounting negative no-currency pattern.
    pub fn get_negative_accounting(&'a self) -> Option<&'a DoublePlaceholderPattern> {
        let idx = self.indices.accounting_negative?;
        self.patterns.get(idx as usize)
    }
}
