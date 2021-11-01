// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::WrapErr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: &Path) -> eyre::Result<BufReader<File>> {
    #[cfg(feature = "log")]
    log::trace!("Reading: {:?}", path);
    File::open(&path)
        .map(BufReader::new)
        .wrap_err_with(|| format!("Failed to open {}", path.display()))
}

/// Read the contents of the file at `path` and return it as a `String`.
pub fn read_path_to_string(path: &Path) -> eyre::Result<String> {
    let mut reader = open_reader(path)?;
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .wrap_err_with(|| format!("Failed to read {}", path.display()))?;
    Ok(buffer)
}

/// Helper function which returns a sorted list of the contents of a directory.
pub fn get_dir_contents(root: &Path) -> eyre::Result<Vec<PathBuf>> {
    let mut result = vec![];
    for entry in fs::read_dir(root).wrap_err_with(|| root.display().to_string())? {
        let path = entry.wrap_err_with(|| root.display().to_string())?.path();
        result.push(path);
    }
    result.sort();
    Ok(result)
}
