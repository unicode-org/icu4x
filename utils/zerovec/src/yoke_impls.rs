// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This way we can copy-paste Yokeable impls
#![allow(clippy::forget_copy)]

use crate::flexzerovec::FlexZeroVec;
use crate::map::ZeroMapBorrowed;
use crate::map::ZeroMapKV;
use crate::map2d::ZeroMap2dBorrowed;
use crate::ule::*;
use crate::{VarZeroVec, ZeroMap, ZeroMap2d, ZeroVec};
use core::{mem, ptr};
use yoke::*;

// This impl is similar to the impl on Cow and is safe for the same reasons
/// This impl can be made available by enabling the optional `yoke` feature of the `zerovec` crate
unsafe impl<'a, T: 'static + AsULE + ?Sized> Yokeable<'a> for ZeroVec<'static, T> {
    type Output = ZeroVec<'a, T>;
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    #[inline]
    fn transform_owned(self) -> Self::Output {
        self
    }
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
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
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    #[inline]
    fn transform_owned(self) -> Self::Output {
        self
    }
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

// This impl is similar to the impl on Cow and is safe for the same reasons
/// This impl can be made available by enabling the optional `yoke` feature of the `zerovec` crate
unsafe impl<'a> Yokeable<'a> for FlexZeroVec<'static> {
    type Output = FlexZeroVec<'a>;
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    #[inline]
    fn transform_owned(self) -> Self::Output {
        self
    }
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
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
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    #[inline]
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
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
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
    &'static <K as ZeroMapKV<'static>>::Slice: for<'b> Yokeable<'b>,
    &'static <V as ZeroMapKV<'static>>::Slice: for<'b> Yokeable<'b>,
{
    type Output = ZeroMapBorrowed<'a, K, V>;
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    #[inline]
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
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
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
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K0, K1, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    #[inline]
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
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
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
    &'static <K0 as ZeroMapKV<'static>>::Slice: for<'b> Yokeable<'b>,
    &'static <K1 as ZeroMapKV<'static>>::Slice: for<'b> Yokeable<'b>,
    &'static <V as ZeroMapKV<'static>>::Slice: for<'b> Yokeable<'b>,
{
    type Output = ZeroMap2dBorrowed<'a, K0, K1, V>;
    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce the lack of a lifetime with the `K0, K1, V: 'static` bound
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    #[inline]
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
    #[inline]
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    #[inline]
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

#[cfg(test)]
#[allow(non_camel_case_types)]
mod test {
    use super::*;
    use crate::{vecs::FlexZeroSlice, VarZeroSlice, ZeroSlice};

    // Note: The following derives cover Yoke as well as Serde and CrabBake. These may partially
    // duplicate tests elsewhere in this crate, but they are here for completeness.

    #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    struct DeriveTest_ZeroVec<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: ZeroVec<'data, u16>,
    }

    #[derive(yoke::Yokeable)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    struct DeriveTest_ZeroSlice<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: &'data ZeroSlice<u16>,
    }

    #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    struct DeriveTest_FlexZeroVec<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: FlexZeroVec<'data>,
    }

    #[derive(yoke::Yokeable)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    struct DeriveTest_FlexZeroSlice<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: &'data FlexZeroSlice,
    }

    #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    struct DeriveTest_VarZeroVec<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: VarZeroVec<'data, str>,
    }

    #[derive(yoke::Yokeable)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    struct DeriveTest_VarZeroSlice<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: &'data VarZeroSlice<str>,
    }

    #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMap<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: ZeroMap<'data, [u8], str>,
    }

    #[derive(yoke::Yokeable)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMapBorrowed<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: ZeroMapBorrowed<'data, [u8], str>,
    }

    #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMapWithULE<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: ZeroMap<'data, ZeroSlice<u32>, str>,
    }

    #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMap2d<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: ZeroMap2d<'data, u16, u16, str>,
    }

    #[derive(yoke::Yokeable)]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[cfg_attr(feature = "crabbake", derive(crabbake::Bakeable), crabbake(path = zerovec::yoke_impl::test))]
    #[yoke(prove_covariance_manually)]
    struct DeriveTest_ZeroMap2dBorrowed<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        _data: ZeroMap2dBorrowed<'data, u16, u16, str>,
    }
}
