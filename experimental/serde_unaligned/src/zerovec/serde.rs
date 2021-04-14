// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::ZeroVec;
use crate::ule::*;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;
use std::marker::PhantomData;

struct ZeroVecVisitor<T> {
    marker: PhantomData<fn() -> T>,
}

impl<T> Default for ZeroVecVisitor<T> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for ZeroVecVisitor<T>
where
    T: 'de + Deserialize<'de> + AsULE,
    <<T as AsULE>::ULE as ULE>::Error: fmt::Display,
{
    type Value = ZeroVec<'de, T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence or borrowed buffer of fixed-width elements")
    }

    fn visit_borrowed_bytes<E>(self, bytes: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ZeroVec::try_from_bytes(bytes).map_err(de::Error::custom)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec: Vec<T::ULE> = if let Some(capacity) = seq.size_hint() {
            Vec::with_capacity(capacity)
        } else {
            Vec::new()
        };
        while let Some(value) = seq.next_element::<T>()? {
            vec.push(T::as_unaligned(&value));
        }
        Ok(ZeroVec::Owned(vec))
    }
}

impl<'de, 'a, T> Deserialize<'de> for ZeroVec<'a, T>
where
    T: 'de + Deserialize<'de> + AsULE,
    <<T as AsULE>::ULE as ULE>::Error: fmt::Display,
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = ZeroVecVisitor::default();
        if deserializer.is_human_readable() {
            deserializer.deserialize_seq(visitor)
        } else {
            deserializer.deserialize_bytes(visitor)
        }
    }
}

impl<T> Serialize for ZeroVec<'_, T>
where
    T: Serialize + AsULE + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut seq = serializer.serialize_seq(Some(self.len()))?;
            for value in self.iter() {
                seq.serialize_element(&value)?;
            }
            seq.end()
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    pub const TEST_SLICE: &[u32] = &[
        0x020100, 0x060504, 0x0a0908, 0x0e0d0c, 0x121110, 0x161514, 0x1a1918, 0x1e1d1c, 0x222120,
        0x262524, 0x2a2928, 0x2e2d2c, 0x323130, 0x363534, 0x3a3938, 0x3e3d3c, 0x424140, 0x464544,
        0x4a4948, 0x4e4d4c,
    ];

    const JSON_STR: &'static str = "[131328,394500,657672,920844,1184016,1447188,1710360,1973532,2236704,2499876,2763048,3026220,3289392,3552564,3815736,4078908,4342080,4605252,4868424,5131596]";

    const BINCODE_BUF: &[u8] = &[
        80, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 4, 5, 6, 0, 8, 9, 10, 0, 12, 13, 14, 0, 16, 17, 18, 0,
        20, 21, 22, 0, 24, 25, 26, 0, 28, 29, 30, 0, 32, 33, 34, 0, 36, 37, 38, 0, 40, 41, 42, 0,
        44, 45, 46, 0, 48, 49, 50, 0, 52, 53, 54, 0, 56, 57, 58, 0, 60, 61, 62, 0, 64, 65, 66, 0,
        68, 69, 70, 0, 72, 73, 74, 0, 76, 77, 78, 0,
    ];

    #[test]
    fn test_serde_json() {
        let zerovec_orig = ZeroVec::from_aligned(TEST_SLICE);
        let json_str = serde_json::to_string(&zerovec_orig).expect("serialize");
        assert_eq!(JSON_STR, json_str);
        // ZeroVec should deserialize from JSON to either Vec or ZeroVec
        let vec_new: Vec<u32> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to Vec");
        assert_eq!(
            zerovec_orig,
            ZeroVec::<u32>::from_aligned(vec_new.as_slice())
        );
        let zerovec_new: ZeroVec<u32> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to ZeroVec");
        assert_eq!(zerovec_orig, zerovec_new);
        assert!(matches!(zerovec_new, ZeroVec::Owned(_)));
    }

    #[test]
    fn test_serde_bincode() {
        let zerovec_orig = ZeroVec::from_aligned(TEST_SLICE);
        let bincode_buf = bincode::serialize(&zerovec_orig).expect("serialize");
        assert_eq!(BINCODE_BUF, bincode_buf);
        // ZeroVec should deserialize from Bincode to ZeroVec but not Vec
        bincode::deserialize::<Vec<u32>>(&bincode_buf).expect_err("deserialize from buffer to Vec");
        let zerovec_new: ZeroVec<u32> =
            bincode::deserialize(&bincode_buf).expect("deserialize from buffer to ZeroVec");
        assert_eq!(zerovec_orig, zerovec_new);
        assert!(matches!(zerovec_new, ZeroVec::Borrowed(_)));
    }

    #[test]
    fn test_chars_valid() {
        // 1-byte, 2-byte, 3-byte, and 4-byte character in UTF-8 (not as relevant in UTF-32)
        let zerovec_orig = ZeroVec::from_aligned(&['w', 'ω', '文', '𑄃']);
        let bincode_buf = bincode::serialize(&zerovec_orig).expect("serialize");
        let zerovec_new: ZeroVec<char> =
            bincode::deserialize(&bincode_buf).expect("deserialize from buffer to ZeroVec");
        assert_eq!(zerovec_orig, zerovec_new);
        assert!(matches!(zerovec_new, ZeroVec::Borrowed(_)));
    }

    #[test]
    fn test_chars_invalid() {
        // 119 and 120 are valid, but not 0xD800 (high surrogate)
        let zerovec_orig: ZeroVec<u32> = ZeroVec::from_aligned(&[119, 0xD800, 120]);
        let bincode_buf = bincode::serialize(&zerovec_orig).expect("serialize");
        let zerovec_result = bincode::deserialize::<ZeroVec<char>>(&bincode_buf);
        assert!(matches!(zerovec_result, Err(_)));
    }
}
