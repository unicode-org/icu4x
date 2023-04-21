# icu_person_names_formatter [![crates.io](https://img.shields.io/crates/v/icu_person_names_formatter)](https://crates.io/crates/icu_person_names_formatter)

Provides person name formatter according to language-dependent conventions.

This module is published as its own crate ([`icu_person_names_formatter`](https://docs.rs/icu_person_names_formatter/latest/icu_person_names_formatter/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
`PersonNamesFormatter` is the main structure of the component. It accepts a set of arguments
which allow it to collect necessary data from the data provider, and once
instantiated, can be used to format Person names as strings.

Refer to the ICU User Guide sections for Person Names Formatter that give an
[introduction](https://unicode-org.github.io/icu/userguide/person_names_formatter/) and explain
[basic concepts](https://unicode-org.github.io/icu/userguide/person_names_formatter/concepts.html).

## Examples

As its most basic purpose, `PersonNamesFormatter` offers locale-aware formatting:

TODO

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
