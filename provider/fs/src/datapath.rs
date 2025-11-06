// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::{Path, PathBuf};

use icu_provider::marker::DataMarkerId;

pub(crate) fn marker_to_path(marker: DataMarkerId, root: &Path) -> PathBuf {
    let mut path = PathBuf::from(root);
    let mut last = 0;
    for i in 1..marker.name().len() {
        if marker
            .name()
            .as_bytes()
            .get(i + 1)
            .is_none_or(|b| b.is_ascii_uppercase())
        {
            path.push(marker.name()[last..=i].to_ascii_lowercase());
            last = i + 1;
        }
    }
    path
}

#[test]
fn test_marker_to_path() {
    use icu_provider::hello_world::HelloWorldV1;
    use icu_provider::prelude::*;
    assert_eq!(
        marker_to_path(HelloWorldV1::INFO.id, Path::new("")),
        Path::new("hello/world/v1"),
    );
}
