// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{Criterion, black_box, criterion_group, criterion_main};

use icu_segmenter::LineSegmenter;
#[cfg(feature = "unstable")]
use icu_segmenter::WordSegmenter;
#[cfg(feature = "unstable")]
use icu_segmenter::neo::{LineSegmenter as NeoLineSegmenter, WordSegmenter as NeoWordSegmenter};
use icu_segmenter::options::LineBreakOptions;
use icu_segmenter::options::LineBreakStrictness;
use icu_segmenter::options::LineBreakWordOption;

// Example is MIT license.
const TEST_STR_EN: &str = "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
const TEST_STR_TH: &str =
    "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย";

#[cfg(feature = "unstable")]
const TEST_STR_JA: &str =
    "こんにちは世界こんにちは世界こんにちは世界こんにちは世界こんにちは世界こんにちは世界";
#[cfg(feature = "unstable")]
const TEST_STR_HAN: &str = "中文分词需要词典中文分词需要词典中文分词需要词典中文分词需要词典";
#[cfg(feature = "unstable")]
const TEST_STR_TH_JA: &str = "ภาษาไทยこんにちは世界ภาษาไทยこんにちは世界ภาษาไทยこんにちは世界";
#[cfg(feature = "unstable")]
const TEST_STR_TH_HAN: &str = "ภาษาไทย龟山岛ภาษาไทย龟山岛ภาษาไทย龟山岛";
#[cfg(feature = "unstable")]
const TEST_STR_EN_TH: &str = "Thai text ภาษาไทยภาษาไทย mixed with non-complex Latin text";
#[cfg(feature = "unstable")]
const TEST_STR_LONG_MIXED: &str = include_str!("../tests/testdata/SegmenterBenchMixed.txt");

#[cfg(feature = "unstable")]
const COMPARISON_CASES: &[TextCase] = &[
    TextCase {
        name: "english",
        text: TEST_STR_EN,
    },
    TextCase {
        name: "thai",
        text: TEST_STR_TH,
    },
    TextCase {
        name: "japanese",
        text: TEST_STR_JA,
    },
    TextCase {
        name: "han",
        text: TEST_STR_HAN,
    },
    TextCase {
        name: "thai_japanese",
        text: TEST_STR_TH_JA,
    },
    TextCase {
        name: "thai_han",
        text: TEST_STR_TH_HAN,
    },
    TextCase {
        name: "english_thai",
        text: TEST_STR_EN_TH,
    },
    TextCase {
        name: "long_mixed",
        text: TEST_STR_LONG_MIXED,
    },
];

#[cfg(feature = "unstable")]
#[derive(Clone, Copy)]
struct TextCase {
    name: &'static str,
    text: &'static str,
}

#[cfg(feature = "unstable")]
// Consume breakpoint values without collecting them into a Vec.
fn consume_breakpoints(iter: impl Iterator<Item = usize>) -> usize {
    black_box(iter.fold(0usize, |checksum, breakpoint| {
        checksum.wrapping_mul(31).wrapping_add(breakpoint)
    }))
}

#[cfg(feature = "unstable")]
macro_rules! bench_dictionary_str {
    ($group:expr, $published:expr, $neo:expr) => {{
        let published = $published;
        let neo = $neo;

        for case in COMPARISON_CASES {
            $group.bench_function(format!("published/dictionary/{}", case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(black_box(published).segment_str(black_box(case.text)))
                })
            });

            $group.bench_function(format!("neo/dictionary/{}", case.name), |b| {
                b.iter(|| consume_breakpoints(black_box(neo).segment_str(black_box(case.text))))
            });
        }
    }};
}

#[cfg(feature = "unstable")]
macro_rules! bench_dictionary_utf8 {
    ($group:expr, $published:expr, $neo:expr) => {{
        let published = $published;
        let neo = $neo;

        for case in COMPARISON_CASES {
            $group.bench_function(format!("published/dictionary/{}", case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(
                        black_box(published).segment_utf8(black_box(case.text).as_bytes()),
                    )
                })
            });

            $group.bench_function(format!("neo/dictionary/{}", case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(
                        black_box(neo).segment_utf8(black_box(case.text).as_bytes()),
                    )
                })
            });
        }
    }};
}

#[cfg(feature = "unstable")]
macro_rules! bench_dictionary_utf16 {
    ($group:expr, $published:expr, $neo:expr) => {{
        let published = $published;
        let neo = $neo;

        for case in COMPARISON_CASES {
            let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

            $group.bench_function(format!("published/dictionary/{}", case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(black_box(published).segment_utf16(black_box(&utf16)))
                })
            });

            $group.bench_function(format!("neo/dictionary/{}", case.name), |b| {
                b.iter(|| consume_breakpoints(black_box(neo).segment_utf16(black_box(&utf16))))
            });
        }
    }};
}

fn line_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/Latin1");

    let segmenter = LineSegmenter::new_dictionary(Default::default());

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let segmenter_css = LineSegmenter::new_dictionary(options);

    group.bench_function("En", |b| {
        b.iter(|| {
            black_box(&segmenter)
                .segment_latin1(black_box(TEST_STR_EN).as_bytes())
                .count()
        })
    });

    group.bench_function("En CSS", |b| {
        b.iter(|| {
            black_box(&segmenter_css)
                .segment_latin1(black_box(TEST_STR_EN).as_bytes())
                .count()
        })
    });
}

