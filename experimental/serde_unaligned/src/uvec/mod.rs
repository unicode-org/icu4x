// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "serde")]
mod serde;

use crate::byte_slice::*;
use std::borrow::Cow;

/// The inner representation of a `UVec`. A `UVec` cannot be constructed directly from a
/// `UVecInner` because data may need validation; use the `From` impls on `UVec` instead.
// TODO: Consider a custom Debug impl
#[derive(Debug)]
pub enum UVecInner<'a, T> {
    Owned(Vec<T>),
    Aligned(&'a [T]),
    /// Buffer with little-endian data
    UnalignedLE(&'a [u8]),
    // Buffer with native-endian data (TODO)
    // UnalignedNE(&'a [u8]),
}

/// A vector that may be borrowed and may point at unaligned data.
#[derive(Debug)]
pub struct UVec<'a, T>(UVecInner<'a, T>);

impl<T> From<Vec<T>> for UVec<'_, T> {
    fn from(vec: Vec<T>) -> Self {
        Self(UVecInner::Owned(vec))
    }
}

impl<'a, T> From<&'a [T]> for UVec<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        Self(UVecInner::Aligned(slice))
    }
}

impl<'a, 'b, T> PartialEq<UVec<'b, T>> for UVec<'a, T>
where
    for<'h> T: Copy + From<ByteSliceLE<'h, 4>> + PartialEq,
{
    fn eq(&self, other: &UVec<'b, T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        // TODO: Use an iterator here
        for i in 0..self.len() {
            if self.get(i) != other.get(i) {
                return false;
            }
        }
        true
    }
}

impl<'a, T> UVec<'a, T> {
    pub fn from_unaligned_le_bytes(bytes: &'a [u8]) -> Self {
        Self(UVecInner::UnalignedLE(bytes))
    }

    pub fn from_unaligned_ne_bytes(_bytes: &'a [u8]) -> Self {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        use UVecInner::*;
        match &self.0 {
            Owned(vec) => vec.len(),
            Aligned(slice) => slice.len(),
            UnalignedLE(bytes) => bytes.len() / std::mem::size_of::<T>(),
        }
    }

    pub fn into_inner(self) -> UVecInner<'a, T> {
        self.0
    }
}

// TODO: I guess we can't implement std::ops::Index because that returns references, right?
impl<'a, T> UVec<'a, T>
where
    for<'h> T: Copy + From<ByteSliceLE<'h, 4>>,
{
    pub fn get(&self, index: usize) -> Option<T> {
        use UVecInner::*;
        match &self.0 {
            Owned(vec) => vec.get(index).copied(),
            Aligned(slice) => slice.get(index).copied(),
            UnalignedLE(bytes) => bytes
                .as_chunks::<4>()
                .0
                .get(index)
                .map(ByteSliceLE::<4>::from)
                .map(T::from),
        }
    }
}

impl<'a, T> UVec<'a, T>
where
    T: Copy,
    ByteArrayLE<4>: From<T>,
{
    pub fn write_to_stream_le(&self, stream: &mut dyn std::io::Write) -> std::io::Result<usize> {
        use UVecInner::*;
        let mut result: usize = 0;
        match &self.0 {
            Owned(vec) => {
                for value in vec.iter() {
                    result += stream.write(&ByteArrayLE::<4>::from(*value).as_bytes())?;
                }
            }
            Aligned(slice) => {
                for value in slice.iter() {
                    result += stream.write(&ByteArrayLE::<4>::from(*value).as_bytes())?;
                }
            }
            UnalignedLE(bytes) => {
                result += stream.write(bytes)?;
            }
        };
        Ok(result)
    }

    pub fn to_le_bytes(&self) -> Cow<'a, [u8]> {
        use UVecInner::*;
        if let UnalignedLE(bytes) = self.0 {
            return Cow::Borrowed(bytes);
        }
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len() * std::mem::size_of::<T>());
        self.write_to_stream_le(&mut bytes)
            .expect("Writing to memory buffer");
        Cow::Owned(bytes)
    }
}

impl<'a, T> UVec<'a, T>
where
    T: Copy + Default + std::ops::AddAssign + From<ByteSliceLE<'a, 4>>,
{
    pub fn sum(&self) -> T {
        use UVecInner::*;
        let mut result: T = Default::default();
        match &self.0 {
            Owned(vec) => {
                for value in vec.iter() {
                    result += *value;
                }
            }
            Aligned(slice) => {
                for value in slice.iter() {
                    result += *value;
                }
            }
            UnalignedLE(bytes) => {
                for chunk in bytes.array_chunks::<4>() {
                    result += T::from(ByteSliceLE::<4>::from(chunk));
                }
            }
        };
        result
    }
}

