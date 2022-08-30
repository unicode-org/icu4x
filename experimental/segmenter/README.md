# icu_segmenter [![crates.io](https://img.shields.io/crates/v/icu_segmenter)](https://crates.io/crates/icu_segmenter)

\[Experimental\] Segment strings by lines, graphemes, word, and sentences.

This module is published as its own crate ([`icu_segmenter`](https://docs.rs/icu_segmenter/latest/icu_segmenter/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

This module contains segmenter implementation for the following rules.

- Line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.
- Grapheme cluster breaker, word breaker, and sentence breaker that are compatible with
  [Unicode Standard Annex #29][UAX29].

[UAX14]: https://www.unicode.org/reports/tr14/
[UAX29]: https://www.unicode.org/reports/tr29/

## Examples

### Line Break

Segment a string with default options:

```rust
use icu_segmenter::LineBreakSegmenter;

let provider = icu_testdata::get_provider();
let segmenter = LineBreakSegmenter::try_new(&provider).expect("Data exists");

let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
assert_eq!(&breakpoints, &[6, 11]);
```

See [`LineBreakSegmenter`] for more examples.

### Grapheme Cluster Break

See [`GraphemeClusterBreakSegmenter`] for examples.

### Word Break

Segment a string:

```rust
use icu_segmenter::WordBreakSegmenter;
let provider = icu_testdata::get_provider();
let segmenter = WordBreakSegmenter::try_new(&provider).expect("Data exists");

let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
assert_eq!(&breakpoints, &[0, 5, 6, 11]);
```

See [`WordBreakSegmenter`] for more examples.

### Sentence Break

See [`SentenceBreakSegmenter`] for examples.

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
