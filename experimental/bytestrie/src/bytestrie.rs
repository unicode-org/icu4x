use crate::impl_const::*;
use zerovec::ZeroVec;

pub struct BytesTrie<'trie> {
    pub bytes: ZeroVec<'trie, u8>,
}

// Enum returned by many functions of BytesTrieReader
pub enum BytesTrieResult {
    /**
     * The input unit(s) did not continue a matching string.
     * Once current()/next() return USTRINGTRIE_NO_MATCH,
     * all further calls to current()/next() will also return USTRINGTRIE_NO_MATCH,
     * until the trie is reset to its original state or to a saved state.
     */
    NoMatch,
    /**
     * The input unit(s) continued a matching string
     * but there is no value for the string so far.
     * (It is a prefix of a longer string.)
     */
    NoValue,
    /**
     * The input unit(s) continued a matching string
     * and there is a value for the string so far.
     * This value will be returned by getValue().
     * No further input byte/unit can continue a matching string.
     */
    FinalValue(i32),
    /**
     * The input unit(s) continued a matching string
     * and there is a value for the string so far.
     * This value will be returned by getValue().
     * Another input byte/unit can continue a matching string.
     */
    IntermediateValue(i32),
}

impl Into<Option<i32>> for BytesTrieResult {
    fn into(self) -> Option<i32> {
        match self {
            BytesTrieResult::NoMatch | BytesTrieResult::NoValue => None,
            BytesTrieResult::FinalValue(x) | BytesTrieResult::IntermediateValue(x) => Some(x),
        }
    }
}

impl<'trie> From<Vec<u8>> for BytesTrie<'trie> {
    fn from(bytes: Vec<u8>) -> Self {
        todo!()
    }
}

impl<'trie> From<ZeroVec<'trie, u8>> for BytesTrie<'trie> {
    fn from(bytes: ZeroVec<'trie, u8>) -> Self {
        todo!()
    }
}

impl<'trie> BytesTrie<'trie> {
    fn reader(&'trie self) -> BytesTrieReader<'trie> {
        BytesTrieReader::new(self)
    }
}

pub struct BytesTrieReader<'trie> {
    remaining_match_length: i32,
    position: usize,
    trie: &'trie BytesTrie<'trie>,
}

impl<'trie> BytesTrieReader<'trie> {
    pub fn new(trie: &'trie BytesTrie<'trie>) -> Self {
        BytesTrieReader {
            remaining_match_length: 0,
            position: 0,
            trie: trie,
        }
    }

    /**
     * Returns the state of this trie as a 64-bit integer.
     * The state value is never 0.
     *
     * @return opaque state value
     * @see reset_to_state64
     */
    pub fn get_state64(&self) -> u64 {
        ((self.remaining_match_length as u64 + 2) << STATE64_REMAINING_SHIFT) | self.position as u64
    }

    /**
     * Resets this trie to the saved state.
     * Unlike resetToState(State), the 64-bit state value
     * must be from get_state64() from the same trie object or
     * from one initialized the exact same way.
     * Because of no validation, this method is faster.
     *
     * @param state The opaque trie state value from get_state64().
     * @see get_state64
     * @see reset
     */
    pub fn reset_to_state64(&mut self, state: u64) {
        self.remaining_match_length = ((state >> STATE64_REMAINING_SHIFT) - 2) as i32;
        self.position = (state & STATE64_POS_MASK) as usize;
    }
    /**
     * Determines whether the byte sequence so far matches, whether it has a value,
     * and whether another input byte can continue a matching byte sequence.
     * @return BytesTrieResult.
     */
    pub fn current(&self) -> BytesTrieResult {
        todo!()
    }

    /**
     * Traverses the trie from the initial state for this input byte.
     * Equivalent to reset().next(inByte).
     * @param in_byte Input byte value. Values -0x100..-1 are treated like 0..0xff.
     *               Values below -0x100 and above 0xff will never match.
     * @return BytesTrieResult.
     */
    pub fn first(&self, in_byte: i32) -> BytesTrieResult {
        todo!()
    }

    /**
     * Traverses the trie from the current state for this input byte.
     * @param in_byte Input byte value. Values -0x100..-1 are treated like 0..0xff.
     *               Values below -0x100 and above 0xff will never match.
     * @return BytesTrieResult.
     */
    pub fn next(&self, in_byte: i32) -> BytesTrieResult {
        todo!()
    }

    /**
     * Traverses the trie from the current state for this byte sequence.
     * @param bytes A byte sequence.
     * @return The match/value Result.
     */
    pub fn next_bytes(&self, bytes: &[i8]) -> BytesTrieResult {
        todo!()
    }
}

impl<'trie> Iterator for BytesTrieReader<'trie> {
    type Item = (&'trie str, i32);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
