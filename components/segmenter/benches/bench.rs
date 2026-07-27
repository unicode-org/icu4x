// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{Criterion, black_box, criterion_group, criterion_main};

use icu_segmenter::LineSegmenter;
use icu_segmenter::WordSegmenter;
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
        name: "Ja",
        text: TEST_STR_JA,
    },
    TextCase {
        name: "Han",
        text: TEST_STR_HAN,
    },
    TextCase {
        name: "Th_Ja",
        text: TEST_STR_TH_JA,
    },
    TextCase {
        name: "Th_Han",
        text: TEST_STR_TH_HAN,
    },
    TextCase {
        name: "En_Th",
        text: TEST_STR_EN_TH,
    },
    TextCase {
        name: "Mixed",
        text: TEST_STR_LONG_MIXED,
    },
];

#[cfg(feature = "unstable")]
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
        (LineSegmenter::new_auto(options), "/auto"),
        (LineSegmenter::new_lstm(options), "/lstm"),
        (LineSegmenter::new_dictionary(options), "/dictionary"),
        #[cfg(feature = "unstable")]
        (
            LineSegmenter::new_neo_for_non_complex_scripts(options),
            "/neo",
        ),
        #[cfg(feature = "unstable")]
        (LineSegmenter::new_neo_auto(options), "/neo/auto"),
        #[cfg(feature = "unstable")]
        (LineSegmenter::new_neo_lstm(options), "/neo/lstm"),
        #[cfg(feature = "unstable")]
        (
            LineSegmenter::new_neo_dictionary(options),
            "/neo/dictionary",
        ),
    ]
}

fn word_segmenters(
    options: icu_segmenter::options::WordBreakInvariantOptions,
) -> impl IntoIterator<Item = (icu_segmenter::WordSegmenterBorrowed<'static>, &'static str)> {
    [
        (WordSegmenter::new_for_non_complex_scripts(options), ""),
        (WordSegmenter::new_auto(options), "/auto"),
        (WordSegmenter::new_lstm(options), "/lstm"),
        (WordSegmenter::new_dictionary(options), "/dictionary"),
        #[cfg(feature = "unstable")]
        (
            WordSegmenter::new_neo_for_non_complex_scripts(options),
            "/neo",
        ),
        #[cfg(feature = "unstable")]
        (WordSegmenter::new_neo_auto(options), "/neo/auto"),
        #[cfg(feature = "unstable")]
        (WordSegmenter::new_neo_lstm(options), "/neo/lstm"),
        #[cfg(feature = "unstable")]
        (
            WordSegmenter::new_neo_dictionary(options),
            "/neo/dictionary",
        ),
    ]
}

fn line_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/Latin1");

    for case in COMPARISON_CASES {
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

    for case in COMPARISON_CASES {
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

fn line_break_iter_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/Potential UTF8");

    for case in COMPARISON_CASES {
        let mut options = LineBreakOptions::default();
        if case.name.contains("CSS") {
            options.strictness = Some(LineBreakStrictness::Anywhere);
            options.word_option = Some(LineBreakWordOption::BreakAll);
        }

        for (segmenter, variant) in line_segmenters(options) {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_utf8(black_box(case.text.as_bytes()))
                        .count()
                })
            });
        }
    }
}

fn line_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Line Break/UTF16");

    for case in COMPARISON_CASES {
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

fn word_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Word Break/Latin1");

    for case in COMPARISON_CASES {
        if !case.text.is_ascii() {
            continue;
        }

        for (segmenter, variant) in word_segmenters(Default::default()) {
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

    for case in COMPARISON_CASES {
        for (segmenter, variant) in word_segmenters(Default::default()) {
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

fn word_break_iter_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Word Break/Potential UTF8");

    for case in COMPARISON_CASES {
        for (segmenter, variant) in word_segmenters(Default::default()) {
            group.bench_function(format!("{}{}", case.name, variant), |b| {
                b.iter(|| {
                    black_box(&segmenter)
                        .segment_utf8(black_box(case.text.as_bytes()))
                        .count()
                })
            });
        }
    }
}

fn word_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Word Break/UTF16");

    for case in COMPARISON_CASES {
        let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

        for (segmenter, variant) in word_segmenters(Default::default()) {
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
    line_break_iter_utf8,
    line_break_iter_utf16,
    word_break_iter_latin1,
    word_break_iter_str,
    word_break_iter_utf8,
    word_break_iter_utf16,
);
criterion_main!(benches);
