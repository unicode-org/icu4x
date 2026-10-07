# AGENTS.md

Instructions for coding agents in the ICU4X repository. People should start
with [CONTRIBUTING.md](CONTRIBUTING.md) and the
[contributor guide](documents/contributor_guide/README.md).

## Working style

- Understand the issue before you write code.
- One logical change per PR. Keep descriptions and comments short.
- Fill in the `## Changelog` section ([changelog.md](documents/process/changelog.md)).

## Commands

- Format: `cargo fmt`. Lint: `cargo clippy-all`.
- Test one crate: `cargo test -p <crate> --all-features`.
- Fast checks: `cargo quick`. Tidy checks (license, fmt, READMEs): `cargo tidy`.
- Regenerate generated files with the commands in CONTRIBUTING.md.

## Rules

Each link has ❌/✅ examples and the reason.

- [AV001](documents/contributor_guide/avoid/AV001_no_panics_in_library_code.md): No `unwrap`, `expect`, indexing, or `panic!` in library code. Return `Result`, or use a fallback with `debug_assert!`.
- [AV002](documents/contributor_guide/avoid/AV002_no_needless_allocation.md): Don't allocate when you can borrow. Return `Cow` or a `Writeable`, not `String`.
- [AV003](documents/contributor_guide/avoid/AV003_no_std_in_library_crates.md): No `std` in library crates. Use `core` and `alloc`.
- [AV004](documents/contributor_guide/avoid/AV004_zero_copy_data_structs.md): Data structs are zero-copy: `Cow<'data, str>` or `zerovec` types, with `#[serde(borrow)]`.
- [AV005](documents/contributor_guide/avoid/AV005_stable_serialized_layout.md): Don't change the serialized layout of a released data struct.

*Checked against: ICU4X 2.3 (`main` @ `2fa9afb769`).*
