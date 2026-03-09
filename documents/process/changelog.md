# ICU4X changelogs

ICU4X uses a collaborative changelog process where the burden of writing changelog entries is shared among all contributors. This ensures that entries are written while the context of the change is still fresh and reduces the collation workload during the release process.

## PR Author and Reviewer Responsibilities

Every PR that introduces user-visible changes, new APIs, or significant improvements must include a changelog entry.

### The `## Changelog` Section

The PR template includes a mandatory `## Changelog` section at the end of the description. CI enforces that this section contains something, or is explicitly marked N/A.


**PR Authors** are primarily responsible for writing the changelog entry in this section. If the PR contains multiple changes that belong in different sections of the final changelog (e.g., a component change and a utility change), provide .

**Reviewers** are expected to review the changelog entry for accuracy and clarity. You may edit the PR description to improve the entry or write one on behalf of a new contributor. While the author is primarily the responsible for the changelog; reviewers should also consider it their responsibility: if fixes need to be made, just make them.

If a PR is not changelog-relevant (e.g., internal refactoring, CI changes, typo fixes in internal comments), use `## Changelog (N/A)`. You may optionally provide a brief justification for the benefit of reviewers.

**Deferring:** If you are making a partial change or expect the API to change in a follow-up PR, you can say so in the changelog entry by writing something like `TBD in future PR, see issue #xxx` (or just `Changelog: TBD`). This allows you to defer the work while maintaining accountability.

Remember that these changelog entries are collated by a human, so you can include freeform notes to help the changelog collator.

### What to Include

A good changelog entry should be concise but informative.

For API changes, you MUST list all newly added or modified public items. This includes:

* New types (structs, enums, unions).
* New methods (excluding those on new types).
* New traits.
* New trait items (excluding those on new traits).
* New trait implementations (excluding those on new types).
* New re-exports and type aliases.
* New type parameters on existing items.
* New struct fields or enum variants.
* New Cargo features.

Changes to *stable* APIs are mandatory. Changes to *unstable* APIs are optional but encouraged if they are significant.

Additions to FFI that mirror the primary API need not be mentioned in the changelog unless the API getting an FFI version was already publicly available in a prior release. FFI-specific changes should of course be mentioned in the changelog.

### Formatting


Entries should follow this format:
- One top-level line per final `CHANGELOG.md` entry. It should include a component.
- Sub-bullets listing the specific API changes, if any.

Example:
```md
icu_calendar: Add arithmetic options for `Date`
  * New types: `DateAddOptions`, `DateDifferenceError`
  * New methods: `Date::try_new_added()`, `Date::try_new_until()`
  * New associated type: `Calendar::DifferenceError`
```

If a large number of APIs are added (e.g., during a Unicode update), it is acceptable to link to the PR or another document for the full list.

## Changelog Collation

The collation process occurs during the release process and is typically driven by the release driver. It can also be done mid-cycle resulting in an `# Unreleased` section in the changelog.


### Gathering Entries

The release driver gathers all changelog entries from the `## Changelog` sections of all PRs merged since the last release. This can be done by scraping PR descriptions via the GitHub API.

`./tools/scripts/extract_changelog.py <revset>` will extract changelog entries from a list of ICU4X commits.

### Manual Collation and Merging

The gathered entries are manually collated into the draft `CHANGELOG.md`. The collator should:

* Merge related entries from multiple PRs into a single cohesive entry.
* Organize entries by category (e.g., Components, Data model and providers, FFI, Utils). Use the structure from previous changelog entries.
* Order entries by importance (API changes first, then behavior changes, then optimizations).
* Adjust wording for consistency.
* Reaching out to PR authors for further clarification.

### Version changes

Most ICU4X components are versioned with the "ICU4X version", but ICU4X utils and some crates like `icu_pattern` are not.

The changelog entry for these crates should mention a version diff, e.g. `icu_pattern: 0.4.1 -> 0.4.2`, or "No change" if there was no change made.

### Handling Deferrals

For any PRs with `TBD` in the `## Changelog` entry, the collator is responsible for:

* Checking if a followup PR has been merged with a complete changelog.
* If so, verifying that the changelog entry in the followup PR completely covers the written API.
* If not, following up with the original PR author or writing the entry themselves based on the cumulative changes.


### Final Review

Drafts of the changelog may be incrementally landed, but the TC must review the changelog before making a release. Given that TC review takes time, it is okay for the release driver to ask for review on a changelog that is missing some upcoming work as long as the TC approval is requested while mentioning details of the upcoming work (and once that work lands, the TC is updated on that having happened).

## Out-of-cycle releases

Out-of-cycle changelogs should use a single entry for each individual crate released, e.g. something like this:

```markdown
- `databake`: 0.1.5
  - Fixed [#3356](https://github.com/unicode-org/icu4x/pull/3356), adding `allow` for clippy false-positives
```

