// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ffi::c_void;
use core::fmt::Debug;
use core::mem::{size_of, MaybeUninit, align_of};
use core::ptr::{addr_of, addr_of_mut};
use core::{mem::ManuallyDrop, ptr::NonNull};
use alloc::sync::Arc;
use alloc::{string::String, rc::Rc};
use alloc::boxed::Box;
use stable_deref_trait::{StableDeref, CloneStableDeref};
use yoke::CloneableCart;

type WrappedType = Box<[u8]>;

const SENTINEL_PTR: *mut WrappedType = align_of::<WrappedType>() as *mut WrappedType;

/// A thin wrapper over [`NonNull`] that works with `*const T`
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct NonNullWrappedPtr(NonNull<WrappedType>);

impl NonNullWrappedPtr {
    #[inline]
    pub const fn new_sentinel() -> Self {
        // Safety: `SENTINEL` is non-null
        Self(unsafe { NonNull::new_unchecked(SENTINEL_PTR) })
    }
    #[inline]
    pub fn new_from_rc(rc: Rc<WrappedType>) -> Self {
        Self::new_raw(Rc::into_raw(rc))
    }
    #[inline]
    pub fn new_from_arc(rc: Arc<WrappedType>) -> Self {
        Self::new_raw(Arc::into_raw(rc))
    }
    #[inline]
    fn new_raw(ptr: *const WrappedType) -> Self {
        debug_assert_ne!(ptr, SENTINEL_PTR, "Creating from [A]Rc should never be the sentinel ptr");
        debug_assert_ne!(ptr, 0 as *const WrappedType, "Creating from [A]Rc should never be the null ptr");
        // Safety: ptr is non-null
        Self(unsafe { NonNull::new_unchecked(ptr as *mut WrappedType) })
    }

    /// Safety: this `NonNullWrappedPtr` must have been created with
    /// `new_from_rc` or `new_sentinel`
    #[inline]
    pub unsafe fn take_assume_rc(&mut self) -> Option<Rc<WrappedType>> {
        self.take_raw().map(|ptr| Rc::from_raw(ptr))
    }
    /// Safety: this `NonNullWrappedPtr` must have been created with
    /// `new_from_arc` or `new_sentinel`
    #[inline]
    pub unsafe fn take_assume_arc(&mut self) -> Option<Arc<WrappedType>> {
        self.take_raw().map(|ptr| Arc::from_raw(ptr))
    }
    #[inline]
    fn take_raw(&mut self) -> Option<*const WrappedType> {
        let ptr = self.0.as_ptr() as *const WrappedType;
        *self = Self::new_sentinel();
        if ptr == SENTINEL_PTR {
            None
        } else {
            Some(ptr)
        }
    }
}

/// An `Option<Rc>` with a niche, using a sentinel pointer for `None`.
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionRcBytes(NonNullWrappedPtr);

impl OptionRcBytes {
    pub const fn new_none() -> Self {
        Self(NonNullWrappedPtr::new_sentinel())
    }
    pub fn new(boxed: Box<[u8]>) -> Self {
        Self::from_rc(Rc::new(boxed))
    }
    pub fn from_rc(rc: Rc<WrappedType>) -> Self {
        Self(NonNullWrappedPtr::new_from_rc(rc))
    }
    pub fn take_rc(&mut self) -> Option<Rc<WrappedType>> {
        // Safety: this was created from `new_from_rc` or `new_sentinel`
        unsafe { self.0.take_assume_rc() }
    }
    pub fn as_bytes(&self) -> Option<&WrappedType> {
        todo!()
    }
}

impl Drop for OptionRcBytes {
    fn drop(&mut self) {
        self.take_rc();
    }
}

impl Clone for OptionRcBytes {
    fn clone(&self) -> Self {
        todo!()
    }
}

unsafe impl CloneableCart for OptionRcBytes {}

/// An `Option<Arc>` with a niche, using a sentinel pointer for `None`.
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionArcBytes(NonNullWrappedPtr);

impl OptionArcBytes {
    pub const fn new_none() -> Self {
        // Safety: SENTINEL is not zero
        Self(NonNullWrappedPtr::new_sentinel())
    }
    pub fn new(boxed: Box<[u8]>) -> Self {
        Self::from_rc(Arc::new(boxed))
    }
    pub fn from_rc(rc: Arc<WrappedType>) -> Self {
        Self(NonNullWrappedPtr::new_from_arc(rc))
    }
    pub fn take_rc(&mut self) -> Option<Rc<WrappedType>> {
        // Safety: this was created from `new_from_arc` or `new_sentinel`
        unsafe { self.0.take_assume_arc() }
    }
    pub fn as_rc(&self) -> Option<&Arc<WrappedType>> {
        todo!()
    }
}

impl Drop for OptionArcBytes {
    fn drop(&mut self) {
        let old_self = core::mem::replace(self, Self::new_none());
        old_self.into_rc();
    }
}

impl Clone for OptionArcBytes {
    fn clone(&self) -> Self {
        todo!()
    }
}

unsafe impl CloneableCart for OptionArcBytes {}

unsafe impl Send for OptionArcBytes {}

unsafe impl Sync for OptionArcBytes {}

// unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T, A> {}
// unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T, A> {}
