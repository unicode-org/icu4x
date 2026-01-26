// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Methods for normalizing Latin1 input into UTF-16 output.
//!
//! NFC is not available, since Latin1 input is already known to be
//! in NFC.

use core::mem::MaybeUninit;

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

/// Returns the compatibility decompostion given a `TABLE` value.
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

/// Copies at least `items` code units from `src` to `dst` with
/// zero extension. Can copy up to 15 more items to make good use
/// of SIMD.
///
/// Returns `items` to include at least `items` values were copied
/// or a smaller number to indicate that fewer were copied.
#[inline(always)]
fn copy(src: &[u8], dst: &mut [MaybeUninit<u16>], items: usize) -> usize {
    let len = core::cmp::min(src.len(), dst.len());
    let low_four = items & 0b1111;
    let rounded_up = if low_four == 0 {
        items
    } else {
        ((items >> 4) + 1) << 4
    };
    let (ret, cap) = if rounded_up > len {
        if items > len {
            (len, len)
        } else {
            (items, items)
        }
    } else {
        (items, rounded_up)
    };
    let src_capped = &src[..cap];
    let dst_capped = &mut dst[..cap];
    let (src_chunks, src_tail) = src_capped.as_chunks::<16>();
    let (dst_chunks, dst_tail) = dst_capped.as_chunks_mut::<16>();
    for (src_chunk, dst_chunk) in src_chunks.iter().zip(dst_chunks.iter_mut()) {
        dst_chunk[0] = MaybeUninit::new(u16::from(src_chunk[0]));
        dst_chunk[1] = MaybeUninit::new(u16::from(src_chunk[1]));
        dst_chunk[2] = MaybeUninit::new(u16::from(src_chunk[2]));
        dst_chunk[3] = MaybeUninit::new(u16::from(src_chunk[3]));
        dst_chunk[4] = MaybeUninit::new(u16::from(src_chunk[4]));
        dst_chunk[5] = MaybeUninit::new(u16::from(src_chunk[5]));
        dst_chunk[6] = MaybeUninit::new(u16::from(src_chunk[6]));
        dst_chunk[7] = MaybeUninit::new(u16::from(src_chunk[7]));
        dst_chunk[8] = MaybeUninit::new(u16::from(src_chunk[8]));
        dst_chunk[9] = MaybeUninit::new(u16::from(src_chunk[9]));
        dst_chunk[10] = MaybeUninit::new(u16::from(src_chunk[10]));
        dst_chunk[11] = MaybeUninit::new(u16::from(src_chunk[11]));
        dst_chunk[12] = MaybeUninit::new(u16::from(src_chunk[12]));
        dst_chunk[13] = MaybeUninit::new(u16::from(src_chunk[13]));
        dst_chunk[14] = MaybeUninit::new(u16::from(src_chunk[14]));
        dst_chunk[15] = MaybeUninit::new(u16::from(src_chunk[15]));
    }
    for (src_slot, dst_slot) in src_tail.iter().zip(dst_tail.iter_mut()) {
        *dst_slot = MaybeUninit::new(u16::from(*src_slot));
    }
    ret
}

/// Normalize Latin1 `text` to NFD UTF-16 written to `sink`.
pub fn normalize_nfd(src: &[u8], dst: &mut [MaybeUninit<u16>], prefix: usize, value_from_table: u16) -> (usize, usize, u16) {
    debug_assert_ne!(value_from_table, 0);
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[0x20..];

    let dst_len = dst.len();
    let src_len = if src.len() < dst_len {
        // Cap the source so that we don't read
        // further than we can write anyway.
        dst_len
    } else {
        src.len()
    };

    let mut passthrough = if prefix > src_len {
        // Cap the prefix so that it can't be longer
        // than the source.
        prefix
    } else {
        src_len
    };

    let mut src_left = &src[..src_len];
    let mut src_left_len = src_len;
    let mut dst_left = &dst;
    // Set up `iter` as if it has already seen
    // `passthough` bytes from `src_left`
    let mut iter = src_left[passthrough..].iter();
    // Value read from table.
    let mut v = value_from_table;
    loop {
        let written = copy(src_left, dst_left, passthrough);
        let ret = (src_len - src_left_len + written, dst_len - dst_left.len() + written, v);
        if (written != passthrough) || v == 0 {
            return ret;
        }
        let dst_pending = &mut dst_left[passthrough..];
        // Start varying part
        if let Some((target, tail)) = dst_pending.split_first_chunk_mut::<2>() {
            target[0] = MaybeUninit::new(v >> 8);
            target[1] = MaybeUninit::new((v & 0xFF) + 0x0300);
            dst_left = tail;
        } else {
            return ret;
        };
        // End varying part
        loop {
            if let Some(val) = table.get(c.wrapping_sub(0xC0) as usize) {
                v = *val;
                if v == 0 {
                    continue;
                }
                src_left_len = src_left.len();
                let remaining = iter.as_slice();
                passthough = src_left_len - remaining.len() - 1;
                src_left = iter.as_slice();
                break; // Continue outer
            }
            v = 0; // Take the return;
            break; // Continue outer
        }
    }
}

