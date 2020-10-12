// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
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
pub trait Serializer: Deref<Target = SyntaxOption> {
    /// Serializes an object to a sink.
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error>;
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StyleOption {
    /// Print the smallest possible JSON, to reduce file size.
    Compact,
    /// Pretty-print JSON, to make it more readable.
    Pretty,
}

/// A serializer for JavaScript Object Notation (JSON).
pub struct JsonSerializer {
    syntax: SyntaxOption,
    style: StyleOption,
}

/// Options bag for initializing a JsonSerializer.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct JsonSerializerOptions {
    /// Format style to use when dumping output.
    pub style: StyleOption,
}

impl Default for JsonSerializerOptions {
    fn default() -> Self {
        Self {
            style: StyleOption::Compact,
        }
    }
}

impl Deref for JsonSerializer {
    type Target = SyntaxOption;

    fn deref(&self) -> &Self::Target {
        &self.syntax
    }
}

impl Serializer for JsonSerializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        mut sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        match self.style {
            StyleOption::Compact => {
                obj.erased_serialize(&mut erased_serde::Serializer::erase(
                    &mut serde_json::Serializer::new(&mut sink),
                ))?;
            }
            StyleOption::Pretty => {
                obj.erased_serialize(&mut erased_serde::Serializer::erase(
                    &mut serde_json::Serializer::pretty(&mut sink),
                ))?;
            }
        };
        // Write an empty line at the end of the document
        writeln!(sink)?;
        Ok(())
    }
}

impl JsonSerializer {
    pub fn new(options: &JsonSerializerOptions) -> Self {
        Self {
            syntax: SyntaxOption::Json,
            style: options.style,
        }
    }
}
