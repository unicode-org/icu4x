// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use tinystr::TinyAsciiStr;

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use tinystr::TinyAsciiStr;

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr1 {
        pub(super) data: u8,
    }

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr2 {
        pub(super) data: u16,
    }

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr4 {
        pub(super) data: u32,
    }

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr8 {
        pub(super) data: u64,
    }

    impl TinyAsciiStr1 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr1, ICU4XError> {
            TinyAsciiStr::<1>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str(&self) -> &str {
            <&TinyAsciiStr<1>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<1>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<1>::from(self).is_empty()
        }
    }

    impl TinyAsciiStr2 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr2, ICU4XError> {
            TinyAsciiStr::<2>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str(&self) -> &str {
            <&TinyAsciiStr<2>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<2>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<2>::from(self).is_empty()
        }
    }

    impl TinyAsciiStr4 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr4, ICU4XError> {
            TinyAsciiStr::<4>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str(&self) -> &str {
            <&TinyAsciiStr<4>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<4>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<4>::from(self).is_empty()
        }
    }

    impl TinyAsciiStr8 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr8, ICU4XError> {
            TinyAsciiStr::<8>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str(&self) -> &str {
            <&TinyAsciiStr<8>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<8>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<8>::from(self).is_empty()
        }
    }
}

macro_rules! impl_tinystr_convert {
    ($n:literal, $ffi_t:path, $utype: ident) => {
        impl From<TinyAsciiStr<$n>> for $ffi_t {
            fn from(other: TinyAsciiStr<$n>) -> Self {
                Self {
                    data: <$utype>::from_ne_bytes(*other.all_bytes()),
                }
            }
        }
        impl From<$ffi_t> for TinyAsciiStr<$n> {
            fn from(other: $ffi_t) -> Self {
                // Safety: The data field is private, and the only way to construct
                // one of these is the From impl above
                unsafe { TinyAsciiStr::from_bytes_unchecked(other.data.to_ne_bytes()) }
            }
        }
        impl From<&$ffi_t> for TinyAsciiStr<$n> {
            fn from(other: &$ffi_t) -> Self {
                // Safety: The data field is private, and the only way to construct
                // one of these is the From impl above
                unsafe { TinyAsciiStr::from_bytes_unchecked(other.data.to_ne_bytes()) }
            }
        }
        impl From<&$ffi_t> for &TinyAsciiStr<$n> {
            fn from(other: &$ffi_t) -> Self {
                // Safety: other.data is based on from_ne_bytes, which is always a cast
                unsafe { core::mem::transmute(&other.data) }
            }
        }
    };
}

impl_tinystr_convert!(1, ffi::TinyAsciiStr1, u8);
impl_tinystr_convert!(2, ffi::TinyAsciiStr2, u16);
impl_tinystr_convert!(4, ffi::TinyAsciiStr4, u32);
impl_tinystr_convert!(8, ffi::TinyAsciiStr8, u64);
