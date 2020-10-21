# ICU4X [![Docs](https://docs.rs/icu/badge.svg)](https://docs.rs/icu) [![Build Status](https://github.com/unicode-org/icu4x/workflows/Build%20&%20Test/badge.svg)](https://github.com/unicode-org/icu4x/actions) [![Coverage Status](https://coveralls.io/repos/github/unicode-org/icu4x/badge.svg?branch=master)](https://coveralls.io/github/unicode-org/icu4x?branch=master)

Welcome to the home page for the `ICU4X` project.

`ICU4X` provides components enabling wide range of software internationalization.
It draws deeply from the experience of [`ICU4C`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/), [`ICU4J`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/) and [`ECMA-402`](https://github.com/tc39/ecma402/) and relies on data from the [`CLDR`](http://cldr.unicode.org/) project.

The design goals of `ICU4X` are:

* Small and modular code
* Pluggable locale data
* Availability and ease of use in multiple programming languages
* Written by i18n experts to encourage best practices

An example `ICU4X` powered application in Rust may look like this:

```toml
icu = "0.1"
icu_provider_fs = "0.1"
```

```rust
use icu::locid::macros::langid;
use icu::datetime::{DateTimeFormat, date::MockDateTime, options::style};
use icu_provider_fs::FsDataProvider;

fn main() {
    let lid = langid!("ar");

    let date: MockDateTime = "2020-10-14T13:21:00".parse()
        .expect("Failed to parse a date time.");

    let provider = FsDataProvider::try_new("./icu4x-data")
        .expect("Failed to initialize Data Provider.");

    let options = style::Bag {
        time: Some(style::Time::Medium),
        date: Some(style::Date::Long),
        ..Default::default()
    }.into();

    let dtf = DateTimeFormat::try_new(lid, &provider, &options)
        .expect("Failed to initialize Date Time Format");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```


## Documentation

For an introduction the project, please visit [`Introduction to ICU4X for Rust`](https://github.com/unicode-org/icu4x/wiki/Introduction-to-ICU4X-for-Rust) tutorial.

For technical information on how to use ICU4X, visit our [API docs](https://unicode-org.github.io/icu4x-docs/doc/icu/index.html).

More information about the project can be found on our [wiki](https://github.com/unicode-org/icu4x/wiki) and in [the docs subdirectory](docs/index.md).


## Development

`ICU4X` is developed by the `ICU4X-SC`. We are a subcommittee of ICU-TC in the Unicode Consortium focused on providing solutions for client-side internationalization.  See [unicode.org](https://www.unicode.org/consortium/techchairs.html) for more information on our governance.

Please subscribe to this repository to participate in discussions.  If you want to contribute, see our [contributing.md](CONTRIBUTING.md).

## Charter

*For the full charter, including answers to frequently asked questions, see [charter.md](docs/charter.md).*

ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.

ICU4X, or "ICU for X", will be built from the start with several key design constraints:

1. Small and modular code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by i18n experts to encourage best practices.

ICU4X will provide an ECMA-402-compatible API surface in the target client-side platforms, including the web platform, iOS, Android, WearOS, WatchOS, Flutter, and Fuchsia, supported in programming languages including Rust, JavaScript, Objective-C, Java, Dart, and C++.

### Benchmark dashboards

| Component     | Runtime                                                                  |
|---------------|--------------------------------------------------------------------------|
| locid         | [link](https://unicode-org.github.io/icu4x-docs/dev/components/locid)    |
| uniset        | [link](https://unicode-org.github.io/icu4x-docs/dev/components/uniset)   |
| fixed_decimal | [link](https://unicode-org.github.io/icu4x-docs/dev/utils/fixed_decimal) |
| plurals       | [link](https://unicode-org.github.io/icu4x-docs/dev/components/plurals)  |
| datetime      | [link](https://unicode-org.github.io/icu4x-docs/dev/components/datetime) |
