# icu_codepointtrie_builder [![crates.io](https://img.shields.io/crates/v/icu_codepointtrie_builder)](https://crates.io/crates/icu_codepointtrie_builder)

<!-- cargo-rdme start -->

`icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.

This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
Because it is normally expected for [`CodePointTrie`] data to be pre-compiled, this crate is not
optimized for speed; it should be used during a build phase.

Under the hood, this crate uses a pure-Rust port of the [`CodePointTrie`] builder code
from ICU4C, [`UMutableCPTrie`].
For more context, see <https://github.com/unicode-org/icu4x/issues/1837>.

## Examples

```rust
use icu::collections::codepointtrie::TrieType;
use icu_codepointtrie_builder::CodePointTrieBuilder;

let default_value = 1u8;
let error_value = 2;

let mut builder =
    CodePointTrieBuilder::new(default_value, error_value, TrieType::Small);
builder.set_value(0, 3);
builder.set_value(1, 4);
builder.set_value(2, 5);
builder.set_value(3, 6);
let cpt = builder.build();

assert_eq!(cpt.get32(0), 3);
assert_eq!(cpt.get32(1), 4);
assert_eq!(cpt.get32(2), 5);
assert_eq!(cpt.get32(3), 6);
assert_eq!(cpt.get32(4), 1); // default value
assert_eq!(cpt.get32(u32::MAX), 2); // error value
```

[`ICU4X`]: ../icu/index.html
[`CodePointTrie`]: icu_collections::codepointtrie::CodePointTrie
[`UMutableCPTrie`]: (https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/umutablecptrie_8h.html#ad8945cf34ca9d40596a66a1395baa19b)

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
