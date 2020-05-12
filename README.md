ICU4X
=====

Welcome to the home page for the ICU4X-SC.  We are a subcommittee of ICU-TC in the Unicode Consortium focused on providing solutions for client-side internationalization.

Please subscribe to this repository to participate in discussions.  If you would like to join the core development team, apply to join [our mailing list](https://groups.google.com/a/unicode.org/forum/#!forum/omnicu-core).

![Rust](https://github.com/echeran/icu4x/workflows/Rust/badge.svg?branch=master)

## Charter

*For the full charter, including answers to frequently asked questions, see [charter.md](docs/charter.md).*

ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.

ICU4X, or "ICU for X", will be built from the start with several key design constraints:

1. Small and modular code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by i18n experts to encourage best practices.

ICU4X will provide an ECMA-402-compatible API surface in the target client-side platforms, including the web platform, iOS, Android, WearOS, WatchOS, Flutter, and Fuchsia, supported in programming languages including Rust, JavaScript, Objective-C, Java, Dart, and C++.
