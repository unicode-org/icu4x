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
    // Safety: all byte representations are valid u16s
    unsafe { cast_bytes_to_slice(bytes) }
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
                vec.push(PotentialUtf16::from_slice(
                    // Safety: all byte representations are valid u16s
                    unsafe { cast_bytes_to_slice(bytes) }?,
                ));
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
    // Safety: all byte representations are valid i32s
    unsafe { cast_bytes_to_slice(bytes) }.map(Some)
}

/// TODO
pub fn option_u32<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [u32]>, D::Error> {
    let Some(bytes) = <Option<&[u8]>>::deserialize(deserializer)? else {
        return Ok(None);
    };
    // Safety: all byte representations are valid u32s
    unsafe { cast_bytes_to_slice(bytes) }.map(Some)
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
    // SAFETY:
    // 1. `cast_bytes_to_slice` verifies that the byte slice is aligned to 4 bytes (the alignment of `I32Pair`) and the length is a multiple of 8 bytes (the size of `I32Pair`).
    // 2. `I32Pair` uses `#[repr(C)]` to guarantee a stable layout without padding.
    // 3. All bit patterns are valid for `i32` (plain-old-data), so the cast is sound.
    unsafe { cast_bytes_to_slice(bytes) }
}

/// TODO
pub fn option_i32_tuple<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [I32Pair]>, D::Error> {
    let Some(bytes) = <Option<&[u8]>>::deserialize(deserializer)? else {
        return Ok(None);
    };
    // SAFETY:
    // 1. `cast_bytes_to_slice` verifies that the byte slice is aligned to 4 bytes (the alignment of `I32Pair`) and the length is a multiple of 8 (the size of `I32Pair`).
    // 2. `I32Pair` uses `#[repr(C)]` to guarantee a stable layout without padding.
    // 3. All bit patterns are valid for `i32` (plain-old-data), so the cast is sound.
    unsafe { cast_bytes_to_slice(bytes) }.map(Some)
}

/// Casts a slice of byte to a slice of T
///
/// # Errors
///
/// Errors if `T` is a Zero-Sized Type (ZST), or if the length or alignment of the byte slice is incorrect.
///
/// # Safety
///
/// Alignment and length are checked, however the caller has to guarantee that the byte representation is valid for type T.
pub unsafe fn cast_bytes_to_slice<T, E: Error>(bytes: &[u8]) -> Result<&[T], E> {
    if size_of::<T>() == 0
        || bytes.as_ptr().align_offset(align_of::<T>()) != 0
        || bytes.len() % size_of::<T>() != 0
    {
        return Err(E::custom("Wrong length or align"));
    }

    // Safety: The check gurantees length and alignment
    Ok(unsafe {
        core::slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / size_of::<T>())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    type E = value::Error;

    #[test]
    fn cast_aligned_correct_length() {
        let data: [u32; 2] = [1, 2];
        let bytes: &[u8] =
            unsafe { core::slice::from_raw_parts(data.as_ptr() as *const u8, size_of_val(&data)) };
        let result: Result<&[u32], E> = unsafe { cast_bytes_to_slice(bytes) };
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), &[1u32, 2]);
    }

    #[test]
    fn cast_wrong_length_only() {
        // 4-byte aligned but 5 bytes long (not a multiple of 4)
        let data: [u32; 2] = [1, 2];
        let bytes: &[u8] = unsafe { core::slice::from_raw_parts(data.as_ptr() as *const u8, 5) };
        let result: Result<&[u32], E> = unsafe { cast_bytes_to_slice(bytes) };
        assert!(result.is_err(), "wrong length alone must be rejected");
    }

    #[test]
    fn cast_wrong_alignment_only() {
        // Correctly sized (4 bytes) but misaligned by 1
        let data: [u8; 8] = [0; 8];
        // Find a u32-aligned start, then offset by 1
        let aligned_start = data.as_ptr().align_offset(align_of::<u32>());
        let misaligned = &data[aligned_start + 1..aligned_start + 5];
        assert_eq!(misaligned.len(), 4); // correct length for one u32
        assert_ne!(misaligned.as_ptr().align_offset(align_of::<u32>()), 0);
        let result: Result<&[u32], E> = unsafe { cast_bytes_to_slice(misaligned) };
        assert!(result.is_err(), "wrong alignment alone must be rejected");
    }

    #[test]
    fn cast_zst() {
        let bytes: &[u8] = &[];
        let result: Result<&[()], E> = unsafe { cast_bytes_to_slice(bytes) };
        assert!(result.is_err(), "ZSTs must be rejected");
    }
}
