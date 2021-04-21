// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::vecs::ZeroVecLike;
use crate::ule::*;
use crate::VarZeroVec;
use crate::ZeroVec;
use std::cmp::Ordering;

// this lifetime should be a GAT on Container once that is possible
pub trait ZeroMapKV<'a>: Sized {
    type Container: ZeroVecLike<'a, Self, NeedleType = Self::NeedleType, GetType = Self::GetType>
        + Sized;
    type NeedleType: ?Sized;
    type GetType: ?Sized;
    type SerializeType: ?Sized;
    fn as_needle(&self) -> &Self::NeedleType;
    fn cmp_get(&self, g: &Self::GetType) -> Ordering;
    // This uses a callback because it's not possible to return owned-or-borrowed
    // types without GATs
    fn get_as_ser<R>(g: &Self::GetType, f: impl FnOnce(&Self::SerializeType) -> R) -> R;
}

macro_rules! impl_sized_kv {
    ($($ty:ident),+) => {
        $(
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
                fn get_as_ser<R>(g: &Self::GetType, f: impl FnOnce(&Self) -> R) -> R {
                    f(&Self::from_unaligned(g))
                }
            }
        )+
    };
}

impl_sized_kv!(u16, u32, u64, u128, i16, i32, i64, i128, char);

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
    fn get_as_ser<R>(g: &str, f: impl FnOnce(&str) -> R) -> R {
        f(g)
    }
}
