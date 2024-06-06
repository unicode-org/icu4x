# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

<!-- cargo-rdme start -->

`icu_datagen` is a library to generate data files that can be used in ICU4X data providers.

Data files can be generated either programmatically (i.e. in `build.rs`), or through a
command-line utility.


Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md).

## Examples

### Rust API

```rust
use icu_datagen::blob_exporter::*;
use icu_datagen::prelude::*;
use std::fs::File;

DatagenDriver::new()
    .with_markers([icu::list::provider::AndListV1Marker::INFO])
    .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
    .export(
        &DatagenProvider::new_latest_tested(),
        BlobExporter::new_v2_with_sink(Box::new(
            File::create("data.postcard").unwrap(),
        )),
    )
    .unwrap();
```

### Command line

The command line interface can be installed through Cargo.

```bash
$ cargo install icu_datagen
```

Once the tool is installed, you can invoke it like this:

```bash
$ icu4x-datagen --markers all --locales de en-AU --format blob --out data.postcard
```

More details can be found by running `--help`.

## Cargo features

This crate has a lot of dependencies, some of which are not required for all operating modes. These default Cargo features
can be disabled to reduce dependencies:
* `baked_exporter`
  * enables the [`baked_exporter`] module
  * enables the `--format mod` CLI argument
* `blob_exporter`
  * enables the [`blob_exporter`] module, a reexport of [`icu_provider_blob::export`]
  * enables the `--format blob` CLI argument
* `fs_exporter`
  * enables the [`fs_exporter`] module, a reexport of [`icu_provider_fs::export`]
  * enables the `--format dir` CLI argument
* `networking`
  * enables methods on [`DatagenProvider`] that fetch source data from the network
  * enables the `--cldr-tag`, `--icu-export-tag`, and `--segmenter-lstm-tag` CLI arguments that download data
* `rayon`
  * enables parallelism during export
* `use_wasm` / `use_icu4c`
  * see the documentation on [`icu_codepointtrie_builder`](icu_codepointtrie_builder#build-configuration)
* `bin`
  * required by the CLI and enabled by default to make `cargo install` work
* `icu_experimental`
  * enables data generation for markers defined in the unstable `icu_experimental` crate
  * note that this features affects the behaviour of `all_markers`

The meta-feature `experimental_components` is available to activate all experimental components.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
