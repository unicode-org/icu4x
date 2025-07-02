// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains helpers for zero-copy deserialization of slices other than `&[u8]`.

use serde::de::*;

/// TODO
pub fn option_utf_16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de potential_utf::PotentialUtf16>, D::Error> {
    let bytes = <Option<&[u8]>>::deserialize(deserializer)?;
    if let Some(bytes) = bytes {
        if bytes.as_ptr().align_offset(core::mem::align_of::<u16>()) != 0
            || bytes.len() % core::mem::size_of::<u16>() != 0
        {
            return Err(D::Error::custom("Wrong length or align"));
        }
        // Safety: The check gurantees length and alignment
        Ok(Some(potential_utf::PotentialUtf16::from_slice(unsafe {
            core::slice::from_raw_parts(
                bytes.as_ptr() as *const u16,
                bytes.len() / core::mem::size_of::<u16>(),
            )
        })))
    } else {
        Ok(None)
    }
}

/// TODO
pub fn vec_utf_16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<alloc::vec::Vec<&'de potential_utf::PotentialUtf16>, D::Error> {
    struct Utf16Visitor;

    impl<'de> Visitor<'de> for Utf16Visitor {
        type Value = alloc::vec::Vec<&'de potential_utf::PotentialUtf16>;

        fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
            write!(formatter, "a sequence of UTF-16 slices")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = alloc::vec::Vec::with_capacity(seq.size_hint().unwrap_or_default());
            while let Some(bytes) = seq.next_element::<&[u8]>()? {
                if bytes.as_ptr().align_offset(core::mem::align_of::<u16>()) != 0
                    || bytes.len() % core::mem::size_of::<u16>() != 0
                {
                    return Err(A::Error::custom("Wrong length or align"));
                }

                // Safety: The check gurantees length and alignment
                vec.push(potential_utf::PotentialUtf16::from_slice(unsafe {
                    core::slice::from_raw_parts(
                        bytes.as_ptr() as *const u16,
                        bytes.len() / core::mem::size_of::<u16>(),
                    )
                }));
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
    let bytes = <Option<&[u8]>>::deserialize(deserializer)?;
    if let Some(bytes) = bytes {
        if bytes.as_ptr().align_offset(core::mem::align_of::<i32>()) != 0
            || bytes.len() % core::mem::size_of::<i32>() != 0
        {
            return Err(D::Error::custom("Wrong length or align"));
        }
        // Safety: The check gurantees length and alignment
        Ok(Some(unsafe {
            core::slice::from_raw_parts(
                bytes.as_ptr() as *const i32,
                bytes.len() / core::mem::size_of::<i32>(),
            )
        }))
    } else {
        Ok(None)
    }
}

/// TODO
pub fn option_u32<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [u32]>, D::Error> {
    let bytes = <Option<&[u8]>>::deserialize(deserializer)?;
    if let Some(bytes) = bytes {
        if bytes.as_ptr().align_offset(core::mem::align_of::<u32>()) != 0
            || bytes.len() % core::mem::size_of::<u32>() != 0
        {
            return Err(D::Error::custom("Wrong length or align"));
        }
        // Safety: The check gurantees length and alignment
        Ok(Some(unsafe {
            core::slice::from_raw_parts(
                bytes.as_ptr() as *const u32,
                bytes.len() / core::mem::size_of::<u32>(),
            )
        }))
    } else {
        Ok(None)
    }
}

/// TODO
pub fn i32_tuple<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<&'de [(i32, i32)], D::Error> {
    let bytes = <&[u8]>::deserialize(deserializer)?;
    if bytes
        .as_ptr()
        .align_offset(core::mem::align_of::<(i32, i32)>())
        != 0
        || bytes.len() % core::mem::size_of::<(i32, i32)>() != 0
    {
        return Err(D::Error::custom("Wrong length or align"));
    }
    // Safety: The check gurantees length and alignment
    Ok(unsafe {
        core::slice::from_raw_parts(
            bytes.as_ptr() as *const (i32, i32),
            bytes.len() / core::mem::size_of::<(i32, i32)>(),
        )
    })
}

/// TODO
#[expect(clippy::type_complexity)] // serde...
pub fn option_i32_tuple<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<&'de [(i32, i32)]>, D::Error> {
    let bytes = <Option<&[u8]>>::deserialize(deserializer)?;
    if let Some(bytes) = bytes {
        if bytes
            .as_ptr()
            .align_offset(core::mem::align_of::<(i32, i32)>())
            != 0
            || bytes.len() % core::mem::size_of::<(i32, i32)>() != 0
        {
            return Err(D::Error::custom("Wrong length or align"));
        }
        // Safety: The check gurantees length and alignment
        Ok(Some(unsafe {
            core::slice::from_raw_parts(
                bytes.as_ptr() as *const (i32, i32),
                bytes.len() / core::mem::size_of::<(i32, i32)>(),
            )
        }))
    } else {
        Ok(None)
    }
}
