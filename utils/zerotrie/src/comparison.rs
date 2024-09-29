// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A comparator that keeps lowercase and uppercase ASCII letters adjacent.

use core::cmp::Ordering;

#[inline]
pub(crate) const fn shift(x: u8) -> u8 {
    (x << 3) | (x >> 5)
}

#[inline]
pub(crate) fn cmp(a: u8, b: u8) -> Ordering {
    shift(a).cmp(&shift(b))
}

#[inline]
pub(crate) fn cmpi(a: u8, b: u8) -> Ordering {
    shift(a.to_ascii_lowercase()).cmp(&shift(b.to_ascii_lowercase()))
}

#[inline]
pub(crate) fn cmp_slices(a: &[u8], b: &[u8]) -> Ordering {
    let a_iter = a.iter().copied().map(shift);
    let b_iter = b.iter().copied().map(shift);
    Iterator::cmp(a_iter, b_iter)
}

#[test]
fn test_basic_cmp() {
    let mut all_bytes = (0u8..=255u8).collect::<Vec<_>>();
    all_bytes.sort_by(|a, b| cmp(*a, *b));

    assert_eq!(cmp(b'A', b'a'), Ordering::Less);
    assert_eq!(cmp(b'B', b'b'), Ordering::Less);
    assert_eq!(cmp(b'a', b'B'), Ordering::Less);

    assert_eq!(cmpi(b'A', b'a'), Ordering::Equal);
    assert_eq!(cmpi(b'B', b'b'), Ordering::Equal);
    assert_eq!(cmpi(b'a', b'B'), Ordering::Less);

    let full_order = [
        0, 32, 64, 96, 128, 160, 192, 224, 1, 33, 65, 97, 129, 161, 193, 225, 2, 34, 66, 98, 130,
        162, 194, 226, 3, 35, 67, 99, 131, 163, 195, 227, 4, 36, 68, 100, 132, 164, 196, 228, 5,
        37, 69, 101, 133, 165, 197, 229, 6, 38, 70, 102, 134, 166, 198, 230, 7, 39, 71, 103, 135,
        167, 199, 231, 8, 40, 72, 104, 136, 168, 200, 232, 9, 41, 73, 105, 137, 169, 201, 233, 10,
        42, 74, 106, 138, 170, 202, 234, 11, 43, 75, 107, 139, 171, 203, 235, 12, 44, 76, 108, 140,
        172, 204, 236, 13, 45, 77, 109, 141, 173, 205, 237, 14, 46, 78, 110, 142, 174, 206, 238,
        15, 47, 79, 111, 143, 175, 207, 239, 16, 48, 80, 112, 144, 176, 208, 240, 17, 49, 81, 113,
        145, 177, 209, 241, 18, 50, 82, 114, 146, 178, 210, 242, 19, 51, 83, 115, 147, 179, 211,
        243, 20, 52, 84, 116, 148, 180, 212, 244, 21, 53, 85, 117, 149, 181, 213, 245, 22, 54, 86,
        118, 150, 182, 214, 246, 23, 55, 87, 119, 151, 183, 215, 247, 24, 56, 88, 120, 152, 184,
        216, 248, 25, 57, 89, 121, 153, 185, 217, 249, 26, 58, 90, 122, 154, 186, 218, 250, 27, 59,
        91, 123, 155, 187, 219, 251, 28, 60, 92, 124, 156, 188, 220, 252, 29, 61, 93, 125, 157,
        189, 221, 253, 30, 62, 94, 126, 158, 190, 222, 254, 31, 63, 95, 127, 159, 191, 223, 255,
    ];
    assert_eq!(all_bytes, full_order);
}
