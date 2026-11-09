// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};

/// This type is intended to be similar to the type `MaybeDangling<T>`
/// proposed in [RFC 3336].
///
/// The effect of this is that in Rust's safety model, types inside here are not
/// expected to have any memory dependent validity properties (`dereferenceable`, `noalias`).
///
/// Since Rust 1.96.0 ([rust-lang/rust#147228]), [`ManuallyDrop<T>`] wraps
/// `core::mem::MaybeDangling<T>` ([rust-lang/rust#118166]) internally, which suppresses
/// field retagging (`noalias` / `dereferenceable`) in `rustc` and Miri while preserving
/// layout niches of `T` (unlike `MaybeUninit<T>`, which is a `union` and strips all niches).
///
/// See [#3696] and [#2095] for testcases where `Yoke` fails under miri's field-retagging mode
/// if not using [`KindaSortaDangling`].
///
/// After [RFC 3336] lands publicly we can use `MaybeDangling` directly instead.
///
/// Note that a version of this type also exists publicly as the [`maybe_dangling`]
/// crate; which also exports a patched `ManuallyDrop` with similar semantics and
/// does not require `T: 'static`. Consider using this if you need something more general
/// and are okay with adding dependencies.
///
/// [RFC 3336]: https://rust-lang.github.io/rfcs/3336-maybe-dangling.html
/// [rust-lang/rust#118166]: https://github.com/rust-lang/rust/issues/118166
/// [rust-lang/rust#147228]: https://github.com/rust-lang/rust/pull/147228
/// [#3696]: https://github.com/unicode-org/icu4x/issues/3696
/// [#2095]: https://github.com/unicode-org/icu4x/issues/2095
/// [`maybe_dangling`](https://docs.rs/maybe-dangling/0.1.0/maybe_dangling/struct.MaybeDangling.html)
#[repr(transparent)]
pub(crate) struct KindaSortaDangling<T> {
    // Safety invariant: `dangle` is a live (not-yet-dropped) `T`.
    dangle: ManuallyDrop<T>,
}

impl<T> KindaSortaDangling<T> {
    #[inline]
    pub(crate) const fn new(dangle: T) -> Self {
        KindaSortaDangling {
            dangle: ManuallyDrop::new(dangle),
        }
    }
    #[inline]
    pub(crate) fn into_inner(self) -> T {
        let manual = ManuallyDrop::new(self);
        // SAFETY: `ManuallyDrop` and `KindaSortaDangling` are both `#[repr(transparent)]`
        // over `T`, and by `dangle`'s safety invariant the wrapped value is a live `T`.
        // Wrapping `self` in `ManuallyDrop` suppresses `KindaSortaDangling::drop` so the
        // inner `T` is not dropped a second time.
        unsafe { (&raw const manual).cast::<T>().read() }
    }
}

impl<T> Deref for KindaSortaDangling<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.dangle
    }
}

impl<T> DerefMut for KindaSortaDangling<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.dangle
    }
}

impl<T> Drop for KindaSortaDangling<T> {
    #[inline]
    fn drop(&mut self) {
        // Drop in place via `ManuallyDrop::drop` rather than moving `T` out by value,
        // avoiding `noalias` / `dereferenceable` retags on `T`.
        //
        // SAFETY: By `dangle`'s safety invariant, `self.dangle` is a live `T` on entry
        // to `drop` and is not accessed or dropped again afterwards.
        unsafe {
            ManuallyDrop::drop(&mut self.dangle);
        }
    }
}
