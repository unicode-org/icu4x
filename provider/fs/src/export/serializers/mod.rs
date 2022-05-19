// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod bincode;
pub mod json;
pub mod postcard;

use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use std::io;

/// A simple serializer trait that works on whole objects.
pub trait AbstractSerializer {
    /// Serializes an object to a sink.
    fn serialize(
        &self,
        obj: DataPayload<SerializeMarker>,
        sink: &mut dyn io::Write,
    ) -> Result<(), DataError>;

    /// Gets the buffer format currently being serialized.
    fn get_buffer_format(&self) -> BufferFormat;
}
