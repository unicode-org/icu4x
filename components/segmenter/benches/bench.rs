// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::time::Duration;

use icu_segmenter::GraphemeClusterSegmenter;
use icu_segmenter::LineSegmenter;
use icu_segmenter::WordSegmenter;
use icu_segmenter::options::LineBreakOptions;
use icu_segmenter::options::LineBreakStrictness;
use icu_segmenter::options::LineBreakWordOption;

// Example is MIT license.
const TEST_STR_EN: &str = "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
const TEST_STR_TH: &str =
    "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย";
const TEST_STR_JA: &str =
    "こんにちは世界こんにちは世界こんにちは世界こんにちは世界こんにちは世界こんにちは世界";
const TEST_STR_LONG_MIXED: &str = include_str!("../tests/testdata/SegmenterBenchMixed.txt");
// Spaces terminate complex-script runs, so these stress patterns are separator-free.
const TEST_STR_CONTINUOUS_TH: &str = "ภาษาไทย";
const TEST_STR_CONTINUOUS_HAN: &str = "漢字";

const COMPLEX_CACHE_DEFAULT_SIZES: &[usize] = &[16 * 1024];
const COMPLEX_CACHE_LARGE_SIZES: &[usize] = &[1024, 16 * 1024, 256 * 1024, 1024 * 1024];

const LINE_CASES: &[TextCase] = &[
    TextCase {
        name: "En",
        text: TEST_STR_EN,
    },
    TextCase {
        name: "En CSS",
        text: TEST_STR_EN,
    },
    TextCase {
        name: "Th",
        text: TEST_STR_TH,
    },
    TextCase {
        name: "Mixed",
        text: TEST_STR_LONG_MIXED,
    },
];

const WORD_CASES: &[TextCase] = &[
    TextCase {
        name: "En",
        text: TEST_STR_EN,
    },
    TextCase {
        name: "Ja",
        text: TEST_STR_JA,
    },
    TextCase {
        name: "Mixed",
        text: TEST_STR_LONG_MIXED,
    },
];

const GRAPHEME_CASES: &[TextCase] = &[TextCase {
    name: "Mixed",
    text: TEST_STR_LONG_MIXED,
}];

#[derive(Clone, Copy)]
struct TextCase {
    name: &'static str,
    text: &'static str,
}

fn line_segmenters(
    options: LineBreakOptions<'_>,
) -> impl IntoIterator<Item = (icu_segmenter::LineSegmenterBorrowed<'static>, &'static str)> {
    [
        (LineSegmenter::new_for_non_complex_scripts(options), ""),
        (LineSegmenter::new_lstm(options), "/lstm"),
        (LineSegmenter::new_dictionary(options), "/dictionary"),
        #[cfg(feature = "unstable")]
        (
            LineSegmenter::new_neo_for_non_complex_scripts(options),
            "/neo",
        ),
        #[cfg(feature = "unstable")]
        (
            {
                let mut segmenter = LineSegmenter::new_neo_for_non_complex_scripts(options);
                segmenter.load_lstm();
                segmenter
            },
            "/neo/lstm",
        ),
        #[cfg(feature = "unstable")]
        (
            {
                let mut segmenter = LineSegmenter::new_neo_for_non_complex_scripts(options);
                segmenter.load_dictionary();
                segmenter
            },
            "/neo/dictionary",
        ),
    ]
}

fn word_segmenters()
-> impl IntoIterator<Item = (icu_segmenter::WordSegmenterBorrowed<'static>, &'static str)> {
    [
        (
            WordSegmenter::new_for_non_complex_scripts(Default::default()),
            "",
        ),
        (
            WordSegmenter::new_dictionary(Default::default()),
            "/dictionary",
        ),
        #[cfg(feature = "unstable")]
        (
            WordSegmenter::new_neo_for_non_complex_scripts(Default::default()),
            "/neo",
        ),
        #[cfg(feature = "unstable")]
        (
            {
                let mut segmenter =
                    WordSegmenter::new_neo_for_non_complex_scripts(Default::default());
                segmenter.load_dictionary();
                segmenter
            },
            "/neo/dictionary",
        ),
    ]
}

fn grapheme_segmenters() -> impl IntoIterator<
    Item = (
        icu_segmenter::GraphemeClusterSegmenterBorrowed<'static>,
        &'static str,
    ),
> {
    [
        (GraphemeClusterSegmenter::new(), ""),
        #[cfg(feature = "unstable")]
        (GraphemeClusterSegmenter::new_neo(), "/neo"),
    ]
}

fn repeat_to_char_count(pattern: &str, count: usize) -> String {
    pattern.chars().cycle().take(count).collect()
}

