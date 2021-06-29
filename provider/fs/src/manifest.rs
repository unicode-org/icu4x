// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::{Deserialize, Serialize};

/// File name of the manifest. The manifest always uses JSON, even if the serializer isn't JSON.
pub const MANIFEST_FILE: &str = "manifest.json";

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AliasOption {
    /// Do not de-duplicate data.
    NoAliases,
    /// De-duplicate data by using filesystem symlinks.
    Symlink,
    // TODO: Alias based on a field in the JSON file
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SyntaxOption {
    /// Serialize using JavaScript Object Notation (JSON).
    Json,
    /// Serialize using Bincode.
    Bincode,
    // Future: Consider adding a custom format option here.
    // Custom {
    //     file_extension: String,
    // }
}

impl SyntaxOption {
    /// Gets the file extension associated with the given syntax.
    pub fn get_file_extension(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Bincode => "bincode",
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Manifest {
    /// Strategy for de-duplicating locale data.
    pub aliasing: AliasOption,
    /// Which data serialization file format is used.
    pub syntax: SyntaxOption,
}
