// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
};

/// A type implementing `IsCovariant<'a>` is covariant with respect to lifetime `'a`.
///
/// Lifetime parameters that are safely cast in [`Yokeable`] are also valid for `IsCovariant`.
///
/// `IsCovariant` exists primarily to serve in trait bounds. The primary use case is to safely
/// perform lifetime casting on trait objects (`dyn Trait`). This enables a type-erased [`Yoke`]
/// consisting of only trait objects. See the examples.
///
/// `IsCovariant` is auto-implemented in [`#[derive(Yokeable)]`](macro@crate::Yokeable).
///
/// # Safety
///
/// This trait is safe to implement on types with a _covariant_ lifetime parameter. This will
/// occur when the lifetime parameter is used within references, but not in the arguments of
/// function pointers or in mutable positions (either in `&mut` or via interior mutability).
///
/// If a struct has multiple lifetime parameters, only the one used in `IsCovariant<'a>` needs to
/// be covariant.
///
/// # Examples
///
/// Implementing on a simple struct with a single covariant lifetime:
///
/// ```
/// # use yoke::*;
/// struct MyStruct<'a>(&'a str);
///
/// // This is safe because 'a is covariant
/// unsafe impl<'a> IsCovariant<'a> for MyStruct<'a> {}
/// ```
///
/// By constraining the trait `ExampleTrait<'a>` on `IsCovariant<'a>`, we can safely implement
/// [`Yokeable`] and [`ZeroCopyFrom`] on its trait object:
///
/// ```
/// # use yoke::*;
/// # use core::mem;
/// trait ExampleTrait<'a>: IsCovariant<'a> {
///     fn get_message(&self) -> &'a str;
/// }
///
/// // This wrapper is required because of the blanket Yokeable impl on &'static T
/// pub struct ExampleTraitDynRef<'a>(pub &'a dyn ExampleTrait<'a>);
///
/// // The following impl is safe because the trait object requires IsCovariant.
/// unsafe impl<'a> Yokeable<'a> for ExampleTraitDynRef<'static> {
///     type Output = ExampleTraitDynRef<'a>;
///     fn transform(&'a self) -> &'a Self::Output {
///         unsafe { mem::transmute(self) }
///     }
///
///     fn transform_owned(self) -> Self::Output {
///         unsafe { mem::transmute(self) }
///     }
///
///     unsafe fn make(from: Self::Output) -> Self {
///         unsafe { mem::transmute(from) }
///     }
///
///     fn transform_mut<F>(&'a mut self, f: F)
///     where
///         F: 'static + FnOnce(&'a mut Self::Output),
///     {
///         unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
///     }
/// }
///
/// impl<'zcf, 'a> ZeroCopyFrom<'zcf, dyn ExampleTrait<'a> + 'a> for ExampleTraitDynRef<'zcf> {
///     fn zero_copy_from(this: &'zcf (dyn ExampleTrait<'a> + 'a)) -> ExampleTraitDynRef<'zcf> {
///         // This is safe because the trait object requires IsCovariant.
///         ExampleTraitDynRef(unsafe { core::mem::transmute(this) })
///     }
/// }
///
/// // Implement ExampleTrait on the struct from the previous example
/// # struct MyStruct<'a>(&'a str);
/// # unsafe impl<'a> IsCovariant<'a> for MyStruct<'a> {}
/// impl<'a> ExampleTrait<'a> for MyStruct<'a> {
///     fn get_message(&self) -> &'a str {
///         self.0
///     }
/// }
///
/// // Example usage: a Yoke of a trait object
/// let s = "Hello World".to_string();
/// let yoke: Yoke<ExampleTraitDynRef<'static>, Box<dyn ExampleTrait>> =
///     Yoke::attach_to_zero_copy_cart(Box::new(MyStruct(&s)));
///
/// assert_eq!(yoke.get().0.get_message(), "Hello World");
/// ```
///
/// [`Yoke`]: crate::Yoke
/// [`Yokeable`]: crate::Yokeable
/// [`ZeroCopyFrom`]: crate::ZeroCopyFrom
pub unsafe trait IsCovariant<'a>: 'a {}

// IsCovariant is implemented on the standard library Copy types in macro_impls.rs

// The following impls are safe because there is only one lifetime, 'a, and 'a is covariant

unsafe impl<'a> IsCovariant<'a> for () {}

unsafe impl<'a> IsCovariant<'a> for str {}
#[cfg(feature = "alloc")]
unsafe impl<'a> IsCovariant<'a> for String {}

unsafe impl<'a, T: IsCovariant<'a>> IsCovariant<'a> for Option<T> {}

unsafe impl<'a, T1: IsCovariant<'a>, T2: IsCovariant<'a>> IsCovariant<'a> for (T1, T2) {}

unsafe impl<'a, T: IsCovariant<'a>> IsCovariant<'a> for [T] {}

unsafe impl<'a, T: IsCovariant<'a>, const N: usize> IsCovariant<'a> for [T; N] {}

#[cfg(feature = "alloc")]
unsafe impl<'a, T: IsCovariant<'a> + ?Sized> IsCovariant<'a> for Box<T> {}

#[cfg(feature = "alloc")]
unsafe impl<'a, T: IsCovariant<'a> + ?Sized> IsCovariant<'a> for Rc<T> {}

// This is safe because T has a covariant lifetime, and Cow's lifetime is also covariant
#[cfg(feature = "alloc")]
unsafe impl<'a, T: IsCovariant<'a> + ToOwned + ?Sized> IsCovariant<'a> for Cow<'a, T> where
    <T as ToOwned>::Owned: Sized
{
}

// This is safe because T has a covariant lifetime, and the reference lifetime is also covariant
unsafe impl<'a, T: IsCovariant<'a> + ?Sized> IsCovariant<'a> for &'a T {}
