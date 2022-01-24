// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use crate::erased::{ErasedBoxCart, ErasedRcCart};
use crate::trait_hack::YokeTraitHack;
use crate::IsCovariant;
use crate::Yokeable;
use core::marker::PhantomData;
use core::ops::Deref;
use stable_deref_trait::StableDeref;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use alloc::sync::Arc;

/// A Cow-like borrowed object "yoked" to its backing data.
///
/// This allows things like zero copy deserialized data to carry around
/// shared references to their backing buffer.
///
/// `Y` (the [`Yokeable`]) is the object containing the references,
/// and will typically be of the form `Foo<'static>`. The `'static` is
/// not the actual lifetime of the data, rather it is a convenient way to erase
/// the lifetime and make it dynamic.
///
/// `C` is the "cart", which `Y` may contain references to. After the yoke is constructed,
/// the cart serves little purpose except to guarantee that `Y`'s references remain valid
/// for as long as the yoke remains in memory (by calling the destructor at the appropriate moment).
///
/// The primary constructor for [`Yoke`] is [`Yoke::attach_to_cart()`]. Several variants of that
/// constructor are provided to serve numerous types of call sites and `Yoke` signatures.
///
/// In general, `C` is a concrete type, but it is also possible for it to be a trait object;
/// for more information, see [`IsCovariant`].
///
/// # Example
///
/// For example, we can use this to store zero-copy deserialized data in a cache:
///
/// ```rust
/// # use yoke::{Yoke, Yokeable};
/// # use std::rc::Rc;
/// # use std::borrow::Cow;
/// # fn load_from_cache(_filename: &str) -> Rc<[u8]> {
/// #     // dummy implementation
/// #     Rc::new([0x5, 0, 0, 0, 0, 0, 0, 0, 0x68, 0x65, 0x6c, 0x6c, 0x6f])
/// # }
///
/// fn load_object(filename: &str) -> Yoke<Cow<'static, str>, Rc<[u8]>> {
///     let rc: Rc<[u8]> = load_from_cache(filename);
///     Yoke::<Cow<'static, str>, Rc<[u8]>>::attach_to_cart_badly(rc, |data: &[u8]| {
///         // essentially forcing a #[serde(borrow)]
///         Cow::Borrowed(bincode::deserialize(data).unwrap())
///     })
/// }
///
/// let yoke = load_object("filename.bincode");
/// assert_eq!(&**yoke.get(), "hello");
/// assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
/// ```
///
pub struct Yoke<Y: for<'a> Yokeable<'a>, C> {
    // must be the first field for drop order
    // this will have a 'static lifetime parameter, that parameter is a lie
    yokeable: Y,
    cart: C,
}

