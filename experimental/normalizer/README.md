# icu_normalizer [![crates.io](https://img.shields.io/crates/v/icu_normalizer)](https://crates.io/crates/icu_normalizer)

`icu_normalizer` is one of the ICU4X components.

This API provides necessary functionality for normalizing text into Unicode
Normalization Forms.

## Implementation notes

The normalizer data layout is not based on the ICU4C design at all. Instead, the normalization
data layout is a clean-slate design optimized for the concept of fusing the NFD decomposition
into the collator. That is, the normalizer proper is a by-product of the collator-motivated
data layout and the design isn’t informed by NFC needs. Instead, NFC is treated as a separate
project that is assumed to build on top of the NFD data, but beyond vaguely assuming that
that’s possible (from the spec definitions, it has to be at least in non-performance-optimized
sense), the needs of NFC haven’t informed the NFD data layout at all.

Notably, the data structure is optimized for a starter decomposing to itself, which is the
most common case, and for a starter decomposing to a starter and a non-starter. Notably, in
this case, the collator in particular makes use of the knowledge that the second character
of such a decomposition is a non-starter. Therefore, decomposition into two starters is
handled by generic fallback path that looks the decomposion from an array by offset and length
instead of baking a BMP character pair directly into a trie value.

The normalizer operates on a lazy iterator over Unicode scalar values (Rust `char`) internally
and iterating over guaranteed-valid UTF-8, potentially-invalid UTF-8, and potentially-invalid
UTF-16 is a step that doesn’t leak into the normalizer internals. UTF errors are treated as
U+FFFD.

Since the normalizer produces output with `char` values read from the data, UB ensues if
invalid data with values outside the scalar value range is used. TODO: data validation.

The decompositions of non-starters are hard-coded. At present in Unicode, these appear
to be special cases falling into two categories:

1. Deprecated Greek combining marks.
2. Particular Tibetan vowel sings.

Hopefully Unicode never adds more decomposing non-starters, but if it does, a code update
is needed instead of a mere data update.

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
