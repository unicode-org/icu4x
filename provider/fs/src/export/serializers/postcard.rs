// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use super::Error;
use icu_provider::buf::BufferFormat;
use std::io;

/// A serializer for Postcard.
pub struct Serializer;

/// Options bag for initializing a [`postcard::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Options;

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        let mut serializer = postcard::Serializer {
            output: postcard::flavors::StdVec(Vec::new()),
        };
        obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(&mut serializer))?;
        sink.write_all(&serializer.output.0)?;
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
