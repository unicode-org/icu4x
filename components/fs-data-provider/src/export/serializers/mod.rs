pub mod json;

#[cfg(feature = "bincode")]
pub mod bincode;

use crate::manifest::SyntaxOption;
use std::io;
use std::ops::Deref;

/// An Error type specifically for the Serializer that doesn't carry filenames
pub enum Error {
    Io(io::Error),
    Serializer(erased_serde::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<erased_serde::Error> for Error {
    fn from(err: erased_serde::Error) -> Self {
        Self::Serializer(err)
    }
}

/// A simple serializer trait that works on whole objects.
pub trait AbstractSerializer: Deref<Target = SyntaxOption> {
    /// Serializes an object to a sink.
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error>;
}
