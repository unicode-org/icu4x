#[derive(Clone, Copy, PartialEq)]
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

pub trait Trie {
    fn first(&mut self, trie_data: &[u8], in_unut: i32) -> TrieResult;
    fn next(&mut self, trie_data: &[u8], in_unut: i32) -> TrieResult;
    fn box_clone(&self) -> Box<dyn Trie>;
}

impl Clone for Box<dyn Trie> {
    fn clone(&self) -> Box<dyn Trie> {
        self.box_clone()
    }
}
