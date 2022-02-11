// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// In this case consistency between impls is more important
// than using pointer casts
#![allow(clippy::transmute_ptr_to_ptr)]

use crate::{IsCovariant, Yokeable, ZeroCopyFrom};
use core::{mem, ptr};

macro_rules! copy_yoke_impl {
    () => {
        #[inline]
        fn transform(&self) -> &Self::Output {
            self
        }
        #[inline]
        fn transform_owned(self) -> Self::Output {
            self
        }
        #[inline]
        unsafe fn make(this: Self::Output) -> Self {
            this
        }
        #[inline]
        fn transform_mut<F>(&'a mut self, f: F)
        where
            F: 'static + for<'b> FnOnce(&'b mut Self::Output),
        {
            f(self)
        }
    };
}
macro_rules! impl_copy_type {
    ($ty:ident) => {
        unsafe impl<'a> Yokeable<'a> for $ty {
            type Output = Self;
            copy_yoke_impl!();
        }
        impl<'a> ZeroCopyFrom<'a, $ty> for $ty {
            #[inline]
            fn zero_copy_from(this: &'a Self) -> Self {
                // Essentially only works when the struct is fully Copy
                *this
            }
        }
        unsafe impl<'a> IsCovariant<'a> for $ty {}
    };
}

impl_copy_type!(u8);
impl_copy_type!(u16);
impl_copy_type!(u32);
impl_copy_type!(u64);
impl_copy_type!(u128);
impl_copy_type!(usize);
impl_copy_type!(i8);
impl_copy_type!(i16);
impl_copy_type!(i32);
impl_copy_type!(i64);
impl_copy_type!(i128);
impl_copy_type!(isize);
impl_copy_type!(char);
impl_copy_type!(bool);

// This is for when we're implementing Yoke on a complex type such that it's not
// obvious to the compiler that the lifetime is covariant
macro_rules! unsafe_complex_yoke_impl {
    () => {
        fn transform(&'a self) -> &'a Self::Output {
            unsafe { mem::transmute(self) }
        }

        fn transform_owned(self) -> Self::Output {
            debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
            unsafe {
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
            // Cast away the lifetime of Self
            unsafe { f(mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
        }
    };
}

unsafe impl<'a, T: 'static + for<'b> Yokeable<'b>> Yokeable<'a> for Option<T> {
    type Output = Option<<T as Yokeable<'a>>::Output>;
    unsafe_complex_yoke_impl!();
}

unsafe impl<'a, T1: 'static + for<'b> Yokeable<'b>, T2: 'static + for<'b> Yokeable<'b>> Yokeable<'a>
    for (T1, T2)
{
    type Output = (<T1 as Yokeable<'a>>::Output, <T2 as Yokeable<'a>>::Output);
    unsafe_complex_yoke_impl!();
}

unsafe impl<'a, T: Yokeable<'a>, const N: usize> Yokeable<'a> for [T; N] {
    type Output = [<T as Yokeable<'a>>::Output; N];
    unsafe_complex_yoke_impl!();
}

// This can be cleaned up once `[T; N]`::each_ref() is stabilized
// https://github.com/rust-lang/rust/issues/76118
macro_rules! array_zcf_impl {
    ($n:expr; $($i:expr),+) => {
        impl<'a, C, T: ZeroCopyFrom<'a, C>> ZeroCopyFrom<'a, [C; $n]> for [T; $n] {
            fn zero_copy_from(this: &'a [C; $n]) -> Self {
                [
                    $(
                        <T as ZeroCopyFrom<C>>::zero_copy_from(&this[$i])
                    ),+

                ]
            }
        }
    }
}

array_zcf_impl!(1; 0);
array_zcf_impl!(2; 0, 1);
array_zcf_impl!(3; 0, 1, 2);
array_zcf_impl!(4; 0, 1, 2, 3);
array_zcf_impl!(5; 0, 1, 2, 3, 4);
array_zcf_impl!(6; 0, 1, 2, 3, 4, 5);
array_zcf_impl!(7; 0, 1, 2, 3, 4, 5, 6);
array_zcf_impl!(8; 0, 1, 2, 3, 4, 5, 6, 7);
array_zcf_impl!(9; 0, 1, 2, 3, 4, 5, 6, 7, 8);
array_zcf_impl!(10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
array_zcf_impl!(11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
array_zcf_impl!(12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
array_zcf_impl!(13; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
array_zcf_impl!(14; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
array_zcf_impl!(15; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
array_zcf_impl!(16; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
