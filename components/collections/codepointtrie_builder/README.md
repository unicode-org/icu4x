# icu_codepointtrie_builder [![crates.io](https://img.shields.io/crates/v/icu_codepointtrie_builder)](https://crates.io/crates/icu_codepointtrie_builder)

`icu_codepointtrie_builder` is a utility crate of the [`ICU4X`] project.

This crate exposes functionality to build a [`CodePointTrie`] from values provided at runtime.
Because it is normally expected for CodePointTrie data to be pre-compiled, this crate is not
optimized for speed; it should be used during a build phase.

Under the hood, this crate uses the CodePointTrie builder code from ICU4C, [`UMutableCPTrie`].
For more context, see <https://github.com/unicode-org/icu4x/issues/1837>.

## Build configuration

This crate has two primary modes it can be used in: `"wasm"` and `"icu4c"`, exposed as
Cargo features. If both are enabled, the code will internally use the wasm codepath.

The `"wasm"` mode uses an included wasm module that was built by linking
it to ICU4C, run on a wasm runtime. It pulls in a lot of dependencies to do this, but
it should just work with no further effort.

The `"icu4c"` mode requires some extra effort: it links to a local copy of ICU4C.
If using Cargo, you can use the `ICU4C_LIB_PATH` environment variable to point this to
a directory full of ICU4X static or shared libraries, and `ICU4C_LINK_STATICALLY` to use
static linking (if using dynamic linking you will have to set `[DY]LD_LIBRARY_PATH` at runtime
as well). If building directly, make sure this path is provided via `-L`, and that the
CLI requests to link against `icuuc`, `icui18n` and `icudata` via `-l` flags.

By default ICU4C uses *renamed* symbols, where each function is suffixed with a version number.
This crate by default will link to ICU4C 72 symbols. If you have built it with renaming
disabled, you can provide the `ICU4C_DISABLE_RENAMING` flag to cargo, or pass `--cfg icu4c_disable_renaming`.
Versions other than ICU4C 72 are not currently supported.

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
[`UMutableCPTrie`]: (https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/umutablecptrie_8h.html#ad8945cf34ca9d40596a66a1395baa19b)

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
