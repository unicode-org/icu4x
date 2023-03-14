// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::RangeInclusive;

/// This is an iterator that turns an iterator over code point ranges
/// into an iterator producing the complement set of ranges
pub(crate) struct RangeListIteratorComplementer<I> {
    iter: I,
    /// The previous range closing value we saw
    prev_end: Option<u32>,
}

impl<I> RangeListIteratorComplementer<I>
where
    I: Iterator<Item = RangeInclusive<u32>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            prev_end: None,
        }
    }
}

const MAX: u32 = char::MAX as u32;

impl<I> Iterator for RangeListIteratorComplementer<I>
where
    I: Iterator<Item = RangeInclusive<u32>>,
{
    type Item = RangeInclusive<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev_end) = self.prev_end {
            // We had already begun iterating

            if prev_end == MAX {
                // We're already done iterating
                //
                // Code point iteration should never hit MAX if there are still elements
                // to yield
                debug_assert!(self.iter.next().is_none());
                return None;
            }

            let next = self.iter.next();
            // These are inclusive ranges, we should explicitly exclude
            // the bounds
            let start = prev_end + 1;
            if let Some(next) = next {
                // Inclusive range, exclude the bound when complementing
                let end = *next.start() - 1;
                self.prev_end = Some(*next.end());
                return Some(start..=end);
            } else {
                self.prev_end = Some(MAX);
                return Some(start..=MAX);
            }
        } else {
            // We've just started iterating
            let next = self.iter.next();
            if let Some(next) = next {
                if *next.start() == 0 {
                    // The iterator starts at 0, so we don't need to insert a 0..=something range.
                    if *next.end() == MAX {
                        // The iterator covers the entire range, its complement is empty
                        return None;
                    } else {
                        // Inclusive range, exclude the bound when complementing
                        let start = *next.end() + 1;

                        // We are complementing this set, and so far we don't know where our first
                        // complemented range ends, so we need more data to get it
                        let next2 = self.iter.next();
                        if let Some(next2) = next2 {
                            // Inclusive range, exclude the bound when complementing
                            let end = *next2.start() - 1;
                            self.prev_end = Some(*next2.end());
                            return Some(start..=end);
                        } else {
                            // We only had one entry in the set from 0..=end,
                            // its complement is (end + 1)..=MAX
                            self.prev_end = Some(MAX);
                            return Some(start..=MAX);
                        }
                    }
                } else {
                    // Range starts at nonzero value, so first
                    // entry in complement will be 0..=(start - 1)
                    self.prev_end = Some(*next.end());
                    // Inclusive range, exclude the bound when complementing
                    let end = *next.start() - 1;
                    return Some(0..=end);
                }
            } else {
                // Completely empty iterator! Return the full range

                // Also set prev_end to MAX so that we don't come down this branch again
                self.prev_end = Some(MAX);
                return Some(0..=MAX);
            }
        }
    }
}

/// This is an iterator that coalesces adjacent ranges in an iterator over code
/// point ranges
pub(crate) struct RangeListIteratorCoalescer<I> {
    iter: I,
    peek: Option<RangeInclusive<u32>>,
}

impl<I> RangeListIteratorCoalescer<I>
where
    I: Iterator<Item = RangeInclusive<u32>>,
{
    pub fn new(iter: I) -> Self {
        Self { iter, peek: None }
    }
}

impl<I> Iterator for RangeListIteratorCoalescer<I>
where
    I: Iterator<Item = RangeInclusive<u32>>,
{
    type Item = RangeInclusive<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the initial range we're working with: either a leftover
        // range from last time, or the next range
        let (start, mut end) = if let Some(ref peek) = self.peek {
            (*peek.start(), *peek.end())
        } else if let Some(next) = self.iter.next() {
            (*next.start(), *next.end())
        } else {
            // No ranges, exit early
            return None;
        };

        // Keep pulling ranges
        while let Some(next) = self.iter.next() {
            if *next.start() == end + 1 {
                // Range has no gap, coalesce
                end = *next.end();
            } else {
                // Range has a gap, return what we have so far, update
                // peek
                self.peek = Some(next);
                return Some(start..=end);
            }
        }

        // Ran out of elements, exit
        self.peek = None;
        Some(start..=end)
    }
}
