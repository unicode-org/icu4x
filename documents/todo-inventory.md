# TODO / FIXME inventory (Rust sources)

This file was generated to support ongoing triage of `TODO` and `FIXME` comments in `*.rs` files across the ICU4X workspace. Regenerate counts with:

```bash
grep -r -n -E 'TODO|FIXME' --include='*.rs' . | wc -l
```

## Snapshot summary (workspace root, all `*.rs`)

| Metric | Approximate count |
|--------|-------------------|
| Lines containing `TODO` or `FIXME` | 373 |
| Lines with a GitHub-style `#NNNN` reference | 164 |
| Lines without an issue reference (“orphan” comment) | 209 |
| Lines in paths whose names suggest tests/benches | 38 |
| Lines containing `FIXME` | 1 |

## Counts by top-level directory

| Prefix | Lines (approx.) |
|--------|-----------------|
| `components/` | 246 |
| `utils/` | 60 |
| `provider/` | 46 |
| `ffi/` | 17 |
| `tools/` | 4 |

## Most-referenced GitHub issues (from grep of `*.rs`)

| Issue | References |
|-------|------------|
| [#4467](https://github.com/unicode-org/icu4x/issues/4467) | 11 |
| [#5643](https://github.com/unicode-org/icu4x/issues/5643) | 11 |
| [#3550](https://github.com/unicode-org/icu4x/issues/3550) | 11 |
| [#6064](https://github.com/unicode-org/icu4x/issues/6064) | 6 |
| [#3647](https://github.com/unicode-org/icu4x/issues/3647) | 6 |
| [#3736](https://github.com/unicode-org/icu4x/issues/3736) | 6 |
| [#3957](https://github.com/unicode-org/icu4x/issues/3957) | 6 |
| [#487](https://github.com/unicode-org/icu4x/issues/487) | 4 |
| [#501](https://github.com/unicode-org/icu4x/issues/501) | 4 |
| [#1410](https://github.com/unicode-org/icu4x/issues/1410) | 4 |

## How to use this inventory

1. **Issue-linked (`TODO(#NNNN)`)** — Treat as authoritative until the issue is closed and the code is updated. Prefer keeping the reference in-tree.
2. **Orphan comments** — Either file a GitHub issue and add `#NNNN`, or replace with a normal `// Note:` if the work is explicitly out of scope.
3. **Tests marked `TODO`** — Usually low priority unless they document a correctness bug (then fix the bug or tighten the test).

## Verified issue status (plan “verify-issues” sample)

Checked via GitHub API on 2026-04-20; all were **open** and titles still matched the in-code themes:

| Issue | Title (short) | Notes |
|-------|----------------|-------|
| [#3077](https://github.com/unicode-org/icu4x/issues/3077) | Add person name formatting in ICU4X | Still open as umbrella work; initials regression test for multi-part given names is fixed in-tree by capping derived initials at two (see `derive_missing_initials`). |
| [#6064](https://github.com/unicode-org/icu4x/issues/6064) | Handle negative sub pattern in currency pattern | Still active; TODOs in currency datagen remain appropriate. |
| [#3838](https://github.com/unicode-org/icu4x/issues/3838) | Support currency patterns that vary by numbering system | Open; matches `TODO(#3838)` in currency essentials. |
| [#3958](https://github.com/unicode-org/icu4x/issues/3958) | Cleanup transliteration runtime | Open; checklist includes removing module-level clippy allows—large follow-up. |

## Implemented follow-ups from triage

User-facing notes for these changes live under **icu4x 2.2 → `icu_experimental` → `displaynames` / `personnames`** and **`icu_calendar`** in the root [`CHANGELOG.md`](../CHANGELOG.md).

- **DisplayNames `Fallback`**: `RegionDisplayNames`, `ScriptDisplayNames`, `LanguageDisplayNames`, and `VariantDisplayNames` now honor [`DisplayNamesOptions::fallback`](https://github.com/unicode-org/icu4x/blob/main/components/experimental/src/displaynames/options.rs) via `Option<Cow<'_, str>>` on `of` (see `components/experimental/src/displaynames/displaynames.rs`).
- **`Date` `Ord`**: Documented why `cmp` may disagree with calendar semantics when [`Calendar::check_date_compatibility`](https://github.com/unicode-org/icu4x/blob/main/components/calendar/src/calendar.rs) fails (`components/calendar/src/date.rs`).
- **Person names initials**: `derive_missing_initials` caps at two generated initials for multi-part given strings so output matches the two-slot `initial_sequence_pattern` case (see `derive_missing_initials.rs` tests).
- **Comment hygiene**: Fixed spelling “inconcistencies” → “inconsistencies” in `utils/calendrical_calculations/src/astronomy.rs`.
- **Transliteration #3958**: Not changed here; removing `#![expect(clippy::indexing_slicing, clippy::unwrap_used)]` from `transliterator/mod.rs` remains a larger cleanup tracked on the issue checklist.

## Regenerating detailed line lists

For a full path-annotated list (large):

```bash
grep -r -n -E 'TODO|FIXME' --include='*.rs' . | sort > todo-lines.txt
```

Do not mass-delete TODOs without closing the underlying gap; many are the only durable pointer to cross-release work.
