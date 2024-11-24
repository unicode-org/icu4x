// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use litemap::LiteMap;
use zerotrie::ZeroTriePerfectHash;
use zerotrie::ZeroTrieSimpleAscii;
use zerotrie::ByteStr;

mod testdata {
    use zerotrie::ByteStr;
    include!("data/data.rs");
}

use testdata::strings_to_litemap;

const NON_EXISTENT_STRINGS: &[&str] = &[
    "a9PS", "ahsY", "ahBO", "a8IN", "xk8o", "xv1l", "xI2S", "618y", "d6My", "uszy",
];

macro_rules! assert_bytes_eq {
    ($len:literal, $a:expr, $b:expr) => {
        assert_eq!($len, $a.len());
        assert_eq!($a, $b);
    };
}

fn check_simple_ascii_trie<S>(items: &LiteMap<&ByteStr, usize>, trie: &ZeroTrieSimpleAscii<S>)
where
    S: AsRef<[u8]> + ?Sized,
{
    // Check that each item is in the trie
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v));
    }
    // Check that some items are not in the trie
    for s in NON_EXISTENT_STRINGS.iter() {
        assert_eq!(trie.get(s.as_bytes()), None);
    }
    // Check that the iterator returns items in the same order as the LiteMap
    assert!(items
        .iter()
        .map(|(s, v)| (String::from_utf8(s.as_bytes().to_vec()).unwrap(), *v))
        .eq(trie.iter()));
    // Check that the const builder works
    let const_trie = ZeroTrieSimpleAscii::try_from_litemap_with_const_builder(items).unwrap();
    assert_eq!(trie.as_bytes(), const_trie.as_bytes());
}

fn check_phf_ascii_trie<S>(items: &LiteMap<&ByteStr, usize>, trie: &ZeroTriePerfectHash<S>)
where
    S: AsRef<[u8]> + ?Sized,
{
    // Check that each item is in the trie
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v));
    }
    // Check that some items are not in the trie
    for s in NON_EXISTENT_STRINGS.iter() {
        assert_eq!(trie.get(s.as_bytes()), None);
    }
    // Check that the iterator returns the contents of the LiteMap
    // Note: Since the items might not be in order, we collect them into a new LiteMap
    let recovered_items: LiteMap<Box<ByteStr>, usize> = trie.iter().map(|(k, v)| (ByteStr::from_boxed_bytes(k.into_boxed_slice()), v)).collect();
    assert_eq!(
        items.to_borrowed_keys_values::<ByteStr, usize, Vec<_>>(),
        recovered_items.to_borrowed_keys_values()
    );
}

fn check_phf_bytes_trie<S>(items: &LiteMap<&ByteStr, usize>, trie: &ZeroTriePerfectHash<S>)
where
    S: AsRef<[u8]> + ?Sized,
{
    // Check that each item is in the trie
    for (k, v) in items.iter() {
        assert_eq!(trie.get(k.as_bytes()), Some(*v), "{k:?}");
    }
    // Check that some items are not in the trie
    for s in NON_EXISTENT_STRINGS.iter() {
        assert_eq!(trie.get(s.as_bytes()), None, "{s:?}");
    }
    // Check that the iterator returns the contents of the LiteMap
    // Note: Since the items might not be in order, we collect them into a new LiteMap
    let recovered_items: LiteMap<Box<ByteStr>, usize> = trie.iter().map(|(k, v)| (ByteStr::from_boxed_bytes(k.into_boxed_slice()), v)).collect();
    assert_eq!(
        items.to_borrowed_keys_values::<ByteStr, usize, Vec<_>>(),
        recovered_items.to_borrowed_keys_values()
    );
}