pub fn is_normalized_nfd_up_to(text: &[u8]) -> (usize, u16) {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[0x20..];

    let mut iter = text.iter();
    while let Some(c) = iter.next() {
        if let Some(val) = table.get(c.wrapping_sub(0xC0) as usize) {
            v = *val;
            if v == 0 {
                continue;
            }
            return (text.len() - iter.as_slice().len() - 1, v);
        }
    }
    (text.len(), 0)
}

/*
/// Normalize Latin1 `text` to NFKD UTF-16 written to `sink`.
pub fn normalize_nfkd_to<W: Write16 + ?Sized>(mut text: &[u8], sink: &mut W) -> core::fmt::Result {
    'outer: loop {
        let mut iter = text.iter();
        loop {
            let Some(c) = iter.next() else {
                sink.write_latin1_slice(text)?;
                return Ok(());
            };
            if let Some(val) = TABLE.get(c.wrapping_sub(0xA0) as usize) {
                let v = *val;
                if v == 0 {
                    continue;
                }

                let remaining = iter.as_slice();
                // Indexing is OK, because the index is by construction in range.
                #[expect(clippy::indexing_slicing)]
                let prefix = &text[..text.len() - remaining.len() - 1];
                sink.write_latin1_slice(prefix)?;

                let hi = v >> 8;
                if hi != 0 {
                    sink.write_slice(&[hi, (v & 0xFF) + 0x0300])?;
                } else {
                    sink.write_slice(compatibility_decomposition(v))?;
                }

                text = remaining;
                continue 'outer;
            }
        }
    }
}

/// Normalize Latin1 `text` to NFKC UTF-16 written to `sink`.
pub fn normalize_nfkc_to<W: Write16 + ?Sized>(mut text: &[u8], sink: &mut W) -> core::fmt::Result {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[..0x20];

    'outer: loop {
        let mut iter = text.iter();
        loop {
            let Some(c) = iter.next() else {
                sink.write_latin1_slice(text)?;
                return Ok(());
            };
            if let Some(val) = table.get(c.wrapping_sub(0xA0) as usize) {
                let v = *val;
                if v == 0 {
                    continue;
                }

                let remaining = iter.as_slice();
                // Indexing is OK, because the index is by construction in range.
                #[expect(clippy::indexing_slicing)]
                let prefix = &text[..text.len() - remaining.len() - 1];
                sink.write_latin1_slice(prefix)?;

                sink.write_slice(compatibility_decomposition(v))?;

                text = remaining;
                continue 'outer;
            }
        }
    }
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
                    return text.len() - iter.as_slice().len() - 1;
                }
            }
        } else {
            return text.len();
        }
    }
}

/// Split Latin1 `text` into `(head, tail)` such that the first
/// byte of `tail` is the first byte of input that is not in NFKD.
/// If `text` is fully in NFKD, `tail` is empty.
pub fn is_normalized_nfkd_up_to(text: &[u8]) -> usize {
    let mut iter = text.iter();
    loop {
        if let Some(c) = iter.next() {
            if let Some(val) = TABLE.get(c.wrapping_sub(0xA0) as usize) {
                if *val != 0 {
                    return text.len() - iter.as_slice().len() - 1;
                }
            }
        } else {
            return text.len();
        }
    }
}

/// Split Latin1 `text` into `(head, tail)` such that the first
/// byte of `tail` is the first byte of input that is not in NFKC.
/// If `text` is fully in NFKC, `tail` is empty.
pub fn is_normalized_nfkc_up_to(text: &[u8]) -> usize {
    // Indexing is OK, because the index is statically in range.
    #[expect(clippy::indexing_slicing)]
    let table = &TABLE[..0x20];
    let mut iter = text.iter();
    loop {
        if let Some(c) = iter.next() {
            if let Some(val) = table.get(c.wrapping_sub(0xA0) as usize) {
                let v = *val;
                if v != 0 {
                    return text.len() - iter.as_slice().len() - 1;
                }
            }
        } else {
            return text.len();
        }
    }
}
*/