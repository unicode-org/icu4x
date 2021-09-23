// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::map::ZeroMapKV;
use crate::ule::*;
use crate::{VarZeroVec, ZeroMap, ZeroVec};
use core::{mem, ptr};
use yoke::*;

// This impl is similar to the impl on Cow and is safe for the same reasons
/// This impl can be made available by enabling the optional `yoke` feature of the `zerovec` crate
unsafe impl<'a, T: 'static + AsULE + ?Sized> Yokeable<'a> for ZeroVec<'static, T> {
    type Output = ZeroVec<'a, T>;
    fn transform(&'a self) -> &'a ZeroVec<'a, T> {
        self
    }
    fn transform_owned(self) -> ZeroVec<'a, T> {
        self
    }
    unsafe fn make(from: ZeroVec<'a, T>) -> Self {
        debug_assert!(mem::size_of::<ZeroVec<'a, T>>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

// This impl is similar to the impl on Cow and is safe for the same reasons
/// This impl can be made available by enabling the optional `yoke` feature of the `zerovec` crate
unsafe impl<'a, T: 'static + AsVarULE> Yokeable<'a> for VarZeroVec<'static, T> {
    type Output = VarZeroVec<'a, T>;
    fn transform(&'a self) -> &'a VarZeroVec<'a, T> {
        self
    }
    fn transform_owned(self) -> VarZeroVec<'a, T> {
        self
    }
    unsafe fn make(from: VarZeroVec<'a, T>) -> Self {
        debug_assert!(mem::size_of::<VarZeroVec<'a, T>>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

/// This impl can be made available by enabling the optional `yoke` feature of the `zerovec` crate
#[allow(clippy::transmute_ptr_to_ptr)]
unsafe impl<'a, K, V> Yokeable<'a> for ZeroMap<'static, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b>,
    V: 'static + for<'b> ZeroMapKV<'b>,
{
    type Output = ZeroMap<'a, K, V>;
    fn transform(&'a self) -> &'a ZeroMap<'a, K, V> {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    fn transform_owned(self) -> ZeroMap<'a, K, V> {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        unsafe {
            // Similar problem as transform(), but we need to use ptr::read since
            // the compiler isn't sure of the sizes
            let ptr: *const Self::Output = (&self as *const Self).cast();
            mem::forget(self);
            ptr::read(ptr)
        }
    }
    unsafe fn make(from: ZeroMap<'a, K, V>) -> Self {
        debug_assert!(mem::size_of::<ZeroMap<'a, K, V>>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

impl<'a, T: 'static + AsULE + ?Sized> ZeroCopyFrom<ZeroVec<'a, T>> for ZeroVec<'static, T> {
    fn zero_copy_from<'b>(cart: &'b ZeroVec<'a, T>) -> ZeroVec<'b, T> {
        ZeroVec::Borrowed(cart.as_slice())
    }
}

impl<'a, T: 'static + AsVarULE + Clone> ZeroCopyFrom<VarZeroVec<'a, T>> for VarZeroVec<'static, T> {
    fn zero_copy_from<'b>(cart: &'b VarZeroVec<'a, T>) -> VarZeroVec<'b, T> {
        // the owned variant is not compatible with the borrowed one
        // clones are shallow for the borrowed variant anyway
        Clone::clone(cart)
    }
}

impl<'a, K, V> ZeroCopyFrom<ZeroMap<'a, K, V>> for ZeroMap<'static, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b>,
    V: 'static + for<'b> ZeroMapKV<'b>,
    <K as ZeroMapKV<'static>>::Container: for<'b> ZeroCopyFrom<<K as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'static>>::Container: for<'b> ZeroCopyFrom<<V as ZeroMapKV<'b>>::Container>,
    <K as ZeroMapKV<'static>>::Container:
        for<'b> Yokeable<'b, Output = <K as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'static>>::Container:
        for<'b> Yokeable<'b, Output = <V as ZeroMapKV<'b>>::Container>,
{
    fn zero_copy_from<'b>(cart: &'b ZeroMap<'a, K, V>) -> ZeroMap<'b, K, V> {
        ZeroMap {
            keys: <<K as ZeroMapKV<'static>>::Container as ZeroCopyFrom<_>>::zero_copy_from(
                &cart.keys,
            ),
            values: <<V as ZeroMapKV<'static>>::Container as ZeroCopyFrom<_>>::zero_copy_from(
                &cart.values,
            ),
        }
    }
}
