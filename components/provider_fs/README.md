# icu_provider_fs [![crates.io](http://meritbadge.herokuapp.com/icu_provider_fs)](https://crates.io/crates/icu_provider_fs)

`icu_fs_data_provider` is one of the `ICU4X` components.

It reads ICU4X data files from the filesystem in a given directory. It can also export data to
the filesystem via an iterable data provider (see the `export` module).

# Examples

```rust
use icu_provider_fs::FsDataProvider;

let provider = FsDataProvider::try_new("/path/to/data/directory")
    .expect_err("Specify a real directory in the line above");
```

# Resource Formats

`ICU4X` data can be stored in different formats. At the moment there are:

* JSON - Textual format, easy to read
* Bincode - Binary, fast resource format

The directory passed to the `FsDataProvider` constructor may contain either of them.

# Exporting data

To generate the data required for `FsDataProvider`, use the following script:

```bash
cargo run
  --features export-bin
  --
  --cldr-tag 37.0.0
  --out ./icu4x-data
  --all-keys
```

To export `bincode` format, use

```bash
cargo run
  --features export-bin
  --featuers bincode
  --features serialize_none
  --
  --cldr-tag 37.0.0
  --out ./icu4x-data
  --all-keys
  -s bincode
```

*Notice:* In order to use `bincode` encoded data in production, `icu_provider_fs` has to be
added with `bincode` feature.

# More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
