// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Bake;

/// A trait for an item that can bake to something other than itself.
///
/// For an unsafe version of this trait, see [`CustomBakeUnsafe`].
///
/// The type implementing this trait should have an associated function
/// with the following signature:
///
/// ```ignore
/// /// The argument should have been returned from [`Self::to_custom_bake`].
/// pub fn from_custom_bake(baked: CustomBake::BakedType) -> Self
/// ```
pub trait CustomBake {
    /// The type of the custom bake.
    type BakedType<'a>: Bake
    where
        Self: 'a;
    /// Returns `self` as the custom bake type.
    fn to_custom_bake(&self) -> Self::BakedType<'_>;
}

/// Same as [`CustomBake`] but allows for the constructor to be `unsafe`.
///
/// # Safety
///
/// The type implementing this trait MUST have an associated unsafe function
/// with the following signature:
///
/// ```ignore
/// /// Safety: the argument MUST have been returned from [`Self::to_custom_bake`].
/// pub unsafe fn from_custom_bake(baked: CustomBakeUnsafe::BakedType) -> Self
/// ```
pub unsafe trait CustomBakeUnsafe {
    /// The type of the custom bake.
    type BakedType<'a>: Bake
    where
        Self: 'a;
    /// Returns `self` as the custom bake type.
    fn to_custom_bake(&self) -> Self::BakedType<'_>;
}
