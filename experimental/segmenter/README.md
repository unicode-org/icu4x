![](https://github.com/makotokato/uax14_rs/workflows/CI/badge.svg)

# uax14_rs

A line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.

[UAX14]: http://www.unicode.org/reports/tr14/

```rust
use uax14_rs::LineBreakIterator;

fn main () {
    let mut iter = LineBreakIterator::new("Hello World");
    let result: Vec<usize> = iter.collect();
    println!("{:?}", result);
}
```

With CSS property.
```rust
use uax14_rs::{LineBreakIterator, LineBreakRule, WordBreakRule};

fn main() {
    let iter = LineBreakIterator::new_with_break_rule(
        "Hello World",
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.collect();
    println!("{:?}", result);
}
```

Use Latin 1 string for C binding and etc.

```rust
use uax14_rs::LineBreakIteratorLatin1;

fn main () {
    let s = "Hello World";
    let iter = LineBreakIteratorLatin1::new(s.as_bytes());
    let result: Vec<usize> = iter.collect();
    println!("{:?}", result);
}
```

## Generating property table

Copy the following files to tools directory. Then run `python ./generate_properties.py` in `tools` directory. Machine generated files are moved to `src` directory.
- <https://www.unicode.org/Public/UCD/latest/ucd/LineBreak.txt>
- <https://www.unicode.org/Public/UCD/latest/ucd/EastAsianWidth.txt>
