use crate::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn open_reader(path: PathBuf) -> Result<BufReader<File>, Error> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(Error::IoError(err, path)),
    };
    Ok(BufReader::new(file))
}
