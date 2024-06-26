# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

<!-- cargo-rdme start -->

`icu_datagen` is a library to generate data files that can be used in ICU4X data providers.

For command-line usage, see the [`icu4x-datagen` binary](https://crates.io/crate/icu4x-datagen).

Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md).

## Examples

```rust
use icu_datagen::blob_exporter::*;
use icu_datagen::prelude::*;
use icu_datagen_bikeshed::DatagenProvider;
use std::fs::File;

let provider = DatagenProvider::new_latest_tested();

DatagenDriver::new([LocaleFamily::FULL], FallbackOptions::no_deduplication(), LocaleFallbacker::try_new_unstable(&provider).unwrap())
    .with_markers([icu::list::provider::AndListV2Marker::INFO])
    .export(
        &provider,
        BlobExporter::new_v2_with_sink(Box::new(
            File::create("data.postcard").unwrap(),
        )),
    )
    .unwrap();
```

## Cargo features

This crate has a lot of dependencies, some of which are not required for all operating modes. These default Cargo features
can be disabled to reduce dependencies:
* `baked_exporter`
  * enables the [`baked_exporter`] module, a reexport of [`icu_provider_baked::export`]
  * enables the `--format mod` CLI argument
* `blob_exporter`
  * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
  * enables the `--format blob` CLI argument
* `fs_exporter`
  * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
  * enables the `--format dir` CLI argument
* `rayon`
  * enables parallelism during export
* `experimental`
  * enables data generation for markers defined in the unstable `icu_experimental` crate
  * note that this features affects the behaviour of `all_markers`

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
