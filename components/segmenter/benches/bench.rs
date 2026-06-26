// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[cfg(feature = "unstable")]
use icu_segmenter::neo::{LineSegmenter as NeoLineSegmenter, WordSegmenter as NeoWordSegmenter};
use icu_segmenter::options::LineBreakOptions;
use icu_segmenter::options::LineBreakStrictness;
use icu_segmenter::options::LineBreakWordOption;
use icu_segmenter::{LineSegmenter, WordSegmenter};

const TEXT_ENGLISH: &str = "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
const TEXT_THAI: &str =
    "ภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทยภาษาไทย";
const TEXT_JAPANESE: &str =
    "こんにちは世界こんにちは世界こんにちは世界こんにちは世界こんにちは世界こんにちは世界";
const TEXT_HAN: &str = "中文分词需要词典中文分词需要词典中文分词需要词典中文分词需要词典";
const TEXT_THAI_JAPANESE: &str = "ภาษาไทยこんにちは世界ภาษาไทยこんにちは世界ภาษาไทยこんにちは世界";
const TEXT_THAI_HAN: &str = "ภาษาไทย龟山岛ภาษาไทย龟山岛ภาษาไทย龟山岛";

// Keep these case names stable: follow-up optimization PRs use them to compare
// the same scripts and mixed complex-language traversals across implementations.
const TEXT_CASES: &[TextCase] = &[
    TextCase {
        name: "english",
        text: TEXT_ENGLISH,
    },
    TextCase {
        name: "thai",
        text: TEXT_THAI,
    },
    TextCase {
        name: "japanese",
        text: TEXT_JAPANESE,
    },
    TextCase {
        name: "han",
        text: TEXT_HAN,
    },
    TextCase {
        name: "thai_japanese",
        text: TEXT_THAI_JAPANESE,
    },
    TextCase {
        name: "thai_han",
        text: TEXT_THAI_HAN,
    },
];

#[derive(Clone, Copy)]
struct TextCase {
    name: &'static str,
    text: &'static str,
}

fn consume_breakpoints(iter: impl Iterator<Item = usize>) -> usize {
    black_box(iter.fold(0usize, |checksum, breakpoint| {
        checksum.wrapping_mul(31).wrapping_add(breakpoint)
    }))
}

macro_rules! bench_str {
    ($group:expr, $variant:literal, $published:expr, $neo:expr) => {{
        let published = $published;

        for case in TEXT_CASES {
            $group.bench_function(format!("published_{}/{}", $variant, case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(black_box(published).segment_str(black_box(case.text)))
                })
            });
        }

        #[cfg(feature = "unstable")]
        {
            let neo = $neo;

            for case in TEXT_CASES {
                $group.bench_function(format!("neo_{}/{}", $variant, case.name), |b| {
                    b.iter(|| consume_breakpoints(black_box(neo).segment_str(black_box(case.text))))
                });
            }
        }
    }};
}

macro_rules! bench_utf8 {
    ($group:expr, $variant:literal, $published:expr, $neo:expr) => {{
        let published = $published;

        for case in TEXT_CASES {
            $group.bench_function(format!("published_{}/{}", $variant, case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(
                        black_box(published).segment_utf8(black_box(case.text).as_bytes()),
                    )
                })
            });
        }

        #[cfg(feature = "unstable")]
        {
            let neo = $neo;

            for case in TEXT_CASES {
                $group.bench_function(format!("neo_{}/{}", $variant, case.name), |b| {
                    b.iter(|| {
                        consume_breakpoints(
                            black_box(neo).segment_utf8(black_box(case.text).as_bytes()),
                        )
                    })
                });
            }
        }
    }};
}

