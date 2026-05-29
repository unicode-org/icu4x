// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains helpers for zero-copy deserialization of slices other than `&[u8]`.

use alloc::vec::Vec;
use potential_utf::PotentialUtf16;
use serde_core::de::*;

/// TODO
pub fn option_utf_16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de PotentialUtf16>, D::Error> {
    let Some(bytes) = <Option<&[u8]>>::deserialize(deserializer)? else {
        return Ok(None);
    };
    cast_bytes_to_u16_slice(bytes)
        .map(PotentialUtf16::from_slice)
        .map(Some)
}

/// TODO
pub fn vec_utf_16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<&'de PotentialUtf16>, D::Error> {
    struct Utf16Visitor;

    impl<'de> Visitor<'de> for Utf16Visitor {
        type Value = Vec<&'de PotentialUtf16>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(formatter, "a sequence of UTF-16 slices")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or_default());
            while let Some(bytes) = seq.next_element::<&[u8]>()? {
                vec.push(PotentialUtf16::from_slice(cast_bytes_to_u16_slice(bytes)?));
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(Utf16Visitor)
}

/// TODO
pub fn option_i32<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [i32]>, D::Error> {
    let Some(bytes) = <Option<&[u8]>>::deserialize(deserializer)? else {
        return Ok(None);
    };
    let words = cast_bytes_to_u32_slice(bytes)?;
    Ok(Some(cast_u32_to_i32_slice(words)))
}

/// TODO
pub fn option_u32<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [u32]>, D::Error> {
    let Some(bytes) = <Option<&[u8]>>::deserialize(deserializer)? else {
        return Ok(None);
    };
    cast_bytes_to_u32_slice(bytes).map(Some)
}

/// A layout-guaranteed pair of 32-bit signed integers.
///
/// # Safety Layout Invariants
///
/// We apply `#[repr(C)]` to guarantee that this struct has a stable and defined layout matching the C ABI:
/// 1. The fields are laid out sequentially in declaration order.
/// 2. Since `i32` has size 4 and alignment 4, there is no padding between the first and second fields.
/// 3. The total size of the struct is exactly 8 bytes, and the alignment matches the alignment of `i32` (4 bytes).
///
/// This ensures that casting a correctly aligned 4-byte raw byte slice of size `8 * N` to `&[I32Pair]` is fully sound and stable.
#[repr(C)]
#[allow(
    clippy::exhaustive_structs,
    reason = "Stable struct representing a specific C layout"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I32Pair(pub i32, pub i32);

/// TODO
pub fn i32_tuple<'de, D: Deserializer<'de>>(deserializer: D) -> Result<&'de [I32Pair], D::Error> {
    let bytes = <&[u8]>::deserialize(deserializer)?;
    let words = cast_bytes_to_u32_slice(bytes)?;
    cast_u32_to_i32_pair_slice(words)
}

/// TODO
pub fn option_i32_tuple<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [I32Pair]>, D::Error> {
    let Some(bytes) = <Option<&[u8]>>::deserialize(deserializer)? else {
        return Ok(None);
    };
    let words = cast_bytes_to_u32_slice(bytes)?;
    cast_u32_to_i32_pair_slice(words).map(Some)
}

/// Casts a `&[u8]` slice to `&[u32]`.
///
/// # Errors
///
/// Errors if the slice is not 4-byte aligned, or if its length is not a multiple of 4.
pub fn cast_bytes_to_u32_slice<E: Error>(bytes: &[u8]) -> Result<&[u32], E> {
    if bytes.as_ptr().align_offset(align_of::<u32>()) != 0 || bytes.len() % size_of::<u32>() != 0 {
        return Err(E::custom("Wrong length or align"));
    }
    // SAFETY: The check above guarantees 4-byte alignment and size correctness,
    // and all bit patterns are valid for u32.
    Ok(unsafe {
        core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / size_of::<u32>())
    })
}

