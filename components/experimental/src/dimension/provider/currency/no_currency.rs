// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for no-currency patterns.

use alloc::borrow::Cow;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;

icu_provider::data_marker!(
    /// `CurrencyNoCurrencyPatternsV1`
    CurrencyNoCurrencyPatternsV1,
    CurrencyNoCurrencyPatterns<'static>,
);

/// Currency NoCurrency patterns data struct.
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
    /// Standard positive pattern (required, e.g., "{0}" or "‏{0}").
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard: Cow<'data, SinglePlaceholderPattern>,

    /// Standard negative pattern (optional, e.g., "-{0}" or "‏-{0}").
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub standard_negative: Option<Cow<'data, SinglePlaceholderPattern>>,

    /// Accounting positive pattern (optional, fallback to `standard`).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub accounting_positive: Option<Cow<'data, SinglePlaceholderPattern>>,

    /// Accounting negative pattern (optional, e.g., "({0})").
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub accounting_negative: Option<Cow<'data, SinglePlaceholderPattern>>,
}

icu_provider::data_struct!(CurrencyNoCurrencyPatterns<'_>, #[cfg(feature = "datagen")]);
