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
        pub(super) _private_data: u8,
    }

    pub struct OptionTinyAsciiStr1 {
        pub(super) _private_data: u8,
    }

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr2 {
        pub(super) _private_data: u16,
    }

    pub struct OptionTinyAsciiStr2 {
        pub(super) _private_data: u16,
    }

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr4 {
        pub(super) _private_data: u32,
    }

    pub struct OptionTinyAsciiStr4 {
        pub(super) _private_data: u32,
    }

    #[diplomat::rust_link(tinystr::TinyAsciiStr, Struct)]
    pub struct TinyAsciiStr8 {
        pub(super) _private_data: u64,
    }

    pub struct OptionTinyAsciiStr8 {
        pub(super) _private_data: u64,
    }

    impl TinyAsciiStr1 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr1, ICU4XError> {
            TinyAsciiStr::<1>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str<'a>(&'a self) -> &'a str {
            <&TinyAsciiStr<1>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<1>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<1>::from(self).is_empty()
        }
    }

    impl OptionTinyAsciiStr1 {
        pub fn create_some(value: TinyAsciiStr1) -> Self {
            Self {
                _private_data: value._private_data,
            }
        }

        pub fn create_none() -> Self {
            Self {
                _private_data: u8::MAX,
            }
        }

        pub fn is_some(&self) -> bool {
            self._private_data != u8::MAX
        }

        pub fn is_none(&self) -> bool {
            self._private_data == u8::MAX
        }

        pub fn get_value(&self) -> DiplomatResult<TinyAsciiStr1, ()> {
            if self._private_data == u8::MAX {
                Err(()).into()
            } else {
                Ok(TinyAsciiStr1 {
                    _private_data: self._private_data,
                })
                .into()
            }
        }
    }

    impl TinyAsciiStr2 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr2, ICU4XError> {
            TinyAsciiStr::<2>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str<'a>(&'a self) -> &'a str {
            <&TinyAsciiStr<2>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<2>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<2>::from(self).is_empty()
        }
    }

    impl OptionTinyAsciiStr2 {
        pub fn create_some(value: TinyAsciiStr2) -> Self {
            Self {
                _private_data: value._private_data,
            }
        }

        pub fn create_none() -> Self {
            Self {
                _private_data: u16::MAX,
            }
        }

        pub fn is_some(&self) -> bool {
            self._private_data != u16::MAX
        }

        pub fn is_none(&self) -> bool {
            self._private_data == u16::MAX
        }

        pub fn get_value(&self) -> DiplomatResult<TinyAsciiStr2, ()> {
            if self._private_data == u16::MAX {
                Err(()).into()
            } else {
                Ok(TinyAsciiStr2 {
                    _private_data: self._private_data,
                })
                .into()
            }
        }
    }

    impl TinyAsciiStr4 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr4, ICU4XError> {
            TinyAsciiStr::<4>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str<'a>(&'a self) -> &'a str {
            <&TinyAsciiStr<4>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<4>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<4>::from(self).is_empty()
        }
    }

    impl OptionTinyAsciiStr4 {
        pub fn create_some(value: TinyAsciiStr4) -> Self {
            Self {
                _private_data: value._private_data,
            }
        }

        pub fn create_none() -> Self {
            Self {
                _private_data: u32::MAX,
            }
        }

        pub fn is_some(&self) -> bool {
            self._private_data != u32::MAX
        }

        pub fn is_none(&self) -> bool {
            self._private_data == u32::MAX
        }

        pub fn get_value(&self) -> DiplomatResult<TinyAsciiStr4, ()> {
            if self._private_data == u32::MAX {
                Err(()).into()
            } else {
                Ok(TinyAsciiStr4 {
                    _private_data: self._private_data,
                })
                .into()
            }
        }
    }

    impl TinyAsciiStr8 {
        pub fn create_from_str(s: &str) -> DiplomatResult<TinyAsciiStr8, ICU4XError> {
            TinyAsciiStr::<8>::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }

        pub fn as_str<'a>(&'a self) -> &'a str {
            <&TinyAsciiStr<8>>::from(self).as_str()
        }

        pub fn len(&self) -> usize {
            TinyAsciiStr::<8>::from(self).len()
        }

        pub fn is_empty(&self) -> bool {
            TinyAsciiStr::<8>::from(self).is_empty()
        }
    }

    impl OptionTinyAsciiStr8 {
        pub fn create_some(value: TinyAsciiStr8) -> Self {
            Self {
                _private_data: value._private_data,
            }
        }

        pub fn create_none() -> Self {
            Self {
                _private_data: u64::MAX,
            }
        }

        pub fn is_some(&self) -> bool {
            self._private_data != u64::MAX
        }

        pub fn is_none(&self) -> bool {
            self._private_data == u64::MAX
        }

        pub fn get_value(&self) -> DiplomatResult<TinyAsciiStr8, ()> {
            if self._private_data == u64::MAX {
                Err(()).into()
            } else {
                Ok(TinyAsciiStr8 {
                    _private_data: self._private_data,
                })
                .into()
            }
        }
    }
}