#[test]
fn test_basic() {
    let lm1a: LiteMap<&ByteStr, usize> = testdata::basic::DATA_ASCII.iter().map(|(k, v)| (ByteStr::from_bytes(k), *v)).collect();
    let lm1b: LiteMap<&ByteStr, usize> = lm1a.to_borrowed_keys();
    let lm2: LiteMap<&ByteStr, usize> = testdata::basic::DATA_UNICODE.iter().map(|(k, v)| (ByteStr::from_bytes(k), *v)).collect();
    let lm3: LiteMap<&ByteStr, usize> = testdata::basic::DATA_BINARY.iter().map(|(k, v)| (ByteStr::from_bytes(k), *v)).collect();

    let expected_bytes = testdata::basic::TRIE_ASCII;
    let trie = ZeroTrieSimpleAscii::try_from(&lm1a).unwrap();
    assert_bytes_eq!(26, trie.as_bytes(), expected_bytes);
    check_simple_ascii_trie(&lm1a, &trie);

    let trie = ZeroTriePerfectHash::try_from(&lm1b).unwrap();
    assert_bytes_eq!(26, trie.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&lm1a, &trie);

    let expected_bytes = testdata::basic::TRIE_UNICODE;
    let trie = ZeroTriePerfectHash::try_from(&lm2).unwrap();
    assert_bytes_eq!(39, trie.as_bytes(), expected_bytes);
    check_phf_bytes_trie(&lm2, &trie);

    let expected_bytes = testdata::basic::TRIE_BINARY;
    let trie = ZeroTriePerfectHash::try_from(&lm3).unwrap();
    assert_bytes_eq!(26, trie.as_bytes(), expected_bytes);
    check_phf_bytes_trie(&lm3, &trie);
}

#[test]
fn test_empty() {
    let trie = ZeroTrieSimpleAscii::try_from(&LiteMap::<&ByteStr, usize>::new_vec()).unwrap();
    assert_eq!(trie.byte_len(), 0);
    assert!(trie.is_empty());
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.as_bytes(), &[]);
}

