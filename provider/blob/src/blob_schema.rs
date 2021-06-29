// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use litemap::LiteMap;

/// A versioned Serde schema for ICU4X data blobs.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BlobSchema<'s> {
    #[serde(borrow)]
    V001(BlobSchemaV1<'s>),
}

/// Version 1 of the ICU4X data blob schema.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BlobSchemaV1<'s> {
    // TODO(#829): Use ZeroMap instead of LiteMap.
    #[serde(borrow)]
    pub resources: LiteMap<&'s str, &'s [u8]>,
}
