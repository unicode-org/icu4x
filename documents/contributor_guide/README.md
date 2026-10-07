# ICU4X contributor guide

This guide explains the Rust patterns and design rules that ICU4X depends
on. They are easy to miss, especially for new contributors: the code shows
what ICU4X does, but not why, or which patterns are rules.

It is written for:

- **New contributors** who know Rust, but not ICU4X.
- **Reviewers**, who can link a rule in a PR comment to say what to change
  and why.
- **Maintainers** who want design decisions written down, with the reasons.
- **Coding agents**, through the short rules in [`avoid/`](avoid/README.md)
  and the root [`AGENTS.md`](../../AGENTS.md).

This guide teaches. It does not replace the existing documents.
These documents are the source of truth:

| Topic | Source of truth |
|---|---|
| Building, testing, regenerating files, review | [CONTRIBUTING.md](../../CONTRIBUTING.md) |
| Coding rules | [style_guide.md](../process/style_guide.md) |
| Design principles | [principles.md](../design/principles.md) |
| Invalid data and panics | [data_safety.md](../design/data_safety.md) |
| Data stability | [data_versioning.md](../process/data_versioning.md) |
| Requirements for stable components | [graduation.md](../process/graduation.md) |

If this guide disagrees with one of them, the source of truth wins.
Please open an issue so we can fix the guide.

## Chapters

| Chapter | Read it when |
|---|---|
| `orientation.md` *(planned)* | You are new to the repository. |
| `i18n_basics.md` *(planned)* | Locales, CLDR, or Unicode properties are new to you. |
| `errors_and_panics.md` *(planned)* | Before your first code PR. |
| `cow_and_borrowing.md` *(planned)* | Before your first code PR. |
| `zerovec_and_ule.md` *(planned)* | You change a data struct, or use `zerovec`. |
| `yoke_and_zerofrom.md` *(planned)* | You wonder what `'data`, `Yokeable`, or `DataPayload` are for. |
| `data_provider.md` *(planned)* | You write a constructor, or load data. |
| `data_structs_and_datagen.md` *(planned)* | You add or change a data struct. |
| `no_std_and_dependencies.md` *(planned)* | You add a dependency or a Cargo feature, or want to use `std`. |
| `api_design.md` *(planned)* | You add or change a public API. |
| `ffi_diplomat.md` *(planned)* | You add a public API. Stable APIs also need FFI. |
| `performance_and_size.md` *(planned)* | You change hot code or the data layout. |
| `first_pr.md` *(planned)* | You are about to open a PR. |

`glossary.md` *(planned)* will explain ICU4X words like *marker*,
*payload*, *baked data*, and *GIGO*.

## Reading paths

- **First code PR:** orientation → errors_and_panics → cow_and_borrowing → first_pr.
- **Data work:** also i18n_basics, zerovec_and_ule, yoke_and_zerofrom,
  data_provider, and data_structs_and_datagen.
- **API work:** also no_std_and_dependencies, api_design, ffi_diplomat, and
  performance_and_size.

## Avoid rules

[avoid/README.md](avoid/README.md) lists short rules. Each rule says what to
avoid, what to do instead, and why. Each rule has a permanent ID (like
`AV001`) and its own file, so reviewers can link it in a comment.

A new rule is `Proposed`. It becomes `Accepted` when a maintainer approves it.
`AGENTS.md` lists only accepted rules.

## Conventions

- Each chapter and rule says which ICU4X version it was checked against.
- `tools/md-tests` compiles and runs the `rust` code blocks in CI.
  Blocks marked `rust,ignore` are excerpts. The text above an excerpt names
  its file.
- Code paths like `components/casemap/src/casemapper.rs` start at the
  repository root.
- `TODO(verify): ...` marks a statement that no maintainer has confirmed yet.
- Simple English and short sentences. The "why" comes before the "how".

## Changing this guide

- Start from [templates/chapter.md](templates/chapter.md) or
  [templates/avoid_rule.md](templates/avoid_rule.md).
- Send one chapter, or a few rules, per PR. Write `## Changelog (N/A)` in the
  PR description (see [changelog.md](../process/changelog.md)).
- Add each new chapter or rule file to `tools/md-tests/src/lib.rs`.
- If you change an API that the guide shows, update the guide in the same PR.
- Owners: TODO(verify): names to be agreed with the maintainers.

*Checked against: ICU4X 2.3 (`main` @ `2fa9afb769`, 2026-10-06).*
