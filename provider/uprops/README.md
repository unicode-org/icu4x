# icu_provider_uprops [![crates.io](http://meritbadge.herokuapp.com/icu_provider_uprops)](https://crates.io/crates/icu_provider_uprops)

`icu_provider_uprops` contains implementations of the [`ICU4X`]
[`DataProvider`] interface backed by TOML files exported by the
ICU4C icuwriteuprops tool. Create a directory containing TOML files for
the necessary Unicode properties and then pass the path into the
[`PropertiesDataProvider`].

**Important:** This data provider implementation is not optimized
for production use.  It is much more efficient if you use
[`FsDataProvider`] or [`StaticDataProvider`] instead.

[`ICU4X`]: ../icu/index.html
[`DataProvider`]: icu_provider::prelude::DataProvider
[`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
[`StaticDataProvider`]: ../icu_provider_blob/struct.StaticDataProvider.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
