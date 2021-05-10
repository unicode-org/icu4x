// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Yokeable;
use stable_deref_trait::StableDeref;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

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
/// `C` is the "cart", which `Y` may contain references to. A [`Yoke`] can be constructed
/// with such references using [`Self::attach_to_cart()`].
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
    /// Construct a [`Yoke`] by yokeing an object to a cart. This is the primary constructor
    /// for [`Yoke`].
    ///
    /// This method is currently unusable due to a [compiler bug](https://github.com/rust-lang/rust/issues/84937),
    /// use [`Yoke::attach_to_cart_badly()`] instead
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

    /// Temporary version of [`Yoke::attach_to_cart()`]
    /// that doesn't hit https://github.com/rust-lang/rust/issues/84937
    ///
    /// See its docs for more details
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
    pub fn get<'a>(&'a self) -> &'a <Y as Yokeable<'a>>::Output {
        self.yokeable.transform()
    }

    /// Get a reference to the backing cart.
    ///
    /// This can be useful when building caches, etc
    pub fn backing_cart(&self) -> &C {
        &self.cart
    }

    /// Mutate the stored [`Yokeable`] data.
    ///
    /// See [`Yokeable::with_mut()`] for why this operation is safe.
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
    /// #     unsafe fn make(from: Bar<'a>) -> Self {
    /// #         let ret = mem::transmute_copy(&from);
    /// #         mem::forget(from);
    /// #         ret
    /// #     }
    /// #
    /// #     fn with_mut<F>(&'a mut self, f: F)
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
        self.yokeable.with_mut(f)
    }
}

impl<Y: for<'a> Yokeable<'a>, C: StableDeref> Yoke<Y, Option<C>> {
    /// Construct a new [`Yoke`] from static data. There will be no
    /// references to `cart` here since [`Yokeable`]s are `'static`,
    /// this is good for e.g. constructing fully owned
    /// [`Yoke`]s with no internal borrowing.
    ///
    /// This can be paired with [`Yoke::attach_to_option_cart()`] to mix owned
    /// and borrowed data.
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

    /// Similar to [`Yoke::attach_to_cart()`], except it constructs a `Yoke<Y, Option<C>>`
    /// instead, where the cart is `Some(..)`.
    ///
    /// This allows mixing [`Yoke`]s constructed from owned and borrowed data, when
    /// paired with [`Yoke::new_owned()`].
    ///
    /// This method is currently unusable due to a [compiler bug](https://github.com/rust-lang/rust/issues/84937),
    /// use [`Yoke::attach_to_option_cart_badly()`] instead
    pub fn attach_to_option_cart<F>(cart: C, f: F) -> Self
    where
        F: for<'de> FnOnce(&'de <C as Deref>::Target) -> <Y as Yokeable<'de>>::Output,
    {
        let deserialized = f(cart.deref());
        Self {
            yokeable: unsafe { Y::make(deserialized) },
            cart: Some(cart),
        }
    }
    /// Temporary version of [`Yoke::attach_to_option_cart()`]
    /// that doesn't hit https://github.com/rust-lang/rust/issues/84937
    ///
    /// See its docs for more details
    pub fn attach_to_option_cart_badly(
        cart: C,
        f: for<'de> fn(&'de <C as Deref>::Target) -> <Y as Yokeable<'de>>::Output,
    ) -> Self {
        let deserialized = f(cart.deref());
        Self {
            yokeable: unsafe { Y::make(deserialized) },
            cart: Some(cart),
        }
    }
}

/// Clone requires that the cart derefs to the same address after it is cloned. This works for Rc, Arc, and &'a T.
/// For all other cart types, clone `.baking_cart()` and re-use `attach_to_cart()`.
impl<Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, Rc<T>>
where
    for<'a> <Y as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(self.get().clone()) },
            cart: self.cart.clone(),
        }
    }
}

impl<'b, Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, &'b T>
where
    for<'a> <Y as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(self.get().clone()) },
            cart: self.cart,
        }
    }
}

impl<Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, Arc<T>>
where
    for<'a> <Y as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(self.get().clone()) },
            cart: self.cart.clone(),
        }
    }
}

impl<Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, Option<Rc<T>>>
where
    for<'a> <Y as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(self.get().clone()) },
            cart: self.cart.clone(),
        }
    }
}

impl<Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, Option<Arc<T>>>
where
    for<'a> <Y as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(self.get().clone()) },
            cart: self.cart.clone(),
        }
    }
}

impl<'b, Y: for<'a> Yokeable<'a>, T: ?Sized> Clone for Yoke<Y, Option<&'b T>>
where
    for<'a> <Y as Yokeable<'a>>::Output: Clone,
{
    fn clone(&self) -> Self {
        Yoke {
            yokeable: unsafe { Y::make(self.get().clone()) },
            cart: self.cart,
        }
    }
}