#[test]
fn test_single_empty_value() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str(""), 10), //
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), Some(10));
    assert_eq!(trie.get(b"x"), None);
    let expected_bytes = &[0b10001010];
    assert_eq!(trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(1, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_single_byte_string() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str("x"), 10), //
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', 0b10001010];
    assert_bytes_eq!(2, trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(2, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_single_string() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str("xyz"), 10), //
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"x"), None);
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', b'y', b'z', 0b10001010];
    assert_bytes_eq!(4, trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(4, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_prefix_strings() {
    let litemap: LiteMap<&ByteStr, usize> = [(ByteStr::from_str("x"), 0), (ByteStr::from_str("xy"), 1)].into_iter().collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xyz"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[b'x', 0b10000000, b'y', 0b10000001];
    assert_bytes_eq!(4, trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(4, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_single_byte_branch() {
    let litemap: LiteMap<&ByteStr, usize> = [(ByteStr::from_str("x"), 0), (ByteStr::from_str("y"), 1)].into_iter().collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"xy"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[0b11000010, b'x', b'y', 1, 0b10000000, 0b10000001];
    assert_bytes_eq!(6, trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(6, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_multi_byte_branch() {
    let litemap: LiteMap<&ByteStr, usize> = [(ByteStr::from_str("axb"), 0), (ByteStr::from_str("ayc"), 1)].into_iter().collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[
        b'a', 0b11000010, b'x', b'y', 2, b'b', 0b10000000, b'c', 0b10000001,
    ];
    assert_bytes_eq!(9, trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(9, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_linear_varint_values() {
    let litemap: LiteMap<&ByteStr, usize> = [(ByteStr::from_str(""), 100), (ByteStr::from_str("x"), 500), (ByteStr::from_str("xyz"), 5000)]
        .into_iter()
        .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b"xy"), None);
    assert_eq!(trie.get(b"xz"), None);
    assert_eq!(trie.get(b"xyzz"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[0x90, 0x54, b'x', 0x93, 0x64, b'y', b'z', 0x90, 0x96, 0x78];
    assert_bytes_eq!(10, trie.as_bytes(), expected_bytes);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(10, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_bug() {
    let litemap: LiteMap<&ByteStr, usize> = [(ByteStr::from_str("abc"), 100), (ByteStr::from_str("abcd"), 500), (ByteStr::from_str("abcde"), 5000)]
        .into_iter()
        .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b"ab"), None);
    assert_eq!(trie.get(b"abd"), None);
    assert_eq!(trie.get(b"abCD"), None);
    check_simple_ascii_trie(&litemap, &trie);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_varint_branch() {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let litemap: LiteMap<&ByteStr, usize> = (0..chars.len())
        .map(|i| (ByteStr::from_str(chars.get(i..i + 1).unwrap()), i))
        .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_simple_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11100000, // branch varint lead
        0x14,       // branch varint trail
        // search array:
        b'A', b'a', b'B', b'b', b'C', b'c', b'D', b'd', b'E', b'e',
        b'F', b'f', b'G', b'g', b'H', b'h', b'I', b'i', b'J', b'j',
        b'K', b'k', b'L', b'l', b'M', b'm', b'N', b'n', b'O', b'o',
        b'P', b'p', b'Q', b'q', b'R', b'r', b'S', b's', b'T', b't',
        b'U', b'u', b'V', b'v', b'W', b'w', b'X', b'x', b'Y', b'y',
        b'Z', b'z',
        // offset array:
        1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24,
        25, 27, 28, 30, 31, 33, 34, 36, 37, 39, 40, 42, 43, 45, 46,
        48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76,
        78, 80, 82, 84, 86,
        // values (mix of single-byte and multi-byte):
        (0x80 | 0),  0x90, 10, (0x80 | 1),  0x90, 11, (0x80 | 2),  0x90, 12,
        (0x80 | 3),  0x90, 13, (0x80 | 4),  0x90, 14, (0x80 | 5),  0x90, 15,
        (0x80 | 6),  0x90, 16, (0x80 | 7),  0x90, 17, (0x80 | 8),  0x90, 18,
        (0x80 | 9),  0x90, 19, (0x80 | 10), 0x90, 20, (0x80 | 11), 0x90, 21,
        (0x80 | 12), 0x90, 22, (0x80 | 13), 0x90, 23, (0x80 | 14), 0x90, 24,
        (0x80 | 15), 0x90, 25,
        0x90, 0, 0x90, 26, 0x90, 1, 0x90, 27, 0x90, 2, 0x90, 28,
        0x90, 3, 0x90, 29, 0x90, 4, 0x90, 30, 0x90, 5, 0x90, 31,
        0x90, 6, 0x90, 32, 0x90, 7, 0x90, 33, 0x90, 8, 0x90, 34,
        0x90, 9, 0x90, 35,
    ];
    assert_bytes_eq!(193, trie.as_bytes(), expected_bytes);

    #[rustfmt::skip]
    let expected_bytes = &[
        0b11100000, // branch varint lead
        0x14,       // branch varint trail
        // PHF metadata:
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 10, 12, 16, 4, 4, 4, 4, 4, 4, 8,
        4, 4, 4, 16, 16, 16, 16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 7,
        // search array:
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'u', b'v', b'w', b'D', b'E', b'F', b'q',
        b'r', b'A', b'B', b'C', b'x', b'y', b'z', b's',
        b'H', b'I', b'J', b'G', b'P', b'Q', b'R', b'S',
        b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'K',
        b'L', b'M', b'N', b'O', b'g', b'a', b'b', b'c',
        b't', b'd', b'f', b'e',
        // offset array:
        2, 4, 6, 8, 10, 12, 14,
        16, 18, 20, 22, 24, 25, 26, 27,
        29, 31, 32, 33, 34, 36, 38, 40,
        42, 43, 44, 45, 46, 47, 49, 51,
        53, 55, 57, 59, 61, 63, 65, 67,
        68, 69, 70, 71, 72, 74, 76, 78,
        80, 82, 84, 86,
        // values:
        0x90, 17, 0x90, 18, 0x90, 19, 0x90, 20, 0x90, 21, 0x90, 22, 0x90, 23,
        0x90, 24, 0x90, 25, 0x90, 30, 0x90, 31, 0x90, 32, 0x80 | 3, 0x80 | 4,
        0x80 | 5, 0x90, 26, 0x90, 27, 0x80, 0x80 | 1, 0x80 | 2, 0x90, 33,
        0x90, 34, 0x90, 35, 0x90, 28, 0x80 | 7, 0x80 | 8, 0x80 | 9, 0x80 | 6,
        0x80 | 15, 0x90, 0, 0x90, 1, 0x90, 2, 0x90, 3, 0x90, 4, 0x90, 5,
        0x90, 6, 0x90, 7, 0x90, 8, 0x90, 9, 0x80 | 10, 0x80 | 11, 0x80 | 12,
        0x80 | 13, 0x80 | 14, 0x90, 16, 0x90, 10, 0x90, 11, 0x90, 12, 0x90, 29,
        0x90, 13, 0x90, 15, 0x90, 14,
    ];
    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(246, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);
}

#[test]
fn test_below_wide() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str("abcdefghijklmnopqrstuvwxyz"), 1),
        (ByteStr::from_str("bcdefghijklmnopqrstuvwxyza"), 2),
        (ByteStr::from_str("cdefghijklmnopqrstuvwxyzab"), 3),
        (ByteStr::from_str("defghijklmnopqrstuvwxyzabc"), 4),
        (ByteStr::from_str("efghijklmnopqrstuvwxyzabcd"), 5),
        (ByteStr::from_str("fghijklmnopqrstuvwxyzabcde"), 6),
        (ByteStr::from_str("ghijklmnopqrstuvwxyzabcdef"), 7),
        (ByteStr::from_str("hijklmnopqrstuvwxyzabcdefg"), 8),
        (ByteStr::from_str("ijklmnopqrstuvwxyzabcdefgh"), 9),
        (ByteStr::from_str("jklmnopqrstuvwxyzabcd"), 10),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_simple_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11001010, // branch
        // search array:
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        // offset array:
        26, 52, 78, 104, 130, 156, 182, 208, 234,
        // offset data:
        b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0x81,
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0x82,
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0x83,
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
        b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
        0x84,
        b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
        b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0x85,
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
        0x86,
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0x87,
        b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
        b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
        0x88,
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
        0x89,
        b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
        b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0x8A,
    ];
    assert_bytes_eq!(275, trie.as_bytes(), expected_bytes);
}

#[test]
fn test_at_wide() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str("abcdefghijklmnopqrstuvwxyz"), 1),
        (ByteStr::from_str("bcdefghijklmnopqrstuvwxyza"), 2),
        (ByteStr::from_str("cdefghijklmnopqrstuvwxyzab"), 3),
        (ByteStr::from_str("defghijklmnopqrstuvwxyzabc"), 4),
        (ByteStr::from_str("efghijklmnopqrstuvwxyzabcd"), 5),
        (ByteStr::from_str("fghijklmnopqrstuvwxyzabcde"), 6),
        (ByteStr::from_str("ghijklmnopqrstuvwxyzabcdef"), 7),
        (ByteStr::from_str("hijklmnopqrstuvwxyzabcdefg"), 8),
        (ByteStr::from_str("ijklmnopqrstuvwxyzabcdefgh"), 9),
        (ByteStr::from_str("jklmnopqrstuvwxyzabcde"), 10),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_simple_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11100001, // branch lead
        0x6A, // branch trail
        // search array:
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        // offset array (wide):
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        26, 52, 78, 104, 130, 156, 182, 208, 234,
        // offset data:
        b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0x81,
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0x82,
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0x83,
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
        b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
        0x84,
        b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
        b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0x85,
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
        0x86,
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0x87,
        b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
        b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
        0x88,
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
        0x89,
        b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
        b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
        0x8A,
    ];
    assert_bytes_eq!(286, trie.as_bytes(), expected_bytes);
}

#[test]
fn test_at_wide_plus() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str("abcdefghijklmnopqrstuvwxyz"), 1),
        (ByteStr::from_str("bcdefghijklmnopqrstuvwxyza"), 2),
        (ByteStr::from_str("cdefghijklmnopqrstuvwxyzab"), 3),
        (ByteStr::from_str("defghijklmnopqrstuvwxyzabc"), 4),
        (ByteStr::from_str("efghijklmnopqrstuvwxyzabcd"), 5),
        (ByteStr::from_str("fghijklmnopqrstuvwxyzabcde"), 6),
        (ByteStr::from_str("ghijklmnopqrstuvwxyzabcdef"), 7),
        (ByteStr::from_str("hijklmnopqrstuvwxyzabcdefg"), 8),
        (ByteStr::from_str("ijklmnopqrstuvwxyzabcdefgh"), 9),
        (ByteStr::from_str("jklmnopqrstuvwxyzabcdef"), 10),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), None);
    assert_eq!(trie.get(b"abc"), None);
    check_simple_ascii_trie(&litemap, &trie);
    #[rustfmt::skip]
    let expected_bytes = &[
        0b11100001, // branch lead
        0x6A, // branch trail
        // search array:
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
        // offset array (wide):
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        26, 52, 78, 104, 130, 156, 182, 208, 234,
        // offset data:
        b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        0x81,
        b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a',
        0x82,
        b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
        b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b',
        0x83,
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q',
        b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c',
        0x84,
        b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
        b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd',
        0x85,
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e',
        0x86,
        b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0x87,
        b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u',
        b'v', b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g',
        0x88,
        b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h',
        0x89,
        b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w',
        b'x', b'y', b'z', b'a', b'b', b'c', b'd', b'e', b'f',
        0x8A,
    ];
    assert_bytes_eq!(287, trie.as_bytes(), expected_bytes);
}

#[test]
fn test_everything() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str(""), 0),
        (ByteStr::from_str("axb"), 100),
        (ByteStr::from_str("ayc"), 2),
        (ByteStr::from_str("azd"), 3),
        (ByteStr::from_str("bxe"), 4),
        (ByteStr::from_str("bxefg"), 500),
        (ByteStr::from_str("bxefh"), 6),
        (ByteStr::from_str("bxei"), 7),
        (ByteStr::from_str("bxeikl"), 8),
    ]
    .into_iter()
    .collect();
    let trie = ZeroTrieSimpleAscii::try_from(&litemap.as_sliced()).unwrap();
    assert_eq!(trie.get(b""), Some(0));
    assert_eq!(trie.get(b"a"), None);
    assert_eq!(trie.get(b"ax"), None);
    assert_eq!(trie.get(b"ay"), None);
    check_simple_ascii_trie(&litemap, &trie);
    let expected_bytes = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        b'a',       //
        b'b',       //
        13,         //
        0b11000011, // branch of 3
        b'x',       //
        b'y',       //
        b'z',       //
        3,          //
        5,          //
        b'b',       //
        0b10010000, // value 100 (lead)
        0x54,       // value 100 (trail)
        b'c',       //
        0b10000010, // value 2
        b'd',       //
        0b10000011, // value 3
        b'x',       //
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        b'f',       //
        b'i',       //
        7,          //
        0b11000010, // branch of 2
        b'g',       //
        b'h',       //
        2,          //
        0b10010011, // value 500 (lead)
        0x64,       // value 500 (trail)
        0b10000110, // value 6
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
    ];
    assert_bytes_eq!(36, trie.as_bytes(), expected_bytes);

    #[rustfmt::skip]
    let expected_bytes = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        b'a',       //
        b'b',       //
        13,         //
        0b11000011, // start of 'a' subtree: branch of 3
        b'x',       //
        b'y',       //
        b'z',       //
        3,          //
        5,          //
        b'b',       //
        0b10010000, // value 100 (lead)
        0x54,       // value 100 (trail)
        b'c',       //
        0b10000010, // value 2
        b'd',       //
        0b10000011, // value 3
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        b'f',       //
        b'i',       //
        7,          //
        0b11000010, // branch of 2
        b'g',       //
        b'h',       //
        2,          //
        0b10010011, // value 500 (lead)
        0x64,       // value 500 (trail)
        0b10000110, // value 6
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
    ];
    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(36, trie_phf.as_bytes(), expected_bytes);
    check_phf_ascii_trie(&litemap, &trie_phf);

    let zhm: zerovec::ZeroMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 73);

    let zhm: zerovec::ZeroMap<[u8], u8> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b as u8)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 63);

    let zhm: zerovec::ZeroHashMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 146);

    let zhm: zerovec::ZeroHashMap<[u8], u8> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b as u8)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 136);
}

