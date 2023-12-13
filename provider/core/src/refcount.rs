// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::boxed::Box;
use core::fmt::Debug;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use yoke::CloneableCart;

#[cfg(not(feature = "sync"))]
pub(crate) use alloc::rc::Rc as SelectedRc;
#[cfg(feature = "sync")]
pub(crate) use alloc::sync::Arc as SelectedRc;

type WrappedType = Box<[u8]>;

const SENTINEL_PTR: *mut WrappedType = core::mem::align_of::<WrappedType>() as *mut WrappedType;

/// A thin wrapper over [`NonNull`] that works with `*const T`
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct OptionSelectedRcBytes(NonNull<WrappedType>);

impl OptionSelectedRcBytes {
    #[inline]
    pub const fn new_sentinel() -> Self {
        // Safety: `SENTINEL_PTR` is non-null
        Self(unsafe { NonNull::new_unchecked(SENTINEL_PTR) })
    }

    #[inline]
    pub fn from_selected_rc(selected_rc: SelectedRc<WrappedType>) -> Self {
        let ptr = SelectedRc::into_raw(selected_rc);
        debug_assert_ne!(
            ptr, SENTINEL_PTR,
            "Creating from [A]Rc should never be the sentinel ptr"
        );
        debug_assert_ne!(
            ptr, 0 as *const WrappedType,
            "Creating from [A]Rc should never be the null ptr"
        );
        // Safety: ptr is non-null
        Self(unsafe { NonNull::new_unchecked(ptr as *mut WrappedType) })
    }

    #[inline]
    pub fn take(&mut self) -> Option<SelectedRc<WrappedType>> {
        let ptr = self.0.as_ptr() as *const WrappedType;
        // Safety: `SENTINEL_PTR` is non-null
        self.0 = unsafe { NonNull::new_unchecked(SENTINEL_PTR) };
        if ptr == SENTINEL_PTR {
            None
        } else {
            // Safety: this type has only 2 constructors, and we already
            // checked for the sentinel value.
            Some(unsafe { SelectedRc::from_raw(ptr) })
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        let ptr = self.0.as_ptr() as *const WrappedType;
        if ptr == SENTINEL_PTR {
            None
        } else {
            // Safety: this type has only 2 constructors, and we already
            // checked for the sentinel value.
            let wrapped_type: &WrappedType = unsafe { self.0.as_ref() };
            Some(&*wrapped_type)
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

// Safety: type has the same semantics as Option<Rc<_>>
// which implements CloneableCart
unsafe impl CloneableCart for OptionSelectedRcBytes {}

// Safety: same bounds as Arc
#[cfg(feature = "sync")]
unsafe impl Send for OptionSelectedRcBytes where WrappedType: Sync + Send {}

// Safety: same bounds as Arc
#[cfg(feature = "sync")]
unsafe impl Sync for OptionSelectedRcBytes where WrappedType: Sync + Send {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    #[global_allocator]
    static ALLOCATOR: MeasuringAllocator = MeasuringAllocator;

    // Inspired by the assert_no_alloc crate
    struct MeasuringAllocator;

    impl MeasuringAllocator {
        // We need to track allocations on each thread independently
        thread_local! {
            static ACTIVE: Cell<bool> = Cell::new(false);
            static TOTAL_ALLOCATED: Cell<usize> = Cell::new(0);
            static TOTAL_DEALLOCATED: Cell<usize> = Cell::new(0);
        }

        pub fn start_measure() {
            Self::ACTIVE.with(|c| c.set(true));
        }

        pub fn end_measure() -> (usize, usize) {
            Self::ACTIVE.with(|c| c.set(false));
            (
                Self::TOTAL_ALLOCATED.with(|c| c.take()),
                Self::TOTAL_DEALLOCATED.with(|c| c.take()),
            )
        }
    }

    unsafe impl GlobalAlloc for MeasuringAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if Self::ACTIVE.with(|f| f.get()) {
                Self::TOTAL_ALLOCATED.with(|c| c.set(c.get() + layout.size()));
            }
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            if Self::ACTIVE.with(|f| f.get()) {
                Self::TOTAL_DEALLOCATED.with(|c| c.set(c.get() + layout.size()));
            }
            System.dealloc(ptr, layout)
        }
    }

    const SAMPLE_BYTES: &[u8] = b"abCDEfg";

    const W: usize = core::mem::size_of::<usize>();

    #[test]
    fn test_new_sentinel() {
        MeasuringAllocator::start_measure();
        {
            OptionSelectedRcBytes::new_sentinel();
        }
        assert_eq!((0, 0), MeasuringAllocator::end_measure());
        MeasuringAllocator::start_measure();
        {
            let mut x = OptionSelectedRcBytes::new_sentinel();
            assert!(matches!(x.take(), None));
        }
        assert_eq!((0, 0), MeasuringAllocator::end_measure());
    }

    #[test]
    fn test_new_rc() {
        MeasuringAllocator::start_measure();
        {
            let x = OptionSelectedRcBytes::from_selected_rc(
                SAMPLE_BYTES.to_vec().into_boxed_slice().into(),
            );
            assert_eq!(Some(SAMPLE_BYTES), x.as_bytes());
        }
        let (allocated, deallocated) = MeasuringAllocator::end_measure();
        assert_eq!(allocated, deallocated);
        assert_eq!(allocated, 4 * W + SAMPLE_BYTES.len());
    }

    #[test]
    fn test_rc_clone() {
        MeasuringAllocator::start_measure();
        {
            let x = OptionSelectedRcBytes::from_selected_rc(
                SAMPLE_BYTES.to_vec().into_boxed_slice().into(),
            );
            assert_eq!(Some(SAMPLE_BYTES), x.as_bytes());
            let (allocated, deallocated) = MeasuringAllocator::end_measure();
            assert_eq!(allocated, 4 * W + SAMPLE_BYTES.len());
            assert_eq!(deallocated, 0);
            MeasuringAllocator::start_measure();
            {
                let y = x.clone();
                assert_eq!(Some(SAMPLE_BYTES), x.as_bytes());
                assert_eq!(Some(SAMPLE_BYTES), y.as_bytes());
                assert_eq!(x.as_bytes(), y.as_bytes());
            }
            assert_eq!((0, 0), MeasuringAllocator::end_measure());
            {
                let mut y = x.clone();
                let rc = y.take().unwrap();
                assert_eq!(SelectedRc::strong_count(&rc), 2);
                assert!(matches!(y.take(), None));
            }
            assert_eq!((0, 0), MeasuringAllocator::end_measure());
            MeasuringAllocator::start_measure();
        }
        let (allocated, deallocated) = MeasuringAllocator::end_measure();
        assert_eq!(allocated, 0);
        assert_eq!(deallocated, 4 * W + SAMPLE_BYTES.len());
    }
}
