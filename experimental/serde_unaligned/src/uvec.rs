use std::mem::size_of;

pub struct ByteSlice<'a, const N: usize>(pub &'a [u8; N]);

impl<'a> From<ByteSlice<'a, 4>> for u32 {
    fn from(bytes: ByteSlice<'a, 4>) -> u32 {
        u32::from_le_bytes(*bytes.0)
    }
}

impl<'a> From<&'a [u8; 4]> for ByteSlice<'a, 4> {
    fn from(bytes: &'a [u8; 4]) -> Self {
        Self(bytes)
    }
}

enum UVecInner<'a, T> {
    Owned(Vec<T>),
    Aligned(&'a [T]),
    Unaligned(&'a [u8]),
}

pub struct UVec<'a, T>(UVecInner<'a, T>);

impl<'a, T> UVec<'a, T> {
    pub fn from_unaligned_bytes(bytes: &'a [u8]) -> Self {
        Self(UVecInner::Unaligned(bytes))
    }
}

// TODO: I guess we can't implement std::ops::Index because that returns references, right?
impl<'a, T> UVec<'a, T>
where
    T: Copy + From<ByteSlice<'a, 4>>,
{
    pub fn get(&self, index: usize) -> Option<T> {
        use UVecInner::*;
        match &self.0 {
            Owned(vec) => vec.get(index).copied(),
            Aligned(slice) => slice.get(index).copied(),
            Unaligned(bytes) => bytes
                .as_chunks::<4>()
                .0
                .get(index)
                .map(ByteSlice::<4>::from)
                .map(T::from),
        }
    }
}

#[repr(align(8))]
struct Aligned<T>(pub T);

// This is aligned so that we can test unaligned behavior at odd offsets
const ALIGNED_TEST_BUFFER: Aligned<[u8; 80]> = Aligned([
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
]);

pub const TEST_BUFFER: &[u8] = &ALIGNED_TEST_BUFFER.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let a: u32 = 0x03020100;
        let b: u32 = 0x07060504;
        let c: u32 = 0x0b0a0908;
        {
            let vec = vec![a, b, c];
            let uvec = UVec(UVecInner::Owned(vec));
            assert_eq!(uvec.get(0), Some(a));
            assert_eq!(uvec.get(1), Some(b));
            assert_eq!(uvec.get(2), Some(c));
        }
        {
            let slice = [a, b, c];
            let uvec = UVec(UVecInner::Aligned(&slice));
            assert_eq!(uvec.get(0), Some(a));
            assert_eq!(uvec.get(1), Some(b));
            assert_eq!(uvec.get(2), Some(c));
        }
        {
            let uvec = UVec::<u32>(UVecInner::Unaligned(&TEST_BUFFER));
            assert_eq!(uvec.get(0), Some(a));
            assert_eq!(uvec.get(1), Some(b));
            assert_eq!(uvec.get(2), Some(c));
        }
    }
}