impl<Y: for<'a> Yokeable<'a>, C: StableDeref> Yoke<Y, C> {
    /// Construct a [`Yoke`] by yokeing an object to a cart in a closure.
    ///
    /// See also [`Yoke::try_attach_to_cart()`] to return a `Result` from the closure.
    ///
    /// Due to [compiler bug #84937](https://github.com/rust-lang/rust/issues/84937), call sites
    /// for this function may not compile; if this happens, use
    /// [`Yoke::attach_to_cart_badly()`] instead.
    pub fn attach_to_cart<F>(cart: C, f: F) -> Self
    where
        F: for<'de> FnOnce(&'de <C as Deref>::Target) -> <Y as Yokeable<'de>>::Output,
    {
        let deserialized = f(cart.deref());
        Self {
            yokeable: unsafe { Y::make(deserialized) },
            cart,
        }
    }

    /// Construct a [`Yoke`] by yokeing an object to a cart. If an error occurs in the
    /// deserializer function, the error is passed up to the caller.
    ///
    /// Due to [compiler bug #84937](https://github.com/rust-lang/rust/issues/84937), call sites
    /// for this function may not compile; if this happens, use
    /// [`Yoke::try_attach_to_cart_badly()`] instead.
    pub fn try_attach_to_cart<E, F>(cart: C, f: F) -> Result<Self, E>
    where
        F: for<'de> FnOnce(&'de <C as Deref>::Target) -> Result<<Y as Yokeable<'de>>::Output, E>,
    {
        let deserialized = f(cart.deref())?;
        Ok(Self {
            yokeable: unsafe { Y::make(deserialized) },
            cart,
        })
    }

    /// Construct a [`Yoke`] by yokeing an object to a cart in a closure.
    ///
    /// For a version of this function that takes a `FnOnce` instead of a raw function pointer,
    /// see [`Yoke::attach_to_cart()`].
    ///
    /// # Example
    ///
    /// For example, we can use this to store zero-copy deserialized data in a cache:
    ///
    /// ```rust
    /// # use yoke::{Yoke, Yokeable};
    /// # use std::rc::Rc;
    /// # use std::borrow::Cow;
    /// # fn load_from_cache(_filename: &str) -> Rc<[u8]> {
    /// #     // dummy implementation
    /// #     Rc::new([0x5, 0, 0, 0, 0, 0, 0, 0, 0x68, 0x65, 0x6c, 0x6c, 0x6f])
    /// # }
    ///
    /// fn load_object(filename: &str) -> Yoke<Cow<'static, str>, Rc<[u8]>> {
    ///     let rc: Rc<[u8]> = load_from_cache(filename);
    ///     Yoke::<Cow<'static, str>, Rc<[u8]>>::attach_to_cart_badly(rc, |data: &[u8]| {
    ///         // essentially forcing a #[serde(borrow)]
    ///         Cow::Borrowed(bincode::deserialize(data).unwrap())
    ///     })
    /// }
    ///
    /// let yoke: Yoke<Cow<str>, _> = load_object("filename.bincode");
    /// assert_eq!(&**yoke.get(), "hello");
    /// assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
    /// ```
    pub fn attach_to_cart_badly(
        cart: C,
        f: for<'de> fn(&'de <C as Deref>::Target) -> <Y as Yokeable<'de>>::Output,
    ) -> Self {
        let deserialized = f(cart.deref());
        Self {
            yokeable: unsafe { Y::make(deserialized) },
            cart,
        }
    }

    /// Construct a [`Yoke`] by yokeing an object to a cart. If an error occurs in the
    /// deserializer function, the error is passed up to the caller.
    ///
    /// For a version of this function that takes a `FnOnce` instead of a raw function pointer,
    /// see [`Yoke::try_attach_to_cart()`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use yoke::{Yoke, Yokeable};
    /// # use std::rc::Rc;
    /// # use std::borrow::Cow;
    /// let rc = Rc::new([0xb, 0xa, 0xd]);
    ///
    /// let yoke_result: Result<Yoke<Cow<str>, Rc<[u8]>>, _> =
    ///     Yoke::try_attach_to_cart_badly(rc, |data: &[u8]| {
    ///         bincode::deserialize(data)
    ///     });
    ///
    /// assert!(matches!(yoke_result, Err(_)));
    /// ```
    pub fn try_attach_to_cart_badly<E>(
        cart: C,
        f: for<'de> fn(&'de <C as Deref>::Target) -> Result<<Y as Yokeable<'de>>::Output, E>,
    ) -> Result<Self, E> {
        let deserialized = f(cart.deref())?;
        Ok(Self {
            yokeable: unsafe { Y::make(deserialized) },
            cart,
        })
    }
}

