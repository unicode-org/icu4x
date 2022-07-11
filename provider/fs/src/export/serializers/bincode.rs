// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use bincode::config::Options as _;
use icu_provider::buf::BufferFormat;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::io;

/// A serializer for Bincode.
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Serializer;

/// Options bag for initializing a [`bincode::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Options;

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &DataPayload<ExportMarker>,
        sink: &mut dyn io::Write,
    ) -> Result<(), DataError> {
        obj.serialize(&mut bincode::Serializer::new(
            sink,
            bincode::config::DefaultOptions::new().with_fixint_encoding(),
        ))
        .map_err(|e| DataError::custom("Bincode serialize").with_display_context(&e))?;
        Ok(())
    }

    fn get_buffer_format(&self) -> BufferFormat {
        BufferFormat::Bincode1
    }
}

impl Serializer {
    pub fn new(_options: Options) -> Self {
        Self {}
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
