// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # Introduction to ICU4X for Rust
//!
//! `ICU4X` is an implementation of
//! [Internationalization Components of Unicode](http://site.icu-project.org/) (ICU)
//! intended to be modular, performant and flexible.
//!
//! The library provides a layer of APIs for all software to enable
//! internationalization capabilities.
//!
//! To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected
//! components, or add a dependency on the meta-crate [`icu`] which brings a
//! reasonable selection of components in the most user-friendly configuration of
//! features.
//!
//! In this tutorial we are going to start with the meta-crate and then introduce a
//! customization.
//!
//! ## 1. Requirements
//!
//! For this tutorial we assume the user has basic Rust knowledge. If acquiring it
//! is necessary,
//! [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
//! We also assume that the user is familiar with a terminal and have `git`, `rust`
//! and `cargo`
//! installed.
//!
//! To verify that, open a terminal and check that the results are similar to:
//!
//! ```console
//! user@host:~/projects/icu$ git --version
//! git version 2.31.1
//! user@host:~/projects/icu$ cargo --version
//! cargo 1.51.0 (43b129a20 2021-03-16)
//! ```
//!
//! In this tutorial we are going to use a directory relative to the user's home
//! directory
//! `~/projects/icu/`. The `~` in the path indicates the relative location of the
//! user home directory.
//!
//! Please create the directory structure necessary.
//!
//! ## 2. Creating MyApp
//!
//! To initialize a binary application, navigate to the root directory of our
//! project and initialize a new binary app called `myapp`:
//!
//! ```console
//! cd ~/projects/icu
//! cargo init --bin myapp
//! ```
//!
//! The result is a new directory `~/projects/icu/myapp` with a file `./src/main.rs`
//! which is the main file for our application.
//!
//! ## 3. Vendoring in ICU4X
//!
//! `ICU4X`'s main meta package is called `icu`, so to start using it, all one has
//! to do it edit their `~/projects/icu/myapp/Cargo.toml`, locate
//! the `[dependencies]` section and add:
//!
//! ```toml
//! [dependencies]
//! icu = "0.6"
//! ```
//!
//! After saving the changes, calling `cargo check` should vendor in `ICU4X`
//! dependency.
//!
//! ## 4. Accessing components
//!
//! `ICU4X` comes with a variety of components allowing to manage various facets of
//! software internationalization.
//!
//! Most of those features depend on the selection of a [`LanguageIdentifier`] which
//! is a particular combination of language, script, region with optional variants.
//! An examples of such locales are
//! `en-US` (American English), `sr-Cyrl` (Serbian with Cyrylic script) or `es-AR` (
//! Argentinian Spanish).
//!
//! [`LanguageIdentifier`] is a low level struct which is commonly used to represent
//! user selection, available localization data and management between them.
//!
//! In `ICU4X` `Locale` is a part of the [`icu::locid`] component, since here we
//! already added dependency on `icu`, we can refer to it via `icu::locid`.
//!
//! Let's update our application to use it.
//!
//! Open `~/projects/icu/myapp/src/main.rs` and edit it to:
//!
//! ```rust
//! use icu::locid::Locale;
//!
//! fn main() {
//!     let loc: Locale = "ES-AR".parse()
//!         .expect("Failed to parse locale.");
//!
//!     if loc.id.language.as_str() == "es" {
//!         println!("¡Hola amigo!");
//!     }
//!
//!     println!("You are using: {}", loc);
//! }
//! ```
//!
//! *Notice:* `ICU4X` canonicalized the locales's syntax which uses lowercase letter
//! for the language portion.
//!
//! After saving it, call `cargo run` in `~/projects/icu/myapp` and it should
//! display:
//!
//! ```text
//! ¡Hola amigo!
//! You are using: es-AR
//! ```
//!
//! Congratulations! `ICU4X` has been used to semantically operate on a locale and
//! the first string is now displayed only if the user is using a locale with
//! Spanish `language` part!
//!
//! ### Convenience macro
//!
//! The scenario of working with statically declared Locales and their subtags is
//! common. It's a bit unergonomic to have to perform the parsing of them at runtime
//! and handle a parser error in such case.
//!
//! For that purpose, ICU4X provides a macro one can use to parse it at compilation
//! time:
//!
//! ```rust
//! use icu::locid::locale;
//!
//! let loc = locale!("ES-AR");
//!
//! if loc.id.language.as_str() == "es" {
//! println ! ("¡Hola amigo!");
//! }
//!
//! println!("You are using: {}", loc);
//! ```
//!
//! In this case, the parsing is performed at compilation time, so we don't need to
//! handle an error case. Try passing an malformed identifier, like "foo-bar" and
//! try to call `cargo check`.
//!
//! *Notice:* The compile time macros [`langid!`], and [`locale!`] don't support
//! variants or extension tags, as storing these requires allocation. If you have
//! such a tag you need to use runtime parsing.
//!
//! Next, let's add some more complex functionality.
//!
//! ## 5. Data Management
//!
//! While language identifier API is purely algorithmic, many internationalization
//! APIs use data to perform operations. The most common data set used in Unicode
//! Internationalization is called
//! `CLDR` - `Common Locale Data Repository`.
//!
//! Data management is a complex and non-trivial area which often requires
//! customizations for particular environments and integrations into projects
//! ecosystem.
//!
//! The way `ICU4X` plugs into that dataset is one of its novelties aiming at making
//! the data management more flexible and enable better integration in asynchronous
//! environments.
//!
//! In result, compared to most internationalization solutions, working with `ICU4X`
//! and data is a bit more explicit. `ICU4X` provides a trait
//! called [`DataProvider`] and a number of concrete APIs that implement that trait
//! for different scenarios. Users are also free to design their own providers that
//! best fit into their ecosystem requirements.
//!
//! In this tutorial we are going to use ICU4X's "test" data provider and then move
//! on to a synchronous file-system data provider which uses ICU4X format JSON
//! resource files.
//!
//! ### Test data
//!
//! ICU4X's repository comes with pre-generated test data that covers all of its
//! keys for a select set of locales. For production use it is recommended one use
//! the steps in
//! [Generating Data](#generating-data) to generate custom data, but for the
//! purposes of trying stuff out, it is sufficient to use the data providers
//! exported by [`icu_testdata`].
//!
//! ### Using test data
//!
//! First, we need to register our choice of the provider
//! in `~/projects/icu/myapp/Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! icu = "1.0.0"
//! icu_testdata = "1.0.0"
//! ```
//!
//! and then we can use it in our code:
//!
//! ```rust
//! let _provider = icu_testdata::unstable();
//! ```
//!
//! While this app doesn't do anything on its own yet, we now have a loaded data
//! provider, and can use it to format a date:
//!
//! ```rust
//! use icu::locid::locale;
//! use icu::calendar::DateTime;
//! use icu::datetime::{DateTimeFormatter, DateTimeFormatterOptions, options::length};
//! use icu_testdata;
//!
//! let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
//!     .expect("Failed to create a datetime.").to_any();
//!
//! let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium).into();
//!
//! let dtf = DateTimeFormatter::try_new_unstable( & icu_testdata::unstable(), & locale!("ja").into(), options)
//!     .expect("Failed to initialize DateTimeFormat");
//!
//! let formatted_date = dtf.format( & date).expect("Should return formatted date");
//! println!("📅: {}", formatted_date);
//! ```
//!
//! *Notice:* Before proceeding, update your path to the ICU4X data directory.
//!
//! If all went well, running the app with `cargo run` should display:
//!
//! ```text
//! 📅: 2020年10月14日 13:21:00
//! ```
//!
//! Here's an internationalized date!
//!
//! *Notice:* Default `cargo run` builds and runs a `debug` mode of the binary. If
//! you want to evaluate performance, memory or size of this example,
//! use `cargo run --release`. Our example is also using `json` resource format.
//! Generate the data in `postcard`
//! (and use [`BlobDataProvider`]) for better performance.
//!
//! ### Using data from the filesystem
//!
//! If you have ICU4X data on the file system in a JSON format, it can be loaded via
//! [`FsDataProvider`]:
//!
//! ```toml
//! [dependencies]
//! icu = { version = "1.0.0" }
//! icu_provider_fs = { version = "1.0.0", features = ["deserialize_json"] }
//! ```
//!
//! ```rust
//! use icu_provider_fs::FsDataProvider;
//!
//! let provider = FsDataProvider::try_new("/path/to/data/directory")
//!     .expect_err("Specify a real directory in the line above");
//!
//! ```
//!
//! The ICU4X repository has test data checked in tree in `provider/testdata/data`,
//! however it is recommended one generate data on their own as described in the
//! [next section](#generating data). Under the hood, [`icu_testdata`] is simply
//! loading this data.
//!
//! ### Generating data
//!
//! For production usage, it is better to generate your own data that is filtered to
//! suit your needs.
//!
//! We're going to use [JSON CLDR](https://github.com/unicode-cldr/cldr-json) as our
//! source data. JSON CLDR is an export
//! of [CLDR data](http://cldr.unicode.org/index/downloads) into JSON maintained by
//! Unicode.
//!
//! We are also going to use Unicode property data shipped as a zip file in the
//! ICU4C release.
//!
//! The [`icu_datagen`] component has a binary application which will fetch the CLDR data
//! and generate ICU4X data out of it.
//!
//! ```console
//! git clone https://github.com/unicode-org/icu4x
//! cd icu4x
//! git checkout icu@0.6.0
//! cargo run --bin icu4x-datagen --features bin -- \
//! --cldr-tag 41.0.0 \
//! --icuexport-tag release-71-1 \
//! --out ~/projects/icu/icu4x-data \
//! --all-keys --all-locales
//! ```
//!
//! The last command is a bit dense, so let's dissect it.
//!
//! * First, we call `cargo run` which runs a binary in the crate
//! * We tell it that the binary is named `icu4x-datagen`
//! * Then we use `--` to separate arguments to `cargo` from arguments to our app
//! * Then we pass `--cldr-tag` which informs the program which CLDR version to use
//! * Then we pass `--icuexport-tag` which informs the program which ICU-exported
//! data version to use
//! * Then we pass `--out` directory which is where we want the generated ICU4X data
//! to be stored
//! * Finally, we set `--all-keys` which specify that we want to export all keys
//! available
//!
//! After that step, it should be possible to navigate
//! to `~/projects/icu/icu4x-data` and there should be a `manifest.json` file, and
//! directories with data.
//!
//! *Notice:* In this tutorial we export data as compact `JSON` which provides
//! decent performance and readable data files. There are other formats and options
//! for formatting of the data available. Please
//! consult `cargo run --bin icu4x-datagen -- --help` for details.
//!
//! *Notice:* In particular, in production, the `postcard` format will yield better
//! performance results.
//!
//! *Notice:* For offline or unconventional use, the user can also
//! pass `--cldr-root` to a local clone of the CLDR repository instead
//! of `--cldr-tag`.
//!
//! ## 6. Summary
//!
//! This concludes this introduction tutorial.
//!
//! With the help of [`DateTimeFormatter`], [`Locale`] and [`DataProvider`] we
//! formatted a date to Japanese, but that's just a start.
//!
//! The scope of internationalization domain is broad and there are many components
//! with non-trivial interactions between them.
//!
//! As the feature set of `ICU4X` grows, more and more user interface concepts will
//! become available for internationalization, and more features for fine tuning how
//! the operations are performed will become available.
//!
//!
//! # Hooking up a data provider
//!
//! [`DataProvider`] is a general mechanism for loading data required for ICU4X
//! components to operate from a source.
//!
//! At the moment, [`DataProvider`] is only synchronous, but the model of plugging
//! it in is intended to extend to asynchronous [`DataProvider`] later.
//!
//! ## Data
//!
//! The first step is to ensure that the provider has a structures to represent the
//! data which will be collected. The structures live in a `provider` module in your
//! crate and should represent the data efficiently (rather than 1-1 match to CLDR
//! data model).
//!
//! ## Types of providers
//!
//! Any component that needs to use [`DataProvider`] should only depend
//! on `icu_provider` crate and use the [`DataProvider`] trait. The specific
//! implementations such as
//! [`icu_provider_blob::BlobDataProvider`] will be provided by the downstream
//! consumer of the component.
//!
//! ## Hooking up data provider
//!
//! Each component should use [`DataProvider`] only to construct the instance of
//! each main struct that requires data. It means that all heavy data pulling should
//! happen in the constructor, which, in result, must be fallible. Currently,
//! since [`DataProvider`] is synchronous, the constructor may be synchronous as well,
//! but in the future we expect to have both synchronous and asynchronous data
//! providers and constructors.
//!
//! ## Example
//!
//! ```rust
//! use displaydoc::Display;
//! use icu_provider::{DataPayload, DataProvider, DataRequest, DataError};
//! use icu::locid::Locale;
//! use icu::decimal::provider::{DecimalSymbolsV1Marker, DecimalSymbolsV1};
//!
//! #[derive(Display, Debug, Copy, Clone)]
//! pub enum MyError {
//!     /// Some custom error
//!     SomeError,
//!
//!     /// An error originating inside of the data provider.
//!     #[displaydoc("{0}")]
//!     DataProvider(DataError),
//! }
//!
//! #[cfg(feature = "std")]
//! impl std::error::Error for MyError {}
//!
//! impl From<DataError> for MyError {
//!     fn from(e: DataError) -> Self {
//!         MyError::DataProvider(e)
//!     }
//! }
//!
//! pub struct AdditiveIdentity(char);
//!
//! impl AdditiveIdentity {
//!     pub fn try_new<L: Into<Locale>, D: DataProvider<DecimalSymbolsV1Marker>>(
//!         locale: L,
//!         data_provider: &D,
//!     ) -> Result<Self, MyError> {
//!         let response: DataPayload<DecimalSymbolsV1Marker> = data_provider.load(DataRequest {
//!             locale: &locale.into().into(),
//!             metadata: Default::default(),
//!         })?.take_payload()?;
//!
//!         let decimal_data: &DecimalSymbolsV1 = response.get();
//!         Ok(Self(decimal_data.digits[0]))
//!     }
//! }
//!
//! ```
//!
//!
//! # Writing a New Data Struct
//!
//! ICU4X is a heavily data-driven library. Most new features or components will
//! require pulling in data from an external source.
//!
//! This tutorial aims to help ICU4X contributors add new data to the data pipeline.
//! It is recommended that readers
//! review [data_pipeline.md](../design/data_pipeline.md) for additional theory
//! behind the design decisions in the data provider.
//!
//! ## Lifecycle of ICU4X Data
//!
//! It is important to understand the phases of life of ICU4X data as it makes its
//! way from the data source, like CLDR, to the data struct used at runtime. The
//! following flowchart shows the phases and how they connect:
//!
//! ![Lifecycle of Data in ICU4X][lifecycle]
//!
//! The following steps take place at build time:
//!
//! 1. First, the source data file is obtained from an external source. Examples
//! could include the CLDR JSON release or the Unicode Character Database.
//! 2. Second, we use a Serde definition to deserialize the source data file. This
//! Serde implementation does not need to be optimized for performance.
//! 3. Third, we transform from the source data struct to the ICU4X runtime data
//! struct. This step can be expensive, because it is normally run as an offline
//! build step.
//! 4. Fourth, the ICU4X runtime data struct is serialized to either JSON, if
//! debugging is important, or to a blob store, if being prepared for use in
//! production.
//!
//! Step 1 is generally a manual step for clients, but may be automated for ICU4X
//! testdata in tools such as `icu4x-testdata-download`. Steps 2-4 are performed as
//! part of `icu4x-datagen`. Both of these tools are explained in more detail below.
//!
//! At runtime, only one step is performed: the data struct is deserialized from the
//! JSON or blob emitted in step 4.
//!
//! When deserializing from the blob store, it is a design principle of ICU4X that
//! no heap allocations will be required. We have many utilities and abstractions to
//! help make this safe and easy.
//!
//! ## Code Layout
//!
//! With a mental model of the lifecycle of data in ICU4X, we can discuss where to
//! find the code that performs each step.
//!
//! ### Data Structs
//!
//! The data struct definitions should live in the crate that uses them. By
//! convention, the top-level module `provider` should contain the struct
//! definitions. For example:
//!
//! - `icu::decimal::provider::DecimalSymbolsV1`
//! - `icu::locale_canonicalizer::provider::LikelySubtagsV1`
//! - `icu::uniset::provider::PropertyCodePointSetV1`
//!
//! In general, data structs should be annotated with `#[icu_provider::data_struct]`
//! , and they should support *at least* `Debug`, `PartialEq`, `Clone`, `Default`,
//! and Serde `Serialize` and
//! `Deserialize`.
//!
//! As explained in *data_pipeline.md*, the data struct should support zero-copy
//! deserialization. The `#[icu_provider::data_struct]` annotation will enforce this
//! for you.
//! **See more information in
//! [style_guide.md](https://github.com/unicode-org/icu4x/blob/main/docs/process/style_guide.md#zero-copy-in-dataprovider-structs--required)
//! ,** as well as the example below in this tutorial.
//!
//! ### Data Download
//!
//! The first step to introduce data into the ICU4X pipeline is to download it from
//! an external source. This corresponds to step 1 above.
//!
//! When clients use ICU4X, this is generally a manual step, although we may provide
//! tooling to assist with it. For the purpose of ICU4X test data, the tool
//! [`icu4x-testdata-download-source`](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/index.html)
//! should automatically download data from the external source and save it in the
//! ICU4X tree.
//! `icu4x-testdata-download-source` should not do anything other than downloading
//! the raw source data.
//!
//! ### Source Data Providers
//!
//! "Source data providers" read from a source data file, deserialize it, and
//! transform it to an ICU4X data struct. This corresponds to steps 2 and 3 above.
//!
//! Although they may share common code, source data providers are implemented
//! specific to their data source. There are therefore many source data providers in
//! ICU4X.
//!
//! Examples of source data providers include:
//!
//! - [`NumbersProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/transform/cldr/struct.NumbersProvider.html)
//! - [`BinaryPropertyCodePointSetDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/transform/uprops/struct.BinaryPropertyCodePointSetDataProvider.html)
//! - [&hellip; more examples](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/transform/index.html)
//!
//! Source data providers must implement the following traits:
//!
//! - `DataProvider<M>`] or `DynamicDataProvider<M>` for one or more data
//! markers `M`; this impl is the main step where data transformation takes place
//! - `IterableDataProvider<M>`, required for the data exporter (see below)
//! - `DynamicDataProvider<SerializeMarker>`
//! and `IterableDynamicDataProvider<SerializeMarker>`, usually implemented with
//! the macro
//! [`impl_dynamic_data_provider!`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/macro.impl_dynamic_data_provider.html)
//! after the above traits have been implemented
//!
//! Source data providers are often complex to write. Rules of thumb:
//!
//! - Optimize for readability and maintainability. The source data providers are
//! not used in production, so performance is not a driving concern; however, we
//! want the transformer to be fast enough to make a good developer experience.
//! - If the data source is similar to an existing data source (e.g., importing new
//! data from CLDR JSON), try to share code with existing data providers for that
//! source.
//! - If the data source is novel, feel free to add a new module
//! under [`icu_datagen::transform`].
//!
//! ### Data Exporters and Runtime Data Providers
//!
//! "Data exporters" read from one or more ICU4X data structs and dump them to
//! storage. This corresponds to step 4 above.
//!
//! Examples of data exporters include:
//!
//! - [`FilesystemExporter`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/export/fs_exporter/struct.FilesystemExporter.html)
//! - [`BlobExporter`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/export/struct.BlobExporter.html)
//!
//! "Runtime data providers" are ones that read serialized ICU4X data structs and
//! deserialize them for use at runtime. These are the providers where performance
//! is the key driving factor.
//!
//! Examples of runtime data providers include:
//!
//! - [`FsDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html)
//! - [`BlobDataProvider`](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html)
//!
//! **Most ICU4X contributors will not need to touch the data exporters or runtime
//! data providers.**
//! New implementations are only necessary when adding a new ICU4X data struct
//! storage mechanism.
//!
//! ### Data Generation Tool (`icu4x-datagen`)
//!
//! The [data generation tool, i.e., `icu4x-datagen`](https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/index.html)
//! , ties together the source data providers with a data exporter.
//!
//! When adding new data structs, it is necessary to make `icu4x-datagen` aware of
//! your source data provider. To do this, edit
//! [*
//! provider/datagen/src/registry.rs*](https://github.com/unicode-org/icu4x/blob/main/provider/datagen/src/registry.rs)
//! and add your data provider to the macro
//!
//! ```compile_fail
//! use icu_provider::foo::FooV1Marker;
//!
//! registry! {
//!     // ...
//!     FooV1Marker,
//! }
//! ```
//!
//! as well as to the list of keys
//!
//! ```rust
//! pub mod foo {
//!     use yoke::Yokeable;
//!     use serde::{Deserialize, Serialize};
//!     use icu_provider::{DataMarker, KeyedDataMarker, DataKey};
//!
//!     #[derive(
//!     Serialize, Deserialize, Clone, Default, PartialEq, Yokeable,
//!     )]
//!     struct FooV1 {
//!         message: String,
//!     }
//!
//!     struct FooV1Marker {}
//!
//!     impl DataMarker for FooV1Marker {
//!         type Yokeable = FooV1;
//!     }
//!
//!     impl KeyedDataMarker for FooV1Marker {
//!         const KEY: DataKey = icu_provider::data_key!("foo/bar@1");
//!     }
//! }
//!
//! ```
//!
//! When finished, run from the top level:
//!
//! ```console
//! cargo make testdata
//! ```
//!
//! If everything is hooked together properly, JSON files for your new data struct
//! should appear under *provider/testdata/data/json*, and the file *
//! provider/testdata/data/testdata.postcard*
//! should have changed.
//!
//! ## Example
//!
//! The following example shows all the pieces that make up the data pipeline
//! for `DecimalSymbolsV1`.
//!
//! ### Data Struct
//!
//! https://github.com/unicode-org/icu4x/blob/dbb02a18b48a63100c748e6ef3f39d5c734810f9/components/decimal/src/provider.rs#L59-L95
//!
//! The above snippet is an abridged definition for `DecimalSymbolsV1`. Note how
//! the lifetime parameter `'data` is passed down into all fields that may need to
//! borrow data.
//!
//! ### CLDR JSON Deserialize
//!
//! https://github.com/unicode-org/icu4x/blob/dbb02a18b48a63100c748e6ef3f39d5c734810f9/provider/datagen/src/transform/cldr/cldr_serde/numbers.rs#L92-L115
//!
//! The above snippet is an abridged definition of the Serde structure corresponding
//! to CLDR JSON. Since this Serde definition is not used at runtime, it does not
//! need to be zero-copy.
//!
//! ### Transformer
//!
//! There are 2 major traits that must be implemented.
//!
//! *DataProvider*
//!
//! Use the data inside self.source and emit it as an ICU4X data struct.
//! This is the core transform operation. This step could take a lot of
//! work, such as pre-parsing patterns, re-organizing the data, etc.
//! This method will be called once per option returned by supported_locales.
//!
//! https://github.com/unicode-org/icu4x/blob/dbb02a18b48a63100c748e6ef3f39d5c734810f9/provider/core/src/hello_world.rs#L105-L122
//!
//! *IterableDataProvider*
//!
//! This should list all supported locales.
//!
//! https://github.com/unicode-org/icu4x/blob/dbb02a18b48a63100c748e6ef3f39d5c734810f9/provider/core/src/hello_world.rs#L265-L277
//!
//!
//!
//! The above snippets are abridged version of code illustrating the most important
//! boilerplate for implementing and ICU4X data transform.
//!
//!
//! [`DateTimeFormatter`]: icu::datetime::DateTimeFormatter
//! [`Locale`]: icu::locid::Locale
//! [`DataProvider`]: icu_provider::DataProvider
//! [`FsDataProvider`]: icu_provider_fs::FsDataProvider
//! [`BlobDataProvider`]: icu_provider_blob::BlobDataProvider
//! [`langid!`]: icu::locid::langid
//! [`locale!`]: icu::locid::locale
//! [`LanguageIdentifier`]: icu::locid::LanguageIdentifier
//!
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("lifecycle", "../../docs/assets/data_lifecycle.svg"),
))]
#![cfg_attr(
not(feature = "doc-images"),
doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]

