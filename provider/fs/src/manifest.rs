// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::buf::BufferFormat;

/// File name of the manifest. The manifest always uses JSON, even if the serializer isn't JSON.
pub const MANIFEST_FILE: &str = "manifest.json";

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AliasOption {
    /// Do not de-duplicate data.
    NoAliases,
    /// De-duplicate data by using filesystem symlinks.
    Symlink,
    // TODO: Alias based on a field in the JSON file
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct Manifest {
    /// Strategy for de-duplicating locale data.
    pub aliasing: AliasOption,
    /// Which data serialization file format is used.
    #[serde(rename = "syntax")]
    pub buffer_format: BufferFormat,
}

impl Manifest {
    /// Gets the file extension associated with the given buffer format in the manifest.
    pub(crate) fn get_file_extension(&self) -> &str {
        match self.buffer_format {
            BufferFormat::Json => "json",
            BufferFormat::Bincode1 => "bincode",
            BufferFormat::Postcard07 => "postcard",
            // BufferFormat is non_exhaustive, so we need a catchall case.
            // This case could be triggered if a new buffer format is added to the core library
            // before it gets added to FsDataProvider.
            _ => "und",
        }
    }
}
