ICU4X
=====

Welcome to the home page for the ICU4X-SC.  We are a subcommittee of ICU-TC in the Unicode Consortium focused on providing solutions for client-side internationalization.  See [unicode.org](https://www.unicode.org/consortium/techchairs.html) for more information on our governance.

Please subscribe to this repository to participate in discussions.  If you want to contribute, see our [contributing.md](CONTRIBUTING.md).

[![Build Status](https://github.com/unicode-org/icu4x/workflows/Build%20&%20Test/badge.svg)](https://github.com/unicode-org/icu4x/actions) [![Coverage Status](https://coveralls.io/repos/github/unicode-org/icu4x/badge.svg?branch=master)](https://coveralls.io/github/unicode-org/icu4x?branch=master)

## Charter

*For the full charter, including answers to frequently asked questions, see [charter.md](docs/charter.md).*

ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.

ICU4X, or "ICU for X", will be built from the start with several key design constraints:

1. Small and modular code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by i18n experts to encourage best practices.

ICU4X will provide an ECMA-402-compatible API surface in the target client-side platforms, including the web platform, iOS, Android, WearOS, WatchOS, Flutter, and Fuchsia, supported in programming languages including Rust, JavaScript, Objective-C, Java, Dart, and C++.

## Documentation

More information about the project can be found on our [wiki](https://github.com/unicode-org/icu4x/wiki) and in [the docs subdirectory](docs/index.md).  To see technical information on how to use ICU4X, visit our [API docs](https://unicode-org.github.io/icu4x-docs/doc/icu_locale/index.html).

### Benchmark dashboards

| Component     | Runtime                                                                  |
|---------------|--------------------------------------------------------------------------|
| locale        | [link](https://unicode-org.github.io/icu4x-docs/dev/components/locale)   |
| uniset        | [link](https://unicode-org.github.io/icu4x-docs/dev/components/uniset)   |
| fixed-decimal | [link](https://unicode-org.github.io/icu4x-docs/dev/utils/fixed-decimal) |
