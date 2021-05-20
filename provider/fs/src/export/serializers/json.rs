// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use super::Error;
use crate::manifest::SyntaxOption;
use std::io;
use std::ops::Deref;

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
    syntax: SyntaxOption,
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

impl Deref for Serializer {
    type Target = SyntaxOption;

    fn deref(&self) -> &Self::Target {
        &self.syntax
    }
}

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        mut sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        match self.style {
            StyleOption::Compact => {
                obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
                    &mut serde_json::Serializer::new(&mut sink),
                ))?;
            }
            StyleOption::Pretty => {
                obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
                    &mut serde_json::Serializer::pretty(&mut sink),
                ))?;
            }
        };
        // Write an empty line at the end of the document
        writeln!(sink)?;
        Ok(())
    }
}

impl Serializer {
    pub fn new(options: Options) -> Self {
        Self {
            syntax: SyntaxOption::Json,
            style: options.style,
        }
    }
}
