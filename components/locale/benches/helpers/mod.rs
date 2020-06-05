mod macros;

use std::fs::File;
use std::io::{BufReader, Error};

pub fn read_fixture<'l, T>(path: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
