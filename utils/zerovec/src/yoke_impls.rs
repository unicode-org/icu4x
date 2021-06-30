// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::map::ZeroMapKV;
use crate::ule::*;
use crate::{VarZeroVec, ZeroMap, ZeroVec};
use std::{mem, ptr};
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
unsafe impl<
        'a,
        K: 'static + ZeroMapKV<'static> + Yokeable<'a>,
        V: 'static + ZeroMapKV<'static> + Yokeable<'a>,
    > Yokeable<'a> for ZeroMap<'static, K, V>
where
    K::Output: ZeroMapKV<'a>,
    V::Output: ZeroMapKV<'a>,
{
    type Output = ZeroMap<'a, K::Output, V::Output>;
    fn transform(&'a self) -> &'a ZeroMap<'a, K::Output, V::Output> {
        unsafe {
            // Unfortunately, because K and V are generic, rustc is
            // unaware that these are covariant types, and cannot perform this cast automatically.
            // We transmute it instead, and enforce covariance with the `K, V: Yokeable<'a>` bounds
            mem::transmute::<&Self, &Self::Output>(self)
        }
    }
    fn transform_owned(self) -> ZeroMap<'a, K::Output, V::Output> {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        unsafe {
            // Similar problem as transform(), but we need to use ptr::read since
            // the compiler isn't sure of the sizes
            let ptr: *const Self::Output = (&self as *const Self).cast();
            mem::forget(self);
            ptr::read(ptr)
        }
    }
    unsafe fn make(from: ZeroMap<'a, K::Output, V::Output>) -> Self {
        debug_assert!(
            mem::size_of::<ZeroMap<'a, K::Output, V::Output>>() == mem::size_of::<Self>()
        );
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
