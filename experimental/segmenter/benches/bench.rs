// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{criterion_group, criterion_main, Criterion};

use icu_segmenter::LineBreakIterator;
use icu_segmenter::LineBreakIteratorLatin1;
use icu_segmenter::LineBreakIteratorUtf16;
use icu_segmenter::LineBreakRule;
use icu_segmenter::WordBreakRule;

// Example is MIT license.
const TEST_STR: &str = "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
const TEST_STR2: &str =
    "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย";

fn line_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/Latin1");

    group.bench_function("En", |b| {
        b.iter(|| LineBreakIteratorLatin1::new(TEST_STR.as_bytes()).count())
    });

    group.bench_function("En CSS", |b| {
        b.iter(|| {
            LineBreakIteratorLatin1::new_with_break_rule(
                TEST_STR.as_bytes(),
                LineBreakRule::Anywhere,
                WordBreakRule::BreakAll,
                true,
            )
            .count()
        })
    });
}

fn line_break_iter_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF8");

    group.bench_function("En", |b| {
        b.iter(|| LineBreakIterator::new(TEST_STR).count())
    });

    group.bench_function("Th", |b| {
        b.iter(|| LineBreakIterator::new(TEST_STR2).count())
    });
}

fn line_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF16");

    let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
    group.bench_function("En", |b| {
        b.iter(|| LineBreakIteratorUtf16::new(&utf16).count())
    });

    group.bench_function("En CSS", |b| {
        b.iter(|| {
            LineBreakIteratorUtf16::new_with_break_rule(
                &utf16,
                LineBreakRule::Anywhere,
                WordBreakRule::BreakAll,
                true,
            )
            .count()
        })
    });

    let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
    group.bench_function("Th", |b| {
        b.iter(|| {
            LineBreakIteratorUtf16::new_with_break_rule(
                &utf16,
                LineBreakRule::Anywhere,
                WordBreakRule::BreakAll,
                true,
            )
            .count()
        })
    });
}

criterion_group!(
    benches,
    line_break_iter_latin1,
    line_break_iter_utf8,
    line_break_iter_utf16
);
criterion_main!(benches);
