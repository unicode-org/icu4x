// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Trait asserting that a type is `repr(transparent)`. Used as a bound
/// for functions that require this invariant.
///
/// # Safety
///
/// 1. This outer type *must* be `repr(transparent)` over `Inner`
/// 2. `validate_inner` *must* return `false` if `Inner` does not uphold
///    invariants enforced by this outer type.
pub(crate) unsafe trait Transparent<Inner: ?Sized> {
    fn validate_inner(inner: &Inner) -> bool;
}

/// Implements private helper functions for `repr(transparent)` types.
macro_rules! impl_transparent_helpers {
    ($outer:ident($inner:path)) => {
        impl $outer {
            /// Casts the inner type to the outer type.
            /// 
            /// This function is safe, but it does not validate invariants
            /// that the outer type might enforce. It is made available as
            /// a private fn which can be called by another fn.
            #[allow(dead_code)]
            const fn cast_ref_unchecked(inner: &$inner) -> &$outer
            where
                $outer: Transparent<$inner>,
            {
                // Safety: Outer is repr(transparent) over Inner
                // (enforced via trait safety invariant)
                unsafe { &*(inner as *const $inner as *const $outer) }
            }
            /// Casts the inner type to the outer type.
            /// 
            /// This function is safe, but it does not validate invariants
            /// that the outer type might enforce. It is made available as
            /// a private fn which can be called by another fn.
            #[allow(dead_code)]
            const fn cast_box_unchecked(inner: alloc::boxed::Box<$inner>) -> alloc::boxed::Box<$outer>
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

macro_rules! impl_transparent_varule {
    ($outer:ident($inner:path)) => {
        // Safety:
        //
        // 1. `repr(transparent)`, enforced by trait bound, implies no padding
        // 2. `repr(transparent)`, enforced by trait bound, implies alignment 1
        // 3. Composition of `repr(transparent)` `validate_by_slice` with
        //    `validate_inner` from the `Transparent` trait implies
        //    valid bytes
        // 4. The `repr(transparent)` `validate_byte_slice` implies
        //    that all bytes are covered
        // 5. Composition of `repr(transparent)` `from_byte_slice_unchecked` with
        //    `cast_ref_unchecked` retains the same reference
        // 6. Other methods are left as default
        // 7. Equality enforced via `Eq` bound
        unsafe impl zerovec::ule::VarULE for $outer
        where
            $outer: Transparent<$inner> + Eq,
        {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
                let inner = <$inner>::parse_byte_slice(bytes)?;
                Self::validate_inner(inner)
                    .then_some(())
                    .ok_or(zerovec::ZeroVecError::parse::<Self>())
            }
            #[inline]
            unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
                let inner = <$inner>::from_byte_slice_unchecked(bytes);
                Self::cast_ref_unchecked(inner)
            }
        }
    };
}
