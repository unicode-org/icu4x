// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Yoke;
use crate::Yokeable;
#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::ops::Deref;
use stable_deref_trait::StableDeref;

/// Trait for types that can be crated from a reference to a cart type `C` with no allocations.
///
/// A type can be the `ZeroCopyFrom` target of multiple cart types.
///
/// The intention is for `ZeroCopyFrom` to produce a struct from a cart with as little work as
/// possible. Although it is technically possible to implement `ZeroCopyFrom` without being
/// zero-copy (using heap allocations), doing so defeats the purpose of `ZeroCopyFrom`.
///
/// For example, `impl ZeroCopyFrom<C> for Cow<str>` should return a `Cow::Borrowed` pointing at
/// data in the cart `C`, even if the cart is itself fully owned.
///
/// # Examples
///
/// Implementing `ZeroCopyFrom` on a custom data struct:
///
/// ```
/// use yoke::Yokeable;
/// use yoke::ZeroCopyFrom;
/// use std::borrow::Cow;
///
/// struct MyStruct<'data> {
///     message: Cow<'data, str>,
/// }
///
/// unsafe impl<'a> Yokeable<'a> for MyStruct<'static> {
///     // (not shown; see `Yokeable` for examples)
/// #    type Output = MyStruct<'a>;
/// #    fn transform(&'a self) -> &'a Self::Output {
/// #        self
/// #    }
/// #    fn transform_owned(self) -> MyStruct<'a> {
/// #        // covariant lifetime cast, can be done safely
/// #        self
/// #    }
/// #    unsafe fn make(from: Self::Output) -> Self {
/// #        std::mem::transmute(from)
/// #    }
/// #    fn transform_mut<F>(&'a mut self, f: F)
/// #    where
/// #        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
/// #    {
/// #        unsafe {
/// #            f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
/// #                self,
/// #            ))
/// #        }
/// #    }
/// }
///
/// // Reference from a borrowed version of self
/// impl<'data> ZeroCopyFrom<MyStruct<'data>> for MyStruct<'static> {
///     fn zero_copy_from<'b>(cart: &'b MyStruct<'data>) -> MyStruct<'b> {
///         MyStruct {
///             message: Cow::Borrowed(&cart.message)
///         }
///     }
/// }
///
/// // Reference from a string slice directly
/// impl ZeroCopyFrom<str> for MyStruct<'static> {
///     fn zero_copy_from<'b>(cart: &'b str) -> MyStruct<'b> {
///         MyStruct {
///             message: Cow::Borrowed(cart)
///         }
///     }
/// }
/// ```
pub trait ZeroCopyFrom<C: ?Sized>: for<'a> Yokeable<'a> {
    /// Clone the cart `C` into a [`Yokeable`] struct, which may retain references into `C`.
    fn zero_copy_from<'b>(cart: &'b C) -> <Self as Yokeable<'b>>::Output;
}

impl<'b, 's, Y, C> Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a> + ZeroCopyFrom<<C as Deref>::Target>,
    C: StableDeref + Deref,
{
    /// Construct a [`Yoke`]`<Y, C>` from a cart implementing `StableDeref` by zero-copy cloning
    /// the cart to `Y` and then yokeing that object to the cart.
    ///
    /// The type `Y` must implement [`ZeroCopyFrom`]`<C::Target>`. This trait is auto-implemented
    /// on many common types and can be custom implemented or derived in order to make it easier
    /// to construct a `Yoke`.
    ///
    /// # Example
    ///
    /// Attach to a cart:
    ///
    /// ```
    /// use yoke::Yoke;
    /// use std::borrow::Cow;
    ///  
    /// let yoke = Yoke::<
    ///     Cow<'static, str>,
    ///     String
    /// >::attach_to_zero_copy_cart("demo".to_string());
    ///
    /// assert_eq!("demo", yoke.get());
    /// ```
    #[inline]
    pub fn attach_to_zero_copy_cart(cart: C) -> Self {
        Yoke::<Y, C>::attach_to_cart_badly(cart, Y::zero_copy_from)
    }
}

