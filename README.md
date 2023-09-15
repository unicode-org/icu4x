# ICU4X [![Docs](https://docs.rs/icu/badge.svg)](https://docs.rs/icu) [![Build Status](https://github.com/unicode-org/icu4x/actions/workflows/build-test.yml/badge.svg)](https://github.com/unicode-org/icu4x/actions) [![Coverage Status (Coveralls)](https://coveralls.io/repos/github/unicode-org/icu4x/badge.svg?branch=main)](https://coveralls.io/github/unicode-org/icu4x?branch=main) [![Coverage Status (Codecov)](https://codecov.io/gh/unicode-org/icu4x/branch/main/graph/badge.svg)](https://app.codecov.io/gh/unicode-org/icu4x/tree/main)

Welcome to the home page for the `ICU4X` project.

`ICU4X` provides components enabling wide range of software internationalization.
It draws deeply from the experience of [`ICU4C`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/), [`ICU4J`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/) and [`ECMA-402`](https://github.com/tc39/ecma402/) and relies on data from the [`CLDR`](http://cldr.unicode.org/) project.

The design goals of `ICU4X` are:

* Small and modular code
* Pluggable locale data
* Availability and ease of use in multiple programming languages
* Written by internationalization experts to encourage best practices

***Stay informed!*** Join our public, low-traffic mailing list: [icu4x-announce@unicode.org](https://groups.google.com/a/unicode.org/g/icu4x-announce).  *Note: After subscribing, check your spam folder for a confirmation.*

## Documentation

For an introduction to the project, please visit the ["Introduction to ICU4X for Rust"](docs/tutorials/intro.md) tutorial. Further tutorials can be found in the [tutorial index](docs/tutorials/index.md).

For technical information on how to use ICU4X, visit our [API docs (latest stable)](https://docs.rs/icu/latest/) or [API docs (tip of main)](https://unicode-org.github.io/icu4x/docs/icu/).

More information about the project can be found in [the docs subdirectory](docs/README.md).

## Quick Start

An example `ICU4X` powered application in Rust may look like below...

`Cargo.toml`:

```toml
[dependencies]
icu = "1.0.0"
icu_testdata = "1.0.0"
```

`src/main.rs`:

```rust
use icu::calendar::{DateTime, indian::Indian};
use icu::datetime::{options::length, TypedDateTimeFormatter};
use icu::locid::locale;

let options =
    length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();

let dtf = TypedDateTimeFormatter::<Indian>::try_new_unstable(&icu_testdata::unstable(), &locale!("es").into(), options)
    .expect("Failed to create DateTimeFormatter instance.");

let date = DateTime::try_new_iso_datetime(2020, 9, 12, 12, 35, 0)
    .expect("Failed to parse date.")
    .to_calendar(Indian);

let formatted_date = dtf.format(&date);
assert_eq!(
    formatted_date.to_string(),
    "21 de bhadra de 1942 saka, 12:35:00"
);
```

## Development

`ICU4X` is developed by the `ICU4X-SC`. We are a subcommittee of ICU-TC in the Unicode Consortium focused on providing solutions for client-side internationalization.  See [unicode.org](https://www.unicode.org/consortium/techchairs.html) for more information on our governance.

Please subscribe to this repository to participate in discussions.  If you want to contribute, see our [contributing.md](CONTRIBUTING.md).

## Charter

*For the full charter, including answers to frequently asked questions, see [charter.md](docs/process/charter.md).*

ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side internationalization for their products in resource-constrained environments.

ICU4X, or "ICU for X", will be built from the start with several key design constraints:

1. Small and modular code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by internationalization experts to encourage best practices.

ICU4X will provide an ECMA-402-compatible API surface in the target client-side platforms, including the web platform, iOS, Android, WearOS, WatchOS, Flutter, and Fuchsia, supported in programming languages including Rust, JavaScript, Objective-C, Java, Dart, and C++.

## Licensing and Copyright

Copyright © 2020-2023 Unicode, Inc. Unicode and the Unicode Logo are registered trademarks of Unicode, Inc. in the United States and other countries.

The project is released under [LICENSE](./LICENSE).

A CLA is required to contribute to this project - please refer to the [CONTRIBUTING.md](./CONTRIBUTING.md) file (or start a Pull Request) for more information.
