use tinystr_neo::*;

// Tests largely adapted from `tinystr` crate
// https://github.com/zbraniecki/tinystr/blob/4e4eab55dd6bded7f29a18b41452c506c461716c/tests/serde.rs

macro_rules! test_roundtrip {
    ($f:ident, $n:literal, $val:expr, $bytes:expr) => {
        #[test]
        fn $f() {
            let tiny: TinyAsciiStr<$n> = $val.parse().unwrap();
            let json_string = serde_json::to_string(&tiny).unwrap();
            let expected_json = concat!("\"", $val, "\"");
            assert_eq!(json_string, expected_json);
            let recover: TinyAsciiStr<$n> = serde_json::from_str(&json_string).unwrap();
            assert_eq!(&*tiny, &*recover);

            let bin = bincode::serialize(&tiny).unwrap();
            assert_eq!(bin, &tiny.all_bytes()[..]);
            let debin: TinyAsciiStr<$n> = bincode::deserialize(&bin).unwrap();
            assert_eq!(&*tiny, &*debin);

            let post = postcard::to_stdvec(&tiny).unwrap();
            assert_eq!(post, &tiny.all_bytes()[..]);
            let debin: TinyAsciiStr<$n> = postcard::from_bytes(&post).unwrap();
            assert_eq!(&*tiny, &*debin);
        }
    };
}

test_roundtrip!(test_roundtrip4_1, 4, "en", [101, 110, 0, 0]);
test_roundtrip!(test_roundtrip4_2, 4, "Latn", [76, 97, 116, 110]);
test_roundtrip!(
    test_roundtrip8,
    8,
    "calendar",
    [99, 97, 108, 101, 110, 100, 97, 114]
);
test_roundtrip!(
    test_roundtrip16,
    16,
    "verylongstring",
    [118, 101, 114, 121, 108, 111, 110, 103, 115, 116, 114, 105, 110, 103, 0, 0]
);
test_roundtrip!(
    test_roundtrip10,
    11,
    "shortstring",
    [115, 104, 111, 114, 116, 115, 116, 114, 105, 110, 103]
);
test_roundtrip!(
    test_roundtrip30,
    24,
    "veryveryverylongstring",
    [
        118, 101, 114, 121, 118, 101, 114, 121, 118, 101, 114, 121, 108, 111, 110, 103, 115, 116,
        114, 105, 110, 103, 0, 0
    ]
);
