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
