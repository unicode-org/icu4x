use crate::trie::*;

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

#[derive(Clone)]
pub struct UCharsTrie {
    pos_: Option<usize>,
    root_: usize,
    remaining_match_length_: Option<usize>,
}

impl Trie for UCharsTrie {
    // Traverses the trie from the initial state for this input char.
    // Equivalent to reset() then next(inUnit)
    fn first(&mut self, trie_data: &[u8], c: i32) -> TrieResult {
        let uchars = unsafe { &*(trie_data as *const [u8] as *const [u16]) };
        self.remaining_match_length_ = None;
        self.next_impl(uchars, self.root_, c as u16)
    }

    // Traverses the trie from the current state for this input char.
    fn next(&mut self, trie_data: &[u8], c: i32) -> TrieResult {
        let uchars = unsafe { &*(trie_data as *const [u8] as *const [u16]) };
        if self.pos_.is_none() {
            return TrieResult::NoMatch;
        }
        let in_byte = c as u16;
        let mut pos = self.pos_.unwrap();
        if let Some(length) = self.remaining_match_length_ {
            // Remaining part of a linear-match node
            if in_byte == uchars[pos] {
                pos += 1;
                self.pos_ = Some(pos);
                if length == 0 {
                    self.remaining_match_length_ = None;
                    let node = uchars[pos];
                    if node >= MIN_VALUE_LEAD {
                        return Self::value_result(node);
                    }
                } else {
                    self.remaining_match_length_ = Some(length);
                }
                return TrieResult::NoValue;
            }
            self.stop();
            TrieResult::NoMatch
        } else {
            self.next_impl(uchars, pos, in_byte)
        }
    }

    fn box_clone(&self) -> Box<dyn Trie> {
        Box::new(UCharsTrie {
            pos_: self.pos_,
            root_: self.root_,
            remaining_match_length_: self.remaining_match_length_,
        })
    }
}

impl UCharsTrie {
    pub fn new(offset: usize) -> Self {
        Self {
            pos_: Some(offset / 2),
            root_: offset / 2,
            remaining_match_length_: None,
        }
    }

    fn branch_next(
        &mut self,
        uchars: &[u16],
        pos: usize,
        length: usize,
        in_unit: u16,
    ) -> TrieResult {
        let mut pos = pos;
        let mut length = length;
        if length == 0 {
            length = uchars[pos] as usize;
            pos += 1;
        }
        length += 1;

        // The length of the branch is the number of units to select from.
        // The data structure encodes a binary search.
        while length > MAX_BRANCH_LINEAR_SUB_NODE_LENGTH {
            if in_unit < uchars[pos] {
                length >>= 1;
                pos = self.jump_by_delta(uchars, pos + 1);
            } else {
                length = length - (length >> 1);
                pos = self.skip_delta(uchars, pos + 1);
            }
        }
        // Drop down to linear search for the last few bytes.
        // length>=2 because the loop body above sees length>kMaxBranchLinearSubNodeLength>=3
        // and divides length by 2.
        loop {
            if in_unit == uchars[pos] {
                pos += 1;
                let mut node = uchars[pos];
                if node & VALUE_IS_FINAL != 0 {
                    // Leave the final value for getValue() to read.
                    self.pos_ = Some(pos);
                    return TrieResult::FinalValue;
                }
                // Use the non-final value as the jump delta.
                pos += 1;

                if node < MIN_TWO_UNIT_VALUE_LEAD {
                    pos += node as usize;
                } else if node < THREE_UNIT_VALUE_LEAD {
                    pos += (((node - MIN_TWO_UNIT_VALUE_LEAD) as u32) << 16) as usize
                        | uchars[pos] as usize;
                    pos += 1;
                } else {
                    pos += (uchars[pos] as usize) << 16 | uchars[pos + 1] as usize;
                    pos += 2;
                }
                node = uchars[pos];
                self.pos_ = Some(pos);

                if node >= MIN_VALUE_LEAD {
                    return UCharsTrie::value_result(node);
                }
                return TrieResult::NoValue;
            }
            length -= 1;
            pos = self.skip_value(uchars, pos + 1);
            if length <= 1 {
                break;
            }
        }

        if in_unit == uchars[pos] {
            pos += 1;
            self.pos_ = Some(pos);
            let node = uchars[pos];
            if node >= MIN_VALUE_LEAD {
                return Self::value_result(node);
            }
            TrieResult::NoValue
        } else {
            self.stop();
            TrieResult::NoMatch
        }
    }

    fn next_impl(&mut self, uchars: &[u16], pos: usize, in_unit: u16) -> TrieResult {
        let mut node = uchars[pos];
        let mut pos = pos + 1;
        loop {
            if node < MIN_LINEAR_MATCH {
                return self.branch_next(uchars, pos, node as usize, in_unit);
            } else if node < MIN_VALUE_LEAD {
                // Match the first of length+1 units.
                let length = node - MIN_LINEAR_MATCH;
                if in_unit == uchars[pos] {
                    pos += 1;
                    if length == 0 {
                        self.remaining_match_length_ = None;
                        self.pos_ = Some(pos);
                        node = uchars[pos];
                        if node >= MIN_VALUE_LEAD {
                            return Self::value_result(node);
                        }
                        return TrieResult::NoValue;
                    }
                    self.remaining_match_length_ = Some(length as usize - 1);
                    self.pos_ = Some(pos);
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
        self.pos_ = None;
    }

    fn jump_by_delta(&self, uchars: &[u16], pos: usize) -> usize {
        let delta = uchars[pos];
        if delta < MIN_TWO_UNIT_DELTA_LEAD {
            // nothing to do
            pos + 1 + delta as usize
        } else if delta == THREE_UNIT_DELTA_LEAD {
            let delta = ((uchars[pos + 1] as usize) << 16) | (uchars[pos + 2] as usize);
            pos + delta + 3
        } else {
            let delta =
                ((delta - MIN_TWO_UNIT_DELTA_LEAD) as usize) << 16 | (uchars[pos + 1] as usize);
            pos + delta + 2
        }
    }

    fn skip_value(&self, uchars: &[u16], pos: usize) -> usize {
        let lead_byte = uchars[pos];
        skip_value(pos + 1, lead_byte & 0x7fff)
    }

    fn skip_delta(&self, uchars: &[u16], pos: usize) -> usize {
        let delta = uchars[pos];
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
}
