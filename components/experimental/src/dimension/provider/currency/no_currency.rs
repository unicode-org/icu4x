// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for no-currency patterns.

use icu_pattern::DoublePlaceholderPattern;
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use super::PatternPair;
use crate::dimension::currency::options::CurrencyUsage;

icu_provider::data_marker!(
    /// Provides standard and accounting currency patterns with no currency symbol placeholder.
    CurrencyPatternsNoCurrencyV1,
    CurrencyPatternsNoCurrency<'static>,
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
pub struct CurrencyPatternsNoCurrency<'data> {
    /// A packed list of distinct no-currency patterns referenced by [`NoCurrencyPatternIndices`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, DoublePlaceholderPattern>,

    /// Indices into `patterns` for each formatting variant.
    pub indices: NoCurrencyPatternIndices,
}

/// Indices into `patterns` in [`CurrencyPatternsNoCurrency`].
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

icu_provider::data_struct!(CurrencyPatternsNoCurrency<'_>, #[cfg(feature = "datagen")]);

impl<'a> CurrencyPatternsNoCurrency<'a> {
    /// A pass-through pattern containing only placeholder `{0}` (the formatted numeric value).
    ///
    /// This is used as a safe fallback if an index in [`NoCurrencyPatternIndices`] is out of bounds
    /// due to corrupt or malformed provider data.
    const PASS_THROUGH: &'static DoublePlaceholderPattern =
        DoublePlaceholderPattern::from_ref_store_unchecked("\u{2}\u{1}");

    /// Returns the no-currency patterns for `usage`.
    pub fn patterns(&'a self, usage: CurrencyUsage) -> PatternPair<'a> {
        let i = self.indices;
        let (positive, negative) = match usage {
            CurrencyUsage::Standard => (i.standard, i.standard_negative),
            CurrencyUsage::Accounting => (i.accounting_positive, i.accounting_negative),
        };
        PatternPair {
            positive: self.patterns.get(positive as usize).unwrap_or_else(|| {
                debug_assert!(false, "Pattern index is out of bounds; data may be corrupt");
                Self::PASS_THROUGH
            }),
            negative: negative.and_then(|i| self.patterns.get(i as usize)),
        }
    }
}

#[test]
fn test_pass_through_pattern() {
    use writeable::assert_writeable_eq;

    assert_eq!(
        CurrencyPatternsNoCurrency::PASS_THROUGH,
        &*DoublePlaceholderPattern::try_from_str("{0}", Default::default()).unwrap()
    );

    assert_writeable_eq!(
        CurrencyPatternsNoCurrency::PASS_THROUGH.interpolate((123, "")),
        "123"
    );
}