fn complex_cache(c: &mut Criterion) {
    // The pre-optimization 1M word case takes about 20 seconds per iteration.
    let large = std::env::var_os("ICU4X_SEGMENTER_BENCH_LARGE").is_some();
    let sizes = if large {
        COMPLEX_CACHE_LARGE_SIZES
    } else {
        COMPLEX_CACHE_DEFAULT_SIZES
    };
    let prefix_size = if large { 1024 * 1024 } else { 16 * 1024 };

    let word_segmenter = WordSegmenter::new_dictionary(Default::default());
    let line_segmenter = LineSegmenter::new_dictionary(Default::default());
    let mut all_group = c.benchmark_group("Complex Break Cache/UTF8/All");
    if large {
        all_group.sample_size(10);
        all_group.warm_up_time(Duration::from_secs(1));
        all_group.measurement_time(Duration::from_secs(2));
    }

    for &char_count in sizes {
        let han = repeat_to_char_count(TEST_STR_CONTINUOUS_HAN, char_count);
        let thai = repeat_to_char_count(TEST_STR_CONTINUOUS_TH, char_count);
        all_group.throughput(Throughput::Elements(char_count as u64));

        all_group.bench_with_input(
            BenchmarkId::new("Word/Dictionary/ContinuousHan", char_count),
            &han,
            |b, text| {
                b.iter(|| {
                    black_box(&word_segmenter)
                        .segment_str(black_box(text.as_str()))
                        .count()
                })
            },
        );
        all_group.bench_with_input(
            BenchmarkId::new("Line/Dictionary/ContinuousThai", char_count),
            &thai,
            |b, text| {
                b.iter(|| {
                    black_box(&line_segmenter)
                        .segment_str(black_box(text.as_str()))
                        .count()
                })
            },
        );
    }
    all_group.finish();

    let han = repeat_to_char_count(TEST_STR_CONTINUOUS_HAN, prefix_size);
    let thai = repeat_to_char_count(TEST_STR_CONTINUOUS_TH, prefix_size);
    let mut prefix_group = c.benchmark_group("Complex Break Cache/UTF8/Prefix10");

    prefix_group.bench_with_input(
        BenchmarkId::new("Word/Dictionary/ContinuousHan", prefix_size),
        &han,
        |b, text| {
            b.iter(|| {
                black_box(&word_segmenter)
                    .segment_str(black_box(text.as_str()))
                    .take(10)
                    .count()
            })
        },
    );
    prefix_group.bench_with_input(
        BenchmarkId::new("Line/Dictionary/ContinuousThai", prefix_size),
        &thai,
        |b, text| {
            b.iter(|| {
                black_box(&line_segmenter)
                    .segment_str(black_box(text.as_str()))
                    .take(10)
                    .count()
            })
        },
    );
    prefix_group.finish();
}

fn line_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/Latin1");

    for case in LINE_CASES {
        if !case.text.is_ascii() {
            continue;
        }

        let mut options = LineBreakOptions::default();
        if case.name.contains("CSS") {
            options.strictness = Some(LineBreakStrictness::Anywhere);
            options.word_option = Some(LineBreakWordOption::BreakAll);
        }

        for (segmenter, variant) in line_segmenters(options) {
            if variant.contains("lstm") || variant.contains("dictionary") {
                // these don't add anything, we only compare to auto to determine the potential complex overhead
                continue;
            }
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_latin1(black_box(case.text.as_bytes()))
                        .count()
                })
            });
        }
    }
}

fn line_break_iter_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF8");

    for case in LINE_CASES {
        let mut options = LineBreakOptions::default();
        if case.name.contains("CSS") {
            options.strictness = Some(LineBreakStrictness::Anywhere);
            options.word_option = Some(LineBreakWordOption::BreakAll);
        }

        for (segmenter, variant) in line_segmenters(options) {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_str(black_box(case.text))
                        .count()
                })
            });
        }
    }
}

fn line_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF16");

    for case in LINE_CASES {
        let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

        let mut options = LineBreakOptions::default();
        if case.name.contains("CSS") {
            options.strictness = Some(LineBreakStrictness::Anywhere);
            options.word_option = Some(LineBreakWordOption::BreakAll);
        }

        for (segmenter, variant) in line_segmenters(options) {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_utf16(black_box(&utf16))
                        .count()
                })
            });
        }
    }
}

fn grapheme_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grapheme Break/Latin1");

    for case in GRAPHEME_CASES {
        if !case.text.is_ascii() {
            continue;
        }

        for (segmenter, variant) in grapheme_segmenters() {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_latin1(black_box(case.text.as_bytes()))
                        .count()
                })
            });
        }
    }
}

fn grapheme_break_iter_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grapheme Break/UTF8");

    for case in GRAPHEME_CASES {
        for (segmenter, variant) in grapheme_segmenters() {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_str(black_box(case.text))
                        .count()
                })
            });
        }
    }
}

fn grapheme_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grapheme Break/UTF16");

    for case in GRAPHEME_CASES {
        let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

        for (segmenter, variant) in grapheme_segmenters() {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_utf16(black_box(&utf16))
                        .count()
                })
            });
        }
    }
}

fn word_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Word Break/Latin1");

    for case in WORD_CASES {
        if !case.text.is_ascii() {
            continue;
        }

        for (segmenter, variant) in word_segmenters() {
            if variant.contains("lstm") || variant.contains("dictionary") {
                // these don't add anything, we only compare to auto to determine the potential complex overhead
                continue;
            }
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_latin1(black_box(case.text.as_bytes()))
                        .count()
                })
            });
        }
    }
}

fn word_break_iter_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Word Break/UTF8");

    for case in WORD_CASES {
        for (segmenter, variant) in word_segmenters() {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_str(black_box(case.text))
                        .count()
                })
            });
        }
    }
}

fn word_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Word Break/UTF16");

    for case in WORD_CASES {
        let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

        for (segmenter, variant) in word_segmenters() {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_utf16(black_box(&utf16))
                        .count()
                })
            });
        }
    }
}

criterion_group!(
    benches,
    line_break_iter_latin1,
    line_break_iter_str,
    line_break_iter_utf16,
    grapheme_break_iter_latin1,
    grapheme_break_iter_str,
    grapheme_break_iter_utf16,
    word_break_iter_latin1,
    word_break_iter_str,
    word_break_iter_utf16,
    complex_cache,
);
criterion_main!(benches);
