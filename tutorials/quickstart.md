# Introduction to ICU4X for Rust

`ICU4X` is an implementation of [Internationalization Components of Unicode](https://icu.unicode.org/) (ICU) intended to be modular, performant and flexible.

The library provides a layer of APIs for all software to enable internationalization capabilities.

To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected components, or add a dependency on the meta-crate `icu` which brings the full selection of components in the most user-friendly configuration of features.

In this tutorial we are going to build up to writing an app that uses the `icu::datetime` component to format a date and time, covering various topics in the process.

## 1. Requirements

For this tutorial we assume the user has basic Rust knowledge. If acquiring it is necessary, the [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
We also assume that the user is familiar with a terminal and have `rust` and `cargo` installed.

To verify that, open a terminal and check that the results are similar to:

```console
cargo --version
# cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

## 2. Creating an app with ICU4X as a dependency

Use `cargo` to initialize a binary application:

```console
cargo new --bin myapp
cd myapp
```

Then add a dependency on `ICU4X`'s main crate, `icu`:

```console
cargo add icu
```

## 3. Locales

`ICU4X` comes with a variety of components allowing to manage various facets of software internationalization.

Most of those features depend on the selection of a `Locale` which is a particular combination of language, script, region with optional variants. An examples of such locales are `en-US` (American English), `sr-Cyrl` (Serbian with Cyrillic script) or `ar-EG-u-nu-latn` (Egyptian Arabic with ASCII numerals).

In `ICU4X` `Locale` is a part of the `locale_core` component. If the user needs just this one feature, they can use `icu_locale_core` crate as a dependency, but since here we already added a dependency on `icu`, we can refer to it via `icu::locale`.

Let's use this in our application.

Open `src/main.rs` and edit it to:

```rust
use icu::locale::Locale;

fn main() {
    let loc: Locale = "ES-AR".parse()
        .expect("should be a valid locale");

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

### Convenience macro

The scenario of working with statically declared `Locale`s is common.

It's a bit unergonomic to have to parse them at runtime and handle a parser error in such case.

For that purpose, ICU4X provides a macro one can use to parse it at compilation time:

```rust
use icu::locale::{Locale, locale, subtags::language};

const LOCALE: Locale = locale!("ES-AR");

fn main() {
    if LOCALE.id.language == language!("es") {
        println!("¡Hola!");
    }

    println!("You are using: {}", LOCALE);
}
```

In this case, the parsing is performed at compilation time, so we don't need to handle an error case. Try passing an malformed identifier, like "foo-bar" and call `cargo check`.

Next, let's add some more complex functionality.

## 4. Using an ICU4X component

We're going to extend our app to use the `icu::datetime` component to format a date and time. This component requires data; we will look at custom data generation later and for now use the default included data,
which is exposed through constructors such as `try_new`.

```rust
use icu::locale::{Locale, locale};
use icu::calendar::Date;
use icu::datetime::{DateTimeFormatter, fieldsets::YMD};

const LOCALE: Locale = locale!("ja"); // let's try some other language

fn main() {

    let dtf = DateTimeFormatter::try_new(
        LOCALE.into(),
        YMD::long(),
    )
    .expect("ja data should be available");

    let date = Date::try_new_iso(2020, 10, 14)
        .expect("date should be valid");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

If all went well, running the app with `cargo run` should display:

```text
📅: 2020年10月14日
```

Here's an internationalized date!

*Notice:* By default, `cargo run` builds and runs a `debug` mode of the binary. If you want to evaluate performance, memory or size of this example, use `cargo run --release`.


## 5. Data Management

While the locale API is purely algorithmic, many internationalization APIs like the date formatting API require more complex data to work. You've seen this in the previous example where we had to call `.expect("ja data should be available")` after the constructor.

Data management is a complex and non-trivial area which often requires customizations for particular environments and integrations into a project's ecosystem.

The way `ICU4X` handles data is one of its novelties, aimed at making the data management more flexible and enabling better integration in asynchronous environments.

`ICU4X` by default contains data for a a wide range of CLDR locales[^1], meaning that for most languages, the constructors can be considered infallible and you can `expect` or `unwrap` them, as we did above.

However, shipping the library with all locales will have a size impact on your binary. It also requires you to update your binary whenever CLDR data changes, which happens twice a year. To learn how to solve these problems, see our [data management](data-management.md) tutorial.

[^1]: All locales with coverage level `basic`, `moderate`, or `modern` in [`CLDR`](https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/coverageLevels.json)

## 6. Summary

This concludes this introduction tutorial. With the help of `Locale` and `DateTimeFormatter` we formatted a date to Japanese, but that's just the start. 

Internationalization is a broad domain and there are many more components in `ICU4X`.

Next, learn how to [generate optimized data for your binary](data-management.md), [configure your Cargo.toml file](../examples/cargo), or continue exploring by reading [the docs](https://docs.rs/icu/latest/).



