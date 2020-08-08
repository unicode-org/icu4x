Documentation
=============

This is the index of the documentation for ICU4X.  When adding a new document
here, please make an effort to update the index as well, for ease of
navigation.  The Summary of each page is its first paragraph.

Document | Summary
---------|---------
[charter.md](charter.md) | ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.
[bylaws.md](bylaws.md) | ICU4X-SC abides by the bylaws listed in this document.
[data-pipeline.md](data-pipeline.md) | One of the key design principles of ICU4X is to make locale data small and portable, allowing it to be pulled from multiple sources depending on the needs of the application. This document explains how that goal can be achieved.
[ecosystem.md](ecosystem.md) | This document tracks the crates that already exist in the ecosystem that cover functionality that we may wish to cover in ICU4X.
[markdown-tips.md](markdown-tips.md) | According to bylaws.md, designs should make their way to GitHub Markdown files somewhat early in the process. However, since Markdown is not a WYSIWYG platform like Google Docs, it takes a bit of time before you become accustomed to the practice. This document contains tips on the workflow.
[crate-ownership.md](crate-ownership.md) | Describes ownership rules for public crates (on crates.io).
[principles.md](principles.md) | These principles are not cast in stone, but are strong guidelines for developers.
[string-representation.md](string-representation.md) | String representation on the library boundary.
[triaging.md](triaging.md) | ICU4X uses GitHub for tracking feature requests and work items.
[wasm-vs-transpiler.md](wasm-vs-transpiler.md) | We've considered two approaches to a problem of write logic once, deploy it on multiple platforms/languages with minimal effort.
[wrapper-layer.md](wrapper-layer.md) | A key piece of rolling out ergonomic ICU4X APIs for Rust and other target languages is the *wrapper layer*: code that sits between the client and the lower-level ICU4X libraries.  This document explains the design behind the ICU4X wrapper layer.

