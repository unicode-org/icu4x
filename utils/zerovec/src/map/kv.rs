// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::vecs::ZeroVecLike;
use crate::ule::*;
use crate::VarZeroVec;
use crate::ZeroVec;
use std::cmp::Ordering;

/// Trait marking types which are allowed to be keys or values in [`ZeroMap`](super::ZeroMap).
///
/// Users should not be calling methods of this trait directly, however if you are
/// implementing your own [`AsULE`] or [`AsVarULE`] type you may wish to implement
/// this trait.
// this lifetime should be a GAT on Container once that is possible
#[allow(clippy::upper_case_acronyms)] // KV is not an acronym
pub trait ZeroMapKV<'a>: Sized {
    /// The container that can be used with this type: [`ZeroVec`] or [`VarZeroVec`].
    type Container: ZeroVecLike<'a, Self, NeedleType = Self::NeedleType, GetType = Self::GetType>
        + Sized;
    /// The type to use with `Container::binary_search()`
    ///
    /// This type will be predetermined by the choice of `Self::Container`
    type NeedleType: ?Sized;
    /// The type produces by `Container::get()`
    ///
    /// This type will be predetermined by the choice of `Self::Container`
    type GetType: ?Sized;
    /// The type to use whilst serializing. This may not necessarily be `Self`, however it
    /// must serialize to the exact same thing as `Self`
    type SerializeType: ?Sized;
    /// Convert to a needle for searching
    fn as_needle(&self) -> &Self::NeedleType;
    /// Compare this type with a `Self::GetType`. This must produce the same result as
    /// if `g` were converted to `Self`
    fn cmp_get(&self, g: &Self::GetType) -> Ordering;
    /// Obtain a version of this type suitable for serialization
    ///
    /// This uses a callback because it's not possible to return owned-or-borrowed
    /// types without GATs
    fn with_ser<R>(g: &Self::GetType, f: impl FnOnce(&Self::SerializeType) -> R) -> R;
}

macro_rules! impl_sized_kv {
    ($ty:ident) => {
        impl<'a> ZeroMapKV<'a> for $ty {
            type Container = ZeroVec<'a, $ty>;
            type NeedleType = $ty;
            type GetType = <$ty as AsULE>::ULE;
            type SerializeType = $ty;
            fn as_needle(&self) -> &Self {
                self
            }
            fn cmp_get(&self, g: &Self::GetType) -> Ordering {
                self.cmp(&$ty::from_unaligned(g))
            }

            fn with_ser<R>(g: &Self::GetType, f: impl FnOnce(&Self) -> R) -> R {
                f(&Self::from_unaligned(g))
            }
        }
    };
}

impl_sized_kv!(u16);
impl_sized_kv!(u32);
impl_sized_kv!(u64);
impl_sized_kv!(u128);
impl_sized_kv!(i16);
impl_sized_kv!(i32);
impl_sized_kv!(i64);
impl_sized_kv!(i128);
impl_sized_kv!(char);

impl<'a> ZeroMapKV<'a> for String {
    type Container = VarZeroVec<'a, String>;
    type NeedleType = str;
    type GetType = str;
    type SerializeType = str;
    fn as_needle(&self) -> &str {
        &self
    }
    fn cmp_get(&self, g: &str) -> Ordering {
        (&**self).cmp(g)
    }

    fn with_ser<R>(g: &str, f: impl FnOnce(&str) -> R) -> R {
        f(g)
    }
}