use crate::trait_hack::YokeTraitHack;

pub trait ZeroCopyFromV2<'a, C: ?Sized>: 'a {
    fn zero_copy_from_v2(cart: &'a C) -> Self;
}

impl<'a, C: ?Sized, T> ZeroCopyFromV2<'a, C> for YokeTraitHack<T>
where
    T: ZeroCopyFromV2<'a, C>
{
    #[inline]
    fn zero_copy_from_v2(cart: &'a C) -> Self {
        YokeTraitHack(T::zero_copy_from_v2(cart))
    }
}

impl<Y, C> Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> YokeTraitHack<<Y as Yokeable<'a>>::Output>: ZeroCopyFromV2<'a, <C as Deref>::Target>,
    C: StableDeref + Deref,
{
    /// Construct a [`Yoke`]`<Y, C>` from a cart implementing `StableDeref` by zero-copy cloning
    /// the cart to `Y` and then yokeing that object to the cart.
    ///
    /// The type `Y` must implement [`ZeroCopyFrom`]`<C::Target>`. This trait is auto-implemented
    /// on many common types and can be custom implemented or derived in order to make it easier
    /// to construct a `Yoke`.
    ///
    /// # Example
    ///
    /// Attach to a cart:
    ///
    /// ```
    /// use yoke::Yoke;
    /// use std::borrow::Cow;
    ///  
    /// let yoke = Yoke::<
    ///     Cow<'static, str>,
    ///     String
    /// >::attach_to_zero_copy_v2_cart("demo".to_string());
    ///
    /// assert_eq!("demo", yoke.get());
    /// ```
    pub fn attach_to_zero_copy_v2_cart(cart: C) -> Self {
        Yoke::<Y, C>::attach_to_cart(cart, |c| {
            YokeTraitHack::<<Y as Yokeable>::Output>::zero_copy_from_v2(c).0
        })
    }
}

// Note: The following could be blanket implementations, but that would require constraining the
// blanket `T` on `T: 'static`, which may not be desirable for all downstream users who may wish
// to customize their `ZeroCopyFrom` impl. The blanket implementation may be safe once Rust has
// specialization.

#[cfg(feature = "alloc")]
impl ZeroCopyFrom<str> for Cow<'static, str> {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b str) -> Cow<'b, str> {
        Cow::Borrowed(cart)
    }
}

#[cfg(feature = "alloc")]
impl<'a> ZeroCopyFromV2<'a, str> for Cow<'a, str> {
    #[inline]
    fn zero_copy_from_v2(cart: &'a str) -> Self {
        Cow::Borrowed(cart)
    }
}

#[cfg(feature = "alloc")]
impl ZeroCopyFrom<String> for Cow<'static, str> {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b String) -> Cow<'b, str> {
        Cow::Borrowed(cart)
    }
}

#[cfg(feature = "alloc")]
impl<'a> ZeroCopyFromV2<'a, String> for Cow<'a, str> {
    #[inline]
    fn zero_copy_from_v2(cart: &'a String) -> Self {
        Cow::Borrowed(cart)
    }
}

impl ZeroCopyFrom<str> for &'static str {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b str) -> &'b str {
        cart
    }
}

impl<'a> ZeroCopyFromV2<'a, str> for &'a str {
    #[inline]
    fn zero_copy_from_v2(cart: &'a str) -> Self {
        cart
    }
}

#[cfg(feature = "alloc")]
impl ZeroCopyFrom<String> for &'static str {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b String) -> &'b str {
        cart
    }
}

#[cfg(feature = "alloc")]
impl<'a> ZeroCopyFromV2<'a, String> for &'a str {
    #[inline]
    fn zero_copy_from_v2(cart: &'a String) -> Self {
        cart
    }
}

