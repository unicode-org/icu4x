// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;
use icu_provider::serde::SerializeMarker;
use std::io::{self, Write};

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StyleOption {
    /// Print the smallest possible JSON, to reduce file size.
    Compact,
    /// Pretty-print JSON, to make it more readable.
    Pretty,
}

/// A serializer for JavaScript Object Notation (JSON).
pub struct Serializer {
    style: StyleOption,
}

/// Options bag for initializing a [`serde_json::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    /// Format style to use when dumping output.
    pub style: StyleOption,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            style: StyleOption::Compact,
        }
    }
}

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: DataPayload<SerializeMarker>,
        sink: &mut dyn io::Write,
    ) -> Result<(), DataError> {
        let mut sink = crlify::BufWriterWithLineEndingFix::new(sink);
        match self.style {
            StyleOption::Compact => obj.serialize(&mut serde_json::Serializer::new(&mut sink)),
            StyleOption::Pretty => obj.serialize(&mut serde_json::Serializer::pretty(&mut sink)),
        }?;
        // Write an empty line at the end of the document
        writeln!(sink)?;
        Ok(())
    }

    fn get_buffer_format(&self) -> BufferFormat {
        BufferFormat::Json
    }
}

impl Serializer {
    pub fn new(options: Options) -> Self {
        Self {
            style: options.style,
        }
    }

    pub fn pretty() -> Self {
        Self::new(Options {
            style: StyleOption::Pretty,
            ..Default::default()
        })
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
