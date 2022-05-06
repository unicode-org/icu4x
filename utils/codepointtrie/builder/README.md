# icu_codepointtrie_builder [![crates.io](https://img.shields.io/crates/v/icu_codepointtrie_builder)](https://crates.io/crates/icu_codepointtrie_builder)

`icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.

This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
Because it is normally expected for CodePointTrie data to be pre-compiled, this crate is not
optimized for speed; it should be used during a build phase.

Under the hood, this crate uses the CodePointTrie builder code from ICU4C, [`UMutableCPTrie`],
shipped as a WebAssembly module and then JIT-compiled at runtime. For more context, see
<https://github.com/unicode-org/icu4x/issues/1837>.

## Examples

```rust
use icu_codepointtrie::CodePointTrie;
use icu_codepointtrie::TrieType;
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_codepointtrie_builder::CodePointTrieBuilderData;

let default_value = 1;
let error_value = 2;
let values_by_code_point = &[3, 4, 5, 6];

let cpt: CodePointTrie<u8> = CodePointTrieBuilder {
    data: CodePointTrieBuilderData::ValuesByCodePoint(values_by_code_point),
    default_value,
    error_value,
    trie_type: TrieType::Small
}.build();

assert_eq!(cpt.get(0), 3);
assert_eq!(cpt.get(1), 4);
assert_eq!(cpt.get(2), 5);
assert_eq!(cpt.get(3), 6);
assert_eq!(cpt.get(4), 1); // default value
assert_eq!(cpt.get(u32::MAX), 2); // error value
```

[`ICU4X`]: ../icu/index.html
[`UMutableCPTrie`]: (https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/umutablecptrie_8h.html#ad8945cf34ca9d40596a66a1395baa19b)

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
