// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Debug;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use alloc::boxed::Box;
use yoke::CloneableCart;

#[cfg(not(feature = "sync"))]
use alloc::rc::Rc as SelectedRc;
#[cfg(feature = "sync")]
use alloc::sync::Arc as SelectedRc;

type WrappedType = Box<[u8]>;

const SENTINEL_PTR: *mut WrappedType = core::mem::align_of::<WrappedType>() as *mut WrappedType;

/// A thin wrapper over [`NonNull`] that works with `*const T`
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionSelectedRcBytes(NonNull<WrappedType>);

impl OptionSelectedRcBytes {
    #[inline]
    pub const fn new_sentinel() -> Self {
        // Safety: `SENTINEL` is non-null
        Self(unsafe { NonNull::new_unchecked(SENTINEL_PTR) })
    }

    #[inline]
    pub fn from_selected_rc(selected_rc: SelectedRc<WrappedType>) -> Self {
        let ptr = SelectedRc::into_raw(selected_rc);
        debug_assert_ne!(ptr, SENTINEL_PTR, "Creating from [A]Rc should never be the sentinel ptr");
        debug_assert_ne!(ptr, 0 as *const WrappedType, "Creating from [A]Rc should never be the null ptr");
        // Safety: ptr is non-null
        Self(unsafe { NonNull::new_unchecked(ptr as *mut WrappedType) })
    }

    #[inline]
    pub fn take(&mut self) -> Option<SelectedRc<WrappedType>> {
        let ptr = self.0.as_ptr() as *const WrappedType;
        *self = Self::new_sentinel();
        if ptr == SENTINEL_PTR {
            None
        } else {
            // Safety: this type has only 2 constructors, and we already
            // checked for the sentinel value.
            Some(unsafe { SelectedRc::from_raw(ptr) })
        }
    }
}

impl Drop for OptionSelectedRcBytes {
    fn drop(&mut self) {
        self.take();
    }
}

impl Clone for OptionSelectedRcBytes {
    fn clone(&self) -> Self {
        let ptr = self.0.as_ptr() as *const WrappedType;
        if ptr == SENTINEL_PTR {
            Self::new_sentinel()
        } else {
            // Safety: this type has only 2 constructors, and we already
            // checked for the sentinel value.
            let selected_rc = unsafe { SelectedRc::from_raw(ptr) };
            // Increase the ref count
            let _ = ManuallyDrop::new(selected_rc.clone());
            Self::from_selected_rc(selected_rc)
        }
    }
}

unsafe impl CloneableCart for OptionSelectedRcBytes {}

#[cfg(feature = "sync")]
unsafe impl Send for OptionSelectedRcBytes {}

#[cfg(feature = "sync")]
unsafe impl Sync for OptionSelectedRcBytes {}

// unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T, A> {}
// unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T, A> {}
