use crate::ule::*;

/// A borrowed u8 slice of little-endian data
#[derive(Debug)]
pub struct ByteSliceLE<'a, const N: usize>(pub &'a [u8; N]);

/// An owned u8 array of little-endian data
#[derive(Debug)]
pub struct ByteArrayLE<const N: usize>(pub [u8; N]);

/// A borrowed u8 slice of native-endian data
#[derive(Debug)]
pub struct ByteSliceNE<'a, const N: usize>(pub &'a [u8; N]);

/// An owned u8 array of native-endian data
#[derive(Debug)]
pub struct ByteArrayNE<const N: usize>(pub [u8; N]);

macro_rules! impl_byte_slice_size {
    ($size:literal) => {
        impl<'a> From<&'a [u8; $size]> for ByteSliceLE<'a, $size> {
            #[inline(always)]
            fn from(le_bytes: &'a [u8; $size]) -> Self {
                Self(le_bytes)
            }
        }
        impl From<[u8; $size]> for ByteArrayLE<$size> {
            #[inline(always)]
            fn from(le_bytes: [u8; $size]) -> Self {
                Self(le_bytes)
            }
        }
        impl ByteArrayLE<$size> {
            #[inline(always)]
            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }
        }
        impl ULE for ByteArrayLE<$size> {
            type Error = std::convert::Infallible;
            #[inline(always)]
            fn parse_bytes(bytes: &[u8]) -> Result<&[Self], Self::Error> {
                let data = bytes.as_ptr();
                let len = bytes.len() / $size;
                Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
            }
            #[inline(always)]
            fn as_bytes(slice: &[Self]) -> &[u8] {
                let data = slice.as_ptr();
                let len = slice.len() * $size;
                unsafe { std::slice::from_raw_parts(data as *const u8, len) }
            }
        }
    };
}

macro_rules! impl_byte_slice_type {
    ($type:ty, $size:literal) => {
        impl<'a> From<ByteSliceLE<'a, $size>> for $type {
            #[inline(always)]
            fn from(byte_slice: ByteSliceLE<'a, $size>) -> $type {
                <$type>::from_le_bytes(*byte_slice.0)
            }
        }
        impl From<$type> for ByteArrayLE<$size> {
            #[inline(always)]
            fn from(value: $type) -> Self {
                Self(value.to_le_bytes())
            }
        }
        impl AsULE for $type {
            type ULE = ByteArrayLE<$size>;
            #[inline(always)]
            fn as_unaligned(&self) -> Self::ULE {
                ByteArrayLE(self.to_le_bytes())
            }
            #[inline(always)]
            fn from_unaligned(unaligned: &Self::ULE) -> Self {
                <$type>::from_le_bytes(unaligned.0)
            }
        }
    };
}

impl_byte_slice_size!(2);
impl_byte_slice_size!(4);
impl_byte_slice_size!(8);
impl_byte_slice_size!(16);

impl_byte_slice_type!(u16, 2);
impl_byte_slice_type!(u32, 4);
impl_byte_slice_type!(u64, 8);
impl_byte_slice_type!(u128, 16);

impl_byte_slice_type!(i16, 2);
impl_byte_slice_type!(i32, 4);
impl_byte_slice_type!(i64, 8);
impl_byte_slice_type!(i128, 16);
