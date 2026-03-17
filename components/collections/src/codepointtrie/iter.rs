// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::iter::FusedIterator;
use core::marker::PhantomData;

use crate::codepointtrie::AbstractCodePointTrie;
use crate::codepointtrie::TrieValue;

/// Provides a trie accessor for types (likely iterators)
/// that are holding a reference to a type that implements
/// `AbstractCodePointTrie`.
pub trait WithTrie<'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Get a reference to the trie.
    fn trie(&self) -> &'trie T;
}

/// Iterator over `str` by `char` and `TrieValue`.
#[derive(Debug)]
pub struct CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// # Safety Invariant
    ///
    /// `delegate` must represent a slice of well-formed UTF-8,
    ///  except temporarily within the implementation of `next()` and
    /// `next_back()`. In particular, `next()` and `next_back()` may
    /// assume that this invariant holds upon entry into `next()`/`next_back()`
    /// and the invariant must hold upon return from `next()`/`next_back()`.
    delegate: core::slice::Iter<'slice, u8>,
    trie: &'trie T,
    phantom: PhantomData<V>,
}

impl<'slice, 'trie, T, V> CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Construct a new `CharsWithTrie`.
    #[inline]
    pub fn new(s: &'slice str, trie: &'trie T) -> Self {
        Self {
            // Field invariant upheld: `s` is well-formed UTF-8,
            // so `delegate` is as well.
            delegate: s.as_bytes().iter(),
            trie,
            phantom: PhantomData,
        }
    }

    /// Obtains the remainder of the iterator as a string slice.
    #[inline]
    pub fn as_str(&self) -> &'slice str {
        // SAFETY: OK due to field invariant of `delegate` guaranteeing
        // well-formed UTF-8.
        unsafe { core::str::from_utf8_unchecked(self.delegate.as_slice()) }
    }
}

impl<'slice, 'trie, T, V> Clone for CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            // Field invariant upheld: Clone of well-formed UTF-8 is well-formed UTF-8.
            delegate: self.delegate.clone(),
            trie: self.trie,
            phantom: PhantomData,
        }
    }
}

impl<'slice, 'trie, T, V> WithTrie<'trie, T, V> for CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.trie
    }
}

