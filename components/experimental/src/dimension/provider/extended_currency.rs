// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_plurals::provider::PluralElementsPackedCow;
use icu_provider::prelude::*;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

/// Currency Extended V1 data struct.
#[icu_provider::data_struct(marker(
    CurrencyExtendedDataV1Marker,
    "currency/extended@1",
    attributes_domain = "currency",
))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyExtendedDataV1<'data> {
    /// Contains the localized display names for a currency based on plural rules.
    /// For instance, in the "en" locale for the "USD" currency:
    ///     - "US Dollars" when count is `zero`,
    ///     - "US Dollar" when count is `one`,
    ///     ... etc.
    /// # NOTE
    ///    Regards to the [Unicode Report TR35](https://unicode.org/reports/tr35/tr35-numbers.html#Currencies),
    ///    If no matching for specific count, the `other` count will be used.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub display_names: PluralElementsPackedCow<'data, str>,
}

impl<'data> CurrencyExtendedDataV1<'data> {
    /// Construct an instance directly from a byte slice.
    ///
    /// # Safety
    ///
    /// The bytes must represent a valid [`icu_plurals::provider::PluralElementsPackedULE`]
    pub const unsafe fn from_bytes_unchecked(bytes: &'data [u8]) -> Self {
        Self {
            display_names: icu_plurals::provider::PluralElementsPackedCow {
                elements: alloc::borrow::Cow::Borrowed(
                    // Safety: this function's safety invariant guarantees that the bytes
                    // represent a valid `PluralElementsPackedULE`
                    icu_plurals::provider::PluralElementsPackedULE::from_bytes_unchecked(bytes),
                ),
            },
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for CurrencyExtendedDataV1<'_> {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        use zerovec::ule::VarULE;
        ctx.insert("icu_experimental::dimension::provider::extended_currency");
        let bytes = self.display_names.elements.as_bytes().bake(ctx);
        // Safety: The bytes are returned by `PluralElementsPackedULE::slice_as_bytes`.
        databake::quote! { unsafe {
            icu_experimental::dimension::provider::extended_currency::CurrencyExtendedDataV1::from_bytes_unchecked(#bytes)
        }}
    }
}

#[cfg(feature = "datagen")]
impl databake::BakeSize for CurrencyExtendedDataV1<'_> {
    fn borrows_size(&self) -> usize {
        self.display_names.borrows_size()
    }
}