impl<C, T: ZeroCopyFrom<C>> ZeroCopyFrom<Option<C>> for Option<T> {
    fn zero_copy_from<'b>(cart: &'b Option<C>) -> Option<<T as Yokeable<'b>>::Output> {
        cart.as_ref()
            .map(|c| <T as ZeroCopyFrom<C>>::zero_copy_from(c))
    }
}

impl<'a, C, T: ZeroCopyFromV2<'a, C>> ZeroCopyFromV2<'a, Option<C>> for Option<T> {
    fn zero_copy_from_v2(cart: &'a Option<C>) -> Self {
        cart.as_ref()
            .map(|c| <T as ZeroCopyFromV2<C>>::zero_copy_from_v2(c))
    }
}

impl<C1, T1: ZeroCopyFrom<C1>, C2, T2: ZeroCopyFrom<C2>> ZeroCopyFrom<(C1, C2)> for (T1, T2) {
    fn zero_copy_from<'b>(
        cart: &'b (C1, C2),
    ) -> (<T1 as Yokeable<'b>>::Output, <T2 as Yokeable<'b>>::Output) {
        (
            <T1 as ZeroCopyFrom<C1>>::zero_copy_from(&cart.0),
            <T2 as ZeroCopyFrom<C2>>::zero_copy_from(&cart.1),
        )
    }
}

impl<'a, C1, T1: ZeroCopyFromV2<'a, C1>, C2, T2: ZeroCopyFromV2<'a, C2>> ZeroCopyFromV2<'a, (C1, C2)> for (T1, T2) {
    fn zero_copy_from_v2(
        cart: &'a (C1, C2),
    ) -> Self {
        (
            <T1 as ZeroCopyFromV2<C1>>::zero_copy_from_v2(&cart.0),
            <T2 as ZeroCopyFromV2<C2>>::zero_copy_from_v2(&cart.1),
        )
    }
}

// These duplicate the functionality from above and aren't quite necessary due
// to deref coercions, however for the custom derive to work, there always needs
// to be `impl ZCF<T> for T`, otherwise it may fail to perform the necessary
// type inference. Deref coercions do not typically work when sufficient generics
// or inference are involved, and the proc macro does not necessarily have
// enough type information to figure this out on its own.
#[cfg(feature = "alloc")]
impl<B: ToOwned + ?Sized + 'static> ZeroCopyFrom<Cow<'_, B>> for Cow<'static, B> {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b Cow<'_, B>) -> Cow<'b, B> {
        Cow::Borrowed(cart)
    }
}

#[cfg(feature = "alloc")]
impl<'a, B: ToOwned + ?Sized> ZeroCopyFromV2<'a, Cow<'_, B>> for Cow<'a, B> {
    #[inline]
    fn zero_copy_from_v2(cart: &'a Cow<'_, B>) -> Self {
        Cow::Borrowed(cart)
    }
}

impl ZeroCopyFrom<&'_ str> for &'static str {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b &'_ str) -> &'b str {
        cart
    }
}

impl<'a> ZeroCopyFromV2<'a, &'_ str> for &'a str {
    #[inline]
    fn zero_copy_from_v2(cart: &'a &'_ str) -> &'a str {
        cart
    }
}

impl<T> ZeroCopyFrom<[T]> for &'static [T] {
    #[inline]
    fn zero_copy_from<'b>(cart: &'b [T]) -> &'b [T] {
        cart
    }
}

impl<'a, T> ZeroCopyFromV2<'a, [T]> for &'a [T] {
    #[inline]
    fn zero_copy_from_v2(cart: &'a [T]) -> &'a [T] {
        cart
    }
}

#[test]
fn test_v2() {
    let yoke = Yoke::<
        Cow<'static, str>,
        String
    >::attach_to_zero_copy_v2_cart("demo".to_string());
    assert_eq!("demo", yoke.get());
}