impl<'slice, 'trie, T, V> Iterator for CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    type Item = (char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lead = *self.delegate.next()?;
        if lead < 0x80 {
            // SAFETY: We checked the invariant of `ascii` immediately
            // above.
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete one-byte sequence.
            return Some((char::from(lead), unsafe { self.trie.ascii(lead) }));
        }
        // SAFETY: Due to the field invariant of `delegate`, we may assume that we
        // have a valid lead byte. Not need to check for other cases.
        if lead < 0xE0 {
            // Two-byte sequence.
            // SAFETY: The field invariant of `delegate` providing UTF-8 well-formedness upon entry into this method and
            // having seen a two-byte lead we may assume the presence of a trail byte.
            let trail = *unsafe { self.delegate.next().unwrap_unchecked() };
            let high_five = u32::from(lead & 0b11_111);
            let low_six = u32::from(trail & 0b111_111);
            // SAFETY: By construction and the two above safety remarks, `high_five` and
            // `low_six` conform to the invariant of `utf8_two_byte`.
            let v = unsafe { self.trie.utf8_two_byte(high_five, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
            // valid (not overlong) two-byte lead and `trail` must be a valid
            // trail. Therefore, the following shift and OR stays in the
            // scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_five << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete two-byte sequence.
            return Some((c, v));
        }
        if lead < 0xF0 {
            // Three-byte sequence.
            // SAFETY: The field invariant of `delegate` providing UTF-8 well-formedness upon entry into this method and
            // having seen a two-byte lead we may assume the presence of two trail bytes.
            let second = *unsafe { self.delegate.next().unwrap_unchecked() };
            let third = *unsafe { self.delegate.next().unwrap_unchecked() };
            let high_ten = (u32::from(lead & 0b1111) << 6) | u32::from(second & 0b111_111);
            let low_six = u32::from(third & 0b111_111);
            // SAFETY: By construction and the safety remarks on the path to this point,
            // `high_ten` and `low_six` conform to the invariant of `utf8_three_byte`.
            let v = unsafe { self.trie.utf8_three_byte(high_ten, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
            // valid (not overlong) three-byte lead and `second` and `third`
            // must be valid trails. Therefore, the following shift and OR
            // stays in the scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_ten << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete three-byte sequence.
            return Some((c, v));
        }
        // Four-byte sequence
        // SAFETY: The field invariant of `delegate` providing UTF-8 well-formedness upon entry into this method and
        // having seen a two-byte lead we may assume the presence of three trail bytes.
        let second = *unsafe { self.delegate.next().unwrap_unchecked() };
        let third = *unsafe { self.delegate.next().unwrap_unchecked() };
        let fourth = *unsafe { self.delegate.next().unwrap_unchecked() };
        // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
        // valid (not overlong or out-of-range) four-byte lead and `second`,
        // `third`, and `fourth` must be valid trails. Therefore, the
        // following shift and OR stays in the scalar value range.
        let c = unsafe {
            char::from_u32_unchecked(
                (u32::from(lead & 0b111) << 18)
                    | (u32::from(second & 0b111_111) << 12)
                    | (u32::from(third & 0b111_111) << 6)
                    | u32::from(fourth & 0b111_111),
            )
        };
        // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
        // a complete four-byte sequence.
        Some((c, self.trie.supplementary(c as u32)))
    }

    #[inline]
    fn count(self) -> usize {
        self.as_str().chars().count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.as_str().chars().size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    // TODO: Delegate advance_by to `Chars` once stabilized.
}

impl<'slice, 'trie, T, V> DoubleEndedIterator for CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let last = *self.delegate.next_back()?;
        if last < 0x80 {
            // SAFETY: We checked the invariant of `ascii` immediately
            // above.
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete one-byte sequence.
            return Some((char::from(last), unsafe { self.trie.ascii(last) }));
        }
        // SAFETY Due to the field invariant of `delegate`,
        // `last` must be a valid trail byte and it is preceded either by a lead byte for a
        // two-byte sequence or by another trail byte.
        let second_last = *unsafe { self.delegate.next_back().unwrap_unchecked() };
        if second_last >= 0b1100_0000 {
            // Two-byte sequence.
            let high_five = u32::from(second_last & 0b11_111);
            let low_six = u32::from(last & 0b111_111);
            // SAFETY: By construction and the two above safety remarks, `high_five` and `low_six` conform
            // to the invariant of `utf8_two_byte`.
            let v = unsafe { self.trie.utf8_two_byte(high_five, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `second_last` must be a
            // valid (not overlong) two-byte lead and `last` must be a valid
            // trail. Therefore, the following shift and OR stays in the
            // scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_five << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete two-byte sequence.
            return Some((c, v));
        }
        // SAFETY: Due to the field invariant of `delegate`,
        // `second_last` must be a valid trail byte and it is preceded either by a lead byte for a
        // three-byte sequence or by another trail byte.
        let third_last = *unsafe { self.delegate.next_back().unwrap_unchecked() };
        if third_last >= 0b1100_0000 {
            // Three-byte sequence
            let high_ten =
                (u32::from(third_last & 0b1111) << 6) | u32::from(second_last & 0b111_111);
            let low_six = u32::from(last & 0b111_111);
            // SAFETY: By construction and the safety remarks on the path to this point, `high_ten` and `low_six` conform
            // to the invariant of `utf8_three_byte`.
            let v = unsafe { self.trie.utf8_three_byte(high_ten, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `third_last` must be a
            // valid (not overlong) three-byte lead and `second_last` and `last`
            // must be valid trails. Therefore, the following shift and OR
            // stays in the scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_ten << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete three-byte sequence.
            return Some((c, v));
        }
        // Four-byte sequence
        // SAFETY: Due to the field invariant of `delegate`, we may assume the
        // presence of a lead byte.
        let lead = *unsafe { self.delegate.next_back().unwrap_unchecked() };
        // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
        // valid (not overlong or out-of-range) four-byte lead and `third_last`,
        // `second_last`, and `last` must be valid trails. Therefore, the
        // following shift and OR stays in the scalar value range.
        let c = unsafe {
            char::from_u32_unchecked(
                (u32::from(lead & 0b111) << 18)
                    | (u32::from(third_last & 0b111_111) << 12)
                    | (u32::from(second_last & 0b111_111) << 6)
                    | u32::from(last & 0b111_111),
            )
        };
        // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
        // a complete four-byte sequence.
        Some((c, self.trie.supplementary(c as u32)))
    }
}

impl<'slice, 'trie, T, V> FusedIterator for CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
}
// --

/// Iterator over `str` by `char` and `TrieValue`.
#[derive(Debug)]
pub struct CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    offset: usize,
    delegate: CharsWithTrie<'slice, 'trie, T, V>,
}

impl<'slice, 'trie, T, V> CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Construct a new `CharIndicesWithTrie`.
    #[inline]
    pub fn new(s: &'slice str, trie: &'trie T) -> Self {
        Self {
            offset: 0,
            delegate: CharsWithTrie::new(s, trie),
        }
    }

    /// Obtains the remainder of the iterator as a string slice.
    #[inline]
    pub fn as_str(&self) -> &'slice str {
        self.delegate.as_str()
    }
}

impl<'slice, 'trie, T, V> Clone for CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            delegate: self.delegate.clone(),
        }
    }
}

