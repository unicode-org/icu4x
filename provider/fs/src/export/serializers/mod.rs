// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod json;

#[cfg(feature = "bincode")]
pub mod bincode;

use displaydoc::Display;
use icu_provider::buf::BufferFormat;
use std::io;

/// An Error type specifically for the [`Serializer`](serde::Serializer) that doesn't carry filenames
#[derive(Display, Debug)]
pub enum Error {
    #[displaydoc("{0}")]
    Io(io::Error),
    #[displaydoc("{0}")]
    Serializer(erased_serde::Error),
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<erased_serde::Error> for Error {
    fn from(e: erased_serde::Error) -> Self {
        Error::Serializer(e)
    }
}

/// A simple serializer trait that works on whole objects.
pub trait AbstractSerializer {
    /// Serializes an object to a sink.
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error>;

    /// Gets the buffer format currently being serialized.
    fn get_buffer_format(&self) -> BufferFormat;
}
