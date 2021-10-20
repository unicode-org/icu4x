# icu_properties [![crates.io](http://meritbadge.herokuapp.com/icu_properties)](https://crates.io/crates/icu_properties)

`icu_properties` is a utility crate of the [`ICU4X`] project.

This component provides definitions of [Unicode Properties] and APIs for
retrieving property data in an appropriate data structure.

Currently, only binary property APIs are supported, with APIs that return
a [`UnicodeSet`]. See the [`sets`] module for more details.

[`ICU4X`]: ../icu/index.html
[Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
[`UnicodeSet`]: icu_uniset::UnicodeSet
[`sets`]: crate::sets

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
