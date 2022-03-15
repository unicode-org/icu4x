# icu_properties [![crates.io](https://img.shields.io/crates/v/icu_properties)](https://crates.io/crates/icu_properties)

`icu_properties` is one of the [`ICU4X`] components.

This component provides definitions of [Unicode Properties] and APIs for
retrieving property data in an appropriate data structure.

APIs that return a [`UnicodeSet`] exist for binary properties and certain enumerated
properties. See the [`sets`] module for more details.

APIs that return a [`CodePointTrie`] exist for certain enumerated properties. See the
[`maps`] module for more details.

## Examples

### Property data as `UnicodeSet`s

```rust
use icu::properties::{maps, sets, GeneralCategory};

let provider = icu_testdata::get_provider();

// A binary property as a `UnicodeSet`

let payload =
    sets::get_emoji(&provider)
        .expect("The data should be valid");
let data_struct = payload.get();
let emoji = &data_struct.inv_list;

assert!(emoji.contains('🎃'));  // U+1F383 JACK-O-LANTERN
assert!(!emoji.contains('木'));  // U+6728

// An individual enumerated property value as a `UnicodeSet`

let payload = maps::get_general_category(&provider)
    .expect("The data should be valid");
let data_struct = payload.get();
let gc = &data_struct.code_point_trie;
let line_sep = gc.get_set_for_value(GeneralCategory::LineSeparator);

assert!(line_sep.contains_u32(0x2028));
assert!(!line_sep.contains_u32(0x2029));
```

### Property data as `CodePointTrie`s

```rust
use icu::properties::{maps, Script};

let provider = icu_testdata::get_provider();

let payload =
    maps::get_script(&provider)
        .expect("The data should be valid");
let data_struct = payload.get();
let script = &data_struct.code_point_trie;

assert_eq!(script.get('🎃' as u32), Script::Common);  // U+1F383 JACK-O-LANTERN
assert_eq!(script.get('木' as u32), Script::Han);  // U+6728
```

[`ICU4X`]: ../icu/index.html
[Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
[`UnicodeSet`]: icu_uniset::UnicodeSet
[`CodePointTrie`]: icu_codepointtrie::CodePointTrie
[`sets`]: crate::sets

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
