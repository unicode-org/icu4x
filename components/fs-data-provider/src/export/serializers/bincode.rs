use super::AbstractSerializer;
use super::Error;
use crate::manifest::SyntaxOption;
use std::io;
use std::ops::Deref;

/// A serializer for Bincode.
pub struct Serializer<T: bincode::config::Options + Clone> {
    syntax: SyntaxOption,
    bincode_options: T,
}

/// Options bag for initializing a bincode::Serializer.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Options<T: bincode::config::Options> {
    pub bincode_options: T,
}

impl Default for Options<bincode::config::DefaultOptions> {
    fn default() -> Self {
        Self {
            bincode_options: bincode::config::DefaultOptions::new(),
        }
    }
}

impl<T: bincode::config::Options + Clone> Deref for Serializer<T> {
    type Target = SyntaxOption;

    fn deref(&self) -> &Self::Target {
        &self.syntax
    }
}

impl<T: bincode::config::Options + Clone> AbstractSerializer for Serializer<T> {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        mut sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        obj.erased_serialize(&mut erased_serde::Serializer::erase(
            &mut bincode::Serializer::new(&mut sink, self.bincode_options.clone()),
        ))?;
        Ok(())
    }
}

impl<T: bincode::config::Options + Clone> Serializer<T> {
    pub fn new(options: Options<T>) -> Self {
        Self {
            syntax: SyntaxOption::Bincode,
            bincode_options: options.bincode_options,
        }
    }
}