impl<'slice, 'trie, T, V> WithTrie<'trie, T, V> for CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.delegate.trie()
    }
}

impl<'slice, 'trie, T, V> Iterator for CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    type Item = (usize, char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let old_len = self.as_str().len();
        let (c, v) = self.delegate.next()?;
        let old_offset = self.offset;
        self.offset += old_len - self.as_str().len();
        Some((old_offset, c, v))
    }

    #[inline]
    fn count(self) -> usize {
        self.as_str().chars().count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.as_str().chars().size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    // TODO: Delegate advance_by to `Chars` once stabilized.
}

impl<'slice, 'trie, T, V> DoubleEndedIterator for CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (c, v) = self.delegate.next_back()?;
        Some((self.offset + self.as_str().len(), c, v))
    }
}

impl<'slice, 'trie, T, V> FusedIterator for CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
}

// --

/// Adds convenience methods to `str`.
pub trait CharsWithTrieEx<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Method for easily creating `CharsWithTrie` on `str` analogously to `chars()`.
    fn chars_with_trie(&'slice self, trie: &'trie T) -> CharsWithTrie<'slice, 'trie, T, V>;

    /// Method for easily creating `CharIndicesWithTrie` on `str` analogously to `char_indices()`.
    fn char_indices_with_trie(
        &'slice self,
        trie: &'trie T,
    ) -> CharIndicesWithTrie<'slice, 'trie, T, V>;
}

impl<'slice, 'trie, T, V> CharsWithTrieEx<'slice, 'trie, T, V> for str
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Method for easily creating `CharsWithTrie` on `str` analogously to `chars()`.
    #[inline]
    fn chars_with_trie(&'slice self, trie: &'trie T) -> CharsWithTrie<'slice, 'trie, T, V> {
        CharsWithTrie::new(self, trie)
    }

    /// Method for easily creating `CharIndicesWithTrie` on `str` analogously to `char_indices()`.
    #[inline]
    fn char_indices_with_trie(
        &'slice self,
        trie: &'trie T,
    ) -> CharIndicesWithTrie<'slice, 'trie, T, V> {
        CharIndicesWithTrie::new(self, trie)
    }
}

// --

/// Iterator over `str` by `char` and `TrieValue` but
/// the trie value for ASCII is `V::default()` instead of
/// reading from the trie. (`V::default()` can be optimized
/// on at compile time while reading the trie's default value
/// is a run-time operation.)
#[derive(Debug)]
pub struct CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// # Safety Invariant
    ///
    /// `delegate` must represent a slice of well-formed UTF-8,
    ///  except temporarily within the implementation of `next()` and
    /// `next_back()`. In particular, `next()` and `next_back()` may
    /// assume that this invariant holds upon entry into `next()`/`next_back()`
    /// and the invariant must hold upon return from `next()`/`next_back()`.
    ///
    /// Note: All-safety-relavant code is copypaste from `CharsWithTrie`.
    delegate: core::slice::Iter<'slice, u8>,
    trie: &'trie T,
    phantom: PhantomData<V>,
}