impl<Y: for<'a> Yokeable<'a>, C> Yoke<Y, C> {
    /// Obtain a valid reference to the yokeable data
    ///
    /// This essentially transforms the lifetime of the internal yokeable data to
    /// be valid.
    /// For example, if you're working with a `Yoke<Cow<'static, T>, C>`, this
    /// will return an `&'a Cow<'a, T>`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use yoke::{Yoke, Yokeable};
    /// # use std::rc::Rc;
    /// # use std::borrow::Cow;
    /// # fn load_from_cache(_filename: &str) -> Rc<[u8]> {
    /// #     // dummy implementation
    /// #     Rc::new([0x5, 0, 0, 0, 0, 0, 0, 0, 0x68, 0x65, 0x6c, 0x6c, 0x6f])
    /// # }
    /// #
    /// # fn load_object(filename: &str) -> Yoke<Cow<'static, str>, Rc<[u8]>> {
    /// #     let rc: Rc<[u8]> = load_from_cache(filename);
    /// #     Yoke::<Cow<'static, str>, Rc<[u8]>>::attach_to_cart_badly(rc, |data: &[u8]| {
    /// #         Cow::Borrowed(bincode::deserialize(data).unwrap())
    /// #     })
    /// # }
    ///
    /// // load_object() defined in the example at the top of this page
    /// let yoke: Yoke<Cow<str>, _> = load_object("filename.bincode");
    /// assert_eq!(yoke.get(), "hello");
    /// ```
    #[inline]
    pub fn get<'a>(&'a self) -> &'a <Y as Yokeable<'a>>::Output {
        self.yokeable.transform()
    }

    /// Get a reference to the backing cart.
    ///
    /// This can be useful when building caches, etc. However, if you plan to store the cart
    /// separately from the yoke, read the note of caution below in [`Yoke::into_backing_cart`].
    pub fn backing_cart(&self) -> &C {
        &self.cart
    }

    /// Get the backing cart by value, dropping the yokeable object.
    ///
    /// **Caution:** Calling this method could cause information saved in the yokeable object but
    /// not the cart to be lost. Use this method only if the yokeable object cannot contain its
    /// own information.
    ///
    /// # Example
    ///
    /// Good example: the yokeable object is only a reference, so no information can be lost.
    ///
    /// ```
    /// use yoke::Yoke;
    ///
    /// let local_data = "foo".to_string();
    /// let yoke = Yoke::<
    ///     &'static str,
    ///     Box<String>
    /// >::attach_to_zero_copy_cart(Box::new(local_data));
    /// assert_eq!(*yoke.get(), "foo");
    ///
    /// // Get back the cart
    /// let cart = yoke.into_backing_cart();
    /// assert_eq!(&*cart, "foo");
    /// ```
    ///
    /// Bad example: information specified in `.with_mut()` is lost.
    ///
    /// ```
    /// use yoke::Yoke;
    /// use std::borrow::Cow;
    ///
    /// let local_data = "foo".to_string();
    /// let mut yoke = Yoke::<
    ///     Cow<'static, str>,
    ///     Box<String>
    /// >::attach_to_zero_copy_cart(Box::new(local_data));
    /// assert_eq!(yoke.get(), "foo");
    ///
    /// // Override data in the cart
    /// yoke.with_mut(|cow| {
    ///     let mut_str = cow.to_mut();
    ///     mut_str.clear();
    ///     mut_str.push_str("bar");
    /// });
    /// assert_eq!(yoke.get(), "bar");
    ///
    /// // Get back the cart
    /// let cart = yoke.into_backing_cart();
    /// assert_eq!(&*cart, "foo"); // WHOOPS!
    /// ```
    pub fn into_backing_cart(self) -> C {
        self.cart
    }

    /// Unsafe function for replacing the cart with another
    ///
    /// This can be used for type-erasing the cart, for example.
    ///
    /// # Safety
    ///
    /// - `f()` must not panic
    /// - References from the yokeable `Y` should still be valid for the lifetime of the
    ///   returned cart type.
    ///
    /// Typically, this means implementing `f` as something which _wraps_ the inner cart type.
    /// `Yoke` only really cares about destructors for its carts so it's fine to erase other
    /// information about the cart, as long as the backing data will still be destroyed at the
    /// same time.
    #[inline]
    pub unsafe fn replace_cart<C2>(self, f: impl FnOnce(C) -> C2) -> Yoke<Y, C2> {
        Yoke {
            yokeable: self.yokeable,
            cart: f(self.cart),
        }
    }

    /// Mutate the stored [`Yokeable`] data.
    ///
    /// See [`Yokeable::transform_mut()`] for why this operation is safe.
    ///
    /// # Example
    ///
    /// This can be used to partially mutate the stored data, provided
    /// no _new_ borrowed data is introduced.
    ///
    /// ```rust
    /// # use yoke::{Yoke, Yokeable};
    /// # use std::rc::Rc;
    /// # use std::borrow::Cow;
    /// # use std::mem;
    /// # fn load_from_cache(_filename: &str) -> Rc<[u8]> {
    /// #     // dummy implementation
    /// #     Rc::new([0x5, 0, 0, 0, 0, 0, 0, 0, 0x68, 0x65, 0x6c, 0x6c, 0x6f])
    /// # }
    /// #
    /// # fn load_object(filename: &str) -> Yoke<Bar<'static>, Rc<[u8]>> {
    /// #     let rc: Rc<[u8]> = load_from_cache(filename);
    /// #     Yoke::<Bar<'static>, Rc<[u8]>>::attach_to_cart_badly(rc, |data: &[u8]| {
    /// #         // A real implementation would properly deserialize `Bar` as a whole
    /// #         Bar {
    /// #             numbers: Cow::Borrowed(bincode::deserialize(data).unwrap()),
    /// #             string: Cow::Borrowed(bincode::deserialize(data).unwrap()),
    /// #             owned: Vec::new(),
    /// #         }
    /// #     })
    /// # }
    ///
    /// // also implements Yokeable
    /// struct Bar<'a> {
    ///     numbers: Cow<'a, [u8]>,
    ///     string: Cow<'a, str>,
    ///     owned: Vec<u8>,
    /// }
    ///
    /// // `load_object()` deserializes an object from a file
    /// let mut bar: Yoke<Bar, _> = load_object("filename.bincode");
    /// assert_eq!(bar.get().string, "hello");
    /// assert!(matches!(bar.get().string, Cow::Borrowed(_)));
    /// assert_eq!(&*bar.get().numbers, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    /// assert!(matches!(bar.get().numbers, Cow::Borrowed(_)));
    /// assert_eq!(&*bar.get().owned, &[]);
    ///
    /// bar.with_mut(|bar| {
    ///     bar.string.to_mut().push_str(" world");
    ///     bar.owned.extend_from_slice(&[1, 4, 1, 5, 9]);   
    /// });
    ///
    /// assert_eq!(bar.get().string, "hello world");
    /// assert!(matches!(bar.get().string, Cow::Owned(_)));
    /// assert_eq!(&*bar.get().owned, &[1, 4, 1, 5, 9]);
    /// // Unchanged and still Cow::Borrowed
    /// assert_eq!(&*bar.get().numbers, &[0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    /// assert!(matches!(bar.get().numbers, Cow::Borrowed(_)));
    ///
    /// # unsafe impl<'a> Yokeable<'a> for Bar<'static> {
    /// #     type Output = Bar<'a>;
    /// #     fn transform(&'a self) -> &'a Bar<'a> {
    /// #         self
    /// #     }
    /// #
    /// #     fn transform_owned(self) -> Bar<'a> {
    /// #         // covariant lifetime cast, can be done safely
    /// #         self
    /// #     }
    /// #
    /// #     unsafe fn make(from: Bar<'a>) -> Self {
    /// #         let ret = mem::transmute_copy(&from);
    /// #         mem::forget(from);
    /// #         ret
    /// #     }
    /// #
    /// #     fn transform_mut<F>(&'a mut self, f: F)
    /// #     where
    /// #         F: 'static + FnOnce(&'a mut Self::Output),
    /// #     {
    /// #         unsafe { f(mem::transmute(self)) }
    /// #     }
    /// # }
    /// ```
    pub fn with_mut<'a, F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut <Y as Yokeable<'a>>::Output),
    {
        self.yokeable.transform_mut(f)
    }

    /// Helper function allowing one to wrap the cart type in an `Option<T>`.
    #[inline]
    pub fn wrap_cart_in_option(self) -> Yoke<Y, Option<C>> {
        unsafe {
            // safe because the cart is preserved, just wrapped
            self.replace_cart(Some)
        }
    }
}

