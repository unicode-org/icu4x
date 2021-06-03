// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Yoke;
use crate::Yokeable;
use std::borrow::Cow;
use std::rc::Rc;

/// Trait for types that can clone a cart type `C` with no allocations. A type can be the
/// `ZeroCopyClone` target of multiple cart types.
///
/// It is possible to implement `ZeroCopyClone` without being zero-copy (using heap allocations),
/// but doing so will reduce the performance of various `Yoke` constructors. The intention is for
/// `ZeroCopyClone` to produce a struct from a cart with as little work as possible.
///
/// # Examples
///
/// ```
/// use yoke::Yokeable;
/// use yoke::ZeroCopyClone;
/// use std::borrow::Cow;
///
/// struct MyStruct<'s> {
///     message: Cow<'s, str>,
/// }
///
/// unsafe impl<'a> Yokeable<'a> for MyStruct<'static> {
///     // (not shown; see `Yokeable` for examples)
/// #    type Output = MyStruct<'a>;
/// #    fn transform(&'a self) -> &'a Self::Output {
/// #        self
/// #    }
/// #    unsafe fn make(from: Self::Output) -> Self {
/// #        std::mem::transmute(from)
/// #    }
/// #    fn with_mut<F>(&'a mut self, f: F)
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
/// impl<'s> ZeroCopyClone<MyStruct<'s>> for MyStruct<'static> {
///     fn zcc<'b>(cart: &'b MyStruct<'s>) -> MyStruct<'b> {
///         MyStruct {
///             message: Cow::Borrowed(&cart.message)
///         }
///     }
/// }
///
/// // Reference from a string slice directly
/// impl ZeroCopyClone<str> for MyStruct<'static> {
///     fn zcc<'b>(cart: &'b str) -> MyStruct<'b> {
///         MyStruct {
///             message: Cow::Borrowed(cart)
///         }
///     }
/// }
/// ```
pub trait ZeroCopyClone<C: ?Sized>: for<'a> Yokeable<'a> {
    /// Clone the cart `C` into a [`Yokeable`] struct, which may retain references into `C`.
    fn zcc<'b>(cart: &'b C) -> <Self as Yokeable<'b>>::Output;
}

impl<'b, 's, Y: ZeroCopyClone<C> + for<'a> Yokeable<'a>, C: ?Sized> Yoke<Y, &'b C> {
    /// Construct a [`Yoke`]`<Y, &C>` from a borrowed cart by zero-copy cloning the cart to `Y` and
    /// then yokeing that object to the cart.
    ///
    /// This results in a [`Yoke`] bound to the lifetime of the reference to the borrowed cart.
    ///
    /// The type `Y` must implement [`ZeroCopyClone`]`<C>`.
    ///
    /// # Example
    ///
    /// ```
    /// use yoke::Yoke;
    /// use std::borrow::Cow;
    ///  
    /// let yoke = Yoke::<
    ///     Cow<'static, str>,
    ///     &str
    /// >::attach_to_borrowed_cart("demo");
    ///
    /// assert_eq!("demo", yoke.get());
    /// ```
    pub fn attach_to_borrowed_cart(cart: &'b C) -> Self {
        Yoke::<Y, &'b C>::attach_to_cart_badly(cart, Y::zcc)
    }
}

impl<'b, 's, Y: ZeroCopyClone<C> + for<'a> Yokeable<'a>, C: ?Sized> Yoke<Y, Box<C>> {
    /// Construct a [`Yoke`]`<Y, Box<C>>` from a boxed cart by zero-copy cloning the cart to `Y` and
    /// then yokeing that object to the cart.
    ///
    /// This results in a [`Yoke`] bound to the lifetime of data within the cart. If the cart is
    /// fully owned, then the resulting [`Yoke`] will be `'static`.
    ///
    /// The type `Y` must implement [`ZeroCopyClone`]`<C>`.
    ///
    /// # Example
    ///
    /// ```
    /// use yoke::Yoke;
    /// use std::borrow::Cow;
    ///
    /// let box_cart = Box::new("demo".to_string());
    ///  
    /// let yoke = Yoke::<
    ///     Cow<'static, str>,
    ///     Box<String>
    /// >::attach_to_box_cart(box_cart);
    ///
    /// assert_eq!("demo", yoke.get());
    /// ```
    pub fn attach_to_box_cart(cart: Box<C>) -> Self {
        Yoke::<Y, Box<C>>::attach_to_cart_badly(cart, Y::zcc)
    }
}

impl<'b, 's, Y: ZeroCopyClone<C> + for<'a> Yokeable<'a>, C: ?Sized> Yoke<Y, Rc<C>> {
    /// Construct a [`Yoke`]`<Y, Rc<C>>` from a reference-counted cart by zero-copy cloning the
    /// cart to `Y` and then yokeing that object to the cart.
    ///
    /// This results in a [`Yoke`] bound to the lifetime of data within the cart. If the cart is
    /// fully owned, then the resulting [`Yoke`] will be `'static`.
    ///
    /// The type `Y` must implement [`ZeroCopyClone`]`<C>`.
    ///
    /// # Example
    ///
    /// ```
    /// use yoke::Yoke;
    /// use std::borrow::Cow;
    /// use std::rc::Rc;
    ///
    /// let rc_cart = Rc::from("demo".to_string());
    ///  
    /// let yoke = Yoke::<
    ///     Cow<'static, str>,
    ///     Rc<String>
    /// >::attach_to_rc_cart(rc_cart);
    ///
    /// assert_eq!("demo", yoke.get());
    /// ```
    pub fn attach_to_rc_cart(cart: Rc<C>) -> Self {
        Yoke::<Y, Rc<C>>::attach_to_cart_badly(cart, Y::zcc)
    }
}

// Note: The following could be blanket implementations, but that would require constraining the
// blanket `T` on `T: 'static`, which may not be desirable for all downstream users who may wish
// to customize their `ZeroCopyClone` impl. The blanket implementation may be safe once Rust has
// specialization.

impl ZeroCopyClone<str> for Cow<'static, str> {
    fn zcc<'b>(cart: &'b str) -> Cow<'b, str> {
        Cow::Borrowed(cart)
    }
}

impl ZeroCopyClone<String> for Cow<'static, str> {
    fn zcc<'b>(cart: &'b String) -> Cow<'b, str> {
        Cow::Borrowed(cart)
    }
}

impl ZeroCopyClone<str> for &'static str {
    fn zcc<'b>(cart: &'b str) -> &'b str {
        cart
    }
}

impl ZeroCopyClone<String> for &'static str {
    fn zcc<'b>(cart: &'b String) -> &'b str {
        cart
    }
}
