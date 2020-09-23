use crate::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: PathBuf) -> Result<BufReader<File>, Error> {
    File::open(&path)
        .map(BufReader::new)
        .map_err(|e| Error::IoError(e, path))
}

/// Helper function which returns a sorted list of subdirectories.
pub fn get_subdirectories(root: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut result = vec![];
    for entry in fs::read_dir(root).map_err(|e| Error::IoError(e, root.to_path_buf()))? {
        let entry = entry.map_err(|e| Error::IoError(e, root.to_path_buf()))?;
        let path = entry.path();
        result.push(path);
    }
    result.sort();
    Ok(result)
}
