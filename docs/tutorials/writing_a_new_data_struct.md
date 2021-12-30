# Writing a New Data Struct

ICU4X is a heavily data-driven library. Most new features or components will require pulling in data from an external source.

This tutorial aims to help ICU4X contributors add new data to the data pipeline. It is recommended that readers review [data_pipeline.md](../design/data_pipeline.md) for additional theory behind the design decisions in the data provider.

## Lifecycle of ICU4X Data

It is important to understand the phases of life of ICU4X data as it makes its way from the data source, like CLDR, to the data struct used at runtime. The following flowchart shows the phases and how they connect:

![Lifecycle of Data in ICU4X](../assets/data_lifecycle.svg)

The following steps take place at build time:

1. First, the source data file is obtained from an external source. Examples could include the CLDR JSON release or the Unicode Character Database.
2. Second, we use a Serde definition to deserialize the source data file. This Serde implementation does not need to be optimized for performance.
3. Third, we transform from the source data struct to the ICU4X runtime data struct. This step can be expensive, because it is normally run as an offline build step.
4. Fourth, the ICU4X runtime data struct is serialized to either JSON, if debugging is important, or to a blob store, if being prepared for use in production.

Step 1 is generally a manual step for clients, but may be automated for ICU4X testdata in tools such as `icu4x-testdata-download`. Steps 2-4 are performed as part of `icu4x-datagen`. Both of these tools are explained in more detail below.

At runtime, only one step is performed: the data struct is deserialized from the JSON or blob emitted in step 4.

When deserializing from the blob store, it is a design principle of ICU4X that no heap allocations will be required. We have many utilities and abstractions to help make this safe and easy.

## Code Layout

With a mental model of the lifecycle of data in ICU4X, we can discuss where to find the code that performs each step.

### Data Structs

The data struct definitions should live in the crate that uses them. By convention, the top-level module `provider` should contain the struct definitions. For example:

- `icu::decimal::provider::DecimalSymbolsV1`
- `icu::locale_canonicalizer::provider::LikelySubtagsV1`
- `icu::uniset::provider::UnicodePropertyV1`

In general, data structs should be annotated with `#[icu_provider::data_struct]`, and they should support *at least* `Debug`, `PartialEq`, `Clone`, `Default`, and Serde `Serialize` and `Deserialize`.

