// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use zerovec::ule::AsULE;
use zerovec::ZeroVec;

// Match-node lead unit values, after masking off intermediate-value bits:

// 00..0f: Branch node. If node!=0 then the length is node+1, otherwise
// the length is one more than the next byte.

// For a branch sub-node with at most this many entries, we drop down
// to a linear search.
const MAX_BRANCH_LINEAR_SUB_NODE_LENGTH: usize = 5;

// 0030..003f: Linear-match node, match 1..16 units and continue reading the next node.
const MIN_LINEAR_MATCH: u16 = 0x30;
const MAX_LINEAR_MATCH_LENGTH: u16 = 0x10;

// Match-node lead unit bits 14..6 for the optional intermediate value.
// If these bits are 0, then there is no intermediate value.
// Otherwise, see the *NodeValue* constants below.
const MIN_VALUE_LEAD: u16 = MIN_LINEAR_MATCH + MAX_LINEAR_MATCH_LENGTH; // 0x40
const NODE_TYPE_MASK: u16 = MIN_VALUE_LEAD - 1; // 0x003f

// A final-value node has bit 15 set.
const VALUE_IS_FINAL: u16 = 0x8000;

// Compact value: After testing bit 0, shift right by 15 and then use the following thresholds.
const MAX_ONE_UNIT_VALUE: u16 = 0x3fff;

const MIN_TWO_UNIT_VALUE_LEAD: u16 = MAX_ONE_UNIT_VALUE + 1; // 0x4000

const MAX_ONE_UNIT_NODE_VALUE: u16 = 0xff;

const MIN_TWO_UNIT_NODE_VALUE_LEAD: u16 = MIN_VALUE_LEAD + ((MAX_ONE_UNIT_NODE_VALUE + 1) << 6); // 0x4040

const THREE_UNIT_NODE_VALUE_LEAD: u16 = 0x7fc0;

const THREE_UNIT_VALUE_LEAD: u16 = 0x7fff;

// Compact delta integers.
const MAX_ONE_UNIT_DELTA: u16 = 0xfbff;
const MIN_TWO_UNIT_DELTA_LEAD: u16 = MAX_ONE_UNIT_DELTA + 1; // 0xfc00
const THREE_UNIT_DELTA_LEAD: u16 = 0xffff;

fn skip_value(pos: usize, lead: u16) -> usize {
    if lead < MIN_TWO_UNIT_VALUE_LEAD {
        pos
    } else if lead < THREE_UNIT_VALUE_LEAD {
        pos + 1
    } else {
        pos + 2
    }
}

fn skip_node_value(pos: usize, lead: u16) -> usize {
    if lead < MIN_TWO_UNIT_NODE_VALUE_LEAD {
        pos
    } else if lead < THREE_UNIT_NODE_VALUE_LEAD {
        pos + 1
    } else {
        pos + 2
    }
}

/// This struct represents a de-serialized Char16Trie that was exported from
/// ICU binary data.
///
/// The trie consists of a series of char16_t-serialized nodes for incremental
/// Unicode string/char16_t sequence matching. (char16_t=16-bit unsigned integer)
/// The root node is at the beginning of the trie data.
///
/// Types of nodes are distinguished by their node lead unit ranges.
/// After each node, except a final-value node, another node follows to
/// encode match values or continue matching further units.
///
/// Node types:
///  - Final-value node: Stores a 32-bit integer in a compact, variable-length format.
///    The value is for the string/char16_t sequence so far.
///  - Match node, optionally with an intermediate value in a different compact format.
///    The value, if present, is for the string/char16_t sequence so far.
///
///  Aside from the value, which uses the node lead unit's high bits:
///
///  - Linear-match node: Matches a number of units.
///  - Branch node: Branches to other nodes according to the current input unit.
///    The node unit is the length of the branch (number of units to select from)
///    minus 1. It is followed by a sub-node:
///    - If the length is at most MAX_BRANCH_LINEAR_SUB_NODE_LENGTH, then
///      there are length-1 (key, value) pairs and then one more comparison unit.
///      If one of the key units matches, then the value is either a final value for
///      the string so far, or a "jump" delta to the next node.
///      If the last unit matches, then matching continues with the next node.
///      (Values have the same encoding as final-value nodes.)
///    - If the length is greater than MAX_BRANCH_LINEAR_SUB_NODE_LENGTH, then
///      there is one unit and one "jump" delta.
///      If the input unit is less than the sub-node unit, then "jump" by delta to
///      the next sub-node which will have a length of length/2.
///      (The delta has its own compact encoding.)
///      Otherwise, skip the "jump" delta to the next sub-node
///      which will have a length of length-length/2.
///
/// For more information:
/// - [ICU4C UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)
/// - [ICU4J CharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html) API.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct Char16Trie<'data> {
    /// An array of u16 containing the trie data.
    #[serde(borrow)]
    pub data: ZeroVec<'data, u16>,
}

