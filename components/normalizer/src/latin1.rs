// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Methods for normalizing Latin1 input into a UTF-16 sink.
//!
//! NFC is not available, since Latin1 input is already known to be
//! in NFC.

use write16::Write16;

/// Entries start from U+00A0 NO-BREAK SPACE. If the character is
/// always its own normalization, the value in the table is 0.
/// If the character has a compatibility decompositons, the value
/// in the table is the index into `COMPATIBILITY_DECOMPOSITIONS`
/// shifted left by two and the length of the subslice of
/// `COMPATIBILITY_DECOMPOSITIONS` in the low 2 bits. This means
/// that the high half is zero. Otherwise, the high 8 bits are the
/// first character of the canonical decomposition and the low 8
/// bits are the offset that needs to be added to U+0300 to get the
/// second character of the canonical decomposition.
static TABLE: [u16; 96] = [
    0x01,   // nbsp
    0,      // ¡
    0,      // ¢
    0,      // £
    0,      // ¤
    0,      // ¥
    0,      // ¦
    0,      // §
    0x02,   // ¨
    0,      // ©
    0x09,   // ª
    0,      // «
    0,      // ¬
    0,      // shy
    0,      // ®
    0x0E,   // ¯
    0,      // °
    0,      // ±
    0x41,   // ²
    0x45,   // ³
    0x16,   // ´
    0x1D,   // µ
    0,      // ¶
    0,      // ·
    0x22,   // ¸
    0x2D,   // ¹
    0x29,   // º
    0,      // »
    0x2F,   // ¼
    0x3B,   // ½
    0x47,   // ¾
    0,      // ¿
    0x4100, // À
    0x4101, // Á
    0x4102, // Â
    0x4103, // Ã
    0x4108, // Ä
    0x410A, // Å
    0,      // Æ
    0x4327, // Ç
    0x4500, // È
    0x4501, // É
    0x4502, // Ê
    0x4508, // Ë
    0x4900, // Ì
    0x4901, // Í
    0x4902, // Î
    0x4908, // Ï
    0,      // Ð
    0x4E03, // Ñ
    0x4F00, // Ò
    0x4F01, // Ó
    0x4F02, // Ô
    0x4F03, // Õ
    0x4F08, // Ö
    0,      // ×
    0,      // Ø
    0x5500, // Ù
    0x5501, // Ú
    0x5502, // Û
    0x5508, // Ü
    0x5901, // Ý
    0,      // Þ
    0,      // ß
    0x6100, // à
    0x6101, // á
    0x6102, // â
    0x6103, // ã
    0x6108, // ä
    0x610A, // å
    0,      // æ
    0x6327, // ç
    0x6500, // è
    0x6501, // é
    0x6502, // ê
    0x6508, // ë
    0x6900, // ì
    0x6901, // í
    0x6902, // î
    0x6908, // ï
    0,      // ð
    0x6E03, // ñ
    0x6F00, // ò
    0x6F01, // ó
    0x6F02, // ô
    0x6F03, // õ
    0x6F08, // ö
    0,      // ÷
    0,      // ø
    0x7500, // ù
    0x7501, // ú
    0x7502, // û
    0x7508, // ü
    0x7901, // ý
    0,      // þ
    0x7908, // ÿ
];

/// Table containing the compatibility decompositions.
static COMPATIBILITY_DECOMPOSITIONS: [u16; 20] = [
    0x0020, 0x0308, 0x0061, 0x0020, 0x0304, 0x0020, 0x0301, 0x03BC, 0x0020, 0x0327, 0x006F, 0x0031,
    0x2044, 0x0034, 0x0031, 0x2044, 0x0032, 0x0033, 0x2044, 0x0034,
];

/// Writes the compatibility decompostion of `c` to `sink`.
fn compatibility_decomposition(val: u16) -> &'static [u16] {
    debug_assert!(val <= 0xFF);
    let len = val & 0b11;
    let index = val >> 2;
    COMPATIBILITY_DECOMPOSITIONS
        .get(index as usize..index as usize + len as usize)
        .unwrap_or_else(|| {
            // Internal bug, not even GIGO, never supposed to happen
            debug_assert!(false);
            &[]
        })
}

