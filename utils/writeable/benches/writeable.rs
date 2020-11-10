use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fmt;
use writeable::Writeable;

/// A sample type implementing Writeable
struct WriteableMessage<'s> {
    message: &'s str,
}

impl Writeable for WriteableMessage<'_> {
    fn write_to(&self, sink: &mut dyn fmt::Write) -> fmt::Result {
        sink.write_str(self.message)
    }

    fn write_len(&self) -> usize {
        self.message.len()
    }
}

/// A sample type implementing Display
struct DisplayMessage<'s> {
    message: &'s str,
}

impl fmt::Display for DisplayMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message)
    }
}

const SHORT_STR: &str = "short";
const MEDIUM_STR: &str = "this is a medium-length string";
const LONG_STR: &str = "this string is very very very very very very very very very very very very very very very very very very very very very very very very long";

fn overview_bench(c: &mut Criterion) {
    c.bench_function("writeable/overview", |b| {
        #[allow(clippy::suspicious_map)]
        b.iter(|| {
            // This benchmark runs to_string on short, medium, and long strings in one batch.
            WriteableMessage {
                message: black_box(SHORT_STR),
            }
            .writeable_to_string();
            WriteableMessage {
                message: black_box(MEDIUM_STR),
            }
            .writeable_to_string();
            WriteableMessage {
                message: black_box(LONG_STR),
            }
            .writeable_to_string();
        });
    });

    #[cfg(feature = "bench")]
    {
        writeable_benches(c);
        display_benches(c);
    }
}

#[cfg(feature = "bench")]
fn writeable_benches(c: &mut Criterion) {
    c.bench_function("writeable/to_string/short", |b| {
        b.iter(|| {
            WriteableMessage {
                message: black_box(SHORT_STR),
            }
            .writeable_to_string()
        });
    });
    c.bench_function("writeable/to_string/medium", |b| {
        b.iter(|| {
            WriteableMessage {
                message: black_box(MEDIUM_STR),
            }
            .writeable_to_string()
        });
    });
    c.bench_function("writeable/to_string/long", |b| {
        b.iter(|| {
            WriteableMessage {
                message: black_box(LONG_STR),
            }
            .writeable_to_string()
        });
    });
}

#[cfg(feature = "bench")]
fn display_benches(c: &mut Criterion) {
    c.bench_function("display/to_string/short", |b| {
        b.iter(|| {
            DisplayMessage {
                message: black_box(SHORT_STR),
            }
            .to_string()
        });
    });
    c.bench_function("display/to_string/medium", |b| {
        b.iter(|| {
            DisplayMessage {
                message: black_box(MEDIUM_STR),
            }
            .to_string()
        });
    });
    c.bench_function("display/to_string/long", |b| {
        b.iter(|| {
            DisplayMessage {
                message: black_box(LONG_STR),
            }
            .to_string()
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
