// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! APIs to support in-place transliteration.
//!
//! Runs
//!

use crate::transliterator::MatchData;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use core::ops::Range;
use core::str;

use super::Filter;

// TODO: do we really want this much unsafe? a lot of the API could be replaced with safe, but
//  checked String methods.

// TODO: if we loosen the invariant of Replaceable UTF-8 content to only need UTF-8 content in
//  the visible range, we would not need to make_contiguous in Insertable, which would avoid
//  moving around the tail after a non-final function call.

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

    // /// # Safety
    // /// The caller must ensure that `range` is a valid UTF-8 range into the visible content.
    // pub(crate) unsafe fn replace_range<'b>(
    //     &'b mut self,
    //     range: Range<usize>,
    //     size_hint: Option<usize>,
    // ) -> Insertable<'a, 'b> {
    //     // TODO: if start is always cursor, maybe change signature to take only end?
    //     //  that would also mean we could replace Insertable.start with Insertable._rep.cursor + ignore_pre_len
    //     // TODO: maybe add checks about frozen range?
    //     // TODO: replace_range should probably be combined with a cursor update, otherwise cursor
    //     //  invariant can break if cursor is inside the range.
    //     debug_assert_eq!(range.start, self.cursor);
    //     // the passed range is into self.visible_content(), which starts at offset self.ignore_pre_len
    //     let adjusted_range = range.start + self.ignore_pre_len..range.end + self.ignore_pre_len;
    //     Insertable::new_with_size_hint(self, adjusted_range, size_hint.unwrap_or(0))
    // }

    pub(crate) fn replace_modifiable_with_str(&mut self, s: &str) {
        let range = self.allowed_range();
        // content is offset by ignore_pre_len relative to the visible content
        let adjusted_range = range.start + self.ignore_pre_len..range.end + self.ignore_pre_len;
        self.content.splice(adjusted_range, s.bytes());
    }

    // pub(crate) fn replace_modifiable_range<'b>(
    //     &'b mut self,
    //     size_hint: Option<usize>,
    // ) -> Insertable<'a, 'b> {
    //     // SAFETY: allowed_range is a valid UTF-8 range into the visible content
    //     unsafe {
    //         self.replace_range(self.allowed_range(), size_hint)
    //     }
    // }

    /// Returns the full current content as a `&str`.
    pub(crate) fn as_str(&self) -> &str {
        debug_assert!(str::from_utf8(self.visible_content()).is_ok());
        // SAFETY: Replaceable's invariant states that content is always valid UTF-8
        unsafe { str::from_utf8_unchecked(self.visible_content()) }
    }

    /// Returns the current modifiable content as a `&str`.
    pub(crate) fn as_str_modifiable(&self) -> &str {
        &self.as_str()[self.allowed_range()]
    }

    /// Returns the current content before the cursor as a `&str`.
    pub(crate) fn as_str_ante(&self) -> &str {
        &self.as_str()[..self.cursor()]
    }

    /// Returns the current modifiable content after the cursor as a `&str`.
    pub(crate) fn as_str_key(&self) -> &str {
        &self.as_str()[self.cursor()..self.allowed_upper_bound()]
    }

    /// Returns the current content after the cursor as a `&str`. `key_match_len` is the length of
    /// the key match and must be a valid UTF-8 index into the visible slice..
    pub(crate) fn as_str_post(&self, key_match_len: usize) -> &str {
        &self.as_str()[(self.cursor() + key_match_len)..]
    }

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
        // // eprintln!("cursor called with raw_cursor: {}, ignore_pre_len: {}", self.raw_cursor, self.ignore_pre_len);
        self.cursor
    }

    /// Advances the cursor by one char.
    pub(crate) fn step_cursor(&mut self) {
        let step_len = self.as_str()[self.cursor()..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(0);
        // // eprintln!("step_cursor: {}", step_len);
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

    pub(super) fn start_match(&mut self) -> RepMatcher<'a, '_, false> {
        let cursor = self.cursor;
        RepMatcher {
            rep: self,
            key_match_len: 0,
            cursor,
            ante_match_len: 0,
            post_match_len: 0,
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

        // eprintln!("computing filtered run for rep: {self:?}, start: {start}, run_start: {run_start}, run_end: {run_end}");

        let freeze_post_len = self.visible_content().len() - run_end;

        Some(Replaceable {
            content: self.content,
            // safety: these uphold the invariants
            freeze_pre_len: run_start,
            freeze_post_len,
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
        // // eprintln!("allowed_upper_bound called with len: {}, freeze_post_len: {}, ignore_pre_len: {}", self.content.len(), self.freeze_post_len, self.ignore_pre_len);
        self.visible_content().len() - self.freeze_post_len
    }

    /// Returns the byte slice of the content that is currently visible.
    fn visible_content(&self) -> &[u8] {
        &self.content[self.ignore_pre_len..self.visible_content_upper_bound()]
    }

    /// Returns the current length of the visible content.
    fn visible_len(&self) -> usize {
        self.visible_content().len()
    }

    fn visible_content_upper_bound(&self) -> usize {
        self.content.len() - self.ignore_post_len
    }
}

impl<'a> Debug for Replaceable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", &self.content[..self.ignore_pre_len])?;
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
        write!(f, "{:?}", &self.content[self.content.len() - self.ignore_post_len..])?;

        Ok(())
    }
}

// For safety reasons, we must ensure that the key is not matched after post matching has started.
// One way to enforce this at compile-time, is with the KEY_FINISHED generic.
#[derive(Debug)]
pub(super) struct RepMatcher<'a, 'b, const KEY_FINISHED: bool> {
    rep: &'b mut Replaceable<'a>,
    key_match_len: usize, // relative to rep.cursor()
    ante_match_len: usize, // relative to rep.cursor()
    post_match_len: usize, // relative to rep.cursor() + key_match_len
    cursor: usize, // relative to the visible content
}

// we can only finish a KEY_FINISHED = true matcher
impl<'a, 'b> RepMatcher<'a, 'b, true> {
    pub(super) fn finish_match(self) -> Insertable<'a, 'b> {
        Insertable::from_matcher(self)
    }
}

// we can only finish matching the key once
impl<'a, 'b> RepMatcher<'a, 'b, false> {
    pub(super) fn finish_key(self) -> RepMatcher<'a, 'b, true> {
        RepMatcher {
            rep: self.rep,
            key_match_len: self.key_match_len,
            ante_match_len: self.ante_match_len,
            post_match_len: self.post_match_len,
            cursor: self.cursor,
        }
    }
}

impl<'a, 'b, const KEY_FINISHED: bool> RepMatcher<'a, 'b, KEY_FINISHED> {
    fn remaining(&self) -> usize {
        if KEY_FINISHED {
            self.rep.visible_len() - self.cursor
        } else {
            self.rep.allowed_upper_bound() - self.cursor
        }
    }

    fn remaining_forward_slice(&self) -> &str {
        if KEY_FINISHED {
            &self.rep.as_str()[self.cursor..]
        } else {
            &self.rep.as_str()[self.cursor..self.rep.allowed_upper_bound()]
        }
    }

    #[inline]
    fn ante_cursor(&self) -> usize {
        self.rep.cursor - self.ante_match_len
    }

    fn remaining_ante_slice(&self) -> &str {
        &self.rep.as_str()[..self.ante_cursor()]
    }
}

impl<'a, 'b, const KEY_FINISHED: bool> Utf8Matcher<Forward> for RepMatcher<'a, 'b, KEY_FINISHED> {
    fn cursor(&self) -> usize {
        self.cursor
    }

    fn str_range(&self, range: Range<usize>) -> Option<&str> {
        self.rep.as_str().get(range)
    }
    
    fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    fn match_str(&self, s: &str) -> bool {
        self.remaining_forward_slice().starts_with(s)
    }

    fn match_start_anchor(&self) -> bool {
        self.cursor == 0
    }

    fn match_end_anchor(&self) -> bool {
        // no matter if we're matching key or post, we must be completely at the end of the string
        self.cursor == self.rep.visible_len()
    }

    fn consume(&mut self, len: usize) -> bool {
        if len <= self.remaining() {
            assert!(self.remaining_forward_slice().is_char_boundary(len));
            if KEY_FINISHED {
                self.post_match_len += len;
            } else {
                self.key_match_len += len;
            }
            self.cursor += len;
            true
        } else {
            false
        }
    }

    fn next_char(&self) -> Option<char> {
        self.remaining_forward_slice().chars().next()
    }
}

// we can always reverse match, no matter if the key has finished matching or not
impl<'a, 'b, const KEY_FINISHED: bool> Utf8Matcher<Reverse> for RepMatcher<'a, 'b, KEY_FINISHED> {
    fn cursor(&self) -> usize {
        self.ante_cursor()
    }

    fn str_range(&self, range: Range<usize>) -> Option<&str> {
        self.rep.as_str().get(range)
    }

    fn is_empty(&self) -> bool {
        self.ante_cursor() == 0
    }

    fn match_str(&self, s: &str) -> bool {
        self.remaining_ante_slice().ends_with(s)
    }

    fn match_start_anchor(&self) -> bool {
        self.ante_cursor() == 0
    }

    fn match_end_anchor(&self) -> bool {
        self.ante_cursor() == self.rep.visible_len()
    }

    fn consume(&mut self, len: usize) -> bool {
        if len <= self.ante_cursor() {
            assert!(self.remaining_ante_slice().is_char_boundary(self.ante_cursor() - len));
            self.ante_match_len += len;
            true
        } else {
            false
        }
    }

    fn next_char(&self) -> Option<char> {
        self.remaining_ante_slice().chars().next_back()
    }
}

mod sealed {
    pub(crate) trait Sealed {}
    impl Sealed for super::Forward {}
    impl Sealed for super::Reverse {}
}

pub(super) struct Forward;
pub(super) struct Reverse;
pub(super) trait MatchDirection: sealed::Sealed {}
impl MatchDirection for Forward {}
impl MatchDirection for Reverse {}

pub(super) trait Utf8Matcher<D: MatchDirection>: Debug {
    fn cursor(&self) -> usize;

    fn str_range(&self, range: Range<usize>) -> Option<&str>;

    fn is_empty(&self) -> bool;
    fn match_str(&self, s: &str) -> bool;

    fn match_and_consume_str(&mut self, s: &str) -> bool {
        if self.match_str(s) {
            self.consume(s.len())
        } else {
            false
        }
    }

    fn match_and_consume_char(&mut self, c: char) -> bool {
        self.match_and_consume_str(c.encode_utf8(&mut [0; 4]))
    }

    fn match_start_anchor(&self) -> bool;
    fn match_end_anchor(&self) -> bool;
    fn consume(&mut self, len: usize) -> bool;
    fn next_char(&self) -> Option<char>;
}


// TODO: write about invariants. they're basically the same as replaceable's
// TODO about complexity: in the worst case, we need to move the full `end_len` tail for every
//  push. A good size_hint avoids this.
//  one fix could be to preemptively move the tail to the end of the buffer whenever the capacity
//  changes. Should give us the same benefits as an exponential allocation strategy.
// TODO: describe what this even is
pub(crate) struct Insertable<'a, 'b> {
    // Replaceable's invariants may temporarily be broken while this Insertable is alive
    _rep: &'b mut Replaceable<'a>,
    start: usize,
    end_len: usize,
    curr: usize,
    ante_match_len: usize,
    key_match_len: usize,
    post_match_len: usize,
}

impl<'a, 'b> Insertable<'a, 'b> {
    // /// # Safety
    // /// The caller must ensure that `range` is a valid UTF-8 range into `content`.
    // unsafe fn new_with_size_hint(
    //     rep: &'b mut Replaceable<'a>,
    //     range: Range<usize>,
    //     size_hint: usize,
    // ) -> Insertable<'a, 'b> {
    //     let end_len = rep.content.len() - range.end;
    //     let mut s = Self {
    //         _rep: rep,
    //         start: range.start,
    //         end_len,
    //         curr: range.start,
    //         key_match_len: 0,
    //         ante_match_len: 0,
    //         post_match_len: 0,
    //     };
    //     s.apply_size_hint(size_hint);
    //     s
    // }

    fn from_matcher(matcher: RepMatcher<'a, 'b, true>) -> Insertable<'a, 'b> {
        // we start replacing from the left
        let start = matcher.rep.ignore_pre_len + matcher.rep.cursor;
        // whatever is not matched *by the key* is unaffected by this Insertable
        let end_len = (matcher.rep.visible_len() - (matcher.rep.cursor() + matcher.key_match_len)) + matcher.rep.ignore_post_len;

        Insertable {
            _rep: matcher.rep,
            start,
            end_len,
            curr: start,
            key_match_len: matcher.key_match_len,
            ante_match_len: matcher.ante_match_len,
            post_match_len: matcher.post_match_len,
        }
    }

    pub(crate) fn apply_size_hint(&mut self, size: usize) {
        let free_bytes = self.free_range().len();
        if free_bytes < size {
            self._rep.content.splice(
                self.end()..self.end(),
                core::iter::repeat(0).take(size - free_bytes),
            );
        }
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
    pub(super) unsafe fn as_replaceable(
        &mut self,
        visible_range: Range<usize>,
    ) -> InsertableGuard<impl FnMut(&[u8]) + '_> {
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
        let adjusted_visible_range =
            visible_range.start + self.start..visible_range.end + self.start;
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
        // eprintln!("WARNING: free space not sufficient for Insertable::push_bytes");

        self._rep
            .content
            .splice(self.free_range(), bytes.iter().copied());
        self.curr = self.end();
    }

    pub(crate) fn curr_replacement_len(&self) -> usize {
        self.curr - self.start
    }

    pub(crate) fn curr_replacement(&self) -> &str {
        // SAFETY: the invariant states that this part of the content is valid UTF-8
        unsafe { str::from_utf8_unchecked(&self._rep.content[self.start..self.curr]) }
    }

    pub(super) fn offset_here(&self) -> CursorOffset {
        // SAFETY: curr_replacement_len returns a valid UTF-8 prefix length of this Insertable
        unsafe { CursorOffset::byte(self.curr_replacement_len()) }
    }

    // TODO(now) rewrite now that no matchdata is passed
    pub(super) fn commit(mut self, cursor_offset: CursorOffset) {
        self.make_contiguous();

        // SAFETY: make_contiguous ensures that `content` is contiguous, valid UTF-8 again, thus
        // we can call methods of Replaceable again.
        let rep = &self._rep;

        let old_cursor = rep.cursor;
        let replacement_len = self.curr_replacement_len();

        let cursor = match cursor_offset.0 {
            CursorOffsetInner::Default => old_cursor + replacement_len,
            CursorOffsetInner::Byte(offset) => old_cursor + offset,
            CursorOffsetInner::CharsOffEnd(count) => {
                // they key has already been replaced, so the post match starts `replacement_len`
                // bytes after the cursor
                let post_start = old_cursor + replacement_len;
                // the replacement d oes not affect the post match length
                let post_end = post_start + self.post_match_len;
                let matched_post = &rep.as_str()[post_start..post_end];
                // compute byte-length of `count` chars in post
                let post_offset_len = matched_post
                    .chars()
                    .take(count as usize)
                    .map(char::len_utf8)
                    .sum::<usize>();

                // we are not allowed to move the cursor into unmodifiable territory
                let max_cursor = rep.allowed_range().end;
                max_cursor.min(old_cursor + replacement_len + post_offset_len)
            }
            CursorOffsetInner::CharsOffStart(count) => {
                let ante = rep.as_str_ante();
                // restricting to the matched substr ensures no overflow of the cursor past the
                // contexts, which is good
                let matched_ante = &ante[(ante.len() - self.ante_match_len)..];
                // compute byte-length of `count` chars in ante (from the right)
                let ante_len = matched_ante
                    .chars()
                    .rev()
                    .take(count as usize)
                    .map(char::len_utf8)
                    .sum::<usize>();

                // clamping cursor to the modifiable range
                let min_cursor = rep.allowed_range().start;
                // not underflowing because ante_len is at most the cursor, because the cursor is
                // right after where the ante context ends.
                min_cursor.max(old_cursor - ante_len)
            }
        };
        // SAFETY: all cases guarantee a valid UTF-8 index into the visible slice
        unsafe { self._rep.set_cursor(cursor) };
    }

    fn make_contiguous(&mut self) {
        // need to move the tail of the Vec to fill the remainder of the free range
        self._rep
            .content
            .splice(self.free_range(), core::iter::empty());
    }

    fn free_range(&self) -> Range<usize> {
        // eprintln!("free_range: curr: {}, end: {}", self.curr, self.end());
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
        write!(f, "{}", unsafe {
            &str::from_utf8_unchecked(&self._rep.content[self.start..self.curr])
        })
    }
}

pub(super) struct InsertableGuard<'a, F>
where
    F: FnMut(&[u8]),
{
    rep: Replaceable<'a>,
    on_drop: F,
}

impl<'a, F> InsertableGuard<'a, F>
where
    F: FnMut(&[u8]),
{
    fn new(rep: Replaceable<'a>, on_drop: F) -> Self {
        Self { rep, on_drop }
    }

    pub(crate) fn child(&mut self) -> Replaceable {
        self.rep.child()
    }
}

impl<'a, F> Drop for InsertableGuard<'a, F>
where
    F: FnMut(&[u8]),
{
    fn drop(&mut self) {
        (self.on_drop)(&self.rep.content[..]);
    }
}

/// Stores the kinds of cursor offsets that a replacement can produce.
#[derive(Debug, Clone, Copy, Default)]
enum CursorOffsetInner {
    /// The default offset, which just puts the cursor at the end of the replacement.
    #[default]
    Default,
    /// A byte offset into the replacement string that is ready to use.
    Byte(usize),
    /// A `char`-based offset for after the replacement string.
    CharsOffEnd(u16),
    /// A `char`-based offset for before the replacement string.
    CharsOffStart(u16),
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct CursorOffset(CursorOffsetInner);

impl CursorOffset {
    /// # Safety
    /// The caller must ensure that `offset` is a valid UTF-8 index into the replacement string
    pub(super) unsafe fn byte(offset: usize) -> Self {
        Self(CursorOffsetInner::Byte(offset))
    }

    pub(super) fn chars_off_end(count: u16) -> Self {
        Self(CursorOffsetInner::CharsOffEnd(count))
    }

    pub(super) fn chars_off_start(count: u16) -> Self {
        Self(CursorOffsetInner::CharsOffStart(count))
    }
}