macro_rules! impl_tinystr_convert {
    ($n:literal, $ffi_t:path, $utype: ident) => {
        impl From<TinyAsciiStr<$n>> for $ffi_t {
            fn from(other: TinyAsciiStr<$n>) -> Self {
                Self {
                    _private_data: <$utype>::from_ne_bytes(*other.all_bytes()),
                }
            }
        }
        impl From<$ffi_t> for TinyAsciiStr<$n> {
            fn from(other: $ffi_t) -> Self {
                // Safety: The _private_data field is private, and the only way to construct
                // one of these is the From impl above
                unsafe { TinyAsciiStr::from_bytes_unchecked(other._private_data.to_ne_bytes()) }
            }
        }
        impl From<&$ffi_t> for TinyAsciiStr<$n> {
            fn from(other: &$ffi_t) -> Self {
                // Safety: The _private_data field is private, and the only way to construct
                // one of these is the From impl above
                unsafe { TinyAsciiStr::from_bytes_unchecked(other._private_data.to_ne_bytes()) }
            }
        }
        impl From<&$ffi_t> for &TinyAsciiStr<$n> {
            fn from(other: &$ffi_t) -> Self {
                // Safety: other._private_data is based on from_ne_bytes, which is always a cast
                unsafe { core::mem::transmute(&other._private_data) }
            }
        }
    };
}

impl_tinystr_convert!(1, ffi::TinyAsciiStr1, u8);
impl_tinystr_convert!(2, ffi::TinyAsciiStr2, u16);
impl_tinystr_convert!(4, ffi::TinyAsciiStr4, u32);
impl_tinystr_convert!(8, ffi::TinyAsciiStr8, u64);

macro_rules! impl_option_tinystr_convert {
    ($n:literal, $ffi_t:path, $utype: ident) => {
        impl From<Option<TinyAsciiStr<$n>>> for $ffi_t {
            fn from(other: Option<TinyAsciiStr<$n>>) -> Self {
                match other {
                    Some(v) => Self::create_some(v.into()),
                    None => Self::create_none(),
                }
            }
        }
        impl From<$ffi_t> for Option<TinyAsciiStr<$n>> {
            fn from(other: $ffi_t) -> Self {
                Result::from(other.get_value()).map(Into::into).ok()
            }
        }
        impl From<&$ffi_t> for Option<TinyAsciiStr<$n>> {
            fn from(other: &$ffi_t) -> Self {
                Result::from(other.get_value()).map(Into::into).ok()
            }
        }
    };
}

impl_option_tinystr_convert!(1, ffi::OptionTinyAsciiStr1, u8);
impl_option_tinystr_convert!(2, ffi::OptionTinyAsciiStr2, u16);
impl_option_tinystr_convert!(4, ffi::OptionTinyAsciiStr4, u32);
impl_option_tinystr_convert!(8, ffi::OptionTinyAsciiStr8, u64);
