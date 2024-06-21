// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::boxed::Box;

/// Safety: this type *must* be `repr(transparent)` over `Inner`.
pub(crate) unsafe trait Transparent<Inner: ?Sized> {}

pub(crate) const fn safe_cast_ref<Inner, Outer>(inner: &Inner) -> &Outer where Inner: ?Sized, Outer: Transparent<Inner> + ?Sized {
    // Safety: Outer is repr(transparent) over Inner
    // (enforced via trait safety invariant)
    unsafe { &*(inner as *const Inner as *const Outer) }
}

pub(crate) const fn safe_cast_box<Inner, Outer>(inner: Box<Inner>) -> Box<Outer> where Inner: ?Sized, Outer: Transparent<Inner> + ?Sized {
    // Safety: Outer is repr(transparent) over Inner
    // (enforced via trait safety invariant)
    unsafe { core::mem::transmute(inner) }
}
