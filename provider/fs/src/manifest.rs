// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::buf::BufferFormat;
use icu_provider::prelude::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    /// Which data serialization file format is used.
    #[serde(rename = "syntax")]
    pub buffer_format: BufferFormat,
}

impl Manifest {
    /// File name of the manifest. The manifest always uses JSON, even if the serializer isn't JSON.
    pub const NAME: &'static str = "manifest.json";

    /// Gets the file extension associated with the given buffer format in the manifest.
    pub fn get_file_extension(&self) -> Result<&str, DataError> {
        match self.buffer_format {
            BufferFormat::Json => Ok("json"),
            BufferFormat::Bincode1 => Ok("bincode"),
            BufferFormat::Postcard07 => Ok("postcard"),
            // BufferFormat is non_exhaustive, so we need a catchall case.
            // This case could be triggered if a new buffer format is added to the core library
            // before it gets added to FsDataProvider.
            bf => Err(DataErrorKind::UnavailableBufferFormat(bf).into_error()),
        }
    }
}
