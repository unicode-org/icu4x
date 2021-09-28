// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::bytestrie::BytesTrie;
use crate::error::Error;
use zerovec::ZeroVec;
use zerovec::map::ZeroVecLike;

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
    bytes: zerovec::ZeroVec<'a, i8>
}

trait TrieBuilder {
    fn write(&mut self, byte: i8) -> usize;
    fn writeMany(&mut self,bytes: &[i8] ) -> usize;
}

impl TrieBuilder for zerovec::ZeroVec<'_, i8> {
    
    fn write(&mut self,byte: i8) -> usize{
        self.push(byte);
        self.len()
    }

    fn writeMany(&mut self, bytes: &[i8]) -> usize {
        self.reserve(bytes.len());
        // todo consider using memcpy
        for &x in bytes {
            self.push(x);
        }
        self.len()
    }

}


enum Node {
    FinalValueTrieNode,
    ValueNode,
    LinearMatchNode,
    BranchNode
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
        let bytes = self.buildBytes(options)?;
        Result::Ok(BytesTrie{bytes})
    }

    pub fn buildBytes(& mut self, options: &BytesTrieOption) -> Result<ZeroVec<'a, u8>, Error> {
        if self.elements.is_empty() {
            return Err(Error::FromDeserialized {
              reason: "Illegal argument. BytesTrie must be non-empty",  
            })
        }

        self.elements.sort_unstable_by(|a,b| b.string.cmp(&a.string));
        //Duplicate strings are not allowed.
        let iter =         self.elements.iter();
        let mut prev = self.elements[0].string;
        for i in 1..self.elements.len()  {
            let current = self.elements[i].string;
            if prev == current {
                return Err(Error::FromDeserialized{
                    reason: "Duplicate strings are not allowed"
                
                })
            }
        }


        
        Err(Error::FromDeserialized {
            reason: "Not implemented yet",
        })
    }

    fn makeNode(start: i32, limit: i32, unitIndex: u32) -> Result<Node, Error>{
        todo!()
    }

    fn writeValueAndFinal(i: i32, isFinal: bool) -> i32 {
        // if(0<=i && i<=BytesTrieBuilder::MAX_ONE_BYTE_VALUE) {
        //     return write(((BytesTrieBuilder::MIN_ONE_BYTE_VALUE_LEAD+i)<<1)|isFinal);
        // }
        // char intBytes[5];
        // int32_t length=1;
        // if(i<0 || i>0xffffff) {
        //     intBytes[0]=(char)BytesTrie::kFiveByteValueLead;
        //     intBytes[1]=(char)((uint32_t)i>>24);
        //     intBytes[2]=(char)((uint32_t)i>>16);
        //     intBytes[3]=(char)((uint32_t)i>>8);
        //     intBytes[4]=(char)i;
        //     length=5;
        // // } else if(i<=BytesTrie::kMaxOneByteValue) {
        // //     intBytes[0]=(char)(BytesTrie::kMinOneByteValueLead+i);
        // } else {
        //     if(i<=BytesTrie::kMaxTwoByteValue) {
        //         intBytes[0]=(char)(BytesTrie::kMinTwoByteValueLead+(i>>8));
        //     } else {
        //         if(i<=BytesTrie::kMaxThreeByteValue) {
        //             intBytes[0]=(char)(BytesTrie::kMinThreeByteValueLead+(i>>16));
        //         } else {
        //             intBytes[0]=(char)BytesTrie::kFourByteValueLead;
        //             intBytes[1]=(char)(i>>16);
        //             length=2;
        //         }
        //         intBytes[length++]=(char)(i>>8);
        //     }
        //     intBytes[length++]=(char)i;
        // }
        // intBytes[0]=(char)((intBytes[0]<<1)|isFinal);
        // return write(intBytes, length);
        todo!()
    }
}