macro_rules! bench_utf16 {
    ($group:expr, $variant:literal, $published:expr, $neo:expr) => {{
        let published = $published;

        for case in TEXT_CASES {
            let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

            $group.bench_function(format!("published_{}/{}", $variant, case.name), |b| {
                b.iter(|| {
                    consume_breakpoints(black_box(published).segment_utf16(black_box(&utf16)))
                })
            });
        }

        #[cfg(feature = "unstable")]
        {
            let neo = $neo;

            for case in TEXT_CASES {
                let utf16 = case.text.encode_utf16().collect::<Vec<u16>>();

                $group.bench_function(format!("neo_{}/{}", $variant, case.name), |b| {
                    b.iter(|| consume_breakpoints(black_box(neo).segment_utf16(black_box(&utf16))))
                });
            }
        }
    }};
}

fn line_break_iter_latin1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/line/segment_latin1");

    let published = LineSegmenter::new_dictionary(Default::default());
    #[cfg(feature = "unstable")]
    let neo = NeoLineSegmenter::new_dictionary(Default::default());

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let published_css = LineSegmenter::new_dictionary(options);
    #[cfg(feature = "unstable")]
    let neo_css = NeoLineSegmenter::new_dictionary(options);

    group.bench_function("published_dictionary/english", |b| {
        b.iter(|| {
            consume_breakpoints(
                black_box(published).segment_latin1(black_box(TEXT_ENGLISH).as_bytes()),
            )
        })
    });

    #[cfg(feature = "unstable")]
    group.bench_function("neo_dictionary/english", |b| {
        b.iter(|| {
            consume_breakpoints(black_box(neo).segment_latin1(black_box(TEXT_ENGLISH).as_bytes()))
        })
    });

    group.bench_function("published_dictionary_css/english", |b| {
        b.iter(|| {
            consume_breakpoints(
                black_box(published_css).segment_latin1(black_box(TEXT_ENGLISH).as_bytes()),
            )
        })
    });

    #[cfg(feature = "unstable")]
    group.bench_function("neo_dictionary_css/english", |b| {
        b.iter(|| {
            consume_breakpoints(
                black_box(neo_css).segment_latin1(black_box(TEXT_ENGLISH).as_bytes()),
            )
        })
    });
}

fn line_break_iter_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/line/segment_utf8");

    bench_utf8!(
        group,
        "auto",
        LineSegmenter::new_auto(Default::default()),
        NeoLineSegmenter::new_auto(Default::default())
    );
    bench_utf8!(
        group,
        "lstm",
        LineSegmenter::new_lstm(Default::default()),
        NeoLineSegmenter::new_lstm(Default::default())
    );
    bench_utf8!(
        group,
        "dictionary",
        LineSegmenter::new_dictionary(Default::default()),
        NeoLineSegmenter::new_dictionary(Default::default())
    );

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let published_css = LineSegmenter::new_dictionary(options);
    #[cfg(feature = "unstable")]
    let neo_css = NeoLineSegmenter::new_dictionary(options);

    group.bench_function("published_dictionary_css/english", |b| {
        b.iter(|| {
            consume_breakpoints(
                black_box(published_css).segment_utf8(black_box(TEXT_ENGLISH).as_bytes()),
            )
        })
    });

    #[cfg(feature = "unstable")]
    group.bench_function("neo_dictionary_css/english", |b| {
        b.iter(|| {
            consume_breakpoints(black_box(neo_css).segment_utf8(black_box(TEXT_ENGLISH).as_bytes()))
        })
    });
}

fn line_break_iter_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/line/segment_str");

    bench_str!(
        group,
        "auto",
        LineSegmenter::new_auto(Default::default()),
        NeoLineSegmenter::new_auto(Default::default())
    );
    bench_str!(
        group,
        "lstm",
        LineSegmenter::new_lstm(Default::default()),
        NeoLineSegmenter::new_lstm(Default::default())
    );
    bench_str!(
        group,
        "dictionary",
        LineSegmenter::new_dictionary(Default::default()),
        NeoLineSegmenter::new_dictionary(Default::default())
    );

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let published_css = LineSegmenter::new_dictionary(options);
    #[cfg(feature = "unstable")]
    let neo_css = NeoLineSegmenter::new_dictionary(options);

    group.bench_function("published_dictionary_css/english", |b| {
        b.iter(|| {
            consume_breakpoints(black_box(published_css).segment_str(black_box(TEXT_ENGLISH)))
        })
    });

    #[cfg(feature = "unstable")]
    group.bench_function("neo_dictionary_css/english", |b| {
        b.iter(|| consume_breakpoints(black_box(neo_css).segment_str(black_box(TEXT_ENGLISH))))
    });
}

