// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Criterion benches for `icu_experimental::messageformat`.
//!
//! Covers three representative workloads:
//!
//! 1. **tiny** — plain-text message (zero placeholders).
//! 2. **medium** — simple message with one `$var :string` placeholder.
//! 3. **selector** — `.match` message with one `:integer` selector and
//!    three plural variants.
//!
//! Each benchmark measures the `parse + validate + build + format_to_string`
//! pipeline end-to-end.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_experimental::messageformat::{MessageFormatter, OwnedInputs};
use icu_locale_core::locale;

fn bench_tiny(c: &mut Criterion) {
    c.bench_function("messageformat::tiny", |b| {
        b.iter(|| {
            let f = MessageFormatter::builder()
                .source(black_box("Hello, world!"))
                .locale(locale!("en"))
                .build()
                .unwrap();
            let inputs: &[(&str, &str)] = &[];
            let (out, _) = f.format_to_string(&inputs);
            black_box(out);
        });
    });
}

fn bench_medium(c: &mut Criterion) {
    c.bench_function("messageformat::medium", |b| {
        b.iter(|| {
            let f = MessageFormatter::builder()
                .source(black_box(
                    "Hello, {$user :string}! You have {$count :integer} new messages.",
                ))
                .locale(locale!("en"))
                .build()
                .unwrap();
            let inputs = OwnedInputs::new()
                .with_str("user", "Ada")
                .with_number("count", 42_i64);
            let (out, _) = f.format_to_string(&inputs);
            black_box(out);
        });
    });
}

fn bench_selector(c: &mut Criterion) {
    let src = ".input {$count :integer}\n\
               .match $count\n\
               0 {{You have no items.}}\n\
               one {{You have one item.}}\n\
               * {{You have {$count} items.}}";
    c.bench_function("messageformat::selector", |b| {
        b.iter(|| {
            let f = MessageFormatter::builder()
                .source(black_box(src))
                .locale(locale!("en"))
                .build()
                .unwrap();
            let inputs = OwnedInputs::new().with_number("count", 5_i64);
            let (out, _) = f.format_to_string(&inputs);
            black_box(out);
        });
    });
}

criterion_group!(benches, bench_tiny, bench_medium, bench_selector);
criterion_main!(benches);
