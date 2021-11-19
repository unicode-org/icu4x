// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::vecs::MutableZeroVecLike;
use crate::ule::*;
use crate::VarZeroVec;
use crate::ZeroVec;
use alloc::boxed::Box;

/// Trait marking types which are allowed to be keys or values in [`ZeroMap`](super::ZeroMap).
///
/// Users should not be calling methods of this trait directly, however if you are
/// implementing your own [`AsULE`] or [`VarULE`] type you may wish to implement
/// this trait.
// this lifetime should be a GAT on Container once that is possible
#[allow(clippy::upper_case_acronyms)] // KV is not an acronym
pub trait ZeroMapKV<'a> {
    /// The container that can be used with this type: [`ZeroVec`] or [`VarZeroVec`].
    type Container: MutableZeroVecLike<
            'a,
            Self,
            GetType = Self::GetType,
            OwnedType = Self::OwnedType,
            SerializeType = Self::SerializeType,
        > + Sized;
    /// The type produced by `Container::get()`
    ///
    /// This type will be predetermined by the choice of `Self::Container`
    type GetType: ?Sized + 'static;
    /// The type to use whilst serializing. This may not necessarily be `Self`, however it
    /// must serialize to the exact same thing as `Self`
    type SerializeType: ?Sized;
    /// The type produced by `Container::replace()` and `Container::remove()`,
    /// also used during deserialization. If `Self` is human readable serialized,
    /// deserializing to `Self::OwnedType` should produce the same value once
    /// passed through `Self::owned_as_self()`
    type OwnedType: 'static;
}

macro_rules! impl_sized_kv {
    ($ty:ident) => {
        impl<'a> ZeroMapKV<'a> for $ty {
            type Container = ZeroVec<'a, $ty>;
            type GetType = <$ty as AsULE>::ULE;
            type SerializeType = $ty;
            type OwnedType = $ty;
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

impl<'a> ZeroMapKV<'a> for str {
    type Container = VarZeroVec<'a, str>;
    type GetType = str;
    type SerializeType = str;
    type OwnedType = Box<str>;
}

impl<'a> ZeroMapKV<'a> for [u8] {
    type Container = VarZeroVec<'a, [u8]>;
    type GetType = [u8];
    type SerializeType = [u8];
    type OwnedType = Box<[u8]>;
}
