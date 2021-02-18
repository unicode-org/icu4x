## `terrain`

`terrain` is a crate providing VecMap, a highly simplistic "flat" key-value map
based off of a single sorted vector.

The goal of this crate is to provide a map that is good enough for small
sizes, and does not carry the binary size impact of [`HashMap`]
or [`BTreeMap`].

If binary size is not a concern, [`BTreeMap`] may be a better choice
for your use case. It behaves very similarly to `VecMap` for less than 12 elements,
and upgrades itself gracefully for larger inputs.

 [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
 [`BTreeMap`]: https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html