// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::UVec;
use crate::ule::*;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::fmt;
use std::marker::PhantomData;

struct UVecVisitor<T> {
    marker: PhantomData<fn() -> T>,
}

impl<T> Default for UVecVisitor<T> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for UVecVisitor<T>
where
    T: 'de + Deserialize<'de> + AsULE,
    <<T as AsULE>::ULE as ULE>::Error: fmt::Display,
{
    type Value = UVec<'de, T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence or byte buffer of integer-like elements")
    }

    fn visit_bytes<E>(self, _bytes: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        todo!()
    }

    fn visit_borrowed_bytes<E>(self, bytes: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UVec::from_unaligned_le_bytes(bytes).map_err(de::Error::custom)
    }

    fn visit_byte_buf<E>(self, _bytes: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        todo!()
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec: Vec<T> = if let Some(capacity) = seq.size_hint() {
            Vec::with_capacity(capacity)
        } else {
            Vec::new()
        };
        while let Some(value) = seq.next_element()? {
            vec.push(value);
        }
        Ok(UVec::from(vec))
    }
}

impl<'de, 'a, T> Deserialize<'de> for UVec<'a, T>
where
    T: 'de + Deserialize<'de> + AsULE,
    <<T as AsULE>::ULE as ULE>::Error: fmt::Display,
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = UVecVisitor::default();
        if deserializer.is_human_readable() {
            deserializer.deserialize_seq(visitor)
        } else {
            deserializer.deserialize_bytes(visitor)
        }
    }
}

impl<T> Serialize for UVec<'_, T>
where
    T: Serialize + AsULE + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut seq = serializer.serialize_seq(Some(self.len()))?;
            // TODO: Use an iterator here once implemented
            for i in 0..self.len() {
                seq.serialize_element(&self.get(i).expect("Within length"))?;
            }
            seq.end()
        } else {
            serializer.serialize_bytes(&self.to_le_bytes())
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    const JSON_STR: &'static str = "[131328,394500,657672,920844,1184016,1447188,1710360,1973532,2236704,2499876,2763048,3026220,3289392,3552564,3815736,4078908,4342080,4605252,4868424,5131596]";

    const BINCODE_BUF: &[u8] = &[
        80, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 4, 5, 6, 0, 8, 9, 10, 0, 12, 13, 14, 0, 16, 17, 18, 0,
        20, 21, 22, 0, 24, 25, 26, 0, 28, 29, 30, 0, 32, 33, 34, 0, 36, 37, 38, 0, 40, 41, 42, 0,
        44, 45, 46, 0, 48, 49, 50, 0, 52, 53, 54, 0, 56, 57, 58, 0, 60, 61, 62, 0, 64, 65, 66, 0,
        68, 69, 70, 0, 72, 73, 74, 0, 76, 77, 78, 0,
    ];

    #[test]
    fn test_serde_json() {
        let uvec_orig = UVec::from(TEST_SLICE);
        let json_str = serde_json::to_string(&uvec_orig).expect("serialize");
        assert_eq!(JSON_STR, json_str);
        // UVec should deserialize from JSON to either Vec or UVec
        let vec_new: Vec<u32> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to Vec");
        assert_eq!(uvec_orig, UVec::<u32>::from(vec_new));
        let uvec_new: UVec<u32> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to UVec");
        assert_eq!(uvec_orig, uvec_new);
        assert!(matches!(uvec_new.into_inner(), UVecInner::Owned(_)));
    }

    #[test]
    fn test_serde_bincode() {
        let uvec_orig = UVec::from(TEST_SLICE);
        let bincode_buf = bincode::serialize(&uvec_orig).expect("serialize");
        assert_eq!(BINCODE_BUF, bincode_buf);
        // UVec should deserialize from Bincode to UVec but not Vec
        bincode::deserialize::<Vec<u32>>(&bincode_buf).expect_err("deserialize from buffer to Vec");
        let uvec_new: UVec<u32> =
            bincode::deserialize(&bincode_buf).expect("deserialize from buffer to UVec");
        assert_eq!(uvec_orig, uvec_new);
        assert!(matches!(uvec_new.into_inner(), UVecInner::UnalignedLE(_)));
        // Fallback behavior when we can't keep a reference to the Bincode buffer
        // TODO: This doesn't work yet. See #632
        // let uvec_owned: UVec<u32> = bincode::deserialize_from(bincode_buf.as_slice())
        //     .expect("deserialize from Reader to UVec");
    }

    #[test]
    fn test_chars_valid() {
        // 1-byte, 2-byte, 3-byte, and 4-byte character in UTF-8 (not as relevant in UTF-32)
        let uvec_orig = UVec::from(vec!['w', 'ω', '文', '𑄃']);
        let bincode_buf = bincode::serialize(&uvec_orig).expect("serialize");
        let uvec_new: UVec<char> =
            bincode::deserialize(&bincode_buf).expect("deserialize from buffer to UVec");
        assert_eq!(uvec_orig, uvec_new);
        assert!(matches!(uvec_new.into_inner(), UVecInner::UnalignedLE(_)));
    }

    #[test]
    fn test_chars_invalid() {
        // 119 and 120 are valid, but not 0xD800 (high surrogate)
        let uvec_orig: UVec<u32> = UVec::from(vec![119, 0xD800, 120]);
        let bincode_buf = bincode::serialize(&uvec_orig).expect("serialize");
        let uvec_result = bincode::deserialize::<UVec<char>>(&bincode_buf);
        assert!(matches!(uvec_result, Err(_)));
    }
}
