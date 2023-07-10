// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # Internal layout of ZeroTrie
//!
//! A ZeroTrie is composed of a series of nodes stored in sequence in a byte slice.
//!
//! There are 4 types of nodes:
//!
//! 1. ASCII (`0xxxxxxx`): matches a literal ASCII byte.
//! 2. Span (`101xxxxx`): matches a span of non-ASCII bytes.
//! 3. Value (`100xxxxx`): associates a value with a string
//! 4. Branch (`11xxxxxx`): matches one of a set of bytes.
//!
//! Span, Value, and Branch nodes contain a varint, which has different semantics for each:
//!
//! - Span varint: length of the span
//! - Value varint: value associated with the string
//! - Branch varint: number of edges in the branch and width of the offset table
//!
//! The exact structure of the Branch node is what varies between ZeroTrie types.
//!
//! Here is an example ZeroTrie without branch nodes:
//!
//! ```
//! use zerotrie::ZeroTrieSimpleAscii;
//!
//! let bytes = [
//!     b'a', // ASCII literal
//!     0b10001010, // value 10
//!     b'b', // ASCII literal
//!     0b10100010, // span of 3
//!     0x81, // first byte in span
//!     0x91, // second byte in span
//!     0xA1, // third and final byte in span
//!     0b1000100, // value 4
//! ];
//!
//! let trie = ZeroTrieSimpleAscii::from_bytes(&bytes);
//!
//! assert_eq!(trie.get(b"a"), Some(10));
//! assert_eq!(trie.get(b"ab"), None);
//! assert_eq!(trie.get(b"b"), None);
//! assert_eq!(trie.get(b"ab\x81\x91\xA1"), Some(4));
//! ```

use crate::byte_phf::PerfectByteHashMap;
use crate::varint::read_varint_meta2;
use crate::varint::read_varint_meta3;
use core::ops::Range;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Like slice::split_at but returns an Option instead of panicking.
///
/// Debug-panics if `mid` is out of range.
#[inline]
fn debug_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        debug_assert!(false, "debug_split_at: index expected to be in range");
        None
    } else {
        // Note: We're trusting the compiler to inline this and remove the assertion
        // hiding on the top of slice::split_at: `assert(mid <= self.len())`
        Some(slice.split_at(mid))
    }
}

/// Like slice::split_at but returns an Option instead of panicking.
#[inline]
fn maybe_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        None
    } else {
        // Note: We're trusting the compiler to inline this and remove the assertion
        // hiding on the top of slice::split_at: `assert(mid <= self.len())`
        Some(slice.split_at(mid))
    }
}

#[inline]
fn debug_get(slice: &[u8], index: usize) -> Option<u8> {
    match slice.get(index) {
        Some(x) => Some(*x),
        None => {
            debug_assert!(false, "debug_get: index expected to be in range");
            None
        }
    }
}

#[inline]
fn debug_get_range(slice: &[u8], range: Range<usize>) -> Option<&[u8]> {
    match slice.get(range) {
        Some(x) => Some(x),
        None => {
            debug_assert!(false, "debug_get_range: indices expected to be in range");
            None
        }
    }
}

/// Given a slice starting with an offset table, returns the trie for the given index.
///
/// Arguments:
/// - `trie` = a trie pointing at an offset table (after the branch node and search table)
/// - `i` = the desired index within the offset table
/// - `n` = the number of items in the offset table
/// - `w` = the width of the offset table items minus one
#[inline]
fn get_branch(mut trie: &[u8], i: usize, n: usize, mut w: usize) -> Option<&[u8]> {
    let mut p = 0usize;
    let mut q = 0usize;
    loop {
        let indices;
        (indices, trie) = debug_split_at(trie, n - 1)?;
        p = (p << 8)
            + if i == 0 {
                0
            } else {
                debug_get(indices, i - 1)? as usize
            };
        q = match indices.get(i) {
            Some(x) => (q << 8) + *x as usize,
            None => trie.len(),
        };
        if w == 0 {
            break;
        }
        w -= 1;
    }
    debug_get_range(trie, p..q)
}

