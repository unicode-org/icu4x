# Introduction to ICU4X for Rust

`ICU4X` is an implementation of [Internationalization Components of Unicode](http://site.icu-project.org/) (ICU) intended to be modular, performant and flexible.

The library provides a layer of APIs for all software to enable internationalization capabilities.

To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected components, or add a dependency on the meta-crate `icu` which brings a reasonable selection of components in the most user-friendly configuration of features.

In this tutorial we are going to build up to writing an app that uses the `icu::datetime` component to format a date and time, covering various topics in the process.

# 1. Requirements

For this tutorial we assume the user has basic Rust knowledge. If acquiring it is necessary, the [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
We also assume that the user is familiar with a terminal and have `rust` and `cargo` installed.

To verify that, open a terminal and check that the results are similar to:

```
$ cargo --version
cargo 1.51.0 (43b129a20 2021-03-16)
```

# 2. Creating MyApp

Use `cargo` to initialize a binary application:

```
cargo new --bin myapp
cd myapp
```

# 3. Adding ICU4X as a dependency

`ICU4X`'s main meta package is called `icu`, so to start using it, all one has to do it edit their `Cargo.toml`. The easiest way to do this is to run:

```sh
$ cargo add icu
```

which should generate:

```toml
[dependencies]
icu = "1.0"
```

After saving the changes, commands like `cargo check` will download the `icu` Rust package and use it for the build.

# 4. Locales

`ICU4X` comes with a variety of components allowing to manage various facets of software internationalization.

Most of those features depend on the selection of a `Locale` which is a particular combination of language, script, region with optional variants. An examples of such locales are `en-US` (American English), `sr-Cyrl` (Serbian with Cyrillic script) or `ar-EG-u-nu-latn` (Egyptian Arabic with ASCII numerals).

In `ICU4X` `Locale` is a part of the `locid` component. If the user needs just this one feature, they can use `icu_locid` crate as a dependency, but since here we already added a dependency on `icu`, we can refer to it via `icu::locid`.

Let's update our application to use it.

Open `src/main.rs` and edit it to:

```rust
use icu::locid::Locale;

fn main() {
    let loc: Locale = "ES-AR".parse()
        .expect("Failed to parse locale.");

    if loc.id.language.as_str() == "es" {
        println!("¡Hola!");
    }

    println!("You are using: {}", loc);
}
```

After saving it, call `cargo run` and it should display:

```
¡Hola!
You are using: es-AR
```

*Notice:* Here, `ICU4X` canonicalized the locales's syntax which uses lowercase letters for the language portion.

Congratulations! `ICU4X` has been used to semantically operate on a locale!

## Convenience macro

The scenario of working with statically declared `Locale`s is common.

It's a bit unergonomic to have to parse them at runtime and handle a parser error in such case.

For that purpose, ICU4X provides a macro one can use to parse it at compilation time:

```rust
use icu::locid::{Locale, locale};

const LOCALE: Locale = locale!("ES-AR");

fn main() {
    if LOCALE.id.language.as_str() == "es" {
        println!("¡Hola amigo!");
    }

    println!("You are using: {}", loc);
}
```

In this case, the parsing is performed at compilation time, so we don't need to handle an error case. Try passing an malformed identifier, like "foo-bar" and call `cargo check`.

*Notice:* `locale!` does not support variants or extension tags (e.g. `u-nu-latn`), as storing these requires allocation. If you have such a tag you need to use runtime parsing.

Next, let's add some more complex functionality.

# 5. Basic Data Management

While the locale API is purely algorithmic, many internationalization APIs require more complex data to work. The most common data set used in Unicode Internationalization is called `CLDR` - `Common Locale Data Repository`.

Data management is a complex and non-trivial area which often requires customizations for particular environments and integrations into projects ecosystem.

The way `ICU4X` plugs into that dataset is one of its novelties, aiming at making the data management more flexible and enabling better integration in asynchronous environments.

As a result, compared to most internationalization solutions, working with data in `ICU4X` is a bit more explicit. `ICU4X` provides a trait called `DataProvider` (as well as `BufferProvider` and `AnyProvider`) and a number of concrete APIs that implement these traits for different scenarios.
Users are also free to design their own providers that best fit into their ecosystem requirements.

`BufferProvider` and `AnyProvider` abstract over different ways the data may be loaded: `BufferProvider` abstracts over data providers that deserialize data, whereas `AnyProvider` abstracts over data providers that can provide concrete Rust objects, like data providers that use [`databake`](https://docs.rs/databake).

In this tutorial we are going to use ICU4X's test data provider and then move on to a generate our own data.

# 6. Using an ICU4X component

We're going to extend our app to use the `icu::datetime` component to format a date and time.

First, we need to register our choice of the provider, `icu_testdata`, in `Cargo.toml`, which can be done by running:

```sh
$ cargo add icu_testdata
```

which should generate:

```
[dependencies]
icu = "1.0"
icu_testdata = "1.0"
```

*Notice:* `icu_testdata` contains testing providers that support all ICU4X keys for a small representative set of locales. It contains both a `BufferProvider` (`icu_testdata::buffer()`), and an `AnyProvider` (`icu_testdata::any()`). The latter requires fewer features, so we will be using that.

We can then use it in our code:

```rust
use icu::locid::{Locale, locale};

const LOCALE: Locale = locale!("ES-AR");

fn main() {
    let _provider = icu_testdata::any();

    // ...
}
```

We now have a loaded data provider, and can use it to format a date:

```rust
use icu::locid::{Locale, locale};
use icu::calendar::DateTime;
use icu::datetime::{DateTimeFormatter, options::length};

const LOCALE: Locale = locale!("ja"); // let's try some other language

fn main() {
    let provider = icu_testdata::any();

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = DateTimeFormatter::try_new_with_any_provider(&provider, &locale!("ja").into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");

    // DateTimeFormatter works with data from any calendar, we need to cast to DateTime<AnyCalendar>
    // For smaller codesize you can use TypedDateTimeFormatter<Gregorian> with the DateTime<Gregorian>
    // that we have constructed
    let date = date.to_any();

    let formatted_date = dtf.format(&date).expect("Formatting should succeed");

    println!("📅: {}", formatted_date);
}
```

If all went well, running the app with `cargo run` should display:

```
📅: 2020年10月14日 13:21:00
```

Here's an internationalized date!

*Notice:* By default, `cargo run` builds and runs a `debug` mode of the binary. If you want to evaluate performance, memory or size of this example, use `cargo run --release`.

# 7. Advanced Data Management

## Generating data

Unless you're happy shipping your app with the ~10 locales supported by `icu_testdata, you will need generate your own data that is filtered to suit your needs.

We're going to use the `icu_datagen` crate to do this. It pulls in [CLDR data](http://cldr.unicode.org/index/downloads) and Unicode property data from ICU4C releases to generate the data.

The `icu_datagen` crate has a command line interface as well as a Rust API, which can be used in `build.rs` files. Here we will use the CLI.

```
cargo run -p icu_datagen --features bin -- \
    --cldr-tag latest \
    --icuexport-tag latest \
    --out my-data \
    --format blob \
    --all-keys --all-locales
```

The last command is a bit dense, so let's dissect it.

* First, we call `cargo run` which runs a binary
* We tell it that the crate is named `icu_datagen`
* We use `--features bin` to be able to build the binary (otherwise the crate will only contain the library)
* Then we use `--` to separate arguments to `cargo` from arguments to `icu_datagen`
* `--cldr-tag` informs the program which CLDR version to use
* `--icuexport-tag` informs the program which ICU-exported data version to use
* `--out` is the location where we want the generated ICU4X data to be stored
* `--format blob` to set the format of the output (we'll discuss formats later)
* and finally `--all-keys --all-locales` to specify that we want to export all keys available, for all locales.

This should generate a `my-data` file containing your data in serialized form.

*Notice:* `--cldr-tag` and `--icuexport-tag` fetch the specified version from the network. For offline or unconventional use, the user can also pass `--cldr-root`/`--icuexport-root` to use local versions.
*Notice:* `--all-keys --all-locales` is a lot of data, in many cases you probably only want the keys for a particular component, for a set of locales you plan to support in the application. Datagen has alternate flags like `--keys` or `--key-file` for this level of control.
*Notice:* This command builds in debug mode since it builds faster; but if you plan on running `icu_datagen` a lot (for example, if you wish to create multiple per-key blobs to selectively load), we highly recommend using `--release`.

## Using the generated data

Once you have generated your data, it needs to be loaded as a data provider. The blob format we chose can be loaded by `BlobDataProvider` from the `icu_provider_blob` crate.

This provider performs deserialization, so it's a `BufferProvider`. This means that the feature `"serde"` needs to be enabled on `icu` (or the specific `icu_foo` component crate).

The whole thing can be done by running:

```sh
$ cargo add icu --features serde
$ cargo add icu_provider_blob
```

You may also run `cargo remove icu_testdata` as we don't need it anymore.

This should generate:

```toml
[dependencies]
icu = { version = "1.0", features = ["serde"] }
icu_provider_blob = { version = "1.0.0" }
icu_provider = { version = "1.0.0" }
```

We can then use it like so

```rs
use icu_provider_blob::BlobDataProvider;

fn main() {
    let _provider = BlobDataProvider::try_new_from_owned_blob(
            std::fs::read("my-data")
                .expect("Failed to read file")
                .into_boxed_slice()
    )
    .expect("Failed to initialize Data Provider.");
}
```

You will also need to use the `with_buffer_provider` constructors, e.g. `DateTimeFormatter::try_new_with_buffer_provider()`.

Here is a full example:

```rust
use icu::locid::locale;
use icu::calendar::DateTime;
use icu::datetime::{DateTimeFormatter, options::length};
use icu_provider_blob::BlobDataProvider;

fn main() {
    let buffer_provider = BlobDataProvider::try_new_from_owned_blob(
            std::fs::read("my-data")
                .expect("Failed to read file")
                .into_boxed_slice()
    )
    .expect("Failed to initialize Data Provider.");

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = DateTimeFormatter::try_new_with_buffer_provider(&buffer_provider, &locale!("ja").into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");
    let date = date.to_any();

    let formatted_date = dtf.format(&date).expect("Formatting should succeed");

    println!("📅: {}", formatted_date);
}
```

## Other formats

### `dir` and `FsDataProvider`

The different `format`s are:
* `blob`: a serialized binary blob. This format is useful for production use when data loading should happen at runtime (such as when downloading language packs)
* `mod`: a Rust module containing the data.
* `dir`: a directory of JSON files. This is mostly useful for debugging.

The `dir` format will generate a directory tree of data files in JSON (although the `--syntax` option can be used to generate `postcard` or `bincode` files, which doesn't have many practical uses). This directory can be read by the `FsDataProvider` from the `icu_provider_fs` crate. You will also need to activate the feature for the chosen syntax on the `icu_provider` crate.

This is also a buffer provider, so you will need to use the `with_buffer_provider` constructors.

```sh
cargo add icu_provider --features deserialize_json
cargo add icu_provider_fs
```

```rust
let _provider = icu_provider_fs::FsDataProvider::try_new("my-data").expect("Failed to initialize Data Provider.");
```

*Notice:* JSON is human readable, which can come in handy. Take a look at the [testdata in JSON form](https://github.com/unicode-org/icu4x/tree/icu%401.0.0/provider/testdata/data/json)

### `mod` and `BakedDataProvider`

The `mod` format will generate a Rust module that defines a data provider. This format naturally has no deserialization overhead, but cannot be dynamically loaded at runtime.

Using this might require some additional dependencies, the `icu_datagen` tool will let you know what to install. You can use it by directly including the source using the `include!` macro.

```rust
include!("my-data/mod.rs");
let _provider = BakedDataProvider;
```

With this provider, you can use the `unstable` constructors. These are only guaranteed to work if the `BakedDataProvider` was generated with the same version of ICU4X that you are building with, but if you build the data as part of your a build pipeline, that shouldn't be a problem.

You can also make the `BakedDataProvider` implement the `AnyProvider` trait, so that it can be used with `_with_any_provider` constructors. Using these constructors is slightly less performant than the `unstable` ones, but, as the name suggests, stable across releases.

```rust
include!("my-data/mod.rs");
include!("my-data/any.rs");
let _any_provider = BakedDataProvider;
```

*Notice:* If you're curious what these Rust files look like, take a look at the [testdata](https://github.com/unicode-org/icu4x/tree/icu%401.0.0/provider/testdata/data/baked)

# 6. Summary

This concludes this introduction tutorial.

With the help of `DateTimeFormat`, `Locale` and `DataProvider` we formatted a date to Japanese, but that's just a start.

The scope of internationalization domain is broad and there are many components with non-trivial interactions between them.

As the feature set of `ICU4X` grows, more and more user interface concepts will become available for internationalization, and more features for fine tuning how the operations are performed will become available.
