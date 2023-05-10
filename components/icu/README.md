# icu [![crates.io](https://img.shields.io/crates/v/icu)](https://crates.io/crates/icu)

`icu` is the main meta-crate of the `ICU4X` project.

It provides a comprehensive selection of functionality found in
[International Components for Unicode](http://icu.unicode.org/)
in their canonical configurations intended to enable software
internationalization capabilities.

This crate exists to collect the most important functionality for users
together in one place.
It does not bring any unique functionality, but rather,
it re-exports the relevant crates as modules.
The exported crate corresponding to each module is also
available in a stand-alone manner, i.e. `icu::list` as `icu_list`.

## Data Management

Most functionality relies on data which clients have to provide to the APIs.

`ICU4X` uses the concept of a [`DataProvider`] to separate data from logic.
Data providers come in many different forms; the following providers are provided
by `ICU4X` in separate crates:
* [`BlobDataProvider`]: uses an in-memory serde-serialized blob. This is the most flexible provider, and
  data can be updated at runtime.
* `BakedDataProvider`: a code-generated provider that contains the data directly in Rust code. This is
  the most efficient provider as it's serialization-free, and allows for compile-time optimizations.
* [`FsDataProvider`]: uses a file system tree of Serde files. This is mostly useful for development and
  not recommended in production for performance reasons.
* [`icu_provider_adapters`]: this crate contains APIs to combine providers or
  provide additional functionality such as locale fallback.

The data that is required by these providers (in `BakedDataProvider`'s case, the provider itself) can be
generated and customized using the [`icu_datagen`] crate.

The following example uses the [`icu_testdata`] crate, which contains prepackaged data providers
for a small set of locales.

## Example

```rust
use icu::calendar::DateTime;
use icu::datetime::{options::length, DateTimeFormatter};
use icu::locid::locale;
use writeable::assert_writeable_eq;

let options = length::Bag::from_date_time_style(
    length::Date::Long,
    length::Time::Medium,
)
.into();

let dtf = DateTimeFormatter::try_new_unstable(
    &icu_testdata::unstable(),
    &locale!("es").into(),
    options,
)
.expect("Failed to create DateTimeFormatter instance.");

let date = DateTime::try_new_iso_datetime(2020, 9, 12, 12, 35, 0)
    .expect("Failed to parse date.");
let date = date.to_any();

let formatted_date = dtf.format(&date).expect("Formatting failed");
assert_writeable_eq!(formatted_date, "12 de septiembre de 2020, 12:35:00");

let formatted_date_string =
    dtf.format_to_string(&date).expect("Formatting failed");
assert_eq!(formatted_date_string, "12 de septiembre de 2020, 12:35:00");
```

## Features

ICU4X components share a set of common Cargo features that control whether core pieces of
functionality are compiled. These features are:

- `std`: Whether to include `std` support. Without this Cargo feature, `icu` is `#[no_std]`-compatible
- `serde`: Whether to include `serde::Deserialize` implementations for data structs, such as [`SymbolsV1`],
  and `serde::{Serialize, Deserialize}` implementations for core library types, such as [`Locale`]. These are
  required with `serde`-backed providers like [`BlobDataProvider`][^1].
- `experimental`: Whether to enable experimental preview features. Modules enabled with
  this feature may not be production-ready and could change at any time.

The following Cargo features are only available on the individual crates, but not on this meta-crate:

- `datagen`: Whether to implement `serde::Serialize` and functionality that is only required during data generation.
- `bench`: Whether to enable exhaustive benchmarks. This can be enabled on individual crates
  when running `cargo bench`.

There are additional features that, when enabled on specific crates, enable functionality across ICU4X:

- `icu_provider/sync`: makes [`DataPayload`] implement `Send + Sync`, which in turn
  makes most ICU4X objects also implement `Send + Sync`.
- `icu_provider/deserialize_*`: enables ICU4X buffer providers to read various different
  serialization formats. See [`BufferProvider`](icu_provider::BufferProvider) for details.

[^1]: If using blob data, you need to enable one of the deserialization Cargo features on the `icu_provider` crate, as noted above.


[`DataProvider`]: icu_provider::DataProvider
[`DataPayload`]: icu_provider::DataPayload
[`FsDataProvider`]: https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html
[`BlobDataProvider`]: https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html
[`icu_testdata`]: https://docs.rs/icu_testdata/latest/icu_testdata/
[`icu_provider_adapters`]: https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/
[`icu_datagen`]: https://docs.rs/icu_datagen/latest/icu_datagen/
[`Locale`]: crate::locid::Locale
[`SymbolsV1`]: crate::decimal::provider::DecimalSymbolsV1

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
