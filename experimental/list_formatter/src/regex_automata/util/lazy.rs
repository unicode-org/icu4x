use core::{
    cell::Cell,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use alloc::{boxed::Box, vec::Vec};

#[inline(always)]
pub(crate) fn get_or_init<T: Send + Sync + 'static>(
    location: &'static AtomicPtr<T>,
    init: impl FnOnce() -> T,
) -> &'static T {
    let mut ptr = location.load(Ordering::Acquire);
    if ptr.is_null() {
        let new_dfa = Box::new(init());
        ptr = Box::into_raw(new_dfa);
        let result = location.compare_exchange(
            ptr::null_mut(),
            ptr,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        if let Err(old) = result {
            let redundant = unsafe { Box::from_raw(ptr) };
            drop(redundant);
            ptr = old;
        }
    }
    unsafe { &*ptr }
}
