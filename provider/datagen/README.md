# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

`icu_datagen` is a library to generate data files that can be used in ICU4X data providers.

Data files can be generated either programmatically (i.e. in `build.rs`), or through a
command-line utility.

## Examples

### `build.rs`

```rust
use icu_datagen::*;
use icu_locid::langid;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    icu_datagen::datagen(
        Some(&[langid!("de"), langid!("en-AU")]),
        &icu_datagen::keys(&["list/and@1"]),
        &SourceData::default().with_uprops(PathBuf::from("/path/to/uprops/root")),
        Out::Blob(Box::new(File::create("data.postcard").unwrap())),
        false,
    ).unwrap();
}
```

### Command line
The command line interface is available with the `bin` feature.
```bash
cargo run --features bin -- \
    --uprops-root /path/to/uprops/root \
    --all-keys \
    --locales de,en-AU \
    --format blob \
    --out data.postcard
```
More details can be found by running `--help`.

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
