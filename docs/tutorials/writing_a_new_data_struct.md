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

At runtime, only one step is performed: the data struct is deserialized from the JSON or blob emitted in step 4.

When deserializing from the blob store, it is a design principle of ICU4X that no heap allocations will be required. We have many utilities and abstractions to help make this safe and easy.

## Code Layout

With a mental model of the lifecycle of data in ICU4X, we can discuss where to find the code that performs each step.

### Data Structs

The data struct definitions should live in the crate that uses them. By convention, the top-level module `provider` should contain the struct definitions. For example:

- `icu::decimal::provider::DecimalSymbolsV1`
- `icu::locale_canonicalizer::provider::LikelySubtagsV1`
- `icu::uniset::provider::UnicodePropertyV1`

If adding a new crate, you may also need to add a new data category to the [`ResourceCategory` enum](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/enum.ResourceCategory.html) in `icu_provider`. This may change in the future.

### Source Data Providers

"Source data providers" read from a source data file, deserialize it, and transform it to an ICU4X data struct. This corresponds to steps 1, 2, and 3 above.

Although they may share common code, source data providers are implemented specific to their data source. There are therefore many source data providers in ICU4X.

Examples of source data providers include:

- [`CldrJsonDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.CldrJsonDataProvider.html#)
    - [`NumbersProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.NumbersProvider.html)
    - [`PluralsProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.PluralsProvider.html)
    - [`DateSymbolsProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/struct.DateSymbolsProvider.html)
    - [&hellip; more examples](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_cldr/transform/index.html)
- `BinaryPropertiesDataProvider`
- [`HelloWorldProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/hello_world/struct.HelloWorldProvider.html)

Source data providers must implement the following traits:

- `DataProvider<M>` for one or more data markers `M`; this impl is the main step where data transformation takes place
- `DataProvider<SerdeSeDataStructMarker>`, usually implemented with the macro [`impl_dyn_provider!`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/macro.impl_dyn_provider.html)
- [`IterableDataProviderCore`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/iter/trait.IterableDataProviderCore.html), required for the data exporter (see below)

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

TODO