macro_rules! utf8_byte {
    ($ch:expr, $i:literal) => {{
        let mut utf8_encoder_buf = [0u8; 4];
        $ch.encode_utf8(&mut utf8_encoder_buf);
        utf8_encoder_buf[$i]
    }};
}

#[test]
fn test_non_ascii() {
    let litemap: LiteMap<&ByteStr, usize> = [
        (ByteStr::from_str(""), 0),
        (ByteStr::from_str("axb"), 100),
        (ByteStr::from_str("ayc"), 2),
        (ByteStr::from_str("azd"), 3),
        (ByteStr::from_str("bxe"), 4),
        (ByteStr::from_str("bxefg"), 500),
        (ByteStr::from_str("bxefh"), 6),
        (ByteStr::from_str("bxei"), 7),
        (ByteStr::from_str("bxeikl"), 8),
        (ByteStr::from_str("bxeiklmΚαλημέρααα"), 9),
        (ByteStr::from_str("bxeiklmαnλo"), 10),
        (ByteStr::from_str("bxeiklmη"), 11),
    ]
    .into_iter()
    .collect();

    #[rustfmt::skip]
    let expected_bytes = &[
        0b10000000, // value 0
        0b11000010, // branch of 2
        b'a',       //
        b'b',       //
        13,         //
        0b11000011, // start of 'a' subtree: branch of 3
        b'x',       //
        b'y',       //
        b'z',       //
        3,          //
        5,          //
        b'b',       //
        0b10010000, // value 100 (lead)
        0x54,       // value 100 (trail)
        b'c',       //
        0b10000010, // value 2
        b'd',       //
        0b10000011, // value 3
        b'x',       // start of 'b' subtree
        b'e',       //
        0b10000100, // value 4
        0b11000010, // branch of 2
        b'f',       //
        b'i',       //
        7,          //
        0b11000010, // branch of 2
        b'g',       //
        b'h',       //
        2,          //
        0b10010011, // value 500 (lead)
        0x64,       // value 500 (trail)
        0b10000110, // value 6
        0b10000111, // value 7
        b'k',       //
        b'l',       //
        0b10001000, // value 8
        b'm',       //
        0b10100001, // span of length 1
        utf8_byte!('Κ', 0), // NOTE: all three letters have the same lead byte
        0b11000011, // branch of 3
        utf8_byte!('α', 1),
        utf8_byte!('η', 1),
        utf8_byte!('Κ', 1),
        6,
        7,
        // 21,
        // 27,
        b'n',
        0b10100010, // span of length 2
        utf8_byte!('λ', 0),
        utf8_byte!('λ', 1),
        b'o',
        0b10001010, // value 10
        0b10001011, // value 11
        0b10110000, // span of length 18 (lead)
        0b00000010, // span of length 18 (trail)
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        utf8_byte!('λ', 0),
        utf8_byte!('λ', 1),
        utf8_byte!('η', 0),
        utf8_byte!('η', 1),
        utf8_byte!('μ', 0),
        utf8_byte!('μ', 1),
        utf8_byte!('έ', 0),
        utf8_byte!('έ', 1),
        utf8_byte!('ρ', 0),
        utf8_byte!('ρ', 1),
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        utf8_byte!('α', 0),
        utf8_byte!('α', 1),
        0b10001001, // value 9
    ];
    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_bytes_eq!(73, trie_phf.as_bytes(), expected_bytes);
    check_phf_bytes_trie(&litemap, &trie_phf);
}

