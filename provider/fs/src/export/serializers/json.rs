// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serializer configurations for [`serde_json`].

use super::AbstractSerializer;
use icu_provider::buf::BufferFormat;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use std::io;

/// Choices for how to render the JSON files.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StyleOption {
    /// Print the smallest possible JSON, to reduce file size.
    Compact,
    /// Pretty-print JSON, to make it more readable.
    Pretty,
}

/// A serializer for JavaScript Object Notation (JSON).
///
/// # Examples
///
/// ```
/// use icu_provider_fs::export::serializers;
/// use icu_provider_fs::export::FilesystemExporter;
///
/// let serializer = serializers::postcard::Serializer::new(Default::default());
///
/// // Then pass it to a FilesystemExporter:
/// let demo_path = std::env::temp_dir().join("icu4x_json_serializer_demo");
/// FilesystemExporter::try_new(
///     Box::from(serializer),
///     demo_path.clone().into(),
/// )
/// .unwrap();
/// std::fs::remove_dir_all(&demo_path).expect("Cleaning up test directory");
/// ```
#[derive(Debug)]
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
        obj: &DataPayload<ExportMarker>,
        mut sink: &mut dyn io::Write,
    ) -> Result<(), DataError> {
        match self.style {
            StyleOption::Compact => obj.serialize(&mut serde_json::Serializer::new(&mut sink)),
            StyleOption::Pretty => obj.serialize(&mut serde_json::Serializer::pretty(&mut sink)),
        }
        .map_err(|e| DataError::custom("JSON serialize").with_display_context(&e))?;
        // Write an empty line at the end of the document
        writeln!(sink)?;
        Ok(())
    }

    fn get_buffer_format(&self) -> BufferFormat {
        BufferFormat::Json
    }

    fn is_text_format(&self) -> bool {
        true
    }
}

impl Serializer {
    /// Creates a new serializer for [`serde_json`].
    pub fn new(options: Options) -> Self {
        Self {
            style: options.style,
        }
    }

    /// Convenience function to create a JSON serializer with the
    /// [`StyleOption::Pretty`] format.
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
