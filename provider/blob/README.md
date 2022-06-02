# icu_provider_blob [![crates.io](https://img.shields.io/crates/v/icu_provider_blob)](https://crates.io/crates/icu_provider_blob)

`icu_provider_blob` contains implementations of the [`ICU4X`] [`BufferProvider`] interface
that load data from a single blob.

There are two exports:

1. [`BlobDataProvider`] supports data blobs loaded dynamically at runtime.
2. [`StaticDataProvider`] supports data blobs baked into the binary at compile time.

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