fn line_break_iter_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF8");

    let segmenter_auto = LineSegmenter::new_auto(Default::default());
    let segmenter_lstm = LineSegmenter::new_lstm(Default::default());
    let segmenter_dictionary = LineSegmenter::new_dictionary(Default::default());

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let segmenter_css_dictionary = LineSegmenter::new_dictionary(options);

    // No need to test "auto", "lstm", or "dictionary" constructor variants since English uses only
    // UAX14 rules for line breaking.
    group.bench_function("En", |b| {
        b.iter(|| {
            black_box(&segmenter_dictionary)
                .segment_str(black_box(TEST_STR_EN))
                .count()
        })
    });

    group.bench_function("En CSS", |b| {
        b.iter(|| {
            black_box(&segmenter_css_dictionary)
                .segment_str(black_box(TEST_STR_EN))
                .count()
        })
    });

    let segmenters = [
        (&segmenter_auto, "auto"),
        (&segmenter_lstm, "lstm"),
        (&segmenter_dictionary, "dictionary"),
    ];
    for (segmenter, variant) in segmenters {
        group.bench_function("Th/".to_string() + variant, |b| {
            b.iter(|| {
                black_box(&segmenter)
                    .segment_str(black_box(TEST_STR_TH))
                    .count()
            })
        });
    }
}

fn line_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF16");

    let utf16_en: Vec<u16> = TEST_STR_EN.encode_utf16().collect();
    let utf16_th: Vec<u16> = TEST_STR_TH.encode_utf16().collect();

    let segmenter_auto = LineSegmenter::new_auto(Default::default());
    let segmenter_lstm = LineSegmenter::new_lstm(Default::default());
    let segmenter_dictionary = LineSegmenter::new_dictionary(Default::default());

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let segmenter_css_dictionary = LineSegmenter::new_dictionary(options);

    // No need to test "auto", "lstm", or "dictionary" constructor variants since English uses only
    // UAX14 rules for line breaking.
    group.bench_function("En", |b| {
        b.iter(|| {
            black_box(&segmenter_dictionary)
                .segment_utf16(black_box(&utf16_en))
                .count()
        })
    });

    group.bench_function("En CSS", |b| {
        b.iter(|| {
            black_box(&segmenter_css_dictionary)
                .segment_utf16(black_box(&utf16_en))
                .count()
        })
    });

    let segmenters = [
        (&segmenter_auto, "auto"),
        (&segmenter_lstm, "lstm"),
        (&segmenter_dictionary, "dictionary"),
    ];
    for (segmenter, variant) in segmenters {
        group.bench_function("Th/".to_string() + variant, |b| {
            b.iter(|| {
                black_box(&segmenter)
                    .segment_utf16(black_box(&utf16_th))
                    .count()
            })
        });
    }
}

#[cfg(feature = "unstable")]
fn line_break_comparison_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter Comparison/Line/segment_str");

    bench_dictionary_str!(
        group,
        LineSegmenter::new_dictionary(Default::default()),
        NeoLineSegmenter::new_dictionary(Default::default())
    );
}

#[cfg(feature = "unstable")]
fn line_break_comparison_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter Comparison/Line/segment_utf8");

    bench_dictionary_utf8!(
        group,
        LineSegmenter::new_dictionary(Default::default()),
        NeoLineSegmenter::new_dictionary(Default::default())
    );
}

#[cfg(feature = "unstable")]
fn line_break_comparison_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter Comparison/Line/segment_utf16");

    bench_dictionary_utf16!(
        group,
        LineSegmenter::new_dictionary(Default::default()),
        NeoLineSegmenter::new_dictionary(Default::default())
    );
}

#[cfg(feature = "unstable")]
fn word_break_comparison_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter Comparison/Word/segment_str");

    bench_dictionary_str!(
        group,
        WordSegmenter::new_dictionary(Default::default()),
        NeoWordSegmenter::new_dictionary(Default::default())
    );
}

#[cfg(feature = "unstable")]
fn word_break_comparison_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter Comparison/Word/segment_utf8");

    bench_dictionary_utf8!(
        group,
        WordSegmenter::new_dictionary(Default::default()),
        NeoWordSegmenter::new_dictionary(Default::default())
    );
}

#[cfg(feature = "unstable")]
fn word_break_comparison_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter Comparison/Word/segment_utf16");

    bench_dictionary_utf16!(
        group,
        WordSegmenter::new_dictionary(Default::default()),
        NeoWordSegmenter::new_dictionary(Default::default())
    );
}

#[cfg(not(feature = "unstable"))]
criterion_group!(
    benches,
    line_break_iter_latin1,
    line_break_iter_utf8,
    line_break_iter_utf16
);

#[cfg(feature = "unstable")]
criterion_group!(
    benches,
    line_break_iter_latin1,
    line_break_iter_utf8,
    line_break_iter_utf16,
    line_break_comparison_str,
    line_break_comparison_utf8,
    line_break_comparison_utf16,
    word_break_comparison_str,
    word_break_comparison_utf8,
    word_break_comparison_utf16
);
criterion_main!(benches);
