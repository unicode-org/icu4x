# icu_datagen_bikeshed [![crates.io](https://img.shields.io/crates/v/icu_datagen_bikeshed)](https://crates.io/crates/icu_datagen_bikeshed)

<!-- cargo-rdme start -->

`icu_datagen_bikeshed` defines [`DatagenProvider`], the authorative ICU4X [`DataProvider`] that produces data from
CLDR and ICU sources.

As the name suggests, [`DatagenProvider`] is mainly intended as a source for the `icu_datagen` crate,
which transforms the data into a more efficient format.

## Cargo features

* `networking`
  * enables networking support to download CLDR and ICU source data from GitHub
* `use_wasm` / `use_icu4c`
  * see the documentation on [`icu_codepointtrie_builder`](icu_codepointtrie_builder#build-configuration)
* `experimental`
  * enables markers defined in the unstable `icu::experimental` module

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
