ICU4X Data Stability Policy
===========================

One of the value propositions driving ICU4X is the ability to share one data file across multiple ICU4X instances, something which is difficult to support in ICU4C's approach to data files. This document discusses the approach ICU4X will take toward data file versioning in order to achieve the value proposition.

## Background

ICU4C uses version-and-platform-specific data files. For example, "icudt68l.dat" can be used for ICU version 68 on little-endian platforms. However, that file is not guaranteed to work on ICU 67, ICU 69, or on big-endian platforms. (ICU has a small number of data files, such as zoneinfo64.res, which are designed for easy timezone updating and are designed to work across version numbers.)

ICU4X aims to solve this problem by making data files compatible across multiple versions.

### Use Cases

The following are reasons clients want easily shareable data files:

1. Multiple copies of ICU4X, such as those on a mobile operating system, should be able to share their data to reduce disk space and provide a more consistent user experience.
2. A locale data service (CLDR-as-a-service) should be able to serve data that is consumable by multiple versions of the ICU4X client code.

### Objectives

The following are goals that ICU4X's data versioning system should achieve:

1. Older code should be able to read newer data files.
    - For example, an app that is a few years old should be able to consume an ICU4X data file built with the latest features and CLDR version.
2. Newer code should be able to read older data files.
    - For example, an operating system service that is a few years old should be able to provide data to apps built with newer ICU4X.
3. In order to avoid breaking changes, newer component code should compile against older data provider code.
    - For example, ICU4X should be able to be updated within the same major version number without breaking custom data provider code that was written previously.
4. In order to reduce bloat, code and data that is required for compatibility purposes should have a timeline for sunset and removal.
    - For example, a rewrite of the collator may involve new code and data; there should be a timeline for removing support for the old data format.

## Policy

### I. Stable Serialized Data Schema

In order to guarantee that data files created with older ICU4X versions continue to be readable by newer versions, the serialization format must remain stable over time.

This is achieved by using the stable Postcard 1.0 format and enforcing Postcard stability by explicitly incorporating [postcard/fingerprints.csv](https://github.com/unicode-org/icu4x/blob/main/provider/testdata/data/postcard/fingerprints.csv) into code reviews.

### II. Retain Old Keys When Possible

When adding new features to an ICU4X component, there may be a desire to remove an older data struct and replace it with a newer one. However, when possible, changes should seek to add resource keys rather than replace them.

Use cases where _replacement_ may be warranted include:

1. A new data struct reduces data size or code size relative to the old data struct.
2. Data in the old data struct is obsolete and being replaced by a smaller data struct.
3. ICU4X is undergoing a major version number change.

### III. Data File Versioning

Data files can only contain the contents known at the time they are built. For example, an older data file cannot contain data that is needed for newer code. However, a newer data file can still contain data needed for older code.

The following strategy should be employed:

1. **Old Code, New Data:** Datagen can add obsolete resource keys to the data file in order to support a particular minimum version of ICU4X. Depending on the datagen configuration, it should be possible to support ***both the current and the previous major version***. For example, a data file built for ICU4X 3.7.1 may choose to support versions down to ICU4X 2.0.0. This may result in increased data size, since both old data and new data are shipped in parallel; however, shared data files are designed to reduce data size overall.
2. **New Code, Old Data:** Stable ICU4X code should be able to read from data files built for any ICU4X version ***with the same major version number***. The goal is always to provide a "best effort" result. New code has several options when consuming old data:
    1. Map the old data struct to the new data struct at runtime
    2. If (1) is infeasible, retain code required to evaluate the old struct
    3. Use the Default impl for the new data struct

In addition, new data structs should only be added in minor releases, not patch releases.

#### Data File Visualization

We can visualize this model as follows:

| Code Versions Supported | 1.0 | 1.1 | 1.2 | 2.0 | 2.1 | 2.2 |
|---|---|---|---|---|---|---|
| Data @ 1.0 | ✔️ | ☑️ | ☑️ | ❌ | ❌ | ❌ |
| Data @ 1.2 | ❌ | ❌ | ✔️ | ❌ | ❌ | ❌ |
| Data @ 1.2 w/ 1.0 compat | ✔️ | ✔️ | ✔️ | ❌ | ❌ | ❌ |
| Data @ 2.0 | ❌ | ❌ | ❌ | ✔️ | ☑️ | ☑️ |
| Data @ 2.0 w/ 1.0 compat | ✔️ | ✔️ | ✔️ | ✔️ | ☑️ | ☑️ |
| Data @ 2.2 | ❌ | ❌ | ❌ | ❌ | ❌ | ✔️ |
| Data @ 2.2 w/ 2.0 compat | ❌ | ❌ | ❌ | ✔️ | ✔️ | ✔️ |
| Data @ 2.2 w/ 1.0 compat | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ | ✔️ |

- ✔️ = supported in all constructors
- ☑️ = supported in compatibility constructors
- ❌ = not supported

#### Blob Version versus Data Struct Version

Adding older data structs to a newer data file is easy. How do we handle cases where we want to change the top-level blob layout? There are two cases here:

1. New binary format (e.g. Postcard 2.0)
2. New blob layout (e.g. change ZeroMap to ZeroHashMap)

In case 1, the new binary format should be added as a new syntax feature, parallel to the `postcard_1`, `bincode_1`, and `json` features.

In case 2, the `BlobSchema` will need to gain a new variant. When building data files with compatibility for older code versions, it will need to use a `BlobSchema` variant that is supported by all code versions that are intended to be supported.

### IV. Constructor Versioning

*Also see: [icu_provider::constructors](https://icu4x.unicode.org/doc/icu_provider/constructors/index.html)*

All ICU4X functions that take a data provider should expose three signatures:

- `*_unstable` works only with an _exactly matching_ data version.
- `*_with_any_provider` and `*_with_buffer_provider` are _compatibility constructors_ supporting the current data version and all data versions from the current major release.

Note that the compatibility constructors may require additional code in order to map from older to newer data structs, as described in the previous section.