impl<Y: for<'a> Yokeable<'a>> Yoke<Y, ()> {
    /// Construct a new [`Yoke`] from static data. There will be no
    /// references to `cart` here since [`Yokeable`]s are `'static`,
    /// this is good for e.g. constructing fully owned
    /// [`Yoke`]s with no internal borrowing.
    ///
    /// This is similar to [`Yoke::new_owned()`] but it does not allow you to
    /// mix the [`Yoke`] with borrowed data. This is primarily useful
    /// for using [`Yoke`] in generic scenarios.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use yoke::Yoke;
    /// # use std::borrow::Cow;
    /// # use std::rc::Rc;
    ///
    /// let owned: Cow<str> = "hello".to_owned().into();
    /// // this yoke can be intermingled with actually-borrowed Yokes
    /// let yoke: Yoke<Cow<str>, ()> = Yoke::new_always_owned(owned);
    ///
    /// assert_eq!(yoke.get(), "hello");
    /// ```
    pub fn new_always_owned(yokeable: Y) -> Self {
        Self { yokeable, cart: () }
    }

    /// Obtain the yokeable out of a `Yoke<Y, ()>`
    ///
    /// For most `Yoke` types this would be unsafe but it's
    /// fine for `Yoke<Y, ()>` since there are no actual internal
    /// references
    pub fn into_yokeable(self) -> Y {
        self.yokeable
    }
}

impl<Y: for<'a> Yokeable<'a>, C: StableDeref> Yoke<Y, Option<C>> {
    /// Construct a new [`Yoke`] from static data. There will be no
    /// references to `cart` here since [`Yokeable`]s are `'static`,
    /// this is good for e.g. constructing fully owned
    /// [`Yoke`]s with no internal borrowing.
    ///
    /// This can be paired with [`Yoke:: wrap_cart_in_option()`] to mix owned
    /// and borrowed data.
    ///
    /// If you do not wish to pair this with borrowed data, [`Yoke::new_always_owned()`] can
    /// be used to get a [`Yoke`] API on always-owned data.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use yoke::Yoke;
    /// # use std::borrow::Cow;
    /// # use std::rc::Rc;
    ///
    /// let owned: Cow<str> = "hello".to_owned().into();
    /// // this yoke can be intermingled with actually-borrowed Yokes
    /// let yoke: Yoke<Cow<str>, Option<Rc<[u8]>>> = Yoke::new_owned(owned);
    ///
    /// assert_eq!(yoke.get(), "hello");
    /// ```
    pub fn new_owned(yokeable: Y) -> Self {
        Self {
            yokeable,
            cart: None,
        }
    }

    /// Obtain the yokeable out of a `Yoke<Y, Option<C>>` if possible.
    ///
    /// If the cart is `None`, this returns `Some`, but if the cart is `Some`,
    /// this returns `self` as an error.
    pub fn try_into_yokeable(self) -> Result<Y, Self> {
        match self.cart {
            Some(_) => Err(self),
            None => Ok(self.yokeable),
        }
    }
}

/// This trait marks cart types that do not change source on cloning
///
/// This is conceptually similar to [`stable_deref_trait::CloneStableDeref`],
/// however [`stable_deref_trait::CloneStableDeref`] is not (and should not) be
/// implemented on [`Option`] (since it's not [`Deref`]). [`CloneableCart`] essentially is
/// "if there _is_ data to borrow from here, cloning the cart gives you an additional
/// handle to the same data".
///
/// # Safety
/// This trait is safe to implement `StableDeref` types which, once `Clone`d, point to the same underlying data.
///
/// (This trait is also implemented on `Option<T>` and `()`, which are the two non-`StableDeref` cart types that
/// Yokes can be constructed for)
pub unsafe trait CloneableCart: Clone {}

