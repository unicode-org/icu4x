// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};

/// This type is intended to be similar to the type `MaybeDangling<T>`
/// proposed in [RFC 3336].
///
/// The effect of this is that in Rust's safety model, types inside here are not
/// expected to have any memory dependent validity properties (`dereferenceable`, `noalias`).
///
/// See [#3696] for a testcase where `Yoke` fails this under miri's field-retagging mode.
///
/// This has `T: 'static` since we don't need anything
/// else and we don't want to have to think (more) about variance.
///
/// After [RFC 3336] lands we can use `MaybeDangling` instead.
///
/// [RFC 3336]: https://github.com/rust-lang/rfcs/pull/3336
/// [#3696]: https://github.com/unicode-org/icu4x/issues/3696
pub(crate) struct KindaSortaDangling<T: 'static> {
    /// Safety invariant: This is always an initialized T, never uninit or other
    /// invalid bit patterns
    dangle: MaybeUninit<T>,
}

impl<T: 'static> KindaSortaDangling<T> {
    pub(crate) const fn new(dangle: T) -> Self {
        KindaSortaDangling {
            dangle: MaybeUninit::new(dangle),
        }
    }
    pub(crate) const fn into_inner(self) -> T {
        unsafe { self.dangle.assume_init() }
    }
}

impl<T: 'static> Deref for KindaSortaDangling<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: Safe due to the safety invariant on `dangle`;
        // we can always assume initialized
        unsafe { self.dangle.assume_init_ref() }
    }
}

impl<T: 'static> DerefMut for KindaSortaDangling<T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: Safe due to the safety invariant on `dangle`;
        // we can always assume initialized
        unsafe { self.dangle.assume_init_mut() }
    }
}
