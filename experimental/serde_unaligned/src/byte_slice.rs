/// A borrowed u8 slice of little-endian data
pub struct ByteSliceLE<'a, const N: usize>(pub &'a [u8; N]);

/// An owned u8 array of little-endian data
pub struct ByteArrayLE<const N: usize>(pub [u8; N]);

/// A borrowed u8 slice of native-endian data
pub struct ByteSliceNE<'a, const N: usize>(pub &'a [u8; N]);

/// An owned u8 array of native-endian data
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
