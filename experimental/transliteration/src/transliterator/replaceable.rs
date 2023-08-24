// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::Range;
use core::str;
use alloc::string::String;
use alloc::vec::Vec;

use super::Filter;

/// Wrapper for in-place transliteration. Stores the currently allowed transliteration range.
pub(crate) struct Replaceable<'a> {
    // guaranteed to be valid UTF-8
    // only content[freeze_pre_len..content.len()-freeze_post_len] is mutable
    content: &'a mut Vec<u8>,
    // guaranteed to be a valid UTF-8 index into content
    freeze_pre_len: usize,
    // `content.len() - freeze_post_len` is guaranteed to be a valid UTF-8 index into content
    freeze_post_len: usize,
    // note: for Function Calls (that only "see" the input argument), we also need a "ignore_pre_len" and "ignore_post_len", probably
}

// note: would be great to have something like Replaceable::replace_range(&mut self, range) -> &mut Insertable 
// where Insertable supports things like pushing chars, strs, etc, and they would directly go to the corresponding range
// of the backing Vec<u8>. pushing more things than range.len would reallocate the backing vec to make space on the fly,
// pushing fewer things than range.len would move around the tail of the Vec on Insertable::drop to fill the "empty space".
// a "rope" or "cord" to replace the Vec<u8> might also be nice

impl<'a> Replaceable<'a> {
    /// # Safety
    /// The caller must ensure `buf` is valid UTF-8.
    pub(crate) unsafe fn new(buf: &'a mut Vec<u8>) -> Self {
        Self {
            content: buf,
            // these lengths uphold the invariants
            freeze_pre_len: 0,
            freeze_post_len: 0,
        }
    }

    // TODO: design
    pub(crate) unsafe fn splice(&mut self, range: Range<usize>, replacement: &[u8]) {
        self.content.splice(range, replacement.iter().copied());
    }

    pub(crate) fn as_str(&self) -> &str {
        debug_assert!(str::from_utf8(&self.content[..]).is_ok());
        // SAFETY: Replaceable's invariant states that content is always valid UTF-8
        unsafe { str::from_utf8_unchecked(&self.content[..]) }
    }

    // TODO: this is unsafe, need to guarantee pos/len are valid for UTF-8
    pub(crate) fn freeze_at(&mut self, pos: usize, len: usize) {
        debug_assert!(pos < self.content.len() && len <= self.content.len() - pos);

        self.freeze_pre_len = pos;
        self.freeze_post_len = self.content.len() - pos - len;
    }

    /// Returns the range of bytes that are currently allowed to be modified.
    /// 
    /// This is guaranteed to be a range compatible with the internal UTF-8.
    pub(crate) fn allowed_range(&self) -> Range<usize> {
        self.freeze_pre_len..(self.content.len() - self.freeze_post_len)
    }

    pub(crate) fn with_range(&mut self, range: Range<usize>) -> Replaceable {
        Replaceable { content: self.content, freeze_pre_len: range.start, freeze_post_len: range.end }
    }

    // pub(crate) fn get(&self, pos: usize) -> Option<u8> {
    //     self.content.get(pos).copied()
    // }

    // /// Returns the next run (run_start_index, run_length) that occurs after `start`, if one exists.
    // pub(crate) fn next_filtered_run(
    //     &self,
    //     start: usize,
    //     filter: &Filter,
    // ) -> Option<(usize, usize)> {
    //     debug_assert!(start < self.content.len());

    //     let run_start = self.find_first_in(start, filter)?;
    //     let run_end = self.find_first_out(run_start, filter)?;
    //     let run_length = run_end - run_start;

    //     Some((run_start, run_length))
    // }

    // fn find_first_in(&self, start: usize, filter: &Filter) -> Option<usize> {
    //     let tail = &self.as_str()[start..];
    //     let (idx, _) = tail.char_indices().find(|&(_, c)| filter.contains(c))?;
    //     Some(start + idx)
    // }

    // fn find_first_out(&self, start: usize, filter: &Filter) -> Option<usize> {
    //     let tail = &self.as_str()[start..];
    //     let (idx, _) = tail.char_indices().find(|&(_, c)| !filter.contains(c))?;
    //     Some(start + idx)
    // }
}