/// Version of [`get_branch()`] specialized for the case `w == 0` for performance
#[inline]
fn get_branch_w0(mut trie: &[u8], i: usize, n: usize) -> Option<&[u8]> {
    let indices;
    (indices, trie) = debug_split_at(trie, n - 1)?;
    let p = if i == 0 {
        0
    } else {
        debug_get(indices, i - 1)? as usize
    };
    let q = match indices.get(i) {
        Some(x) => *x as usize,
        None => trie.len(),
    };
    debug_get_range(trie, p..q)
}

enum ByteType {
    Ascii,
    Span,
    Value,
    Match,
}

impl core::fmt::Debug for ByteType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use ByteType::*;
        f.write_str(match *self {
            Ascii => "a",
            Span => "s",
            Value => "v",
            Match => "m",
        })
    }
}

#[inline]
fn byte_type(b: u8) -> ByteType {
    match b & 0b11100000 {
        0b10000000 => ByteType::Value,
        0b10100000 => ByteType::Span,
        0b11000000 => ByteType::Match,
        0b11100000 => ByteType::Match,
        _ => ByteType::Ascii,
    }
}

// DISCUSS: This function is 7% faster *on aarch64* if we assert a max on w.
//
// | Bench         | No Assert, x86_64 | No Assert, aarch64 | Assertion, x86_64 | Assertion, aarch64 |
// |---------------|-------------------|--------------------|-------------------|--------------------|
// | basic         | ~187.51 ns        | ~97.586 ns         | ~199.11 ns        | ~99.236 ns         |
// | subtags_10pct | ~9.5557 µs        | ~4.8696 µs         | ~9.5779 µs        | ~4.5649 µs         |
// | subtags_full  | ~137.75 µs        | ~76.016 µs         | ~142.02 µs        | ~70.254 µs         |

/// Query the trie assuming all branch nodes are binary search.
pub fn get_bsearch_only(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    loop {
        let (b, x, i, search);
        (b, trie) = trie.split_first()?;
        let byte_type = byte_type(*b);
        (x, trie) = match byte_type {
            ByteType::Ascii => (0, trie),
            ByteType::Span | ByteType::Value => read_varint_meta3(*b, trie)?,
            ByteType::Match => read_varint_meta2(*b, trie)?,
        };
        if let Some((c, temp)) = ascii.split_first() {
            if matches!(byte_type, ByteType::Ascii) {
                if b == c {
                    // Matched a byte
                    ascii = temp;
                    continue;
                } else {
                    // Byte that doesn't match
                    return None;
                }
            }
            if matches!(byte_type, ByteType::Value) {
                // Value node, but not at end of string
                continue;
            }
            if matches!(byte_type, ByteType::Span) {
                let (trie_span, ascii_span);
                (trie_span, trie) = debug_split_at(trie, x)?;
                (ascii_span, ascii) = maybe_split_at(ascii, x)?;
                if trie_span == ascii_span {
                    // Matched a byte span
                    continue;
                } else {
                    // Byte span that doesn't match
                    return None;
                }
            }
            // Branch node
            let (x, w) = if x >= 256 { (x & 0xff, x >> 8) } else { (x, 0) };
            // See comment above regarding this assertion
            debug_assert!(w <= 3, "get: w > 3 but we assume w <= 3");
            let w = w & 0x3;
            let x = if x == 0 { 256 } else { x };
            // Always use binary search
            (search, trie) = debug_split_at(trie, x)?;
            i = search.binary_search(c).ok()?;
            trie = if w == 0 {
                get_branch_w0(trie, i, x)
            } else {
                get_branch(trie, i, x, w)
            }?;
            ascii = temp;
            continue;
        } else {
            if matches!(byte_type, ByteType::Value) {
                // Value node at end of string
                return Some(x);
            }
            return None;
        }
    }
}

