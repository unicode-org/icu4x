# icu_segmenter [![crates.io](https://img.shields.io/crates/v/icu_segmenter)](https://crates.io/crates/icu_segmenter)

A segmenter implementation for the following rules.

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

let provider = icu_provider::inv::InvariantDataProvider;
let segmenter = LineBreakSegmenter::try_new(&provider)
    .expect("Data exists");

let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
assert_eq!(&breakpoints, &[6, 11]);
```

Segment a string with CSS option overrides:

```rust
use icu_segmenter::{LineBreakSegmenter, LineBreakOptions, LineBreakRule, WordBreakRule};

let mut options = LineBreakOptions::default();
options.line_break_rule = LineBreakRule::Strict;
options.word_break_rule = WordBreakRule::BreakAll;
options.ja_zh = false;
let provider = icu_provider::inv::InvariantDataProvider;
let segmenter = LineBreakSegmenter::try_new_with_options(&provider, options)
    .expect("Data exists");

let breakpoints: Vec<usize> = segmenter.segment_str("Hello World").collect();
assert_eq!(&breakpoints, &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
```

Segment a Latin1 byte string:

```rust
use icu_segmenter::LineBreakSegmenter;

let provider = icu_provider::inv::InvariantDataProvider;
let segmenter = LineBreakSegmenter::try_new(&provider)
    .expect("Data exists");

let breakpoints: Vec<usize> = segmenter.segment_latin1(b"Hello World").collect();
assert_eq!(&breakpoints, &[6, 11]);
```

### Grapheme Cluster Break

Segment a string:

```rust
use icu_segmenter::GraphemeClusterBreakIterator;

let breakpoints: Vec<usize> = GraphemeClusterBreakIterator::new("Hello 🗺").collect();
assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 10]); // World Map (U+1F5FA) is encoded in four bytes in UTF-8.
```

Segment a Latin1 byte string:

```rust
use icu_segmenter::GraphemeClusterBreakIteratorLatin1;

let breakpoints: Vec<usize> = GraphemeClusterBreakIteratorLatin1::new(b"Hello World").collect();
assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
```

### Word Break

Segment a string:

```rust
use icu_segmenter::WordBreakIterator;

let breakpoints: Vec<usize> = WordBreakIterator::new("Hello World").collect();
assert_eq!(&breakpoints, &[0, 5, 6, 11]);
```

Segment a Latin1 byte string:

```rust
use icu_segmenter::WordBreakIteratorLatin1;

let breakpoints: Vec<usize> = WordBreakIteratorLatin1::new(b"Hello World").collect();
assert_eq!(&breakpoints, &[0, 5, 6, 11]);
```

### Sentence Break

Segment a string:

```rust
use icu_segmenter::SentenceBreakIterator;

let breakpoints: Vec<usize> = SentenceBreakIterator::new("Hello World").collect();
assert_eq!(&breakpoints, &[0, 11]);
```

Segment a Latin1 byte string:

```rust
use icu_segmenter::SentenceBreakIteratorLatin1;

let breakpoints: Vec<usize> = SentenceBreakIteratorLatin1::new(b"Hello World").collect();
assert_eq!(&breakpoints, &[0, 11]);
```

## Generating property table

Copy the following files to `tools` directory. Then run `./generate_properties.py` in `tools` directory (requires Python 3.8+). Machine generated files are moved to `src` directory.
- <https://www.unicode.org/Public/UCD/latest/ucd/LineBreak.txt>
- <https://www.unicode.org/Public/UCD/latest/ucd/EastAsianWidth.txt>
- <https://www.unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt>

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
