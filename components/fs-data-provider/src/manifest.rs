use serde::{Deserialize, Serialize};

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
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SyntaxOption {
    /// Serialize using JavaScript Object Notation (JSON).
    Json,
}

impl SyntaxOption {
    /// Gets the file extension normally associated with the given syntax
    pub fn get_file_extension(&self) -> &'static str {
        match self {
            SyntaxOption::Json => "json",
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Manifest {
    pub aliasing: AliasOption,
    pub syntax: SyntaxOption,
}
