# Writing a New Data Struct

ICU4X is a heavily data-driven library. Most new features or components will require pulling in data from an external source.

This tutorial aims to help ICU4X contributors add new data to the data pipeline. It is recommended that readers review [data_pipeline.md](../design/data_pipeline.md) for additional theory behind the design decisions in the data provider.

## Lifecycle of ICU4X Data

It is important to understand the phases of life of ICU4X data as it makes its way from the data source, like CLDR, to the data struct used at runtime. The following flowchart shows the phases and how they connect:

![Lifecycle of Data in ICU4X](../assets/data_lifecycle.svg)

The following steps take place at build time:

1. Source data file is obtained from an external source. Examples could include the CLDR JSON release or the Unicode Character Database.
2. The source data is parsed and transformed into a runtime data struct. This step can be expensive, because it is normally run as an offline build step.
3. The runtime data struct is stored in a way so that a provider can use it: a postcard blob, JSON directory tree, Rust module, etc.

These steps are performed by the `icu_datagen`, but clients can also write their own data generation logic.

When deserializing from the blob store, it is a design principle of ICU4X that no heap allocations will be required. We have many utilities and abstractions to help make this safe and easy.

## Code Layout

With a mental model of the lifecycle of data in ICU4X, we can discuss where to find the code that performs each step.

### Data Structs

The data struct definitions should live in the crate that uses them. By convention, the top-level module `provider` should contain the struct definitions. For example:

- `icu::decimal::provider::DecimalSymbolsV1`
- `icu::locale_canonicalizer::provider::LikelySubtagsV1`
- `icu::uniset::provider::PropertyCodePointSetV1`

In general, data structs should be annotated with `#[icu_provider::data_struct]`, and they should support *at least* `Debug`, `PartialEq`, `Clone`, `Default`, and Serde `Serialize` and `Deserialize`.

