// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod chars;
mod plain;
pub mod samples;

pub use chars::CharULE;
pub use plain::PlainOldULE;

pub trait ULE
where
    Self: Sized,
{
    type Error;

    fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error>;
    fn as_byte_slice(slice: &[Self]) -> &[u8];
}

pub trait AsULE {
    type ULE: ULE;

    fn as_unaligned(&self) -> Self::ULE;
    fn from_unaligned(unaligned: &Self::ULE) -> Self;
}
