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

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
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

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
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
    use crate::samples::*;

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