/// This struct represents an iterator over a Char16Trie.
pub struct Char16TrieIterator<'a> {
    /// A reference to the Char16Trie data to iterate over.
    trie: &'a [<u16 as zerovec::ule::AsULE>::ULE],
    /// Index of next trie unit to read, or None if there are no more matches.
    pos: Option<usize>,
    /// Remaining length of a linear-match node, minus 1, or None if not in
    /// such a node.
    remaining_match_length: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrieResult {
    /// The input unit(s) did not continue a matching string.
    /// Once next() return TrieResult::NoMatch, all further calls to next()
    /// will also return TrieResult::NoMatch.
    NoMatch,
    /// The input unit(s) matched a string but there is no value for the string
    /// so far.  (It is a prefix of a longer string.)
    NoValue,
    /// The input unit(s) continued a matching string and there is a value for
    /// the string so far. No further input byte/unit can continue a matching
    /// string.
    FinalValue(i32),
    /// The input unit(s) continued a matching string and there is a value for
    /// the string so far.  Another input byte/unit can continue a matching
    /// string.
    Intermediate(i32),
}

impl<'a> Char16TrieIterator<'a> {
    /// Returns a new [`Char16Iterator`] backed by borrowed data for the `trie` array
    pub fn new(trie: &'a [<u16 as zerovec::ule::AsULE>::ULE]) -> Self {
        Self {
            trie,
            pos: Some(0),
            remaining_match_length: None,
        }
    }

    /// Traverses the trie from the current state for this input char.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_char16trie::char16trie::{Char16Trie, Char16TrieIterator, TrieResult};
    /// use zerovec::ZeroVec;
    ///
    /// // A Char16Trie containing the ASCII characters 'a' and 'b'.
    /// let trie_data = vec![48, 97, 176, 98, 32868];
    /// let trie = Char16Trie {
    ///     data: ZeroVec::from_slice(trie_data.as_slice()),
    /// };
    ///
    /// let mut itor = Char16TrieIterator::new(trie.data.as_slice());
    /// let res = itor.next('a' as i32);
    /// assert_eq!(res, TrieResult::Intermediate(1));
    /// let res = itor.next('b' as i32);
    /// assert_eq!(res, TrieResult::FinalValue(100));
    /// let res = itor.next('c' as i32);
    /// assert_eq!(res, TrieResult::NoMatch);
    /// ```
    pub fn next(&mut self, c: i32) -> TrieResult {
        if self.pos.is_none() {
            return TrieResult::NoMatch;
        }
        let mut pos = self.pos.unwrap();
        if let Some(length) = self.remaining_match_length {
            // Remaining part of a linear-match node
            if c == self.get(pos).into() {
                pos += 1;
                self.pos = Some(pos);
                if length == 0 {
                    self.remaining_match_length = None;
                    let node = self.get(pos);
                    if node >= MIN_VALUE_LEAD {
                        return self.value_result(pos);
                    }
                } else {
                    self.remaining_match_length = Some(length - 1);
                }
                return TrieResult::NoValue;
            }
            self.stop();
            TrieResult::NoMatch
        } else {
            self.next_impl(pos, c)
        }
    }

    fn get(&self, pos: usize) -> u16 {
        u16::from_unaligned(self.trie[pos])
    }

    fn branch_next(&mut self, pos: usize, length: usize, in_unit: i32) -> TrieResult {
        let mut pos = pos;
        let mut length = length;
        if length == 0 {
            length = self.get(pos) as usize;
            pos += 1;
        }
        length += 1;

        // The length of the branch is the number of units to select from.
        // The data structure encodes a binary search.
        while length > MAX_BRANCH_LINEAR_SUB_NODE_LENGTH {
            if in_unit < self.get(pos).into() {
                length >>= 1;
                pos = self.jump_by_delta(pos + 1);
            } else {
                length = length - (length >> 1);
                pos = self.skip_delta(pos + 1);
            }
        }
        // Drop down to linear search for the last few bytes.
        // length>=2 because the loop body above sees length>kMaxBranchLinearSubNodeLength>=3
        // and divides length by 2.
        loop {
            if in_unit == self.get(pos).into() {
                pos += 1;
                let mut node = self.get(pos);
                if node & VALUE_IS_FINAL != 0 {
                    self.pos = Some(pos);
                    return self.value_result(pos);
                }
                // Use the non-final value as the jump delta.
                pos += 1;

                if node < MIN_TWO_UNIT_VALUE_LEAD {
                    pos += node as usize;
                } else if node < THREE_UNIT_VALUE_LEAD {
                    pos += (((node - MIN_TWO_UNIT_VALUE_LEAD) as u32) << 16) as usize
                        | self.get(pos) as usize;
                    pos += 1;
                } else {
                    pos += (self.get(pos) as usize) << 16 | self.get(pos + 1) as usize;
                    pos += 2;
                }
                node = self.get(pos);
                self.pos = Some(pos);

                if node >= MIN_VALUE_LEAD {
                    return self.value_result(pos);
                }
                return TrieResult::NoValue;
            }
            length -= 1;
            pos = self.skip_value(pos + 1);
            if length <= 1 {
                break;
            }
        }

        if in_unit == self.get(pos).into() {
            pos += 1;
            self.pos = Some(pos);
            let node = self.get(pos);
            if node >= MIN_VALUE_LEAD {
                return self.value_result(pos);
            }
            TrieResult::NoValue
        } else {
            self.stop();
            TrieResult::NoMatch
        }
    }

