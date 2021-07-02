// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::{Cow, ToOwned};
use std::{mem, ptr};

/// A [`Yokeable`] type is essentially one with a covariant lifetime parameter,
/// matched to the parameter in the trait definition. The trait allows one to cast
/// the covariant lifetime to and from `'static`.
///
/// While Rust does not yet have GAT syntax, for the purpose of this documentation
/// we shall refer to "`Self` with a lifetime `'a`" with the syntax `Self<'a>`.
/// Self<'static> is a stand-in for the HKT Self<'_>: lifetime -> type.
///
/// [`Yokeable`]  exposes ways to cast between `Self<'static>` and `Self<'a>` generically.
/// This is useful for turning covariant lifetimes to _dynamic_ lifetimes, where `'static` is
/// used as a way to "erase" the lifetime.
///
/// # Implementation safety
///
/// This trait is safe to implement on types with a _covariant_ lifetime parameter, i.e. one where
/// [`Self::transform()`]'s body can simply be `{ self }`. This will occur when the lifetime
/// parameter is used within references, but not in the arguments of function pointers or in mutable
/// positions (either in `&mut` or via interior mutability)
///
/// This trait must be implemented on the `'static` version of such a type, e.g. one should
/// implement `Yokeable<'a>` (for all `'a`) on `Cow<'static, T>`.
///
/// There are further constraints on implementation safety on individual methods.
///
/// # Trait bounds
///
/// [Compiler bug #85636](https://github.com/rust-lang/rust/issues/85636) makes it tricky to add
/// trait bounds on `Yokeable::Output`. For more information and for workarounds, see
/// [`crate::trait_hack`].
///
/// # Implementation example
///
/// This crate will eventually expose a custom derive that makes it possible to implement this
/// trait safely without much ceremony. Such a custom derive will include static checks
/// for the covariance of each field. In the meantime, this trait can typically be implemented as
/// follows:
///
/// ```rust
/// # use yoke::Yokeable;
/// # use std::borrow::Cow;
/// # use std::{mem, ptr};
/// struct Bar<'a> {
///     numbers: Cow<'a, [u8]>,
///     string: Cow<'a, str>,
///     owned: Vec<u8>,
/// }
///
/// unsafe impl<'a> Yokeable<'a> for Bar<'static> {
///     type Output = Bar<'a>;
///     fn transform(&'a self) -> &'a Bar<'a> {
///         // covariant lifetime cast, can be done safely
///         self
///     }
///
///     unsafe fn make(from: Bar<'a>) -> Self {
///         // We're just doing mem::transmute() here, however Rust is
///         // not smart enough to realize that Bar<'a> and Bar<'static> are of
///         // the same size, so instead we use transmute_copy
///
///         // This assert will be optimized out, but is included for additional
///         // peace of mind as we are using transmute_copy
///         debug_assert!(mem::size_of::<Bar<'a>>() == mem::size_of::<Self>());
///         let ptr: *const Self = (&from as *const Self::Output).cast();
///         mem::forget(from);
///         ptr::read(ptr)
///     }
///
///     fn transform_mut<F>(&'a mut self, f: F)
///     where
///         F: 'static + FnOnce(&'a mut Self::Output),
///     {
///         unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
///     }
/// }
/// ```
pub unsafe trait Yokeable<'a>: 'static {
    /// This type MUST be `Self` with the `'static` replaced with `'a`, i.e. `Self<'a>`
    type Output: 'a;

