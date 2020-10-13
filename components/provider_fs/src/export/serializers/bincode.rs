use super::AbstractSerializer;
use super::Error;
use crate::manifest::SyntaxOption;
use bincode::config::Options as _;
use std::io;
use std::ops::Deref;

// See https://github.com/unicode-org/icu4x/issues/335
static_assertions::assert_cfg!(
    feature = "serialize_none",
    "The serialize_none feature must be enabled when exporting to bincode. See #335"
);

/// A serializer for Bincode.
pub struct Serializer {
    syntax: SyntaxOption,
}

/// Options bag for initializing a bincode::Serializer.
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
        obj.erased_serialize(&mut erased_serde::Serializer::erase(
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