/// Query the trie assuming branch nodes could be either binary search or PHF.
pub fn get_phf_limited(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    loop {
        let (b, x, i, search);
        (b, trie) = trie.split_first()?;
        let byte_type = byte_type(*b);
        (x, trie) = match byte_type {
            ByteType::Ascii => (0, trie),
            ByteType::Span | ByteType::Value => read_varint_meta3(*b, trie)?,
            ByteType::Match => read_varint_meta2(*b, trie)?,
        };
        if let Some((c, temp)) = ascii.split_first() {
            if matches!(byte_type, ByteType::Ascii) {
                if b == c {
                    // Matched a byte
                    ascii = temp;
                    continue;
                } else {
                    // Byte that doesn't match
                    return None;
                }
            }
            if matches!(byte_type, ByteType::Value) {
                // Value node, but not at end of string
                continue;
            }
            if matches!(byte_type, ByteType::Span) {
                let (trie_span, ascii_span);
                (trie_span, trie) = debug_split_at(trie, x)?;
                (ascii_span, ascii) = maybe_split_at(ascii, x)?;
                if trie_span == ascii_span {
                    // Matched a byte span
                    continue;
                } else {
                    // Byte span that doesn't match
                    return None;
                }
            }
            // Branch node
            let (x, w) = if x >= 256 { (x & 0xff, x >> 8) } else { (x, 0) };
            // See comment above regarding this assertion
            debug_assert!(w <= 3, "get: w > 3 but we assume w <= 3");
            let w = w & 0x3;
            let x = if x == 0 { 256 } else { x };
            if x < 16 {
                // binary search
                (search, trie) = debug_split_at(trie, x)?;
                i = search.binary_search(c).ok()?;
            } else {
                // phf
                (search, trie) = debug_split_at(trie, x * 2 + 1)?;
                i = PerfectByteHashMap::from_store(search).get(*c)?;
            }
            trie = if w == 0 {
                get_branch_w0(trie, i, x)
            } else {
                get_branch(trie, i, x, w)
            }?;
            ascii = temp;
            continue;
        } else {
            if matches!(byte_type, ByteType::Value) {
                // Value node at end of string
                return Some(x);
            }
            return None;
        }
    }
}

/// Query the trie without the limited capacity assertion.
pub fn get_phf_extended(mut trie: &[u8], mut ascii: &[u8]) -> Option<usize> {
    loop {
        let (b, x, i, search);
        (b, trie) = trie.split_first()?;
        let byte_type = byte_type(*b);
        (x, trie) = match byte_type {
            ByteType::Ascii => (0, trie),
            ByteType::Span | ByteType::Value => read_varint_meta3(*b, trie)?,
            ByteType::Match => read_varint_meta2(*b, trie)?,
        };
        if let Some((c, temp)) = ascii.split_first() {
            if matches!(byte_type, ByteType::Ascii) {
                if b == c {
                    // Matched a byte
                    ascii = temp;
                    continue;
                } else {
                    // Byte that doesn't match
                    return None;
                }
            }
            if matches!(byte_type, ByteType::Value) {
                // Value node, but not at end of string
                continue;
            }
            if matches!(byte_type, ByteType::Span) {
                let (trie_span, ascii_span);
                (trie_span, trie) = debug_split_at(trie, x)?;
                (ascii_span, ascii) = maybe_split_at(ascii, x)?;
                if trie_span == ascii_span {
                    // Matched a byte span
                    continue;
                } else {
                    // Byte span that doesn't match
                    return None;
                }
            }
            // Branch node
            let (x, w) = if x >= 256 { (x & 0xff, x >> 8) } else { (x, 0) };
            let x = if x == 0 { 256 } else { x };
            if x < 16 {
                // binary search
                (search, trie) = debug_split_at(trie, x)?;
                i = search.binary_search(c).ok()?;
            } else {
                // phf
                (search, trie) = debug_split_at(trie, x * 2 + 1)?;
                i = PerfectByteHashMap::from_store(search).get(*c)?;
            }
            trie = if w == 0 {
                get_branch_w0(trie, i, x)
            } else {
                get_branch(trie, i, x, w)
            }?;
            ascii = temp;
            continue;
        } else {
            if matches!(byte_type, ByteType::Value) {
                // Value node at end of string
                return Some(x);
            }
            return None;
        }
    }
}

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub(crate) struct ZeroTrieIterator<'a> {
    use_phf: bool,
    state: Vec<(&'a [u8], Vec<u8>, usize)>,
}

