// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: find a way to keep cldr_testData uptodate

use icu_provider::_internal::locid::Locale;
use icu_transliteration::Transliterator;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_all_cldr() {
    // broken BCP47 ids in CLDR
    let map = HashMap::from([("und-t-d0-ascii", "und-t-und-latn-d0-ascii")]);

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
        let locale = map.get(locale).unwrap_or(&locale);
        let locale: Locale = locale.parse().unwrap();
        let t = Transliterator::try_new(locale.clone()).unwrap();
        test_file(t, path);
    }
}

#[track_caller]
fn test_file(t: Transliterator, path: PathBuf) {
    let data = std::fs::read_to_string(&path).unwrap();
    let lines = data.lines().filter(|x| !x.starts_with('#'));
    let test_cases = lines
        .map(|l| {
            let mut parts = l.splitn(2, '\t');
            let input = parts.next().unwrap();
            let output = parts.next().unwrap();
            (input, output)
        })
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
