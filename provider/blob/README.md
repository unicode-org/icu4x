# icu_provider_blob [![crates.io](https://img.shields.io/crates/v/icu_provider_blob)](https://crates.io/crates/icu_provider_blob)

`icu_provider_blob` contains [`BlobDataProvider`], a [`BufferProvider`] implementation that
supports loading data from a single serialized blob.

To build blob data, use the `--format blob` option of [`icu_datagen`]. For example, to build
"hello world" data, run:

```bash
$ cargo run --features bin -p icu_datagen -- \
    --format blob \
    --hello-world-key \
    --all-locales \
    --out hello_world.postcard
```

For examples, see the specific data providers.

[`ICU4X`]: ../icu/index.html
[`BufferProvider`]: icu_provider::BufferProvider
[`icu_datagen`]: ../icu_datagen/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
