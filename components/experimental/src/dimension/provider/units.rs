// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::provider::PluralElementsPackedCow;
use icu_provider::prelude::*;

#[icu_provider::data_struct(marker(
    UnitsDisplayNameV1Marker,
    "units/displaynames@1",
    attributes_domain = "units"
))]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsDisplayNameV1<'data> {
    // TODO: use `MeasureUnit` for the units key instead of strings.
    /// Contains the long width patterns for the units.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: PluralElementsPackedCow<'data, SinglePlaceholderPattern>,
}

impl<'data> UnitsDisplayNameV1<'data> {
    /// Construct an instance directly from a byte slice.
    ///
    /// # Safety
    ///
    /// The bytes must represent a valid [`icu_plurals::provider::PluralElementsPackedULE`]
    pub const unsafe fn from_bytes_unchecked(bytes: &'data [u8]) -> Self {
        Self {
            patterns: icu_plurals::provider::PluralElementsPackedCow {
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
impl databake::Bake for UnitsDisplayNameV1<'_> {
    fn bake(&self, ctx: &databake::CrateEnv) -> databake::TokenStream {
        use zerovec::ule::VarULE;
        ctx.insert("icu_experimental::dimension::provider::units");
        let bytes = self.patterns.elements.as_bytes().bake(ctx);
        // Safety: The bytes are returned by `PluralElementsPackedULE::slice_as_bytes`.
        databake::quote! { unsafe {
            icu_experimental::dimension::provider::units::UnitsDisplayNameV1::from_bytes_unchecked(#bytes)
        }}
    }
}

#[cfg(feature = "datagen")]
impl databake::BakeSize for UnitsDisplayNameV1<'_> {
    fn borrows_size(&self) -> usize {
        self.patterns.borrows_size()
    }
}
