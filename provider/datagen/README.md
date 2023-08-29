# icu_datagen [![crates.io](https://img.shields.io/crates/v/icu_datagen)](https://crates.io/crates/icu_datagen)

<!-- cargo-rdme start -->

`icu_datagen` is a library to generate data files that can be used in ICU4X data providers.

Data files can be generated either programmatically (i.e. in `build.rs`), or through a
command-line utility.


Also see our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/docs/tutorials/data_management.md).

## Examples

### Rust API

```rust
use icu_datagen::blob_exporter::*;
use icu_datagen::prelude::*;
use std::fs::File;

DatagenDriver::new()
    .with_keys([icu::list::provider::AndListV1Marker::KEY])
    .export(
        &DatagenProvider::latest_tested(),
        BlobExporter::new_with_sink(Box::new(
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
$ icu4x-datagen --keys all --locales de en-AU --format blob --out data.postcard
```
More details can be found by running `--help`.

## Features

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
* `legacy_api`
  * enables the deprecated pre-1.3 API
  * enabled by default for semver stability
  * will be removed in 2.0.

Experimental unstable ICU4X components are behind features which are not enabled by default. Note that these features
affect the behaviour of [`all_keys`]:
* `icu_compactdecimal`
* `icu_displaynames`
* `icu_relativetime`
* `icu_singlenumberformatter`
* `icu_transliteration`
* `icu_unitsconversion`
* ...

The meta-feature `experimental_components` is available to activate all experimental components.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
