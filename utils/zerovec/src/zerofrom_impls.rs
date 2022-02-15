use zerofrom::ZeroFrom;

use crate::map::ZeroMapKV;

use crate::ule::*;
use crate::{VarZeroVec, ZeroMap, ZeroMap2d, ZeroVec};

impl<'zcf, T> ZeroFrom<'zcf, ZeroVec<'_, T>> for ZeroVec<'zcf, T>
where
    T: 'static + AsULE + ?Sized,
{
    #[inline]
    fn zero_copy_from(cart: &'zcf ZeroVec<'_, T>) -> Self {
        ZeroVec::Borrowed(cart.as_ule_slice())
    }
}

impl<'zcf, T> ZeroFrom<'zcf, VarZeroVec<'_, T>> for VarZeroVec<'zcf, T>
where
    T: 'static + VarULE + ?Sized,
{
    #[inline]
    fn zero_copy_from(cart: &'zcf VarZeroVec<'_, T>) -> Self {
        cart.as_slice().into()
    }
}

impl<'zcf, 's, K, V> ZeroFrom<'zcf, ZeroMap<'s, K, V>> for ZeroMap<'zcf, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K as ZeroMapKV<'zcf>>::Container: ZeroFrom<'zcf, <K as ZeroMapKV<'s>>::Container>,
    <V as ZeroMapKV<'zcf>>::Container: ZeroFrom<'zcf, <V as ZeroMapKV<'s>>::Container>,
{
    fn zero_copy_from(cart: &'zcf ZeroMap<'s, K, V>) -> Self {
        ZeroMap {
            keys: K::Container::zero_copy_from(&cart.keys),
            values: V::Container::zero_copy_from(&cart.values),
        }
    }
}

impl<'zcf, 's, K0, K1, V> ZeroFrom<'zcf, ZeroMap2d<'s, K0, K1, V>> for ZeroMap2d<'zcf, K0, K1, V>
where
    K0: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    K1: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K0 as ZeroMapKV<'zcf>>::Container: ZeroFrom<'zcf, <K0 as ZeroMapKV<'s>>::Container>,
    <K1 as ZeroMapKV<'zcf>>::Container: ZeroFrom<'zcf, <K1 as ZeroMapKV<'s>>::Container>,
    <V as ZeroMapKV<'zcf>>::Container: ZeroFrom<'zcf, <V as ZeroMapKV<'s>>::Container>,
{
    fn zero_copy_from(cart: &'zcf ZeroMap2d<'s, K0, K1, V>) -> Self {
        ZeroMap2d {
            keys0: K0::Container::zero_copy_from(&cart.keys0),
            joiner: ZeroVec::zero_copy_from(&cart.joiner),
            keys1: K1::Container::zero_copy_from(&cart.keys1),
            values: V::Container::zero_copy_from(&cart.values),
        }
    }
}
