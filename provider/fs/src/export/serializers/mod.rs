// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub mod json;

#[cfg(feature = "bincode")]
pub mod bincode;

use crate::manifest::SyntaxOption;
use std::io;
use std::ops::Deref;
use thiserror::Error;

/// An Error type specifically for the [`Serializer`](serde::Serializer) that doesn't carry filenames
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serializer(#[from] erased_serde::Error),
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
