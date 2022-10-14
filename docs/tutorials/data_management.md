# Data management in ICU4X

Unless you're happy shipping your app with the ~10 locales supported by `icu_testdata` (and with the size impact of all keys), you will need generate your own data that is customized to suit your needs.

This tutorial introduces data providers beyond `icu_testdata`, as well as the `icu_datagen` tool.

# 1. Prerequesites

This tutorial assumes you have finished the [introductory tutorial](intro.md) and continues where that tutorial left off. In particular, you should still have the latest version of code for `myapp`.

# 2. Generating data

Data generation is done using the `icu_datagen` crate, which pulls in data from [Unicode's *Common Locale Data Repository* (*CLDR*)](http://cldr.unicode.org/index/downloads) and from `ICU4C` releases to generate `ICU4X` data. The crate has a command line interface as well as a Rust API, which can be used for example in `build.rs` files. Here we will use the CLI.

First we will need to install the binary:

```console
$ cargo install icu_datagen --features bin
```

Get a coffee, this might take a while ☕.

Once installed, run:

```console
$ icu4x-datagen --cldr-tag latest --icuexport-tag latest --out my-data-blob --format blob --all-keys --all-locales
```

Let's dissect this invocation:
* `--cldr-tag` selects the CLDR version to use
* `--icuexport-tag` selects the ICU-exported data version to use
* `--out` is the location where we want the generated ICU4X data to be stored
* `--format` sets the format of the output (we'll discuss formats later)
* `--all-keys` `--all-locales` specifies that we want to include all data for all locales

This will generate a `my-data-blob` file containing the serialized data.

# 3. Using the generated data

Once we have generated some data, it needs to be loaded as a data provider. The blob format we chose can be loaded by `BlobDataProvider` from the `icu_provider_blob` crate.

This provider performs deserialization, so it's a `BufferProvider`. This means that the feature `"serde"` needs to be enabled on `icu`.

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
use icu_provider_blob::BlobDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let blob = std::fs::read("my-data-blob").expect("Failed to read file");
    let buffer_provider = 
        BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
            .expect("Failed to initialize Data Provider.");

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

You might have noticed that the blob we generated is a hefty 13MB. This is no surprise, as we included `--all-keys` `--all-locales`. However, our binary only uses date formatting data in Japanese. There's room for optimization:

```console
$ icu4x-datagen --overwrite --cldr-tag latest --icuexport-tag latest --out my-data-blob --format blob --keys-for-bin target/debug/myapp --locales ja
```

The `--keys-for-bin` argument tells `icu4x-datagen` to analyze the binary and only include keys that are used by its code. In addition, we know that we only need data for the Japanese locale. This significantly reduces the blob's file size, to 54KB, and our program still works. Quite the improvement!

But there is more to optimize. You might have noticed this in the output of the `icu4x-datagen` invocation, which lists 21 keys, including clearly irrelevant ones like `datetime/ethopic/datesymbols@1`. Remember how we had to convert our `DateTime<Gregorian>` into a `DateTime<AnyCalendar>` in order to use the `DateTimeFormatter`? Turns out, as `DateTimeFormatter` contains logic for many different calendars, datagen includes data for all of these as well.

We can instead use `TypedDateTimeFormatter<Gregorian>`, which only supports formatting `DateTime<Gregorian>`s:

```rust,no_run
use icu::locid::{locale, Locale};
use icu::calendar::{DateTime, Gregorian};
use icu::datetime::{TypedDateTimeFormatter, options::length};
use icu_provider_blob::BlobDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let blob = std::fs::read("my-data-blob").expect("Failed to read file");
    let buffer_provider = 
        BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
            .expect("Failed to initialize Data Provider.");

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

Rebuilding the application and rerunning datagen awards us with a 3KB data blob, which only contains 7 data keys!

# 5. Other formats

So far we've used `--format blob` and `BlobDataProvider`. This is useful if we want to ship code and data separately, but there are other options.

## `mod` and `BakedDataProvider`

The `mod` format will generate a Rust module that defines a data provider. This format naturally has no deserialization overhead, and allows for compile-time optimizations (data slicing isn't really necessary, as the compiler will do it for us), but cannot be dynamically loaded at runtime.

Let's give it a try:

```console
$ icu4x-datagen --cldr-tag latest --icuexport-tag latest --out my-data-mod --format mod --keys-for-bin target/debug/myapp --locales ja
```

The output might tell you additional crates that need to be installed. Don't worry, these are transitive dependencies already anyway, but are required directly now to construct our data:

```console
$ cargo add icu_provider
$ cargo add litemap
$ cargo add zerovec
```

We can then use the data by directly including the source with the `include!` macro.

```rust,compile_fail
extern crate alloc; // required as BakedDataProvider is written for #[no_std]
use icu::locid::{locale, Locale};
use icu::calendar::DateTime;
use icu::datetime::{TypedDateTimeFormatter, options::length};

const LOCALE: Locale = locale!("ja");

include!("../my-data-mod/mod.rs"); // defines BakedDataProvider

fn main() {
    let unstable_provider = BakedDataProvider;

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = TypedDateTimeFormatter::try_new_unstable(&unstable_provider, &LOCALE.into(), options.into())
        .expect("Failed to initialize DateTimeFormatter");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("Failed to create a datetime.");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

With this provider, we can use the `unstable` constructors. These are only guaranteed to work if the `BakedDataProvider` was generated with the same version of ICU4X that you are building with, but if you build the data as part of your a build pipeline, that shouldn't be a problem.

You can also make the `BakedDataProvider` implement the `AnyProvider` trait, so that it can be used with `_with_any_provider` constructors. Using these constructors is slightly less performant than the `unstable` ones, but, as the name suggests, stable across (minor) releases.

```rust,compile_fail
include!("../my-data-mod/mod.rs");
include!("../my-data-mod/any.rs");
let _any_provider = BakedDataProvider;
```

## `dir` and `FsDataProvider`

The `dir` format will generate a directory tree of data files in JSON (although the `--syntax` option can be used to generate `postcard` or `bincode` files, which doesn't have many practical uses).

Let's give it a try:

```console
$ icu4x-datagen --cldr-tag latest --icuexport-tag latest --out my-data-dir --format dir --keys-for-bin target/debug/myapp --locales ja
```

This directory can be read by the `FsDataProvider` from the `icu_provider_fs` crate. You will also need to activate the feature for the chosen syntax on the `icu_provider` crate.

Same as `BlobDataProvider`, this also a buffer provider, so you will need to activate `icu`'s `serde` feature and use the `with_buffer_provider` constructors.

```console
$ cargo add icu --features serde
$ cargo add icu_provider --features deserialize_json
$ cargo add icu_provider_fs
```

```rust,no_run
use icu::locid::{locale, Locale};
use icu::calendar::DateTime;
use icu::datetime::{TypedDateTimeFormatter, options::length};
use icu_provider_fs::FsDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let buffer_provider = FsDataProvider::try_new("my-data-dir")
           .expect("Failed to initialize Data Provider");

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

You can learn more about datagen, including the Rust API which we have not used in this tutorial, by reading [the docs](https://icu4x.unicode.org/doc/icu_datagen/).
