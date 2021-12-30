// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for data providers that produce opaque buffers.

use crate::prelude::*;

pub struct BufferMarker;

impl DataMarker for BufferMarker {
    type Yokeable = &'static [u8];
}

pub trait BufferProvider {
    fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError>;
}

/// An enum expressing all Serde formats known to ICU4X.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum BufferFormat {
    /// Serialize using JavaScript Object Notation (JSON).
    Json,
    /// Serialize using Bincode version 1.
    Bincode1,
    /// Serialize using Postcard version 0.7.
    Postcard07,
}
