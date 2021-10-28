// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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

pub struct UCharsTrieIterator<'a> {
    trie: &'a [u16],
    pos: Option<usize>,
    remaining_match_length: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrieResult {
    // The input unit(s) did not continue a matching string.
    // Once current()/next() return TrieResult::NoMatch,
    // all further calls to current()/next() will also return TrieResult::NoMatch,
    // until the trie is reset to its original state or to a saved state.
    NoMatch,
    // The input unit(s) continued a matching string
    // but there is no value for the string so far.
    // (It is a prefix of a longer string.)
    NoValue,
    // The input unit(s) continued a matching string
    // and there is a value for the string so far.
    // No further input byte/unit can continue a matching string.
    FinalValue,
    // The input unit(s) continued a matching string
    // and there is a value for the string so far.
    // Another input byte/unit can continue a matching string.
    Intermediate,
}

impl<'a> UCharsTrieIterator<'a> {
    pub fn new(trie: &'a [u16], offset: usize) -> Self {
        Self {
            trie,
            pos: Some(offset / 2),
            remaining_match_length: None,
        }
    }

    // Traverses the trie from the current state for this input char.
    pub fn next(&mut self, c: i32) -> TrieResult {
        if self.pos.is_none() {
            return TrieResult::NoMatch;
        }
        let in_byte = c as u16;
        let mut pos = self.pos.unwrap();
        if let Some(length) = self.remaining_match_length {
            // Remaining part of a linear-match node
            if in_byte == self.trie[pos] {
                pos += 1;
                self.pos = Some(pos);
                if length == 0 {
                    self.remaining_match_length = None;
                    let node = self.trie[pos];
                    if node >= MIN_VALUE_LEAD {
                        return Self::value_result(node);
                    }
                } else {
                    self.remaining_match_length = Some(length);
                }
                return TrieResult::NoValue;
            }
            self.stop();
            TrieResult::NoMatch
        } else {
            self.next_impl(pos, in_byte)
        }
    }

    fn branch_next(&mut self, pos: usize, length: usize, in_unit: u16) -> TrieResult {
        let mut pos = pos;
        let mut length = length;
        if length == 0 {
            length = self.trie[pos] as usize;
            pos += 1;
        }
        length += 1;

        // The length of the branch is the number of units to select from.
        // The data structure encodes a binary search.
        while length > MAX_BRANCH_LINEAR_SUB_NODE_LENGTH {
            if in_unit < self.trie[pos] {
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
            if in_unit == self.trie[pos] {
                pos += 1;
                let mut node = self.trie[pos];
                if node & VALUE_IS_FINAL != 0 {
                    // Leave the final value for get_value() to read.
                    self.pos = Some(pos);
                    return TrieResult::FinalValue;
                }
                // Use the non-final value as the jump delta.
                pos += 1;

                if node < MIN_TWO_UNIT_VALUE_LEAD {
                    pos += node as usize;
                } else if node < THREE_UNIT_VALUE_LEAD {
                    pos += (((node - MIN_TWO_UNIT_VALUE_LEAD) as u32) << 16) as usize
                        | self.trie[pos] as usize;
                    pos += 1;
                } else {
                    pos += (self.trie[pos] as usize) << 16 | self.trie[pos + 1] as usize;
                    pos += 2;
                }
                node = self.trie[pos];
                self.pos = Some(pos);

                if node >= MIN_VALUE_LEAD {
                    return Self::value_result(node);
                }
                return TrieResult::NoValue;
            }
            length -= 1;
            pos = self.skip_value(pos + 1);
            if length <= 1 {
                break;
            }
        }

        if in_unit == self.trie[pos] {
            pos += 1;
            self.pos = Some(pos);
            let node = self.trie[pos];
            if node >= MIN_VALUE_LEAD {
                return Self::value_result(node);
            }
            TrieResult::NoValue
        } else {
            self.stop();
            TrieResult::NoMatch
        }
    }

    fn next_impl(&mut self, pos: usize, in_unit: u16) -> TrieResult {
        let mut node = self.trie[pos];
        let mut pos = pos + 1;
        loop {
            if node < MIN_LINEAR_MATCH {
                return self.branch_next(pos, node as usize, in_unit);
            } else if node < MIN_VALUE_LEAD {
                // Match the first of length+1 units.
                let length = node - MIN_LINEAR_MATCH;
                if in_unit == self.trie[pos] {
                    pos += 1;
                    if length == 0 {
                        self.remaining_match_length = None;
                        self.pos = Some(pos);
                        node = self.trie[pos];
                        if node >= MIN_VALUE_LEAD {
                            return Self::value_result(node);
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
                pos = skip_value(pos, node);
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
        let delta = self.trie[pos];
        if delta < MIN_TWO_UNIT_DELTA_LEAD {
            // nothing to do
            pos + 1 + delta as usize
        } else if delta == THREE_UNIT_DELTA_LEAD {
            let delta = ((self.trie[pos + 1] as usize) << 16) | (self.trie[pos + 2] as usize);
            pos + delta + 3
        } else {
            let delta =
                ((delta - MIN_TWO_UNIT_DELTA_LEAD) as usize) << 16 | (self.trie[pos + 1] as usize);
            pos + delta + 2
        }
    }

    fn skip_value(&self, pos: usize) -> usize {
        let lead_byte = self.trie[pos];
        skip_value(pos + 1, lead_byte & 0x7fff)
    }

    fn skip_delta(&self, pos: usize) -> usize {
        let delta = self.trie[pos];
        if delta < MIN_TWO_UNIT_DELTA_LEAD {
            pos + 1
        } else if delta == THREE_UNIT_DELTA_LEAD {
            pos + 3
        } else {
            pos + 2
        }
    }

    fn value_result(node: u16) -> TrieResult {
        let node = node & VALUE_IS_FINAL;
        match node {
            VALUE_IS_FINAL => TrieResult::FinalValue,
            _ => TrieResult::Intermediate,
        }
    }

    pub fn get_value(&self) -> Option<i32> {
        if self.pos.is_none() {
            return None;
        }
        let mut pos = self.pos.unwrap();
        let lead_unit = self.trie[pos];
        pos += 1;
        if lead_unit & VALUE_IS_FINAL == VALUE_IS_FINAL {
            Some(self.read_value(pos, lead_unit & 0x7fff))
        } else {
            Some(self.read_node_value(pos, lead_unit))
        }
    }

    fn read_value(&self, pos: usize, lead_unit: u16) -> i32 {
        if lead_unit < MIN_TWO_UNIT_VALUE_LEAD {
            lead_unit.into()
        } else if lead_unit < THREE_UNIT_VALUE_LEAD {
            ((lead_unit - MIN_TWO_UNIT_VALUE_LEAD) as i32) << 16 | self.trie[pos] as i32
        } else {
            ((self.trie[pos] as i32) << 16) | self.trie[pos + 1] as i32
        }
    }

    fn read_node_value(&self, pos: usize, lead_unit: u16) -> i32 {
        if lead_unit < MIN_TWO_UNIT_VALUE_LEAD {
            ((lead_unit >> 6) - 1).into()
        } else if lead_unit < THREE_UNIT_VALUE_LEAD {
            ((((lead_unit & 0x7fc0) - MIN_TWO_UNIT_NODE_VALUE_LEAD) as i32) << 10)
                | self.trie[pos] as i32
        } else {
            ((self.trie[pos] as i32) << 16) | self.trie[pos + 1] as i32
        }
    }
}
