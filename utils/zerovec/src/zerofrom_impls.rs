// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerofrom::ZeroFrom;
use crate::map::ZeroMapKV;
use crate::ule::*;
use crate::{VarZeroVec, ZeroMap, ZeroMap2d, ZeroVec};

impl<'zf, T> ZeroFrom<'zf, ZeroVec<'_, T>> for ZeroVec<'zf, T>
where
    T: 'static + AsULE + ?Sized,
{
    #[inline]
    fn zero_from(cart: &'zf ZeroVec<'_, T>) -> Self {
        ZeroVec::Borrowed(cart.as_ule_slice())
    }
}

impl<'zf, T> ZeroFrom<'zf, VarZeroVec<'_, T>> for VarZeroVec<'zf, T>
where
    T: 'static + VarULE + ?Sized,
{
    #[inline]
    fn zero_from(cart: &'zf VarZeroVec<'_, T>) -> Self {
        cart.as_slice().into()
    }
}

impl<'zf, 's, K, V> ZeroFrom<'zf, ZeroMap<'s, K, V>> for ZeroMap<'zf, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <K as ZeroMapKV<'s>>::Container>,
    <V as ZeroMapKV<'zf>>::Container: ZeroFrom<'zf, <V as ZeroMapKV<'s>>::Container>,
{
    fn zero_from(cart: &'zf ZeroMap<'s, K, V>) -> Self {
        ZeroMap {
            keys: K::Container::zero_from(&cart.keys),
            values: V::Container::zero_from(&cart.values),
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
    fn zero_from(cart: &'zf ZeroMap2d<'s, K0, K1, V>) -> Self {
        ZeroMap2d {
            keys0: K0::Container::zero_from(&cart.keys0),
            joiner: ZeroVec::zero_from(&cart.joiner),
            keys1: K1::Container::zero_from(&cart.keys1),
            values: V::Container::zero_from(&cart.values),
        }
    }
}
