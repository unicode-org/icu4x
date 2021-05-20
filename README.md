# ICU4X [![Docs](https://docs.rs/icu/badge.svg)](https://docs.rs/icu) [![Build Status](https://github.com/unicode-org/icu4x/workflows/Build%20&%20Test/badge.svg)](https://github.com/unicode-org/icu4x/actions) [![Coverage Status (Coveralls)](https://coveralls.io/repos/github/unicode-org/icu4x/badge.svg?branch=main)](https://coveralls.io/github/unicode-org/icu4x?branch=main) [![Coverage Status (Codecov)](https://codecov.io/gh/unicode-org/icu4x/branch/main/graph/badge.svg)](https://codecov.io/gh/unicode-org/icu4x)

Welcome to the home page for the `ICU4X` project.

`ICU4X` provides components enabling wide range of software internationalization.
It draws deeply from the experience of [`ICU4C`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/), [`ICU4J`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/) and [`ECMA-402`](https://github.com/tc39/ecma402/) and relies on data from the [`CLDR`](http://cldr.unicode.org/) project.

The design goals of `ICU4X` are:

* Small and modular code
* Pluggable locale data
* Availability and ease of use in multiple programming languages
* Written by i18n experts to encourage best practices

***Stay informed!*** Join our public, low-traffic mailing list: [icu4x-announce@unicode.org](https://corp.unicode.org/mailman/listinfo/icu4x-announce).  *Note: After subscribing, check your spam folder for a confirmation.*

## Documentation

For an introduction to the project, please visit [`Introduction to ICU4X for Rust`](docs/tutorials/intro.md) tutorial.

For technical information on how to use ICU4X, visit our [API docs](https://unicode-org.github.io/icu4x-docs/doc/icu/index.html).

More information about the project can be found in [the docs subdirectory](docs/README.md).

## Quick Start

An example `ICU4X` powered application in Rust may look like this:

```toml
icu = "0.2"
icu_provider_fs = "0.2"
```

```rust
use icu::locid::macros::langid;
use icu::locid::Locale;
use icu::datetime::{DateTimeFormat, mock::datetime::MockDateTime, options::length};
use icu_provider_fs::FsDataProvider;

fn main() {
    let loc: Locale = langid!("pl").into();

    let date: MockDateTime = "2020-10-14T13:21:00".parse()
        .expect("Failed to parse a datetime.");

    let provider = FsDataProvider::try_new("/home/{USER}/projects/icu/icu4x-data")
        .expect("Failed to initialize Data Provider.");

    let options = length::Bag {
        time: Some(length::Time::Medium),
        date: Some(length::Date::Long),
        ..Default::default()
    }.into();

    let dtf = DateTimeFormat::try_new(loc, &provider, &options)
        .expect("Failed to initialize DateTimeFormat");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

## Development

`ICU4X` is developed by the `ICU4X-SC`. We are a subcommittee of ICU-TC in the Unicode Consortium focused on providing solutions for client-side internationalization.  See [unicode.org](https://www.unicode.org/consortium/techchairs.html) for more information on our governance.

Please subscribe to this repository to participate in discussions.  If you want to contribute, see our [contributing.md](CONTRIBUTING.md).

## Charter

*For the full charter, including answers to frequently asked questions, see [charter.md](docs/process/charter.md).*

ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.

ICU4X, or "ICU for X", will be built from the start with several key design constraints:

1. Small and modular code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by i18n experts to encourage best practices.

ICU4X will provide an ECMA-402-compatible API surface in the target client-side platforms, including the web platform, iOS, Android, WearOS, WatchOS, Flutter, and Fuchsia, supported in programming languages including Rust, JavaScript, Objective-C, Java, Dart, and C++.

### Benchmark dashboards

The [performance benchmarks](docs/process/benchmarking.md) are all run on Ubuntu, and are broken out by component.

* [locid](https://unicode-org.github.io/icu4x-docs/benchmarks/perf/components/locid)
* [uniset](https://unicode-org.github.io/icu4x-docs/benchmarks/perf/components/uniset)
* [fixed_decimal](https://unicode-org.github.io/icu4x-docs/benchmarks/perf/utils/fixed_decimal)
* [plurals](https://unicode-org.github.io/icu4x-docs/benchmarks/perf/components/plurals)
* [datetime](https://unicode-org.github.io/icu4x-docs/benchmarks/perf/components/datetime)

The [memory benchmarks](tools/benchmark#icu_benchmark_memory) are run separately on each platform, and examples are individually instrumented.

* [macOS](https://unicode-org.github.io/icu4x-docs/benchmarks/memory/macos-latest/)
* [Ubuntu](https://unicode-org.github.io/icu4x-docs/benchmarks/memory/ubuntu-latest/)
* [Windows](https://unicode-org.github.io/icu4x-docs/benchmarks/memory/windows-latest/)
