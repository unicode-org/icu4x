// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use icu_locid::LanguageIdentifier;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: &Path) -> Result<BufReader<File>, DatagenError> {
    log::trace!("Reading: {:?}", path);
    File::open(&path)
        .map(BufReader::new)
        .map_err(|e| (e, path).into())
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

fn get_langid_subdirectories_internal(
    root: &Path,
) -> Result<impl Iterator<Item = (LanguageIdentifier, PathBuf)>, DatagenError> {
    let mut result = vec![];
    for entry in fs::read_dir(root).map_err(|e| (e, root))? {
        let entry = entry.map_err(|e| (e, root))?;
        let path = entry.path();
        result.push(path);
    }
    Ok(result.into_iter().map(|path| {
        #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        (
            LanguageIdentifier::from_str(&path.file_name().unwrap().to_string_lossy()).unwrap(),
            path,
        )
    }))
}

/// Helper function which returns an unsorted list of langids for which subdirectories exist.
pub fn get_langid_subdirectories(
    root: &Path,
) -> Result<impl Iterator<Item = LanguageIdentifier>, DatagenError> {
    get_langid_subdirectories_internal(root).map(|iter| iter.map(|(l, _)| l))
}

/// Helper function which returns the subdirectory for the selected language, if it exists.
pub fn get_langid_subdirectory(
    root: &Path,
    langid: &LanguageIdentifier,
) -> Result<Option<PathBuf>, DatagenError> {
    get_langid_subdirectories_internal(root).map(|mut iter| {
        iter.find(|(langid2, _)| langid2 == langid)
            .map(|(_, path)| path)
    })
}
