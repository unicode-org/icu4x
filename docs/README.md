Documentation
=============

This is the index of the documentation for ICU4X.  When adding a new document
here, please make an effort to update the index as well, for ease of
navigation.  The Summary of each page is its first paragraph.

## Tutorials

List of tutorials on how to use ICU4X.

User-facing tutorials are listed in [index.md](tutorials/index.md). The following list are additional contributor-focused tutorials.

Document | Summary
---------|---------
[markdown_tips.md](tutorials/markdown_tips.md) | According to bylaws.md, designs should make their way to GitHub Markdown files somewhat early in the process. However, since Markdown is not a WYSIWYG platform like Google Docs, it takes a bit of time before you become accustomed to the practice. This document contains tips on the workflow.
[writing_a_new_data_struct.md](tutorials/writing_a_new_data_struct.md) | Tutorial for ICU4X contributors on how to add locale data to the data provider pipeline.

## Process

List of documents describing various aspects of the process which ICU4X project uses.

Document | Summary
---------|---------
[benchmarking.md](process/benchmarking.md) | Description of the model and facets of performance benchmarking that ICU4X is using.
[boilerplate.md](process/boilerplate.md) | Reference for ICU4X-specific conventions on boilerplate (extra code/annotations) in source files.
[bylaws.md](process/bylaws.md) | ICU4X-SC abides by the bylaws listed in this document.
[charter.md](process/charter.md) | ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side internationalization for their products in resource-constrained environments.
[ci_build.md](process/ci_build.md) | An overview of continuous integration setup in ICU4X.
[crate_ownership.md](process/crate_ownership.md) | Describes ownership rules for public crates (on crates.io).
[data_versioning.md](process/data_versioning.md) | An overview of ICU4X's data version policy.
[prd.md](process/prd.md) | Product Requirement Document for ICU4X 1.0
[release.md](process/release.md) | Documentation on release process.
[roadmap.md](process/roadmap.md) | ICU4X 1.0 Roadmap
[rust_versions.md](process/rust_versions.md) | An overview of ICU4X's Rust version policy.
[style_guide.md](process/style_guide.md) | A guide to best practices for writing Rust code in ICU4x.
[triaging.md](process/triaging.md) | ICU4X uses GitHub for tracking feature requests and work items.

## Design

List of documents describing design decisions for ICU4X.

Document | Summary
---------|---------
[data_pipeline.md](design/data_pipeline.md) | One of the key design principles of ICU4X is to make locale data small and portable, allowing it to be pulled from multiple sources depending on the needs of the application. This document explains how that goal can be achieved.
[data_safety.md](design/data_safety.md) | How we think about error handling in locale data.
[enums_or_ids.md](design/enums_or_ids.md) | When to use enums and when to use identifiers to represent entities in ICU4X
[locale_fallback_and_negotiation.md](design/locale_fallback_and_negotiation.md) | How we think about horizontal and vertical fallback when designing ICU4X.
[principles.md](design/principles.md) | These principles are not cast in stone, but are strong guidelines for developers.
[string_representation.md](design/string_representation.md) | String representation on the library boundary.
[why_rust.md](design/why_rust.md) | Why the team chose to write ICU4X in the Rust programming language.

## Research

List of research documenting ICU4X's findings and architectural explorations.

Document | Summary
---------|---------
[data_phases.md](research/data_phases.md) | The situations when information is known and how that influenced the design of ResourcePath.
[datetime_input.md](research/datetime_input.md) | Exploration of potential design models for Date & Time APIs with support for calendar systems and open to strong integration with internationalization.
[datetime.md](research/datetime.md) | High-level outline of how ICU4X sees Date & Time formatting separation of concerns between Internationalization and Date & Time management.
[ecosystem.md](research/ecosystem.md) | This document tracks the crates that already exist in the ecosystem that cover functionality that we may wish to cover in ICU4X.
[format_to_parts.md](research/format_to_parts.md) | Discussion on alternate data models to support ECMA-402 formatToParts
[wasm_vs_transpiler.md](research/wasm_vs_transpiler.md) | We've considered two approaches to a problem of write logic once, deploy it on multiple platforms/languages with minimal effort.
[wrapper_layer.md](research/wrapper_layer.md) | A key piece of rolling out ergonomic ICU4X APIs for Rust and other target languages is the *wrapper layer*: code that sits between the client and the lower-level ICU4X libraries.  This document explains the design behind the ICU4X wrapper layer.

## Posts

List of posts by ICU4X.

Document | Summary
---------|---------
[20210427_ICU4X_02_Release.md](posts/20210427_ICU4X_02_Release.md) | ICU4X 0.2 Release
[20201015_ICU4X_Project_Announcement.md](posts/20201015_ICU4X_Project_Announcement.md) | ICU4X Project Announcement
