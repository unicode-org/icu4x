// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::erased::ErasedDataProvider;
use icu_provider_fs::FsDataProvider;
use std::{mem, ptr, slice, str};

#[repr(C)]
/// Safety: This should only be constructed via `from_boxed`
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
    /// This is necessary for returning uninitialized values to C
    pub unsafe fn zeroed() -> Self {
        ICU4XDataProvider {
            _field1: 0,
            _field2: 0,
        }
    }

    pub fn from_boxed(x: Box<dyn ErasedDataProvider<'static>>) -> Self {
        unsafe {
            // if the layout changes this will error
            mem::transmute(x)
        }
    }

    pub fn into_boxed(self) -> Box<dyn ErasedDataProvider<'static>> {
        debug_assert!(self._field1 != 0);
        unsafe { mem::transmute(self) }
    }

    pub fn as_dyn_ref(&self) -> &dyn ErasedDataProvider<'static> {
        debug_assert!(self._field1 != 0);
        unsafe {
            // &dyn Trait and Box<dyn Trait> have the same layout
            let borrowed_erased: ICU4XDataProvider = ptr::read(self);
            mem::transmute(borrowed_erased)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_erased_data_provider_destroy(d: ICU4XDataProvider) {
    let _ = d.into_boxed();
}

#[repr(C)]
pub struct ICU4XCreateDataProviderResult {
    provider: ICU4XDataProvider,
    success: bool,
}

#[no_mangle]
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