/// Casts a `&[u8]` slice to `&[u16]`.
///
/// # Errors
///
/// Errors if the slice is not 2-byte aligned, or if its length is not a multiple of 2.
pub fn cast_bytes_to_u16_slice<E: Error>(bytes: &[u8]) -> Result<&[u16], E> {
    if bytes.as_ptr().align_offset(align_of::<u16>()) != 0 || bytes.len() % size_of::<u16>() != 0 {
        return Err(E::custom("Wrong length or align"));
    }
    // SAFETY: The check above guarantees 2-byte alignment and size correctness,
    // and all bit patterns are valid for u16.
    Ok(unsafe {
        core::slice::from_raw_parts(bytes.as_ptr() as *const u16, bytes.len() / size_of::<u16>())
    })
}

/// Casts a `&[u32]` slice to `&[i32]`.
///
/// This cast is always safe because `u32` and `i32` have identical size and alignment,
/// and all bit patterns are valid for both.
pub fn cast_u32_to_i32_slice(words: &[u32]) -> &[i32] {
    // SAFETY: u32 and i32 have identical size (4) and alignment (4),
    // and all bit patterns are valid for both.
    unsafe { core::slice::from_raw_parts(words.as_ptr() as *const i32, words.len()) }
}

/// Casts a `&[u32]` slice to `&[I32Pair]`.
///
/// # Errors
///
/// Errors if the length of `words` is not even.
pub fn cast_u32_to_i32_pair_slice<E: Error>(words: &[u32]) -> Result<&[I32Pair], E> {
    if words.len() % 2 != 0 {
        return Err(E::custom("Wrong length"));
    }
    // SAFETY:
    // 1. u32 and I32Pair have identical alignment (4 bytes), so alignment is guaranteed.
    // 2. I32Pair is annotated with #[repr(C)] and has no padding.
    // 3. Its fields are i32, which are plain-old-data (all bit patterns valid).
    // 4. The size of I32Pair is exactly 8 bytes (two u32s), and the length check guarantees we do not read out of bounds.
    Ok(unsafe { core::slice::from_raw_parts(words.as_ptr() as *const I32Pair, words.len() / 2) })
}

#[cfg(test)]
mod tests {
    use super::*;

    type E = value::Error;

    #[test]
    fn cast_bytes_to_u32_correct() {
        let data: [u32; 2] = [1, 2];
        let bytes: &[u8] =
            unsafe { core::slice::from_raw_parts(data.as_ptr() as *const u8, size_of_val(&data)) };
        let result: Result<&[u32], E> = cast_bytes_to_u32_slice(bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &[1u32, 2]);
    }

    #[test]
    fn cast_bytes_to_u32_wrong_length() {
        let data: [u32; 2] = [1, 2];
        let bytes: &[u8] = unsafe { core::slice::from_raw_parts(data.as_ptr() as *const u8, 5) };
        let result: Result<&[u32], E> = cast_bytes_to_u32_slice(bytes);
        assert!(result.is_err());
    }

    #[test]
    fn cast_bytes_to_u32_wrong_alignment() {
        let data: [u8; 8] = [0; 8];
        let aligned_start = data.as_ptr().align_offset(align_of::<u32>());
        let misaligned = &data[aligned_start + 1..aligned_start + 5];
        assert_eq!(misaligned.len(), 4);
        let result: Result<&[u32], E> = cast_bytes_to_u32_slice(misaligned);
        assert!(result.is_err());
    }

    #[test]
    fn cast_u32_to_i32_pair_correct() {
        let words: [u32; 4] = [1, 2, 3, 4];
        let result: Result<&[I32Pair], E> = cast_u32_to_i32_pair_slice(&words);
        assert!(result.is_ok());
        let slice = result.unwrap();
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], I32Pair(1, 2));
        assert_eq!(slice[1], I32Pair(3, 4));
    }

    #[test]
    fn cast_u32_to_i32_pair_wrong_length() {
        let words: [u32; 3] = [1, 2, 3];
        let result: Result<&[I32Pair], E> = cast_u32_to_i32_pair_slice(&words);
        assert!(result.is_err());
    }
}
