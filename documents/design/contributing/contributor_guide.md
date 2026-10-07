# ICU4X contributor guide: design proposal

This document proposes a contributor guide in `documents/contributor_guide/` and a short root `AGENTS.md`. The guide would teach the Rust patterns and design rules that ICU4X depends on, and record what to avoid and why. It is a draft for discussion. Nothing in it is policy yet.

All paths, symbols, and quotes were checked at ICU4X `main` @ `2fa9afb769` (2026-10-06). The `rust` code blocks in the drafts were compiled and run (see §8).

Each proposed file is shown in full in its own section, inside a fenced `markdown` code block.

---

## 0. Context

1. **Target:** ICU4X 2.3 on `main`. Each file says so.
2. **Related threads:** the proposal continues current maintainer threads:
   - [#8556](https://github.com/unicode-org/icu4x/issues/8556) "Documentation by-agents for-agents": human docs written with care, a top-level `AGENTS.md`, less verbosity.
   - [#7593](https://github.com/unicode-org/icu4x/issues/7593) "Instructions for New Contributors".
   - [#2675](https://github.com/unicode-org/icu4x/issues/2675) "More intro material from a Rustacean point of view".
3. **No AI policy:** [#7586](https://github.com/unicode-org/icu4x/pull/7586) (AI disclosure in the PR template) is still open. So `AGENTS.md` only states code conventions.
4. **Short beats complete:** maintainers asked for short, reviewable docs, so every file has a length budget.

Open questions are in §9.

---

## 1. Design decisions

The first outline of this guide had numbered files (`01_orientation.md` … `11_your_first_pr.md`), one chapter for all zero-copy topics, and a separate databake chapter. This section lists what stayed, what changed and why, and the main risks.

### Keep

- **Two layers.** Chapters teach. The `avoid/` rules are short and easy to link, so a reviewer can paste a rule link instead of explaining it again.
- **One file per rule**, with a permanent ID.
- **A root `AGENTS.md`.** #8556 proposes the same thing.

### Change

| # | Change | Why |
|---|---|---|
| 1 | Drop the number prefixes (`03_…`). Keep the order in `README.md`, and later in `SUMMARY.md`. | Adding a chapter would rename files and break links from issues, PRs, and `AGENTS.md`. mdBook takes the order from `SUMMARY.md`, not from file names. |
| 2 | Split zero-copy into `zerovec_and_ule.md` and `yoke_and_zerofrom.md`. | They solve different problems: byte layout plus `unsafe` ULE invariants, and payloads that borrow from their own buffer. Each needs its own exercises. |
| 3 | Merge databake and data versioning into `data_structs_and_datagen.md`. | Contributors meet databake only as one step of "add a data struct → generate → bake → keep it stable". So one chapter covers the whole lifecycle. |
| 4 | Add `errors_and_panics.md`. | `unwrap` and indexing are the most common lint failures in new code, and a typical LLM habit. The chapter connects the panic rules in the style guide with GIGO in data_safety.md. |
| 5 | Fold dependencies into `no_std_and_dependencies.md`. | Both are about what a crate may pull in. The CI checks for them (`ci-job-nostd`, `depcheck`) belong together. |
| 6 | Add `glossary.md`. | Words like *marker*, *payload*, *baked*, *blob*, *attributes*, *GIGO*, and *ULE* appear everywhere. One definition each helps non-native readers and agents. |
| 7 | Add `templates/chapter.md` and `templates/avoid_rule.md`. | New chapters and rules keep the same shape. |
| 8 | Keep `orientation.md` and `first_pr.md` thin. | CONTRIBUTING.md already covers commands, regenerating files, and review. These two chapters only map and link. |
| 9 | Rule template: add `Applies to`, `Source`, `Checked against`, and an `Automation` section, plus the status `Retired`. Write metadata as a bullet list, not YAML front matter. | `Applies to` stops rules from being applied to code they are not meant for, such as tests or `tools/`. mdBook prints YAML front matter as plain text. |
| 10 | Write code paths as `inline code`, not links. Use relative links into `documents/`, and absolute URLs for root files and issues. | Relative links into `documents/` work on GitHub, and in an mdBook whose `src` is `documents/` (with `SUMMARY.md` there, so no files move). Plain paths can't break as links, and a script can check them (§8). |

### Risks

| Risk | Evidence in the repo today | Mitigation |
|---|---|---|
| **Staleness** | The existing docs have already drifted: style_guide.md still requires `IcuResult<T>`, which no `.rs` file uses. It also lists `VarZeroVec<'data, String>`, which does not compile now (`String` is not `VarULE`, `str` is). graduation.md says the zero-copy check is in make-testdata, but [#8474](https://github.com/unicode-org/icu4x/pull/8474) moved it to `tools/make/bakeddata`. writing_a_new_data_struct.md still shows `#[icu_provider::data_struct]` and `KEYS`. documents/README.md links to a `tutorials/index.md` that does not exist. | Compile every `rust` block in CI. Check paths and symbols with a script. Put a "Checked against" line in every file. Update the guide in the same PR as the API change. Do a short check at each release (§8). |
| **Duplication** | style_guide.md (1,078 lines) already states most rules. | Link, don't copy. Each rule has a `Source` field and adds only examples and the reason. If a rule and its source disagree, the source wins and we fix the rule. |
| **Scope creep** | 13 chapters and 15 rules are a lot to review. | Roll out in phases (below), one chapter per PR. Length budgets: a chapter ≤ ~2,500 words including code and exercises (about 10 minutes of reading), a rule ≤ ~500 words, `AGENTS.md` ≤ 60 lines. A new rule needs evidence, such as a review comment or an issue. |
| **Verbosity** | #8556: maintainers want docs that people wrote with care. LLM output tends to be long. | People edit every file. Short sentences, no decorative diagrams. Reviewers can reject a PR for length alone. |
| **Policy overreach** | AI disclosure (#7586) is undecided. Some candidate rules are not written policy. | Rules start as `Proposed`. `AGENTS.md` has conventions only. Statements nobody has confirmed are marked `TODO(verify)`. |
| **No owner** | Docs without an owner decay. | Name an owner and a maintainer co-owner (§8). |

### Rollout

| PR | Content |
|---|---|
| 1 | `README.md`, templates, `avoid/README.md`, AV001, AV004, AV005, `AGENTS.md` (lists only these three rules) |
| 2 | `cow_and_borrowing.md`, AV002, AV003 (the chapter links to them), and the `tools/md-tests` hook |
| 3+ | One chapter per PR: errors_and_panics, zerovec_and_ule, yoke_and_zerofrom, data_provider, data_structs_and_datagen, glossary, then the rest |

Until a chapter exists, `README.md` lists it without a link, marked *(planned)*.

---

## 2. Final table of contents

```text
documents/contributor_guide/
├── README.md
├── glossary.md
├── orientation.md
├── i18n_basics.md
├── errors_and_panics.md
├── cow_and_borrowing.md
├── zerovec_and_ule.md
├── yoke_and_zerofrom.md
├── data_provider.md
├── data_structs_and_datagen.md
├── no_std_and_dependencies.md
├── api_design.md
├── ffi_diplomat.md
├── performance_and_size.md
├── first_pr.md
├── templates/
│   ├── chapter.md
│   └── avoid_rule.md
└── avoid/
    ├── README.md
    └── AV001_no_panics_in_library_code.md … AV015_todo_needs_issue.md
AGENTS.md                      (repository root)
documents/README.md            (existing index: add one row for the guide)
```

| File | Content | Links to existing docs |
|---|---|---|
| `README.md` | Start here: audiences, chapter map, reading paths, conventions, how to change the guide. | CONTRIBUTING.md, style_guide.md, principles.md, data_safety.md, data_versioning.md, graduation.md |
| `glossary.md` | Short definitions: locale, CLDR, marker, payload, data provider, compiled/blob/runtime data, attributes, fallback, ULE, GIGO, `Writeable`, Diplomat. | data_architecture.md, data_pipeline.md |
| `orientation.md` | A map of the repo (`components/`, `utils/`, `provider/`, `ffi/`, `tools/`), kinds of crates, and which `cargo make ci-job-*` checks what. Commands stay in CONTRIBUTING.md. | CONTRIBUTING.md, ci_build.md, rust_versions.md, boilerplate.md |
| `i18n_basics.md` | Just enough i18n to read component code: locales and BCP 47, CLDR, Unicode properties, plural rules, and why results depend on locale data. | locale_fallback_and_negotiation.md, enums_or_ids.md, `tutorials/` |
| `errors_and_panics.md` | No panics in library code. `Result` vs. GIGO with `debug_assert!`, `Writeable` vs. `TryWriteable`, and the GIGO CI job. | style_guide.md (Error Handling, Lints › Panics, Integer Overflow), data_safety.md, boilerplate.md |
| `cow_and_borrowing.md` | Borrow first, allocate only when needed: `Cow`, `Writeable`, and borrowed/owned type pairs. *Full sample in §4.* | style_guide.md (allocations, `to_string`, strings in structs, zero-copy), string_representation.md |
| `zerovec_and_ule.md` | Why data structs use `ZeroVec`, `VarZeroVec`, `ZeroMap`, and `VarZeroCow`. What ULE and VarULE are, derived vs. hand-written impls, and the safety checklist. | style_guide.md (Zero-copy in DataProvider structs), data_safety.md, principles.md (Safety), `utils/zerovec` docs and `utils/zerovec/src/ule/mod.rs` |
| `yoke_and_zerofrom.md` | How a payload borrows from a buffer that it owns (`Yokeable`, `'data`), why `#[yoke(prove_covariance_manually)]` exists, and `ZeroFrom` for cheap borrowed copies. | data_architecture.md (Zero-copy), `utils/yoke` and `utils/zerofrom` docs |
| `data_provider.md` | Markers, requests, and `DataPayload`. Compiled vs. blob vs. runtime data. The constructor family (`new`, `try_new`, `*_unstable`, `*_with_buffer_provider`). Fallback. | data_pipeline.md, data_architecture.md, locale_fallback_and_negotiation.md, `icu_provider::constructors` docs, tutorials/data-management.md, tutorials/data-provider-runtime.md |
| `data_structs_and_datagen.md` | The life of a data struct: define, generate, bake (databake), record in `fingerprints.csv`, and keep stable. It explains *why* each step exists. | writing_a_new_data_struct.md (the steps), data_versioning.md, data_safety.md, CONTRIBUTING.md (regenerating data) |
| `no_std_and_dependencies.md` | `core`/`alloc` instead of `std`, the `alloc` and `std` features, no global caches, and the dependency allowlist. | principles.md, style_guide.md (Crate Features, Crate Dependencies), boilerplate.md, `tools/noalloctest/README.md`, `tools/make/depcheck/src/allowlist.rs` |
| `api_design.md` | Constructors and argument order. Options bags (`Copy`, `options` module, `#[non_exhaustive]`). Preferences, error types, semver, and the `unstable` feature. | style_guide.md, graduation.md, changelog.md, rust_versions.md |
| `ffi_diplomat.md` | How a Rust API reaches C, C++, JavaScript/TypeScript, and Dart through Diplomat (`ffi/capi`). What Diplomat can't express. `missing_apis.txt` and the coverage allowlist. | graduation.md (FFI coverage), principles.md (Available Across Programming Languages), string_representation.md, `ffi/capi/README.md` |
| `performance_and_size.md` | Code size (dead code elimination, features, `to_string`), stack size (`size_test!` in `components/*/src/size_test_macro.rs`), data size, and benchmarks. | benchmarking.md, style_guide.md (Avoid `to_string`, Avoid `HashMap`) |
| `first_pr.md` | The final checklist: fmt, clippy, tests, regenerated files, the `## Changelog` section, CI jobs, and review etiquette. | CONTRIBUTING.md, changelog.md, ci_build.md, `.github/pull_request_template.md` |
| `templates/*.md` | Skeletons for chapters and rules (below). | — |
| `avoid/README.md`, `avoid/AVnnn_*.md` | The rule index (§5) and one file per rule (§6). | Each rule cites its own source |
| `/AGENTS.md` | Entry point for coding agents: commands and accepted rules (§7). | CONTRIBUTING.md |

### File: `documents/contributor_guide/templates/chapter.md`

````markdown
# <Title>

- **Prerequisites:** <chapters to read first, or external material>
- **Related docs:** <existing ICU4X docs, and what each one covers>
- **Checked against:** ICU4X <x.y> (`main` @ `<sha>`, <YYYY-MM-DD>)

## Why this matters in ICU4X

<The real ICU4X problem. No Rust details yet.>

## The concept (outside ICU4X)

<A minimal standalone example in a `rust` block. It must compile and run.>

## In ICU4X

<Real code. Name the file above each excerpt. Excerpts are `rust,ignore` and
match the source exactly, or say "abridged". Prefer runnable `rust` examples
that use public APIs.>

## Common mistakes

| Mistake | Do instead | Rule |
|---|---|---|

## Exercises

<2–4 tasks. Each solution is in a `<details>` block and compiles.>

## Checklist before your PR

- [ ] <item>
````

### File: `documents/contributor_guide/templates/avoid_rule.md`

````markdown
# AVnnn: <short name>

- **Status:** Proposed | Accepted | Retired
- **Enforced by:** Clippy lint | CI check | review only
- **Applies to:** <crates, files, or kinds of code>
- **Source:** <normative doc section, or the issue or PR where it was decided>
- **Checked against:** ICU4X <x.y> (`main` @ `<sha>`)

## ❌ Don't

<Minimal bad code.>

## ✅ Do instead

<Minimal good code. Point to a real example in the repo.>

## Why

<The real reason, with links.>

## Exceptions

<When breaking the rule is fine.>

## Automation

<How the rule is enforced today, or how it could be.>
````

---

## 3. README

### File: `documents/contributor_guide/README.md`

````markdown
# ICU4X contributor guide

This guide explains the Rust patterns and design rules that ICU4X depends
on. They are hard to learn from the code alone.

It is written for:

- **New contributors** who know Rust, but not ICU4X.
- **Maintainers** who want design decisions written down, with the reasons.
- **Coding agents**, through the short rules in [`avoid/`](avoid/README.md)
  and the root [`AGENTS.md`](https://github.com/unicode-org/icu4x/blob/main/AGENTS.md).

This guide teaches. It does not replace the existing documents.
These documents are the source of truth:

| Topic | Source of truth |
|---|---|
| Building, testing, regenerating files, review | [CONTRIBUTING.md](https://github.com/unicode-org/icu4x/blob/main/CONTRIBUTING.md) |
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
| [orientation.md](orientation.md) | You are new to the repository. |
| [i18n_basics.md](i18n_basics.md) | Locales, CLDR, or Unicode properties are new to you. |
| [errors_and_panics.md](errors_and_panics.md) | Before your first code PR. |
| [cow_and_borrowing.md](cow_and_borrowing.md) | Before your first code PR. |
| [zerovec_and_ule.md](zerovec_and_ule.md) | You change a data struct, or use `zerovec`. |
| [yoke_and_zerofrom.md](yoke_and_zerofrom.md) | You wonder what `'data`, `Yokeable`, or `DataPayload` are for. |
| [data_provider.md](data_provider.md) | You write a constructor, or load data. |
| [data_structs_and_datagen.md](data_structs_and_datagen.md) | You add or change a data struct. |
| [no_std_and_dependencies.md](no_std_and_dependencies.md) | You add a dependency or a Cargo feature, or want to use `std`. |
| [api_design.md](api_design.md) | You add or change a public API. |
| [ffi_diplomat.md](ffi_diplomat.md) | You add a public API. Stable APIs also need FFI. |
| [performance_and_size.md](performance_and_size.md) | You change hot code or the data layout. |
| [first_pr.md](first_pr.md) | You are about to open a PR. |

[glossary.md](glossary.md) explains ICU4X words like *marker*, *payload*,
*baked data*, and *GIGO*.

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
- If you change an API that the guide shows, update the guide in the same PR.
- Owners: TODO(verify): names to be agreed with the maintainers.

*Checked against: ICU4X 2.3 (`main` @ `2fa9afb769`, 2026-10-06).*
````

---

## 4. Sample chapter

The file name has no number (see §1, change 1).

### File: `documents/contributor_guide/cow_and_borrowing.md`

````markdown
# Cow and borrowing

- **Prerequisites:** ownership, references, and lifetimes (chapters 4 and 10
  of [the Rust book](https://doc.rust-lang.org/book/)), and
  [orientation.md](orientation.md).
- **Related docs:**
  - [style_guide.md](../process/style_guide.md) has the rules:
    [Avoid implicit allocations](../process/style_guide.md#avoid-implicit-allocations--suggested),
    [Avoid `to_string`](../process/style_guide.md#avoid-to_string--suggested),
    [Conventions for strings in structs](../process/style_guide.md#conventions-for-strings-in-structs--suggested),
    and [Zero-copy in DataProvider structs](../process/style_guide.md#zero-copy-in-dataprovider-structs--required).
    This chapter explains why they exist.
  - [string_representation.md](../design/string_representation.md) covers
    text encodings and caller-allocated output at the API boundary. This
    chapter does not repeat it.
  - [zerovec_and_ule.md](zerovec_and_ule.md) continues with data structs.
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`, 2026-10-06)

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
| A string field in a struct | See [Conventions for strings in structs](../process/style_guide.md#conventions-for-strings-in-structs--suggested): `Cow<'data, str>` in data structs |

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
See [AV005](avoid/AV005_stable_serialized_layout.md).

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
([AV003](avoid/AV003_no_std_in_library_crates.md)), so write
`alloc::borrow::Cow`, not `std::borrow::Cow`.

A crate that must also work without `alloc` can't use `Cow` at all. For
example, `components/decimal/src/lib.rs` defines its own small `Cow` enum
when the `alloc` feature is off. This is rare. If you think you need it, ask
in your issue first. [no_std_and_dependencies.md](no_std_and_dependencies.md)
has the details.

## Common mistakes

| Mistake | Do instead | Rule |
|---|---|---|
| Return `String` when the result is often the input | Return `Cow<'a, str>` | [AV002](avoid/AV002_no_needless_allocation.md) |
| Call `format!` or `to_string()` in library code | Return a type that implements `Writeable` | [AV002](avoid/AV002_no_needless_allocation.md) |
| Take `&T`, then clone it inside | Take `T` by value | [AV002](avoid/AV002_no_needless_allocation.md) |
| Call `.into_owned()` "just in case" | Keep the `Cow` until you really need a `String` | [AV002](avoid/AV002_no_needless_allocation.md) |
| Use `String`, `Vec`, or `&'data str` in a data struct | Use `Cow<'data, str>` or a `zerovec` type | [AV004](avoid/AV004_zero_copy_data_structs.md) |
| Forget `#[serde(borrow)]` on a `Cow` field | Add `#[cfg_attr(feature = "serde", serde(borrow))]` | [AV004](avoid/AV004_zero_copy_data_structs.md) |
| Use `std::borrow::Cow` in a library crate | Use `alloc::borrow::Cow` | [AV003](avoid/AV003_no_std_in_library_crates.md) |

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

## 5. Candidate avoid rules

This is the draft of `avoid/README.md`. All 15 rules start as `Proposed`. "Source" is where the rule already exists today. `TODO(verify)` marks the parts I could not find in a written source.

### File: `documents/contributor_guide/avoid/README.md`

````markdown
# Avoid rules

Short rules for ICU4X code. Each rule file has ❌/✅ examples, the reason,
the exceptions, and how the rule is enforced. IDs and file names never
change. A retired rule keeps its file and points to its replacement.

| ID | Avoid | Enforced by | Source | Status |
|---|---|---|---|---|
| [AV001](AV001_no_panics_in_library_code.md) | `unwrap`, `expect`, indexing, or `panic!` in library code | Clippy | style_guide.md: Don't Panic, Lints › Panics | Proposed |
| [AV002](AV002_no_needless_allocation.md) | Allocating when you can borrow: `String` results, `format!`/`to_string`, taking `&T` to clone it; return `Cow` or a `Writeable` | Review | style_guide.md: Avoid implicit allocations, Avoid `to_string`; pattern in `icu_normalizer`, `icu_casemap` | Proposed |
| [AV003](AV003_no_std_in_library_crates.md) | `std` in library crates; use `core`/`alloc`, and add a `std` feature only for code that needs `std` | Compiler (`no_std`), `ci-job-nostd` | principles.md: No std in core library; boilerplate.md; graduation.md | Proposed |
| [AV004](AV004_zero_copy_data_structs.md) | `String`/`Vec`/maps in data structs, or `Cow` without `#[serde(borrow)]` | CI zero-copy check | style_guide.md: Zero-copy in DataProvider structs; graduation.md | Proposed |
| [AV005](AV005_stable_serialized_layout.md) | Changing the serialized layout of a released data struct | Review (`fingerprints.csv` diff) | data_versioning.md; #8250 | Proposed |
| [AV006](AV006_ule_safety.md) | Hand-written `unsafe impl ULE`/`VarULE` when `#[zerovec::make_ule]`/`make_varule` works; hand-written impls must follow the checklist | Review | `utils/zerovec/src/ule/mod.rs` (safety checklist); principles.md: Safety. TODO(verify): "prefer derives" is not written policy | Proposed |
| [AV007](AV007_ffi_for_public_api.md) | Public APIs that Diplomat can't express, or new APIs with no FFI and no entry in `ffi/capi/tests/missing_apis.txt` | `ci-job-diplomat`, review | graduation.md (FFI coverage); principles.md: Available Across Programming Languages. TODO(verify): list of API shapes Diplomat can't express | Proposed |
| [AV008](AV008_dependency_allowlist.md) | New runtime dependencies without owner approval | `depcheck` (`ci-job-tidy`) | `tools/make/depcheck/src/allowlist.rs`; style_guide.md: Avoid heavy dependencies | Proposed |
| [AV009](AV009_gigo_for_invalid_data.md) | Data invariants that need expensive validation; handle invalid data with GIGO instead | `ci-job-test-gigo`, review | data_safety.md | Proposed |
| [AV010](AV010_exhaustiveness.md) | Exhaustive options or error types; `#[non_exhaustive]` on data structs | Clippy (workspace lints) | style_guide.md: Lints › Exhaustiveness | Proposed |
| [AV011](AV011_standard_constructors.md) | New constructor shapes; use the standard set and argument order | Review | graduation.md; `icu_provider::constructors` docs. TODO(verify): graduation.md says "Provider, Locale, Options", but 2.x constructors take preferences | Proposed |
| [AV012](AV012_no_global_caches.md) | Global caches or global mutable state | Review | principles.md: No global caches | Proposed |
| [AV013](AV013_no_hand_edited_generated_files.md) | Editing generated files by hand (baked data, testdata, FFI bindings, READMEs) | CI: `bakeddata-check`, `testdata-check`, `verify-diplomat-gen`, `generated-readme-check`, `codegen-check` | CONTRIBUTING.md (generated files) | Proposed |
| [AV014](AV014_no_behavior_changing_datagen_options.md) | Datagen options that change what a marker's data means; use a separate marker or marker attributes | Review | principles.md: Locale data from multiple sources works seamlessly; data_architecture.md: Attributes vs separate markers | Proposed |
| [AV015](AV015_todo_needs_issue.md) | `TODO`/`FIXME` without an issue number | Review (could be a tidy check) | graduation.md | Proposed |

Future candidates, not numbered yet: unchecked integer overflow (style_guide.md,
`:: required`), `std::collections::HashMap` (style_guide.md), and
`Box::leak` to get `'static` data (TODO(verify)).
````

---

## 6. Three full rules

These three show the three kinds of enforcement: a Clippy lint (AV001), a CI check (AV004), and review only (AV005).

### File: `documents/contributor_guide/avoid/AV001_no_panics_in_library_code.md`

````markdown
# AV001: No panics in library code

- **Status:** Proposed
- **Enforced by:** Clippy lint (`clippy::unwrap_used`, `clippy::expect_used`,
  `clippy::indexing_slicing`, `clippy::panic`), and review
- **Applies to:** library code in crates with the standard `lib.rs` header
  ([boilerplate.md](../../process/boilerplate.md#library-annotations)).
  The style guide asks for the header in primary ICU4X crates; it "need not
  extend to utils". Not test code.
- **Source:** [style_guide.md: Don't Panic](../../process/style_guide.md#dont-panic--required),
  [Lints › Panics](../../process/style_guide.md#panics--required),
  [data_safety.md](../../design/data_safety.md)
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`)

## ❌ Don't

```rust,ignore
let (key, value) = input.split_once('=').unwrap(); // panics if there is no '='
let first = names[0];                              // panics if `names` is empty
let (head, tail) = text.split_at(n);               // panics if `n` is not a char boundary
```

## ✅ Do instead

If the caller can do something about the problem, return an error:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    MissingEquals,
}

pub fn parse_pair(input: &str) -> Result<(&str, &str), ParseError> {
    input.split_once('=').ok_or(ParseError::MissingEquals)
}

assert_eq!(parse_pair("a=b"), Ok(("a", "b")));
assert_eq!(parse_pair("ab"), Err(ParseError::MissingEquals));
```

If the problem can only come from invalid data or a bug, use a fallback and
a debug assertion. From `components/normalizer/src/lib.rs`
(`split_normalized`):

```rust,ignore
text.split_at_checked(up_to).unwrap_or_else(|| {
    // Internal bug, not even GIGO, never supposed to happen
    debug_assert!(false);
    ("", text)
})
```

Use the method that doesn't panic (`get`, `split_at_checked`,
`checked_add`) and handle `None`. Don't add a check only so that you can call
a method that panics.

## Why

- A panic in a library stops the whole program that uses it. The style guide
  says: "never turn an error into a panic by unconditionally unwrapping the
  result in the library".
- ICU4X loads data that it did not create. data_safety.md: "Code should
  never panic at runtime based on invalid data". Code that panics on bad data
  also lets an attacker crash the application (denial of service).
- `debug_assert!` finds the bug in tests and debug builds. In release builds,
  the code continues with the fallback value ("garbage in, garbage out").

## Exceptions

[Lints › Panics](../../process/style_guide.md#panics--required) allows an
`#[allow]`, with a comment, when:

- the panic only happens if fundamental invariants of the codebase are broken;
- the API is documented to panic, and no API that is not documented to panic
  uses it;
- the condition is checked *very nearby*;
- the code is test code.

`.expect("poison")` on a `Mutex` lock is also fine, in crates that use `std`
([Exception: Poison](../../process/style_guide.md#exception-poison)).

## Automation

- **Today:** the `lib.rs` header has
  `#![cfg_attr(not(test), deny(clippy::indexing_slicing, clippy::unwrap_used, clippy::expect_used, clippy::panic))]`,
  and CI runs Clippy (`ci-job-clippy`). `ci-job-test-gigo` runs the tests with
  debug assertions off, so the code after a `debug_assert!` runs in CI too.
- **Not covered:** `split_at`, `unreachable!`, and arithmetic overflow
  ([Do not have unchecked overflow](../../process/style_guide.md#do-not-have-unchecked-overflow--required)).
  Reviewers check them.
- **Idea:** a tidy check that every library crate has the header. Generated
  crates (`provider/data/*`) don't have it. TODO(verify): which other crates
  leave it out on purpose.
- TODO(verify): whether #5974 (move lints to Cargo.toml `[lints]`) should
  include these four. Cargo `[lints]` can't express `not(test)`.
````

### File: `documents/contributor_guide/avoid/AV004_zero_copy_data_structs.md`

````markdown
# AV004: Data structs must be zero-copy

- **Status:** Proposed
- **Enforced by:** CI check (the zero-copy check in `cargo make bakeddata`),
  and review
- **Applies to:** data structs, that is, types registered with
  `icu_provider::data_struct!`
- **Source:** [style_guide.md: Zero-copy in DataProvider structs](../../process/style_guide.md#zero-copy-in-dataprovider-structs--required),
  [graduation.md](../../process/graduation.md),
  [data_architecture.md: Zero-copy](../../process/data_architecture.md#zero-copy)
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`)

## ❌ Don't

```rust,ignore
pub struct CityNames<'data> {
    pub names: Vec<String>,         // allocates on every load
    pub separator: Cow<'data, str>, // no #[serde(borrow)]: allocates too
}
```

## ✅ Do instead

```rust,ignore
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

| Instead of | Use |
|---|---|
| `String`, `&'data str` | `Cow<'data, str>` or `VarZeroCow<'data, str>` |
| `Vec<T>`, where `T` has a fixed size | `ZeroVec<'data, T>` |
| `Vec<String>` | `VarZeroVec<'data, str>` |
| `BTreeMap<K, V>`, `HashMap<K, V>` | `ZeroMap<'data, K, V>` |

Real examples: `ListFormatterPatterns` in
`components/list/src/provider/mod.rs`, and `TimeZoneEssentials` in
`components/datetime/src/provider/time_zones.rs`.

## Why

- Blob data is a byte buffer that is loaded at runtime. A zero-copy struct
  borrows from that buffer. Nothing is copied to the heap, so loading is fast
  and uses little memory.
- Compiled data uses the same struct with `'static` borrows, and datagen
  builds it with owned values. So each field must be able to borrow *and* to
  own. `&'data str` can't own, and `String` can't borrow. The style guide:
  data structs "should also support being fully owned (`'static`)".
- Without `#[serde(borrow)]`, serde deserializes a `Cow` as `Cow::Owned`.
- graduation.md requires no zero-copy violations. The CI list of allowed
  violations is empty, with the comment "Every entry in this list is a bug
  that needs to be addressed before stabilization."

## Exceptions

- Memory that is allocated and freed again during deserialization (for
  example, for validation) is a *transient* violation. It is tolerated if the
  marker is in `EXPECTED_TRANSIENT_VIOLATIONS`. Today these are `ListOrV1`,
  `ListAndV1`, and `ListUnitV1`, because their regex data must be validated.
- Ask the ICU4X team before you add a marker to either list. The CI failure
  message says the same.

## Automation

- **Today:** `ZeroCopyCheckExporter` in `tools/make/bakeddata/src/main.rs`
  serializes every payload with Postcard, deserializes it again with an
  allocator that counts, and fails on new violations. It runs in
  `cargo make bakeddata` (CI job `ci-job-full-datagen`).
- **Stale text:** the failure message still says "update EXPECTED_VIOLATIONS
  in make-testdata.rs", and graduation.md links to
  `provider/source/src/tests/make_testdata.rs`. The check moved in
  [#8474](https://github.com/unicode-org/icu4x/pull/8474). A small PR can fix
  both.
- **Idea:** the check runs only in the full datagen job. A unit test or a lint
  that flags `Cow` fields without `serde(borrow)` would catch the most common
  cause earlier. TODO(verify): whether that is worth the cost.
````

### File: `documents/contributor_guide/avoid/AV005_stable_serialized_layout.md`

````markdown
# AV005: Keep the serialized layout of stable data structs

- **Status:** Proposed
- **Enforced by:** review only. The `fingerprints.csv` diff shows reviewers
  that serialized data changed.
- **Applies to:** data structs of markers that shipped in a stable release
- **Source:** [data_versioning.md](../../process/data_versioning.md);
  example: [#8250](https://github.com/unicode-org/icu4x/pull/8250)
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`)

## ❌ Don't

Remove, reorder, or retype a field of a released data struct, and expect
regenerated data to fix it:

```diff
 pub struct TimeZoneEssentials<'data> {
     pub offset_separator: Cow<'data, str>,
     pub offset_pattern: Cow<'data, SinglePlaceholderPattern>,
-    pub offset_zero: Cow<'data, str>,
     pub offset_unknown: Cow<'data, str>,
 }
```

With only this change, data files built by older versions no longer load
correctly, and older code can't read the new data.

## ✅ Do instead

**Option A: change the Rust struct, keep the serialized layout.** This is
what #8250 did. The Rust struct lost the unused field, but hand-written serde
impls in `components/datetime/src/provider/time_zones.rs` still read and
write it:

```rust,ignore
// Deserialize: read the old field, then drop it.
let Raw {
    offset_separator,
    offset_pattern,
    offset_unknown,
    offset_zero: _offset_zero,
} = Raw::deserialize(deserializer)?;

// Serialize (datagen only): write a placeholder, so old code can read new data.
offset_zero: Cow::Borrowed(""),
```

**Option B: add a new marker.** A marker has its version in its name and
path, for example `SegmenterBreakLineV1` with the path
`segmenter/break/line/v1`. A new layout gets a new marker next to the old
one, which follows "Retain Old Keys When Possible" in data_versioning.md.
TODO(verify): link a PR that did this for a layout change.

## Why

- Postcard, the format of blob data, does not store field names. It reads
  fields in order. If a field disappears, the next field reads its bytes.
- data_versioning.md wants both directions to work: "Older code should be
  able to read newer data files", and "Stable ICU4X code should be able to
  read from data files built for any ICU4X version with the same major
  version number".
- The provider module docs say: "While the serde representation of data
  structs is guaranteed to be stable, their Rust representation might not
  be." The serialized layout is the contract. The Rust struct is not.

## Exceptions

- Markers that are not stable yet, for example markers behind the `unstable`
  feature. TODO(verify): the exact rule for experimental markers.
- A new major version. data_versioning.md allows replacing data structs then.

## Automation

- **Today:** `provider/data/*/fingerprints.csv` records the size and a hash
  of every payload. So every change to serialized data shows up in the PR
  diff. But the diff can't tell a safe change from a breaking one.
- **Idea:** a CI test that loads blob data built by the last release with the
  current code ("new code, old data"). TODO(verify): whether a test like this
  already exists.
- **Stale link:** data_versioning.md links to
  `provider/datagen/tests/data/postcard/fingerprints.csv`. The files are now
  `provider/data/*/fingerprints.csv`.
````

---

## 7. AGENTS.md

This draft lists all 15 candidates so reviewers can see them. In PR 1, the file would list only the rules that are `Accepted` and have a file (AV001, AV004, AV005 at first). The file is 40 lines.

### File: `AGENTS.md`

````markdown
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
- [AV006](documents/contributor_guide/avoid/AV006_ule_safety.md): Prefer `#[zerovec::make_ule]`/`make_varule` to hand-written `unsafe impl ULE`.
- [AV007](documents/contributor_guide/avoid/AV007_ffi_for_public_api.md): Public APIs must work through Diplomat FFI (`ffi/capi`).
- [AV008](documents/contributor_guide/avoid/AV008_dependency_allowlist.md): No new runtime dependencies without owner approval (`cargo make depcheck`).
- [AV009](documents/contributor_guide/avoid/AV009_gigo_for_invalid_data.md): Invalid data never panics. Use GIGO: a fallback plus `debug_assert!`.
- [AV010](documents/contributor_guide/avoid/AV010_exhaustiveness.md): Options and error types are `#[non_exhaustive]`. Data structs are not.
- [AV011](documents/contributor_guide/avoid/AV011_standard_constructors.md): Use the standard constructor set and argument order.
- [AV012](documents/contributor_guide/avoid/AV012_no_global_caches.md): No global caches or global mutable state.
- [AV013](documents/contributor_guide/avoid/AV013_no_hand_edited_generated_files.md): Never edit generated files by hand. Regenerate them.
- [AV014](documents/contributor_guide/avoid/AV014_no_behavior_changing_datagen_options.md): Datagen options must not change what a marker means.
- [AV015](documents/contributor_guide/avoid/AV015_todo_needs_issue.md): Every `TODO` or `FIXME` links an issue: `TODO(#1234)`.

*Checked against: ICU4X 2.3 (`main` @ `2fa9afb769`).*
````

---

## 8. Maintenance plan

| Mechanism | What it catches | Cost |
|---|---|---|
| **md-tests.** Add every guide file to `tools/md-tests/src/lib.rs` (snippet below). | `rust` examples that break after an API change, and `compile_fail` examples that start to compile. | Already in CI: `ci-job-test-docs` runs `cargo test --all-features --doc`. |
| **Path and symbol check.** A small script, run by `cargo tidy`: every inline path exists (`git ls-files`), and every symbol named above an excerpt is still in that file. | Moved or renamed files and functions in `rust,ignore` excerpts, which md-tests can't see. | About 50 lines. TODO(verify): where tidy scripts should live. |
| **Version marker.** `Checked against: ICU4X x.y (main @ sha)` in every file. | Shows readers how old a page is, and gives the owner a diff base: `git diff --stat <sha>..main -- <cited paths>`. | Manual, at each minor release. Could become a line in release.md. |
| **Same-PR rule.** Stated in README.md and first_pr.md, plus a CODEOWNERS entry for `documents/contributor_guide/`. | Changes in meaning that still compile. | One CODEOWNERS line. |
| **Owner.** One owner and one maintainer co-owner, named in README.md and CODEOWNERS. | Pages that nobody feels responsible for. | About one hour per minor release. |
| **Rule lifecycle.** Proposed → Accepted (link the approval) → Retired (the file stays and points to the replacement). IDs are never reused. | Rules that contradict each other or are outdated. | — |
| **AGENTS.md check.** In the same script: every line links an existing rule file with `Status: Accepted`, and the file has at most 60 lines. | `AGENTS.md` drifting away from the rules. | — |
| **Feedback loop.** When reviewers see the same mistake twice (from a person or an agent), add or sharpen a rule and link the PRs. When a rule becomes a lint or CI check, update `Enforced by`. | Rules nobody needs, and mistakes nobody wrote down. | — |

md-tests hook (include paths are relative to `tools/md-tests/src/lib.rs`, as the existing entries are):

```rust
mod contributor_guide {
    #[doc = include_str!("../../../documents/contributor_guide/README.md")]
    mod readme_md {}
    #[doc = include_str!("../../../documents/contributor_guide/cow_and_borrowing.md")]
    mod cow_and_borrowing_md {}
    #[doc = include_str!("../../../documents/contributor_guide/avoid/AV001_no_panics_in_library_code.md")]
    mod av001_md {}
    // One entry per chapter and per rule file.
}
```

Rustdoc treats every code block without a language tag as Rust, so non-Rust blocks must be tagged (`text`, `toml`, `diff`, `console`).

**Draft verification (done for this proposal).** I put the files above in a scratch crate that includes them the same way `tools/md-tests` does, with the same `icu` features (`compiled_data`, `serde`), and built it against `main` @ `2fa9afb769`. Result: all 10 `rust` blocks passed, the one `compile_fail` block failed to compile as expected, and the 15 `rust,ignore` excerpts were skipped. I compared each excerpt with its source file by hand. Note: stable rustdoc does not check error codes such as `compile_fail,E0515`, so the guide does not use them.

---

## 9. Open questions

1. **Scope and format:** is the split into teaching chapters and short avoid rules useful? Is the rule format in `templates/avoid_rule.md` right?
2. **Owner:** which maintainer would co-own the guide?
3. **AGENTS.md timing:** land it in PR 1, or wait until #8556 settles on a format?
4. **`Cow<'data, str>` vs. `VarZeroCow<'data, str>`:** which should the guide recommend for new data structs?
5. **Stale docs first?** Should the stale texts listed in §1 (`IcuResult`, `VarZeroVec<'data, String>`, the make-testdata references, the fingerprints path, `tutorials/index.md`) be fixed in a separate small PR before the guide?
