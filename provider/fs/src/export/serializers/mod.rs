// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serializer configurations for the [`FilesystemExporter`].
//!
//! Each serializer is in a child module.
//!
//! [`FilesystemExporter`]: super::FilesystemExporter

#[doc(hidden)]
pub mod bincode;
#[doc(hidden)]
pub mod json;
#[doc(hidden)]
pub mod postcard;

pub use self::bincode::Serializer as Bincode;
pub use self::json::Serializer as Json;
pub use self::postcard::Serializer as Postcard;

use icu_provider::buf::BufferFormat;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::io;

/// A simple serializer trait that works on whole objects.
pub trait AbstractSerializer: core::fmt::Debug {
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
