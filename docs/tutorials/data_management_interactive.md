# Building Custom Data for the ICU4X Date picker

In this tutorial, we will add additional locale data to your app. ICU4X compiled data contains data for hundreds of languages, but there are languages that have data in CLDR that are not included (generally because they don't have comprehensive coverage). For example, if you try using the locale `ccp` (Chakma) in your app, you will get output like `2023 M11 7`. Believe it or nit, but this is not actually correct output for Chakma. Instead ICU4X fell back to the "root locale", which tries to be as neutral as possible. Note how it avoided calling the month by name by using `M11`, even though we requested a format with a non-numeric month name.

So, let's add some data for Chakma.

## 1. Installing `icu4x-datagen`

Data generation is done using the `icu4x-datagen` tool, which pulls data from [Unicode's *Common Locale Data Repository* (*CLDR*)](http://cldr.unicode.org/index/downloads) and from `ICU4C` releases.

Verify that Rust is installed. If it's not, you can install it in a few seconds from [https://rustup.rs/](https://rustup.rs/).

```console
$ cargo --version
cargo 1.71.1 (7f1d04c00 2023-07-29)
```

Now you can run

```console
cargo install icu_datagen
```

## 2. Generating the data pack

We're ready to generate the data. We will use the blob format, and create a blob that will contain just Chakma data. At runtime we can then load it as needed.

```console
$ icu4x-datagen --keys all --locales ccp --format blob --out ccp.blob
```

This will generate a `ccp.blob` file containing data for Chakma.

Note: if you're having technical difficulties, this file is available [here](https://storage.googleapis.com/static-493776/icu4x_2023-11-03/ccp.blob).

## 3. Using the data pack

### Rust

To use blob data, we will need to add the `icu_provider_blob` crate to our project:

```console
cargo add icu_provider_blob
```

Now, update the instatiation of the date formatter to load data from the blob if the
locale is Chakma:

```rust
// new imports
use icu::locid::locale;
use icu_provider_blob::BlobDataProvider;

const CCP_BLOB_PATH: &str = "<absolute path to ccp.blob>";

let date_formatter = if locale == locale!("ccp") {
    println!("Using buffer provider");

    let blob = std::fs::read(CCP_BLOB_PATH)
        .expect("blob should read successfully")
        .into();

    let provider =
        BlobDataProvider::try_new_from_blob(blob).expect("deserialization should succeed");

    DateFormatter::try_new_with_length_with_buffer_provider(
        &provider,
        &(&locale).into(),
        length::Date::Medium,
    )
    .expect("should have data for selected locale")
} else {
    // As before
    DateFormatter::try_new_with_length(&(&locale).into(), length::Date::Medium)
        .expect("should have data for selected locale")
};
```

Try using `ccp` now!

### JavaScript

Update the formatting logic to load data from the blob if the locale is Chakma. Note that this code uses a callback, as it does an HTTP request:

```javascript

function load_blob(url, callback) {
    const req = new XMLHttpRequest();
    req.open("GET", url, true);
    req.responseType = "arraybuffer";
    req.onload = (event) => {
        if (req.response) {
            callback(new Uint8Array(req.response));
        }
    };
    req.send(null);
}

if (localeStr == "ccp") {
    load_blob("https://storage.googleapis.com/static-493776/icu4x_2023-11-03/ccp.blob", (blob) => {
        let dateFormatter = ICU4XDateFormatter.create_with_length(
            ICU4XDataProvider.create_from_byte_slice(blob),
            locale,
            ICU4XDateLength.Medium,
        );
        document.getElementById("output").innerText = dateFormatter.format_iso_date(isoDate);
    })
} else {
    let dateFormatter = ICU4XDateFormatter.create_with_length(
        ICU4XDataProvider.create_compiled(),
        locale,
        ICU4XDateLength.Medium,
    );
    document.getElementById("output").innerText = dateFormatter.format_iso_date(isoDate);
}
```

Try using `ccp` now!

## 4. Slimming the data pack

Note: the following steps are currently only possible in Rust. 🤷

When we ran `icu4x-datagen`, we passed `--keys all`, which make it generate *all* data for the Chakma locale, even though we only need date formatting. We can make `icu4x-datagen` analyze our binary to figure out which keys are needed:

```console
$ icu4x-datagen --keys-for-bin target/debug/tutorial --locales ccp --format blob --out ccp_smaller.blob
```

Note: you usually want to build with the `--release` flag, and analyze that binary, but we don't have all day.

This should generate a lot fewer keys!

Let's look at the sizes:

```console
$ wc -c *.blob
656767 ccp.blob
 45471 ccp_smaller.blob
```

This is much better! Rerun your app with `ccp_smaller.blob` to make sure it still works!

## 5. Slimming the data pack ... again

The last datagen invocation still produced a lot of keys, as you saw in its output. This is because we used the `DateFormatter` API, which can format dates for a lot of different calendars. However, we are only using it with an ISO/Gregorian calendar date, so we don't *really* need Coptic, Indian, etc. date formatting data. There are two ways to fix this:

### Option 1: Specify keys manually

Recall the output of the last datagen invocation:

```console
INFO  [icu_datagen] Generating key calendar/japanese@1
INFO  [icu_datagen] Generating key calendar/japanext@1
INFO  [icu_datagen] Generating key datetime/buddhist/datelengths@1
INFO  [icu_datagen] Generating key datetime/buddhist/datesymbols@1
INFO  [icu_datagen] Generating key datetime/coptic/datelengths@1
INFO  [icu_datagen] Generating key datetime/coptic/datesymbols@1
INFO  [icu_datagen] Generating key datetime/ethiopic/datelengths@1
INFO  [icu_datagen] Generating key datetime/ethiopic/datesymbols@1
INFO  [icu_datagen] Generating key datetime/gregory/datelengths@1
INFO  [icu_datagen] Generating key datetime/gregory/datesymbols@1
INFO  [icu_datagen] Generating key datetime/indian/datelengths@1
INFO  [icu_datagen] Generating key datetime/indian/datesymbols@1
INFO  [icu_datagen] Generating key datetime/japanese/datelengths@1
INFO  [icu_datagen] Generating key datetime/japanese/datesymbols@1
INFO  [icu_datagen] Generating key datetime/japanext/datelengths@1
INFO  [icu_datagen] Generating key datetime/japanext/datesymbols@1
INFO  [icu_datagen] Generating key datetime/week_data@1
INFO  [icu_datagen] Generating key decimal/symbols@1
INFO  [icu_datagen] Generating key plurals/ordinal@1
```

Now let's just remove non-gregorian calendar manually:

```console
INFO  [icu_datagen] Generating key datetime/gregory/datelengths@1
INFO  [icu_datagen] Generating key datetime/gregory/datesymbols@1
INFO  [icu_datagen] Generating key datetime/week_data@1
INFO  [icu_datagen] Generating key decimal/symbols@1
INFO  [icu_datagen] Generating key plurals/ordinal@1
```

We can then use the `--keys` flag to pass these to datagen:

```console
$ icu4x-datagen --keys datetime/gregory/datelengths@1 datetime/gregory/datesymbols@1 datetime/week_data@1 decimal/symbols@1 plurals/ordinal@1 --locales ccp --format blob --out ccp_smallest.blob
```

Let's look at the sizes:

```console
$ wc -c *.blob
656767 ccp.blob
 45471 ccp_smaller.blob
  4639 ccp_smallest.blob
```

Even better! Rerun your app with `ccp_smallest.blob` to make sure it still works!

### Option 2: Using a more granular API

We've seen that `DateFormatter` pulls in a lot of data. It would be nice if we could tell it that we'll only ever use it with Gregorian dates. Turns out we can! `icu::datetime` also exposes a `TypedDateFormatter<C>`, which is generic in a single calendar type. If you use this API instead (instantiated as `TypedDateFormatter<Greogorian>`), `--keys-for-bin` will give you exactly the keys we manually selected in the last section. However, now you can be sure that you didn't make a mistake selecting the keys (which would be an awkward runtime error), and that you will never accidentally pass a non-Gregorian date into the formatter (which would an awkward runtime error with `DateFormatter`, but is a compile-time error with `TypeDateFormatter`).

```rust
let date_formatter = TypedDateFormatter::<Gregorian>::try_new_with_length(
    &(&locale).into(),
    length::Date::Medium,
)
.expect("should have data for selected locale");

println!(
    "Date: {}",
    date_formatter
         // Note that we're not converting to a `Date<Any>` with `.to_any()`
         // anymore. This is where you'll get a compile error if you pass a `Date<Indian>`.
        .format(&iso_date)
        .expect("date should format successfully")
);
```