    fn next_impl(&mut self, pos: usize, in_unit: i32) -> TrieResult {
        let mut node = self.get(pos);
        let mut pos = pos + 1;
        loop {
            if node < MIN_LINEAR_MATCH {
                return self.branch_next(pos, node as usize, in_unit);
            } else if node < MIN_VALUE_LEAD {
                // Match the first of length+1 units.
                let length = node - MIN_LINEAR_MATCH;
                if in_unit == self.get(pos).into() {
                    pos += 1;
                    if length == 0 {
                        self.remaining_match_length = None;
                        self.pos = Some(pos);
                        node = self.get(pos);
                        if node >= MIN_VALUE_LEAD {
                            return self.value_result(pos);
                        }
                        return TrieResult::NoValue;
                    }
                    self.remaining_match_length = Some(length as usize - 1);
                    self.pos = Some(pos);
                    return TrieResult::NoValue;
                }
                // No match
                break;
            } else if (node & VALUE_IS_FINAL) != 0 {
                // No further matching units.
                break;
            } else {
                // Skip intermediate value.
                pos = skip_node_value(pos, node);
                node &= NODE_TYPE_MASK;
            }
        }
        self.stop();
        TrieResult::NoMatch
    }

    fn stop(&mut self) {
        self.pos = None;
    }

    fn jump_by_delta(&self, pos: usize) -> usize {
        let delta = self.get(pos);
        if delta < MIN_TWO_UNIT_DELTA_LEAD {
            // nothing to do
            pos + 1 + delta as usize
        } else if delta == THREE_UNIT_DELTA_LEAD {
            let delta = ((self.get(pos + 1) as usize) << 16) | (self.get(pos + 2) as usize);
            pos + delta + 3
        } else {
            let delta =
                ((delta - MIN_TWO_UNIT_DELTA_LEAD) as usize) << 16 | (self.get(pos + 1) as usize);
            pos + delta + 2
        }
    }

    fn skip_value(&self, pos: usize) -> usize {
        let lead_byte = self.get(pos);
        skip_value(pos + 1, lead_byte & 0x7fff)
    }

    fn skip_delta(&self, pos: usize) -> usize {
        let delta = self.get(pos);
        if delta < MIN_TWO_UNIT_DELTA_LEAD {
            pos + 1
        } else if delta == THREE_UNIT_DELTA_LEAD {
            pos + 3
        } else {
            pos + 2
        }
    }

    fn value_result(&self, pos: usize) -> TrieResult {
        let node = self.get(pos) & VALUE_IS_FINAL;
        let value = self.get_value(pos);
        match node {
            VALUE_IS_FINAL => TrieResult::FinalValue(value),
            _ => TrieResult::Intermediate(value),
        }
    }

    pub fn get_value(&self, pos: usize) -> i32 {
        let lead_unit = self.get(pos);
        if lead_unit & VALUE_IS_FINAL == VALUE_IS_FINAL {
            self.read_value(pos + 1, lead_unit & 0x7fff)
        } else {
            self.read_node_value(pos + 1, lead_unit)
        }
    }

    fn read_value(&self, pos: usize, lead_unit: u16) -> i32 {
        if lead_unit < MIN_TWO_UNIT_VALUE_LEAD {
            lead_unit.into()
        } else if lead_unit < THREE_UNIT_VALUE_LEAD {
            ((lead_unit - MIN_TWO_UNIT_VALUE_LEAD) as i32) << 16 | self.get(pos) as i32
        } else {
            (self.get(pos) as i32) << 16 | self.get(pos + 1) as i32
        }
    }

    fn read_node_value(&self, pos: usize, lead_unit: u16) -> i32 {
        if lead_unit < (MIN_TWO_UNIT_NODE_VALUE_LEAD) {
            ((lead_unit >> 6) - 1).into()
        } else if lead_unit < THREE_UNIT_NODE_VALUE_LEAD {
            (((lead_unit & 0x7fc0) - MIN_TWO_UNIT_NODE_VALUE_LEAD) as i32) << 10
                | self.get(pos) as i32
        } else {
            (self.get(pos) as i32) << 16 | self.get(pos + 1) as i32
        }
    }
}
