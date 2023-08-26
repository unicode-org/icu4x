# Data management in ICU4X

This tutorial introduces data providers as well as the `icu_datagen` tool.

If you're happy shipping your app with the recommended set of locales included by the `compiled_data` default feature, you can stop reading now. If you want to reduce code size, do runtime data loading, or build your own complex data pipelines, this tutorial is for you.

# 1. Prerequisites

This tutorial assumes you have finished the [introductory tutorial](intro.md) and continues where that tutorial left off. In particular, you should still have the latest version of code for `myapp`.

# 2. Generating data

Data generation is done using the `icu_datagen` crate, which pulls in data from [Unicode's *Common Locale Data Repository* (*CLDR*)](http://cldr.unicode.org/index/downloads) and from `ICU4C` releases to generate `ICU4X` data. The crate has a command line interface as well as a Rust API, which can be used for example in `build.rs` files. Here we will use the CLI.

First we will need to install the binary:

```console
$ cargo install icu_datagen
```

Get a coffee, this might take a while ☕.

Once installed, run:

```console
$ icu4x-datagen --keys all --locales full --format blob --out my_data_blob.postcard
```

This will generate a `my_data_blob.postcard` file containing the serialized data for all components in all locales. The file is several megabytes large; we will optimize it later in the tutorial!

`icu4x-datagen` has many options, some of which we'll discover below. The default options should work for most purposes, but check out `icu4x-datagen --help` to learn more about fine-tuning your data.

## Should you check in data to your repository?

You can check in the generated data to your version control system, or you can add it to a build script. There are pros and cons of both approaches.

You should check in the generated data if:

1. You want fully reproducible, deterministic builds
2. You want to reduce build-time dependencies

You should generate it automatically at build time if:

1. You want to automatically download the latest CLDR/Unicode data
2. It is difficult to add large files to your VCS

If you check in the generated data, it is recommended that you configure a job in continuous integration that verifies that the data in your repository reflects the latest CLDR/Unicode releases; otherwise, your app may drift out of date.

## Locale Fallbacking

Data generated with `--format blob` or `--format fs` supports only exact matches for locales, not locales requiring _fallback_. For example, if `en-US` is requested but only `en` data is available, then the data request will fail. Because of this, it is often desirable to configure a `LocaleFallbackProvider`, as illustrated in the remainder of the examples on this page.

# 3. Using the generated data

Once we have generated some data, it needs to be loaded as a data provider. The blob format we chose can be loaded by `BlobDataProvider` from the `icu_provider_blob` crate.

This provider performs deserialization, so it's a `BufferProvider`. This means that the Cargo feature `"serde"` needs to be enabled on `icu`.

Let's update our `Cargo.toml`:

```console
$ cargo add icu --features serde
$ cargo add icu_provider_blob
```

We can then use the provider in our code:

```rust,no_run
use icu::locid::{locale, Locale};
use icu::calendar::DateTime;
use icu::datetime::{DateTimeFormatter, options::length};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::BlobDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let blob = std::fs::read("my_data_blob.postcard").expect("Failed to read file");
    let buffer_provider = 
        BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
            .expect("Failed to initialize Data Provider.");

    // Wrapping the raw BlobDataProvider in a LocaleFallbackProvider enables
    // locales to fall back to other locales, like "en-US" to "en".
    let buffer_provider = LocaleFallbackProvider::try_new_with_buffer_provider(buffer_provider)
        .expect("Provider should contain fallback rules");

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = DateTimeFormatter::try_new_with_buffer_provider(&buffer_provider, &LOCALE.into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_iso_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");
    let date = date.to_any();

    let formatted_date = dtf.format(&date).expect("Formatting should succeed");

    println!("📅: {}", formatted_date);
}
```

*Notice:* We used the `with_buffer_provider` constructor (`DateTimeFormatter::try_new_with_buffer_provider`) now, instead of the `with_any_provider` constructor that we used with test data, as we now have a `BufferProvider`. 

# 4. Data slicing

You might have noticed that the blob we generated is a hefty 13MB. This is no surprise, as we used `--keys all` and `--locales full`. However, our binary only uses date formatting data in Japanese. There's room for optimization:
```console
$ icu4x-datagen --keys-for-bin target/debug/myapp --locales ja --format blob --out my_data_blob.postcard --overwrite
```

The `--keys-for-bin` argument tells `icu4x-datagen` to analyze the binary and only include keys that are used by its code. In addition, we know that we only need data for the Japanese locale. This significantly reduces the blob's file size, to 54KB, and our program still works. Quite the improvement!

But there is more to optimize. You might have noticed this in the output of the `icu4x-datagen` invocation, which lists 21 keys, including clearly irrelevant ones like `datetime/ethopic/datesymbols@1`. Remember how we had to convert our `DateTime<Gregorian>` into a `DateTime<AnyCalendar>` in order to use the `DateTimeFormatter`? Turns out, as `DateTimeFormatter` contains logic for many different calendars, datagen includes data for all of these as well.

We can instead use `TypedDateTimeFormatter<Gregorian>`, which only supports formatting `DateTime<Gregorian>`s:

```rust,no_run
use icu::locid::{locale, Locale};
use icu::calendar::{DateTime, Gregorian};
use icu::datetime::{TypedDateTimeFormatter, options::length};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::BlobDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let blob = std::fs::read("my-data-blob").expect("Failed to read file");
    let buffer_provider = 
        BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
            .expect("Failed to initialize Data Provider.");

    // Wrapping the raw BlobDataProvider in a LocaleFallbackProvider enables
    // locales to fall back to other locales, like "en-US" to "en".
    let buffer_provider = LocaleFallbackProvider::try_new_with_buffer_provider(buffer_provider)
        .expect("Provider should contain fallback rules");

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(&buffer_provider, &LOCALE.into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

This has two advantages: it reduces our code size, as `DateTimeFormatter` includes much more functionality than `TypedDateTimeFormatter<Gregorian>`, and it reduces our data size, as static analysis can now determine that we need even fewer keys. The data size improvement could have also been achieved by manually listing the data keys we think we'll need (there's a `--keys` flag), but we risk a runtime error if we're wrong.

This is a common pattern in `ICU4X`, and most of our APIs are designed with data slicing in mind.

Rebuilding the application and rerunning datagen rewards us with a 3KB data blob, which only contains 7 data keys!

# 5. Other formats

So far we've used `--format blob` and `BlobDataProvider`. This is useful if we want to ship code and data separately, but there are other options.

## `mod` and baked data

The `mod` format will generate a Rust module that contains all the required data directly as Rust code. This format naturally has no deserialization overhead, and allows for compile-time optimizations (data slicing isn't really necessary, as the compiler will do it for us), but cannot be dynamically loaded at runtime.

Let's give it a try:

```console
$ icu4x-datagen --keys-for-bin target/debug/myapp --locales ja --format mod --out my-data-mod
```

The output might tell you additional crates that need to be installed. Don't worry, these are transitive dependencies already anyway, but are required directly now to construct our data:

```console
$ cargo add icu_provider
$ cargo add litemap
$ cargo add zerovec
```

We can then use the data by directly including the source with the `include!` macro.

```rust,compile_fail
extern crate alloc; // required as my-data-mod is written for #[no_std]
use icu::locid::{locale, Locale};
use icu::calendar::DateTime;
use icu::datetime::{TypedDateTimeFormatter, options::length};
use icu_provider_adapters::fallback::LocaleFallbackProvider;

const LOCALE: Locale = locale!("ja");

struct UnstableProvider;
include!("../my-data-mod/mod.rs");
impl_data_provider!(UnstableProvider);

fn main() {
    let unstable_provider = UnstableProvider;

    // Wrapping the raw UnstableProvider in a LocaleFallbackProvider enables
    // locales to fall back to other locales, like "en-US" to "en".
    let unstable_provider = LocaleFallbackProvider::try_new_unstable(unstable_provider)
        .expect("Provider should contain fallback rules");

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = TypedDateTimeFormatter::try_new_unstable(&unstable_provider, &LOCALE.into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

With this provider, we can use the `unstable` constructors. These are only guaranteed to work if the data was generated with the same version of ICU4X that you are building with, but if you build the data as part of your a build pipeline, that shouldn't be a problem.

You can also implement the `AnyProvider` trait, so that it can be used with `_with_any_provider` constructors. Using these constructors is slightly less performant than the `unstable` ones, and it doesn't allow for automatic data slicing, but, as the name suggests, it is stable across (minor) releases.

```rust,compile_fail
impl_any_provider!(MyProvider);
```

## `dir` and `FsDataProvider`

The `dir` format will generate a directory tree of data files in JSON (although the `--syntax` option can be used to generate `postcard` or `bincode` files, which doesn't have many practical uses).

Let's give it a try:

```console
$ icu4x-datagen --keys-for-bin target/debug/myapp --locales ja --format dir --out my-data-dir
```

This directory can be read by the `FsDataProvider` from the `icu_provider_fs` crate. You will also need to activate the Cargo feature for the chosen syntax on the `icu_provider` crate.

Same as `BlobDataProvider`, this also a buffer provider, so you will need to activate `icu`'s `serde` Cargo feature and use the `with_buffer_provider` constructors.

```console
$ cargo add icu --features serde
$ cargo add icu_provider --features deserialize_json
$ cargo add icu_provider_fs
```

```rust,no_run
use icu::locid::{locale, Locale};
use icu::calendar::DateTime;
use icu::datetime::{TypedDateTimeFormatter, options::length};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_fs::FsDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let buffer_provider = FsDataProvider::try_new("my-data-dir")
           .expect("Failed to initialize Data Provider");

    // Wrapping the raw BlobDataProvider in a LocaleFallbackProvider enables
    // locales to fall back to other locales, like "en-US" to "en".
    let buffer_provider = LocaleFallbackProvider::try_new_with_buffer_provider(buffer_provider)
        .expect("Provider should contain fallback rules");

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = TypedDateTimeFormatter::try_new_with_buffer_provider(&buffer_provider, &LOCALE.into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

# 6. Summary

We have learned how to generate data and load it into our programs, optimize data size, and gotten to know the different data providers that are part of `ICU4X`.

For a deeper dive into configuring your data providers in code, see [data_provider.md].

You can learn more about datagen, including the Rust API which we have not used in this tutorial, by reading [the docs](https://docs.rs/icu_datagen/latest/).