#[cfg(feature = "alloc")]
unsafe impl<T: ?Sized> CloneableCart for Rc<T> {}
#[cfg(feature = "alloc")]
unsafe impl<T: ?Sized> CloneableCart for Arc<T> {}
unsafe impl<T: CloneableCart> CloneableCart for Option<T> {}
unsafe impl<'a, T: ?Sized> CloneableCart for &'a T {}
unsafe impl CloneableCart for () {}

/// Clone requires that the cart derefs to the same address after it is cloned. This works for
/// Rc, Arc, and &'a T.
///
/// For other cart types, clone `.backing_cart()` and re-use `.attach_to_cart()`; however, doing
/// so may lose mutations performed via `.with_mut()`.
///
/// Cloning a `Yoke` is often a cheap operation requiring no heap allocations, in much the same
/// way that cloning an `Rc` is a cheap operation. However, if the `yokeable` contains owned data
/// (e.g., from `.with_mut()`), that data will need to be cloned.
impl<Y: for<'a> Yokeable<'a>, C: CloneableCart> Clone for Yoke<Y, C>
where
    for<'a> YokeTraitHack<<Y as Yokeable<'a>>::Output>: Clone,
{
    fn clone(&self) -> Self {
        let this: &Y::Output = self.get();
        // We have an &T not a T, and we can clone YokeTraitHack<T>
        let this_hack = YokeTraitHack(this).into_ref();
        Yoke {
            yokeable: unsafe { Y::make(this_hack.clone().0) },
            cart: self.cart.clone(),
        }
    }
}

// This is safe because Y is 'static and C has a covariant lifetime
unsafe impl<'b, Y: for<'a> Yokeable<'a>, C: IsCovariant<'b>> IsCovariant<'b> for Yoke<Y, C> {}

#[test]
fn test_clone() {
    let local_data = "foo".to_string();
    let y1 = Yoke::<alloc::borrow::Cow<'static, str>, Rc<String>>::attach_to_zero_copy_cart(
        Rc::new(local_data),
    );

    // Test basic clone
    let y2 = y1.clone();
    assert_eq!(y1.get(), "foo");
    assert_eq!(y2.get(), "foo");

    // Test clone with mutation on target
    let mut y3 = y1.clone();
    y3.with_mut(|y| {
        y.to_mut().push_str("bar");
    });
    assert_eq!(y1.get(), "foo");
    assert_eq!(y2.get(), "foo");
    assert_eq!(y3.get(), "foobar");

    // Test that mutations on source do not affect target
    let y4 = y3.clone();
    y3.with_mut(|y| {
        y.to_mut().push_str("baz");
    });
    assert_eq!(y1.get(), "foo");
    assert_eq!(y2.get(), "foo");
    assert_eq!(y3.get(), "foobarbaz");
    assert_eq!(y4.get(), "foobar");
}

