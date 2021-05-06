// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Yoke, Yokeable};
use std::rc::Rc;
use std::sync::Arc;

/// A [`Cart`] is a type that is acceptable as backing storage in a [`Yoke`].
///
/// The essential invariant that must be maintained by implementors is that `Self::Inner` references
/// obtained from this type via `Self::get_inner()` must continue to be valid for the
/// complete lifetime of this type (i.e. until destructors run) provided that this type
/// is never accessed via `&mut` references.
///
/// For example, `Rc<Vec<u8>>` and `Rc<[u8]>` are valid [`Cart`]s, however `Rc<RefCell<Vec<u8>>>` is
/// not, because in the latter type it is possible to use interior mutation to change the data.
///
/// In general, this means "no interior mutability", though interior mutability can be fine
/// if the interior mutability is to something that does not affect references. For example,
/// `Rc<[u8]>` does have interior mutability in the refcount, but you cannot get references
/// to it, so it's fine. On the other hand, `Weak<Box<[u8]>>` cannot be a valid [`Cart`] because
/// it is possible for the backing buffer to be cleaned up without warning.
///
/// Common [`Cart`] types to use with [`Yoke`] are ones wrapping `[u8]` or `str`
/// (`Box<[u8]>`, `Rc<[u8]>`, etc) since those are typical inputs to zero-copy
/// deserialization and parsing.
///
/// Typically the [`Cart`] trait will be implemented by smart pointers whilst [`Cartable`] is used
/// as a helper to talk about types that have the same property provided they are not moved, sucn that
/// [`Cart`] can be implemented on wrappers that can maintain this property even if they are moved.
pub unsafe trait Cart {
    type Inner: ?Sized;
    /// Get the inner data
    fn get_inner(&self) -> &Self::Inner;
}

/// [`Cartable`] is a helper trait for implementng [`Cart`]. It has a similar invariant:
/// all references obtained from this type must continue to be valid for the lifetime
/// of this type provided that this type is never moved or accessed via an `&mut` reference.
///
/// Essentially, this means "no interior mutability", however interior mutability which mutates
/// data that cannot have references taken to it is fine.
///
/// For example, both `Rc` and `Weak` use interior mutability, however in `Rc`'s case it mutates
/// the internal reference count which one cannot take references to, whereas in `Weak`'s case
/// the `T` itself can be destroyed.
pub unsafe trait Cartable {}

unsafe impl<T: Cartable + ?Sized> Cart for Rc<T> {
    type Inner = T;
    fn get_inner(&self) -> &Self::Inner {
        &**self
    }
}

unsafe impl<T: Cartable + ?Sized> Cart for Arc<T> {
    type Inner = T;
    fn get_inner(&self) -> &Self::Inner {
        &**self
    }
}

unsafe impl<T: Cartable + ?Sized> Cart for Box<T> {
    type Inner = T;
    fn get_inner(&self) -> &Self::Inner {
        &**self
    }
}

unsafe impl<T: Cartable> Cart for Vec<T> {
    type Inner = [T];
    fn get_inner(&self) -> &Self::Inner {
        &**self
    }
}

unsafe impl<'a, T: Cartable + ?Sized> Cart for &'a T {
    type Inner = T;
    fn get_inner(&self) -> &Self::Inner {
        &**self
    }
}

unsafe impl Cart for String {
    type Inner = str;
    fn get_inner(&self) -> &Self::Inner {
        &**self
    }
}

unsafe impl<T: Cartable> Cartable for [T] {}
unsafe impl<T: Cart> Cartable for Option<T> {}
unsafe impl Cartable for str {}
unsafe impl Cartable for String {}
unsafe impl Cartable for bool {}
unsafe impl Cartable for char {}
unsafe impl Cartable for u8 {}
unsafe impl Cartable for u16 {}
unsafe impl Cartable for u32 {}
unsafe impl Cartable for u64 {}
unsafe impl Cartable for u128 {}
unsafe impl Cartable for i8 {}
unsafe impl Cartable for i16 {}
unsafe impl Cartable for i32 {}
unsafe impl Cartable for i64 {}
unsafe impl Cartable for i128 {}

/// This is a neat implementation; it allows one to build certain kinds of
/// caches by nesting [`Yoke`]s where both the [`Cart`] and the parsed [`Yokeable`]
/// are cached.
///
/// Essentially, this allows the construction of the type
/// `Yoke<Rc<Yoke<C, Y>>, Y>` and `Weak<Yoke<C, Y>>`: the `Weak` can be stored
/// in your cache, whereas the `Yoke<Rc<..>, Y>` is passed around. The cache entry
/// will automatically clean (most of) itself up when all `Rc`s go out of scope.
///
/// The resultant [`Yoke`] type is a bit more complicated but it's not less efficient
/// since all [`Yoke`] operations except the destructor ignore the [`Cart`].
unsafe impl<Y: for<'a> Yokeable<'a>, C: Cart> Cartable for Yoke<Y, C> {}
