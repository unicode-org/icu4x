// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use core::ops::Range;
use core::str;
use crate::transliterator::{CursorOffset, MatchData};

use super::Filter;

// TODO: do we really want this much unsafe? a lot of the API could be replaced with safe, but
//  checked String methods.

/// Wrapper for in-place transliteration. Stores the currently allowed-to-be-modified
/// transliteration range.
pub(crate) struct Replaceable<'a> {
    // guaranteed to be valid UTF-8
    // only content[ignore_pre_len+freeze_pre_len..content.len()-freeze_post_len] is mutable
    content: &'a mut Vec<u8>,
    // `ignore_pre_len + freeze_pre_len` is guaranteed to be a valid UTF-8 index into content
    freeze_pre_len: usize,
    // `content.len() - freeze_post_len` is guaranteed to be a valid UTF-8 index into content
    freeze_post_len: usize,
    // guaranteed to be a valid UTF-8 index into the visible content.
    cursor: usize,
    // guaranteed to be a valid UTF-8 index into content
    // needed because function calls _only_ see their arguments, but nothing surrounding them.
    ignore_pre_len: usize,
    // guaranteed to be a valid UTF-8 suffix length
    ignore_post_len: usize,
}

// note: would be great to have something like Replaceable::replace_range(&mut self, range) -> &mut Insertable
// where Insertable supports things like pushing chars, strs, etc, and they would directly go to the corresponding range
// of the backing Vec<u8>. pushing more things than range.len would reallocate the backing vec to make space on the fly,
// pushing fewer things than range.len would move around the tail of the Vec on Insertable::drop to fill the "empty space".
// a "rope" or "cord" to replace the Vec<u8> might also be nice

impl<'a> Replaceable<'a> {
    /// # Safety
    /// The caller must ensure `buf` is valid UTF-8 and that `visible_range` is a valid UTF-8 range.
    pub(crate) unsafe fn new(buf: &'a mut Vec<u8>, visible_range: Range<usize>) -> Self {
        debug_assert!(visible_range.end <= buf.len());
        debug_assert!(visible_range.start <= buf.len());
        let ignore_post_len = buf.len() - visible_range.end;
        Self {
            content: buf,
            // these uphold the invariants
            freeze_pre_len: 0,
            freeze_post_len: 0,
            ignore_pre_len: visible_range.start,
            ignore_post_len,
            cursor: 0,
        }
    }

