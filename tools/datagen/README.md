# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

`icu_datagen` contains command-line tools to generate and process ICU4X data.

The tools include:

1. `icu4x-datagen`: Read source data (CLDR JSON) and dump ICU4X-format data.
2. `icu4x-testdata-download`: Download fresh CLDR JSON for testdata.

More details on each tool can be found by running `--help`.

## Examples

Generate ICU4X Postcard blob (single file) for all keys and all locales:

```bash
# Run from the icu4x project folder
$ cargo run --bin icu4x-datagen -- \
    --cldr-tag 39.0.0 \
    --all-keys \
    --all-locales \
    --format blob \
    --out /tmp/icu4x_data/icu4x_data.postcard
```

Extract the keys used by an executable into a key file:

```bash
# Run from the icu4x project folder
$ cargo build --example work_log --release
$ cargo make icu4x-key-extract \
    target/release/examples/work_log \
    /tmp/icu4x_data/work_log+keys.txt
$ cat /tmp/icu4x_data/work_log+keys.txt
```

Generate ICU4X JSON file tree from the key file for Spanish and German:

```bash
# Run from the icu4x project folder
$ cargo run --bin icu4x-datagen -- \
    --cldr-tag 39.0.0 \
    --key-file /tmp/icu4x_data/work_log+keys.txt \
    --locales es \
    --locales de \
    --out /tmp/icu4x_data/work_log_json
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
