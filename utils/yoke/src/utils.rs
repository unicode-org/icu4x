// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Yokeable;
use core::mem::ManuallyDrop;
use core::{mem, ptr};

/// This method can be used to cast away the `Yokeable<'a>`'s lifetime.
///
/// # Safety
///
/// The returned value must be destroyed before the data `from` was borrowing from is.
#[must_use]
#[inline]
pub const unsafe fn make_yokeable<'a, Y: Yokeable<'a>>(from: Y::Output) -> Y {
    // Unfortunately, Rust doesn't think `mem::transmute` is possible since it's not sure the sizes
    // are the same.
    const {
        assert!(mem::size_of::<Y::Output>() == mem::size_of::<Y>());
    }
    let from_ptr: *const Y = (&from as *const Y::Output).cast();
    let _ = ManuallyDrop::new(from);
    // Safety: `ptr` is certainly valid, aligned and points to a properly initialized value, as
    // it comes from a value that was moved into a ManuallyDrop.
    unsafe { ptr::read(from_ptr) }
}

/// Cast from the [`Output`] of a `Yokeable<'static>` to the [`Yokeable`] itself.
///
/// For any implementation that follows the safety contract of `Yokeable<'static>`,
/// `Y` and `Y::Output` are the same type.
///
/// [`Output`]: Yokeable::Output
#[must_use]
#[inline]
pub const fn cast_yokeable<Y: Yokeable<'static>>(from: Y::Output) -> Y {
    // SAFETY:
    // `from` is `'static`, and thus anything it borrows won't be destroyed
    // (at least, not before the returned `Y` is).
    unsafe { self::make_yokeable(from) }
}

/// This method casts `yokeable` between `&'a mut Y<'static>` and `&'a mut Y<'a>`,
/// and passes it to `f`.
///
/// See [`Yokeable::transform_mut`] for why this is safe, noting that the same reasoning about
/// the `'static` return type applies beyond `()` to `R: 'static`.
#[inline]
pub fn transform_mut_yokeable<'a, Y, F, R>(yokeable: &'a mut Y, f: F) -> R
where
    Y: Yokeable<'a>,
    // be VERY CAREFUL changing this signature, it is very nuanced
    F: 'static + for<'b> FnOnce(&'b mut Y::Output) -> R,
    R: 'static,
{
    // Cast away the lifetime of `Y`
    // Safety: this is equivalent to f(transmute(yokeable)), and the documentation of
    // [`Yokeable::transform_mut`] and this function explain why doing so is sound.
    unsafe { f(mem::transmute::<&'a mut Y, &'a mut Y::Output>(yokeable)) }
}
