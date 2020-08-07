use crate::error::Error;
use crate::manifest::SyntaxOption;
use erased_serde;
use std::io;

/// A simple serializer trait that works on whole objects.
pub trait Serializer {
    /// Serializes an object to a sink.
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error>;

    /// Gets syntax metadata associated with the Deserializer.
    fn get_syntax(&self) -> &SyntaxOption;
}

/// A serializer for JavaScript Object Notation (JSON).
pub struct JsonSerializer(SyntaxOption);

impl Default for JsonSerializer {
    fn default() -> Self {
        JsonSerializer(SyntaxOption::Json)
    }
}

impl Serializer for JsonSerializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        let mut json = serde_json::Serializer::new(sink);
        obj.erased_serialize(&mut erased_serde::Serializer::erase(&mut json))?;
        Ok(())
    }

    fn get_syntax(&self) -> &SyntaxOption {
        &self.0
    }
}
