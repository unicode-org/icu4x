<!--- DO NOT EDIT: This file is generated from crate-level Rustdoc (lib.rs) -->

# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

`icu_datagen` is a library to generate data files that can be used in ICU4X data providers.

Data files can be generated either programmatically (i.e. in `build.rs`), or through a
command-line utility.


Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/docs/tutorials/data_management.md)

## Examples

### `build.rs`

```rust
use icu_datagen::prelude::*;
use std::fs::File;

fn main() {
    icu_datagen::datagen(
        Some(&[langid!("de"), langid!("en-AU")]),
        &[icu::list::provider::AndListV1Marker::KEY],
        &SourceData::default(),
        vec![Out::Blob(Box::new(File::create("data.postcard").unwrap()))],
    )
    .unwrap();
}
```

### Command line

The command line interface can be installed through Cargo.

```bash
$ cargo install icu4x-datagen
```

Once the tool is installed, you can invoke it like this:

```bash
$ icu4x-datagen \
>    --keys all \
>    --locales de en-AU \
>    --format blob \
>    --out data.postcard
```
More details can be found by running `--help`.

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