#[test]
fn test_max_branch() {
    // Evaluate a branch with all 256 possible children
    let mut litemap: LiteMap<&ByteStr, usize> = LiteMap::new_vec();
    let all_bytes: Vec<u8> = (u8::MIN..=u8::MAX).collect();
    assert_eq!(all_bytes.len(), 256);
    let all_bytes_prefixed: Vec<[u8; 2]> = (u8::MIN..=u8::MAX).map(|x| [b'\0', x]).collect();
    for b in all_bytes.iter() {
        litemap.insert(ByteStr::from_bytes(core::slice::from_ref(b)), *b as usize);
    }
    for s in all_bytes_prefixed.iter() {
        litemap.insert(ByteStr::from_bytes(s), s[1] as usize);
    }
    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_eq!(trie_phf.byte_len(), 3042);
    check_phf_bytes_trie(&litemap, &trie_phf);
}

#[test]
fn test_short_subtags_10pct() {
    let litemap = strings_to_litemap(testdata::short_subtags_10pct::STRINGS);

    let trie = ZeroTrieSimpleAscii::try_from(&litemap).unwrap();
    assert_eq!(trie.byte_len(), 1050);
    check_simple_ascii_trie(&litemap, &trie);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_eq!(trie_phf.byte_len(), 1100);
    check_phf_ascii_trie(&litemap, &trie_phf);

    let zhm: zerovec::ZeroMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 1329);

    let zhm: zerovec::ZeroMap<[u8], u8> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b as u8)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 1328);

    let zhm: zerovec::ZeroHashMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 2835);

    let zhm: zerovec::ZeroHashMap<[u8], u8> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b as u8)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 2834);
}

