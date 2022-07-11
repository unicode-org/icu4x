// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod bincode;
pub mod json;
pub mod postcard;

use icu_provider::buf::BufferFormat;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::io;

/// A simple serializer trait that works on whole objects.
pub trait AbstractSerializer {
    /// Serializes an object to a sink.
    fn serialize(
        &self,
        obj: &DataPayload<ExportMarker>,
        sink: &mut dyn io::Write,
    ) -> Result<(), DataError>;

    /// Gets the buffer format currently being serialized.
    fn get_buffer_format(&self) -> BufferFormat;

    /// This can be set to get correct CRLF on Windows.
    fn is_text_format(&self) -> bool {
        false
    }
}