As explained in *data_pipeline.md*, the data struct should support zero-copy deserialization. The `#[icu_provider::data_struct]` annotation will enforce this for you. **See more information in [style_guide.md](https://github.com/unicode-org/icu4x/blob/main/docs/process/style_guide.md#zero-copy-in-dataprovider-structs--required),** as well as the example below in this tutorial.

If adding a new crate, you may need to add a new data category to the [`ResourceCategory` enum](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/enum.ResourceCategory.html) in `icu_provider`. This may change in the future.

### Data Download

The first step to introduce data into the ICU4X pipeline is to download it from an external source. This corresponds to step 1 above.

When clients use ICU4X, this is generally a manual step, although we may provide tooling to assist with it. For the purpose of ICU4X test data, the tool [`icu4x-testdata-download`](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/index.html) should automatically download data from the external source and save it in the ICU4X tree. `icu4x-testdata-download` should not do anything other than downloading the raw source data.

### Source Data Providers

"Source data providers" read from a source data file, deserialize it, and transform it to an ICU4X data struct. This corresponds to steps 2 and 3 above.

Although they may share common code, source data providers are implemented specific to their data source. There are therefore many source data providers in ICU4X.

Examples of source data providers include:

- [`CldrJsonDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.CldrJsonDataProvider.html#)
    - [`NumbersProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.NumbersProvider.html)
    - [`PluralsProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.PluralsProvider.html)
    - [`DateSymbolsProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.DateSymbolsProvider.html)
    - [&hellip; more examples](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/index.html)
- `BinaryPropertyUnicodeSetDataProvider`
- [`HelloWorldProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/hello_world/struct.HelloWorldProvider.html)

Source data providers must implement the following traits:

- `DataProvider<M>` for one or more data markers `M`; this impl is the main step where data transformation takes place
- `DataProvider<SerdeSeDataStructMarker>`, usually implemented with the macro [`impl_dyn_provider!`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/macro.impl_dyn_provider.html)
- [`IterableProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/iter/trait.IterableProvider.html), required for the data exporter (see below)

Source data providers are often complex to write. Rules of thumb:

- Optimize for readability and maintainability. The source data providers are not used in production, so performance is not a driving concern; however, we want the transformer to be fast enough to make a good developer experience.
- If the data source is similar to an existing data source (e.g., importing new data from CLDR JSON), try to share code with existing data providers for that source.
- If the data source is novel, feel free to add a new crate under `/provider`.

### Data Exporters and Runtime Data Providers

"Data exporters" read from one or more ICU4X data structs and dump them to storage. This corresponds to step 4 above.

Examples of data exporters include:

- [`FilesystemExporter`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/export/fs_exporter/struct.FilesystemExporter.html)
- [`BlobExporter`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/export/struct.BlobExporter.html)

"Runtime data providers" are ones that read serialized ICU4X data structs and deserialize them for use at runtime. These are the providers where performance is the key driving factor.

Examples of runtime data providers include:

- [`FsDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html)
- [`StaticDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html)

**Most ICU4X contributors will not need to touch the data exporters or runtime data providers.** New implementations are only necessary when adding a new ICU4X data struct storage mechanism.

### Data Generation Tool (`icu4x-datagen`)

The [data generation tool, i.e., `icu4x-datagen`](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/index.html), ties together the source data providers with a data exporters.

When adding new data structs, it may be necessary to make `icu4x-datagen` aware of your source data provider. This is *not* necessary for CLDR JSON providers, so long as they are properly hooked up into `CldrJsonDataProvider`.

1. Add a dependency from `icu_datagen` to the crate containing your source data provider.
2. Edit the code in `icu_datagen` to support your new source provider. You may choose to add a new command-line flag if relevant.

When finished, run from the top level:

```bash
$ cargo make testdata-build-json
$ cargo make testdata-build-blob
```

If everything is hooked together properly, JSON files for your new data struct should appear under *provider/testdata/data/json*, and the file *provider/testdata/data/testdata.postcard* should have changed.

## Example

The following example shows all the pieces that make up the data pipeline for `DecimalSymbolsV1`.

### Data Struct

[*components/decimal/src/provider.rs*](https://github.com/unicode-org/icu4x/blob/main/components/decimal/src/provider.rs)

```rust
#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DecimalSymbolsV1<'data> {
    /// Prefix and suffix to apply when a negative sign is needed.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub minus_sign_affixes: AffixesV1<'data>,

    /// Prefix and suffix to apply when a plus sign is needed.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub plus_sign_affixes: AffixesV1<'data>,

    /// Character used to separate the integer and fraction parts of the number.
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub decimal_separator: Cow<'data, str>,

    // ...
}
```

The above example is an abridged definition for `DecimalSymbolsV1`. Note how the lifetime parameter `'data` is passed down into all fields that may need to borrow data.

### CLDR JSON Deserialize

[*provider/cldr/src/transform/numbers/cldr_serde.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/cldr/src/transform/numbers/cldr_serde.rs)

```rust
pub mod numbers_json {
    //! Serde structs representing CLDR JSON numbers.json files.
    //!
    //! Sample file:
    //! https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/numbers.json

    use super::*;

    // ...

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Numbers {
        #[serde(rename = "defaultNumberingSystem")]
        pub default_numbering_system: TinyStr8,
        #[serde(rename = "minimumGroupingDigits")]
        #[serde(deserialize_with = "deserialize_number_from_string")]
        pub minimum_grouping_digits: u8,
        #[serde(flatten)]
        pub numsys_data: NumberingSystemData,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangNumbers {
        pub numbers: Numbers,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct LangData(pub LiteMap<LanguageIdentifier, LangNumbers>);

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub main: LangData,
    }
}
```

The above example is an abridged definition of the Serde structure corresponding to CLDR JSON. Since this Serde definition is not used at runtime, it does not need to be zero-copy.

### Transformer

[*provider/cldr/src/transform/numbers/mod.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/cldr/src/transform/numbers/mod.rs)

```rust
impl<'data> DataProvider<'data, DecimalSymbolsV1Marker> for NumbersProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, DecimalSymbolsV1Marker>, DataError> {
        // Load the data from CLDR JSON and emit it as an ICU4X data struct.
        // The most important line in this impl is:
        let mut result = DecimalSymbolsV1::try_from(numbers);
    }
}

icu_provider::impl_dyn_provider!(NumbersProvider, {
    _ => DecimalSymbolsV1Marker,
}, SERDE_SE, 'data);

impl<'data> IterableProvider for NumbersProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        // This should list all supported locales, for example.
    }
}

impl TryFrom<&cldr_serde::numbers_json::Numbers> for DecimalSymbolsV1<'static> {
    type Error = Cow<'static, str>;

    fn try_from(other: &cldr_serde::numbers_json::Numbers) -> Result<Self, Self::Error> {
        // This is the core transform operation. This step could take a lot of
        // work, such as pre-parsing patterns, re-organizing the data, etc.
    }
}
```

The above example is an abridged snippet of code illustrating the most important boilerplate for implementing and ICU4X data transform.

### `CldrJsonDataProvider`

New CLDR JSON transformers need to be discoverable from `CldrJsonDataProvider`. To do this, edit [*provider/cldr/src/transform/mod.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/cldr/src/transform/mod.rs) and fill in your data provider in every function, struct, and match. For example:

```rust
pub fn get_all_cldr_keys() -> Vec<ResourceKey> {
    // ...
    result.extend(&numbers::ALL_KEYS);
    // ...
}

#[derive(Debug)]
pub struct CldrJsonDataProvider<'a, 'data> {
    // ...
    numbers: LazyCldrProvider<NumbersProvider>,
    // ...
}

impl<'a> CldrJsonDataProvider<'a, '_> {
    pub fn new(cldr_paths: &'a dyn CldrPaths) -> Self {
        CldrJsonDataProvider {
            // ...
            numbers: Default::default(),
            // ...
        }
    }
}

impl<'a, 'data> DataProvider<'data, SerdeSeDataStructMarker> for CldrJsonDataProvider<'a, 'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, SerdeSeDataStructMarker>, DataError> {
        // ...
        if let Some(result) = self.numbers.try_load_serde(req, self.cldr_paths)? {
            return Ok(result);
        }
        // ...
    }
}

impl<'a> IterableProvider for CldrJsonDataProvider<'a, '_> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        // ...
        if let Some(resp) = self
            .numbers
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(Box::new(resp.into_iter()));
        }
        // ...
    }
}
```

### Data Generation Tool

[*tools/datagen/src/bin/datagen.rs*](https://github.com/unicode-org/icu4x/blob/main/tools/datagen/src/bin/datagen.rs)

```rust
fn export_cldr<'data>(
    matches: &ArgMatches,
    exporter: &mut (impl DataExporter<'data, SerdeSeDataStructMarker> + ?Sized),
    allowed_locales: Option<&[LanguageIdentifier]>,
) -> anyhow::Result<()> {
    // Create the provider after parsing options (not shown):
    let raw_provider = CldrJsonDataProvider::new(cldr_paths.as_ref());

    // Apply locale filters (not shown):
    let provider: &dyn IterableDataProvider<SerdeSeDataStructMarker>;

    // Dump data from the source provider to the data exporter:
    for key in keys.iter() {
        log::info!("Writing key: {}", key);
        icu_provider::export::export_from_iterable(key, provider, exporter)?;
    }

    Ok(())
}
```

The above example is a snippet from `icu4x-datagen` illustrating how `CldrJsonDataProvider` ties in with the data generation tool. Most contributors will not need to touch this part of the code.
