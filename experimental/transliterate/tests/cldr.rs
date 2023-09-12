// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO(#3736): find a way to keep cldr_testData uptodate

use icu_locid::Locale;
use icu_provider::prelude::*;
use icu_transliterate::Transliterator;
use std::path::PathBuf;

#[test]
fn test_all_cldr() {
    let mut in_order =
        std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/cldr_testData"))
            .unwrap()
            .map(|x| x.unwrap().path())
            .collect::<Vec<_>>();
    in_order.sort();
    for path in in_order {
        if path.ends_with("_readme.txt") {
            continue;
        }
        let locale = path.file_stem().unwrap().to_str().unwrap();
        let locale: Locale = locale.parse().unwrap();
        let t = Transliterator::try_new_unstable(
            locale,
            &icu_provider_fs::FsDataProvider::try_new(concat!(
                std::env!("CARGO_MANIFEST_DIR"),
                "/../../provider/datagen/tests/data/json/"
            ))
            .unwrap()
            .as_deserializing(),
        )
        .unwrap();
        test_file(t, path);
    }
}

fn test_file(t: Transliterator, path: PathBuf) {
    let data = std::fs::read_to_string(&path).unwrap();
    let lines = data.lines().filter(|x| !x.starts_with('#'));
    let test_cases = lines
        .map(|l| l.split_once('\t').unwrap())
        .collect::<Vec<_>>();

    eprintln!(
        "Testing {:?} with {} test cases",
        path.file_stem().unwrap(),
        test_cases.len()
    );

    for (idx, (input, output)) in test_cases.into_iter().enumerate() {
        eprintln!(
            "Testing testcase {}! Input: {:?} Output: {:?}",
            idx + 1,
            input,
            output
        );
        let actual = t.transliterate(input.to_string());
        assert_eq!(
            actual,
            output,
            "Transliterator {:?} failed",
            path.file_stem().unwrap()
        );
        eprintln!("Passed testcase {}!", idx + 1);
    }
    eprintln!("Transliterator {:?} passed", path.file_stem().unwrap());
}
