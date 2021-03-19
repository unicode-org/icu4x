// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::erased::ErasedDataProvider;
use icu_provider_fs::FsDataProvider;
use std::{mem, ptr, slice, str};

#[repr(C)]
/// FFI version of [`ErasedDataProvider`]. See its docs for more details.
///
/// # Safety
///
/// This should only be constructed in Rust via [`ICU4XDataProvider::from_boxed()`], or,
/// from the C side, via functions like [`icu4x_fs_data_provider_create()`].
///
/// This can be constructed by the functions in this module like [`icu4x_fs_data_provider_create()`],
/// and must be destroyed by [`icu4x_data_provider_destroy()`].
pub struct ICU4XDataProvider {
    /// Dummy fields to ensure this is the size of a trait object pointer
    /// Can be improved once the Metadata API stabilizes
    _field1: usize,
    _field2: usize,
}

impl ICU4XDataProvider {
    /// This is unsafe because zeroed() can be passed to other functions
    /// and cause UB
    ///
    /// This is necessary for returning uninitialized values to C.
    ///
    /// # Safety
    ///
    /// Only call for values that are to be returned to C and never passed to Rust.
    pub unsafe fn zeroed() -> Self {
        ICU4XDataProvider {
            _field1: 0,
            _field2: 0,
        }
    }

    /// Construct a [`ICU4XDataProvider`] this from a boxed [`ErasedDataProvider`]
    pub fn from_boxed(x: Box<dyn ErasedDataProvider<'static>>) -> Self {
        unsafe {
            // If the layout changes this will error
            // Once Rust gets pointer metadata APIs we should switch to using those
            mem::transmute(x)
        }
    }

    /// Obtain the original boxed Rust [`ErasedDataProvider`] for this
    pub fn into_boxed(self) -> Box<dyn ErasedDataProvider<'static>> {
        debug_assert!(self._field1 != 0);
        // If the layout changes this will error
        // Once Rust gets pointer metadata APIs we should switch to using those
        unsafe { mem::transmute(self) }
    }

    /// Convert a borrowed reference to a borrowed [`ErasedDataProvider`]
    pub fn as_dyn_ref(&self) -> &dyn ErasedDataProvider<'static> {
        debug_assert!(self._field1 != 0);
        unsafe {
            // &dyn Trait and Box<dyn Trait> have the same layout
            // Note that we are reading from a *pointer* to `Box<dyn Trait>`,
            // so we need to `ptr::read` the fat pointer first.
            let borrowed_erased: ICU4XDataProvider = ptr::read(self);
            // If the layout changes this will error
            // Once Rust gets pointer metadata APIs we should switch to using those
            mem::transmute(borrowed_erased)
        }
    }
}

#[no_mangle]
/// Destructor for [`ICU4XDataProvider`].
///
/// # Safety
///
/// Must be used with a valid [`ICU4XDataProvider`] constructed by functions like
/// [`icu4x_fs_data_provider_create()`]
pub unsafe extern "C" fn icu4x_data_provider_destroy(d: ICU4XDataProvider) {
    let _ = d.into_boxed();
}

#[repr(C)]
/// A result type for [`icu4x_fs_data_provider_create`].
pub struct ICU4XCreateDataProviderResult {
    /// Will be zeroed if `success` is `false`, do not use in that case
    pub provider: ICU4XDataProvider,
    // May potentially add a better error type in the future
    pub success: bool,
}

#[no_mangle]
/// Constructs an [`FsDataProvider`] and retirns it as an [`ICU4XDataProvider`].
/// See [`FsDataProvider::try_new()`] for more details.
///
/// # Safety
///
/// `path` and `len` must point to a valid UTF-8 string, with `len` not including
/// a null terminator if any.
///
/// Only access `provider` in the result if `success` is true.
pub unsafe extern "C" fn icu4x_fs_data_provider_create(
    path: *const u8,
    len: usize,
) -> ICU4XCreateDataProviderResult {
    let path = str::from_utf8_unchecked(slice::from_raw_parts(path, len));
    match FsDataProvider::try_new(path.to_string()) {
        Ok(fs) => {
            let erased = Box::new(fs);
            ICU4XCreateDataProviderResult {
                provider: ICU4XDataProvider::from_boxed(erased),
                success: true,
            }
        }
        Err(_) => ICU4XCreateDataProviderResult {
            provider: ICU4XDataProvider::zeroed(),
            success: false,
        },
    }
}
