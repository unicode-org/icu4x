# icu_provider_blob [![crates.io](https://img.shields.io/crates/v/icu_provider_blob)](https://crates.io/crates/icu_provider_blob)

<!-- cargo-rdme start -->

`icu_provider_blob` contains [`BlobDataProvider`], a [`BufferProvider`] implementation that
supports loading data from a single serialized blob.

To build blob data, use the `--format blob` option of [`icu_provider_export`]:

```bash
$ icu4x-datagen --markers all --locales full --format blob --out data.postcard
```

[`ICU4X`]: ../icu/index.html
[`BufferProvider`]: icu_provider::buf::BufferProvider
[`icu_provider_export`]: ../icu_provider_export/index.html

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
