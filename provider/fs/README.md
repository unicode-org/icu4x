# icu_provider_fs [![crates.io](https://img.shields.io/crates/v/icu_provider_fs)](https://crates.io/crates/icu_provider_fs)

<!-- cargo-rdme start -->

`icu_provider_fs` is one of the [`ICU4X`] components.

It reads ICU4X data files from the filesystem in a given directory.

## Examples

```rust
use icu_provider_fs::FsDataProvider;

let provider = FsDataProvider::try_new("/path/to/data/directory")
    .expect_err("Specify a real directory in the line above");
```

## Directory Structure

The ICU4X data directory has a file named `manifest.json` at the root, and a nested structure
with a data key ([`DataKey`](icu_provider::DataKey)), and locale ([`DataLocale`](icu_provider::DataLocale))
as the leaf data files. For example, Arabic JSON data for cardinal plural rules lives at `plurals/cardinal@1/ar.json`.

The exact form of the directory structure may change over time. ICU4X uses metadata from
`manifest.json` to dynamically interpret different versions of the directory structure.

```text
├── manifest.json
├── dates
│   └── gregory@1
│       ├── ar-EG.json
│       ├── ar.json
│       ├── be.json
│       ⋮
│       └── und.json
└── plurals
    ├── cardinal@1
    │   ├── ar.json
    │   ├── be.json
    │   ⋮
    │   └── und.json
    └── ordinal@1
        ├── ar.json
        ├── be.json
        ⋮
        └── und.json
```

## Resource Formats

`ICU4X` data can be stored in different formats. At the moment there are:

* JSON - Textual format, easy to read
* Postcard - Binary, small `#[no_std]` resource format
* Bincode - Binary, fast resource format

The directory passed to the [`FsDataProvider`] constructor may contain either of them.

*Notice:* In order for ICU4X to be able to *deserialize* the returned data, the corresponding
Cargo feature has to be activated on the [`icu_provider`] crate. See
[`AsDeserializingBufferProvider::as_deserializing`](icu_provider::serde::AsDeserializingBufferProvider).

## Exporting data

To generate the data required for [`FsDataProvider`], run the following:

```bash
icu4x-datagen --keys all --locales full --format dir
```

To export `postcard` format, use

```bash
icu4x-datagen --keys all --locales full --format dir --syntax postcard
```

[`ICU4X`]: ../icu/index.html

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
