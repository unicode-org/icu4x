// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use super::Error;
use bincode::config::Options as _;
use icu_provider::buf::BufferFormat;
use std::io;

/// A serializer for Bincode.
pub struct Serializer;

/// Options bag for initializing a [`bincode::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Options;

impl Default for Options {
    fn default() -> Self {
        Self {}
    }
}

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        mut sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
            &mut bincode::Serializer::new(
                &mut sink,
                bincode::config::DefaultOptions::new().with_fixint_encoding(),
            ),
        ))?;
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