impl<'slice, 'trie, T, V> CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Construct a new `CharsWithTrieDefaultForAscii`.
    #[inline]
    pub fn new(s: &'slice str, trie: &'trie T) -> Self {
        Self {
            // Field invariant upheld: `s` is well-formed UTF-8,
            // so `delegate` is as well.
            delegate: s.as_bytes().iter(),
            trie,
            phantom: PhantomData,
        }
    }

    /// Obtains the remainder of the iterator as a string slice.
    #[inline]
    pub fn as_str(&self) -> &'slice str {
        // SAFETY: OK due to field invariant of `delegate` guaranteeing
        // well-formed UTF-8.
        unsafe { core::str::from_utf8_unchecked(self.delegate.as_slice()) }
    }
}

impl<'slice, 'trie, T, V> Clone for CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            // Field invariant upheld: Clone of well-formed UTF-8 is well-formed UTF-8.
            delegate: self.delegate.clone(),
            trie: self.trie,
            phantom: PhantomData,
        }
    }
}

impl<'slice, 'trie, T, V> WithTrie<'trie, T, V>
    for CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.trie
    }
}

impl<'slice, 'trie, T, V> Iterator for CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    type Item = (char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lead = *self.delegate.next()?;
        if lead < 0x80 {
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete one-byte sequence.
            return Some((char::from(lead), V::default()));
        }
        // SAFETY: Due to the field invariant of `delegate`, we may assume that we
        // have a valid lead byte. Not need to check for other cases.
        if lead < 0xE0 {
            // Two-byte sequence.
            // SAFETY: The field invariant of `delegate` providing UTF-8 well-formedness upon entry into this method and
            // having seen a two-byte lead we may assume the presence of a trail byte.
            let trail = *unsafe { self.delegate.next().unwrap_unchecked() };
            let high_five = u32::from(lead & 0b11_111);
            let low_six = u32::from(trail & 0b111_111);
            // SAFETY: By construction and the two above safety remarks, `high_five` and
            // `low_six` conform to the invariant of `utf8_two_byte`.
            let v = unsafe { self.trie.utf8_two_byte(high_five, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
            // valid (not overlong) two-byte lead and `trail` must be a valid
            // trail. Therefore, the following shift and OR stays in the
            // scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_five << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete two-byte sequence.
            return Some((c, v));
        }
        if lead < 0xF0 {
            // Three-byte sequence.
            // SAFETY: The field invariant of `delegate` providing UTF-8 well-formedness upon entry into this method and
            // having seen a two-byte lead we may assume the presence of two trail bytes.
            let second = *unsafe { self.delegate.next().unwrap_unchecked() };
            let third = *unsafe { self.delegate.next().unwrap_unchecked() };
            let high_ten = (u32::from(lead & 0b1111) << 6) | u32::from(second & 0b111_111);
            let low_six = u32::from(third & 0b111_111);
            // SAFETY: By construction and the safety remarks on the path to this point,
            // `high_ten` and `low_six` conform to the invariant of `utf8_three_byte`.
            let v = unsafe { self.trie.utf8_three_byte(high_ten, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
            // valid (not overlong) three-byte lead and `second` and `third`
            // must be valid trails. Therefore, the following shift and OR
            // stays in the scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_ten << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete three-byte sequence.
            return Some((c, v));
        }
        // Four-byte sequence
        // SAFETY: The field invariant of `delegate` providing UTF-8 well-formedness upon entry into this method and
        // having seen a two-byte lead we may assume the presence of three trail bytes.
        let second = *unsafe { self.delegate.next().unwrap_unchecked() };
        let third = *unsafe { self.delegate.next().unwrap_unchecked() };
        let fourth = *unsafe { self.delegate.next().unwrap_unchecked() };
        // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
        // valid (not overlong or out-of-range) four-byte lead and `second`,
        // `third`, and `fourth` must be valid trails. Therefore, the
        // following shift and OR stays in the scalar value range.
        let c = unsafe {
            char::from_u32_unchecked(
                (u32::from(lead & 0b111) << 18)
                    | (u32::from(second & 0b111_111) << 12)
                    | (u32::from(third & 0b111_111) << 6)
                    | u32::from(fourth & 0b111_111),
            )
        };
        // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
        // a complete four-byte sequence.
        Some((c, self.trie.supplementary(c as u32)))
    }

    #[inline]
    fn count(self) -> usize {
        self.as_str().chars().count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.as_str().chars().size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    // TODO: Delegate advance_by to `Chars` once stabilized.
}

impl<'slice, 'trie, T, V> DoubleEndedIterator for CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let last = *self.delegate.next_back()?;
        if last < 0x80 {
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete one-byte sequence.
            return Some((char::from(last), V::default()));
        }
        // SAFETY Due to the field invariant of `delegate`,
        // `last` must be a valid trail byte and it is preceded either by a lead byte for a
        // two-byte sequence or by another trail byte.
        let second_last = *unsafe { self.delegate.next_back().unwrap_unchecked() };
        if second_last >= 0b1100_0000 {
            // Two-byte sequence.
            let high_five = u32::from(second_last & 0b11_111);
            let low_six = u32::from(last & 0b111_111);
            // SAFETY: By construction and the two above safety remarks, `high_five` and `low_six` conform
            // to the invariant of `utf8_two_byte`.
            let v = unsafe { self.trie.utf8_two_byte(high_five, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `second_last` must be a
            // valid (not overlong) two-byte lead and `last` must be a valid
            // trail. Therefore, the following shift and OR stays in the
            // scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_five << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete two-byte sequence.
            return Some((c, v));
        }
        // SAFETY: Due to the field invariant of `delegate`,
        // `second_last` must be a valid trail byte and it is preceded either by a lead byte for a
        // three-byte sequence or by another trail byte.
        let third_last = *unsafe { self.delegate.next_back().unwrap_unchecked() };
        if third_last >= 0b1100_0000 {
            // Three-byte sequence
            let high_ten =
                (u32::from(third_last & 0b1111) << 6) | u32::from(second_last & 0b111_111);
            let low_six = u32::from(last & 0b111_111);
            // SAFETY: By construction and the safety remarks on the path to this point, `high_ten` and `low_six` conform
            // to the invariant of `utf8_three_byte`.
            let v = unsafe { self.trie.utf8_three_byte(high_ten, low_six) };
            // SAFETY: Due to the field invariant of `delegate`, `third_last` must be a
            // valid (not overlong) three-byte lead and `second_last` and `last`
            // must be valid trails. Therefore, the following shift and OR
            // stays in the scalar value range.
            let c = unsafe { char::from_u32_unchecked((high_ten << 6) | low_six) };
            // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
            // a complete three-byte sequence.
            return Some((c, v));
        }
        // Four-byte sequence
        // SAFETY: Due to the field invariant of `delegate`, we may assume the
        // presence of a lead byte.
        let lead = *unsafe { self.delegate.next_back().unwrap_unchecked() };
        // SAFETY: Due to the field invariant of `delegate`, `lead` must be a
        // valid (not overlong or out-of-range) four-byte lead and `third_last`,
        // `second_last`, and `last` must be valid trails. Therefore, the
        // following shift and OR stays in the scalar value range.
        let c = unsafe {
            char::from_u32_unchecked(
                (u32::from(lead & 0b111) << 18)
                    | (u32::from(third_last & 0b111_111) << 12)
                    | (u32::from(second_last & 0b111_111) << 6)
                    | u32::from(last & 0b111_111),
            )
        };
        // Field invariant upheld: `delegate` is again well-formed UTF-8, because we consumed
        // a complete four-byte sequence.
        Some((c, self.trie.supplementary(c as u32)))
    }
}

impl<'slice, 'trie, T, V> FusedIterator for CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
}
// --

/// Iterator over `str` by `char` and `TrieValue`.
#[derive(Debug)]
pub struct CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    offset: usize,
    delegate: CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>,
}

impl<'slice, 'trie, T, V> CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Construct a new `CharIndicesWithTrieDefaultForAscii`.
    #[inline]
    pub fn new(s: &'slice str, trie: &'trie T) -> Self {
        Self {
            offset: 0,
            delegate: CharsWithTrieDefaultForAscii::new(s, trie),
        }
    }

    /// Obtains the remainder of the iterator as a string slice.
    #[inline]
    pub fn as_str(&self) -> &'slice str {
        self.delegate.as_str()
    }
}

