// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::VarZeroVec;
use crate::ule::*;
use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{self, Serialize, SerializeSeq, Serializer};
use std::fmt;
use std::marker::PhantomData;

struct VarZeroVecVisitor<T> {
    marker: PhantomData<fn() -> T>,
}

impl<T> Default for VarZeroVecVisitor<T> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for VarZeroVecVisitor<T>
where
    T: 'de + Deserialize<'de> + AsVarULE,
    <<T as AsVarULE>::VarULE as VarULE>::Error: fmt::Display,
{
    type Value = VarZeroVec<'de, T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence or borrowed buffer of bytes")
    }

    fn visit_borrowed_bytes<E>(self, bytes: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        VarZeroVec::try_from_bytes(bytes).map_err(de::Error::custom)
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
        while let Some(value) = seq.next_element::<T>()? {
            vec.push(value);
        }
        Ok(vec.into())
    }
}

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<'de, 'a, T> Deserialize<'de> for VarZeroVec<'a, T>
where
    T: 'de + Deserialize<'de> + AsVarULE,
    <<T as AsVarULE>::VarULE as VarULE>::Error: fmt::Display,
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = VarZeroVecVisitor::default();
        if deserializer.is_human_readable() {
            deserializer.deserialize_seq(visitor)
        } else {
            deserializer.deserialize_bytes(visitor)
        }
    }
}

/// This impl can be made available by enabling the optional `serde` feature of the `zerovec` crate
impl<T> Serialize for VarZeroVec<'_, T>
where
    T: Serialize + AsVarULE + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let mut seq = serializer.serialize_seq(Some(self.len()))?;
            for value in self.iter() {
                // In the case of T=String this creates an unnecessary
                // allocation just to throw it away, but we cannot use this guarantee
                // for all T. We could potentially add a `serialize_unaligned_element`
                // method to AsVarULE, but since serialization performance is not
                // critical, this is currently not done.
                seq.serialize_element(&T::from_unaligned(value))?;
            }
            seq.end()
        } else if let Some(slice) = self.get_slice_for_borrowed() {
            serializer.serialize_bytes(&slice)
        } else {
            // This creates an additional Vec allocation to enable code reuse of
            // VarZeroVec::to_vec()'s. The alternative is to write a different
            // implementation of get_serializable_bytes() which enables us to pull
            // out the byte buffer components bit by bit and use serialize_seq + serialize_element
            let vec = VarZeroVec::get_serializable_bytes(&self.to_vec())
                .ok_or_else(|| ser::Error::custom("VarZeroVec too large to be serialized"))?;
            serializer.serialize_bytes(&vec)
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;

    // ["foo", "bar", "baz", "dolor", "quux", "lorem ipsum"];
    const BYTES: &[u8] = &[
        6, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 9, 0, 0, 0, 14, 0, 0, 0, 18, 0, 0, 0, 102,
        111, 111, 98, 97, 114, 98, 97, 122, 100, 111, 108, 111, 114, 113, 117, 117, 120, 108, 111,
        114, 101, 109, 32, 105, 112, 115, 117, 109,
    ];
    const JSON_STR: &str = "[\"foo\",\"bar\",\"baz\",\"dolor\",\"quux\",\"lorem ipsum\"]";
    const BINCODE_BUF: &[u8] = &[
        57, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 9, 0, 0, 0, 14, 0,
        0, 0, 18, 0, 0, 0, 102, 111, 111, 98, 97, 114, 98, 97, 122, 100, 111, 108, 111, 114, 113,
        117, 117, 120, 108, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109,
    ];

    // ["w", "ω", "文", "𑄃"]
    const NONASCII_STR: &[&str] = &["w", "ω", "文", "𑄃"];
    const NONASCII_BYTES: &[u8] = &[
        4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 119, 207, 137, 230, 150, 135,
        240, 145, 132, 131,
    ];
    #[test]
    fn test_serde_json() {
        let zerovec_orig: VarZeroVec<String> = VarZeroVec::try_from_bytes(BYTES).expect("parse");
        let json_str = serde_json::to_string(&zerovec_orig).expect("serialize");
        assert_eq!(JSON_STR, json_str);
        // VarZeroVec should deserialize from JSON to either Vec or VarZeroVec
        let vec_new: Vec<String> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to Vec");
        assert_eq!(zerovec_orig.to_vec(), vec_new);
        let zerovec_new: VarZeroVec<String> =
            serde_json::from_str(&json_str).expect("deserialize from buffer to VarZeroVec");
        assert_eq!(zerovec_orig.to_vec(), zerovec_new.to_vec());
        assert!(zerovec_new.get_slice_for_borrowed().is_none());
    }

    #[test]
    fn test_serde_bincode() {
        let zerovec_orig: VarZeroVec<String> = VarZeroVec::try_from_bytes(BYTES).expect("parse");
        let bincode_buf = bincode::serialize(&zerovec_orig).expect("serialize");
        assert_eq!(BINCODE_BUF, bincode_buf);
        let zerovec_new: VarZeroVec<String> =
            bincode::deserialize(&bincode_buf).expect("deserialize from buffer to VarZeroVec");
        assert_eq!(zerovec_orig.to_vec(), zerovec_new.to_vec());
        assert!(zerovec_new.get_slice_for_borrowed().is_some());
    }

    #[test]
    fn test_nonascii_bincode() {
        let src_vec = NONASCII_STR
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<_>>();
        let mut zerovec: VarZeroVec<String> =
            VarZeroVec::try_from_bytes(NONASCII_BYTES).expect("parse");
        assert_eq!(zerovec.to_vec(), src_vec);
        let bincode_buf = bincode::serialize(&zerovec).expect("serialize");
        let zerovec_result =
            bincode::deserialize::<VarZeroVec<String>>(&bincode_buf).expect("deserialize");
        assert_eq!(zerovec_result.to_vec(), src_vec);

        // try again with owned zerovec
        zerovec.make_mut();
        let bincode_buf = bincode::serialize(&zerovec).expect("serialize");
        let zerovec_result =
            bincode::deserialize::<VarZeroVec<String>>(&bincode_buf).expect("deserialize");
        assert_eq!(zerovec_result.to_vec(), src_vec);
    }
}
