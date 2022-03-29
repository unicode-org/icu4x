// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use zerovec::maps::ZeroMap2dBorrowed;

/// A versioned Serde schema for ICU4X data blobs.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BlobSchema<'data> {
    #[serde(borrow)]
    V001(BlobSchemaV1<'data>),
}

/// Version 1 of the ICU4X data blob schema.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BlobSchemaV1<'data> {
    /// Map from key hash and locale to buffer
    #[serde(borrow)]
    pub resources: ZeroMap2dBorrowed<'data, ResourceKeyHash, [u8], [u8]>,
}