impl<'a> UVec<'a, u32> {
    pub fn sum_u32(&self) -> u32 {
        use UVecInner::*;
        let mut result = 0;
        match &self.0 {
            Owned(vec) => {
                for value in vec.iter() {
                    result += *value;
                }
            }
            Aligned(slice) => {
                for value in slice.iter() {
                    result += *value;
                }
            }
            UnalignedLE(bytes) => {
                for chunk in bytes.array_chunks::<4>() {
                    result += u32::from_le_bytes(*chunk);
                }
            }
        };
        result
    }
}

#[repr(align(8))]
struct Aligned<T>(pub T);

// This is aligned so that we can test unaligned behavior at odd offsets
const ALIGNED_TEST_BUFFER_LE: Aligned<[u8; 80]> = Aligned([
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
]);

pub const TEST_BUFFER_LE: &[u8] = &ALIGNED_TEST_BUFFER_LE.0;

pub const TEST_SLICE: &[u32] = &[
    0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
    0x23222120, 0x27262524, 0x2b2a2928, 0x2f2e2d2c, 0x33323130, 0x37363534, 0x3b3a3938, 0x3f3e3d3c,
    0x43424140, 0x47464544, 0x4b4a4948, 0x4f4e4d4c,
];

#[cfg(test)]
mod tests {
    use super::*;

    const A: u32 = 0x03020100;
    const B: u32 = 0x07060504;
    const C: u32 = 0x0b0a0908;

    #[test]
    fn test_get() {
        {
            let vec = vec![A, B, C];
            let uvec = UVec(UVecInner::Owned(vec));
            assert_eq!(uvec.get(0), Some(A));
            assert_eq!(uvec.get(1), Some(B));
            assert_eq!(uvec.get(2), Some(C));
        }
        {
            let slice = [A, B, C];
            let uvec = UVec(UVecInner::Aligned(&slice));
            assert_eq!(uvec.get(0), Some(A));
            assert_eq!(uvec.get(1), Some(B));
            assert_eq!(uvec.get(2), Some(C));
        }
        {
            let uvec = UVec::<u32>(UVecInner::UnalignedLE(&TEST_BUFFER_LE));
            assert_eq!(uvec.get(0), Some(A));
            assert_eq!(uvec.get(1), Some(B));
            assert_eq!(uvec.get(2), Some(C));
        }
    }

    #[test]
    fn test_to_stream() {
        {
            let vec = vec![A, B, C];
            let uvec = UVec(UVecInner::Owned(vec));
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer).unwrap();
            assert_eq!(TEST_BUFFER_LE[0..12], buffer);
        }
        {
            let slice = [A, B, C];
            let uvec = UVec(UVecInner::Aligned(&slice));
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer).unwrap();
            assert_eq!(TEST_BUFFER_LE[0..12], buffer);
        }
        {
            let uvec = UVec::<u32>(UVecInner::UnalignedLE(&TEST_BUFFER_LE));
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer).unwrap();
            assert_eq!(TEST_BUFFER_LE, buffer);
        }
    }

    #[test]
    fn test_odd_alignment() {
        assert_eq!(
            Some(0x03020100),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE).get(0)
        );
        assert_eq!(
            Some(0x04030201),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[1..]).get(0)
        );
        assert_eq!(
            Some(0x05040302),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[2..]).get(0)
        );
        assert_eq!(
            Some(0x06050403),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[3..]).get(0)
        );
        assert_eq!(
            Some(0x07060504),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[4..]).get(0)
        );
        assert_eq!(
            Some(0x4e4d4c4b),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[75..]).get(0)
        );
        assert_eq!(
            Some(0x4e4d4c4b),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[3..]).get(18)
        );
        assert_eq!(
            Some(0x4f4e4d4c),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[76..]).get(0)
        );
        assert_eq!(
            Some(0x4f4e4d4c),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE).get(19)
        );
        assert_eq!(
            None,
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[77..]).get(0)
        );
        assert_eq!(
            None,
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE).get(20)
        );
        assert_eq!(
            None,
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[3..]).get(19)
        );
    }
}
