// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Internal helper functions.

use alloc::string::String;

/// Prints a JSON-safe string to the output.
pub fn escape_for_json<'o>(input: &str, output: &'o mut String) -> &'o mut String {
    // From the ECMA-404 specification:
    // "A string is a sequence of Unicode code points wrapped with quotation marks (U+0022).
    // All code points may be placed within the quotation marks except for the code points
    // that must be escaped: quotation mark (U+0022), reverse solidus (U+005C), and the
    // control characters U+0000 to U+001F. There are two-character escape sequence
    // representations of some characters."
    for cp in input.chars() {
        let str_to_append = match cp {
            '\u{0000}' => "\\u0000",
            '\u{0001}' => "\\u0001",
            '\u{0002}' => "\\u0002",
            '\u{0003}' => "\\u0003",
            '\u{0004}' => "\\u0004",
            '\u{0005}' => "\\u0005",
            '\u{0006}' => "\\u0006",
            '\u{0007}' => "\\u0007",
            '\u{0008}' => "\\b",
            '\u{0009}' => "\\t",
            '\u{000A}' => "\\n",
            '\u{000B}' => "\\u000B",
            '\u{000C}' => "\\f",
            '\u{000D}' => "\\r",
            '\u{000E}' => "\\u000E",
            '\u{000F}' => "\\u000F",
            '\u{0010}' => "\\u0010",
            '\u{0011}' => "\\u0011",
            '\u{0012}' => "\\u0012",
            '\u{0013}' => "\\u0013",
            '\u{0014}' => "\\u0014",
            '\u{0015}' => "\\u0015",
            '\u{0016}' => "\\u0016",
            '\u{0017}' => "\\u0017",
            '\u{0018}' => "\\u0018",
            '\u{0019}' => "\\u0019",
            '\u{001A}' => "\\u001A",
            '\u{001B}' => "\\u001B",
            '\u{001C}' => "\\u001C",
            '\u{001D}' => "\\u001D",
            '\u{001E}' => "\\u001E",
            '\u{001F}' => "\\u001F",
            '\u{0022}' => "\\\"",
            '\u{005C}' => "\\\\",
            cp => {
                output.push(cp);
                continue;
            }
        };
        output.push_str(str_to_append);
    }
    output
}

#[test]
fn test_escape_for_json() {
    assert_eq!("", escape_for_json("", &mut String::new()));
    assert_eq!("abc", escape_for_json("abc", &mut String::new()));
    assert_eq!("ab\\nc", escape_for_json("ab\nc", &mut String::new()));
    assert_eq!("ab\\\\c", escape_for_json("ab\\c", &mut String::new()));
    assert_eq!("ab\\\"c", escape_for_json("ab\"c", &mut String::new()));
    assert_eq!(
        "ab\\u0000c",
        escape_for_json("ab\u{0000}c", &mut String::new())
    );
    assert_eq!(
        "ab\\u001Fc",
        escape_for_json("ab\u{001F}c", &mut String::new())
    );
}

/// Const function to compute the FxHash of a byte array with little-endian byte order.
///
/// FxHash is a speedy hash algorithm used within rustc. The algorithm is satisfactory for our
/// use case since the strings being hashed originate from a trusted source (the ICU4X
/// components), and the hashes are computed at compile time, so we can check for collisions.
///
/// We could have considered a SHA or other cryptographic hash function. However, we are using
/// FxHash because:
///
/// 1. There is precedent for this algorithm in Rust
/// 2. The algorithm is easy to implement as a const function
/// 3. The amount of code is small enough that we can reasonably keep the algorithm in-tree
/// 4. FxHash is designed to output 32-bit or 64-bit values, whereas SHA outputs more bits,
///    such that truncation would be required in order to fit into a u32, partially reducing
///    the benefit of a cryptographically secure algorithm
pub const fn fxhash_32(bytes: &[u8]) -> u32 {
    // This code is adapted from https://github.com/rust-lang/rustc-hash,
    // whose license text is reproduced below.
    //
    // Copyright 2015 The Rust Project Developers. See the COPYRIGHT
    // file at the top-level directory of this distribution and at
    // http://rust-lang.org/COPYRIGHT.
    //
    // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
    // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
    // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
    // option. This file may not be copied, modified, or distributed
    // except according to those terms.

    #[inline]
    const fn hash_word_32(mut hash: u32, word: u32) -> u32 {
        const ROTATE: u32 = 5;
        const SEED32: u32 = 0x9e_37_79_b9;
        hash = hash.rotate_left(ROTATE);
        hash ^= word;
        hash = hash.wrapping_mul(SEED32);
        hash
    }

    let mut cursor = 0;
    let mut hash = 0;

    while bytes.len() - cursor >= 4 {
        let word = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]);
        hash = hash_word_32(hash, word);
        cursor += 4;
    }

    if bytes.len() - cursor >= 2 {
        let word = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        hash = hash_word_32(hash, word as u32);
        cursor += 2;
    }

    if bytes.len() - cursor >= 1 {
        hash = hash_word_32(hash, bytes[cursor] as u32);
    }

    hash
}

#[test]
fn test_hash_word_32() {
    assert_eq!(0, fxhash_32(b""));
    assert_eq!(0xF3051F19, fxhash_32(b"a"));
    assert_eq!(0x2F9DF119, fxhash_32(b"ab"));
    assert_eq!(0xCB1D9396, fxhash_32(b"abc"));
    assert_eq!(0x8628F119, fxhash_32(b"abcd"));
    assert_eq!(0xBEBDB56D, fxhash_32(b"abcde"));
    assert_eq!(0x1CE8476D, fxhash_32(b"abcdef"));
    assert_eq!(0xC0F176A4, fxhash_32(b"abcdefg"));
    assert_eq!(0x09AB476D, fxhash_32(b"abcdefgh"));
    assert_eq!(0xB72F5D88, fxhash_32(b"abcdefghi"));
}