impl<'slice, 'trie, T, V> Clone for CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            delegate: self.delegate.clone(),
        }
    }
}

impl<'slice, 'trie, T, V> WithTrie<'trie, T, V>
    for CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.delegate.trie()
    }
}

impl<'slice, 'trie, T, V> Iterator for CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    type Item = (usize, char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let old_len = self.as_str().len();
        let (c, v) = self.delegate.next()?;
        let old_offset = self.offset;
        self.offset += old_len - self.as_str().len();
        Some((old_offset, c, v))
    }

    #[inline]
    fn count(self) -> usize {
        self.as_str().chars().count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.as_str().chars().size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    // TODO: Delegate advance_by to `Chars` once stabilized.
}

impl<'slice, 'trie, T, V> DoubleEndedIterator
    for CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (c, v) = self.delegate.next_back()?;
        Some((self.offset + self.as_str().len(), c, v))
    }
}

impl<'slice, 'trie, T, V> FusedIterator for CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
}

// --

/// Adds convenience methods to `str`.
pub trait CharsWithTrieDefaultForAsciiEx<'slice, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Method for easily creating `CharsWithTrie` on `str` analogously to `chars()`.
    fn chars_with_trie_default_for_ascii(
        &'slice self,
        trie: &'trie T,
    ) -> CharsWithTrieDefaultForAscii<'slice, 'trie, T, V>;

    /// Method for easily creating `CharIndicesWithTrie` on `str` analogously to `char_indices()`.
    fn char_indices_with_trie_default_for_ascii(
        &'slice self,
        trie: &'trie T,
    ) -> CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V>;
}

