// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # Introduction to ICU4X for Rust
//!
//! `ICU4X` is an implementation of [Internationalization Components of Unicode](http://site.icu-project.org/) (ICU) intended to be modular, performant and flexible.
//!
//! The library provides a layer of APIs for all software to enable internationalization capabilities.
//!
//! To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected components, or add a dependency on the meta-crate `icu` which brings a reasonable selection of components in the most user-friendly configuration of features.
//!
//! In this tutorial we are going to build up to writing an app that uses the `icu::datetime` component to format a date and time in a Japanese locale, covering various topics in the process.
//!
//! # 1. Requirements
//!
//! For this tutorial we assume the user has basic Rust knowledge. If acquiring it is necessary, the [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
//! We also assume that the user is familiar with a terminal and have `rust` and `cargo` installed.
//!
//! To verify that, open a terminal and check that the results are similar to:
//!
//! ```console
//! $ cargo --version
//! cargo 1.51.0 (43b129a20 2021-03-16)
//! ```
//!
//! In this tutorial we are going to use a directory relative to the user's home directory `~/projects/icu_tutorial/`.
//!
//! # 2. Creating MyApp
//!
//! To initialize a binary application, navigate to the root directory of our project and initialize a new binary app called `myapp`:
//!
//! ```console
//! cd ~/projects/icu_tutorial
//! cargo new --bin myapp
//! cd myapp
//! ```
//!
//! The result is a new directory `~/projects/icu_tutorial/myapp` with a file `./src/main.rs` which is the main file for our application. For most of this tutorial we'll be working from the `myapp` folder.
//!
//! # 3. Adding ICU4X as a dependency
//!
//! `ICU4X`'s main meta package is called `icu`, so to start using it, all one has to do it edit their `./Cargo.toml`. The easiest way to do this is to run:
//!
//! ```console
//! $ cargo add icu
//! ```
//!
//! which should generate:
//!
//! ```toml
//! [dependencies]
//! icu = "1.0"
//! ```
//!
//! After saving the changes, commands like `cargo check` will download the `icu` Rust package and use it for the build.
//!
//! # 4. Locales
//!
//! `ICU4X` comes with a variety of components allowing to manage various facets of software internationalization.
//!
//! Most of those features depend on the selection of a `Locale` which is a particular combination of language, script, region with optional variants. An examples of such locales are `en-US` (American English), `sr-Cyrl` (Serbian with Cyrillic script) or `ar-EG-u-nu-latn` (Egyptian Arabic with ASCII numerals).
//!
//! In `ICU4X` [`Locale`] is a part of the [`icu::locid`] component. If the user needs just this one feature, they can use [`icu_locid`] crate as a dependency, but since here we already added a dependency on [`icu`], we can refer to it via `icu::locid`.
//!
//! Let's update our application to use it.
//!
//! Open `./src/main.rs` and edit it to:
//!
//! ```rust
//! use icu::locid::Locale;
//!
//! # #[allow(clippy::needless_doctest_main)]
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
//! *Notice:* Here, `ICU4X` canonicalized the locales's syntax which uses lowercase letters for the language portion.
//!
//! After saving it, call `cargo run` in `~/projects/icu_tutorial/myapp` and it should display:
//!
//! ```text
//! ¡Hola!
//! You are using: es-AR
//! ```
//!
//! Congratulations! `ICU4X` has been used to semantically operate on a locale!
//!
//! ## Convenience macro
//!
//! The scenario of working with statically declared `Locale`s is common.
//!
//! It's a bit unergonomic to have to parse them at runtime and handle a parser error in such case.
//!
//! For that purpose, ICU4X provides a macro one can use to parse it at compilation time:
//!
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
//! In this case, the parsing is performed at compilation time, so we don't need to handle an error case. Try passing an malformed identifier, like "foo-bar" and call `cargo check`.
//!
//! *Notice:* `locale!` does not support variants or extension tags (e.g. `u-nu-latn`), as storing these requires allocation. If you have such a tag you need to use runtime parsing.
//!
//! Next, let's add some more complex functionality.
//!
//! # 5. Basic Data Management
//!
//! While the locale API is purely algorithmic, many internationalization APIs require more complex data to work. The most common data set used in Unicode Internationalization is called `CLDR` - `Common Locale Data Repository`.
//!
//! Data management is a complex and non-trivial area which often requires customizations for particular environments and integrations into projects ecosystem.
//!
//! The way `ICU4X` plugs into that dataset is one of its novelties, aiming at making the data management more flexible and enabling better integration in asynchronous environments.
//!
//! As a result, compared to most internationalization solutions, working with data in `ICU4X` is a bit more explicit. `ICU4X` provides a trait called [`DataProvider`] (as well as [`BufferProvider`] and [`AnyProvider`]) and a number of concrete APIs that implement these traits for different scenarios.
//! Users are also free to design their own providers that best fit into their ecosystem requirements.
//!
//! [`BufferProvider`] and [`AnyProvider`] abstract over different ways the data may be loaded: [`BufferProvider`] abstracts over data providers that deserialize data, whereas [`AnyProvider`] abstracts over data providers that can provide concrete Rust objects, like data providers that use [`databake`](https://docs.rs/databake).
//!
//! In this tutorial we are going to use ICU4X's test data provider and then move on to a synchronous file-system data provider which uses ICU4X format JSON resource files.
//!
//! ## Test data
//!
//! ICU4X's repository comes with pre-generated test data that covers all of its keys for a select set of locales. For production use it is recommended one use the steps in [Generating Data](#generating-data) to generate custom data, but for the purposes of learning how to use ICU4X, it is sufficient to use the data providers exported by [`icu_testdata`]. [`icu_testdata::any()`] will produce an [`AnyProvider`], and [`icu_testdata::buffer()`] will produce a [`BufferProvider`]. The [`icu_testdata::any()`] provider requires fewer features be enabled so we're using that.
//!
//! # 6. Using an ICU4X component
//!
//! We're going to write an app that uses the [`icu::datetime`] component to format a date and time in a Japanese locale.
//!
//! First, we need to register our choice of the provider in `./Cargo.toml`, which can be done by running:
//!
//! ```console
//! $ cargo add icu_testdata
//! ```
//!
//! which should generate:
//!
//! ```toml
//! [dependencies]
//! icu = "1.0"
//! icu_testdata = "1.0"
//! ```
//!
//! We can then use it in our code:
//!
//! ```rust
//!
//! # #[allow(clippy::needless_doctest_main)]
//! fn main() {
//!     let _provider = icu_testdata::any();
//! }
//! ```
//!
//! While this app doesn't do anything on its own yet, we now have a loaded data provider, and can use it to format a date:
//!
//! ```rust
//! use icu::locid::locale;
//! use icu::calendar::DateTime;
//! use icu::datetime::{DateTimeFormatter, options::length};
//!
//! # #[allow(clippy::needless_doctest_main)]
//! fn main() {
//!     let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);
//!
//!     let dtf = DateTimeFormatter::try_new_with_any_provider(&icu_testdata::any(), &locale!("ja").into(), options.into())
//!         .expect("Failed to initialize DateTimeFormatter");
//!
//!
//!     let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
//!         .expect("Failed to create a datetime.");
//!     // DateTimeFormatter works with data from any calendar, we need to cast to DateTime<AnyCalendar>
//!     // For smaller codesize you can use TypedDateTimeFormatter<Gregorian> with the DateTime<Gregorian>
//!     // that we have constructed
//!     let date = date.to_any();
//!
//!     let formatted_date = dtf.format(&date).expect("Formatting should succeed");
//!
//!     println!("📅: {}", formatted_date);
//! }
//! ```
//!
//! If all went well, running the app with `cargo run` should display:
//!
//! ```text
//! 📅: 2020年10月14日 13:21:00
//! ```
//!
//! Here's an internationalized date!
//!
//! *Notice:* By default, `cargo run` builds and runs a `debug` mode of the binary. If you want to evaluate performance, memory or size of this example, use `cargo run --release`.
//!
//! # 7. Advanced Data Management
//!
//! ## Using data from the filesystem
//!
//! If you have ICU4X data on the file system in a JSON format, it can be loaded via [`FsDataProvider`], which needs the [`icu_provider_fs`] crate.
//!
//! This provider performs deserialization so is a [`BufferProvider`]. This means that the feature `"serde"` needs to be enabled on [`icu`] (or the specific `icu_foo` component crate), and `"deserialize_json"` needs to be enabled on [`icu_provider`]. There are also `"deserialize_postcard_1"` and `"deserialize_bincode_1"` features available.
//!
//! The whole thing can be done by running:
//!
//!
//! ```console
//! $ cargo add icu --features serde
//! $ cargo add icu_provider_fs
//! $ cargo add icu_provider --features deserialize_json
//! ```
//!
//! You may also run `cargo remove icu_testdata` as we don't need it anymore.
//!
//! This should generate:
//!
//! ```toml
//! [dependencies]
//! icu = { version = "1.0", features = ["serde"] }
//! icu_provider_fs = { version = "1.0.0" }
//! icu_provider = { version = "1.0.0" , features = ["deserialize_json"] }
//! ```
//!
//! We can then use it like so
//!
//! ```no_run
//! use icu_provider_fs::FsDataProvider;
//!
//! # #[allow(clippy::needless_doctest_main)]
//! fn main() {
//!     let _provider = FsDataProvider::try_new("/path/to/data")
//!         .expect("Failed to initialize Data Provider.");
//! }
//! ```
//!
//! You will also need to use the `with_buffer_provider` constructors, e.g. [`icu::datetime::DateTimeFormatter::try_new_with_buffer_provider()`].
//!
//! You will need to update `/path/to/data` to a local copy of generated data.
//!
//! You can get a copy by checking out the ICU4X repository; which has test data checked in at `provider/testdata/data/` (use the `json` subfolder for this specific tutorial section). However it is recommended one generate data on their own as described in the [next section](#generating-data). Under the hood, [`icu_testdata`] is simply loading this data.
//!
//! Production users are also recommended to use [`BlobDataProvider`] from [`icu_provider_blob`], which allows a binary blob of (usually `postcard` format) data to be loaded from memory. This data provider provides the flexibility of controlling where the data is stored; allowing for data to even be loaded lazily over the network.
//!
//! Here is a full example:
//!
//! ```no_run
//! use icu::locid::locale;
//! use icu::calendar::DateTime;
//! use icu::datetime::{DateTimeFormatter, options::length};
//! use icu_provider_fs::FsDataProvider;
//!
//! # #[allow(clippy::needless_doctest_main)]
//! fn main() {
//!     let provider = FsDataProvider::try_new("/path/to/data")
//!         .expect("Failed to initialize Data Provider.");
//!
//!     let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);
//!
//!     let dtf = DateTimeFormatter::try_new_with_buffer_provider(&provider, &locale!("ja").into(), options.into())
//!         .expect("Failed to initialize DateTimeFormatter");
//!
//!
//!     let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
//!         .expect("Failed to create a datetime.");
//!     let date = date.to_any();
//!
//!
//!     let formatted_date = dtf.format(&date).expect("Formatting should succeed");
//!
//!     println!("📅: {}", formatted_date);
//! }
//! ```
//!
//!
//! ## Generating data
//!
//! For production usage, it is better to generate your own data that is filtered to suit your needs.
//!
//! We're going to use [JSON CLDR](https://github.com/unicode-cldr/cldr-json) as our source data. JSON CLDR is an export of [CLDR data](http://cldr.unicode.org/index/downloads) into JSON maintained by Unicode.
//!
//! We are also going to use Unicode property data shipped as a zip file in the ICU4C release.
//!
//! The `datagen` component has a binary application which will fetch the CLDR data and generate ICU4X data out of it.
//!
//! ```console
//! git clone https://github.com/unicode-org/icu4x
//! cd icu4x
//! git checkout icu@1.0.0
//! cargo run --bin icu4x-datagen --features bin -- \
//!   --cldr-tag latest \
//!   --icuexport-tag latest \
//!   --out ~/projects/icu_tutorial/icu4x-data \
//!   --all-keys --all-locales
//! ```
//!
//! The last command is a bit dense, so let's dissect it.
//!
//! * First, we call `cargo run` which runs a binary in the crate
//! * We tell it that the binary is named `icu4x-datagen`
//! * We use `--features bin` to be able to build datagen as a binary (it can also be used as a library)
//! * Then we use `--` to separate arguments to `cargo` from arguments to our app
//! * Then we pass `--cldr-tag` which informs the program which CLDR version to use
//! * Then we pass `--icuexport-tag` which informs the program which ICU-exported data version to use
//! * Then we pass `--out` directory which is where we want the generated ICU4X data to be stored
//! * Finally, we set `--all-keys --all-locales` which specify that we want to export all keys available, for all locales.
//!
//! After that step, it should be possible to navigate to `data`, which will contain a `manifest.json` file and a tree of JSON files.
//!
//!
//! *Notice:* In this tutorial we export data as compact `JSON` which provides decent performance and readable data files. There are other formats and options for formatting of the data available. Please consult `cargo run --bin icu4x-datagen -- --help` for details.
//! *Notice:* In particular, in production, the `postcard` format (`--format=blob --syntax=postcard`) will yield better performance results.
//! *Notice:* For offline or unconventional use, the user can also pass `--cldr-root` to a local clone of the CLDR repository instead of `--cldr-tag`.
//! *Notice:* `--all-keys --all-locales` is a lot of data, in many cases you probably only want the keys for a particular component, for a set of locales you plan to support in the application. Datagen has alternate flags like `--keys` or `--key-file` for this level of control.
//! *Notice:* This command builds in debug mode since it builds faster; but if you plan on running `icu4x-datagen` a lot (for example, if you wish to create multiple per-key postcard blobs to selectively load), we highly recommend using `--release`.
//!
//! # 6. Summary
//!
//! This concludes this introduction tutorial.
//!
//! With the help of [`DateTimeFormatter`], [`Locale`] and [`DataProvider`] we formatted a date to Japanese, but that's just a start.
//!
//! The scope of internationalization domain is broad and there are many components with non-trivial interactions between them.
//!
//! As the feature set of `ICU4X` grows, more and more user interface concepts will become available for internationalization, and more features for fine tuning how the operations are performed will become available.
//!
//! [`DateTimeFormatter`]: icu::datetime::DateTimeFormatter
//! [`Locale`]: icu::locid::Locale
//! [`BufferProvider`]: icu_provider::BufferProvider
//! [`AnyProvider`]: icu_provider::AnyProvider
//! [`DataProvider`]: icu_provider::DataProvider
//! [`FsDataProvider`]: icu_provider_fs::FsDataProvider
//! [`BlobDataProvider`]: icu_provider_blob::BlobDataProvider
//! [`langid!`]: icu::locid::langid
//! [`locale!`]: icu::locid::locale
//! [`LanguageIdentifier`]: icu::locid::LanguageIdentifier
//! [`icu_locid`]: icu::locid
