// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This way we can copy-paste Yokeable impls
#![allow(clippy::forget_copy)]

use crate::map::ZeroMapBorrowed;
use crate::map::ZeroMapKV;
use crate::map::ZeroVecLike;
use crate::map2d::ZeroMap2dBorrowed;
use crate::ule::*;
use crate::{VarZeroVec, ZeroMap, ZeroMap2d, ZeroVec};
use core::{mem, ptr};
use yoke::*;

// This impl is similar to the impl on Cow and is safe for the same reasons
/// This impl can be made available by enabling the optional `yoke` feature of the `zerovec` crate
unsafe impl<'a, T: 'static + AsULE + ?Sized> Yokeable<'a> for ZeroVec<'static, T> {
    type Output = ZeroVec<'a, T>;
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    fn transform_owned(self) -> Self::Output {
        self
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
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
unsafe impl<'a, T: 'static + VarULE + ?Sized> Yokeable<'a> for VarZeroVec<'static, T> {
    type Output = VarZeroVec<'a, T>;
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    fn transform_owned(self) -> Self::Output {
        self
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
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
    K: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K as ZeroMapKV<'static>>::Container: for<'b> Yokeable<'b>,
    <V as ZeroMapKV<'static>>::Container: for<'b> Yokeable<'b>,
{
    type Output = ZeroMap<'a, K, V>;
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    fn transform_owned(self) -> Self::Output {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        unsafe {
            // Similar problem as transform(), but we need to use ptr::read since
            // the compiler isn't sure of the sizes
            let ptr: *const Self::Output = (&self as *const Self).cast();
            mem::forget(self);
            ptr::read(ptr)
        }
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
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
unsafe impl<'a, K, V> Yokeable<'a> for ZeroMapBorrowed<'static, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <<K as ZeroMapKV<'static>>::Container as ZeroVecLike<'static, K>>::BorrowedVariant:
        for<'b> Yokeable<'b>,
    <<V as ZeroMapKV<'static>>::Container as ZeroVecLike<'static, V>>::BorrowedVariant:
        for<'b> Yokeable<'b>,
{
    type Output = ZeroMapBorrowed<'a, K, V>;
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    fn transform_owned(self) -> Self::Output {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        unsafe {
            // Similar problem as transform(), but we need to use ptr::read since
            // the compiler isn't sure of the sizes
            let ptr: *const Self::Output = (&self as *const Self).cast();
            mem::forget(self);
            ptr::read(ptr)
        }
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
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
unsafe impl<'a, K0, K1, V> Yokeable<'a> for ZeroMap2d<'static, K0, K1, V>
where
    K0: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    K1: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K0 as ZeroMapKV<'static>>::Container: for<'b> Yokeable<'b>,
    <K1 as ZeroMapKV<'static>>::Container: for<'b> Yokeable<'b>,
    <V as ZeroMapKV<'static>>::Container: for<'b> Yokeable<'b>,
{
    type Output = ZeroMap2d<'a, K0, K1, V>;
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K0, K1, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    fn transform_owned(self) -> Self::Output {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        unsafe {
            // Similar problem as transform(), but we need to use ptr::read since
            // the compiler isn't sure of the sizes
            let ptr: *const Self::Output = (&self as *const Self).cast();
            mem::forget(self);
            ptr::read(ptr)
        }
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
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
unsafe impl<'a, K0, K1, V> Yokeable<'a> for ZeroMap2dBorrowed<'static, K0, K1, V>
where
    K0: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    K1: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <<K0 as ZeroMapKV<'static>>::Container as ZeroVecLike<'static, K0>>::BorrowedVariant:
        for<'b> Yokeable<'b>,
    <<K1 as ZeroMapKV<'static>>::Container as ZeroVecLike<'static, K1>>::BorrowedVariant:
        for<'b> Yokeable<'b>,
    <<V as ZeroMapKV<'static>>::Container as ZeroVecLike<'static, V>>::BorrowedVariant:
        for<'b> Yokeable<'b>,
{
    type Output = ZeroMap2dBorrowed<'a, K0, K1, V>;
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K0, K1, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    fn transform_owned(self) -> Self::Output {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        unsafe {
            // Similar problem as transform(), but we need to use ptr::read since
            // the compiler isn't sure of the sizes
            let ptr: *const Self::Output = (&self as *const Self).cast();
            mem::forget(self);
            ptr::read(ptr)
        }
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
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
        ZeroVec::Borrowed(cart.as_ule_slice())
    }
}

impl<'a, T: 'static + VarULE + ?Sized> ZeroCopyFrom<VarZeroVec<'a, T>> for VarZeroVec<'static, T> {
    fn zero_copy_from<'b>(cart: &'b VarZeroVec<'a, T>) -> VarZeroVec<'b, T> {
        cart.as_slice().into()
    }
}

impl<'a, K, V> ZeroCopyFrom<ZeroMap<'a, K, V>> for ZeroMap<'static, K, V>
where
    K: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
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

impl<'a, K0, K1, V> ZeroCopyFrom<ZeroMap2d<'a, K0, K1, V>> for ZeroMap2d<'static, K0, K1, V>
where
    K0: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    K1: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    V: 'static + for<'b> ZeroMapKV<'b> + ?Sized,
    <K0 as ZeroMapKV<'static>>::Container: for<'b> ZeroCopyFrom<<K0 as ZeroMapKV<'b>>::Container>,
    <K1 as ZeroMapKV<'static>>::Container: for<'b> ZeroCopyFrom<<K1 as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'static>>::Container: for<'b> ZeroCopyFrom<<V as ZeroMapKV<'b>>::Container>,
    <K0 as ZeroMapKV<'static>>::Container:
        for<'b> Yokeable<'b, Output = <K0 as ZeroMapKV<'b>>::Container>,
    <K1 as ZeroMapKV<'static>>::Container:
        for<'b> Yokeable<'b, Output = <K1 as ZeroMapKV<'b>>::Container>,
    <V as ZeroMapKV<'static>>::Container:
        for<'b> Yokeable<'b, Output = <V as ZeroMapKV<'b>>::Container>,
{
    fn zero_copy_from<'b>(cart: &'b ZeroMap2d<'a, K0, K1, V>) -> ZeroMap2d<'b, K0, K1, V> {
        ZeroMap2d {
            keys0: <<K0 as ZeroMapKV<'static>>::Container as ZeroCopyFrom<_>>::zero_copy_from(
                &cart.keys0,
            ),
            joiner: ZeroVec::zero_copy_from(&cart.joiner),
            keys1: <<K1 as ZeroMapKV<'static>>::Container as ZeroCopyFrom<_>>::zero_copy_from(
                &cart.keys1,
            ),
            values: <<V as ZeroMapKV<'static>>::Container as ZeroCopyFrom<_>>::zero_copy_from(
                &cart.values,
            ),
        }
    }
}

#[cfg(test)]
#[allow(non_camel_case_types)]
mod test {
    use super::*;
    use crate::{VarZeroSlice, ZeroSlice};

    #[derive(yoke::Yokeable, yoke::ZeroCopyFrom)]
    struct DeriveTest_ZeroVec<'data> {
        _data: ZeroVec<'data, u16>,
    }

    #[derive(yoke::Yokeable, yoke::ZeroCopyFrom)]
    struct DeriveTest_VarZeroVec<'data> {
        _data: VarZeroVec<'data, str>,
    }

    #[derive(yoke::Yokeable)]
    struct DeriveTest_VarZeroSlice<'data> {
        _data: &'data VarZeroSlice<str>,
    }

    #[derive(yoke::Yokeable, yoke::ZeroCopyFrom)]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMap<'data> {
        _data: ZeroMap<'data, [u8], str>,
    }

    #[derive(yoke::Yokeable)]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMapBorrowed<'data> {
        _data: ZeroMapBorrowed<'data, [u8], str>,
    }

    #[derive(yoke::Yokeable, yoke::ZeroCopyFrom)]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMapWithULE<'data> {
        _data: ZeroMap<'data, ZeroSlice<u32>, str>,
    }

    #[derive(yoke::Yokeable, yoke::ZeroCopyFrom)]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMap2d<'data> {
        _data: ZeroMap2d<'data, u16, u16, str>,
    }

    #[derive(yoke::Yokeable)]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMap2dBorrowed<'data> {
        _data: ZeroMap2dBorrowed<'data, u16, u16, str>,
    }
}