#[test]
fn test_short_subtags() {
    let litemap = strings_to_litemap(testdata::short_subtags::STRINGS);

    let trie = ZeroTrieSimpleAscii::try_from(&litemap).unwrap();
    assert_eq!(trie.byte_len(), 8793);
    check_simple_ascii_trie(&litemap, &trie);

    let trie_phf = ZeroTriePerfectHash::try_from(&litemap).unwrap();
    assert_eq!(trie_phf.byte_len(), 9400);
    check_phf_ascii_trie(&litemap, &trie_phf);

    let zm: zerovec::ZeroMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zm).unwrap();
    assert_eq!(zhm_buf.len(), 15180);

    let zm: zerovec::ZeroMap<[u8], u8> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b as u8)).collect();
    let zhm_buf = postcard::to_allocvec(&zm).unwrap();
    assert_eq!(zhm_buf.len(), 13302);

    let zhm: zerovec::ZeroHashMap<[u8], usize> = litemap.iter().map(|(a, b)| (a.as_bytes(), b)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 30198);

    let zhm: zerovec::ZeroHashMap<[u8], u8> = litemap.iter().map(|(a, b)| (a.as_bytes(), *b as u8)).collect();
    let zhm_buf = postcard::to_allocvec(&zhm).unwrap();
    assert_eq!(zhm_buf.len(), 28320);
}
