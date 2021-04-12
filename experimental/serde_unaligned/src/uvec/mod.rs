// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "serde")]
mod serde;

use crate::ule::*;
use std::borrow::Cow;
use std::fmt;

/// The inner representation of a `UVec`. A `UVec` cannot be constructed directly from a
/// `UVecInner` because data may need validation; use the `From` impls on `UVec` instead.
// TODO: Consider a custom Debug impl
pub enum UVecInner<'a, T>
where
    T: AsULE,
{
    Owned(Vec<T>),
    Aligned(&'a [T]),
    /// Buffer with little-endian data
    UnalignedLE(&'a [T::ULE]),
    // Buffer with native-endian data (TODO)
    // UnalignedNE(&'a [u8]),
}

impl<T> fmt::Debug for UVecInner<'_, T>
where
    T: AsULE + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use UVecInner::*;
        match self {
            Owned(vec) => write!(f, "UVec::Owned({:?})", vec),
            Aligned(slice) => write!(f, "UVec::Aligned({:?})", slice),
            UnalignedLE(slice) => {
                let vec: Vec<T> = slice.iter().map(|u| T::from_unaligned(u)).collect();
                write!(f, "UVec::UnalignedLE({:?})", vec)
            }
        }
    }
}

/// A vector that may be borrowed and may point at unaligned data.
pub struct UVec<'a, T: AsULE>
where
    T: AsULE,
{
    inner: UVecInner<'a, T>,
}

impl<T> fmt::Debug for UVec<'_, T>
where
    T: AsULE + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.fmt(f)
    }
}

impl<T> From<Vec<T>> for UVec<'_, T>
where
    T: AsULE,
{
    fn from(vec: Vec<T>) -> Self {
        Self {
            inner: UVecInner::Owned(vec),
        }
    }
}

impl<'a, T> From<&'a [T]> for UVec<'a, T>
where
    T: AsULE,
{
    fn from(slice: &'a [T]) -> Self {
        Self {
            inner: UVecInner::Aligned(slice),
        }
    }
}

impl<'a, 'b, T> PartialEq<UVec<'b, T>> for UVec<'a, T>
where
    T: AsULE + Copy + PartialEq,
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

impl<'a, T> UVec<'a, T>
where
    T: AsULE,
{
    pub fn from_unaligned_le_bytes(
        bytes: &'a [u8],
    ) -> Result<Self, <<T as AsULE>::ULE as ULE>::Error> {
        let parsed = T::ULE::parse_byte_slice(bytes)?;
        Ok(Self {
            inner: UVecInner::UnalignedLE(parsed),
        })
    }

    pub fn from_unaligned_ne_bytes(_bytes: &'a [u8]) -> Self {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        use UVecInner::*;
        match &self.inner {
            Owned(vec) => vec.len(),
            Aligned(slice) => slice.len(),
            UnalignedLE(slice) => slice.len(),
        }
    }

    pub fn into_inner(self) -> UVecInner<'a, T> {
        self.inner
    }
}

// TODO: I guess we can't implement std::ops::Index because that returns references, right?
impl<'a, T> UVec<'a, T>
where
    T: AsULE + Copy,
{
    pub fn get(&self, index: usize) -> Option<T> {
        use UVecInner::*;
        match &self.inner {
            Owned(vec) => vec.get(index).copied(),
            Aligned(slice) => slice.get(index).copied(),
            UnalignedLE(slice) => slice.get(index).map(|u| T::from_unaligned(u)),
        }
    }
}

impl<'a, T> UVec<'a, T>
where
    T: AsULE,
{
    pub fn write_to_stream_le(&self, stream: &mut dyn std::io::Write) -> std::io::Result<usize> {
        use UVecInner::*;
        match &self.inner {
            Owned(vec) => {
                let unaligned: Vec<T::ULE> = vec.iter().map(|t| t.as_unaligned()).collect();
                let bytes = T::ULE::as_byte_slice(&unaligned);
                stream.write(bytes)
            }
            Aligned(slice) => {
                let unaligned: Vec<T::ULE> = slice.iter().map(|t| t.as_unaligned()).collect();
                let bytes = T::ULE::as_byte_slice(&unaligned);
                stream.write(bytes)
            }
            UnalignedLE(slice) => {
                let bytes = T::ULE::as_byte_slice(slice);
                stream.write(bytes)
            }
        }
    }

    pub fn to_le_bytes(&self) -> Cow<'a, [u8]> {
        use UVecInner::*;
        if let UnalignedLE(slice) = self.inner {
            return Cow::Borrowed(T::ULE::as_byte_slice(slice));
        }
        // FIXME
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len() * std::mem::size_of::<T>());
        self.write_to_stream_le(&mut bytes)
            .expect("Writing to memory buffer");
        Cow::Owned(bytes)
    }
}

