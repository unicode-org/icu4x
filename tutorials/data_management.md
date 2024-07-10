# Data management in ICU4X

This tutorial introduces data providers as well as the `icu4x-datagen` tool.

If you're happy shipping your app with the recommended set of locales included in `ICU4X`, you can stop reading now. If you want to reduce code size, do runtime data loading, or build your own complex data pipelines, this tutorial is for you.

# 1. Prerequisites

This tutorial assumes you have finished the [introductory tutorial](intro.md) and continues where that tutorial left off. In particular, you should still have the latest version of code for `myapp`.

# 2. Generating data

Data generation is done using the `icu4x-datagen` tool, which pulls in data from [Unicode's *Common Locale Data Repository* (*CLDR*)](http://cldr.unicode.org/index/downloads) and from `ICU4C` releases to generate `ICU4X` data.

First we will need to install the binary:

```console
$ cargo install icu4x-datagen
```

Get a coffee, this might take a while ☕.

Once installed, run:

```console
$ icu4x-datagen --markers all --locales ja --format mod --out my_data
```

This will generate a `my_data` directory containing the data for all components in the `ja` locale.

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

# 3. Using the generated data

Once we have generated the data, we need to instruct `ICU4X` to use it. To do this, set the `ICU4X_DATA_DIR` during the compilation of your app:

```command
ICU4X_DATA_DIR=$(pwd)/my_data cargo run
```

This will replace the data that's bundled in `ICU4X` by your own. It should result in a smaller binary, as we're including only a single locale.

# 4. The `DataProvider`

Replacing `ICU4X`'s bundled data by your own can be useful if you don't require the full set of locales that would otherwise be bundled, but still requires including data at compile time, which is limiting. For more flexible data management, `ICU4X` provides a trait called `DataProvider` which can be used to provide data to `ICU4X` APIs. It also provides several implementations of `DataProvider`s, which all have their own use cases. Users are also free to design their own providers that best fit into their ecosystem requirements.

```rust,ignore
trait DataProvider<M: DataMarker> {
    fn load(&self, req: DataRequest) -> Result<DataResponse, DataError>;
}
```

As you can see, the `DataProvider` trait is fairly simple. It's generic in a `DataMarker`, which (statically) marks the type of data that an implementation returns, and there's a single method that loads the data for a given `DataRequest` (which contains the locale).

## `BufferProvider` and `AnyProvider`

Static markers that determine the type of data is great for data that is compiled into the binary, as it allows the compiler to eliminate unused data, but for runtime data loading it is too restrictive. For this reason `ICU4X` defines two more traits: `BufferProvider` abstracts over data providers that provide opaque byte buffers which can be deserialized, and `AnyProvider` which uses dynamic typing with `Any`.

Because of these three data provider types, every `ICU4X` API has four constructors:
* `try_new`: This uses the built-in data provider
* `try_new_with_any_provider`: This loads data from a provided `&impl AnyProvider`, downcasting it
* `try_new_with_buffer_provider`: This loads data from a provided `&impl BufferProvider`, deserializing it
* `try_new_unstable`: This loads data from a provided `&impl DataProvider<X + Y + ...>`. It is *semver unstable*, because the bounds on the provider might change in case more data is required in the future.

# 5. Using the generated data explicitly

The data we generated in section 2 is actually just Rust code defining `DataProvider` implementations for all markers using hardcoded data (go take a look!).

So far we've used it through the default `try_new` constructor by using the environment variable to replace the built-in data. However, we can also directly access the `DataProvider` implementations if we want, for example to combine it with other providers. For this, we first need to add some dependencies (`icu4x-datagen` did tell you which ones you need):

```console
$ cargo add icu_locale_core
$ cargo add icu_provider
$ cargo add icu_provider_baked
$ cargo add zerotrie
$ cargo add zerovec
```

We can include the generate code with the `include!` macro. The `impl_data_provider!` macro adds the generated implementations to any type.

```rust,compile_fail
extern crate alloc; // required as my-data is written for #[no_std]
use icu::locale::{locale, Locale};
use icu::calendar::DateTime;
use icu::datetime::{DateTimeFormatter, options::length};

const LOCALE: Locale = locale!("ja");

struct MyDataProvider;
include!("../my-data/mod.rs");
impl_data_provider!(MyDataProvider);

fn main() {
    let baked_provider = MyDataProvider;

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = DateTimeFormatter::try_new_unstable(&baked_provider, &LOCALE.into(), options.into())
        .expect("ja data should be available");

    let date = DateTime::try_new_iso_datetime(2020, 10, 14, 13, 21, 28)
        .expect("datetime should be valid");
    let date = date.to_any();

    let formatted_date = dtf.format(&date).expect("formatting should succeed");

    println!("📅: {}", formatted_date);
}
```

# 4. `BlobDataProvider`

The baked data provider is great because it allows serialization-free access to data, but it needs to be known at compile time. `ICU4X`'s other main provider is the `BlobDataProvider` (`icu_provider_blob`), which is a `BufferProvider` that allows loading byte data at runtime.

To use `BufferProvider`s, the Cargo feature `"serde"` needs to be enabled on `icu` to gain access to `_with_buffer_provider` constructors (which include the concrete deserialization code).

Let's update our `Cargo.toml`:

```console
$ cargo add icu --features serde
$ cargo add icu_provider_blob
```

We can generate data for it using the `--format blob2` flag:

```console
$ icu4x-datagen --markers all --locales ja --format blob2 --out my_data_blob.postcard
```

This will generate a `my_data_blob.postcard` file containing the serialized data for all components. The file is several megabytes large; we will optimize it later!

💡 Note: `--format blob2` generates version 2 of the blob format. Alternatively, `--format blob` produces an older blob format which works with ICU4X prior to 1.4 but is not as optimized.

## Locale Fallbacking

Unlike `BakedDataProvider`, `BlobDataProvider` (and `FsDataProvider`) does not perform locale fallbacking. For example, if `en-US` is requested but only `en` data is available, then the data request will fail. To enable fallback, we can wrap the provider in a `LocaleFallbackProvider`.

Note that fallback comes at a cost, as fallbacking code and data has to be included and executed on every request. If you don't need fallback (disclaimer: you probably do), you can use the `BlobDataProvider` directly (for baked data, see [`Options::skip_internal_fallback`](https://docs.rs/icu_provider_baked/latest/icu_provider_baked/export/struct.Options.html)).

We can then use the provider in our code:

```rust,no_run
use icu::locale::{locale, Locale, fallback::LocaleFallbacker};
use icu::calendar::DateTime;
use icu::datetime::{DateTimeFormatter, options::length};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::BlobDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let blob = std::fs::read("my_data_blob.postcard").expect("Failed to read file");
    let buffer_provider = 
        BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
            .expect("blob should be valid");

    let fallbacker = LocaleFallbacker::try_new_with_buffer_provider(&buffer_provider)
        .expect("Provider should contain fallback rules");

    let buffer_provider = LocaleFallbackProvider::new(buffer_provider, fallbacker);

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = DateTimeFormatter::try_new_with_buffer_provider(&buffer_provider, &LOCALE.into(), options.into())
        .expect("blob should contain required markers and `ja` data");

    let date = DateTime::try_new_iso_datetime(2020, 10, 14, 13, 21, 28)
        .expect("datetime should be valid");
    let date = date.to_any();

    let formatted_date = dtf.format(&date).expect("formatting should succeed");

    println!("📅: {}", formatted_date);
}
```

As you can see in the second `expect` message, it's not possible to statically tell whether the correct data markers are included. While `BakedDataProvider` would result in a compile error for missing `DataProvider<M>` implementations, `BlobDataProvider` returns runtime errors if markers are missing.

# 5. Data slicing

You might have noticed that the blob we generated is a hefty 13MB. This is no surprise, as we used `--markers all`. However, our binary only uses date formatting data in Japanese. There's room for optimization:

```console
$ icu4x-datagen --markers-for-bin target/debug/myapp --locales ja --format blob2 --out my_data_blob.postcard --overwrite
```

The `--markers-for-bin` argument tells `icu4x-datagen` to analyze the binary and only include markers that are used by its code. This significantly reduces the blob's file size, to 54KB, and our program still works. Quite the improvement!

But there is more to optimize. You might have noticed this in the output of the `icu4x-datagen` invocation, which lists 24 markers, including clearly irrelevant ones like `datetime/ethopic/datesymbols@1`. Remember how we had to convert our `DateTime<Gregorian>` into a `DateTime<AnyCalendar>` in order to use the `DateTimeFormatter`? Turns out, as `DateTimeFormatter` contains logic for many different calendars, datagen includes data for all of these as well.

We can instead use `TypedDateTimeFormatter<Gregorian>`, which only supports formatting `DateTime<Gregorian>`s:

```rust,no_run
use icu::locale::{locale, Locale, fallback::LocaleFallbacker};
use icu::calendar::{DateTime, Gregorian};
use icu::datetime::{TypedDateTimeFormatter, options::length};
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::BlobDataProvider;

const LOCALE: Locale = locale!("ja");

fn main() {
    let blob = std::fs::read("my-data-blob").expect("Failed to read file");
    let buffer_provider = 
        BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
            .expect("blob should be valid");

    let fallbacker = LocaleFallbacker::try_new_with_buffer_provider(&buffer_provider)
        .expect("Provider should contain fallback rules");

    let buffer_provider = LocaleFallbackProvider::new(buffer_provider, fallbacker);

    let options = length::Bag::from_date_time_style(length::Date::Long, length::Time::Medium);

    let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_with_buffer_provider(&buffer_provider, &LOCALE.into(), options.into())
        .expect("blob should contain required data");

    let date = DateTime::try_new_gregorian_datetime(2020, 10, 14, 13, 21, 28)
        .expect("datetime should be valid");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

This has two advantages: it reduces our code size, as `DateTimeFormatter` includes much more functionality than `TypedDateTimeFormatter<Gregorian>`, and it reduces our data size, as `--markers-for-bin` can now determine that we need even fewer markers. The data size improvement could have also been achieved by manually listing the data markers we think we'll need (using the `--markers` flag), but we risk a runtime error if we're wrong.

This is a common pattern in `ICU4X`, and most of our APIs are designed with data slicing in mind.

Rebuilding the application and rerunning datagen rewards us with a 3KB data blob, which only contains 7 data markers!

These API-level optimizations also apply to compiled data (there's no need to use `--markers-for-bin`, as the compiler will remove unused markers).

# 6. Summary

We have learned how to generate data and load it into our programs, optimize data size, and gotten to know the different data providers that are part of `ICU4X`.

For a deeper dive into configuring your data providers in code, see [data_provider.md].

You can learn more about datagen, including the Rust API which we have not used in this tutorial, by reading [the docs](https://docs.rs/icu_provider_export/latest/).
