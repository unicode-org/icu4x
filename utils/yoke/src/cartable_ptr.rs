// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for optional pointers that may be dropped with niche optimization.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::rc::Rc;
#[cfg(feature = "alloc")]
use alloc::sync::Arc;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

const fn sentinel_for<T>() -> NonNull<T> {
    // Safety: align_of anything is always at least 1
    unsafe { NonNull::new_unchecked(core::mem::align_of::<T>() as *mut T) }
}

#[test]
fn test_min_alignment() {
    assert_eq!(1, core::mem::align_of::<()>());
}

/// An object fully representable by a non-null pointer.
///
/// # Safety
///
/// 1. `into_raw` must not change the address of any referenced data.
pub unsafe trait CartablePointerLike {
    type Ptr;

    /// Converts this pointer-like into a pointer.
    fn into_raw(self) -> NonNull<Self::Ptr>;

    /// Drops any memory associated with this pointer-like.
    ///
    /// # Safety
    ///
    /// 1. The pointer MUST have been returned by this impl's `into_raw`.
    /// 2. The pointer MUST NOT be dangling.
    unsafe fn drop_raw(pointer: NonNull<Self::Ptr>);
}

/// An object that implements [`CartablePointerLike`] that also
/// supports cloning without changing the address of referenced data.
///
/// # Safety
///
/// 1. `clone_raw` must not change the address of any referenced data.
pub unsafe trait CloneableCartablePointerLike: CartablePointerLike {
    /// Clones this pointer-like.
    ///
    /// # Safety
    ///
    /// 1. The pointer MUST have been returned by this impl's `into_raw`.
    /// 2. The pointer MUST NOT be dangling.
    unsafe fn clone_raw(pointer: NonNull<Self::Ptr>);
}

unsafe impl<'a, T> CartablePointerLike for &'a T {
    type Ptr = T;

    fn into_raw(self) -> NonNull<T> {
        self.into()
    }
    unsafe fn drop_raw(_pointer: NonNull<T>) {
        // No-op: references are borrowed from elsewhere
    }
}

unsafe impl<'a, T> CloneableCartablePointerLike for &'a T {
    unsafe fn clone_raw(_pointer: NonNull<T>) {
        // No-op: references are borrowed from elsewhere
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T> CartablePointerLike for Box<T> {
    type Ptr = T;

    fn into_raw(self) -> NonNull<T> {
        // Safety: Boxes must contain data (and not be null)
        unsafe { NonNull::new_unchecked(Box::into_raw(self)) }
    }
    unsafe fn drop_raw(pointer: NonNull<T>) {
        let _ = Box::from_raw(pointer.as_ptr());
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T> CartablePointerLike for Rc<T> {
    type Ptr = T;

    fn into_raw(self) -> NonNull<T> {
        // Safety: Rcs must contain data (and not be null)
        unsafe { NonNull::new_unchecked(Rc::into_raw(self) as *mut T) }
    }
    unsafe fn drop_raw(pointer: NonNull<T>) {
        let _ = Rc::from_raw(pointer.as_ptr());
    }
}

#[cfg(feature = "alloc")]
unsafe impl<'a, T> CloneableCartablePointerLike for Rc<T> {
    unsafe fn clone_raw(pointer: NonNull<T>) {
        let rc = Rc::from_raw(pointer.as_ptr());
        let _ = ManuallyDrop::new(rc.clone());
        let _ = ManuallyDrop::new(rc);
    }
}

#[cfg(feature = "alloc")]
unsafe impl<T> CartablePointerLike for Arc<T> {
    type Ptr = T;

    fn into_raw(self) -> NonNull<T> {
        // Safety: Arcs must contain data (and not be null)
        unsafe { NonNull::new_unchecked(Arc::into_raw(self) as *mut T) }
    }
    unsafe fn drop_raw(pointer: NonNull<T>) {
        let _ = Arc::from_raw(pointer.as_ptr());
    }
}

#[cfg(feature = "alloc")]
unsafe impl<'a, T> CloneableCartablePointerLike for Arc<T> {
    unsafe fn clone_raw(pointer: NonNull<T>) {
        let rc = Rc::from_raw(pointer.as_ptr());
        let _ = ManuallyDrop::new(rc.clone());
        let _ = ManuallyDrop::new(rc);
    }
}

/// A type with similar semantics as `Option<C<T>>` but with a niche.
#[derive(Debug)]
pub struct CartableOptionPointer<C>
where
    C: CartablePointerLike,
{
    /// The inner pointer.
    ///
    /// # Invariants
    ///
    /// 1. Must be either `SENTINEL_PTR` or created from `CartablePointerLike::into_raw`
    /// 2. If non-sentinel, must _always_ be for a valid SelectedRc
    inner: NonNull<C::Ptr>,
    _cartable: PhantomData<C>,
}

impl<C> CartableOptionPointer<C>
where
    C: CartablePointerLike,
{
    /// Creates a new instance corresponding to a `None` value.
    #[inline]
    pub(crate) const fn none() -> Self {
        Self {
            inner: sentinel_for::<C::Ptr>(),
            _cartable: PhantomData,
        }
    }

    /// Creates a new instance corresponding to a `Some` value.
    #[inline]
    pub(crate) fn from_cartable(cartable: C) -> Self {
        let ptr = cartable.into_raw();
        debug_assert_ne!(
            ptr,
            sentinel_for::<C::Ptr>(),
            "Creating from [A]Rc is not expected to be the sentinel ptr"
        );
        // Safety: ptr is non-null because ptr is from SelectedRc::into_raw.
        // SelectedRc can't return a null ptr because it implements Deref.
        Self {
            inner: ptr,
            _cartable: PhantomData,
        }
    }

    /// Returns whether this instance is `None`. From the return value:
    ///
    /// - If `true`, the instance is `None`
    /// - If `false`, the instance is a valid `SelectedRc`
    #[inline]
    pub fn is_none(&self) -> bool {
        self.inner == sentinel_for::<C::Ptr>()
    }
}

impl<C> Drop for CartableOptionPointer<C>
where
    C: CartablePointerLike,
{
    fn drop(&mut self) {
        let ptr = self.inner;
        if ptr != sentinel_for::<C::Ptr>() {
            // By the invariants, `ptr` is a valid raw value since it's
            // either that or sentinel, and we just checked for sentinel.
            // We will replace it with the sentinel and then drop `ptr`.
            self.inner = sentinel_for::<C::Ptr>();
            // Safety: by the invariants, `ptr` is a valid raw value.
            unsafe { C::drop_raw(ptr) }
        }
    }
}

impl<C> Clone for CartableOptionPointer<C>
where
    C: CloneableCartablePointerLike,
{
    fn clone(&self) -> Self {
        let ptr = self.inner;
        if ptr != sentinel_for::<C::Ptr>() {
            // By the invariants, `ptr` is a valid raw value since it's
            // either that or sentinel, and we just checked for sentinel.
            // Safety: by the invariants, `ptr` is a valid raw value.
            unsafe { C::clone_raw(ptr) }
        }
        Self {
            inner: self.inner,
            _cartable: PhantomData,
        }
    }
}
