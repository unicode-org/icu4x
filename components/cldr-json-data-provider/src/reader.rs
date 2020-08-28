use crate::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Helper function to open a file and return failures as a crate error.
pub fn open_reader(path: PathBuf) -> Result<BufReader<File>, Error> {
    File::open(&path)
        .map(BufReader::new)
        .map_err(|e| Error::IoError(e, path))
}
