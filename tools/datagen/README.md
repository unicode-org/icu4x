# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

`icu_datagen` contains command-line tools to generate and process ICU4X data.

The tools include:

1. `icu4x-datagen`: Read source data (CLDR JSON) and dump ICU4X-format data.
2. `icu4x-testdata-download`: Download fresh CLDR JSON for testdata.
3. `icu4x-key-extract`: Extract `ResourceKey` objects present in a compiled executable.

More details on each tool can be found by running `--help`.

## Examples

Generate ICU4X JSON file tree:

```bash
# Run from the icu4x project folder
$ cargo run --bin icu4x-datagen -- \
   --cldr-tag 39.0.0 \
   --all-keys \
   --all-locales \
   --out /tmp/icu4x_data/json
```

Generate ICU4X Postcard blob (single file):

```bash
# Run from the icu4x project folder
$ cargo run --bin icu4x-datagen -- \
   --cldr-tag 39.0.0 \
   --all-keys \
   --all-locales \
   --format blob \
   --out /tmp/icu4x_data/icu4x_data.postcard
```

Generate ICU4X Bincode file tree:

```bash
# Run from the icu4x project folder
$ cargo run --bin icu4x-datagen -- \
   --cldr-tag 39.0.0 \
   --all-keys \
   --all-locales \
   --syntax bincode \
   --out /tmp/icu4x_data/bincode
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
