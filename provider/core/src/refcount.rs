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

// TODO: Const or static?
const NONE_VALUE: WrappedType = None;

/// A thin wrapper over [`NonNull`] that works with `*const T`
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct NonNullConst<T>(NonNull<T>);

impl<T> NonNullConst<T> {
    #[inline]
    pub const fn new_from_reference(reference: &T) -> Self {
        // Safety: references are non-null
        // Note: There is `From<&T> for NonNull<T>` but it isn't const
        Self(unsafe { NonNull::new_unchecked(reference as *const T as *mut T) })
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
    pub const fn as_ptr(&self) -> *const T {
        self.0.as_ptr().cast_const()
    }
}

/// An `Option<Rc>` with a niche, using a sentinel pointer for `None`.
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionRcBytes(NonNullConst<WrappedType>);

impl OptionRcBytes {
    pub const fn new_none() -> Self {
        Self(NonNullConst::new_from_reference(&NONE_VALUE))
    }
    pub fn new(boxed: Box<[u8]>) -> Self {
        Self::from_rc(Rc::new(Some(boxed)))
    }
    pub fn from_rc(rc: Rc<WrappedType>) -> Self {
        Self(NonNullConst::new_from_rc(rc))
    }
    pub fn into_rc(self) -> Option<Rc<WrappedType>> {
        if self.0.as_ptr() == &NONE_VALUE {
            None
        } else {
            // Safety: this type has only 2 constructors. One of them saves
            // a sentinel pointer, and the other was created from an Rc.
            // We already checked for the sentinel, so this must be an Rc.
            let rc = unsafe { Rc::from_raw(self.0.as_ptr()) };
            Some(rc)
        }
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
        Self(NonNullConst::new_from_reference(&NONE_VALUE))
    }
    pub fn new(boxed: Box<[u8]>) -> Self {
        Self::from_rc(Arc::new(Some(boxed)))
    }
    pub fn from_rc(rc: Arc<WrappedType>) -> Self {
        Self(NonNullConst::new_from_arc(rc))
    }
    pub fn into_rc(self) -> Option<Arc<WrappedType>> {
        if self.0.as_ptr() == &NONE_VALUE {
            None
        } else {
            // Safety: this type has only 2 constructors. One of them saves
            // a sentinel pointer, and the other was created from an Rc.
            // We already checked for the sentinel, so this must be an Rc.
            let rc = unsafe { Arc::from_raw(self.0.as_ptr()) };
            Some(rc)
        }
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