impl<Y: for<'a> Yokeable<'a>, C> Yoke<Y, C> {
    /// Allows one to "project" a yoke to perform a transformation on the data, potentially
    /// looking at a subfield, and producing a new yoke. This will move cart, and the provided
    /// transformation is only allowed to use data known to be borrowed from the cart.
    ///
    /// This takes an additional `PhantomData<&()>` parameter as a workaround to the issue
    /// described in [#86702](https://github.com/rust-lang/rust/issues/86702). This parameter
    /// should just be ignored in the function.
    ///
    /// Furthermore,
    /// [compiler bug #84937](https://github.com/rust-lang/rust/issues/84937) prevents
    /// this from taking a capturing closure, however [`Yoke::project_with_capture()`]
    /// can be used for the same use cases.
    ///
    ///
    /// This can be used, for example, to transform data from one format to another:
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.56.***
    ///
    /// ```rust,ignore
    /// # use std::rc::Rc;
    /// # use yoke::Yoke;
    /// #
    /// fn slice(y: Yoke<&'static str, Rc<[u8]>>) -> Yoke<&'static [u8], Rc<[u8]>> {
    ///    y.project(move |yk, _| yk.as_bytes())
    /// }
    ///
    /// ```
    ///
    /// This can also be used to create a yoke for a subfield
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.56.***
    ///
    /// ```rust,ignore
    /// # use std::borrow::Cow;
    /// # use yoke::{Yoke, Yokeable};
    /// # use std::mem;
    /// # use std::rc::Rc;
    /// #
    /// // also safely implements Yokeable<'a>
    /// struct Bar<'a> {
    ///     string_1: &'a str,
    ///     string_2: &'a str,
    /// }
    ///
    /// fn project_string_1(bar: Yoke<Bar<'static>, Rc<[u8]>>) -> Yoke<&'static str, Rc<[u8]>> {
    ///     bar.project(|bar, _| bar.string_1)   
    /// }
    ///
    /// #
    /// # unsafe impl<'a> Yokeable<'a> for Bar<'static> {
    /// #     type Output = Bar<'a>;
    /// #     fn transform(&'a self) -> &'a Bar<'a> {
    /// #         self
    /// #     }
    /// #
    /// #     fn transform_owned(self) -> Bar<'a> {
    /// #         // covariant lifetime cast, can be done safely
    /// #         self
    /// #     }
    /// #
    /// #     unsafe fn make(from: Bar<'a>) -> Self {
    /// #         let ret = mem::transmute_copy(&from);
    /// #         mem::forget(from);
    /// #         ret
    /// #     }
    /// #
    /// #     fn transform_mut<F>(&'a mut self, f: F)
    /// #     where
    /// #         F: 'static + FnOnce(&'a mut Self::Output),
    /// #     {
    /// #         unsafe { f(mem::transmute(self)) }
    /// #     }
    /// # }
    /// ```
    //
    // Safety docs can be found below on `__project_safety_docs()`
    pub fn project<P>(
        self,
        f: for<'a> fn(
            <Y as Yokeable<'a>>::Output,
            PhantomData<&'a ()>,
        ) -> <P as Yokeable<'a>>::Output,
    ) -> Yoke<P, C>
    where
        P: for<'a> Yokeable<'a>,
    {
        let p = f(self.yokeable.transform_owned(), PhantomData);
        Yoke {
            yokeable: unsafe { P::make(p) },
            cart: self.cart,
        }
    }

    /// This is similar to [`Yoke::project`], however it does not move
    /// [`Self`] and instead clones the cart (only if the cart is a [`CloneableCart`])
    ///
    /// This is a bit more efficient than cloning the [`Yoke`] and then calling [`Yoke::project`]
    /// because then it will not clone fields that are going to be discarded.
    pub fn project_cloned<'this, P>(
        &'this self,
        f: for<'a> fn(
            &'this <Y as Yokeable<'a>>::Output,
            PhantomData<&'a ()>,
        ) -> <P as Yokeable<'a>>::Output,
    ) -> Yoke<P, C>
    where
        P: for<'a> Yokeable<'a>,
        C: CloneableCart,
    {
        let p = f(self.get(), PhantomData);
        Yoke {
            yokeable: unsafe { P::make(p) },
            cart: self.cart.clone(),
        }
    }

    /// This is similar to [`Yoke::project`], however it works around it not being able to
    /// use `FnOnce` by using an explicit capture input, until [compiler bug #84937](https://github.com/rust-lang/rust/issues/84937)
    /// is fixed.
    ///
    /// See the docs of [`Yoke::project`] for how this works.
    pub fn project_with_capture<P, T>(
        self,
        capture: T,
        f: for<'a> fn(
            <Y as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> <P as Yokeable<'a>>::Output,
    ) -> Yoke<P, C>
    where
        P: for<'a> Yokeable<'a>,
    {
        let p = f(self.yokeable.transform_owned(), capture, PhantomData);
        Yoke {
            yokeable: unsafe { P::make(p) },
            cart: self.cart,
        }
    }

    /// This is similar to [`Yoke::project_cloned`], however it works around it not being able to
    /// use `FnOnce` by using an explicit capture input, until [compiler bug #84937](https://github.com/rust-lang/rust/issues/84937)
    /// is fixed.
    ///
    /// See the docs of [`Yoke::project_cloned`] for how this works.
    pub fn project_cloned_with_capture<'this, P, T>(
        &'this self,
        capture: T,
        f: for<'a> fn(
            &'this <Y as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> <P as Yokeable<'a>>::Output,
    ) -> Yoke<P, C>
    where
        P: for<'a> Yokeable<'a>,
        C: CloneableCart,
    {
        let p = f(self.get(), capture, PhantomData);
        Yoke {
            yokeable: unsafe { P::make(p) },
            cart: self.cart.clone(),
        }
    }

    /// A version of [`Yoke::project`] that takes a capture and bubbles up an error
    /// from the callback function.
    #[allow(clippy::type_complexity)]
    pub fn try_project_with_capture<P, T, E>(
        self,
        capture: T,
        f: for<'a> fn(
            <Y as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> Result<<P as Yokeable<'a>>::Output, E>,
    ) -> Result<Yoke<P, C>, E>
    where
        P: for<'a> Yokeable<'a>,
    {
        let p = f(self.yokeable.transform_owned(), capture, PhantomData)?;
        Ok(Yoke {
            yokeable: unsafe { P::make(p) },
            cart: self.cart,
        })
    }

    /// A version of [`Yoke::project_cloned`] that takes a capture and bubbles up an error
    /// from the callback function.
    #[allow(clippy::type_complexity)]
    pub fn try_project_cloned_with_capture<'this, P, T, E>(
        &'this self,
        capture: T,
        f: for<'a> fn(
            &'this <Y as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> Result<<P as Yokeable<'a>>::Output, E>,
    ) -> Result<Yoke<P, C>, E>
    where
        P: for<'a> Yokeable<'a>,
        C: CloneableCart,
    {
        let p = f(self.get(), capture, PhantomData)?;
        Ok(Yoke {
            yokeable: unsafe { P::make(p) },
            cart: self.cart.clone(),
        })
    }
}

