// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
use {
    anyhow::{format_err, Error},
    std::{
        clone::Clone,
        cmp::Ordering,
        hash::{Hash, Hasher},
        iter::Iterator,
        ops::Range,
        vec::Vec,
    },
    unic_char_range::{chars, CharIter, CharRange},
};
/// A trait for objects that represent one or more disjoint, non-adjacent
/// [CharRanges](unic_char_range::CharRange).
pub trait MultiCharRange {
    /// Iterate over the disjoint, non-adjacent [CharRange]s in the collection in ascending order.
    fn iter_ranges<'a>(&'a self) -> Box<dyn Iterator<Item = CharRange> + 'a>;
    /// The number of ranges in the collection.
    fn range_count(&self) -> usize;
}
/// A collection of `char`s (i.e. Unicode code points), used for storing large continuous ranges
/// efficiently.
///
/// Lookups and insertions are O(log <var>R</var>), where <var>R</var> is the number of disjoint
/// ranges in the collection.
///
/// The easiest way to create instances is using the
/// [char_collect!](::char_collection::char_collect) macro.
///
/// ```
/// use char_collection::CharCollection;
///
/// let mut collection: CharCollection = char_collect!('a'..='d', 'x'..='z');
/// char_collection += 'e';
/// char_collection += chars!('p'..='t');
/// assert_eq!(
///     collection.iter_ranges().collect(),
///     vec![chars!('a'..='e'), chars!('p'..='t'), chars!('x'..='z')]);
///
/// assert!(collection.contains(&'c'));
/// assert!(collection.contains_range(chars!('q'..='s')));
/// assert!(!collection.contains(&'9'));
///
/// collection -= chars!('t'..='y');
/// assert_eq!(
///     collection.iter_ranges().collect(),
///     vec![chars!('a'..='e', chars!('p'..'s'), chars!('z'..='z'))]);
/// ```
///
/// TODO(kpozin): Implement IntoIter.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct CharCollection {
    ranges: Vec<CharRange>,
}
impl CharCollection {
    /// Create a new, empty `CharCollection`.
    pub fn new() -> CharCollection {
        CharCollection::default()
    }
    /// Create a new `CharCollection` from a list of disjoint, non-adjacent `CharRange`s, pre-sorted
    /// in ascending code point order.
    ///
    /// This factory method is primarily intended for use in deserializing valid representations of
    /// `CharCollections`. Will return an error if ranges are out of order, overlapping, or
    /// adjacent.
    pub fn from_sorted_ranges<T>(ranges: T) -> Result<CharCollection, Error>
    where
        T: IntoIterator<Item = CharRange>,
    {
        // If the original `ranges` is also a Vec, this doesn't result in an extra copy.
        let collection = CharCollection { ranges: ranges.into_iter().collect() };
        let ranges: &Vec<CharRange> = &collection.ranges;
        match (1..ranges.len()).find(|i| (ranges[*i].low as i64 - ranges[*i - 1].high as i64) <= 1)
        {
            Some(i) => Err(format_err!(
                "These ranges are out of order, overlapping, or adjacent: {}, {}",
                format_range(&ranges[i - 1]),
                format_range(&ranges[i])
            )),
            None => Ok(collection),
        }
    }
    /// Create a new `CharCollection` from a list of `char`s, pre-sorted in ascending code point
    /// order.
    ///
    /// This factory method is primarily intended for use in deserializing valid representations of
    /// `CharCollections`. Will return an error if chars are out of order or contain duplicates.
    pub fn from_sorted_chars<T>(chars: T) -> Result<CharCollection, Error>
    where
        T: IntoIterator<Item = char>,
    {
        let mut collection = CharCollection::new();
        for ch in chars.into_iter() {
            collection.append(ch)?;
        }
        Ok(collection)
    }
    /// Iterate over all the `char`s in the collection.
    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.ranges.iter().flat_map(CharRange::iter)
    }
    /// Test whether the collection contains a specific `char`.
    ///
    /// The time complexity is O(log <var>R</var>), where <var>R</var> is the number of ranges in
    /// the collection.
    pub fn contains(&self, ch: &char) -> bool {
        self.find_containing_range(ch).is_ok()
    }
    /// Test whether the collection contains an entire range of characters.
    ///
    /// The time complexity is O(log <var>R</var>), where <var>R</var> is the number of ranges in
    /// the collection.
    pub fn contains_range(&self, range: &CharRange) -> bool {
        if range.is_empty() {
            return false;
        }
        let lower_existing_range = self.find_containing_range(&range.low);
        let upper_existing_range = self.find_containing_range(&range.high);
        // Fully enclosed in existing range.
        return lower_existing_range == upper_existing_range && lower_existing_range.is_ok();
    }
    /// Insert a `char` or other collection of chars into this collection.
    ///
    /// Returns `&mut self` for easy chaining.
    ///
    /// The time complexity is O(<var>T</var> log(<var>R</var> + <var>T</var>)), where <var>R</var>
    /// is the number of ranges in this collection and <var>T</var> is the number of ranges in
    /// `to_add`.
    pub fn insert<V: MultiCharRange>(&mut self, to_add: &V) -> &mut Self {
        to_add.iter_ranges().for_each(|range| self.insert_char_range(&range));
        self
    }
    /// Appends a `char` to the end of the existing collection. Panics if the given `char` is not
    /// higher than the highest code point in the existing collection.
    ///
    /// Returns `&mut self` for easy chaining.
    ///
    /// The time complexity is O(1).
    pub fn append(&mut self, ch: char) -> Result<&mut Self, Error> {
        let mut coalesced = false;
        if let Some(last_range) = self.ranges.last_mut() {
            if last_range.cmp_char(ch) != Ordering::Less {
                return Err(format_err!("Cannot append {} after {}", ch, last_range.high));
            }
            if are_chars_adjacent(&last_range.high, &ch) {
                last_range.high = ch;
                coalesced = true;
            }
        }
        if !coalesced {
            self.ranges.push(chars!(ch..=ch));
        }
        Ok(self)
    }
    /// Appends a `CharRange` to the end of the existing collection. Panics if the given range is
    /// not higher than the highest code point in the existing collection. (The new range _may_ be
    /// adjacent to the previous highest range, but may not overlap.)
    ///
    /// Returns `&mut self` for easy chaining.
    ///
    /// The time complexity is O(1).
    pub fn append_range(&mut self, range: CharRange) -> Result<&mut Self, Error> {
        let mut coalesced = false;
        if let Some(last_range) = self.ranges.last_mut() {
            if last_range.cmp_char(range.low) != Ordering::Less {
                return Err(format_err!(
                    "Cannot append {} after {}",
                    format_range(&range),
                    last_range.high
                ));
            }
            if are_chars_adjacent(&last_range.high, &range.low) {
                last_range.high = range.high;
                coalesced = true;
            }
        }
        if !coalesced {
            self.ranges.push(range);
        }
        Ok(self)
    }
    /// Remove a `char` or other collection of chars from this collection.
    ///
    /// Returns `&mut self` for easy chaining.
    ///
    /// The time complexity is O(<var>T</var> log(<var>R</var> + <var>T</var>)), where <var>R</var>
    /// is the number of ranges in this collection and <var>T</var> is the number of ranges in
    /// `to_remove`.
    pub fn remove<V: MultiCharRange>(&mut self, to_remove: &V) -> &mut Self {
        to_remove.iter_ranges().for_each(|range| self.remove_char_range(&range));
        self
    }
    /// Remove all entries from this collection.
    ///
    /// Returns `&mut self` for easy chaining.
    pub fn clear(&mut self) -> &mut Self {
        self.ranges.clear();
        self
    }
    /// Return the set union of this collection and another one.
    ///
    /// The time complexity is O(min(<var>R</var>, <var>T</var>) log(<var>R</var> + <var>T</var>)),
    /// where <var>R</var> is the number of ranges in this collection and <var>T</var> is the number
    /// of ranges in `rhs`.
    pub fn union<V: MultiCharRange>(&self, rhs: &V) -> CharCollection {
        let mut result: CharCollection;
        if self.range_count() > rhs.range_count() {
            result = self.clone();
            result.insert(rhs);
        } else {
            result = rhs.into();
            result.insert(self);
        }
        result
    }
    /// Return the set intersection of this collection and another one.
    ///
    /// The time complexity is O(min(<var>R</var>, <var>T</var>) log(<var>R</var> + <var>T</var>)),
    /// where <var>R</var> is the number of ranges in this collection and <var>T</var> is the number
    /// of ranges in `rhs`.
    pub fn intersection<V: MultiCharRange>(&self, rhs: &V) -> CharCollection {
        let mut result: CharCollection;
        if self.range_count() > rhs.range_count() {
            result = self.clone();
            let rhs: CharCollection = rhs.into();
            result.remove(&rhs.complement());
        } else {
            result = rhs.into();
            result.remove(&self.complement());
        }
        result
    }
    /// Return the (non-symmetric) set difference of this collection and another one.
    ///
    /// The time complexity is O(<var>T</var> log(<var>R</var> + <var>T</var>)), where <var>R</var>
    /// is the number of ranges in this collection and <var>T</var> is the number of ranges in
    /// `rhs`.
    pub fn difference<V: MultiCharRange>(&self, rhs: &V) -> CharCollection {
        let mut result: CharCollection = self.clone();
        result.remove(rhs);
        result
    }
    /// Return the set complement of this collection (over the universe of `char`s).
    ///
    /// The time complexity is O(<var>R</var>), where <var>R</var> is the number of ranges in this
    /// collection.
    pub fn complement(&self) -> CharCollection {
        if self.ranges.is_empty() {
            return CharCollection::from(&CharRange::all());
        }
        let mut result_ranges: Vec<CharRange> = Vec::new();
        if self.ranges[0].low != '\u{0}' {
            result_ranges.push(CharRange::open_right('\u{0}', self.ranges[0].low));
        }
        let mut prev_high = self.ranges[0].high;
        for range in &self.ranges[1..] {
            result_ranges.push(CharRange::open(prev_high, range.low));
            prev_high = range.high;
        }
        if prev_high != std::char::MAX {
            result_ranges.push(CharRange::open_left(prev_high, std::char::MAX));
        }
        CharCollection { ranges: result_ranges }
    }
    /// Insert a single `CharRange`.
    ///
    /// Depending on how the new range relates to existing ranges in
    /// the collection, it might be subsumed by an existing range, modify the endpoints of an
    /// existing range, or replace one or more existing ranges.
    fn insert_char_range(&mut self, new_range: &CharRange) {
        if new_range.is_empty() {
            return;
        }
        let lower_existing_range = self.find_containing_range(&new_range.low);
        let upper_existing_range = self.find_containing_range(&new_range.high);
        // Fully enclosed in existing range.
        if lower_existing_range == upper_existing_range && lower_existing_range.is_ok() {
            return;
        }
        let new_low: char;
        let new_high: char;
        let remove_from_idx: usize;
        let remove_to_idx: usize;
        match lower_existing_range {
            Ok((idx, lower_existing_range)) => {
                new_low = lower_existing_range.low;
                remove_from_idx = idx;
            }
            Err(idx) => {
                new_low = new_range.low;
                remove_from_idx = idx;
            }
        }
        match upper_existing_range {
            Ok((idx, higher_existing_range)) => {
                new_high = higher_existing_range.high;
                remove_to_idx = idx + 1;
            }
            Err(idx) => {
                new_high = new_range.high;
                remove_to_idx = idx;
            }
        }
        self.replace_ranges(chars!(new_low..=new_high), remove_from_idx..remove_to_idx);
    }
    /// Remove a single `CharRange`.
    ///
    /// Depending on how the removed range relates to existing ranges in the collection, it might
    /// remove or modify the endpoints of existing ranges.
    fn remove_char_range(&mut self, range_to_remove: &CharRange) {
        if range_to_remove.is_empty() {
            return;
        }
        let lower_existing_range = self.find_containing_range(&range_to_remove.low);
        let upper_existing_range = self.find_containing_range(&range_to_remove.high);
        let mut replacement_ranges: Vec<CharRange> = Vec::new();
        let remove_from_idx: usize;
        let remove_to_idx: usize;
        match lower_existing_range {
            Ok((idx, lower_existing_range)) => {
                if lower_existing_range.low < range_to_remove.low {
                    replacement_ranges
                        .push(CharRange::open_right(lower_existing_range.low, range_to_remove.low));
                }
                remove_from_idx = idx;
            }
            Err(idx) => remove_from_idx = idx,
        }
        match upper_existing_range {
            Ok((idx, higher_existing_range)) => {
                if range_to_remove.high < higher_existing_range.high {
                    replacement_ranges.push(CharRange::open_left(
                        range_to_remove.high,
                        higher_existing_range.high,
                    ));
                }
                remove_to_idx = idx + 1;
            }
            Err(idx) => {
                remove_to_idx = idx;
            }
        }
        self.ranges.splice(remove_from_idx..remove_to_idx, replacement_ranges);
    }
    /// Delete all the existing `CharRange`s that fall within `indices_to_replace` in the vector,
    /// and insert `char_range_to_insert` in their place. If the newly formed range is adjacent to
    /// a kept range on its left or right, coalesce them.
    fn replace_ranges(
        &mut self,
        mut char_range_to_insert: CharRange,
        mut indices_to_replace: Range<usize>,
    ) {
        // If the newly formed range is adjacent to the range on its left, coalesce the two.
        if indices_to_replace.start > 0 {
            let prev_char_range = self.ranges[indices_to_replace.start - 1];
            if are_chars_adjacent(&prev_char_range.high, &char_range_to_insert.low) {
                char_range_to_insert.low = prev_char_range.low;
                indices_to_replace.start -= 1;
            }
        }
        // If the newly formed range is adjacent to the range on its right, coalesce the two.
        if indices_to_replace.end < self.ranges.len() {
            let next_char_range = self.ranges[indices_to_replace.end];
            if are_chars_adjacent(&char_range_to_insert.high, &next_char_range.low) {
                char_range_to_insert.high = next_char_range.high;
                indices_to_replace.end += 1;
            }
        }
        self.ranges.splice(indices_to_replace, vec![char_range_to_insert]);
    }
    fn find_containing_range(&self, query: &char) -> Result<(usize, CharRange), usize> {
        let result = self.ranges.binary_search_by(|range| range.cmp_char(query.clone()));
        match result {
            Ok(index) => Ok((index, self.ranges[index])),
            Err(index) => Err(index),
        }
    }
}
impl MultiCharRange for CharCollection {
    fn iter_ranges<'a>(&'a self) -> Box<dyn Iterator<Item = CharRange> + 'a> {
        Box::new(self.ranges.iter().map(|range| range.clone()))
    }
    fn range_count(&self) -> usize {
        self.ranges.len()
    }
}
impl Hash for CharCollection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ranges.iter().for_each(|range| hash_char_range(range, state));
    }
}
fn hash_char_range<H: Hasher>(range: &CharRange, state: &mut H) {
    range.low.hash(state);
    range.high.hash(state);
}
fn are_chars_adjacent(left: &char, right: &char) -> bool {
    let mut iter: CharIter = CharRange::open_right(left.clone(), right.clone()).iter();
    match iter.next_back() {
        None => false,
        Some(next_right) => left == &next_right,
    }
}
fn format_range(range: &CharRange) -> String {
    format!("{}..={}", range.low, range.high)
}
#[cfg(test)]
mod tests {
    use {
        super::{are_chars_adjacent, CharCollection},
        anyhow::Error,
        std::char,
        unic_char_range::{chars, CharRange},
    };
    #[test]
    fn test_from_sorted_ranges() -> Result<(), Error> {
        let expected = char_collect!('a'..='d', 'g'..='l', 'z');
        let actual = CharCollection::from_sorted_ranges(vec![
            chars!('a'..='d'),
            chars!('g'..='l'),
            chars!('z'..='z'),
        ])?;
        assert_eq!(actual, expected);
        Ok(())
    }
    #[test]
    fn test_from_sorted_ranges_out_of_order() {
        assert!(CharCollection::from_sorted_ranges(vec![
            chars!('g'..='l'),
            chars!('a'..='d'),
            chars!('z'..='z'),
        ])
        .is_err());
    }
    #[test]
    fn test_from_sorted_ranges_overlap() {
        assert!(CharCollection::from_sorted_ranges(vec![
            chars!('a'..='d'),
            chars!('c'..='l'),
            chars!('z'..='z'),
        ])
        .is_err());
    }
    #[test]
    fn test_from_sorted_ranges_adjacent() {
        assert!(
            CharCollection::from_sorted_ranges(vec![chars!('a'..='d'), chars!('e'..='g')]).is_err()
        );
    }
    #[test]
    fn test_from_sorted_chars() -> Result<(), Error> {
        let chars = vec!['a', 'b', 'c', 'd', 'g', 'h', 'i', 'j', 'k', 'l', 'z'];
        let expected = char_collect!('a'..='d', 'g'..='l', 'z');
        let actual = CharCollection::from_sorted_chars(chars)?;
        assert_eq!(actual, expected);
        Ok(())
    }
    #[test]
    fn test_from_sorted_chars_out_of_order() {
        let chars = vec!['a', 'b', 'c', 'd', 'g', 'h', 'i', 'j', 'k', 'l', 'e'];
        assert!(CharCollection::from_sorted_chars(chars).is_err());
    }
    #[test]
    fn test_find_containing_range() {
        let collection = char_collect!({ ('a'..='d') + ('g'..='j') + ('l'..='o') + 'z' });
        assert_eq!(collection.find_containing_range(&'0'), Err(0));
        assert_eq!(collection.find_containing_range(&'c'), Ok((0, chars!('a'..='d'))));
        assert_eq!(collection.find_containing_range(&'e'), Err(1));
    }
    #[test]
    fn test_insert_initial() {
        let collection = char_collect!('a'..='d');
        assert_eq!(collection.ranges, vec![chars!('a'..='d')])
    }
    #[test]
    fn test_insert_exact_match() {
        let mut collection = char_collect!('a'..='d', 'g'..='l');
        collection += 'a'..='d';
        assert_eq!(collection.ranges, vec![chars!('a'..='d'), chars!('g'..='l')]);
    }
    #[test]
    fn test_insert_non_overlapping_sorted() {
        let collection = char_collect!('a'..='d', 'g'..='j', 'l'..='o');
        assert_eq!(
            collection.ranges,
            vec![chars!('a'..='d'), chars!('g'..='j'), chars!('l'..='o')]
        );
    }
    #[test]
    fn test_insert_non_overlapping_unsorted() {
        let collection = char_collect!('l'..='o', 'a'..='d', 'l'..='o', 'a'..='d', 'g'..='j');
        assert_eq!(
            collection.ranges,
            vec![chars!('a'..='d'), chars!('g'..='j'), chars!('l'..='o')]
        );
    }
    #[test]
    fn test_insert_overlapping_all_existent() {
        let mut collection = char_collect!('l'..='o', 'a'..='d');
        collection += 'a'..='o';
        assert_eq!(collection.ranges, vec![chars!('a'..='o')]);
    }
    #[test]
    fn test_insert_overlapping_some_existent() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection += 'i'..='n';
        assert_eq!(
            collection.ranges,
            vec![chars!('c'..='e'), chars!('i'..='n'), chars!('p'..='s')]
        );
    }
    #[test]
    fn test_insert_overlapping_with_intersections() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection += 'd'..='k';
        assert_eq!(collection.ranges, vec![chars!('c'..='m'), chars!('p'..='s')]);
    }
    #[test]
    fn test_insert_coalesce_adjacent_ranges() {
        let mut collection = char_collect!('a'..='c', 'j'..='m');
        collection += 'd'..='i';
        assert_eq!(collection.ranges, vec![chars!('a'..='m')]);
    }
    #[test]
    fn test_append() -> Result<(), Error> {
        let mut collection = char_collect!('a'..='c');
        collection.append('d')?.append('g')?.append('h')?.append('i')?.append('z')?;
        assert_eq!(collection, char_collect!('a'..='d', 'g'..='i', 'z'));
        Ok(())
    }
    #[test]
    fn test_append_out_of_order() -> Result<(), Error> {
        let mut collection = char_collect!('a'..='c');
        assert!(collection
            .append('d')?
            .append('g')?
            .append('h')?
            .append('i')?
            .append('e')
            .is_err());
        Ok(())
    }
    #[test]
    fn test_append_range() -> Result<(), Error> {
        let mut collection = char_collect!('a'..='c');
        collection.append_range(chars!('g'..='i'))?.append_range(chars!('j'..='m'))?;
        assert_eq!(collection, char_collect!('a'..='c', 'g'..='m'));
        Ok(())
    }
    #[test]
    fn test_append_range_out_of_order() -> Result<(), Error> {
        let mut collection = char_collect!('a'..='c');
        assert!(collection
            .append_range(chars!('g'..='i'))?
            .append_range(chars!('j'..='m'))?
            .append_range(chars!('k'..='m'))
            .is_err());
        Ok(())
    }
    #[test]
    fn test_remove_exact_range() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection -= 'j'..='m';
        assert_eq!(collection.ranges, vec![chars!('c'..='e'), chars!['p'..='s']]);
    }
    #[test]
    fn test_remove_overlapping_all_existent() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection -= 'c'..='s';
        assert_eq!(collection.ranges, vec![]);
    }
    #[test]
    fn test_remove_overlapping_all_existent_superset() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection -= 'a'..='z';
        assert_eq!(collection.ranges, vec![]);
    }
    #[test]
    fn test_remove_one_subrange() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection -= 'k'..='l';
        assert_eq!(
            collection.ranges,
            vec![chars!('c'..='e'), chars!('j'..='j'), chars!('m'..='m'), chars!('p'..='s')]
        );
    }
    #[test]
    fn test_remove_intersection() {
        let mut collection = char_collect!('c'..='e', 'j'..='m', 'p'..='s');
        collection -= 'd'..='q';
        assert_eq!(collection.ranges, vec![chars!('c'..='c'), chars!('r'..='s')]);
    }
    #[test]
    fn test_complement_simple() {
        let collection = char_collect!(0x10..=0x50, 0x70..=0x70, 0x99..=0x640);
        assert_eq!(
            collection.complement(),
            char_collect!(0x00..=0x0F, 0x51..=0x6F, 0x71..=0x98, 0x641..=(char::MAX as u32))
        );
    }
    #[test]
    fn test_complement_all() {
        let collection = char_collect!(CharRange::all());
        assert_eq!(collection.complement(), char_collect!());
    }
    #[test]
    fn test_complement_none() {
        let collection = char_collect!();
        assert_eq!(collection.complement(), char_collect!(CharRange::all()));
    }
    #[test]
    fn test_complement_includes_min_and_max() {
        let collection = char_collect!(0x0..=0x10, 0x40..=0x50, 0xCCCC..=(char::MAX as u32));
        assert_eq!(collection.complement(), char_collect!(0x11..=0x3F, 0x51..=0xCCCB));
    }
    #[test]
    fn test_union() {
        let collection_a = char_collect!('a'..='g', 'm'..='z', 'B'..='R');
        let collection_b = char_collect!('e'..='q', 'W'..='Y');
        let expected = char_collect!('a'..='z', 'B'..='R', 'W'..='Y');
        assert_eq!(collection_a.union(&collection_b), expected);
        assert_eq!(collection_b.union(&collection_a), expected);
    }
    #[test]
    fn test_intersection() {
        let collection_a = char_collect!('a'..='g', 'm'..='z');
        let collection_b = char_collect!('e'..='q');
        let expected = char_collect!('e'..='g', 'm'..='q');
        assert_eq!(collection_a.intersection(&collection_b), expected);
        assert_eq!(collection_b.intersection(&collection_a), expected);
    }
    #[test]
    fn test_macro_expressions() {
        use unicode_blocks::UnicodeBlockId::Arabic;
        let collection =
            char_collect!({ ('c'..='e') + ('f'..='h') - ('a'..='d') + Arabic + (0x5..=0x42) });
        assert_eq!(collection, char_collect!(0x5..=0x42, 'e'..='h', Arabic));
    }
    #[test]
    fn test_iter() {
        let collection = char_collect!('a'..='c', 'j'..='l', 'x'..='z');
        let v = collection.iter().collect::<Vec<char>>();
        assert_eq!(v, vec!['a', 'b', 'c', 'j', 'k', 'l', 'x', 'y', 'z']);
    }
    #[test]
    fn test_are_chars_adjacent() {
        assert!(are_chars_adjacent(&'a', &'b'));
        assert!(!are_chars_adjacent(&'b', &'a'));
        assert!(!are_chars_adjacent(&'a', &'c'));
    }
}