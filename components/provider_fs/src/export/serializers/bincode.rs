// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use super::Error;
use crate::manifest::SyntaxOption;
use bincode::config::Options as _;
use std::io;
use std::ops::Deref;

/// A serializer for Bincode.
pub struct Serializer {
    syntax: SyntaxOption,
}

/// Options bag for initializing a [`bincode::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Options {}

impl Default for Options {
    fn default() -> Self {
        Self {}
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
        obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
            &mut bincode::Serializer::new(
                &mut sink,
                bincode::config::DefaultOptions::new().with_fixint_encoding(),
            ),
        ))?;
        Ok(())
    }
}

impl Serializer {
    pub fn new(_options: Options) -> Self {
        Self {
            syntax: SyntaxOption::Bincode,
        }
    }
}
