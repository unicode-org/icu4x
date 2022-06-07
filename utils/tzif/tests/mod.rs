// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use combine::{stream, Parser};
use std::{fs::File, path::Path};
use tzif::parse::posix::posix_tz_string;
use tzif::parse::tzif::tzif;
use walkdir::WalkDir;

fn parse_tzif_file<P: AsRef<Path>>(path: P) {
    println!("parsing {:?}", path.as_ref().to_str());
    let file = File::open(path).unwrap();
    let stream = stream::buffered::Stream::new(
        stream::position::Stream::new(stream::read::Stream::new(file)),
        0, /* lookahead */
    );
    let parsed = tzif().parse(stream);
    assert!(parsed.is_ok());
    println!("{:#?}", parsed.unwrap().0);
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

#[test]
fn parse_posix_tz_string() {
    assert!(posix_tz_string()
        .parse(b"WGT3WGST,M3.5.0/-2,M10.5.0/-1".as_slice())
        .is_ok());
}