#[cfg(feature = "alloc")]
impl<Y: for<'a> Yokeable<'a>, C: 'static + Sized> Yoke<Y, Rc<C>> {
    /// Allows type-erasing the cart in a `Yoke<Y, Rc<C>>`.
    ///
    /// The yoke only carries around a cart type for its destructor,
    /// since it needs to be able to guarantee that its internal references
    /// are valid for the lifetime of the Yoke. As such, the actual type of the
    /// Cart is not very useful unless you wish to extract data out of it
    /// via [`Yoke::backing_cart()`]. Erasing the cart allows for one to mix
    /// [`Yoke`]s obtained from different sources.
    ///
    /// In case the cart type is not already `Rc<T>`, you can use
    /// [`Yoke::wrap_cart_in_rc()`] to wrap it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yoke::Yoke;
    /// use yoke::erased::ErasedRcCart;
    /// use std::rc::Rc;
    ///
    /// let buffer1: Rc<String> = Rc::new("   foo bar baz  ".into());
    /// let buffer2: Box<String> = Box::new("  baz quux  ".into());
    ///
    /// let yoke1 = Yoke::<&'static str, _>::attach_to_cart_badly(buffer1, |rc| rc.trim());
    /// let yoke2 = Yoke::<&'static str, _>::attach_to_cart_badly(buffer2, |b| b.trim());
    ///
    ///
    /// let erased1: Yoke<_, ErasedRcCart> = yoke1.erase_rc_cart();
    /// // Wrap the Box in an Rc to make it compatible
    /// let erased2: Yoke<_, ErasedRcCart> = yoke2.wrap_cart_in_rc().erase_rc_cart();
    ///
    /// // Now erased1 and erased2 have the same type!
    /// ```
    ///
    /// Available with the `"alloc"` feature enabled.
    pub fn erase_rc_cart(self) -> Yoke<Y, ErasedRcCart> {
        unsafe {
            // safe because the cart is preserved, just
            // type-erased
            self.replace_cart(|c| c as ErasedRcCart)
        }
    }
}

#[cfg(feature = "alloc")]
impl<Y: for<'a> Yokeable<'a>, C: 'static + Sized> Yoke<Y, Box<C>> {
    /// Allows type-erasing the cart in a `Yoke<Y, Box<C>>`.
    ///
    /// The yoke only carries around a cart type for its destructor,
    /// since it needs to be able to guarantee that its internal references
    /// are valid for the lifetime of the Yoke. As such, the actual type of the
    /// Cart is not very useful unless you wish to extract data out of it
    /// via [`Yoke::backing_cart()`]. Erasing the cart allows for one to mix
    /// [`Yoke`]s obtained from different sources.
    ///
    /// In case the cart type is not already `Box<T>`, you can use
    /// [`Yoke::wrap_cart_in_box()`] to wrap it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use yoke::Yoke;
    /// use yoke::erased::ErasedBoxCart;
    /// use std::rc::Rc;
    ///
    /// let buffer1: Rc<String> = Rc::new("   foo bar baz  ".into());
    /// let buffer2: Box<String> = Box::new("  baz quux  ".into());
    ///
    /// let yoke1 = Yoke::<&'static str, _>::attach_to_cart_badly(buffer1, |rc| rc.trim());
    /// let yoke2 = Yoke::<&'static str, _>::attach_to_cart_badly(buffer2, |b| b.trim());
    ///
    ///
    /// // Wrap the Rc in an Box to make it compatible
    /// let erased1: Yoke<_, ErasedBoxCart> = yoke1.wrap_cart_in_box().erase_box_cart();
    /// let erased2: Yoke<_, ErasedBoxCart> = yoke2.erase_box_cart();
    ///
    /// // Now erased1 and erased2 have the same type!
    /// ```
    ///
    /// Available with the `"alloc"` feature enabled.
    pub fn erase_box_cart(self) -> Yoke<Y, ErasedBoxCart> {
        unsafe {
            // safe because the cart is preserved, just
            // type-erased
            self.replace_cart(|c| c as ErasedBoxCart)
        }
    }
}

#[cfg(feature = "alloc")]
impl<Y: for<'a> Yokeable<'a>, C> Yoke<Y, C> {
    /// Helper function allowing one to wrap the cart type in a `Box<T>`.
    /// Can be paired with [`Yoke::erase_box_cart()`]
    ///
    /// Available with the `"alloc"` feature enabled.
    #[inline]
    pub fn wrap_cart_in_box(self) -> Yoke<Y, Box<C>> {
        unsafe {
            // safe because the cart is preserved, just wrapped
            self.replace_cart(Box::new)
        }
    }
    /// Helper function allowing one to wrap the cart type in an `Rc<T>`.
    /// Can be paired with [`Yoke::erase_rc_cart()`], or generally used
    /// to make the [`Yoke`] cloneable.
    ///
    /// Available with the `"alloc"` feature enabled.
    #[inline]
    pub fn wrap_cart_in_rc(self) -> Yoke<Y, Rc<C>> {
        unsafe {
            // safe because the cart is preserved, just wrapped
            self.replace_cart(Rc::new)
        }
    }
}

