# icu_provider_export [![crates.io](https://img.shields.io/crates/v/icu_provider_export)](https://crates.io/crates/icu_provider_export)

<!-- cargo-rdme start -->

`icu_provider_export` is a library to generate data files that can be used in ICU4X data providers.

For command-line usage, see the [`icu4x-datagen` binary](https://crates.io/crate/icu4x-datagen).

See our [tutorials](https://github.com/unicode-org/icu4x/blob/main/tutorials) for more information about different data providers.

## Examples

```rust
use icu_provider_export::blob_exporter::*;
use icu_provider_export::prelude::*;
use icu_provider_source::SourceDataProvider;
use std::fs::File;

let provider = SourceDataProvider::new();

ExportDriver::new(
    [DataLocaleFamily::FULL],
    DeduplicationStrategy::None.into(),
    LocaleFallbacker::try_new_unstable(&provider).unwrap(),
)
.with_markers([icu::list::provider::ListAndV1::INFO])
.export(
    &provider,
    BlobExporter::new_with_sink(Box::new(
        File::create("data.postcard").unwrap(),
    )),
)
.unwrap();
```

## Cargo features

* `baked_exporter`
  * enables the [`baked_exporter`] module, a reexport of [`icu_provider_baked::export`]
* `blob_exporter`
  * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
* `fs_exporter`
  * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
* `rayon`
  * enables parallelism during export

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
