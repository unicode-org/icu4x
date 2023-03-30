// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use icu_provider_adapters::fork::ForkByKeyProvider;
use icu_provider_fs::FsDataProvider;
use icu_segmenter::WordSegmenter;
use std::path::PathBuf;

// Additional word segmenter tests with complex string.

fn get_segmenter_testdata_provider() -> impl BufferProvider {
    let segmenter_fs_provider = FsDataProvider::try_new(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/json"),
    )
    .unwrap();
    ForkByKeyProvider::new(segmenter_fs_provider, icu_testdata::buffer())
}

#[test]
fn word_break_th() {
    let segmenter_auto =
        WordSegmenter::try_new_auto_unstable(&icu_testdata::unstable()).expect("Data exists");
    let segmenter_lstm =
        WordSegmenter::try_new_lstm_unstable(&icu_testdata::unstable()).expect("Data exists");

    for segmenter in [segmenter_auto, segmenter_lstm] {
        // http://wpt.live/css/css-text/word-break/word-break-normal-th-000.html
        let s = "ภาษาไทยภาษาไทย";
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let iter = segmenter.segment_utf16(&utf16);
        assert_eq!(
            iter.collect::<Vec<usize>>(),
            vec![0, 4, 7, 11, 14],
            "word segmenter with Thai"
        );
        let iter = segmenter.segment_str(s);
        assert_eq!(
            iter.collect::<Vec<usize>>(),
            vec![0, 12, 21, 33, 42],
            "word segmenter with Thai"
        );

        // Combine non-Thai and Thai.
        let s = "aภาษาไทยภาษาไทยb";
        let utf16: Vec<u16> = s.encode_utf16().collect();
        let iter = segmenter.segment_utf16(&utf16);
        assert_eq!(
            iter.collect::<Vec<usize>>(),
            vec![0, 1, 5, 8, 12, 15, 16],
            "word segmenter with Thai and ascii"
        );
    }
}

#[test]
fn word_break_my() {
    let segmenter =
        WordSegmenter::try_new_auto_with_buffer_provider(&get_segmenter_testdata_provider())
            .expect("Data exists");

    let s = "မြန်မာစာမြန်မာစာမြန်မာစာ";
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let iter = segmenter.segment_utf16(&utf16);
    assert_eq!(
        iter.collect::<Vec<usize>>(),
        vec![0, 8, 16, 22, 24],
        "word segmenter with Burmese"
    );
}

#[test]
fn word_break_hiragana() {
    let segmenter_auto =
        WordSegmenter::try_new_auto_unstable(&icu_testdata::unstable()).expect("Data exists");
    let segmenter_dictionary =
        WordSegmenter::try_new_dictionary_unstable(&icu_testdata::unstable()).expect("Data exists");

    for segmenter in [segmenter_auto, segmenter_dictionary] {
        let s = "うなぎうなじ";
        let iter = segmenter.segment_str(s);
        assert_eq!(
            iter.collect::<Vec<usize>>(),
            vec![0, 9, 18],
            "word segmenter with Hiragana"
        );
    }
}

#[test]
fn word_break_mixed_han() {
    let segmenter_auto =
        WordSegmenter::try_new_auto_unstable(&icu_testdata::unstable()).expect("Data exists");
    let segmenter_dictionary =
        WordSegmenter::try_new_dictionary_unstable(&icu_testdata::unstable()).expect("Data exists");

    for segmenter in [segmenter_auto, segmenter_dictionary] {
        let s = "Welcome龟山岛龟山岛Welcome";
        let iter = segmenter.segment_str(s);
        assert_eq!(
            iter.collect::<Vec<usize>>(),
            vec![0, 7, 16, 25, 32],
            "word segmenter with Chinese and letter"
        );
    }
}