    /// # Safety
    /// The caller must ensure that `range` is a valid UTF-8 range into the visible content.
    pub(crate) unsafe fn replace_range<'b>(&'b mut self, range: Range<usize>, size_hint: Option<usize>) -> Insertable<'a, 'b> {
        // TODO: if start is always cursor, maybe change signature to take only end?
        debug_assert_eq!(range.start, self.cursor);
        // the passed range is into self.visible_content(), which starts at offset self.ignore_pre_len
        let adjusted_range = range.start + self.ignore_pre_len..range.end + self.ignore_pre_len;
        Insertable::new_with_size_hint(self, adjusted_range, size_hint.unwrap_or(0))
    }

    // TODO: design
    // TODO: maybe add checks about frozen range?
    pub(crate) unsafe fn splice(
        &mut self,
        range: Range<usize>,
        replacement: &[u8],
        new_cursor: usize,
    ) {
        // TODO: splice should probably be combined with a cursor update, otherwise
        //  the invariants may not hold across function calls

        let ignore_adjusted_range =
            range.start + self.ignore_pre_len..range.end + self.ignore_pre_len;
        self.content
            .splice(ignore_adjusted_range, replacement.iter().copied());

        // SAFETY: the caller ensures that this is a valid index after the replacement is applied.
        self.set_cursor(new_cursor);
    }

    /// Returns the full current content as a `&str`.
    pub(crate) fn as_str(&self) -> &str {
        debug_assert!(str::from_utf8(self.visible_content()).is_ok());
        // SAFETY: Replaceable's invariant states that content is always valid UTF-8
        unsafe { str::from_utf8_unchecked(self.visible_content()) }
    }

    /// Returns the current content before the cursor as a `&str`.
    pub(crate) fn as_str_ante(&self) -> &str {
        // TODO: use the ignore thing as lower bound here
        &self.as_str()[..self.cursor()]
    }

    /// Returns the current modifiable content after the cursor as a `&str`.
    pub(crate) fn as_str_key(&self) -> &str {
        &self.as_str()[self.cursor()..self.allowed_upper_bound()]
    }

    /// Returns the current content after the cursor as a `&str`. `key_match_len` is the length of
    /// the key match and must be a valid UTF-8 index into the visible slice..
    pub(crate) fn as_str_post(&self, key_match_len: usize) -> &str {
        // TODO: use ignore thing as upper bound
        &self.as_str()[(self.cursor() + key_match_len)..]
    }
    //
    // // TODO: this is unsafe, need to guarantee pos/len are valid for UTF-8
    // pub(crate) fn freeze_at(&mut self, pos: usize, len: usize) {
    //     debug_assert!(pos < self.content.len() && len <= self.content.len() - pos);
    //
    //     self.freeze_pre_len = pos;
    //     self.freeze_post_len = self.content.len() - pos - len;
    // }

    /// Returns the range of bytes that are currently allowed to be modified.
    ///
    /// This is guaranteed to be a range compatible with the internal UTF-8.
    pub(crate) fn allowed_range(&self) -> Range<usize> {
        self.freeze_pre_len..self.allowed_upper_bound()
    }

    /// Returns the cursor.
    ///
    /// This is guaranteed to be a valid UTF-8 index into the visible slice.
    pub(crate) fn cursor(&self) -> usize {
        // eprintln!("cursor called with raw_cursor: {}, ignore_pre_len: {}", self.raw_cursor, self.ignore_pre_len);
        self.cursor
    }

    /// Advances the cursor by one char.
    pub(crate) fn step_cursor(&mut self) {
        let step_len = self.as_str()[self.cursor()..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(0);
        // eprintln!("step_cursor: {}", step_len);
        self.cursor += step_len;
    }

    /// Sets the cursor. The cursor must remain in the modifiable window.
    ///
    /// # Safety
    /// The caller must ensure that `cursor` is a valid UTF-8 index into the visible slice.
    pub(crate) unsafe fn set_cursor(&mut self, cursor: usize) {
        debug_assert!(cursor <= self.allowed_upper_bound());
        debug_assert!(cursor >= self.freeze_pre_len);
        self.cursor = cursor;
    }

    /// Returns true if the cursor is at the end of the modifiable range.
    pub(crate) fn is_finished(&self) -> bool {
        // the cursor should never be > the upper bound
        debug_assert!(self.cursor() <= self.allowed_upper_bound());
        self.cursor() >= self.allowed_upper_bound()
    }

    // pub(crate) fn with_range(&mut self, range: Range<usize>) -> Replaceable {
    //     Replaceable { content: self.content, freeze_pre_len: range.start, freeze_post_len: range.end }
    // }

    // pub(crate) fn get(&self, pos: usize) -> Option<u8> {
    //     self.content.get(pos).copied()
    // }

    /// Returns a `Replaceable` with the same content as the current one.
    ///
    /// This is useful for repeated transliterations of the same modifiable range.
    pub(crate) fn child(&mut self) -> Replaceable {
        Replaceable {
            content: self.content,
            freeze_pre_len: self.freeze_pre_len,
            freeze_post_len: self.freeze_post_len,
            cursor: self.cursor,
            ignore_pre_len: self.ignore_pre_len,
            ignore_post_len: self.ignore_post_len,
        }
    }

    // TODO: could replace the F generic with a InternalTransliteratorTrait generic
    pub(crate) fn for_each_run<F>(&mut self, filter: &Filter, mut f: F)
    where
        F: FnMut(&mut Replaceable),
    {
        // runs are in modifiable ranges, so we can only start in our modifiable range.
        let mut start = self.freeze_pre_len;
        // TODO: add to gdoc that this behavior is the same as ICU: transliterations do not happen
        //  when *nothing* in the input matches the filter. this is true even if there are
        //  empty rule matchers (eg `d { > repl`).
        // SAFETY: start is always the result of a function returning valid UTF-8 indices
        while let Some(mut run) = unsafe { self.next_filtered_run(start, filter) } {
            f(&mut run);
            start = run.allowed_upper_bound();
        }
    }

    /// Returns the next run (as a Replaceable with the corresponding frozen range)
    /// that occurs on or after `start`, if one exists.
    ///
    /// # Safety
    /// The caller must ensure that `start` is a valid UTF-8 index into the visible slice.
    unsafe fn next_filtered_run(&mut self, start: usize, filter: &Filter) -> Option<Replaceable> {
        if start == self.allowed_upper_bound() {
            // we have reached the end, there are no more runs
            return None;
        }

        debug_assert!(
            start < self.allowed_upper_bound(),
            "start `{start}` must be within the content length `{}`",
            self.allowed_upper_bound()
        );
        debug_assert!(self.as_str().is_char_boundary(start));

        // TODO: might need to assert start is within the modifiable range

        let run_start;
        let run_end;
        if filter == &Filter::all() {
            // special case for the noop filter

            run_start = start;
            run_end = self.allowed_upper_bound();
        } else {
            run_start = self.find_first_char_in_modifiable_range(start, |c| filter.contains(c))?;
            run_end = self
                .find_first_char_in_modifiable_range(run_start, |c| !filter.contains(c))
                .unwrap_or(self.allowed_upper_bound());
        }

        eprintln!("computing filtered run for rep: {self:?}, start: {start}, run_start: {run_start}, run_end: {run_end}");

        let freeze_post_len = self.visible_content().len() - run_end;

        Some(Replaceable {
            content: self.content,
            // safety: these uphold the invariants
            freeze_pre_len: run_start,
            freeze_post_len,
            // TODO: do we want this?
            cursor: run_start,
            ignore_pre_len: self.ignore_pre_len,
            ignore_post_len: self.ignore_post_len,
        })
    }

    /// Returns the index of the first char in `self.content` that satisfies `f`,
    /// starting at index `start`. The returned index is guaranteed to be a valid UTF-8 index.
    ///
    /// `start` must be a valid UTF-8 index into into the visible slice.
    fn find_first_char_in_modifiable_range<F>(&self, start: usize, f: F) -> Option<usize>
    where
        F: Fn(char) -> bool,
    {
        let tail = &self.as_str()[start..self.allowed_upper_bound()];
        let (idx, _) = tail.char_indices().find(|&(_, c)| f(c))?;
        Some(start + idx)
    }

    /// Returns the current (exclusive) upper bound of the modifiable range.
    ///
    /// This is guaranteed to be a valid UTF-8 index into the visible slice.
    pub(crate) fn allowed_upper_bound(&self) -> usize {
        // eprintln!("allowed_upper_bound called with len: {}, freeze_post_len: {}, ignore_pre_len: {}", self.content.len(), self.freeze_post_len, self.ignore_pre_len);
        self.visible_content().len() - self.freeze_post_len
    }

    /// Returns the byte slice of the content that is currently visible.
    fn visible_content(&self) -> &[u8] {
        &self.content[self.ignore_pre_len..self.visible_content_upper_bound()]
    }

    fn visible_content_upper_bound(&self) -> usize {
        self.content.len() - self.ignore_post_len
    }
}