impl<'slice, 'trie, T, V> CharsWithTrieDefaultForAsciiEx<'slice, 'trie, T, V> for str
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Method for easily creating `CharsWithTrie` on `str` analogously to `chars()`.
    #[inline]
    fn chars_with_trie_default_for_ascii(
        &'slice self,
        trie: &'trie T,
    ) -> CharsWithTrieDefaultForAscii<'slice, 'trie, T, V> {
        CharsWithTrieDefaultForAscii::new(self, trie)
    }

    /// Method for easily creating `CharIndicesWithTrie` on `str` analogously to `char_indices()`.
    #[inline]
    fn char_indices_with_trie_default_for_ascii(
        &'slice self,
        trie: &'trie T,
    ) -> CharIndicesWithTrieDefaultForAscii<'slice, 'trie, T, V> {
        CharIndicesWithTrieDefaultForAscii::new(self, trie)
    }
}

// --

/// Iterator over Latin1 `[u8]` by `char` and `TrieValue`.
#[derive(Debug)]
pub struct Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    delegate: core::slice::Iter<'slice, u8>,
    trie: &'trie T,
    phantom: PhantomData<V>,
}

impl<'slice, 'trie, T, V> Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Construct a new `Latin1CharsWithTrie`.
    #[inline]
    pub fn new(s: &'slice [u8], trie: &'trie T) -> Self {
        Self {
            delegate: s.iter(),
            trie,
            phantom: PhantomData,
        }
    }

    /// Obtains the remainder of the iterator as a slice.
    #[inline]
    pub fn as_slice(&self) -> &'slice [u8] {
        self.delegate.as_slice()
    }
}

impl<'slice, 'trie, T, V> Clone for Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            delegate: self.delegate.clone(),
            trie: self.trie,
            phantom: PhantomData,
        }
    }
}

impl<'slice, 'trie, T, V> WithTrie<'trie, T, V> for Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.trie
    }
}

impl<'slice, 'trie, T, V> Iterator for Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    type Item = (char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let b = *self.delegate.next()?;
        Some((char::from(b), self.trie.latin1(b)))
    }

    #[inline]
    fn count(self) -> usize {
        self.delegate.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.delegate.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    // TODO: Delegate advance_by to `delegate` once stabilized.
}

impl<'slice, 'trie, T, V> DoubleEndedIterator for Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let b = *self.delegate.next_back()?;
        Some((char::from(b), self.trie.latin1(b)))
    }
}

impl<'slice, 'trie, T, V> FusedIterator for Latin1CharsWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
}

// --