#[cfg(feature = "alloc")]
impl<'a> ZeroTrieIterator<'a> {
    pub fn new<S: AsRef<[u8]> + ?Sized>(store: &'a S, use_phf: bool) -> Self {
        ZeroTrieIterator {
            use_phf,
            state: alloc::vec![(store.as_ref(), alloc::vec![], 0)],
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a> Iterator for ZeroTrieIterator<'a> {
    type Item = (Vec<u8>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (mut trie, mut string, mut branch_idx);
        (trie, string, branch_idx) = self.state.pop()?;
        loop {
            let (b, x, span, search);
            let return_trie = trie;
            (b, trie) = match trie.split_first() {
                Some(tpl) => tpl,
                None => {
                    // At end of current branch; step back to the branch node.
                    // If there are no more branches, we are finished.
                    (trie, string, branch_idx) = self.state.pop()?;
                    continue;
                }
            };
            let byte_type = byte_type(*b);
            if matches!(byte_type, ByteType::Ascii) {
                string.push(*b);
                continue;
            }
            (x, trie) = match byte_type {
                ByteType::Ascii => (0, trie),
                ByteType::Span | ByteType::Value => read_varint_meta3(*b, trie)?,
                ByteType::Match => read_varint_meta2(*b, trie)?,
            };
            if matches!(byte_type, ByteType::Span) {
                (span, trie) = debug_split_at(trie, x)?;
                string.extend(span);
                continue;
            }
            if matches!(byte_type, ByteType::Value) {
                let retval = string.clone();
                // Return to this position on the next step
                self.state.push((trie, string, 0));
                return Some((retval, x));
            }
            // Match node
            let (x, w) = if x >= 256 { (x & 0xff, x >> 8) } else { (x, 0) };
            let x = if x == 0 { 256 } else { x };
            if branch_idx + 1 < x {
                // Return to this branch node at the next index
                self.state
                    .push((return_trie, string.clone(), branch_idx + 1));
            }
            let byte = if x < 16 || !self.use_phf {
                // binary search
                (search, trie) = debug_split_at(trie, x)?;
                debug_get(search, branch_idx)?
            } else {
                // phf
                (search, trie) = debug_split_at(trie, x * 2 + 1)?;
                debug_get(search, branch_idx + x + 1)?
            };
            string.push(byte);
            trie = if w == 0 {
                get_branch_w0(trie, branch_idx, x)
            } else {
                get_branch(trie, branch_idx, x, w)
            }?;
            branch_idx = 0;
        }
    }
}

#[cfg(feature = "alloc")]
pub(crate) fn get_iter_phf<S: AsRef<[u8]> + ?Sized>(
    store: &S,
) -> impl Iterator<Item = (Vec<u8>, usize)> + '_ {
    ZeroTrieIterator::new(store, true)
}

/// # Panics
/// Panics if the trie contains non-ASCII items.
#[cfg(feature = "alloc")]
pub(crate) fn get_iter_ascii_or_panic<S: AsRef<[u8]> + ?Sized>(
    store: &S,
) -> impl Iterator<Item = (String, usize)> + '_ {
    ZeroTrieIterator::new(store, false).map(|(k, v)| {
        #[allow(clippy::unwrap_used)] // in signature of function
        let ascii_str = String::from_utf8(k).unwrap();
        (ascii_str, v)
    })
}
