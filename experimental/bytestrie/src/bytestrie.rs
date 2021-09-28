// const uint8_t *bytes_;
// const uint8_t *pos_;
// const uint8_t *initialPos_;
// int32_t remainingMatchLength_;
// int32_t initialRemainingMatchLength_;

// CharString *str_;
// int32_t maxLength_;
// int32_t value_;

// // The stack stores pairs of integers for backtracking to another
// // outbound edge of a branch node.
// // The first integer is an offset from bytes_.
// // The second integer has the str_->length() from before the node in bits 15..0,
// // and the remaining branch length in bits 24..16. (Bits 31..25 are unused.)
// // (We could store the remaining branch length minus 1 in bits 23..16 and not use bits 31..24,
// // but the code looks more confusing that way.)
// UVector32 *stack_;

use zerovec::ZeroVec;

pub struct BytesTrie<'trie> {
    bytes : ZeroVec<'trie, u8>
}

impl <'trie> From<Vec<u8>> for BytesTrie<'trie>{
    fn from(bytes: Vec<u8>) -> Self{
        todo!()
    }
} 

impl <'trie> From<ZeroVec<'trie,u8>> for BytesTrie<'trie>{
    fn from(bytes: ZeroVec<'trie,u8>) -> Self{
        todo!()
    }
} 

impl<'trie> BytesTrie<'trie> {
    fn getState64(&self) -> u64 {
        todo!()
    }

    fn resetToState64(self, state: u64) -> BytesTrie<'trie>{
        todo!()
    }
}