impl<'a> Debug for Replaceable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", unsafe {
            &str::from_utf8_unchecked(self.content)[..self.ignore_pre_len]
        })?;
        write!(f, "[[[")?;
        write!(f, "{}", &self.as_str()[..self.freeze_pre_len])?;
        write!(f, "{{{{{{")?;
        write!(f, "{}", &self.as_str()[self.freeze_pre_len..self.cursor()])?;
        write!(f, "|||")?;
        write!(
            f,
            "{}",
            &self.as_str()[self.cursor()..self.allowed_upper_bound()]
        )?;
        write!(f, "}}}}}}")?;
        write!(f, "{}", &self.as_str()[self.allowed_upper_bound()..])?;

        Ok(())
    }
}

// TODO: write about invariants. they're basically the same as replaceable's
pub(crate) struct Insertable<'a, 'b> {
    // Replaceable's invariants may temporarily be broken while this Insertable is alive
    _rep: &'b mut Replaceable<'a>,
    start: usize,
    end_len: usize,
    curr: usize,
}

impl<'a, 'b> Insertable<'a, 'b> {
    /// # Safety
    /// The caller must ensure that `range` is a valid UTF-8 range into `content`.
    unsafe fn new_with_size_hint(rep: &'b mut Replaceable<'a>, range: Range<usize>, size_hint: usize) -> Insertable<'a, 'b> {
        let end_len =  rep.content.len() - range.end;
        let s = Self {
            _rep: rep,
            start: range.start,
            end_len,
            curr: range.start,
        };

        let free_bytes = s.free_range().len();
        if free_bytes < size_hint {
            s._rep.content.splice(s.end()..s.end(), core::iter::repeat(0).take(size_hint - free_bytes));
        }
        s
    }

