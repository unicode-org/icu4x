# icu_provider_segmenter [![crates.io](https://img.shields.io/crates/v/icu_provider_segmenter)](https://crates.io/crates/icu_provider_segmenter)

`icu_provider_segmenter` contains implementations of the [`ICU4X`] [data provider] interface
based on Unicode properties and TOML files implementing [Unicode Standard Annex #14][UAX14] and
[Unicode Standard Annex #29][UAX29] breaking rules.

**Important:** This provider implementation is not optimized for production use. It is much more
efficient if you use [`FsDataProvider`] or [`StaticDataProvider`] instead.

## Examples

```rust
use icu_provider_segmenter::SegmenterRuleProvider;
let provider = SegmenterRuleProvider::try_new("/path/to/data/directory")
    .expect_err("Specify a real directory in the line above");
```

## Exporting data

To generate the data required by the segmenters, run `cargo make testdata` from the top level.

[`ICU4X`]: ../icu/index.html
[data provider]: icu_provider
[UAX14]: https://www.unicode.org/reports/tr14/
[UAX29]: https://www.unicode.org/reports/tr29/
[`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
[`StaticDataProvider`]: ../icu_provider_blob/struct.StaticDataProvider.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
