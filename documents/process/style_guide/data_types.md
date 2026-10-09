Data Types
==========

This page is part of the [ICU4X Style Guide](README.md). Rules are **required** or **suggested**; see the [Preamble](README.md#preamble).

## Zero-copy in DataProvider structs :: required

All data structs that can be passed through the DataProvider pipeline must support *zero-copy deserialization:* in practice, no heap allocations should be required when deserializing from Bincode-like formats. This means that if the type involves variable-length data like strings, vectors, and maps, it must use a zero-copy type backed by a byte buffer to represent them.

Data structs with zero-copy data should have a `'data` lifetime parameter.

In order to enable zero-copy deserialization via Serde, the `#[serde(borrow)]` annotation is most likely required. However, be aware of [known bugs](https://github.com/serde-rs/serde/issues/2016) regarding `#[serde(borrow)]` with `Option` types.

Examples of types that can be used in zero-copy data structs:

- Strings: `Cow<'data, str>`, except as noted below
- Vectors of fixed-width types: `ZeroVec<'data, T>`
    - Examples: `ZeroVec<'data, u32>`, `ZeroVec<'data, TinyStr8>`
- Vectors of variable-width types: `VarZeroVec<'data, T>`
    - Example: `VarZeroVec<'data, String>`
- Maps: `ZeroMap<'data, K, V>`
    - Example: `ZeroMap<'data, TinyStr4, String>`

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

## Conventions for strings in structs :: suggested

Main issue: [#113](https://github.com/unicode-org/icu4x/issues/113), [#571](https://github.com/unicode-org/icu4x/issues/571)

When structs with public fields contain strings, use the following type conventions:

- `Cow<'data, str>` for data provider structs (those with `'data`).
- `String` for source data structs (with no lifetime).
- `&str` if the struct does not need to own the string.
- [TinyStr](https://github.com/zbraniecki/tinystr) if the string is ASCII-only.

## Pre-validation of options :: suggested

Main issue: [#158](https://github.com/unicode-org/icu4x/issues/158)

When a variable or struct field needs to adhere to certain invariants, such as a currency code being 3 letters or significant digits being between 0 and 20, use a type that can only represent valid option values.

```rust
// BAD
#[derive(Default)]
#[non_exhaustive]
struct MyStructOptions {
    /// Must be between 0 and 20
    pub fraction_digits: usize,
}

// GOOD
#[derive(Default)]
struct FractionDigits(usize);
enum Error {
    OutOfBounds,
}
impl FractionDigits {
    fn try_new(value: usize) -> Result<Self, Error> {
        if value >= 0 && value <= 20 {
            Ok(Self(value))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}
#[derive(Default)]
#[non_exhaustive]
struct MyStructOptions {
    pub fraction_digits: FractionDigits,
}
```

## Pre-parsed fields (exotic types) :: suggested

Main issue: [#523](https://github.com/unicode-org/icu4x/issues/523)

Data in memory should be fully parsed and ready to use. For example, if a data struct contains a datetime pattern, that pattern should be represented as a `Pattern`, not as a string. We call these *exotic types*.

Keep the following in mind when using exotic types:

1. **Stability:** Since exotic types become part of the serialization format of the data struct, their serialized form must remain stable, according to the data struct versioning requirements discussed in [data_pipeline.md](../../design/data_pipeline.md).
2. **Zero-Copy:** If the exotic type involves variable-length data (like a string or a vector), it must also support zero-copy deserialization, as described above. This means that such an exotic type must have a lifetime parameter and internal `Cow`s or `ZeroVec`s for data storage.
3. **Patching:** The exotic type should support an owned (`'static`) mode to allow users to patch their own data into a data struct, as explained above.
4. **Data Integrity:** In most cases, it is insufficient to auto-derive `serde::Deserialize` on an exotic type. Deserialization must perform data validation in order to retain internal invariants of the exotic type.

If it is not possible to obey these requirements in an exotic type, use a standard type instead, but make sure that it requires minimal parsing and post-processing.

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
