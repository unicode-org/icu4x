// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::trait_hack::YokeTraitHack;
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
/// // Reference from a borrowed version of self
/// impl<'zcf> ZeroCopyFrom<'zcf, MyStruct<'_>> for MyStruct<'zcf> {
///     fn zero_copy_from(cart: &'zcf MyStruct<'_>) -> Self {
///         MyStruct {
///             message: Cow::Borrowed(&cart.message)
///         }
///     }
/// }
///
/// // Reference from a string slice directly
/// impl<'zcf> ZeroCopyFrom<'zcf, str> for MyStruct<'zcf> {
///     fn zero_copy_from(cart: &'zcf str) -> Self {
///         MyStruct {
///             message: Cow::Borrowed(cart)
///         }
///     }
/// }
/// ```
pub trait ZeroCopyFrom<'zcf, C: ?Sized>: 'zcf {
    /// Clone the cart `C` into a struct that may retain references into `C`.
    fn zero_copy_from(cart: &'zcf C) -> Self;
}

impl<'zcf, C: ?Sized, T> ZeroCopyFrom<'zcf, C> for YokeTraitHack<T>
where
    T: ZeroCopyFrom<'zcf, C>,
{
    #[inline]
    fn zero_copy_from(cart: &'zcf C) -> Self {
        YokeTraitHack(T::zero_copy_from(cart))
    }
}

impl<Y, C> Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> YokeTraitHack<<Y as Yokeable<'a>>::Output>: ZeroCopyFrom<'a, <C as Deref>::Target>,
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
    pub fn attach_to_zero_copy_cart(cart: C) -> Self {
        Yoke::<Y, C>::attach_to_cart(cart, |c| {
            YokeTraitHack::<<Y as Yokeable>::Output>::zero_copy_from(c).0
        })
    }
}

// Note: The following could be blanket implementations, but that would require constraining the
// blanket `T` on `T: 'static`, which may not be desirable for all downstream users who may wish
// to customize their `ZeroCopyFrom` impl. The blanket implementation may be safe once Rust has
// specialization.

#[cfg(feature = "alloc")]
impl<'zcf> ZeroCopyFrom<'zcf, str> for Cow<'zcf, str> {
    #[inline]
    fn zero_copy_from(cart: &'zcf str) -> Self {
        Cow::Borrowed(cart)
    }
}

#[cfg(feature = "alloc")]
impl<'zcf> ZeroCopyFrom<'zcf, String> for Cow<'zcf, str> {
    #[inline]
    fn zero_copy_from(cart: &'zcf String) -> Self {
        Cow::Borrowed(cart)
    }
}

impl<'zcf> ZeroCopyFrom<'zcf, str> for &'zcf str {
    #[inline]
    fn zero_copy_from(cart: &'zcf str) -> Self {
        cart
    }
}

#[cfg(feature = "alloc")]
impl<'zcf> ZeroCopyFrom<'zcf, String> for &'zcf str {
    #[inline]
    fn zero_copy_from(cart: &'zcf String) -> Self {
        cart
    }
}

impl<'zcf, C, T: ZeroCopyFrom<'zcf, C>> ZeroCopyFrom<'zcf, Option<C>> for Option<T> {
    fn zero_copy_from(cart: &'zcf Option<C>) -> Self {
        cart.as_ref()
            .map(|c| <T as ZeroCopyFrom<C>>::zero_copy_from(c))
    }
}

impl<'zcf, C1, T1: ZeroCopyFrom<'zcf, C1>, C2, T2: ZeroCopyFrom<'zcf, C2>>
    ZeroCopyFrom<'zcf, (C1, C2)> for (T1, T2)
{
    fn zero_copy_from(cart: &'zcf (C1, C2)) -> Self {
        (
            <T1 as ZeroCopyFrom<C1>>::zero_copy_from(&cart.0),
            <T2 as ZeroCopyFrom<C2>>::zero_copy_from(&cart.1),
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
impl<'zcf, B: ToOwned + ?Sized> ZeroCopyFrom<'zcf, Cow<'_, B>> for Cow<'zcf, B> {
    #[inline]
    fn zero_copy_from(cart: &'zcf Cow<'_, B>) -> Self {
        Cow::Borrowed(cart)
    }
}

impl<'zcf> ZeroCopyFrom<'zcf, &'_ str> for &'zcf str {
    #[inline]
    fn zero_copy_from(cart: &'zcf &'_ str) -> &'zcf str {
        cart
    }
}

impl<'zcf, T> ZeroCopyFrom<'zcf, [T]> for &'zcf [T] {
    #[inline]
    fn zero_copy_from(cart: &'zcf [T]) -> &'zcf [T] {
        cart
    }
}
