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
to be special cases falling into three categories:

1. Deprecated Greek combining marks.
2. Particular Tibetan vowel sings.
3. NFKD only: half-width kana voicing marks.

Hopefully Unicode never adds more decomposing non-starters, but if it does, a code update
is needed instead of a mere data update.

## Data size considerations

The normalizer supports three flavors: NFD, NFKD, and the decomposed counterpart of
NFKC_CaseFold without ignoring default ignorables. Logically NFKD adds data on top of NFD
and case fold adds data on top of NFKD (some of the details may be a bit more complicated).

Currently, the complex decomposition expansion data is consolidated such that there is one
data struct that contains the expansions needed by NFD and there's another data instance
that contains the expansions for both NFKD and fusion of case fold and NFKD.

However, similar consolidation hasn't been applied to the trie or to the set of characters
whose decompositions starts with a non-starter. These are tradeoffs between data size and
run-time branchiness. It is unclear if the present situation is the right call.

As of Unicode 14, the set of characters whose decompositions starts with a non-starter
is almost the same all three cases. NFKD adds two characters to the set compared to NFD:
the half-width kana voicing marks. The case fold variant then removes one character
compared to regular NFKD: U+0345 COMBINING GREEK YPOGEGRAMMENI.

There are three alternatives to the current solution of having three pre-computed sets.

First, we could add explicit checks for the chosen normalization form and these three
characters to the set lookup, which would add branches to each set lookup and would
hard-code the assumption that only these three characters may differ between the
normalization forms. Intuitively, it seems like a reasonable guess that these three
characters are all quite unusual and it's unlikely that Unicode would add more
characters that would make the sets differ in the future.

Second, we could store two small sets: additions and removals relative to the base set.
However, using these would involve a heap allocation for the computed actual set.
In a multi-process architecture when using crabbake, having each process carry its
own heap allocation for the lifetime of the process would be worse than making the
static data larger.

The third option would be to make the run-time lookup from three sets: the base,
additions, and removals. Since the additions and removals would be empty sets for NFD,
chances are that this would be less branchy in the NFD case than the first option,
especially if we made `UnicodeSet` be able to be shallow-copied in a way that borrows
its `ZeroVec` instead of making an owning copy of of the wrapped `ZeroVec`.

As for the trie, the alternative would be to have two levels of tries: supplementary
and base where base would be the NFD trie and the default (zero) value from the
supplementary trie would mean falling back to the base trie.

Compared to the current expansion supplement arrangement, this would differ in terms
of run-time cost. In the NFD case, the expansion supplement adds one branch for the
case where a character has a non-self supplementary-plane decomposition, which is
rare. In contrast, having the ability to make a lookup for supplementary trie would
mean having to branch on the presence of the supplementary trie for each starter.
For the normalization forms that would use the supplementary trie, each starter
would go through two full trie lookups instead of one.

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
