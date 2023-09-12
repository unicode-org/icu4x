// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Const function to compute the FxHash of a byte array.
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
// The indexing operations in this function have been reviewed in detail and won't panic.
#[allow(clippy::indexing_slicing)]
pub(crate) const fn fxhash_32_trim(
    bytes: &[u8],
    ignore_leading: usize,
    ignore_trailing: usize,
) -> u32 {
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

    if ignore_leading + ignore_trailing >= bytes.len() {
        return 0;
    }

    #[inline]
    const fn hash_word_32(mut hash: u32, word: u32) -> u32 {
        const ROTATE: u32 = 5;
        const SEED32: u32 = 0x9e_37_79_b9;
        hash = hash.rotate_left(ROTATE);
        hash ^= word;
        hash = hash.wrapping_mul(SEED32);
        hash
    }

    let mut cursor = ignore_leading;
    let end = bytes.len() - ignore_trailing;
    let mut hash = 0;

    while end - cursor >= 4 {
        let word = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]);
        hash = hash_word_32(hash, word);
        cursor += 4;
    }

    if end - cursor >= 2 {
        let word = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]);
        hash = hash_word_32(hash, word as u32);
        cursor += 2;
    }

    if end - cursor >= 1 {
        hash = hash_word_32(hash, bytes[cursor] as u32);
    }

    hash
}

#[test]
fn test_hash_word_32() {
    assert_eq!(0, fxhash_32_trim(b"", 0, 0));
    assert_eq!(0, fxhash_32_trim(b"a", 1, 0));
    assert_eq!(0, fxhash_32_trim(b"a", 0, 1));
    assert_eq!(0, fxhash_32_trim(b"a", 0, 10));
    assert_eq!(0, fxhash_32_trim(b"a", 10, 0));
    assert_eq!(0, fxhash_32_trim(b"a", 1, 1));
    assert_eq!(0xF3051F19, fxhash_32_trim(b"a", 0, 0));
    assert_eq!(0x2F9DF119, fxhash_32_trim(b"ab", 0, 0));
    assert_eq!(0xCB1D9396, fxhash_32_trim(b"abc", 0, 0));
    assert_eq!(0x8628F119, fxhash_32_trim(b"abcd", 0, 0));
    assert_eq!(0xBEBDB56D, fxhash_32_trim(b"abcde", 0, 0));
    assert_eq!(0x1CE8476D, fxhash_32_trim(b"abcdef", 0, 0));
    assert_eq!(0xC0F176A4, fxhash_32_trim(b"abcdefg", 0, 0));
    assert_eq!(0x09AB476D, fxhash_32_trim(b"abcdefgh", 0, 0));
    assert_eq!(0xB72F5D88, fxhash_32_trim(b"abcdefghi", 0, 0));
}
