# ICU4X style guide and contributor docs: design proposal

This document proposes improvements to ICU4X's human-facing contributor documentation, tracked in [#8558](https://github.com/unicode-org/icu4x/issues/8558):

1. Split `documents/process/style_guide.md` into one page per area in `documents/process/style_guide/`, so reviewers can link to a page or to a rule on it.
2. Add the reason (`**Why:**`), `**❌ Don't:**` / `**✅ Do:**` examples, and `**Enforcement:**` to rules in the style guide, update outdated rules, and add missing ones.
3. Fix outdated steps in existing process docs (`writing_a_new_data_struct.md`, `CONTRIBUTING.md`).
4. Add teaching chapters for Rust patterns that the existing docs do not cover (`Cow` and borrowing, zerovec and ULE, yoke and `ZeroFrom`, data providers, i18n basics, FFI with Diplomat, and a glossary).

Agent-specific files (`AGENTS.md` and similar) are out of scope here and are tracked in [#8556](https://github.com/unicode-org/icu4x/issues/8556).

All paths, symbols, and quotes were checked at ICU4X `main` @ `77a3b6ddfb` (2026-10-09, after [#8560](https://github.com/unicode-org/icu4x/pull/8560)). The `rust` code blocks in the drafts were compiled and run (see §7).

---

## 0. Context

1. **Target:** ICU4X 2.3 on `main`.
2. **Three audiences:**
   - **New contributors** who know Rust, but not ICU4X: `Cow` and borrowing, zerovec and ULE, yoke, data providers, databake, `no_std`, API conventions, and Diplomat.
   - **Reviewers:** a link to the section that says what to change and why, so they don't have to explain the same thing in every review.
   - **Future maintainers:** design decisions and their reasons, so they stay in the repo when people move on.
3. **Related threads:**
   - [#8558](https://github.com/unicode-org/icu4x/issues/8558) "Style guide and contributor docs: Rust patterns ICU4X uses, what to avoid, and why" (tracking issue for this proposal).
   - [#8556](https://github.com/unicode-org/icu4x/issues/8556) "Documentation by-agents for-agents" (tracks `AGENTS.md` and norms for agent-facing docs).
   - [#7593](https://github.com/unicode-org/icu4x/issues/7593) "Instructions for New Contributors".
   - [#2675](https://github.com/unicode-org/icu4x/issues/2675) "More intro material from a Rustacean point of view".
4. **Short beats complete:** maintainers asked for short, reviewable docs that improve existing files in place instead of duplicating them.

Open questions are in §8.

---

## 1. Design decisions

An earlier draft of this proposal (#8561, #8562, now closed) put both teaching chapters and a separate `avoid/AV001_*.md` rule tree under `documents/contributor_guide/`, plus a root `AGENTS.md`. Based on maintainer feedback, this revision makes the following changes:

### Core approach

- **Improve the style guide in place instead of a second rule tree.** `documents/process/style_guide.md` already holds ICU4X's coding rules. Rather than adding a parallel `avoid/` directory with `AVnnn` IDs, we move the style guide into `documents/process/style_guide/` with one `.md` page per area (#8585) and add `**Why:**`, `**❌ Don't:**`, `**✅ Do:**`, and `**Enforcement:**` directly to its rules (#8584, #8587).
- **Update process docs in place.** Instead of separate chapters for repository orientation, your first PR, or writing a data struct, we update `CONTRIBUTING.md` and `documents/process/writing_a_new_data_struct.md` (#8588) directly.
- **Teaching chapters only for gaps.** Teaching chapters cover topics that the style guide and process docs do not explain step by step, and link to the style guide for the rules.
- **Keep agent-facing files separate.** `AGENTS.md` and agent-specific norms are tracked in #8556 and are not part of this proposal.

### Specific changes from the initial outline

| # | Change | Why |
|---|---|---|
| 1 | Split `style_guide.md` into `documents/process/style_guide/<area>.md` with an index in `style_guide/README.md` (#8585). | At 1,078 lines, a single file is hard to browse. One page per area gives reviewers a clean link for either a whole topic (`style_guide/data_types.md`) or a single rule (`style_guide/errors_and_panics.md#dont-panic--required`). |
| 2 | Fold the 15 candidate "avoid" rules into the style guide pages (or `CONTRIBUTING.md`) without `AVnnn` IDs. | Avoids maintaining two copies of the same rules. Existing rules get `**Why:**` and ❌/✅ examples; missing rules (such as stable serialized layout, ULE safety, and GIGO) are added to the matching style guide page (§5). |
| 3 | Replace `data_structs_and_datagen.md`, `orientation.md`, `first_pr.md`, `errors_and_panics.md`, `no_std_and_dependencies.md`, `api_design.md`, and `performance_and_size.md` with in-place updates to `writing_a_new_data_struct.md`, `CONTRIBUTING.md`, and the style guide pages. | Those topics already have a home in `documents/process/` or `CONTRIBUTING.md`. Updating them in place fixes stale instructions instead of working around them. |
| 4 | Keep 7 focused teaching chapters: `cow_and_borrowing.md`, `zerovec_and_ule.md`, `yoke_and_zerofrom.md`, `data_provider.md`, `i18n_basics.md`, `ffi_diplomat.md`, and `glossary.md`. | These topics need worked examples and mental models that do not fit inside a style guide rule list. |
| 5 | Split zero-copy teaching into `zerovec_and_ule.md` and `yoke_and_zerofrom.md`. | They solve different problems: byte layout plus `unsafe` ULE invariants, vs. payloads that borrow from their own buffer. |
| 6 | Drop number prefixes (`03_…`) on chapter files. | Adding or reordering a chapter would rename files and break links from issues and PRs. |

### Risks

| Risk | Evidence in the repo | Mitigation |
|---|---|---|
| **Staleness** | Several docs had drifted from 2.x: `style_guide.md` still required `IcuResult<T>` and showed `VarZeroVec<'data, String>`, `graduation.md` pointed to `make-testdata` for the zero-copy check (moved to `tools/make/bakeddata` in [#8474](https://github.com/unicode-org/icu4x/pull/8474)), `documents/README.md` had 6 broken links, and `writing_a_new_data_struct.md` still showed `#[icu_provider::data_struct]` and `KEYS`. | [#8560](https://github.com/unicode-org/icu4x/pull/8560) (merged) fixed the stale references and broken links; [#8588](https://github.com/unicode-org/icu4x/pull/8588) updates `writing_a_new_data_struct.md`; [#8587](https://github.com/unicode-org/icu4x/pull/8587) updates `Use no_std`. Compile `rust` examples in CI via `tools/md-tests` (§7). |
| **Broken inbound links to `style_guide.md`** | Moving `style_guide.md` to `style_guide/README.md` updates all in-repo links (#8585), but 16 older issues and PRs link to `documents/process/style_guide.md`. | Open question in §8: keep a short pointer file at `documents/process/style_guide.md` if maintainers want old issue links to keep working. |
| **Scope and review size** | Rewriting the whole style guide or adding all chapters at once is too large to review. | Ship in small PRs (below). Pure file moves are in separate commits from text changes so git history and review diffs stay clean. |
| **Verbosity** | Maintainers want concise docs written with care (#8556). | Keep rules to a short `**Why:**` + minimal `**❌ Don't:**` / `**✅ Do:**` + `**Enforcement:**`. Keep teaching chapters under ~2,500 words. |
| **Unconfirmed rules** | Some candidate rules reflect review practice that is not yet written policy. | Mark unconfirmed points `TODO(verify)` and ask reviewers explicitly in the PR description. |

### Rollout

The work is split into three tracks matching [#8558](https://github.com/unicode-org/icu4x/issues/8558):

| Track | PR / Step | Content |
|---|---|---|
| **Existing docs** | [#8560](https://github.com/unicode-org/icu4x/pull/8560) *(merged)* | Fix stale references (`IcuResult<T>`, `VarZeroVec<'data, String>`, `graduation.md`, `bakeddata` zero-copy path, `fingerprints.csv` path) and 6 broken links. |
| | [#8588](https://github.com/unicode-org/icu4x/pull/8588) *(open)* | Update `documents/process/writing_a_new_data_struct.md` to `icu_provider::data_struct!`, `provider::MARKERS`, and the current `DecimalSymbols<'_>` / `registry!` API. |
| | Planned PR | `CONTRIBUTING.md`: add a short map of the repo (`components/`, `utils/`, `provider/`, `ffi/`, `tools/`), its CI jobs, and a first-PR checklist. |
| **Style guide** (stack [#8586](https://github.com/unicode-org/icu4x/pull/8586)) | [#8584](https://github.com/unicode-org/icu4x/pull/8584) *(open)* | Add `**Why:**` and ❌/✅ examples to `Don't Panic` and `Zero-copy in DataProvider structs`, plus a new rule `Keep the serialized layout of stable data structs :: required`. |
| | [#8585](https://github.com/unicode-org/icu4x/pull/8585) *(open)* | Move `style_guide.md` to `documents/process/style_guide/README.md`, split out `data_types.md` and `errors_and_panics.md`, and add a `Contents` list. |
| | [#8587](https://github.com/unicode-org/icu4x/pull/8587) *(open)* | Split out `crates.md`, rewrite the stale `Use no_std` rule, and add ❌/✅ examples and `depcheck` enforcement. |
| | Planned PRs | Finish `data_types.md` and `errors_and_panics.md` (new rules for ULE safety, datagen options, and GIGO); split out the remaining area pages (`naming.md`, `layout_and_formatting.md`, `api_design.md`, `passing_values.md`, `traits.md`, `idioms.md`, `lints.md`) and update stale rules (such as `Constructor conventions`); hook compilable examples into `tools/md-tests`. |
| **Teaching chapters** | Planned PRs | One chapter per PR: `cow_and_borrowing.md` (full draft in §4), `zerovec_and_ule.md`, `yoke_and_zerofrom.md`, `data_provider.md`, `i18n_basics.md`, `ffi_diplomat.md`, and `glossary.md`. |

---

## 2. Proposed file layout

```text
CONTRIBUTING.md                                    (existing: add repo map, CI jobs, first-PR checklist)
documents/process/
├── writing_a_new_data_struct.md                   (existing: updated to current 2.x data API in #8588)
└── style_guide/
    ├── README.md                                  (index, Objectives, Preamble; #8585)
    ├── data_types.md                              (Data Types; #8584, #8585)
    ├── errors_and_panics.md                       (Error Handling, Panics, Integer Overflow; #8584, #8585)
    ├── crates.md                                  (Crate Features and Dependencies; #8587)
    ├── naming.md                                  (Naming Conventions; planned)
    ├── layout_and_formatting.md                   (Module and Code Layout, Code Formatting; planned)
    ├── api_design.md                              (Private vs Public, Structs, Options, Constructors, Operators, Binding Traits; planned)
    ├── passing_values.md                          (Sized Types, Pass by Reference vs Value; planned)
    ├── traits.md                                  (Derived Traits; planned)
    ├── idioms.md                                  (Option, Iteration, Enums, Matching; planned)
    └── lints.md                                   (Lints; planned)

documents/contributor_guide/                       (proposed location for teaching chapters; see §8)
├── README.md
├── glossary.md
├── i18n_basics.md
├── cow_and_borrowing.md
├── zerovec_and_ule.md
├── yoke_and_zerofrom.md
├── data_provider.md
└── ffi_diplomat.md
```

### Style guide pages (`documents/process/style_guide/`)

| Page | Sections covered | Key additions / updates |
|---|---|---|
| `README.md` | Contents, Objectives, Preamble, Appendix | Links to each area page and explains how to link to a page or a rule on it (#8585). |
| `data_types.md` | Zero-copy in DataProvider structs, Conventions for strings in structs, Pre-validation of options, Pre-parsed fields (exotic types) | ❌/✅ + `**Why:**` + `bakeddata` enforcement on Zero-copy (#8584); new rules for `Keep the serialized layout of stable data structs` (#8584), ULE safety, and datagen options (§5). |
| `errors_and_panics.md` | Error Handling, Panicking APIs, Non-Panicking APIs (`Don't Panic`, `Avoid to_string`, `Don't Handle Errors`), Returning Errors, Panics in Habits, Integer Overflow | Crate-specific error types instead of `IcuResult<T>` (#8560); ❌/✅ + `**Why:**` on `Don't Panic` (#8584); new rule for GIGO on invalid data (§5). |
| `crates.md` | Crate Features (`Use no_std`, `When to add crate features`), Crate Dependencies (`Avoid heavy dependencies`, `Avoid std::collections::HashMap`) | Rewrite `Use no_std` to match `no_std` + `alloc` and `ci-job-nostd`; add `Cargo.toml` feature example and `depcheck` allowlist enforcement (#8587). |
| `naming.md` | Naming Conventions | Existing rules + examples where needed. |
| `layout_and_formatting.md` | Module and Code Layout, Code Formatting and Linting | Add `TODO(#issue)` convention from `graduation.md` (§5). |
| `api_design.md` | Private vs Public, Structs with Private Fields (`Constructor conventions`), Options structs, Operator Overloading, Binding Traits | Update `Constructor conventions` to 2.x `(provider, prefs, options)` and borrowed/owned pairs (`Foo::new() -> FooBorrowed`); add FFI coverage (`missing_apis.txt`) and no-global-caches rules (§5). |
| `passing_values.md` | Sized Types, Pass by Reference vs Pass by Value (`Avoid implicit allocations`) | Add `Cow` and `Writeable` examples to `Avoid implicit allocations`, linking to `cow_and_borrowing.md`. |
| `traits.md` | Derived Traits (`Implement Copy on error types`, `Default`) | Existing rules. |
| `idioms.md` | Option, Iteration, Enums, Matching | Existing rules. |
| `lints.md` | Lints (`Panics`, `Exhaustiveness`, `Documentation`, `Other Lints`) | Cross-link `Panics` with `errors_and_panics.md`; add ❌/✅ example for `#[non_exhaustive]` on options/errors vs. data structs. |

### Teaching chapters

| File | Content | Links to style guide and existing docs |
|---|---|---|
| `README.md` | Audiences, chapter table, reading paths, how chapters relate to the style guide. | `CONTRIBUTING.md`, `style_guide/README.md`, `principles.md`, `data_safety.md`, `data_versioning.md`, `graduation.md` |
| `glossary.md` | Short definitions: locale, CLDR, marker, payload, data provider, compiled/blob/runtime data, attributes, fallback, ULE, GIGO, `Writeable`, Diplomat. | `data_architecture.md`, `data_pipeline.md` |
| `i18n_basics.md` | Just enough i18n to read component code: locales and BCP 47, CLDR, Unicode properties, plural rules, and why results depend on locale data. | `locale_fallback_and_negotiation.md`, `enums_or_ids.md`, `tutorials/` |
| `cow_and_borrowing.md` | Borrow first, allocate only when needed: `Cow`, `Writeable`, and borrowed/owned type pairs. *Full sample in §4.* | `style_guide/passing_values.md`, `style_guide/errors_and_panics.md`, `style_guide/data_types.md`, `string_representation.md` |
| `zerovec_and_ule.md` | Why data structs use `ZeroVec`, `VarZeroVec`, `ZeroMap`, and `VarZeroCow`. What ULE and VarULE are, derived vs. hand-written impls, and the safety checklist. | `style_guide/data_types.md`, `data_safety.md`, `principles.md`, `utils/zerovec` docs |
| `yoke_and_zerofrom.md` | How a payload borrows from a buffer that it owns (`Yokeable`, `'data`), why `#[yoke(prove_covariance_manually)]` exists, and `ZeroFrom` for cheap borrowed copies. | `data_architecture.md`, `utils/yoke` and `utils/zerofrom` docs |
| `data_provider.md` | Markers, requests, and `DataPayload`. Compiled vs. blob vs. runtime data. The constructor family (`new`, `try_new`, `*_unstable`, `*_with_buffer_provider`). Fallback. | `writing_a_new_data_struct.md`, `data_pipeline.md`, `data_architecture.md`, `locale_fallback_and_negotiation.md`, `tutorials/` |
| `ffi_diplomat.md` | How a Rust API reaches C, C++, JavaScript/TypeScript, and Dart through Diplomat (`ffi/capi`). What Diplomat can't express. `missing_apis.txt` and the coverage allowlist. | `style_guide/api_design.md`, `graduation.md`, `principles.md`, `ffi/capi/README.md` |

### Style guide rule format

Rules in `documents/process/style_guide/*.md` keep their existing heading (`### <Name> :: required` or `:: suggested`, or `##` when the page has one level of rules) and add up to four short blocks:

````markdown
## <Rule name> :: required | suggested

<Rule statement and context.>

**Why:** <The reason ICU4X requires or suggests this, with links to design docs.>

**❌ Don't:**

```rust
// Minimal bad example
```

**✅ Do:**

```rust
// Minimal good example, pointing to a real file in the repo
```

**Enforcement:** <Clippy lint, CI job, `depcheck`, `bakeddata`, or review only.>
````

### Teaching chapter template

````markdown
# <Title>

- **Prerequisites:** <chapters or external Rust book sections to read first>
- **Related docs:** <style guide pages and design docs, and what each covers>
- **Checked against:** ICU4X <x.y> (`main` @ `<sha>`, <YYYY-MM-DD>)

## Why this matters in ICU4X

<The real ICU4X problem. No Rust details yet.>

## The concept (outside ICU4X)

<A minimal standalone example in a `rust` block. It must compile and run.>

## In ICU4X

<Real code. Name the file above each excerpt. Excerpts are `rust,ignore` and
match the source, or say "abridged". Prefer runnable `rust` examples that use
public APIs.>

## Common mistakes

| Mistake | Do instead | Style guide rule |
|---|---|---|

## Exercises

<2–3 tasks. Each solution is in a `<details>` block and compiles.>

## Checklist before your PR

- [ ] <item>
````

---

## 3. Style guide index and teaching chapters index

### File: `documents/process/style_guide/README.md` (header from #8585 and #8587)

````markdown
ICU4X Style Guide
=================

This document outlines the style guide and best practice for code in ICU4X, with a focus on Rust code style.

## Contents

Each area is a section of this page or has its own page. You can link to a page or to a rule on it, for example [Don't Panic](errors_and_panics.md#dont-panic--required).

- [Naming Conventions](#naming-conventions)
- [Module and Code Layout](#module-and-code-layout)
- [Code Formatting and Linting](#code-formatting-and-linting)
- [Private vs Public](#private-vs-public)
- [Derived Traits](#derived-traits)
- [Sized Types](#sized-types)
- [Pass by Reference vs Pass by Value](#pass-by-reference-vs-pass-by-value)
- [Option](#option)
- [Iteration](#iteration)
- [Enums](#enums)
- [Matching](#matching)
- [Structs with Private Fields](#structs-with-private-fields)
- [Options structs With All Public Fields](#options-structs-with-all-public-fields)
- [Data Types](data_types.md) (own page)
- [Errors and Panics](errors_and_panics.md) (own page)
- [Lints](#lints)
- [Crate Features and Dependencies](crates.md) (own page)
- [Operator Overloading](#operator-overloading)
- [Binding Traits to Inbuilt Types](#binding-traits-to-inbuilt-types)
- [Appendix](#appendix)
````

### File: `documents/contributor_guide/README.md` (proposed index for teaching chapters)

````markdown
# ICU4X contributor guide

This guide explains the Rust patterns that ICU4X depends on. They are easy to
miss, especially for new contributors: the code shows what ICU4X does, but not
why, or which patterns are rules.

It is written for three audiences:

- **New contributors** who know Rust, but not ICU4X.
- **Reviewers** who want a link that explains a pattern and why ICU4X uses it.
- **Future maintainers** who want design decisions and their reasons kept in
  the repository.

This guide teaches. It does not replace the process and design docs:

| Topic | Document |
|---|---|
| Building, testing, regenerating files, first-PR checklist | [CONTRIBUTING.md](https://github.com/unicode-org/icu4x/blob/main/CONTRIBUTING.md) |
| Coding rules (with ❌/✅ examples and reasons) | [Style Guide](../process/style_guide/README.md) |
| Adding a data struct step by step | [writing_a_new_data_struct.md](../process/writing_a_new_data_struct.md) |
| Design principles | [principles.md](../design/principles.md) |
| Invalid data and panics | [data_safety.md](../design/data_safety.md) |
| Data stability | [data_versioning.md](../process/data_versioning.md) |
| Requirements for stable components | [graduation.md](../process/graduation.md) |

## Chapters

| Chapter | Read it when |
|---|---|
| [i18n_basics.md](i18n_basics.md) | Locales, CLDR, or Unicode properties are new to you. |
| [cow_and_borrowing.md](cow_and_borrowing.md) | Before your first code PR. |
| [zerovec_and_ule.md](zerovec_and_ule.md) | You change a data struct, or use `zerovec`. |
| [yoke_and_zerofrom.md](yoke_and_zerofrom.md) | You wonder what `'data`, `Yokeable`, or `DataPayload` are for. |
| [data_provider.md](data_provider.md) | You write a constructor, or load data. |
| [ffi_diplomat.md](ffi_diplomat.md) | You add or change a public API. Stable APIs also need FFI. |

[glossary.md](glossary.md) explains ICU4X terms like *marker*, *payload*,
*baked data*, and *GIGO*.

*Checked against: ICU4X 2.3 (`main` @ `77a3b6ddfb`, 2026-10-09).*
````

---

## 4. Sample chapter

### File: `documents/contributor_guide/cow_and_borrowing.md`

````markdown
# Cow and borrowing

- **Prerequisites:** ownership, references, and lifetimes (chapters 4 and 10
  of [the Rust book](https://doc.rust-lang.org/book/)), and
  [CONTRIBUTING.md](https://github.com/unicode-org/icu4x/blob/main/CONTRIBUTING.md).
- **Related docs:**
  - [ICU4X Style Guide](../process/style_guide/README.md) has the rules:
    [Avoid implicit allocations](../process/style_guide/README.md#avoid-implicit-allocations--suggested),
    [Avoid `to_string`](../process/style_guide/errors_and_panics.md#avoid-to_string--suggested),
    [Conventions for strings in structs](../process/style_guide/data_types.md#conventions-for-strings-in-structs--suggested),
    and [Zero-copy in DataProvider structs](../process/style_guide/data_types.md#zero-copy-in-dataprovider-structs--required).
    This chapter explains why they exist.
  - [string_representation.md](../design/string_representation.md) covers
    text encodings and caller-allocated output at the API boundary. This
    chapter does not repeat it.
  - [zerovec_and_ule.md](zerovec_and_ule.md) continues with data structs.
- **Checked against:** ICU4X 2.3 (`main` @ `77a3b6ddfb`, 2026-10-09)

## Why this matters in ICU4X

ICU4X runs on servers and phones, in browsers (WebAssembly), and on small
devices. Some of its crates even work without a heap (see
`tools/noalloctest/README.md`). So ICU4X allocates memory only when it has
to.

Many operations often return their input unchanged:

- Normalizing `"hello"` to NFC gives `"hello"`.
- Lowercasing `"hello world"` gives `"hello world"`.
- A locale string that is already in canonical form stays the same.

A function that returns `String` allocates on every call, even when nothing
changed. A function that returns `Cow<'a, str>` can return the input
(`Cow::Borrowed`). It allocates only when the output is different
(`Cow::Owned`).

Locale data has the same problem. One data struct must work with data from
three sources:

| Data source | Where the bytes are | What the struct holds |
|---|---|---|
| Compiled data (baked into the binary) | Static memory | `&'static` references |
| Blob data (loaded at runtime) | A buffer that the provider owns | References into the buffer |
| Data built at runtime (for example by datagen) | The heap | Owned values |

With `Cow`, one struct type supports all three. This is why ICU4X data
structs have so many `Cow<'data, str>` fields.

## The concept (outside ICU4X)

`Cow<'a, B>` ("clone on write") is an enum from the `alloc` crate. `std`
re-exports it as `std::borrow::Cow`. Simplified:

```rust,ignore
pub enum Cow<'a, B: ?Sized + ToOwned> {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned), // for B = str, this is String
}
```

A function can return either variant. The caller reads both in the same way,
because `Cow` dereferences to `&B`:

```rust
use std::borrow::Cow;

/// Replaces "\r\n" with "\n". Allocates only if the input contains "\r\n".
fn unix_newlines(input: &str) -> Cow<'_, str> {
    if input.contains("\r\n") {
        Cow::Owned(input.replace("\r\n", "\n"))
    } else {
        Cow::Borrowed(input)
    }
}

let unchanged = unix_newlines("one\ntwo");
assert!(matches!(unchanged, Cow::Borrowed(_))); // no allocation

let changed = unix_newlines("one\r\ntwo");
assert!(matches!(changed, Cow::Owned(_)));
assert_eq!(changed, "one\ntwo");

// Need a String? `into_owned` allocates only if the value is borrowed.
let s: String = unchanged.into_owned();
assert_eq!(s, "one\ntwo");
```

A struct field can be a `Cow`, too. Then the same type can borrow or own:

```rust
use std::borrow::Cow;

struct Label<'data> {
    text: Cow<'data, str>,
}

// 1. Borrow from a string literal (like compiled data).
let compiled: Label<'static> = Label { text: Cow::Borrowed("Hello") };

// 2. Borrow from a buffer that someone else owns (like blob data).
let buffer = String::from("Bonjour");
let loaded: Label<'_> = Label { text: Cow::Borrowed(&buffer) };

// 3. Own the value (like data built at runtime). It is still `'static`.
let built: Label<'static> = Label { text: Cow::Owned(String::from("Hallo")) };

assert_eq!(compiled.text, "Hello");
assert_eq!(loaded.text, "Bonjour");
assert_eq!(built.text, "Hallo");
```

A `&'data str` field can do 1 and 2, but not 3. A `String` field can do 3,
but must copy in 1 and 2. Only `Cow` does all three.

Which type to use:

| Situation | Use |
|---|---|
| A parameter that is only read | `&str`, `&[T]`, `&T` |
| A parameter that the function stores | `String` or `T` by value, so the caller decides when to allocate |
| A result that is often the input, unchanged | `Cow<'a, str>` |
| A result that is new, formatted text | A type that implements `Writeable` (see below) |
| A string field in a struct | See [Conventions for strings in structs](../process/style_guide/data_types.md#conventions-for-strings-in-structs--suggested): `Cow<'data, str>` in data structs |

## In ICU4X

### 1. Return the input when nothing changes

`components/normalizer/src/lib.rs`, method `normalize` (inside the
`normalizer_methods!` macro):

```rust,ignore
pub fn normalize<'a>(&self, text: &'a str) -> Cow<'a, str> {
    let (head, tail) = self.split_normalized(text);
    if tail.is_empty() {
        return Cow::Borrowed(head);
    }
    let mut ret = String::new();
    ret.reserve(text.len());
    ret.push_str(head);
    let _ = self.normalize_to(tail, &mut ret);
    Cow::Owned(ret)
}
```

`split_normalized` finds the longest prefix that is already normalized. If
that is the whole text, the function borrows. If not, it allocates once and
normalizes only the rest.

Many other APIs get the same behavior from one helper:
`writeable::to_string_or_borrow` (`utils/writeable/src/to_string_or_borrow.rs`).
It compares the output with the input while it writes, and allocates only
when they start to differ. From `components/casemap/src/casemapper.rs`:

```rust,ignore
pub fn lowercase_to_string<'s>(
    self,
    src: &'s str,
    langid: &LanguageIdentifier,
) -> Cow<'s, str> {
    writeable::to_string_or_borrow(&self.lowercase(src, langid), src.as_bytes())
}
```

You can see both cases:

```rust
use icu::casemap::CaseMapper;
use icu::locale::{langid, Locale};
use std::borrow::Cow;

let cm = CaseMapper::new();
let root = langid!("und");

assert!(matches!(cm.lowercase_to_string("hello world", &root), Cow::Borrowed(_)));
assert!(matches!(cm.lowercase_to_string("Hello World", &root), Cow::Owned(_)));

// `Locale::normalize` uses the same helper.
assert!(matches!(Locale::normalize("pl-Latn-PL"), Ok(Cow::Borrowed(_))));
assert!(matches!(Locale::normalize("pL-latn-pl"), Ok(Cow::Owned(_))));
```

### 2. Return a `Writeable`, not a `String`

A formatter creates new text, so there is nothing to borrow. But the caller
may not need a `String` at all. It may write into an existing buffer, a
`fmt::Formatter`, or a sink from another language. So ICU4X formatters
return a value that implements `writeable::Writeable`, and the caller picks
the sink. This follows "Prefer working with caller-allocated memory" in
[string_representation.md](../design/string_representation.md).

From `components/decimal/src/decimal_formatter.rs`:

```rust,ignore
pub fn format<'l>(&'l self, value: &'l Decimal) -> FormattedDecimal<'l> {
    FormattedDecimal(self.format_sign(
        value.sign,
        self.format_unsigned(Cow::Borrowed(&value.absolute)),
    ))
}

#[cfg(feature = "alloc")]
pub fn format_to_string(&self, value: &Decimal) -> String {
    use writeable::Writeable;
    self.format(value).write_to_string().into_owned()
}
```

`format` only borrows the formatter and the input. It does not format
anything yet. `format_to_string` is a shortcut that is built on `format`,
and it needs the `alloc` feature. The FFI layer writes the same value
straight into a sink that the caller owns (`ffi/capi/src/decimal.rs`):

```rust,ignore
pub fn format(&self, value: &Decimal, write: &mut diplomat_runtime::DiplomatWrite) {
    let _infallible = self.0.format(&value.0).write_to(write);
}
```

`Writeable::write_to_string` returns a `Cow`, too. This is its default
implementation (`utils/writeable/src/lib.rs`):

```rust,ignore
fn write_to_string(&self) -> Cow<'_, str> {
    if let Some(borrow) = self.writeable_borrow() {
        return Cow::Borrowed(borrow);
    }
    let hint = self.writeable_length_hint();
    if hint.is_zero() {
        return Cow::Borrowed("");
    }
    let mut output = String::with_capacity(hint.capacity());
    let _ = self.write_to(&mut output);
    Cow::Owned(output)
}
```

It borrows when it can. If not, it allocates once, and uses the length hint
as the capacity. This is why a correct `writeable_length_hint` matters.

```rust
use icu::decimal::input::Decimal;
use icu::decimal::DecimalFormatter;
use icu::locale::locale;
use writeable::assert_writeable_eq;

let formatter = DecimalFormatter::try_new(locale!("en").into(), Default::default())
    .expect("compiled data includes en");
let value = Decimal::from(1234567);

// `assert_writeable_eq!` checks the text, and also the length hint.
assert_writeable_eq!(formatter.format(&value), "1,234,567");
```

### 3. Data structs: `Cow<'data, str>` with `#[serde(borrow)]`

`components/datetime/src/provider/time_zones.rs` (abridged):

```rust,ignore
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct TimeZoneEssentials<'data> {
    pub offset_separator: Cow<'data, str>,
    pub offset_pattern: Cow<'data, SinglePlaceholderPattern>,
    pub offset_unknown: Cow<'data, str>,
}

// Inside the hand-written `Deserialize` impl:
#[derive(serde::Deserialize)]
struct Raw<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    offset_separator: Cow<'data, str>,
    // ...
}
```

- Compiled data creates this struct with `Cow::Borrowed` and `&'static`
  strings.
- Blob data deserializes it. With `#[serde(borrow)]`, each `Cow` points into
  the blob buffer.
- Datagen builds it from CLDR with `Cow::Owned`.

Without `#[serde(borrow)]`, serde always creates `Cow::Owned`. The code still
works, but every load allocates. CI catches this: the zero-copy check
(`ZeroCopyCheckExporter` in `tools/make/bakeddata/src/main.rs`) deserializes
every payload and counts the allocations. When it fails, it says:

```text
Common cause: did you forget to add `serde(borrow)` to all of the fields in your data struct?
```

Why is `Deserialize` hand-written here? To keep old and new data compatible
after [#8250](https://github.com/unicode-org/icu4x/pull/8250) removed a field.
See [Keep the serialized layout of stable data structs](../process/style_guide/data_types.md#keep-the-serialized-layout-of-stable-data-structs--required).

Some newer data structs use `VarZeroCow<'data, str>` instead (for example
`ListFormatterPatterns` in `components/list/src/provider/mod.rs`).
[zerovec_and_ule.md](zerovec_and_ule.md) explains it.
TODO(verify): which of the two new data structs should use.

### 4. Borrowed and owned types: `CaseMapperBorrowed` and `CaseMapper`

The same idea works for whole types. From
`components/casemap/src/casemapper.rs` (abridged):

```rust,ignore
pub struct CaseMapper {
    pub(crate) data: DataPayload<CaseMapV1>,
}

#[derive(Clone, Debug, Copy)]
pub struct CaseMapperBorrowed<'a> {
    pub(crate) data: &'a CaseMap<'a>,
}

impl CaseMapper {
    pub const fn new() -> CaseMapperBorrowed<'static> { /* compiled data */ }
    pub fn as_borrowed(&self) -> CaseMapperBorrowed<'_> { /* ... */ }
}

impl CaseMapperBorrowed<'static> {
    pub const fn static_to_owned(self) -> CaseMapper { /* ... */ }
}
```

- `CaseMapperBorrowed<'a>` holds a plain reference. It is `Copy`. With
  compiled data, you can even create it in a `const`.
- `CaseMapper` owns a `DataPayload`. You need it when the data comes from a
  provider at runtime (`try_new_unstable`, `try_new_with_buffer_provider`).
  `DataPayload` works like a `Cow` for data structs: it holds either a
  `&'static` struct (`DataPayload::from_static_ref`) or data that it owns.
  [yoke_and_zerofrom.md](yoke_and_zerofrom.md) explains how.
- [graduation.md](../process/graduation.md) requires this shape: if there is
  a borrowed type, `Foo::new()` returns `FooBorrowed`
  ([#5440](https://github.com/unicode-org/icu4x/issues/5440)).

```rust
use icu::casemap::{CaseMapper, CaseMapperBorrowed};
use icu::locale::langid;

// Compiled data: no allocation, and it works in a `const`.
const CM: CaseMapperBorrowed<'static> = CaseMapper::new();
assert_eq!(CM.uppercase_to_string("Straße", &langid!("und")), "STRASSE");

// An owned value, for code that has to store a `CaseMapper`:
let owned: CaseMapper = CM.static_to_owned();
assert_eq!(owned.as_borrowed().lowercase_to_string("ABC", &langid!("und")), "abc");
```

### 5. `Cow` needs `alloc`

`Cow` comes from the `alloc` crate. ICU4X library crates are `no_std`
([Use `no_std`](../process/style_guide/crates.md#use-no_std--suggested)), so write
`alloc::borrow::Cow`, not `std::borrow::Cow`.

A crate that must also work without `alloc` can't use `Cow` at all. For
example, `components/decimal/src/lib.rs` defines its own small `Cow` enum
when the `alloc` feature is off. This is rare. If you think you need it, ask
in your issue first. [Crate Features and Dependencies](../process/style_guide/crates.md)
has the details.

## Common mistakes

| Mistake | Do instead | Style guide rule |
|---|---|---|
| Return `String` when the result is often the input | Return `Cow<'a, str>` | [Avoid implicit allocations](../process/style_guide/README.md#avoid-implicit-allocations--suggested) |
| Call `format!` or `to_string()` in library code | Return a type that implements `Writeable` | [Avoid `to_string`](../process/style_guide/errors_and_panics.md#avoid-to_string--suggested) |
| Take `&T`, then clone it inside | Take `T` by value | [Avoid implicit allocations](../process/style_guide/README.md#avoid-implicit-allocations--suggested) |
| Call `.into_owned()` "just in case" | Keep the `Cow` until you really need a `String` | [Avoid implicit allocations](../process/style_guide/README.md#avoid-implicit-allocations--suggested) |
| Use `String`, `Vec`, or `&'data str` in a data struct | Use `Cow<'data, str>` or a `zerovec` type | [Zero-copy in DataProvider structs](../process/style_guide/data_types.md#zero-copy-in-dataprovider-structs--required) |
| Forget `#[serde(borrow)]` on a `Cow` field | Add `#[cfg_attr(feature = "serde", serde(borrow))]` | [Zero-copy in DataProvider structs](../process/style_guide/data_types.md#zero-copy-in-dataprovider-structs--required) |
| Use `std::borrow::Cow` in a library crate | Use `alloc::borrow::Cow` | [Use `no_std`](../process/style_guide/crates.md#use-no_std--suggested) |

## Exercises

### Exercise 1: Borrow when you can

a) Write `typographic_apostrophes`. It replaces `'` with `’`
(U+2019 RIGHT SINGLE QUOTATION MARK). It must not allocate when the input
has no `'`.

```rust,ignore
fn typographic_apostrophes(input: &str) -> Cow<'_, str> {
    todo!()
}

assert!(matches!(typographic_apostrophes("hello"), Cow::Borrowed("hello")));
assert_eq!(typographic_apostrophes("don't"), "don’t");
```

b) This code does not compile. Why not? How do you fix it?

```rust,compile_fail
use std::borrow::Cow;

fn shout(input: &str) -> Cow<'_, str> {
    let upper = input.to_uppercase();
    Cow::Borrowed(&upper)
}
```

<details>
<summary>Solution</summary>

a)

```rust
use std::borrow::Cow;

fn typographic_apostrophes(input: &str) -> Cow<'_, str> {
    if input.contains('\'') {
        Cow::Owned(input.replace('\'', "’"))
    } else {
        Cow::Borrowed(input)
    }
}

assert!(matches!(typographic_apostrophes("hello"), Cow::Borrowed("hello")));
assert_eq!(typographic_apostrophes("don't"), "don’t");
```

b) `upper` is a local `String`. It is dropped when `shout` returns, so a
reference to it can't leave the function (error E0515). Return the owned
value instead: `Cow::Owned(upper)`. A real implementation would also return
`Cow::Borrowed(input)` when the text is already uppercase, like
`CaseMapperBorrowed::uppercase_to_string` does.

</details>

### Exercise 2: Find the allocation

This made-up data struct compiles and works. But each time it is loaded from
blob data, it allocates. Why?

```rust,ignore
#[derive(serde::Deserialize)]
pub struct CityNames<'data> {
    pub capital: Cow<'data, str>,
    pub largest: Cow<'data, str>,
}
```

<details>
<summary>Solution</summary>

The fields have no `#[serde(borrow)]`. Without it, serde deserializes every
`Cow` as `Cow::Owned`, which copies the string to the heap. In ICU4X, add
`#[cfg_attr(feature = "serde", serde(borrow))]` to each field (serde is an
optional dependency). The CI zero-copy check reports this bug (see section 3).
Note that `#[serde(borrow)]` has known problems with `Option` fields
([serde#2016](https://github.com/serde-rs/serde/issues/2016)).

You can see the difference without a data file. serde's
`BorrowedStrDeserializer` acts like a format that lends out its strings:

```rust
use serde::de::value::{BorrowedStrDeserializer, Error, SeqDeserializer};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

#[derive(Deserialize)]
struct WithBorrow<'data> {
    #[serde(borrow)]
    name: Cow<'data, str>,
}

#[derive(Deserialize)]
struct WithoutBorrow<'data> {
    name: Cow<'data, str>,
}

fn input() -> impl Deserializer<'static, Error = Error> {
    SeqDeserializer::new([BorrowedStrDeserializer::new("Oslo")].into_iter())
}

let with = WithBorrow::deserialize(input()).expect("valid input");
let without = WithoutBorrow::deserialize(input()).expect("valid input");
assert!(matches!(with.name, Cow::Borrowed("Oslo")));
assert!(matches!(without.name, Cow::Owned(_)));
```

</details>

### Exercise 3: Return a `Writeable`

This function allocates on every call:

```rust
fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

assert_eq!(greeting("Ada"), "Hello, Ada!");
```

Write a type `Greeting<'a>` that implements `writeable::Writeable`, with an
exact `writeable_length_hint`. Give it `Display` with
`writeable::impl_display_with_writeable!`. Test it with
`writeable::assert_writeable_eq!`.

<details>
<summary>Solution</summary>

```rust
use core::fmt;
use writeable::{LengthHint, Writeable};

struct Greeting<'a> {
    name: &'a str,
}

impl Writeable for Greeting<'_> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')
    }

    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact("Hello, !".len()) + self.name.len()
    }
}

writeable::impl_display_with_writeable!(Greeting<'_>);

writeable::assert_writeable_eq!(Greeting { name: "Ada" }, "Hello, Ada!");
```

Now the caller chooses: write into an existing sink with no new allocation,
or call `write_to_string()` and get one allocation of the right size.

</details>

## Checklist before your PR

- [ ] Parameters that are only read are references (`&str`, `&T`).
      Parameters that are stored are taken by value.
- [ ] Functions that often return their input unchanged return `Cow`. Tests
      cover both `Cow::Borrowed` and `Cow::Owned`.
- [ ] Formatting APIs return a `Writeable` type. Any `*_to_string` helper is
      built on it.
- [ ] Every `Writeable` has a correct `writeable_length_hint`, tested with
      `assert_writeable_eq!`.
- [ ] Strings in data structs are `Cow<'data, str>` or `zerovec` types, each
      with `#[cfg_attr(feature = "serde", serde(borrow))]`.
- [ ] If you changed a data struct, `cargo make bakeddata <component>` passes.
      It runs the zero-copy check.
- [ ] Library code has no `std::` paths. Use `core::` and `alloc::`.
````

---

## 5. Style guide rule updates and additions

Instead of a separate `avoid/AVnnn_*.md` tree, the 15 rules from the initial draft map directly into `documents/process/style_guide/` (or `CONTRIBUTING.md`). "Kind" shows whether a rule expands an existing style guide section or adds a new heading. `TODO(verify)` marks points that are not yet in a written policy document.

| # | Rule | Target page & heading | Kind | Enforced by | Source | Status |
|---|---|---|---|---|---|---|
| 1 | No `unwrap`, `expect`, indexing, or `panic!` in library code | `errors_and_panics.md`: `Don't Panic :: required` | Expand existing | Clippy (`unwrap_used`, `expect_used`, `indexing_slicing`, `panic`), review | `style_guide`: Don't Panic, Lints › Panics; `data_safety.md` | In [#8584](https://github.com/unicode-org/icu4x/pull/8584) |
| 2 | Don't allocate when you can borrow (`Cow`, `Writeable`) | `passing_values.md`: `Avoid implicit allocations :: suggested`; `errors_and_panics.md`: `Avoid to_string :: suggested` | Expand existing | Review | `style_guide`; `string_representation.md`; pattern in `icu_normalizer`, `icu_casemap` | Planned |
| 3 | `no_std` + `alloc` in library crates; `std` behind a feature | `crates.md`: `Use no_std :: suggested` | Rewrite stale rule | Compiler (`no_std`), `ci-job-nostd` | `principles.md`: No std in core library; `boilerplate.md`; `graduation.md` | In [#8587](https://github.com/unicode-org/icu4x/pull/8587) |
| 4 | Zero-copy types and `#[serde(borrow)]` in data structs | `data_types.md`: `Zero-copy in DataProvider structs :: required` | Expand existing | `cargo make bakeddata` (`ZeroCopyCheckExporter`) | `style_guide`: Zero-copy in DataProvider structs; `graduation.md` | In [#8584](https://github.com/unicode-org/icu4x/pull/8584) |
| 5 | Keep the serialized layout of stable data structs | `data_types.md`: `Keep the serialized layout of stable data structs :: required` | New rule | Review (`fingerprints.csv` diff) | `data_versioning.md`; [#8250](https://github.com/unicode-org/icu4x/pull/8250) | In [#8584](https://github.com/unicode-org/icu4x/pull/8584) |
| 6 | Prefer `#[zerovec::make_ule]` / `make_varule`; follow the safety checklist for hand-written `unsafe impl ULE`/`VarULE` | `data_types.md`: `ULE and VarULE safety :: required` | New rule | Review | `utils/zerovec/src/ule/mod.rs` (safety checklist); `principles.md`: Safety. `TODO(verify)`: "prefer derives" is not written policy | Planned |
| 7 | Public APIs must work through Diplomat FFI or be listed in `ffi/capi/tests/missing_apis.txt` | `api_design.md`: `FFI coverage for public APIs :: required` | New rule | `ci-job-diplomat`, review | `graduation.md` (FFI coverage); `principles.md`: Available Across Programming Languages | Planned |
| 8 | Keep runtime dependencies on the `depcheck` allowlist; gate optional deps behind features | `crates.md`: `When to add crate features :: suggested`, `Avoid heavy dependencies :: suggested` | Expand existing | `cargo make depcheck` (`ci-job-tidy`) | `tools/make/depcheck/src/allowlist.rs`; `style_guide` | In [#8587](https://github.com/unicode-org/icu4x/pull/8587) |
| 9 | Invalid data never panics; avoid expensive validation and handle bad data with GIGO (`debug_assert!` + fallback) | `errors_and_panics.md`: `Invalid data and GIGO :: required` | New rule | `ci-job-test-gigo`, review | `data_safety.md` | Planned |
| 10 | `#[non_exhaustive]` on options and error types, not on data structs | `lints.md`: `Exhaustiveness :: required` | Expand existing | Clippy (`exhaustive_structs`, `exhaustive_enums`) | `style_guide`: Lints › Exhaustiveness | Planned |
| 11 | Standard constructor set (`new`, `try_new`, `*_unstable`, `*_with_buffer_provider`) and `(provider, prefs, options)` order | `api_design.md`: `Constructor conventions :: required` | Rewrite stale rule | Review | `graduation.md`; `icu_provider::constructors` docs | Planned |
| 12 | No global caches or global mutable state | `api_design.md`: `No global caches :: required` | New rule | Review | `principles.md`: No global caches | Planned |
| 13 | Never edit generated files by hand (baked data, testdata, FFI bindings, READMEs) | `CONTRIBUTING.md` (regenerating generated files) | Expand existing | CI: `bakeddata-check`, `testdata-check`, `verify-diplomat-gen`, `generated-readme-check`, `codegen-check` | `CONTRIBUTING.md` | Planned |
| 14 | Datagen options must not change what a marker's data means; use a separate marker or marker attributes | `data_types.md`: `Datagen options and marker semantics :: required` | New rule | Review | `principles.md`: Locale data from multiple sources works seamlessly; `data_architecture.md` | Planned |
| 15 | Every `TODO` or `FIXME` links an issue (`TODO(#1234)`) | `layout_and_formatting.md`: `TODO comments :: suggested` | New rule | Review | `graduation.md` | Planned |

---

## 6. Sample style guide rules (from #8584 and #8587)

These four rules show how `**Why:**`, `**❌ Don't:**`, `**✅ Do:**`, and `**Enforcement:**` fit directly into the style guide pages across the three kinds of enforcement: Clippy (`Don't Panic`), a CI tool (`Zero-copy`, `Use no_std`), and review (`Keep the serialized layout of stable data structs`).

### In `documents/process/style_guide/errors_and_panics.md` (#8584, #8585)

````markdown
### Don't Panic :: required

Call non-panicking data access APIs whenever data is not guaranteed to be safe.

This should not include the contract of code in a different Crate. I.e. if a function in a different Crate promises to return a valid map key, but it's not a compile time checked type (like an enum), then the calling code must allow for it to fail.

See also: the [Panics](README.md#panics--required) section of the style guide.

**Why:** A panic in a library stops the whole program that uses it. ICU4X also loads data that it did not create, and [data_safety.md](../../design/data_safety.md) says that code should never panic at runtime based on invalid data. Otherwise, bad data can crash the application.

**❌ Don't:**

```rust
let (key, value) = input.split_once('=').unwrap(); // panics if there is no '='
let first = names[0];                              // panics if `names` is empty
```

**✅ Do:** If the caller can do something about the problem, return an error:

```rust
pub fn parse_pair(input: &str) -> Result<(&str, &str), ParseError> {
    input.split_once('=').ok_or(ParseError::MissingEquals)
}
```

If the problem can only come from invalid data or an internal bug, use a fallback value and a debug assertion. Tests and debug builds find the bug, and release builds continue with the fallback ("garbage in, garbage out"). From `split_normalized` in `components/normalizer/src/lib.rs`:

```rust
text.split_at_checked(up_to).unwrap_or_else(|| {
    // Internal bug, not even GIGO, never supposed to happen
    debug_assert!(false);
    ("", text)
})
```
````

### In `documents/process/style_guide/data_types.md` (#8584, #8585, #8560)

````markdown
## Zero-copy in DataProvider structs :: required

All data structs that can be passed through the DataProvider pipeline must support *zero-copy deserialization:* in practice, no heap allocations should be required when deserializing from Bincode-like formats. This means that if the type involves variable-length data like strings, vectors, and maps, it must use a zero-copy type backed by a byte buffer to represent them.

Data structs with zero-copy data should have a `'data` lifetime parameter.

In order to enable zero-copy deserialization via Serde, the `#[serde(borrow)]` annotation is most likely required. However, be aware of [known bugs](https://github.com/serde-rs/serde/issues/2016) regarding `#[serde(borrow)]` with `Option` types.

Examples of types that can be used in zero-copy data structs:

- Strings: `Cow<'data, str>`, except as noted below
- Vectors of fixed-width types: `ZeroVec<'data, T>`
    - Examples: `ZeroVec<'data, u32>`, `ZeroVec<'data, TinyStr8>`
- Vectors of variable-width types: `VarZeroVec<'data, T>`
    - Example: `VarZeroVec<'data, str>`
- Maps: `ZeroMap<'data, K, V>`
    - Example: `ZeroMap<'data, TinyStr4, str>`

In addition to supporting zero-copy deserialization, data structs should also support being fully owned (`'static`). For example, `&str` or `&T` require that the data be borrowed from somewhere, and so cannot be used in a data struct. `Cow` and all the other types listed above support the optional ownership model.

**❌ Don't:**

```rust
pub struct CityNames<'data> {
    pub names: Vec<String>,         // allocates on every load
    pub separator: Cow<'data, str>, // no #[serde(borrow)]: allocates too
}
```

**✅ Do:**

```rust
#[derive(Clone, Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CityNames<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: VarZeroVec<'data, str>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub separator: Cow<'data, str>,
}
```

Real examples: `ListFormatterPatterns` in `components/list/src/provider/mod.rs` and `TimeZoneEssentials` in `components/datetime/src/provider/time_zones.rs`.

**Why:** Blob data is a byte buffer that is loaded at runtime. A zero-copy struct borrows from that buffer, so loading copies nothing to the heap. Compiled data uses the same struct with `'static` borrows, and datagen builds it from owned values, so each field must be able to borrow and to own. Without `#[serde(borrow)]`, serde deserializes a `Cow` as `Cow::Owned`, which allocates.

**Enforcement:** `cargo make bakeddata` deserializes every payload with an allocator that counts allocations, and fails on new ones (`ZeroCopyCheckExporter` in `tools/make/bakeddata/src/main.rs`). Its list of allowed violations is empty. Memory that is allocated and freed again during deserialization (for example, to validate data) is allowed only for the markers in `EXPECTED_TRANSIENT_VIOLATIONS`. Ask the ICU4X team before you add a marker to either list.
````

````markdown
## Keep the serialized layout of stable data structs :: required

Main policy: [data_versioning.md](../data_versioning.md)

Data files must stay readable across ICU4X versions: older code reads newer data, and newer code reads data built for any version with the same major version number. Postcard, the blob data format, doesn't store field names; it reads fields in order. So if you remove, reorder, or retype a field of a data struct that shipped in a stable release, existing data files break, even after all data in the repo is regenerated.

**❌ Don't:** Remove a field from a released data struct:

```diff
 pub struct TimeZoneEssentials<'data> {
     pub offset_separator: Cow<'data, str>,
     pub offset_pattern: Cow<'data, SinglePlaceholderPattern>,
-    pub offset_zero: Cow<'data, str>,
     pub offset_unknown: Cow<'data, str>,
 }
```

**✅ Do:** Keep the serialized layout with hand-written serde impls, or add a new marker next to the old one ([Retain Old Keys When Possible](../data_versioning.md#ii-retain-old-keys-when-possible)). [#8250](https://github.com/unicode-org/icu4x/pull/8250) removed `offset_zero` from the Rust struct, but the serde impls in `components/datetime/src/provider/time_zones.rs` still read the field and write a placeholder:

```rust
// Deserialize: read the old field, then drop it.
let Raw { offset_separator, offset_pattern, offset_unknown, offset_zero: _offset_zero } =
    Raw::deserialize(deserializer)?;

// Serialize (datagen only): write a placeholder, so old code can read new data.
offset_zero: Cow::Borrowed(""),
```

Reviewers see changes to serialized data in the `provider/data/*/fingerprints.csv` diff.
````

### In `documents/process/style_guide/crates.md` (#8587)

````markdown
### Use no_std :: suggested

Main issues: [#77](https://github.com/unicode-org/icu4x/issues/77), [#151](https://github.com/unicode-org/icu4x/issues/151)

Library crates are `no_std`: they use `core` and `alloc` instead of `std`. Allocating is fine. [principles.md](../../design/principles.md#no-standard-library-dependencies-in-the-core-library) says that the `icu` crate and all of its dependencies "should be `#[no_std]`, but may use the `alloc` crate".

**Why:** ICU4X runs in resource-constrained environments that don't have a standard library.

**❌ Don't:**

```rust
use std::collections::BTreeMap;
use std::string::String;
```

**✅ Do:** Start `lib.rs` with the [library annotations](../boilerplate.md#library-annotations), and import from `alloc`:

```rust
#![cfg_attr(not(any(test, doc)), no_std)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
```

Add an `std` feature only for code that needs `std`, such as file I/O.

**Enforcement:** `cargo make ci-job-nostd` builds `icu_capi`, which depends on all components, for a target without `std` (`thumbv7m-none-eabi`).
````

---

## 7. Maintenance plan

| Mechanism | What it catches | Cost |
|---|---|---|
| **md-tests.** Add the style guide pages (once pseudocode blocks are marked `rust,ignore`) and every teaching chapter to `tools/md-tests/src/lib.rs` (snippet below). | `rust` examples that break after an API change, and `compile_fail` examples that start to compile. | Already in CI: `ci-job-test-docs` runs `cargo test --all-features --doc`. |
| **Link and path check.** Check relative Markdown links, `#anchor` slugs, and inline file paths (`git ls-files`). | Broken links or moved files after refactors. | Small script; can be wired into `cargo tidy`. |
| **Version marker.** `Checked against: ICU4X x.y (main @ sha)` in teaching chapters. | Shows readers how old a chapter is, and gives a diff base: `git diff --stat <sha>..main -- <cited paths>`. | Manual, at each minor release. |
| **Feedback loop.** When reviewers see the same mistake twice in PRs, add or sharpen the rule on the corresponding style guide page with a ❌/✅ example. | Mistakes that were only tribal knowledge. | — |

md-tests hook (include paths are relative to `tools/md-tests/src/lib.rs`, as the existing entries are):

```rust
mod style_guide {
    #[doc = include_str!("../../../documents/process/style_guide/data_types.md")]
    mod data_types_md {}
    #[doc = include_str!("../../../documents/process/style_guide/errors_and_panics.md")]
    mod errors_and_panics_md {}
    #[doc = include_str!("../../../documents/process/style_guide/crates.md")]
    mod crates_md {}
}

mod contributor_guide {
    #[doc = include_str!("../../../documents/contributor_guide/README.md")]
    mod readme_md {}
    #[doc = include_str!("../../../documents/contributor_guide/cow_and_borrowing.md")]
    mod cow_and_borrowing_md {}
}
```

Rustdoc treats every code block without a language tag as Rust, so non-Rust blocks must be tagged (`text`, `toml`, `diff`, `console`), and illustrative snippets in the style guide must be tagged `rust,ignore`.

**Draft verification.** The sample chapter in §4 was tested in a crate that includes it the same way `tools/md-tests` does, with the same `icu` features (`compiled_data`, `serde`): all `rust` blocks compile and pass, and the `compile_fail` block fails to compile as expected. Note: stable rustdoc does not check error codes such as `compile_fail,E0515`, so the chapters do not use them.

---

## 8. Open questions

1. **Style guide page granularity (#8585):** Is one page per area (`data_types.md`, `errors_and_panics.md`, `crates.md`, `naming.md`, `layout_and_formatting.md`, `api_design.md`, `passing_values.md`, `traits.md`, `idioms.md`, `lints.md`) the right split, or would you prefer fewer grouped pages?
2. **Stub `style_guide.md` for old links (#8585):** All in-repo links are updated in #8585, but 16 older issues and PRs link to `documents/process/style_guide.md`. Should we keep a short file at `documents/process/style_guide.md` that points to `style_guide/README.md`?
3. **Where teaching chapters live (#8558):** Should the 7 teaching chapters (`cow_and_borrowing.md`, `zerovec_and_ule.md`, `yoke_and_zerofrom.md`, `data_provider.md`, `i18n_basics.md`, `ffi_diplomat.md`, `glossary.md`) live in `documents/contributor_guide/`, `tutorials/`, or `documents/design/`?
4. **`Use no_std` requirement level (#8587):** In `crates.md`, should `Use no_std` stay `:: suggested` or become `:: required` for library crates?
5. **`Cow<'data, str>` vs. `VarZeroCow<'data, str>`:** Which should `data_types.md` and the teaching chapters recommend for new data structs?

