# MessageFormat 2 conformance fixtures

Vendored JSON from [unicode-org/message-format-wg](https://github.com/unicode-org/message-format-wg)
`test/tests/` (and schemas), pinned at:

```
dd86e42e10d1d0c9c4401d0781cdd87ee7166366
```

(Update [UPSTREAM_SHA](UPSTREAM_SHA) when refreshing.)

## Refreshing

From the ICU4X repo root, after cloning the WG repo locally:

```sh
export MESSAGE_FORMAT_WG=/path/to/message-format-wg   # optional; default: ../message-format-wg
cargo make sync-mf2-tests
```

This runs [`tools/scripts/sync-mf2-tests.sh`](../../../../tools/scripts/sync-mf2-tests.sh)
(see `tools/make/tests.toml`). It rsyncs `test/tests/` and `test/schemas/` from the
WG checkout and writes [`UPSTREAM_SHA`](UPSTREAM_SHA). It does **not** overwrite this
`README.md` (ICU4X-specific instructions stay here).

If you cannot run the script, sync manually from a WG checkout: copy `test/tests/**/*.json`
and `test/schemas/**` into this directory, then record the upstream `git rev-parse HEAD`
in `UPSTREAM_SHA`.

## Maintainer checklist (WG pin bump)

1. Run `cargo make sync-mf2-tests` (or manual copy) from a WG commit you intend to pin.
   The script copies the prior `tests/functions/number.json` and `currency.json` to
   `fixtures/.icu4x-fixture-backup/functions/` before rsync; if those files changed,
   it prints a `notice:` line — use `diff -u` against the backup to recover ICU4X-only rows.
2. **Merge** upstream JSON with ICU4X-only rows in `tests/functions/number.json` and
   `tests/functions/currency.json` (see [ICU4X-only cases](#icu4x-only-cases) below).
3. Update the **pin** line in this README and confirm [`UPSTREAM_SHA`](UPSTREAM_SHA)
   matches `git -C "$MESSAGE_FORMAT_WG" rev-parse HEAD`.
4. Run `cargo test -p icu_experimental --test messageformat_conformance --all-features`.
5. Fix regressions or add `(file_stem, case_index)` rows to [`KNOWN_FAILURES`](../conformance.rs)
   with a short comment until fixed.
6. If upstream adds `test/tests/functions/unit.json`, follow [When upstream adds `functions/unit.json`](#when-upstream-adds-functionsunitjson) and refresh
   [§2 of `messageformat-tr35-spec-tracking.md`](../../../../messageformat-tr35-spec-tracking.md)
   if the documented `:unit` test story changes.

## Running

```sh
cargo test -p icu_experimental --test messageformat_conformance --all-features
```

Always use `--all-features` so draft handlers (`:date`, `:time`, `:datetime`, `:unit`)
and the full built-in registry match what CI exercises.

## After refreshing the pin

1. Update [UPSTREAM_SHA](UPSTREAM_SHA) to the WG commit you synced from.
2. Run the command above. If any case fails, either fix the implementation or add an
   entry to `KNOWN_FAILURES` in
   [`tests/messageformat/conformance.rs`](../conformance.rs) (file stem + case
   index), with a short comment explaining the gap. Prefer fixing the code so the
   list stays empty when possible.

## ICU4X-only cases

Some entries in `tests/functions/number.json` and `tests/functions/currency.json`
are **not** copied from the WG repo: they lock ECMA-402-style `notation` /
`scientificNotation` behavior and currency + scientific output (including
`minimumFractionDigits` on scientific mantissas). When syncing from upstream,
**merge** WG changes with these extra cases (or re-apply them after a full
replace).

## When upstream adds `functions/unit.json`

The WG repo did not ship `test/tests/functions/unit.json` at the last pin, but
the conformance harness discovers **every** `*.json` under `tests/` automatically.
When a sync brings in `unit.json`:

1. Run `cargo test -p icu_experimental --test messageformat_conformance --all-features`.
2. Fix `:unit` behavior or, only if unavoidable, add `(file_stem, case_index)` rows
   to [`KNOWN_FAILURES`](../conformance.rs) with a short comment.
3. Update [§2 of `messageformat-tr35-spec-tracking.md`](../../../../messageformat-tr35-spec-tracking.md)
   if the fixture changes the documented `:unit` test story.
