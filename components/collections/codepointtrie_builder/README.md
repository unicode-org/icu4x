# icu_codepointtrie_builder [![crates.io](https://img.shields.io/crates/v/icu_codepointtrie_builder)](https://crates.io/crates/icu_codepointtrie_builder)

<!-- cargo-rdme start -->

`icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.

This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
Because it is normally expected for CodePointTrie data to be pre-compiled, this crate is not
optimized for speed; it should be used during a build phase.

Under the hood, this crate uses the CodePointTrie builder code from ICU4C, [`UMutableCPTrie`].
For more context, see <https://github.com/unicode-org/icu4x/issues/1837>.

## Build configuration

This crate has two primary modes it can be used in: `"wasm"` and `"icu4c"`, exposed as
Cargo features. If both are enabled, the code will internally use the wasm codepath.

The `"wasm"` mode uses a Wasm module packaged into this Rust crate that contains
pre-compiled ICU4C CodePointTrie builder code. It evaluates the Wasm module using
the Wasmer runtime, which "just works", but it requires a large number of
Rust/Cargo dependencies.

The `"icu4c"` mode reduces the number of Rust dependencies, but it requires having a local copy
of ICU4C available. To configure `"icu4c"` mode in Cargo, set the following environment variables:

- Set `ICU4C_LIB_PATH` to a directory full of ICU4C static or shared libraries.
- Set `ICU4C_LINK_STATICALLY` to any value to use the static libraries.
- Set `ICU4C_RENAME_VERSION` to the integer ICU4C version if ICU4C has renaming
  enabled. By default, we attempt to link non-renamed symbols.

If using dynamic linking, at runtime, you may need to set `[DY]LD_LIBRARY_PATH`
to the `ICU4C_LIB_PATH`.

If _not_ using Cargo, make sure to pass `ICU4C_LIB_PATH` to the linker via `-L`, link against
`icuuc`, `icui18n` and `icudata` via `-l` flags, and set `--cfg icu4c_enable_renaming` if you need
renamed ICU4C symbols.

## Examples

```rust
use icu::collections::codepointtrie::CodePointTrie;
use icu::collections::codepointtrie::TrieType;
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_codepointtrie_builder::CodePointTrieBuilderData;

let default_value = 1;
let error_value = 2;
let values_by_code_point = &[3, 4, 5, 6];

let cpt: CodePointTrie<u8> = CodePointTrieBuilder {
    data: CodePointTrieBuilderData::ValuesByCodePoint(values_by_code_point),
    default_value,
    error_value,
    trie_type: TrieType::Small,
}
.build();

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
