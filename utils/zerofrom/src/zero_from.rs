// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};
#[cfg(feature = "alloc")]
use alloc::string::String;

/// Trait for types that can be created from a reference to a cart type `C` with no allocations,
/// i.e. a zero-copy (zero-alloc) version of "From"
///
/// A type can be the `ZeroFrom` target of multiple cart types.
///
/// The intention is for `ZeroFrom` to produce a struct from a cart with as little work as
/// possible. Although it is technically possible to implement `ZeroFrom` without being
/// zero-copy (using heap allocations), doing so defeats the purpose of `ZeroFrom`.
///
/// For example, `impl ZeroFrom<C> for Cow<str>` should return a `Cow::Borrowed` pointing at
/// data in the cart `C`, even if the cart is itself fully owned.
///
/// One can use the [`#[derive(ZeroFrom)]`](yoke_derive::ZeroFrom) custom derive to automatically
/// implement this trait.
///
/// # Examples
///
/// Implementing `ZeroFrom` on a custom data struct:
///
/// ```
/// use zerofrom::ZeroFrom;
/// use std::borrow::Cow;
///
/// struct MyStruct<'data> {
///     message: Cow<'data, str>,
/// }
///
/// // Reference from a borrowed version of self
/// impl<'zcf> ZeroFrom<'zcf, MyStruct<'_>> for MyStruct<'zcf> {
///     fn zero_from(cart: &'zcf MyStruct<'_>) -> Self {
///         MyStruct {
///             message: Cow::Borrowed(&cart.message)
///         }
///     }
/// }
///
/// // Reference from a string slice directly
/// impl<'zcf> ZeroFrom<'zcf, str> for MyStruct<'zcf> {
///     fn zero_from(cart: &'zcf str) -> Self {
///         MyStruct {
///             message: Cow::Borrowed(cart)
///         }
///     }
/// }
/// ```
pub trait ZeroFrom<'zcf, C: ?Sized>: 'zcf {
    /// Clone the cart `C` into a struct that may retain references into `C`.
    fn zero_from(cart: &'zcf C) -> Self;
}

// Note: The following could be blanket implementations, but that would require constraining the
// blanket `T` on `T: 'static`, which may not be desirable for all downstream users who may wish
// to customize their `ZeroFrom` impl. The blanket implementation may be safe once Rust has
// specialization.

#[cfg(feature = "alloc")]
impl<'zcf> ZeroFrom<'zcf, str> for Cow<'zcf, str> {
    #[inline]
    fn zero_from(cart: &'zcf str) -> Self {
        Cow::Borrowed(cart)
    }
}

#[cfg(feature = "alloc")]
impl<'zcf> ZeroFrom<'zcf, String> for Cow<'zcf, str> {
    #[inline]
    fn zero_from(cart: &'zcf String) -> Self {
        Cow::Borrowed(cart)
    }
}

impl<'zcf> ZeroFrom<'zcf, str> for &'zcf str {
    #[inline]
    fn zero_from(cart: &'zcf str) -> Self {
        cart
    }
}

#[cfg(feature = "alloc")]
impl<'zcf> ZeroFrom<'zcf, String> for &'zcf str {
    #[inline]
    fn zero_from(cart: &'zcf String) -> Self {
        cart
    }
}

impl<'zcf, C, T: ZeroFrom<'zcf, C>> ZeroFrom<'zcf, Option<C>> for Option<T> {
    fn zero_from(cart: &'zcf Option<C>) -> Self {
        cart.as_ref().map(|c| <T as ZeroFrom<C>>::zero_from(c))
    }
}

impl<'zcf, C1, T1: ZeroFrom<'zcf, C1>, C2, T2: ZeroFrom<'zcf, C2>> ZeroFrom<'zcf, (C1, C2)>
    for (T1, T2)
{
    fn zero_from(cart: &'zcf (C1, C2)) -> Self {
        (
            <T1 as ZeroFrom<C1>>::zero_from(&cart.0),
            <T2 as ZeroFrom<C2>>::zero_from(&cart.1),
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
impl<'zcf, B: ToOwned + ?Sized> ZeroFrom<'zcf, Cow<'_, B>> for Cow<'zcf, B> {
    #[inline]
    fn zero_from(cart: &'zcf Cow<'_, B>) -> Self {
        Cow::Borrowed(cart)
    }
}

impl<'zcf> ZeroFrom<'zcf, &'_ str> for &'zcf str {
    #[inline]
    fn zero_from(cart: &'zcf &'_ str) -> &'zcf str {
        cart
    }
}

impl<'zcf, T> ZeroFrom<'zcf, [T]> for &'zcf [T] {
    #[inline]
    fn zero_from(cart: &'zcf [T]) -> &'zcf [T] {
        cart
    }
}
