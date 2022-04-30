// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: &Path) -> Result<BufReader<File>, DatagenError> {
    log::trace!("Reading: {:?}", path);
    Ok(File::open(&path)
        .map(BufReader::new)
        .map_err(|e| (e, path))?)
}

/// Read the contents of the file at `path` and return it as a `String`.
pub fn read_path_to_string(path: &Path) -> Result<String, DatagenError> {
    let mut reader = open_reader(path)?;
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).map_err(|e| (e, path))?;
    Ok(buffer)
}

/// Helper function which returns a sorted list of the contents of a directory.
pub fn get_dir_contents(root: &Path) -> Result<Vec<PathBuf>, DatagenError> {
    let mut result = vec![];
    for entry in fs::read_dir(root).map_err(|e| (e, root))? {
        let path = entry.map_err(|e| (e, root))?.path();
        result.push(path);
    }
    result.sort();
    Ok(result)
}
