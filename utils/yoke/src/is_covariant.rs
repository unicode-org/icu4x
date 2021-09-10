// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
};

/// A type implementing `IsCovariant<'a>` is covariant with respect to lifetime `'a`.
///
/// All types on which it is safe to implement [`Yokeable`] can also safely
/// implement `IsCovariant`. The difference is that `Yokeable` is defined on the `'static` version
/// of the type, whereas `IsCovariant` is defined on the `'a` version of the type. This makes it
/// useful in trait bounds when a type needs to be covariant for another unsafe operation.
///
/// The primary use case is to safely perform lifetime casting on trait objects (`dyn Trait`).
///
/// # Implementation safety
///
/// See the notes in [`Yokeable`].
///
/// # Example
///
/// The trait is safe to implement on types whose lifetime parameter is covariant:
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
/// [`Yokeable`] on its trait object.
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
/// // The following impl is safe because the lifetime in the trait object is covariant.
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
/// let data = MyStruct(&s);
/// let yoke: Yoke<ExampleTraitDynRef<'static>, Box<MyStruct>> = Yoke::attach_to_cart_badly(
///     Box::new(data),
///     |obj| {
///         ExampleTraitDynRef(obj)
///     });
///
/// assert_eq!(yoke.get().0.get_message(), "Hello World");
/// ```
pub unsafe trait IsCovariant<'a>: 'a {}

// IsCovariant is implemented on the standard library Copy types in macro_impls.rs

// The following impls are safe because there is only one lifetime, 'a, and 'a is covariant

unsafe impl<'a> IsCovariant<'a> for () {}

unsafe impl<'a> IsCovariant<'a> for str {}

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
