# icu_provider_cldr [![crates.io](http://meritbadge.herokuapp.com/icu_provider_cldr)](https://crates.io/crates/icu_provider_cldr)

`icu_provider_cldr` contains implementations of the [`ICU4X`] [`DataProvider`] interface
based on the JSON files shipped by CLDR. Create a [`CldrPaths`] and then pass it into
[`CldrJsonDataProvider`].

This crate contains two implementations of [`CldrPaths`]:

- [`CldrPathsLocal`] points to local copies of the CLDR JSON repositories.
- `CldrPathsDownload` downloads and caches the CLDR JSON repositories. Requires the
  "download" feature.

**Important:** This data provider implementation is not optimized for production use.
It is much more efficient if you use [`FsDataProvider`] instead.

[`ICU4X`]: ../icu/index.html
[`DataProvider`]: icu_provider::prelude::DataProvider
[`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
[`CldrJsonDataProvider`]: transform::CldrJsonDataProvider

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