/// Safety docs for project()
///
/// (Docs are on a private const to allow the use of compile_fail doctests)
///
/// This is safe to perform because of the choice of lifetimes on `f`, that is,
/// `for<a> fn(<Y as Yokeable<'a>>::Output, &'a ()) -> <P as Yokeable<'a>>::Output`.
///
/// What we want this function to do is take a Yokeable (`Y`) that is borrowing from the cart, and
/// produce another Yokeable (`P`) that also borrows from the same cart. There are a couple potential
/// hazards here:
///
/// - `P` ends up borrowing data from `Y` (or elsewhere) that did _not_ come from the cart,
///   for example `P` could borrow owned data from a `Cow`. This would make the `Yoke<P>` dependent
///   on data owned only by the `Yoke<Y>`.
/// - Borrowed data from `Y` escapes with the wrong lifetime
///
/// Let's walk through these and see how they're prevented.
///
/// ```rust, compile_fail
/// # use std::rc::Rc;
/// # use yoke::Yoke;
/// # use std::borrow::Cow;
/// fn borrow_potentially_owned(y: &Yoke<Cow<'static, str>, Rc<[u8]>>) -> Yoke<&'static str, Rc<[u8]>> {
///    y.project_cloned(|cow, _| &*cow)   
/// }
/// ```
///
/// In this case, the lifetime of `&*cow` is `&'this str`, however the function needs to be able to return
/// `&'a str` _for all `'a`_, which isn't possible.
///
///
/// ```rust, compile_fail
/// # use std::rc::Rc;
/// # use yoke::Yoke;
/// # use std::borrow::Cow;
/// fn borrow_potentially_owned(y: Yoke<Cow<'static, str>, Rc<[u8]>>) -> Yoke<&'static str, Rc<[u8]>> {
///    y.project(|cow, _| &*cow)   
/// }
/// ```
///
/// This has the same issue, `&*cow` is borrowing for a local lifetime.
///
/// Similarly, trying to project an owned field of a struct will produce similar errors:
///
/// ```rust,compile_fail
/// # use std::borrow::Cow;
/// # use yoke::{Yoke, Yokeable};
/// # use std::mem;
/// # use std::rc::Rc;
/// #
/// // also safely implements Yokeable<'a>
/// struct Bar<'a> {
///     owned: String,
///     string_2: &'a str,
/// }
///
/// fn project_owned(bar: &Yoke<Bar<'static>, Rc<[u8]>>) -> Yoke<&'static str, Rc<[u8]>> {
///     // ERROR (but works if you replace owned with string_2)
///     bar.project_cloned(|bar, _| &*bar.owned)   
/// }
///
/// #
/// # unsafe impl<'a> Yokeable<'a> for Bar<'static> {
/// #     type Output = Bar<'a>;
/// #     fn transform(&'a self) -> &'a Bar<'a> {
/// #         self
/// #     }
/// #
/// #     fn transform_owned(self) -> Bar<'a> {
/// #         // covariant lifetime cast, can be done safely
/// #         self
/// #     }
/// #
/// #     unsafe fn make(from: Bar<'a>) -> Self {
/// #         let ret = mem::transmute_copy(&from);
/// #         mem::forget(from);
/// #         ret
/// #     }
/// #
/// #     fn transform_mut<F>(&'a mut self, f: F)
/// #     where
/// #         F: 'static + FnOnce(&'a mut Self::Output),
/// #     {
/// #         unsafe { f(mem::transmute(self)) }
/// #     }
/// # }
/// ```
///
/// Borrowed data from `Y` similarly cannot escape with the wrong lifetime because of the `for<'a>`, since
/// it will never be valid for the borrowed data to escape for all lifetimes of 'a. Internally, `.project()`
/// uses `.get()`, however the signature forces the callers to be able to handle every lifetime.
///
///  `'a` is the only lifetime that matters here; `Yokeable`s must be `'static` and since
/// `Output` is an associated type it can only have one lifetime, `'a` (there's nowhere for it to get another from).
/// `Yoke`s can get additional lifetimes via the cart, and indeed, `project()` can operate on `Yoke<_, &'b [u8]>`,
/// however this lifetime is inaccessible to the closure, and even if it were accessible the `for<'a>` would force
/// it out of the output. All external lifetimes (from other found outside the yoke/closures
/// are similarly constrained here.
///
/// Essentially, safety is achieved by using `for<'a> fn(...)` with `'a` used in both `Yokeable`s to ensure that
/// the output yokeable can _only_ have borrowed data flow in to it from the input. All paths of unsoundness require the
/// unification of an existential and universal lifetime, which isn't possible.
const _: () = ();
