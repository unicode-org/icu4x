// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Similar to [`core::str::CharIndices`] for Latin-1 strings, represented as `[u8]`.
///
/// Contrary to [`core::str::CharIndices`], the second element of the
/// [`Iterator::Item`] is a [`u8`], representing a Unicode scalar value in the
/// range U+0000–U+00FF.
#[derive(Clone, Debug)]
pub struct Latin1Indices<'a> {
    front_offset: usize,
    back_offset: usize,
    iter: &'a [u8],
}

impl<'a> Latin1Indices<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            front_offset: 0,
            back_offset: input.len(),
            iter: input,
        }
    }
}

impl Iterator for Latin1Indices<'_> {
    type Item = (usize, u8);

    #[inline]
    fn next(&mut self) -> Option<(usize, u8)> {
        if self.front_offset >= self.back_offset {
            return None;
        }
        self.iter.get(self.front_offset).map(|ch| {
            self.front_offset += 1;
            (self.front_offset - 1, *ch)
        })
    }
}

impl DoubleEndedIterator for Latin1Indices<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, u8)> {
        if self.front_offset >= self.back_offset {
            return None;
        }
        self.back_offset -= 1;
        self.iter
            .get(self.back_offset)
            .map(|ch| (self.back_offset, *ch))
    }
}

/// Similar to [`core::str::CharIndices`] for UTF-16 strings, represented as `[u16]`.
///
/// Contrary to [`core::str::CharIndices`], the second element of the
/// [`Iterator::Item`] is a Unicode code point represented by a [`u32`],
/// rather than a Unicode scalar value represented by a [`char`], because this
/// iterator preserves unpaired surrogates.
#[derive(Clone, Debug)]
pub struct Utf16Indices<'a> {
    front_offset: usize,
    back_offset: usize,
    iter: &'a [u16],
}

impl<'a> Utf16Indices<'a> {
    pub fn new(input: &'a [u16]) -> Self {
        Self {
            front_offset: 0,
            back_offset: input.len(),
            iter: input,
        }
    }
}

impl Iterator for Utf16Indices<'_> {
    type Item = (usize, u32);

    #[inline]
    fn next(&mut self) -> Option<(usize, u32)> {
        if self.front_offset >= self.back_offset {
            return None;
        }
        let (index, ch) = self.iter.get(self.front_offset).map(|ch| {
            self.front_offset += 1;
            (self.front_offset - 1, *ch)
        })?;

        let mut ch = ch as u32;
        if (ch & 0xfc00) != 0xd800 {
            return Some((index, ch));
        }

        if let Some(next) = self.iter.get(self.front_offset) {
            let next = *next as u32;
            if (next & 0xfc00) == 0xdc00 {
                // Combine low and high surrogates to UTF-32 code point.
                ch = ((ch & 0x3ff) << 10) + (next & 0x3ff) + 0x10000;
                self.front_offset += 1;
            }
        }
        Some((index, ch))
    }
}

impl DoubleEndedIterator for Utf16Indices<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, u32)> {
        if self.front_offset >= self.back_offset {
            return None;
        }
        self.back_offset -= 1;
        let mut ch = *self.iter.get(self.back_offset)? as u32;

        // Check if current char is a Low Surrogate
        if (ch & 0xfc00) == 0xdc00 && self.back_offset > self.front_offset {
            if let Some(&prev) = self.iter.get(self.back_offset - 1) {
                let prev = prev as u32;
                if (prev & 0xfc00) == 0xd800 {
                    self.back_offset -= 1;
                    ch = ((prev & 0x3ff) << 10) + (ch & 0x3ff) + 0x10000;
                }
            }
        }

        Some((self.back_offset, ch))
    }
}

#[cfg(test)]
mod tests {
    use crate::indices::*;

    #[test]
    fn latin1_indices() {
        let latin1 = [0x30, 0x31, 0x32];
        let mut indices = Latin1Indices::new(&latin1);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x30);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 1);
        assert_eq!(n.1, 0x31);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x32);
        let n = indices.next();
        assert_eq!(n, None);
    }

    #[test]
    fn utf16_indices_double_ended() {
        let utf16 = [0xd83d, 0xde03, 0x0020, 0xd83c, 0xdf00, 0xd800, 0x0020];
        let mut indices = Utf16Indices::new(&utf16);

        // Check for forward
        assert_eq!(indices.next(), Some((0, 0x1f603)));
        assert_eq!(indices.next(), Some((2, 0x0020)));

        // Check for backward
        assert_eq!(indices.next_back(), Some((6, 0x0020)));
        assert_eq!(indices.next_back(), Some((5, 0xd800)));
        assert_eq!(indices.next_back(), Some((3, 0x1f300)));

        // Check for empty
        assert_eq!(indices.next(), None);
        assert_eq!(indices.next_back(), None);
    }

    #[test]
    fn utf16_indices() {
        let utf16 = [0xd83d, 0xde03, 0x0020, 0xd83c, 0xdf00, 0xd800, 0x0020];
        let mut indices = Utf16Indices::new(&utf16);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x1f603);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x20);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 3);
        assert_eq!(n.1, 0x1f300);
        // This is invalid surrogate pair.
        let n = indices.next().unwrap();
        assert_eq!(n.0, 5);
        assert_eq!(n.1, 0xd800);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 6);
        assert_eq!(n.1, 0x0020);
        let n = indices.next();
        assert_eq!(n, None);
    }
}
