// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! impl_newtype_convert {
    ($rust_t:path, $rust_inner_t:path, $ffi_t:path) => {
        impl From<$rust_t> for $ffi_t {
            fn from(other: $rust_t) -> Self {
                <$rust_inner_t>::from(other).into()
            }
        }
        impl From<$ffi_t> for $rust_t {
            fn from(other: $ffi_t) -> Self {
                <$rust_inner_t>::from(other).into()
            }
        }
        impl From<&$ffi_t> for $rust_t {
            fn from(other: &$ffi_t) -> Self {
                <$rust_inner_t>::from(other).into()
            }
        }
    };
}

macro_rules! impl_optional_convert {
    ($rust_t:path, $ffi_t:path) => {
        impl From<Option<$rust_t>> for $ffi_t {
            fn from(other: Option<$rust_t>) -> Self {
                match other {
                    Some(v) => Self::create_some(v.into()),
                    None => Self::create_none(),
                }
            }
        }
        impl From<$ffi_t> for Option<$rust_t> {
            fn from(other: $ffi_t) -> Self {
                Result::from(other.get_value()).map(Into::into).ok()
            }
        }
        impl From<&$ffi_t> for Option<$rust_t> {
            fn from(other: &$ffi_t) -> Self {
                Result::from(other.get_value()).map(Into::into).ok()
            }
        }
    };
}

macro_rules! impl_optional_newtype_convert {
    ($rust_t:path, $rust_inner_t:path, $ffi_t:path) => {
        impl From<$ffi_t> for Option<$rust_t> {
            fn from(v: $ffi_t) -> Self {
                Option::<$rust_inner_t>::from(v).map(Into::into)
            }
        }

        impl From<&$ffi_t> for Option<$rust_t> {
            fn from(v: &$ffi_t) -> Self {
                Option::<$rust_inner_t>::from(v).map(Into::into)
            }
        }

        impl From<Option<$rust_t>> for $ffi_t {
            fn from(v: Option<$rust_t>) -> Self {
                v.map(<$rust_inner_t>::from).into()
            }
        }
    };
}

pub(crate) use impl_newtype_convert;
pub(crate) use impl_optional_convert;
pub(crate) use impl_optional_newtype_convert;
