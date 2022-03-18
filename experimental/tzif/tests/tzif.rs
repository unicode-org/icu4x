// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::Path;

use icu_provider_tzif::{parse::tzif::TzifParser, reader::file::FileByteReader};
use walkdir::WalkDir;

fn parse_tzif_file<P: AsRef<Path>>(path: P) -> eyre::Result<()> {
    println!("parsing {:?}", path.as_ref().to_str());
    let mut source = FileByteReader::try_from_path(path)?;
    let data = source.parse_tzif()?.into_value();
    println!("{:#?}", data);
    Ok(())
}

#[test]
fn all_iana_tzif_files() -> eyre::Result<()> {
    for entry in WalkDir::new("testdata").follow_links(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            parse_tzif_file(entry.path())?
        }
    }
    Ok(())
}
