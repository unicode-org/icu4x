// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::bytestrie::BytesTrie;
use crate::error::Error;
use crate::impl_const::*;
use zerovec::map::ZeroVecLike;
use zerovec::ZeroVec;

pub struct BytesTrieElement<'a> {
    string: &'a [u8],
    value: i32,
}

impl<'a> BytesTrieElement<'a> {
    fn new(string: &'a str, value: i32) -> BytesTrieElement<'a> {
        BytesTrieElement {
            string: string.as_bytes(),
            value: value,
        }
    }
}

pub struct BytesTrieBuilder<'a> {
    elements: Vec<BytesTrieElement<'a>>,
    bytes: zerovec::ZeroVec<'a, u8>,
}

pub trait TrieBuilder {
    fn write(&mut self, byte: u8) -> usize;
    fn write_many(&mut self, bytes: &[u8]) -> usize;
}

pub enum Node {
    FinalValueTrieNode,
    ValueNode,
    LinearMatchNode,
    BranchNode,
}

pub enum BytesTrieOption {
    BuildFast,
    BuildSmall,
}

impl<'a> BytesTrieBuilder<'a> {
    const MAX_ONE_BYTE_VALUE: i32 = 0x40;
    const MIN_ONE_BYTE_VALUE_LEAD: i32 = 0x7e;