    /// This method must cast `self` between `&'a Self<'static>` and `&'a Self<'a>`.
    ///
    /// # Implementation safety
    ///
    /// If the invariants of [`Yokeable`] are being satisfied, the body of this method
    /// should simply be `{ self }`, though it's acceptable to include additional assertions
    /// if desired.
    fn transform(&'a self) -> &'a Self::Output;

    /// This method can be used to cast away `Self<'a>`'s lifetime.
    ///
    /// # Safety
    ///
    /// The returned value must be destroyed before the data `from` was borrowing from is.
    ///
    /// # Implementation safety
    ///
    /// A safe implementation of this method must be equivalent to a transmute between
    /// `Self<'a>` and `Self<'static>`
    unsafe fn make(from: Self::Output) -> Self;

    /// This method must cast `self` between `&'a mut Self<'static>` and `&'a mut Self<'a>`,
    /// and pass it to `f`.
    ///
    /// # Implementation safety
    ///
    /// A safe implementation of this method must be equivalent to a pointer cast/transmute between
    /// `&mut Self<'a>` and `&mut Self<'static>` being passed to `f`
    ///
    /// # Why is this safe?
    ///
    /// Typically covariant lifetimes become invariant when hidden behind an `&mut`,
    /// which is why the implementation of this method cannot just be `f(self)`.
    /// The reason behind this is that while _reading_ a covariant lifetime that has been cast to a shorter
    /// one is always safe (this is roughly the definition of a covariant lifetime), writing
    /// may not necessarily be safe since you could write a smaller reference to it. For example,
    /// the following code is unsound because it manages to stuff a `'a` lifetime into a `Cow<'static>`
    ///
    /// ```rust,compile_fail
    /// # use std::borrow::Cow;
    /// # use yoke::Yokeable;
    /// struct Foo {
    ///     str: String,
    ///     cow: Cow<'static, str>,
    /// }
    ///
    /// fn unsound<'a>(foo: &'a mut Foo) {
    ///     let a: &str = &foo.str;
    ///     foo.cow.transform_mut(|cow| *cow = Cow::Borrowed(a));
    /// }
    /// ```
    ///
    /// However, this code will not compile because [`Yokeable::transform_mut()`] requires `F: 'static`.
    /// This enforces that while `F` may mutate `Self<'a>`, it can only mutate it in a way that does
    /// not insert additional references. For example, `F` may call `to_owned()` on a `Cow` and mutate it,
    /// but it cannot insert a new _borrowed_ reference because it has nowhere to borrow _from_ --
    /// `f` does not contain any borrowed references, and while we give it `Self<'a>` (which contains borrowed
    /// data), that borrowed data is known to be valid
    ///
    /// Note that the `for<'b>` is also necessary, otherwise the following code would compile:
    ///
    /// ```rust,compile_fail
    /// # use std::borrow::Cow;
    /// # use yoke::Yokeable;
    /// # use std::mem;
    /// #
    /// // also safely implements Yokeable<'a>
    /// struct Bar<'a> {
    ///     num: u8,
    ///     cow: Cow<'a, u8>,
    /// }
    ///
    /// fn unsound<'a>(bar: &'a mut Bar<'static>) {
    ///     bar.transform_mut(move |bar| bar.cow = Cow::Borrowed(&bar.num));
    /// }
    /// #
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
    /// #     fn transform_mut<F>(&'a mut self, f: F)
    /// #     where
    /// #         F: 'static + FnOnce(&'a mut Self::Output),
    /// #     {
    /// #         unsafe { f(mem::transmute(self)) }
    /// #     }
    /// # }
    /// ```
    ///
    /// which is unsound because `bar` could be moved later, and we do not want to be able to
    /// self-insert references to it.
    ///
    /// The `for<'b>` enforces this by stopping the author of the closure from matching up the input
    /// `&'b Self::Output` lifetime with `'a` and borrowing directly from it.
    ///
    /// Thus the only types of mutations allowed are ones that move around already-borrowed data, or
    /// introduce new owned data:
    ///
    /// ```rust
    /// # use std::borrow::Cow;
    /// # use yoke::Yokeable;
    /// struct Foo {
    ///     str: String,
    ///     cow: Cow<'static, str>,
    /// }
    ///
    /// fn sound<'a>(foo: &'a mut Foo) {
    ///     foo.cow.transform_mut(move |cow| cow.to_mut().push('a'));
    /// }
    /// ```
    fn transform_mut<F>(&'a mut self, f: F)
    where
        // be VERY CAREFUL changing this signature, it is very nuanced (see above)
        F: 'static + for<'b> FnOnce(&'b mut Self::Output);
}

unsafe impl<'a, T: 'static + ToOwned + ?Sized> Yokeable<'a> for Cow<'static, T>
where
    <T as ToOwned>::Owned: Sized,
{
    type Output = Cow<'a, T>;
    fn transform(&'a self) -> &'a Cow<'a, T> {
        // Doesn't need unsafe: `'a` is covariant so this lifetime cast is always safe
        self
    }

    unsafe fn make(from: Cow<'a, T>) -> Self {
        // i hate this
        // unfortunately Rust doesn't think `mem::transmute` is possible since it's not sure the sizes
        // are the same
        debug_assert!(mem::size_of::<Cow<'a, T>>() == mem::size_of::<Self>());
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
}

unsafe impl<'a, T: 'static + ?Sized> Yokeable<'a> for &'static T {
    type Output = &'a T;
    fn transform(&'a self) -> &'a &'a T {
        // Doesn't need unsafe: `'a` is covariant so this lifetime cast is always safe
        self
    }

    unsafe fn make(from: &'a T) -> Self {
        mem::transmute(from)
    }

    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        // Cast away the lifetime of Self
        unsafe { f(mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
    }
}