fn line_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/line/segment_utf16");

    bench_utf16!(
        group,
        "auto",
        LineSegmenter::new_auto(Default::default()),
        NeoLineSegmenter::new_auto(Default::default())
    );
    bench_utf16!(
        group,
        "lstm",
        LineSegmenter::new_lstm(Default::default()),
        NeoLineSegmenter::new_lstm(Default::default())
    );
    bench_utf16!(
        group,
        "dictionary",
        LineSegmenter::new_dictionary(Default::default()),
        NeoLineSegmenter::new_dictionary(Default::default())
    );

    let mut options = LineBreakOptions::default();
    options.strictness = Some(LineBreakStrictness::Anywhere);
    options.word_option = Some(LineBreakWordOption::BreakAll);
    let published_css = LineSegmenter::new_dictionary(options);
    #[cfg(feature = "unstable")]
    let neo_css = NeoLineSegmenter::new_dictionary(options);
    let utf16_english = TEXT_ENGLISH.encode_utf16().collect::<Vec<u16>>();

    group.bench_function("published_dictionary_css/english", |b| {
        b.iter(|| {
            consume_breakpoints(black_box(published_css).segment_utf16(black_box(&utf16_english)))
        })
    });

    #[cfg(feature = "unstable")]
    group.bench_function("neo_dictionary_css/english", |b| {
        b.iter(|| consume_breakpoints(black_box(neo_css).segment_utf16(black_box(&utf16_english))))
    });
}

fn word_break_iter_utf8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/word/segment_utf8");

    bench_utf8!(
        group,
        "auto",
        WordSegmenter::new_auto(Default::default()),
        NeoWordSegmenter::new_auto(Default::default())
    );
    bench_utf8!(
        group,
        "lstm",
        WordSegmenter::new_lstm(Default::default()),
        NeoWordSegmenter::new_lstm(Default::default())
    );
    bench_utf8!(
        group,
        "dictionary",
        WordSegmenter::new_dictionary(Default::default()),
        NeoWordSegmenter::new_dictionary(Default::default())
    );
}

fn word_break_iter_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/word/segment_str");

    bench_str!(
        group,
        "auto",
        WordSegmenter::new_auto(Default::default()),
        NeoWordSegmenter::new_auto(Default::default())
    );
    bench_str!(
        group,
        "lstm",
        WordSegmenter::new_lstm(Default::default()),
        NeoWordSegmenter::new_lstm(Default::default())
    );
    bench_str!(
        group,
        "dictionary",
        WordSegmenter::new_dictionary(Default::default()),
        NeoWordSegmenter::new_dictionary(Default::default())
    );
}

fn word_break_iter_utf16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmenter/word/segment_utf16");

    bench_utf16!(
        group,
        "auto",
        WordSegmenter::new_auto(Default::default()),
        NeoWordSegmenter::new_auto(Default::default())
    );
    bench_utf16!(
        group,
        "lstm",
        WordSegmenter::new_lstm(Default::default()),
        NeoWordSegmenter::new_lstm(Default::default())
    );
    bench_utf16!(
        group,
        "dictionary",
        WordSegmenter::new_dictionary(Default::default()),
        NeoWordSegmenter::new_dictionary(Default::default())
    );
}

criterion_group!(
    benches,
    line_break_iter_latin1,
    line_break_iter_utf8,
    line_break_iter_str,
    line_break_iter_utf16,
    word_break_iter_utf8,
    word_break_iter_str,
    word_break_iter_utf16
);
criterion_main!(benches);
