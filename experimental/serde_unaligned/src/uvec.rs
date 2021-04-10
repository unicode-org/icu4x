pub struct ByteSliceLE<'a, const N: usize>(pub &'a [u8; N]);

impl<'a> From<ByteSliceLE<'a, 4>> for u32 {
    fn from(byte_slice: ByteSliceLE<'a, 4>) -> u32 {
        u32::from_le_bytes(*byte_slice.0)
    }
}

impl<'a> From<&'a [u8; 4]> for ByteSliceLE<'a, 4> {
    fn from(le_bytes: &'a [u8; 4]) -> Self {
        Self(le_bytes)
    }
}

pub struct ByteArrayLE<const N: usize>(pub [u8; N]);

impl From<u32> for ByteArrayLE<4> {
    fn from(value: u32) -> Self {
        Self(value.to_le_bytes())
    }
}

impl From<[u8; 4]> for ByteArrayLE<4> {
    fn from(le_bytes: [u8; 4]) -> Self {
        Self(le_bytes)
    }
}

impl ByteArrayLE<4> {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

enum UVecInner<'a, T> {
    Owned(Vec<T>),
    Aligned(&'a [T]),
    /// Buffer with little-endian data
    UnalignedLE(&'a [u8]),
    // Buffer with native-endian data (TODO)
    // UnalignedNE(&'a [u8]),
}

pub struct UVec<'a, T>(UVecInner<'a, T>);

impl<'a, T> UVec<'a, T> {
    pub fn from_unaligned_le_bytes(bytes: &'a [u8]) -> Self {
        Self(UVecInner::UnalignedLE(bytes))
    }
}

// TODO: I guess we can't implement std::ops::Index because that returns references, right?
impl<'a, T> UVec<'a, T>
where
    T: Copy + From<ByteSliceLE<'a, 4>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    const a: u32 = 0x03020100;
    const b: u32 = 0x07060504;
    const c: u32 = 0x0b0a0908;

    #[test]
    fn test_get() {
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
            let uvec = UVec::<u32>(UVecInner::UnalignedLE(&TEST_BUFFER_LE));
            assert_eq!(uvec.get(0), Some(a));
            assert_eq!(uvec.get(1), Some(b));
            assert_eq!(uvec.get(2), Some(c));
        }
    }

    #[test]
    fn test_to_stream() {
        {
            let vec = vec![a, b, c];
            let uvec = UVec(UVecInner::Owned(vec));
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer);
            assert_eq!(TEST_BUFFER_LE[0..12], buffer);
        }
        {
            let slice = [a, b, c];
            let uvec = UVec(UVecInner::Aligned(&slice));
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer);
            assert_eq!(TEST_BUFFER_LE[0..12], buffer);
        }
        {
            let uvec = UVec::<u32>(UVecInner::UnalignedLE(&TEST_BUFFER_LE));
            let mut buffer: Vec<u8> = vec![];
            uvec.write_to_stream_le(&mut buffer);
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