/// Iterator over `str` by `char` and `TrieValue`.
#[derive(Debug)]
pub struct Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    offset: usize,
    delegate: core::slice::Iter<'slice, u8>,
    trie: &'trie T,
    phantom: PhantomData<V>,
}

impl<'slice, 'trie, T, V> Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Construct a new `Latin1CharIndicesWithTrie`.
    #[inline]
    pub fn new(s: &'slice [u8], trie: &'trie T) -> Self {
        Self {
            offset: 0,
            delegate: s.iter(),
            trie,
            phantom: PhantomData,
        }
    }

    /// Obtains the remainder of the iterator as a slice.
    #[inline]
    pub fn as_slice(&self) -> &'slice [u8] {
        self.delegate.as_slice()
    }
}

impl<'slice, 'trie, T, V> Clone for Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            delegate: self.delegate.clone(),
            trie: self.trie,
            phantom: PhantomData,
        }
    }
}

impl<'slice, 'trie, T, V> WithTrie<'trie, T, V> for Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.trie
    }
}

impl<'slice, 'trie, T, V> Iterator for Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    type Item = (usize, char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let b = *self.delegate.next()?;
        let old_offset = self.offset;
        self.offset += 1;
        Some((old_offset, char::from(b), self.trie.latin1(b)))
    }

    #[inline]
    fn count(self) -> usize {
        self.delegate.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.delegate.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }

    // TODO: Delegate advance_by to `delegate` once stabilized.
}

impl<'slice, 'trie, T, V> DoubleEndedIterator for Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let b = *self.delegate.next_back()?;
        Some((
            self.offset + self.as_slice().len(),
            char::from(b),
            self.trie.latin1(b),
        ))
    }
}

impl<'slice, 'trie, T, V> FusedIterator for Latin1CharIndicesWithTrie<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
}

// --

/// Adds convenience methods to `[u8]`.
pub trait Latin1CharsWithTrieEx<'slice, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Method for easily creating `Latin1CharsWithTrie` on `[u8]` analogously to `chars()` on `str`.
    /// (The name is prefixed with `latin1_` to avoid ambiguity with interpreting [u8] as UTF-8.)
    fn latin1_chars_with_trie(
        &'slice self,
        trie: &'trie T,
    ) -> Latin1CharsWithTrie<'slice, 'trie, T, V>;

    /// Method for easily creating `Latin1CharIndicesWithTrie` on `str` analogously to `char_indices()` on `str`.
    /// (The name is prefixed with `latin1_` to avoid ambiguity with interpreting [u8] as UTF-8.)
    fn latin1_char_indices_with_trie(
        &'slice self,
        trie: &'trie T,
    ) -> Latin1CharIndicesWithTrie<'slice, 'trie, T, V>;
}

impl<'slice, 'trie, T, V> Latin1CharsWithTrieEx<'slice, 'trie, T, V> for [u8]
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    /// Method for easily creating `Latin1CharsWithTrie` on `[u8]` analogously to `chars()` on `str`.
    /// (The name is prefixed with `latin1_` to avoid ambiguity with interpreting [u8] as UTF-8.)
    #[inline]
    fn latin1_chars_with_trie(
        &'slice self,
        trie: &'trie T,
    ) -> Latin1CharsWithTrie<'slice, 'trie, T, V> {
        Latin1CharsWithTrie::new(self, trie)
    }

    /// Method for easily creating `Latin1CharIndicesWithTrie` on `str` analogously to `char_indices()` on `str`.
    /// (The name is prefixed with `latin1_` to avoid ambiguity with interpreting [u8] as UTF-8.)
    #[inline]
    fn latin1_char_indices_with_trie(
        &'slice self,
        trie: &'trie T,
    ) -> Latin1CharIndicesWithTrie<'slice, 'trie, T, V> {
        Latin1CharIndicesWithTrie::new(self, trie)
    }
}

// --

/// Wraps an `Iterator<Item = char>` with a reference to
/// an `AbstractCodePointTrie`.
#[derive(Debug)]
pub struct CharIterWithTrie<'trie, T, V, I>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
    I: Iterator<Item = char>,
{
    delegate: I,
    trie: &'trie T,
    phantom: PhantomData<V>,
}