As explained in *data_pipeline.md*, the data struct should support zero-copy deserialization. The `#[icu_provider::data_struct]` annotation will enforce this for you. **See more information in [style_guide.md](https://github.com/unicode-org/icu4x/blob/main/docs/process/style_guide.md#zero-copy-in-dataprovider-structs--required),** as well as the example below in this tutorial.

Additionally, data structs should keep internal invariants to a minimum. For more information, see [data_safety.md](../design/data_safety.md).

### Data Download

The first step to introduce data into the ICU4X pipeline is to download it from an external source. This corresponds to step 1 above.

When clients use ICU4X, this is generally a manual step, although we may provide tooling to assist with it. For the purpose of ICU4X test data, the tool `download-repo-sources` should automatically download data from the external source and save it in the ICU4X tree. `download-repo-sources` should not do anything other than downloading the raw source data.

To download test data into the ICU4X source tree, run: 

```console
$ cargo make download-repo-sources
```

### Source Data Providers

"Source data providers" read from a source data file, deserialize it, and transform it to an ICU4X data struct. This corresponds to steps 2 and 3 above.

Although they may share common code, source data providers are implemented specific to their data source. There are therefore many source data providers in ICU4X.

Examples of source data providers include:

- [`NumbersProvider`](https://unicode-org.github.io/icu4x/docs/icu_datagen/transform/cldr/struct.NumbersProvider.html)
- [`BinaryPropertyCodePointSetDataProvider`](https://unicode-org.github.io/icu4x/docs/icu_datagen/transform/uprops/struct.BinaryPropertyCodePointSetDataProvider.html)
- [&hellip; more examples](https://unicode-org.github.io/icu4x/docs/icu_datagen/transform/index.html)

Source data providers must implement the following traits:

- `DataProvider<M>` or `DynamicDataProvider<M>` for one or more data markers `M`; this impl is the main step where data transformation takes place
- `IterableDataProvider<M>`, required for the data exporter (see below)
- `DynamicDataProvider<SerializeMarker>` and `IterableDynamicDataProvider<SerializeMarker>`, usually implemented with the macro [`impl_dynamic_data_provider!`](https://unicode-org.github.io/icu4x/docs/icu_provider/macro.impl_dynamic_data_provider.html) after the above traits have been implemented

Source data providers are often complex to write. Rules of thumb:

- Optimize for readability and maintainability. The source data providers are not used in production, so performance is not a driving concern; however, we want the transformer to be fast enough to make a good developer experience.
- If the data source is similar to an existing data source (e.g., importing new data from CLDR JSON), try to share code with existing data providers for that source.
- If the data source is novel, feel free to add a new module under `icu_datagen::transform`.

### Data Exporters and Runtime Data Providers

"Data exporters" read from one or more ICU4X data structs and dump them to storage. This corresponds to step 4 above.

Examples of data exporters include:

- [`FilesystemExporter`](https://unicode-org.github.io/icu4x/docs/icu_provider_fs/export/fs_exporter/struct.FilesystemExporter.html)
- [`BlobExporter`](https://unicode-org.github.io/icu4x/docs/icu_provider_blob/export/struct.BlobExporter.html)

"Runtime data providers" are ones that read serialized ICU4X data structs and deserialize them for use at runtime. These are the providers where performance is the key driving factor.

Examples of runtime data providers include:

- [`FsDataProvider`](https://unicode-org.github.io/icu4x/docs/icu_provider_fs/struct.FsDataProvider.html)
- [`BlobDataProvider`](https://unicode-org.github.io/icu4x/docs/icu_provider_blob/struct.BlobDataProvider.html)

**Most ICU4X contributors will not need to touch the data exporters or runtime data providers.** New implementations are only necessary when adding a new ICU4X data struct storage mechanism.

### Data Generation Tool (`icu4x-datagen`)

The [data generation tool, i.e., `icu4x-datagen`](https://unicode-org.github.io/icu4x/docs/icu_datagen/index.html), ties together the source data providers with a data exporter.

When adding new data structs, it is necessary to make `icu4x-datagen` aware of your source data provider. To do this, edit 
[*provider/datagen/src/registry.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/datagen/src/registry.rs) and add your data provider to the macro

```rust,compile_fail
registry!(
    // ...
    FooV1Marker,
)
```

as well as to the list of keys 

```rust

use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
#[icu_provider::data_struct(marker(FooV1Marker, "foo/bar@1"))]
pub struct FooV1<'data> {
  message: Cow<'data, str>,
}

```

When finished, run from the top level:

```bash
$ cargo make testdata
```

If everything is hooked together properly, JSON files for your new data struct should appear under *provider/datagen/tests/data/json*.

## Example

The following example shows all the pieces that make up the data pipeline for `DecimalSymbolsV1`.

### Data Struct

[*components/decimal/src/provider.rs*](https://github.com/unicode-org/icu4x/blob/main/components/decimal/src/provider.rs)

```rust
use std::borrow::Cow;
use icu_provider::prelude::*;
use icu::decimal::provider::{ AffixesV1, GroupingSizesV1 };

/// Symbols and metadata required for formatting a [`FixedDecimal`](crate::FixedDecimal).
#[icu_provider::data_struct(marker(DecimalSymbolsV1Marker, "decimal/symbols@1", extension_key = "nu" ))]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_decimal::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecimalSymbolsV1<'data> {
    /// Prefix and suffix to apply when a negative sign is needed.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub minus_sign_affixes: AffixesV1<'data>,

    /// Prefix and suffix to apply when a plus sign is needed.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub plus_sign_affixes: AffixesV1<'data>,

    /// Character used to separate the integer and fraction parts of the number.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub decimal_separator: Cow<'data, str>,

    /// Character used to separate groups in the integer part of the number.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub grouping_separator: Cow<'data, str>,

    /// Settings used to determine where to place groups in the integer part of the number.
    pub grouping_sizes: GroupingSizesV1,

    /// Digit characters for the current numbering system. In most systems, these digits are
    /// contiguous, but in some systems, such as *hanidec*, they are not contiguous.
    pub digits: [char; 10],
}
```

The above example is an abridged definition for `DecimalSymbolsV1`. Note how the lifetime parameter `'data` is passed down into all fields that may need to borrow data.

### CLDR JSON Deserialize

[*provider/datagen/src/transform/cldr/cldr_serde/numbers.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/datagen/src/transform/cldr/cldr_serde/numbers.rs)


```rust
use icu::locid::LanguageIdentifier;
use itertools::Itertools;
use serde::de::{Deserializer, Error, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use tinystr::TinyStr8;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Numbers {
    #[serde(rename = "defaultNumberingSystem")]
    pub default_numbering_system: TinyStr8,
    #[serde(rename = "minimumGroupingDigits")]
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub minimum_grouping_digits: u8,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangNumbers {
    pub numbers: Numbers,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct LangData(pub HashMap<LanguageIdentifier, LangNumbers>);

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub main: LangData,
}
```


The above example is an abridged definition of the Serde structure corresponding to CLDR JSON. Since this Serde definition is not used at runtime, it does not need to be zero-copy.

### Transformer

[*provider/core/src/data_provider.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/core/src/data_provider.rs)

[*provider/core/src/datagen/iter.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/core/src/datagen/iter.rs)

```rust,compile_fail
impl DataProvider<FooV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<FooV1Marker>, DataError> {
        // Use the data inside self.source and emit it as an ICU4X data struct.
        // This is the core transform operation. This step could take a lot of
        // work, such as pre-parsing patterns, re-organizing the data, etc.
        // This method will be called once per option returned by supported_locales.
    }
}

impl IterableDataProvider<FooV1Marker> for FooProvider {
    fn supported_locales(
        &self,
    ) -> Result<Vec<DataLocale>, DataError> {
        // This should list all supported locales.
    }
}

// Once we have DataProvider and IterableDataProvider, we can apply this macro:
icu_provider::make_exportable_provider!(FooProvider, [FooV1Marker,]);
```

The above example is an abridged snippet of code illustrating the most important boilerplate for implementing and ICU4X data transform.
