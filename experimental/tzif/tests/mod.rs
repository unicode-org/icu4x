use std::{fs::File, path::Path};
use tzif::parse::tzif::tzif;

use combine::{stream, Parser};
use walkdir::WalkDir;

fn parse_tzif_file<P: AsRef<Path>>(path: P) {
    println!("parsing {:?}", path.as_ref().to_str());
    let read = File::open(path).unwrap();
    let stream = stream::buffered::Stream::new(
        stream::position::Stream::new(stream::read::Stream::new(read)),
        1,
    );
    let parsed = tzif().parse(stream);
    assert!(parsed.is_ok());
    //println!("{:#?}", parsed.unwrap().0);
}

#[test]
fn parse_tzif_testdata() {
    for entry in WalkDir::new("testdata").follow_links(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            parse_tzif_file(entry.path())
        }
    }
}
