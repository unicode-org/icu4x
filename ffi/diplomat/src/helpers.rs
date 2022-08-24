// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! impl_newtype_convert {
    ($rust_t:path, $ffi_t:path, pub $ffi_f:ident) => {
        impl From<$rust_t> for $ffi_t {
            fn from(other: $rust_t) -> Self {
                Self {
                    $ffi_f: other.0.into(),
                }
            }
        }
        impl From<$ffi_t> for $rust_t {
            fn from(other: $ffi_t) -> Self {
                $rust_t(other.$ffi_f.into())
            }
        }
    };
}

pub(crate) use impl_newtype_convert;
