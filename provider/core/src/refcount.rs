// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ffi::c_void;
use core::fmt::Debug;
use core::mem::size_of;
use core::{mem::ManuallyDrop, ptr::NonNull};
use alloc::sync::Arc;
use alloc::{string::String, rc::Rc};
use alloc::boxed::Box;
use stable_deref_trait::{StableDeref, CloneStableDeref};

type WrappedType = Option<Box<[u8]>>;

// Safe as a sentinel because 0x1 is not a valid usize address
const SENTINEL: *const WrappedType = 1usize as *const WrappedType;

/// A thin wrapper over [`NonNull`] that works with `*const T`
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct NonNullConst<T>(NonNull<T>);

impl<T> NonNullConst<T> {
    #[inline]
    pub const fn new_sentinel() -> Self {
        // Safety: `SENTINEL` is non-null
        Self(unsafe { NonNull::new_unchecked(SENTINEL as *mut T) })
    }
    #[inline]
    pub fn new_from_rc(rc: Rc<T>) -> Self {
        // Safety: `Rc::into_raw` returns a non-null pointer
        Self(unsafe { NonNull::new_unchecked(Rc::into_raw(rc) as *mut T) })
    }
    #[inline]
    pub fn new_from_arc(rc: Arc<T>) -> Self {
        // Safety: `Arc::into_raw` returns a non-null pointer
        Self(unsafe { NonNull::new_unchecked(Arc::into_raw(rc) as *mut T) })
    }
    #[inline]
    fn into_raw(&mut self) -> Option<*const T> {
        let ptr = self.0.as_ptr() as *const T;
        if ptr == SENTINEL as *const T {
            None
        } else {
            Some(ptr)
        }
    }
    /// Safety: this `NonNullConst` must have been created with
    /// `new_from_rc` or `new_sentinel`
    #[inline]
    pub unsafe fn into_assume_rc(&mut self) -> Option<Rc<T>> {
        self.into_raw().map(|ptr| Rc::from_raw(ptr))
    }
    /// Safety: this `NonNullConst` must have been created with
    /// `new_from_arc` or `new_sentinel`
    #[inline]
    pub unsafe fn into_assume_arc(&mut self) -> Option<Arc<T>> {
        self.into_raw().map(|ptr| Arc::from_raw(ptr))
    }
}

/// An `Option<Rc>` with a niche, using a sentinel pointer for `None`.
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionRcBytes(NonNullConst<WrappedType>);

impl OptionRcBytes {
    pub const fn new_none() -> Self {
        Self(NonNullConst::new_sentinel())
    }
    pub fn new(boxed: Box<[u8]>) -> Self {
        Self::from_rc(Rc::new(Some(boxed)))
    }
    pub fn from_rc(rc: Rc<WrappedType>) -> Self {
        Self(NonNullConst::new_from_rc(rc))
    }
    pub fn into_rc(mut self) -> Option<Rc<WrappedType>> {
        // Safety: this was created from `new_from_rc` or `new_sentinel`
        unsafe { self.0.into_assume_rc() }
    }
    pub fn as_rc(&self) -> Option<&Rc<WrappedType>> {
        todo!()
    }
}

impl Drop for OptionRcBytes {
    fn drop(&mut self) {
        let old_self = core::mem::replace(self, Self::new_none());
        old_self.into_rc();
    }
}

impl Clone for OptionRcBytes {
    fn clone(&self) -> Self {
        todo!()
    }
}

// unsafe impl StableDeref for OptionRcBytes {}

// unsafe impl CloneStableDeref for OptionRcBytes {}

/// An `Option<Arc>` with a niche, using a sentinel pointer for `None`.
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionArcBytes(NonNullConst<WrappedType>);

impl OptionArcBytes {
    pub const fn new_none() -> Self {
        // Safety: SENTINEL is not zero
        Self(NonNullConst::new_sentinel())
    }
    pub fn new(boxed: Box<[u8]>) -> Self {
        Self::from_rc(Arc::new(Some(boxed)))
    }
    pub fn from_rc(rc: Arc<WrappedType>) -> Self {
        Self(NonNullConst::new_from_arc(rc))
    }
    pub fn into_rc(mut self) -> Option<Arc<WrappedType>> {
        // Safety: this was created from `new_from_arc` or `new_sentinel`
        unsafe { self.0.into_assume_arc() }
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

// unsafe impl StableDeref for OptionArcBytes {}

// unsafe impl CloneStableDeref for OptionArcBytes {}