    fn new() -> Result<BytesTrieBuilder<'a>, Error> {
        Err(Error::FromDeserialized {
            reason: "Not implemented yet",
        })
    }

    fn add(&mut self, s: &'a str, value: i32) -> &mut BytesTrieBuilder<'a> {
        self.elements.push(BytesTrieElement::new(s, value));
        self
    }

    pub async fn build(mut self, options: &BytesTrieOption) -> Result<BytesTrie<'a>, Error> {
        let bytes = self.build_bytes(options)?;
        Result::Ok(BytesTrie { bytes })
    }

    pub fn build_bytes(&mut self, options: &BytesTrieOption) -> Result<ZeroVec<'a, u8>, Error> {
        if self.elements.is_empty() {
            return Err(Error::FromDeserialized {
                reason: "Illegal argument. BytesTrie must be non-empty",
            });
        }

        self.elements
            .sort_unstable_by(|a, b| b.string.cmp(&a.string));
        //Duplicate strings are not allowed.
        let iter = self.elements.iter();
        let prev = self.elements[0].string;
        for i in 1..self.elements.len() {
            let current = self.elements[i].string;
            if prev == current {
                return Err(Error::FromDeserialized {
                    reason: "Duplicate strings are not allowed",
                });
            }
        }

        Err(Error::FromDeserialized {
            reason: "Not implemented yet",
        })
    }

    pub fn build_option(&mut self, option: BytesTrieOption) -> Result<ZeroVec<'a, u8>, Error> {
        match option {
            BytesTrieOption::BuildFast => self.write_node(0, self.elements.len(), 0)?,
            BytesTrieOption::BuildSmall => {
                self.make_node(0, self.elements.len(), 0);
            }
        }
        todo!()
    }

    pub fn write_node(
        &mut self,
        start: usize,
        limit: usize,
        unit_index: usize,
    ) -> Result<(), Error> {
        let mut has_value = false;
        let mut value = 0;
        let mut type_ = 0;
        let mut start = start;
        if unit_index == self.elements[start].string.len() {
            // An intermediate or final value.
            value = self.elements[start + 1].value;
            start += 1;
            if (start == limit) {
                self.write_value_and_final(value, true) // final-value node
            }
            has_value = true;
        }
        // Now all [start..limit[ strings are longer than unitIndex.
        let min_unit: u8 = self.elements[start].string[unit_index];
        let max_unit: u8 = self.elements[limit - 1].string[unit_index];
        if (min_unit == max_unit) {
            // Linear-match node: All strings have the same character at unitIndex.
            let mut last_unit_index = self.get_limit_of_linear_match(start, limit - 1, unit_index);
            self.write_node(start, limit, last_unit_index);
            // Break the linear-match sequence into chunks of at most kMaxLinearMatchLength.
            let mut length = last_unit_index - unit_index;
            while (length > MAX_LINEAR_MATCH_LENGTH as usize) {
                last_unit_index -= MAX_LINEAR_MATCH_LENGTH as usize;
                length -= MAX_LINEAR_MATCH_LENGTH as usize;
                self.write_element_units(start, last_unit_index, MAX_LINEAR_MATCH_LENGTH as usize);
                self.write(MIN_LINEAR_MATCH + MAX_LINEAR_MATCH_LENGTH as u8 - 1);
            }
                    self.write_element_units(start, unit_index, length);
                    type_ = MIN_LINEAR_MATCH as usize + length -1;
                } else {
                    // Branch node.
             let       length=self.count_element_units(start, limit, unit_index);
                    // length>=2 because minUnit!=maxUnit.
            //         writeBranchSubNode(start, limit, unitIndex, length);
            //         if(--length<getMinLinearMatch()) {
            //             type=length;
            //         } else {
            //             write(length);
            //             type=0;
            //         }
            }
            //     return writeValueAndType(hasValue, value, type);
        todo!()
    }

    fn write_value_and_final(&mut self, i: i32, is_final: bool) -> () {
        let is_final: u8 = if is_final { 1 } else { 0 };
        let a = match i {
            i if 0 <= i && i <= MAX_ONE_BYTE_VALUE as i32 => {
                let temp: i32 = ((MIN_ONE_BYTE_VALUE_LEAD as i32 + i) << 1) | is_final as i32;
                self.write_bytes(&temp.to_le_bytes());
            }
            i if i < 0 || i > 0xffffff => self.write_bytes(&[
                FIVE_BYTE_VALUE_LEAD,
                (i >> 24) as u8,
                (i >> 16) as u8,
                (i >> 8) as u8,
                i as u8 | is_final,
            ]),
            i if (i <= MAX_TWO_BYTE_VALUE) => self.write_bytes(&[
                (MIN_TWO_BYTE_VALUE_LEAD as i32 + (i >> 8)) as u8,
                i as u8 | is_final,
            ]),
            i if i <= MAX_THREE_BYTE_VALUE => self.write_bytes(&[
                (MIN_THREE_BYTE_VALUE_LEAD + (i >> 16)) as u8,
                (i >> 8) as u8 | is_final,
            ]),
            _ => self.write_bytes(&[
                FOUR_BYTE_VALUE_LEAD as u8,
                (i >> 16) as u8,
                (i >> 8) as u8 | is_final,
            ]),
        };
    }

    fn write_element_units(&mut self, i: usize, byte_index: usize, length: usize) -> () {
        self.write_bytes(&self.elements[i].string[byte_index..byte_index + length]);
    }


fn count_element_units( &self,start: usize,  limit: usize,  byte_index: usize) -> usize {
    let mut length=0;  // Number of different bytes at byteIndex.
    let mut i=start;
    while {
        i += 1;
        let byte=self.elements[i].string[byte_index];
        while i<limit && byte==self.elements[i].string[byte_index] {
            i += 1;
        }
        length += 1;
        i<limit
        } {} 
     length
}

    fn get_limit_of_linear_match(&self, first: usize, last: usize, byte_index: usize) -> usize {
        let first_element = &self.elements[first];
        let last_element = &self.elements[last];
        let min_string_length = first_element.string.len();
        let mut byte_index = byte_index;
        while {
            byte_index += 1;
             byte_index < min_string_length
                && first_element.string[byte_index] == last_element.string[byte_index]
        } {}
        byte_index
    }

    pub fn make_node(
        &mut self,
        start: usize,
        limit: usize,
        unit_index: usize,
    ) -> Result<Node, Error> {
        todo!()
    }

    fn write(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.reserve(bytes.len());
        for &x in bytes.iter().rev() {
            self.bytes.push(x);
        }
        self.bytes.len();
    }

    fn reverse_bytes(&mut self) {
        todo!();
    }
}
