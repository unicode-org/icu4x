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
- **Idea:** the check runs only in the full datagen job. A unit test or a lint
  that flags `Cow` fields without `serde(borrow)` would catch the most common
  cause earlier. TODO(verify): whether that is worth the cost.
