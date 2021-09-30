// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::bytestrie::BytesTrie;
use crate::error::Error;
use zerovec::map::ZeroVecLike;
use zerovec::ZeroVec;

pub struct BytesTrieElement<'a> {
    string: &'a str,
    value: i32,
}

impl<'a> BytesTrieElement<'a> {
    fn new(string: &'a str, value: i32) -> BytesTrieElement<'a> {
        BytesTrieElement {
            string: string,
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

pub enum BytesTrieOption {}

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

    pub fn make_node(start: i32, limit: i32, unit_index: u32) -> Result<Node, Error> {
        todo!()
    }

    fn write(&mut self, byte: u8) -> usize {
        self.bytes.push(byte);
        self.bytes.len()
    }

    fn write_many(&mut self, bytes: &[u8]) -> usize {
        self.bytes.reserve(bytes.len());
        // todo consider using memcpy
        for &x in bytes {
            self.bytes.push(x);
        }
        self.bytes.len()
    }

    fn write_value_and_final(&mut self, i: i32, is_final: bool) -> i32 {
        todo!()
    }
}
