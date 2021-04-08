# litemap [![crates.io](http://meritbadge.herokuapp.com/litemap)](https://crates.io/crates/litemap)

### `litemap`

`litemap` is a crate providing [`LiteMap`], a highly simplistic "flat" key-value map
based off of a single sorted vector.

The goal of this crate is to provide a map that is good enough for small
sizes, and does not carry the binary size impact of [`std::collections::HashMap`]
or [`std::collections::BTreeMap`].

If binary size is not a concern, [`std::collections::BTreeMap`] may be a better choice
for your use case. It behaves very similarly to [`LiteMap`] for less than 12 elements,
and upgrades itself gracefully for larger inputs.


## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
