# Introduction to ICU4X for Rust

`ICU4X` is an implementation of [Internationalization Components of Unicode](http://site.icu-project.org/) (ICU) intended to be modular, performant and flexible.

The library provides a layer of APIs for all software to enable internationalization capabilities.

To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected components, or add a dependency on the meta-crate `icu` which brings the full selection of components in the most user-friendly configuration of features.

In this tutorial we are going to build up to writing an app that uses the `icu::datetime` component to format a date and time, covering various topics in the process.

# 1. Requirements

For this tutorial we assume the user has basic Rust knowledge. If acquiring it is necessary, the [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
We also assume that the user is familiar with a terminal and have `rust` and `cargo` installed.

To verify that, open a terminal and check that the results are similar to:

```console
$ cargo --version
cargo 1.64.0 (387270bc7 2022-09-16)
```

# 2. Creating an app with ICU4X as a dependency

Use `cargo` to initialize a binary application:

```console
cargo new --bin myapp
cd myapp
```

Then add a dependency on `ICU4X`'s main crate, `icu`:

```console
$ cargo add icu
```

# 3. Locales

`ICU4X` comes with a variety of components allowing to manage various facets of software internationalization.

Most of those features depend on the selection of a `Locale` which is a particular combination of language, script, region with optional variants. An examples of such locales are `en-US` (American English), `sr-Cyrl` (Serbian with Cyrillic script) or `ar-EG-u-nu-latn` (Egyptian Arabic with ASCII numerals).

In `ICU4X` `Locale` is a part of the `locid` component. If the user needs just this one feature, they can use `icu_locid` crate as a dependency, but since here we already added a dependency on `icu`, we can refer to it via `icu::locid`.

Let's use this in our application.

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

```text
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

    println!("You are using: {}", LOCALE);
}
```

In this case, the parsing is performed at compilation time, so we don't need to handle an error case. Try passing an malformed identifier, like "foo-bar" and call `cargo check`.

Next, let's add some more complex functionality.

# 4. Basic Data Management

While the locale API is purely algorithmic, many internationalization APIs require more complex data to work. Data management is a complex and non-trivial area which often requires customizations for particular environments and integrations into a project's ecosystem.

The way `ICU4X` handles data is one of its novelties, aimed at making the data management more flexible and enabling better integration in asynchronous environments.

As a result, compared to most internationalization solutions, working with data in `ICU4X` is a bit more explicit. `ICU4X` provides a trait called `DataProvider` (as well as `BufferProvider` and `AnyProvider`) and a number of concrete APIs that implement these traits for different scenarios.
Users are also free to design their own providers that best fit into their ecosystem requirements.

`BufferProvider` and `AnyProvider` abstract over different ways the data may be loaded: `BufferProvider` abstracts over data providers that deserialize data, whereas `AnyProvider` abstracts over data providers that can provide concrete Rust objects.

# 5. Using an ICU4X component

We're going to extend our app to use the `icu::datetime` component to format a date and time. This component requires data, but as we don't want to jump into data management just yet, we will use `ICU4X`'s `icu_testdata` crate. This contains test providers that support all ICU4X keys for a small representative set of locales. It contains a `BufferProvider` (`icu_testdata::buffer()`), an `AnyProvider` (`icu_testdata::any()`), and an unstable data provider (`icu_testdata::unstable()`). The latter two require fewer Cargo features, so we will be using those.

First, we need to add the crate to our `Cargo.toml`:

```console
$ cargo add icu_testdata
```

We can then use it in our code to format a date:

```rust
use icu::locid::{Locale, locale};
use icu::calendar::DateTime;
use icu::datetime::{DateTimeFormatter, options::length};

const LOCALE: Locale = locale!("ja"); // let's try some other language

fn main() {
    let provider = icu_testdata::any();

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = DateTimeFormatter::try_new_with_any_provider(&provider, &LOCALE.into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_iso_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");

    // DateTimeFormatter supports the ISO and native calendars as input via DateTime<AnyCalendar>.
    // For smaller codesize you can use TypedDateTimeFormatter<Gregorian> with a DateTime<Gregorian>
    let date = date.to_any();

    let formatted_date = dtf.format(&date).expect("Formatting should succeed");

    println!("📅: {}", formatted_date);
}
```

If all went well, running the app with `cargo run` should display:

```text
📅: 2020年10月14日 13:21:28
```

Here's an internationalized date!

*Notice:* By default, `cargo run` builds and runs a `debug` mode of the binary. If you want to evaluate performance, memory or size of this example, use `cargo run --release`.

# 6. Summary

This concludes this introduction tutorial. With the help of `DateTimeFormat`, `Locale` and `DataProvider` we formatted a date to Japanese, but that's just the start. 
Internationalization is a broad domain and there are many more components in `ICU4X`.

Next, learn how to [generate optimized data for your binary](data_management.md) or [configure your Cargo.toml file](cargo.md), or continue exploring by reading [the docs](https://docs.rs/icu/latest/).