impl<'a, T> UVec<'a, T>
where
    T: AsULE + Copy + Default + std::ops::AddAssign,
{
    pub fn sum(&self) -> T {
        use UVecInner::*;
        let mut result: T = Default::default();
        match &self.inner {
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
            UnalignedLE(slice) => {
                for value in slice.iter() {
                    result += T::from_unaligned(value)
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
        match &self.inner {
            Owned(vec) => {
                for value in vec.iter() {
                    result = value.wrapping_add(result);
                }
            }
            Aligned(slice) => {
                for value in slice.iter() {
                    result = value.wrapping_add(result);
                }
            }
            UnalignedLE(slice) => {
                for value in slice.iter() {
                    result = u32::from_unaligned(value).wrapping_add(result);
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
    0x00, 0x01, 0x02, 0x00, 0x04, 0x05, 0x06, 0x00, 0x08, 0x09, 0x0a, 0x00, 0x0c, 0x0d, 0x0e, 0x00,
    0x10, 0x11, 0x12, 0x00, 0x14, 0x15, 0x16, 0x00, 0x18, 0x19, 0x1a, 0x00, 0x1c, 0x1d, 0x1e, 0x00,
    0x20, 0x21, 0x22, 0x00, 0x24, 0x25, 0x26, 0x00, 0x28, 0x29, 0x2a, 0x00, 0x2c, 0x2d, 0x2e, 0x00,
    0x30, 0x31, 0x32, 0x00, 0x34, 0x35, 0x36, 0x00, 0x38, 0x39, 0x3a, 0x00, 0x3c, 0x3d, 0x3e, 0x00,
    0x40, 0x41, 0x42, 0x00, 0x44, 0x45, 0x46, 0x00, 0x48, 0x49, 0x4a, 0x00, 0x4c, 0x4d, 0x4e, 0x00,
]);

pub const TEST_BUFFER_LE: &[u8] = &ALIGNED_TEST_BUFFER_LE.0;

pub const TEST_SLICE: &[u32] = &[
    0x020100, 0x060504, 0x0a0908, 0x0e0d0c, 0x121110, 0x161514, 0x1a1918, 0x1e1d1c, 0x222120,
    0x262524, 0x2a2928, 0x2e2d2c, 0x323130, 0x363534, 0x3a3938, 0x3e3d3c, 0x424140, 0x464544,
    0x4a4948, 0x4e4d4c,
];

pub const TEST_SUM: u32 = 52629240;

#[cfg(test)]
mod tests {
    use super::*;

    const A: u32 = 0x020100;
    const B: u32 = 0x060504;
    const C: u32 = 0x0a0908;

    #[test]
    fn test_get() {
        {
            let vec = vec![A, B, C];
            let uvec = UVec::from(vec);
            assert_eq!(uvec.get(0), Some(A));
            assert_eq!(uvec.get(1), Some(B));
            assert_eq!(uvec.get(2), Some(C));
        }
        {
            let slice: &[u32] = &[A, B, C];
            let uvec = UVec::from(slice);
            assert_eq!(uvec.get(0), Some(A));
            assert_eq!(uvec.get(1), Some(B));
            assert_eq!(uvec.get(2), Some(C));
        }
        {
            let uvec = UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE).unwrap();
            assert_eq!(uvec.get(0), Some(A));
            assert_eq!(uvec.get(1), Some(B));
            assert_eq!(uvec.get(2), Some(C));
        }
    }

    #[test]
    fn test_to_stream() {
        {
            let vec = vec![A, B, C];
            let uvec = UVec::from(vec);
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer).unwrap();
            assert_eq!(TEST_BUFFER_LE[0..12], buffer);
        }
        {
            let slice: &[u32] = &[A, B, C];
            let uvec = UVec::from(slice);
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer).unwrap();
            assert_eq!(TEST_BUFFER_LE[0..12], buffer);
        }
        {
            let uvec = UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE).unwrap();
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer).unwrap();
            assert_eq!(TEST_BUFFER_LE, buffer);
        }
    }

    #[test]
    fn test_odd_alignment() {
        assert_eq!(
            Some(0x020100),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE)
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x04000201),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[1..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x05040002),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[2..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x06050400),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[3..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x060504),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[4..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x4e4d4c00),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[75..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x4e4d4c00),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[3..])
                .unwrap()
                .get(18)
        );
        assert_eq!(
            Some(0x4e4d4c),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[76..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            Some(0x4e4d4c),
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE)
                .unwrap()
                .get(19)
        );
        assert_eq!(
            None,
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[77..])
                .unwrap()
                .get(0)
        );
        assert_eq!(
            None,
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE)
                .unwrap()
                .get(20)
        );
        assert_eq!(
            None,
            UVec::<u32>::from_unaligned_le_bytes(&TEST_BUFFER_LE[3..])
                .unwrap()
                .get(19)
        );
    }
}