impl<'trie, T, V, I> CharIterWithTrie<'trie, T, V, I>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
    I: Iterator<Item = char>,
{
    /// Constructs a new `CharIterWithTrie`.
    #[inline]
    pub fn new(iter: I, trie: &'trie T) -> Self {
        Self {
            delegate: iter,
            trie,
            phantom: PhantomData,
        }
    }
}

impl<'trie, T, V, I> WithTrie<'trie, T, V> for CharIterWithTrie<'trie, T, V, I>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
    I: Iterator<Item = char>,
{
    #[inline]
    fn trie(&self) -> &'trie T {
        self.trie
    }
}

impl<'trie, T, V, I> Iterator for CharIterWithTrie<'trie, T, V, I>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
    I: Iterator<Item = char>,
{
    type Item = (char, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.delegate.next()?;
        Some((c, self.trie.scalar(c)))
    }

    #[inline]
    fn count(self) -> usize {
        self.delegate.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.delegate.size_hint()
    }

    // Looks like conditionally implementing `last()` is not allowed.

    // TODO: Delegate advance_by to `delegate` once stabilized.
}

impl<'trie, T, V, I> DoubleEndedIterator for CharIterWithTrie<'trie, T, V, I>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
    I: DoubleEndedIterator<Item = char>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let c = self.delegate.next_back()?;
        Some((c, self.trie.scalar(c)))
    }
}

impl<'trie, T, V, I> FusedIterator for CharIterWithTrie<'trie, T, V, I>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
    I: FusedIterator<Item = char>,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let trie = crate::codepointtrie::planes::get_planes_trie();
        let s = "abäαあ🥳𧉧";
        let mut iter = s.chars_with_trie(&trie);
        assert_eq!(iter.next(), Some(('a', 0)));
        assert_eq!(iter.next(), Some(('b', 0)));
        assert_eq!(iter.next(), Some(('ä', 0)));
        assert_eq!(iter.next(), Some(('α', 0)));
        assert_eq!(iter.next(), Some(('あ', 0)));
        assert_eq!(iter.next(), Some(('🥳', 1)));
        assert_eq!(iter.next(), Some(('𧉧', 2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_backwards() {
        let trie = crate::codepointtrie::planes::get_planes_trie();
        let s = "abäαあ🥳𧉧";
        let mut iter = s.chars_with_trie(&trie);
        assert_eq!(iter.next_back(), Some(('𧉧', 2)));
        assert_eq!(iter.next_back(), Some(('🥳', 1)));
        assert_eq!(iter.next_back(), Some(('あ', 0)));
        assert_eq!(iter.next_back(), Some(('α', 0)));
        assert_eq!(iter.next_back(), Some(('ä', 0)));
        assert_eq!(iter.next_back(), Some(('b', 0)));
        assert_eq!(iter.next_back(), Some(('a', 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_indices_forward() {
        let trie = crate::codepointtrie::planes::get_planes_trie();
        let s = "abäαあ🥳𧉧";
        let mut iter = s.char_indices_with_trie(&trie);
        assert_eq!(iter.next(), Some((0, 'a', 0)));
        assert_eq!(iter.next(), Some((1, 'b', 0)));
        assert_eq!(iter.next(), Some((2, 'ä', 0)));
        assert_eq!(iter.next(), Some((4, 'α', 0)));
        assert_eq!(iter.next(), Some((6, 'あ', 0)));
        assert_eq!(iter.next(), Some((9, '🥳', 1)));
        assert_eq!(iter.next(), Some((13, '𧉧', 2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_indices_backwards() {
        let trie = crate::codepointtrie::planes::get_planes_trie();
        let s = "abäαあ🥳𧉧";
        let mut iter = s.char_indices_with_trie(&trie);
        assert_eq!(iter.next_back(), Some((13, '𧉧', 2)));
        assert_eq!(iter.next_back(), Some((9, '🥳', 1)));
        assert_eq!(iter.next_back(), Some((6, 'あ', 0)));
        assert_eq!(iter.next_back(), Some((4, 'α', 0)));
        assert_eq!(iter.next_back(), Some((2, 'ä', 0)));
        assert_eq!(iter.next_back(), Some((1, 'b', 0)));
        assert_eq!(iter.next_back(), Some((0, 'a', 0)));
        assert_eq!(iter.next(), None);
    }
}