    pub(crate) fn push(&mut self, c: char) {
        let mut buf = [0; 4];
        let c_utf8 = c.encode_utf8(&mut buf);
        self.push_str(c_utf8);
        debug_assert!(self.curr <= self.end());
    }

    pub(crate) fn push_str(&mut self, s: &str) {
        // SAFETY: s is valid UTF-8 by type
        unsafe { self.push_bytes(s.as_bytes()) };
        debug_assert!(self.curr <= self.end());
    }

    /// # Safety
    /// The caller must ensure that `visible_range` is a valid UTF-8 range into the current
    /// replacement.
    pub(super) unsafe fn as_replaceable(&mut self, visible_range: Range<usize>) -> InsertableGuard<impl FnMut(&[u8]) + '_> {
        debug_assert!(visible_range.start <= self.curr_replacement_len());
        debug_assert!(visible_range.end <= self.curr_replacement_len());

        // this is important because the Replaceable's invariant states that the whole content
        // is valid UTF-8.
        self.make_contiguous();

        // The returned replaceable may modify the length of content[self.start..self.curr].
        // Due to the above make_contiguous call, self.curr will always be at self.end() right now.
        // We need a way to update self.curr to self.end() *after* the Replaceable is finished.

        // We do that by returning an InsertableGuard whose `on_drop` callback will update self.curr.

        // the passed visible range is into self.curr_replacement(), which starts at offset self.start
        // of the actual buffer.
        let adjusted_visible_range = visible_range.start + self.start..visible_range.end + self.start;
        let rep = Replaceable::new(self._rep.content, adjusted_visible_range);
        let on_drop = |content: &[u8]| {
            // self.content is contiguous, so we are inserting at the very end
            self.curr = content.len() - self.end_len;
        };
        InsertableGuard::new(rep, on_drop)
    }

    /// # Safety
    /// The caller must ensure that `bytes` is valid UTF-8.
    unsafe fn push_bytes(&mut self, bytes: &[u8]) {
        if self.free_range().len() >= bytes.len() {
            self._rep.content[self.curr..self.curr + bytes.len()].copy_from_slice(bytes);
            self.curr += bytes.len();
            return;
        }
        eprintln!("WARNING: free space not sufficient for Insertable::push_bytes");

        self._rep.content.splice(self.free_range(), bytes.iter().copied());
        self.curr = self.end();
    }

    pub(crate) fn curr_replacement_len(&self) -> usize {
        self.curr - self.start
    }

    pub(crate) fn curr_replacement(&self) -> &str {
        // SAFETY: the invariant states that this part of the content is valid UTF-8
        unsafe { str::from_utf8_unchecked(&self._rep.content[self.start..self.curr]) }
    }

    pub(super) fn commit(mut self, cursor_offset: CursorOffset, data: &MatchData) {
        self.make_contiguous();
        let cursor = cursor_offset.apply(self._rep, data, self.curr_replacement_len());
        // SAFETY: CursorOffset guarantees to be a valid UTF-8 index into the visible slice
        unsafe { self._rep.set_cursor(cursor) };
    }

    fn make_contiguous(&mut self) {
        // need to move the tail of the Vec to fill the remainder of the free range
        self._rep.content.splice(self.free_range(), core::iter::empty());
    }

    fn free_range(&self) -> Range<usize> {
        eprintln!("free_range: curr: {}, end: {}", self.curr, self.end());
        debug_assert!(self.curr <= self.end());
        self.curr..self.end()
    }

    fn end(&self) -> usize {
        self._rep.content.len() - self.end_len
    }
}

impl<'a, 'b> Drop for Insertable<'a, 'b> {
    fn drop(&mut self) {
        self.make_contiguous();
    }
}

impl<'a, 'b> Debug for Insertable<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", unsafe { &str::from_utf8_unchecked(&self._rep.content[self.start..self.curr]) })
    }
}

pub(super) struct InsertableGuard<'a, F>
    where F: FnMut(&[u8])
{
    rep: Replaceable<'a>,
    on_drop: F,
}

impl<'a, F> InsertableGuard<'a, F>
where F: FnMut(&[u8])
{
    fn new(rep: Replaceable<'a>, on_drop: F) -> Self {
        Self { rep, on_drop }
    }

    pub(crate) fn child(&mut self) -> Replaceable {
        self.rep.child()
    }
}

impl<'a, F> Drop for InsertableGuard<'a, F>
where F: FnMut(&[u8])
{
    fn drop(&mut self) {
        (self.on_drop)(&self.rep.content[..]);
    }
}