/// Normalize Latin1 `text` to NFD UTF-16 written to `sink`.
pub fn normalize_nfd_to<W: Write16 + ?Sized>(text: &[u8], sink: &mut W) -> core::fmt::Result {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[0x20..];
    for c in text {
        if let Some(val) = table.get(c.wrapping_sub(0xC0) as usize) {
            let v = *val;
            if v != 0 {
                sink.write_slice(&[v >> 8, (v & 0xFF) + 0x0300])?;
                continue;
            }
        }
        sink.write_slice(&[*c as u16])?;
    }
    Ok(())
}

/// Normalize Latin1 `text` to NFKD UTF-16 written to `sink`.
pub fn normalize_nfkd_to<W: Write16 + ?Sized>(text: &[u8], sink: &mut W) -> core::fmt::Result {
    for c in text {
        if let Some(val) = TABLE.get(c.wrapping_sub(0xA0) as usize) {
            let v = *val;
            if v == 0 {
                // Fall through
            } else {
                let hi = v >> 8;
                if hi != 0 {
                    sink.write_slice(&[hi, (v & 0xFF) + 0x0300])?;
                    continue;
                } else {
                    sink.write_slice(compatibility_decomposition(v))?;
                    continue;
                }
            }
        }
        sink.write_slice(&[*c as u16])?;
    }
    Ok(())
}

/// Normalize Latin1 `text` to NFKC UTF-16 written to `sink`.
pub fn normalize_nfkc_to<W: Write16 + ?Sized>(text: &[u8], sink: &mut W) -> core::fmt::Result {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[..0x20];
    for c in text {
        if let Some(val) = table.get(c.wrapping_sub(0xA0) as usize) {
            let v = *val;
            if v != 0 {
                sink.write_slice(compatibility_decomposition(v))?;
                continue;
            }
        }
        sink.write_slice(&[*c as u16])?;
    }
    Ok(())
}

/// Split Latin1 `text` into `(head, tail)` such that the first
/// byte of `tail` is the first byte of input that is not in NFD.
/// If `text` is fully in NFD, `tail` is empty.
pub fn split_normalized_nfd(text: &[u8]) -> (&[u8], &[u8]) {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[0x20..];
    let mut iter = text.iter();
    loop {
        if let Some(c) = iter.next() {
            if let Some(val) = table.get(c.wrapping_sub(0xC0) as usize) {
                if *val != 0 {
                    let tail = iter.as_slice();
                    return text
                        .split_at_checked(text.len() - tail.len() - 1)
                        .unwrap_or_else(|| {
                            // Internal bug, not even GIGO, never supposed to happen
                            debug_assert!(false);
                            (&[], text)
                        });
                }
            }
        } else {
            return (text, &[]);
        }
    }
}

/// Split Latin1 `text` into `(head, tail)` such that the first
/// byte of `tail` is the first byte of input that is not in NFKD.
/// If `text` is fully in NFKD, `tail` is empty.
pub fn split_normalized_nfkd(text: &[u8]) -> (&[u8], &[u8]) {
    let mut iter = text.iter();
    loop {
        if let Some(c) = iter.next() {
            if let Some(val) = TABLE.get(c.wrapping_sub(0xA0) as usize) {
                if *val != 0 {
                    let tail = iter.as_slice();
                    return text
                        .split_at_checked(text.len() - tail.len() - 1)
                        .unwrap_or_else(|| {
                            // Internal bug, not even GIGO, never supposed to happen
                            debug_assert!(false);
                            (&[], text)
                        });
                }
            }
        } else {
            return (text, &[]);
        }
    }
}

/// Split Latin1 `text` into `(head, tail)` such that the first
/// byte of `tail` is the first byte of input that is not in NFKC.
/// If `text` is fully in NFKC, `tail` is empty.
pub fn split_normalized_nfkc(text: &[u8]) -> (&[u8], &[u8]) {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[..0x20];
    let mut iter = text.iter();
    loop {
        if let Some(c) = iter.next() {
            if let Some(val) = table.get(c.wrapping_sub(0xA0) as usize) {
                let v = *val;
                if v != 0 {
                    let tail = iter.as_slice();
                    return text
                        .split_at_checked(text.len() - tail.len() - 1)
                        .unwrap_or_else(|| {
                            // Internal bug, not even GIGO, never supposed to happen
                            debug_assert!(false);
                            (&[], text)
                        });
                }
            }
        } else {
            return (text, &[]);
        }
    }
}
