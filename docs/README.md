Documentation
=============

This is the index of the documentation for ICU4X.  When adding a new document
here, please make an effort to update the index as well, for ease of
navigation.  The Summary of each page is its first paragraph.

## Tutorials

List of tutorials on how to use ICU4X.

Document | Summary
---------|---------
[intro.md](tutorials/intro.md) | Foo
[data_provider.md](tutorials/data_provider.md) | Foo
[markdown_tips.md](tutorials/markdown_tips.md) | According to bylaws.md, designs should make their way to GitHub Markdown files somewhat early in the process. However, since Markdown is not a WYSIWYG platform like Google Docs, it takes a bit of time before you become accustomed to the practice. This document contains tips on the workflow.

## Process

List of documents describing various aspects of the process which ICU4X project uses.

Document | Summary
---------|---------
[benchmarking.md](process/benchmarking.md) | Foo
[bylaws.md](process/bylaws.md) | ICU4X-SC abides by the bylaws listed in this document.
[charter.md](process/charter.md) | ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.
[crate_ownership.md](process/crate_ownership.md) | Describes ownership rules for public crates (on crates.io).
[release.md](process/release.md) | Foo
[style_guide.md](process/style_guide.md) | A guide to best practices for writing Rust code in ICU4x.
[triaging.md](process/triaging.md) | ICU4X uses GitHub for tracking feature requests and work items.

## Design

List of documents describing design decisions for ICU4X.

Document | Summary
---------|---------
[data_pipeline.md](design/data_pipeline.md) | One of the key design principles of ICU4X is to make locale data small and portable, allowing it to be pulled from multiple sources depending on the needs of the application. This document explains how that goal can be achieved.
[principles.md](design/principles.md) | These principles are not cast in stone, but are strong guidelines for developers.
[string_representation.md](design/string_representation.md) | String representation on the library boundary.

## Research

List of research documenting ICU4X's findings and architectural explorations.

Document | Summary
---------|---------
[date_time.md](research/date_time.md) | Foo
[datetime_input.md](research/datetime_input.md) | Foo
[wasm_vs_transpiler.md](research/wasm_vs_transpiler.md) | We've considered two approaches to a problem of write logic once, deploy it on multiple platforms/languages with minimal effort.
[wrapper_layer.md](research/wrapper_layer.md) | A key piece of rolling out ergonomic ICU4X APIs for Rust and other target languages is the *wrapper layer*: code that sits between the client and the lower-level ICU4X libraries.  This document explains the design behind the ICU4X wrapper layer.
[ecosystem.md](research/ecosystem.md) | This document tracks the crates that already exist in the ecosystem that cover functionality that we may wish to cover in ICU4X.

## Posts

List of posts by ICU4X.

Document | Summary
---------|---------
[20201015_ICU4X_Project_Announcement.md](posts/20201015_ICU4X_Project_Announcement.md) | Foo
