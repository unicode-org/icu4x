// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::relativetime::provider::PluralPatterns;
use icu_pattern::DoublePlaceholder;
use icu_provider::prelude::*;

/// Currency Extended V1 data struct.
#[icu_provider::data_struct(marker(CurrencyPatternsDataV1Marker, "currency/patterns@1"))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyPatternsDataV1<'data> {
    /// Contains the unit patterns for a currency based on plural rules.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: PluralPatterns<'data, DoublePlaceholder>,
}

#[cfg(feature = "datagen")]
impl<'data> CurrencyPatternsDataV1<'data> {
    /// Construct an instance directly from a byte slice.
    /// 
    /// # Safety
    /// 
    /// The bytes must have been returned by [`Self::as_byte_slice`].
    pub const unsafe fn from_byte_slice_unchecked(bytes: &'data [u8]) -> Self {
        Self {
            patterns: PluralPatterns {
                strings: icu_plurals::provider::PluralElementsPackedCow {
                    elements: alloc::borrow::Cow::Borrowed(
                        icu_plurals::provider::PluralElementsPackedULE::from_byte_slice_unchecked(bytes)
                    )
                },
                _phantom: core::marker::PhantomData,
            }
        }
    }

    /// Returns this instance as a byte slice.
    pub fn as_byte_slice(&self) -> &[u8] {
        use zerovec::ule::VarULE;
        self.patterns.strings.elements.as_byte_slice()
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for CurrencyPatternsDataV1<'_> {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        ctx.insert("icu_experimental::dimension::provider::currency_patterns");
        let bytes = self.as_byte_slice().bake(ctx);
        // Safety: The bytes are returned by `self.as_byte_slice()`.
        databake::quote! { unsafe {
            icu_experimental::dimension::provider::currency_patterns::CurrencyPatternsDataV1::from_byte_slice_unchecked(#bytes)
        }}
    }
}

#[cfg(feature = "datagen")]
impl databake::BakeSize for CurrencyPatternsDataV1<'_> {
    fn borrows_size(&self) -> usize {
        self.patterns.borrows_size()
    }
}
