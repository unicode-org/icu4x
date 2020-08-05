use crate::Error;
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

    /// Gets the file extension typically associated with the serialization format.
    fn get_file_extension(&self) -> &'static str;
}

/// A serializer for JavaScript Object Notation (JSON).
pub struct JsonSerializer;

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

    fn get_file_extension(&self) -> &'static str {
        "json"
    }
}
