# icu_char16trie [![crates.io](http://meritbadge.herokuapp.com/icu_char16trie)](https://crates.io/crates/icu_char16trie)

## icu_char16trie [![crates.io](http://meritbadge.herokuapp.com/icu_char16trie)](https://crates.io/crates/icu_char16trie)

`icu_char16trie` is a utility crate of the [`ICU4X`] project.

This component provides a data structure for an time-efficient lookup of values
associated to code points.

It is an implementation of the existing [ICU4C UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)
/ [ICU4J CharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html) API.

### Architecture

ICU4X [`Char16Trie`](crate::char16trie::Char16Trie) is designed to provide a read-only view of UCharsTrie data that is exported from ICU4C.

### Examples

#### Querying a `Char16Trie`

```rust
use icu_char16trie::char16trie::{Char16Trie, Char16TrieIterator, TrieResult};
use zerovec::ZeroVec;

// A Char16Trie containing the ASCII characters 'a' and 'b'.
let trie_data = vec![48, 97, 176, 98, 32868];
let trie = Char16Trie {
    data: ZeroVec::from_slice(trie_data.as_slice()),
};

let mut itor = Char16TrieIterator::new(trie.data.as_slice());
let res = itor.next('a' as i32);
assert_eq!(res, TrieResult::Intermediate(1));
let res = itor.next('b' as i32);
assert_eq!(res, TrieResult::FinalValue(100));
let res = itor.next('c' as i32);
assert_eq!(res, TrieResult::NoMatch);
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
