// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Trait asserting that a type is `repr(transparent)`. Used as a bound
/// for functions that require this invariant.
///
/// Safety: this type *must* be `repr(transparent)` over `Inner`.
pub(crate) unsafe trait Transparent<Inner: ?Sized> {}

/// Implements private helper functions for `repr(transparent)` types.
macro_rules! impl_transparent_helpers {
    ($outer:ident($inner:path)) => {
        impl $outer {
            #[allow(dead_code)]
            const fn safe_cast_ref(inner: &$inner) -> &$outer
            where
                $outer: Transparent<$inner>,
            {
                // Safety: Outer is repr(transparent) over Inner
                // (enforced via trait safety invariant)
                unsafe { &*(inner as *const $inner as *const $outer) }
            }
            #[allow(dead_code)]
            const fn safe_cast_box(inner: alloc::boxed::Box<$inner>) -> alloc::boxed::Box<$outer>
            where
                $outer: Transparent<$inner>,
            {
                // Safety: Outer is repr(transparent) over Inner
                // (enforced via trait safety invariant)
                unsafe { core::mem::transmute(inner) }
            }
        }
    };
}
