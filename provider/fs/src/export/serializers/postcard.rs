// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use icu_provider::buf::BufferFormat;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Encoder, Flavor};
use std::io;

/// A serializer for Postcard.
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Serializer;

/// Options bag for initializing a [`postcard::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Options;

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &DataPayload<ExportMarker>,
        sink: &mut dyn io::Write,
    ) -> Result<(), DataError> {
        let mut serializer = postcard::Serializer {
            output: Encoder {
                flavor: AllocVec::new(),
            },
        };
        obj.serialize(&mut serializer)
            .map_err(|e| DataError::custom("Postcard serialize").with_display_context(&e))?;
        let output = serializer
            .output
            .flavor
            .release()
            .map_err(|_| DataError::custom("postcard release() failed"))?;
        sink.write_all(&output)?;
        Ok(())
    }

    fn get_buffer_format(&self) -> BufferFormat {
        BufferFormat::Postcard07
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
