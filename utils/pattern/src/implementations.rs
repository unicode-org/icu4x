// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Pattern, SinglePlaceholder, SinglePlaceholderPattern};

use alloc::boxed::Box;
use zerovec::{
    maps::ZeroMapKV,
    ule::{UleError, VarULE},
    VarZeroSlice, VarZeroVec,
};

impl<'a> ZeroMapKV<'a> for Pattern<SinglePlaceholder, str> {
    type Container = VarZeroVec<'a, Pattern<SinglePlaceholder, str>>;
    type Slice = VarZeroSlice<Pattern<SinglePlaceholder, str>>;
    type GetType = Pattern<SinglePlaceholder, str>;
    type OwnedType = Box<Pattern<SinglePlaceholder, str>>;
}

/// Implement `VarULE` for `Pattern<SinglePlaceholder, str>`.
///
/// # Safety
///
/// Safety checklist for `ULE`:
///
/// 1. `str` does not include any uninitialized or padding bytes.
/// 2. `str` is aligned to 1 byte.
/// 3. The implementation of `validate_byte_slice()` returns an error
///    if any byte is not valid.
/// 4. The implementation of `validate_byte_slice()` returns an error
///    if the slice cannot be used to build a `Pattern<SinglePlaceholder, str>`
///    in its entirety.
/// 5. The implementation of `from_byte_slice_unchecked()` returns a reference to the same data.
/// 6. `parse_byte_slice()` is equivalent to `validate_byte_slice()` followed by `from_byte_slice_unchecked()`.
/// 7. `str` byte equality is semantic equality.
///
/// # Examples
///
/// ```
/// use core::str::FromStr;
/// use icu_pattern::Pattern;
/// use icu_pattern::SinglePlaceholder;
/// use writeable::assert_writeable_eq;
/// use zerovec::ule::VarULE;
///
/// // Create a pattern from a valid string:
/// let allocated_pattern =
///     Pattern::<SinglePlaceholder, String>::from_str("{0} days")
///         .expect("valid pattern");
///
/// // Transform the store and create a new Pattern. This is valid because
/// // we call `.take_store()` and `.from_byte_slice_unchecked()` on patterns
/// // with the same backend (`SinglePlaceholder`).
/// let store = allocated_pattern.take_store();
///
/// assert!(
///     unsafe {
///         Pattern::<SinglePlaceholder, str>::validate_byte_slice(store.as_bytes()).is_ok()
///    });
///
/// // SAFETY: store comes from a valid pattern
/// let borrowed_pattern: &Pattern<SinglePlaceholder, _> =
///     unsafe {
///         Pattern::<SinglePlaceholder, str>::from_byte_slice_unchecked(store.as_bytes())
///    };
///
/// assert_writeable_eq!(borrowed_pattern.interpolate([5]), "5 days");
/// ```
unsafe impl VarULE for Pattern<SinglePlaceholder, str> {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), UleError> {
        SinglePlaceholderPattern::try_from_utf8_store(bytes)
            .map_err(|_| UleError::parse::<Self>())?;
        Ok(())
    }

    unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
        // SAFETY: As `validate_byte_slice` succeeded, `try_from_utf8_store` succeeded, which implies valid UTF-8
        let store = core::str::from_utf8_unchecked(bytes);

        // SAFETY:
        //      As `validate_byte_slice` succeeded, this means `store` is valid.
        //      And the layout of `Pattern` is the same as `Store`
        //      because `_backend` is zero-sized and `store` is the only other field.
        &*(store as *const str as *const Self)
    }
}
