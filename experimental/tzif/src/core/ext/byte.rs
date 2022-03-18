// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::Context;

use crate::core::{ParseInput, ParseResult, Parsed};

use super::{map::MapValue, then::Then};

/// A trait to create an integer from big-endian bytes.
pub trait FromBEBytes: Sized {
    /// Creates an instance of `Self` crom big-endian bytes.
    fn from_be_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_from_be_bytes {
    ($type:ty) => {
        impl FromBEBytes for $type {
            fn from_be_bytes(bytes: &[u8]) -> Self {
                const LEN: usize = std::mem::size_of::<$type>();
                debug_assert_eq!(bytes.len(), LEN);
                let mut arr = [0; LEN];
                for i in 0..LEN {
                    arr[i] = bytes[i];
                }
                <$type>::from_be_bytes(arr)
            }
        }
    };
}

impl_from_be_bytes!(i32);
impl_from_be_bytes!(u32);
impl_from_be_bytes!(i64);
impl_from_be_bytes!(u64);

macro_rules! impl_parse_int {
    ($name:ident, $type:ty) => {
        /// Creates an integer from big-endian bytes.
        fn $name(&mut self) -> ParseResult<$type, Source> {
            self.parse_int_from_be_bytes::<$type, { std::mem::size_of::<$type>() }>()
                .wrap_err_with(|| format!("{}(): failed to parse integer", stringify!($name)))
        }
    };
}

impl<T, Source> ParseBytes<Source> for T
where
    Source: Clone + ParseInput<u8, Vec<u8>, Source>,
    T: ParseInput<u8, Vec<u8>, Source>,
{
}

/// A trait extension for consuming bytes from a source input.
pub trait ParseBytes<Source: Clone + ParseInput<u8, Vec<u8>, Source>>:
    ParseInput<u8, Vec<u8>, Source>
{
    /// Parses a [`bool`] from a byte.
    /// Ensures that the byte is either 0 or 1, otherwise retuns an error.
    fn parse_bool(&mut self) -> ParseResult<bool, Source> {
        self.next()
            .then_ensure_or_err_with(
                |&byte| byte <= 1,
                |byte| format!("parse_bool(): expected byte to be 0 or 1, but found `{byte:?}`"),
            )
            .map_value(|byte| byte == 1)
    }

    /// Expect the next byte to match the given argument byte.
    /// Returns an error if the bytes are not equal.
    fn expect_byte(&mut self, expected: u8) -> ParseResult<(), Source> {
        self.next()
            .then_ensure_or_err_with(
                |&actual| actual == expected,
                |actual| format!("expect_byte(): expected byte `{expected}` but found byte `{actual:?}`",),
            )
            .map_value(|_| ())
    }

    /// Parse an integer of a given type and length from big-endian bytes.
    fn parse_int_from_be_bytes<Int: FromBEBytes, const N: usize>(
        &mut self,
    ) -> ParseResult<Int, Source> {
        let mut bytes = [0; N];
        let mut source = self.source()?;
        for byte in bytes.iter_mut().take(N) {
            source = source
                .next()
                .then(|&next| *byte = next)
                .wrap_err_with(|| {
                    format!("parse_int(): failed to parse integer of length `{}`", N)
                })?
                .source();
        }
        Ok(Parsed::new(Int::from_be_bytes(&bytes), source))
    }

    impl_parse_int!(parse_i32, i32);
    impl_parse_int!(parse_u32, u32);
    impl_parse_int!(parse_i64, i64);
    impl_parse_int!(parse_u64, u64);
}
