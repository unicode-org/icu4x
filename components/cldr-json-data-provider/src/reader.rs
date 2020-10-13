// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: &Path) -> Result<BufReader<File>, Error> {
    #[cfg(feature = "log")]
    log::trace!("Reading: {:?}", path);
    File::open(&path)
        .map(BufReader::new)
        .map_err(|e| (e, path).into())
}

/// Helper function which returns a sorted list of subdirectories.
pub fn get_subdirectories(root: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut result = vec![];
    for entry in fs::read_dir(root).map_err(|e| (e, root))? {
        let entry = entry.map_err(|e| (e, root))?;
        let path = entry.path();
        result.push(path);
    }
    result.sort();
    Ok(result)
}
