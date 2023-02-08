// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::map::ZeroMapKV;
use crate::ule::*;
use crate::vecs::{FlexZeroSlice, FlexZeroVec};
use crate::{VarZeroSlice, VarZeroVec, ZeroMap, ZeroMap2d, ZeroSlice, ZeroVec};
use zerofrom::ZeroFrom;

impl<'zf, T> ZeroFrom<'zf, ZeroVec<'_, T>> for ZeroVec<'zf, T>
where
    T: 'static + AsULE + ?Sized,
{
    #[inline]
    fn zero_from(other: &'zf ZeroVec<'_, T>) -> Self {
        ZeroVec::new_borrowed(other.as_ule_slice())
    }
}

impl<'zf, T> ZeroFrom<'zf, ZeroSlice<T>> for ZeroVec<'zf, T>
where
    T: 'static + AsULE + ?Sized,
{
    #[inline]
    fn zero_from(other: &'zf ZeroSlice<T>) -> Self {
        ZeroVec::new_borrowed(other.as_ule_slice())
    }
}

impl<'zf, T> ZeroFrom<'zf, ZeroSlice<T>> for &'zf ZeroSlice<T>
where
    T: 'static + AsULE + ?Sized,
{
    #[inline]
    fn zero_from(other: &'zf ZeroSlice<T>) -> Self {
        other
    }
}

impl<'zf> ZeroFrom<'zf, FlexZeroVec<'_>> for FlexZeroVec<'zf> {
    #[inline]
    fn zero_from(other: &'zf FlexZeroVec<'_>) -> Self {
        FlexZeroVec::Borrowed(other)
    }
}

impl<'zf> ZeroFrom<'zf, FlexZeroSlice> for FlexZeroVec<'zf> {
    #[inline]
    fn zero_from(other: &'zf FlexZeroSlice) -> Self {
        FlexZeroVec::Borrowed(other)
    }
}

impl<'zf> ZeroFrom<'zf, FlexZeroSlice> for &'zf FlexZeroSlice {
    #[inline]
    fn zero_from(other: &'zf FlexZeroSlice) -> Self {
        other
    }
}

impl<'zf, T> ZeroFrom<'zf, VarZeroSlice<T>> for VarZeroVec<'zf, T>
where
    T: 'static + VarULE + ?Sized,
{
    #[inline]
    fn zero_from(other: &'zf VarZeroSlice<T>) -> Self {
        other.into()
    }
}

impl<'zf, T> ZeroFrom<'zf, VarZeroVec<'_, T>> for VarZeroVec<'zf, T>
where
    T: 'static + VarULE + ?Sized,
{
    #[inline]
    fn zero_from(other: &'zf VarZeroVec<'_, T>) -> Self {
        other.as_slice().into()
    }
}

impl<'zf, T> ZeroFrom<'zf, VarZeroSlice<T>> for &'zf VarZeroSlice<T>
where
    T: 'static + VarULE + ?Sized,
{
    #[inline]
    fn zero_from(other: &'zf VarZeroSlice<T>) -> Self {
        other
    }
}

impl<'zf, 's, K, V> ZeroFrom<'zf, ZeroMap<'s, K, V>> for ZeroMap<'zf, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <K as ZeroMapKV<'s>>::Container>,
    <V as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <V as ZeroMapKV<'s>>::Container>,
{
    fn zero_from(other: &'zf ZeroMap<'s, K, V>) -> Self {
        ZeroMap {
            keys: K::Container::zero_from(&other.keys),
            values: V::Container::zero_from(&other.values),
        }
    }
}

impl<'zf, 's, K0, K1, V> ZeroFrom<'zf, ZeroMap2d<'s, K0, K1, V>> for ZeroMap2d<'zf, K0, K1, V>
where
    K0: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    K1: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K0 as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <K0 as ZeroMapKV<'s>>::Container>,
    <K1 as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <K1 as ZeroMapKV<'s>>::Container>,
    <V as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <V as ZeroMapKV<'s>>::Container>,
{
    fn zero_from(other: &'zf ZeroMap2d<'s, K0, K1, V>) -> Self {
        ZeroMap2d {
            keys0: K0::Container::zero_from(&other.keys0),
            joiner: ZeroVec::zero_from(&other.joiner),
            keys1: K1::Container::zero_from(&other.keys1),
            values: V::Container::zero_from(&other.values),
        }
    }
}
