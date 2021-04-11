// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::UVec;
use crate::byte_slice::*;
use serde::de::{self, Deserialize, DeserializeOwned, Deserializer, SeqAccess, Visitor};
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
    T: 'de + Deserialize<'de>,
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
        Ok(UVec::from_unaligned_le_bytes(bytes))
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
    T: 'de + Deserialize<'de>,
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
    // TODO: Generalize over all N
    for<'h> T: Serialize + Copy + From<ByteSliceLE<'h, 4>>,
    ByteArrayLE<4>: From<T>,
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

    const JSON_STR: &'static str = "[50462976,117835012,185207048,252579084,319951120,387323156,454695192,522067228,589439264,656811300,724183336,791555372,858927408,926299444,993671480,1061043516,1128415552,1195787588,1263159624,1330531660]";

    const BINCODE_BUF: &[u8] = &[
        80, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
        42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
    ];

    #[test]
    fn test_serde_json() {
        let uvec_orig = UVec::from(TEST_SLICE);
        let json_str = serde_json::to_string(&uvec_orig).expect("serialize");
        assert_eq!(JSON_STR, json_str);
        // UVec should deserialize from JSON to either Vec or UVec
        let vec_new: Vec<u32> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to Vec");
        assert_eq!(uvec_orig, vec_new.into());
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
        bincode::deserialize::<Vec<u32>>(&bincode_buf)
            .expect_err("deserialize from buffer to Vec");
        let uvec_new: UVec<u32> =
            bincode::deserialize(&bincode_buf).expect("deserialize from buffer to UVec");
        assert_eq!(uvec_orig, uvec_new);
        assert!(matches!(uvec_new.into_inner(), UVecInner::UnalignedLE(_)));
        // Fallback behavior when we can't keep a reference to the Bincode buffer
        // TODO: This doesn't work yet. See #632
        // let uvec_owned: UVec<u32> = bincode::deserialize_from(bincode_buf.as_slice())
        //     .expect("deserialize from Reader to UVec");
    }
}
