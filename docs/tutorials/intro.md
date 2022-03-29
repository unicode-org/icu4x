# Introduction to ICU4X for Rust

`ICU4X` is an implementation of [Internationalization Components of Unicode](http://site.icu-project.org/) (ICU) intended to be modular, performant and flexible.

The library provides a layer of APIs for all software to enable internationalization capabilities.

To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected components, or add a dependency on the meta-crate `icu` which brings a reasonable selection of components in the most user-friendly configuration of features.

In this tutorial we are going to start with the meta-crate and then introduce a customization.

# 1. Requirements

For this tutorial we assume the user has basic Rust knowledge. If acquiring it is necessary, [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
We also assume that the user is familiar with a terminal and have `git`, `rust` and `cargo` installed.

To verify that, open a terminal and check that the results are similar to:

```
user@host:~/projects/icu$ git --version
git version 2.31.1
user@host:~/projects/icu$ cargo --version
cargo 1.51.0 (43b129a20 2021-03-16)
```

In this tutorial we are going to use a directory relative to the user's home directory `~/projects/icu/`. The `~` in the path indicates the relative location of the user home directory.
Please crate the directory structure necessary.

# 2. Creating MyApp

To initialize a binary application, navigate to the root directory of our project and initialize a new binary app called `myapp`:

```
cd ~/projects/icu
cargo init --bin myapp
```

The result is a new directory `~/projects/icu/myapp` with a file `./src/main.rs` which is the main file for our application.

# 3. Vendoring in ICU4X

`ICU4X`'s main meta pacakge is called `icu`, so to start using it, all one has to do it edit their `~/projects/icu/myapp/Cargo.toml`, locate the `[dependencies]` section and add:

```toml
[dependencies]
icu = "0.2"
```

After saving the changes, calling `cargo check` should vendor in `ICU4X` dependency.

# 4. Accessing components

`ICU4X` comes with a variety of components allowing to manage various facets of software internationalization.
Most of those features depend on the selection of a `Language Identifier` which is a particular combination of language, script, region with optional variants. An examples of such locales are `en-US` (American English), `sr-Cyrl` (Serbian with Cyrylic script) or `es-AR` (Argentinian Spanish).

`LanguageIdentifier` is a low level struct which is commonly used to represent user selection, available localization data and management between them.

In `ICU4X` `Locale` is a part of the `locid` component. If the user needs just this one feature, they can use `icu_locid` crate as a dependency, but since here we already added dependency on `icu`, we can refer to it via `icu::locid`.

Let's update our application to use it.

Open `~/projects/icu/myapp/src/main.rs` and edit it to:

```rust
use icu::locid::Locale;

fn main() {
    let loc: Locale = "ES-AR".parse()
        .expect("Failed to parse locale.");

    if loc.id.language == "es" {
        println!("¡Hola amigo!");
    }

    println!("You are using: {}", loc);
}
```
*Notice:* `ICU4X` canonicalized the locales's syntax which uses lowercase letter for the language portion.

After saving it, call `cargo run` in `~/projects/icu/myapp` and it should display:

```
¡Hola amigo!
You are using: es-AR
```

Congratulations! `ICU4X` has been used to semantically operate on a locale and the first string is now displayed only if the user is using a locale with Spanish `language` part!

## Convenience macro

The scenario of working with statically declared Locales and their subtags is common.
It's a bit unergonomic to have to perform the parsing of them at runtime and handle a parser error in such case.

For that purpose, ICU4X provides a macro one can use to parse it at compilation time:

```rust
use icu::locid::Locale;
use icu::locid::langid;

fn main() {
    let loc: Locale = langid!("ES-AR").into();

    if loc.id.language == "es" {
        println!("¡Hola amigo!");
    }

    println!("You are using: {}", loc);
}
```

In this case, the parsing is performed at compilation time, so we don't need to handle an error case. Try passing an malformed identifier, like "foo-bar" and try to call `cargo check`.

*Notice:* ICU4X does not expose yet a macro for `locale!` compile time parsing. Instead, `langid!` macro constructs a `LanguageIdentifier`, which we then `Into` a `Locale`.

Next, let's add some more complex functionality.

# 5. Data Management

While language identifier API is purely algorithmic, many internationalization APIs use data to perform operations. The most common data set used in Unicode Internationalization is called `CLDR` - `Common Locale Data Repository`.

Data management is a complex and non-trivial area which often requires customizations for particular environments and integrations into projects ecosystem.

The way `ICU4X` plugs into that dataset is one of its novelties aiming at making the data management more flexible and enable better integration in asynchronous environments.

In result, compared to most internationalization solutions, working with `ICU4X` and data is a bit more explicit. `ICU4X` provides a trait called `DataProvider` and a number of concrete APIs that implement that trait for different scenarios.
Users are also free to design their own providers that best fit into their ecosystem requirements.

In this tutorial we are going to use ICU4X's "test" data provider and then move on to a synchronous file-system data provider which uses ICU4X format JSON resource files.

## Test data

ICU4X's repository comes with pre-generated test data that covers all of its keys for a select set of locales. For production use it is recommended one use the steps in [Generating Data](#Generating Data) to generate a JSON directory tree or postcard blob and feed it to `FsDataProvider` or `BlobDataProvider` respectively, but for the purposes of trying stuff out, it is sufficient to use the data providers exported by `icu_testdata`.


## Using test data

First, we need to register our choice of the provider in `~/projects/icu/myapp/Cargo.toml`:

```
[dependencies]
icu = "0.5"
icu_testdata = "0.5"
```

and then we can use it in our code:

```rust
fn main() {
    let _provider = icu_testdata::get_provider();
}
```

While this app doesn't do anything on its own yet, we now have a loaded data provider, and can use it to format a date:

```rust
use icu::locid::langid;
use icu::locid::Locale;
use icu::datetime::{DateTimeFormat, mock::parse_gregorian_from_str, options::length};

fn main() {
    let loc: Locale = langid!("ja").into();

    let date = parse_gregorian_from_str("2020-10-14T13:21:00")
        .expect("Failed to parse a datetime.");

    let provider = icu_testdata::get_provider();

    let options = length::Bag {
        time: Some(length::Time::Medium),
        date: Some(length::Date::Long),
        ..Default::default()
    }.into();

    let dtf = DateTimeFormat::try_new(loc, &provider, &options)
        .expect("Failed to initialize DateTimeFormat");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```
*Notice:* Before proceeding, update your path to the ICU4X data directory.

If all went well, running the app with `cargo run` should display:

```
📅: 2020年10月14日 13:21:00
```

Here's an internationalized date!

*Notice:* Default `cargo run` builds and runs a `debug` mode of the binary. If you want to evaluate performance, memory or size of this example, use `cargo run --release`. Our example is also using `json` resource format. Generate the data in `postcard` (and use `BlobDataProvider`) for better performance.

## Using data from the filesystem

If you have ICU4X data on the file system in a JSON format, it can be loaded via `FsDataProvider`:

```toml
[dependencies]
icu = "0.5"
icu_provider_fs = {version = "0.5" , features = ["deserialize_json"]}
```

```rs
use icu_provider_fs::FsDataProvider;

fn main() {
    let _provider = FsDataProvider::try_new("/path/to/data")
       .expect("Failed to initialize Data Provider.");
}
```

The ICU4X repository has test data checked in tree in `provider/testdata/data/json`, however it is recommended one generate data on their own as described in the [next section](#generating data). Under the hood, `icu_testdata::get_provider()` is simply loading this data.

## Generating data

For production usage, it is better to generate your own data that is filtered to suit your needs.

We're going to use [JSON CLDR](https://github.com/unicode-cldr/cldr-json) as our source data. JSON CLDR is an export of [CLDR data](http://cldr.unicode.org/index/downloads) into JSON maintained by Unicode.

We are also going to use Unicode property data shipped as a zip file in the ICU4C release.

The `datagen` component has a binary application which will fetch the CLDR data and generate ICU4X data out of it.

```
git clone https://github.com/unicode-org/icu4x
cd icu4x
git checkout icu@0.5.0
cargo run --bin icu4x-datagen --features download -- \
    --cldr-tag 40.0.0 \
    --uprops-tag release-71-1 \
    --out ~/projects/icu/icu4x-data \
    --all-keys --all-locales
```

The last command is a bit dense, so let's dissect it.

* First, we call `cargo run` which runs a binary in the crate
* We tell it that the binary is named `icu4x-datagen`
* Then we use `--` to separate arguments to `cargo` from arguments to our app
* Then we pass `--cldr-tag` which informs the program which CLDR version to use
* Then we pass `--out` directory which is where we want the generated ICU4X data to be stored
* Finally, we set `--all-keys` which specify that we want to export all keys available

After that step, it should be possible to navigate to `~/projects/icu/icu4x-data` and there should be a `manifest.json` file, and directories with data.

*Notice:* In this tutorial we export data as compact `JSON` which provides decent performance and readable data files. There are other formats and options for formatting of the data available. Please consult `cargo run --bin icu4x-datagen -- --help` for details.
*Notice:* In particular, in production, the `postcard` format will yield better performance results.
*Notice:* For offline or unconventional use, the user can also pass `--cldr-root` to a local clone of the CLDR repository instead of `--cldr-tag`.


# 6. Summary

This concludes this introduction tutorial. 

With the help of `DateTimeFormat`, `Locale` and `DataProvider` we formatted a date to Japanese, but that's just a start.

The scope of internationalization domain is broad and there are many components with non-trivial interactions between them.

As the feature set of `ICU4X` grows, more and more user interface concepts will become available for internationalization, and more features for fine tuning how the operations are performed will become available.
