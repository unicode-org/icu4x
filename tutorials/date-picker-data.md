# Interactive Date Picker - Custom Data

In this tutorial, we will add additional locale data to your app. ICU4X compiled data contains data for hundreds of languages, but there are languages that have data in CLDR that are not included (generally because they don't have comprehensive coverage). For example, if you try using the locale `ccp` (Chakma) in your app, you will get output like `2023 M11 7`. Believe it or not, but this is not actually correct output for Chakma. Instead ICU4X fell back to the "root locale", which tries to be as neutral as possible. Note how it avoided calling the month by name by using `M11`, even though we requested a format with a non-numeric month name.

So, let's add some data for Chakma.

## 1. Installing `icu4x-datagen`

Data generation is done using the `icu4x-datagen` tool, which pulls data from [Unicode's *Common Locale Data Repository* (*CLDR*)](https://cldr.unicode.org/index/downloads) and from `ICU4C` releases.

Verify that Rust is installed. If it's not, you can install it in a few seconds from [https://rustup.rs/](https://rustup.rs/).

```console
cargo --version
# cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

Now you can run

```console
cargo install icu4x-datagen
```

## 2. Generating the data pack

We're ready to generate the data. We will use the blob format, and create a blob that will contain just Chakma data. At runtime we can then load it as needed.

```console
icu4x-datagen --markers all --locales ccp --format blob --out ccp.blob
```

This will generate a `ccp.blob` file containing data for Chakma.

💡 Note: if you're having technical difficulties, this file is available [here](https://storage.googleapis.com/static-493776/icu4x_2025-11-11/ccp.blob) (generated with icu4x 2.1).


## 3. Using the data pack

### Rust Part 3

To use blob data, we will need to add the `icu_provider_blob` crate to our project:

```console
cargo add icu_provider_blob --features alloc
```

We also need to enable the `serde` feature on the `icu` crate to enable deserialization support:

```console
cargo add icu --features serde
```

Now, update the instantiation of the datetime formatter to load data from the blob if the
locale is Chakma:

```rust,no_run
// At the top of the file:
use icu::locale::{locale, Locale};
use icu_provider_blob::BlobDataProvider;
use icu::calendar::{Gregorian, Date};
use icu::datetime::{DateTimeFormatter, FixedCalendarDateTimeFormatter, fieldsets::YMD};

// replace the date_formatter creation
let locale = locale!("ccp");
let date_formatter = if locale == locale!("ccp") {
    println!("Using buffer provider");

    let blob = std::fs::read("ccp.blob")
        .expect("blob should read successfully")
        .into();

    let provider =
        BlobDataProvider::try_new_from_blob(blob).expect("deserialization should succeed");

    DateTimeFormatter::try_new_with_buffer_provider(&provider, locale.into(), YMD::medium())
        .expect("should have data for selected locale")
} else {
    // As before
    DateTimeFormatter::try_new(locale.into(), YMD::medium())
        .expect("should have data for specified locale")
};
```

Try using `ccp` now!

### JavaScript Part 3

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
    load_blob("https://storage.googleapis.com/static-493776/icu4x_2025-11-11/ccp.blob", (blob) => {
        let dateTimeFormatter = DateTimeFormatter.createYmdtWithProvider(
            DataProvider.fromByteSlice(blob),
            locale,
            DateTimeLength.Long,
        );
        document.getElementById("output").innerText = dateTimeFormatter.formatIso(isoDate, time);
    })
} else {
    let dateTimeFormatter = DateTimeFormatter.createYmdt(
            locale,
            DateTimeLength.Long,
    );
    document.getElementById("output").innerText = dateTimeFormatter.formatIso(isoDate, time);
}
```

Try using `ccp` now!

## 4. Slimming the data pack

Note: the following steps are currently only possible in Rust. 🤷

When we ran `icu4x-datagen`, we passed `--markers all`, which make it generate *all* data for the Chakma locale, even though we only need date formatting. We can make `icu4x-datagen` analyze our binary to figure out which markers are needed:

```console
cargo build --release
icu4x-datagen --markers-for-bin target/release/tutorial --locales ccp --format blob --out ccp_smaller.blob
```

Note: you usually want to build with the `--release` flag, and analyze that binary.

This should generate a lot fewer markers!

Let's look at the sizes:

```console
wc -c *.blob
# 5448603 ccp.blob
#   13711 ccp_smaller.blob
```

This is much better! Rerun your app with `ccp_smaller.blob` to make sure it still works!

💡 Note: if you're having technical difficulties, this file is available [here](https://storage.googleapis.com/static-493776/icu4x_2025-11-11/ccp_smaller.blob) (generated with icu4x 2.1).

## 5. Slimming the data pack ... again

The last datagen invocation still produced a lot of markers, as you saw in its output. This is because we used the `DateTimeFormatter` API, which can format dates for a lot of different calendars (remember `en-u-ca-hebrew`). However, if we were only using it with a Gregorian calendar date, we wouldn't need Coptic, Indian, etc. date formatting data. Now, how do we communicate this to `--markers-for-bin`? Turns out, `icu::datetime` also exposes a `FixedCalendarDateTimeFormatter`, which is generic in a single calendar type. If you use this API instead, `--markers-for-bin` will only include the markers for that one calendar type.

Replace the `DateTimeFormatter::try_new` calls with `FixedCalendarDateTimeFormatter::try_new`, and change the `format` invocation to convert the input to the Gregorian calendar:

```rust,no_run
    println!("Date: {}", date_formatter.format(&iso_date.to_calendar(Gregorian)));
```

The generic type of `FixedCalendarDateTimeFormatter` will be inferred from the input, which now has type `&Date<Gregorian>` now. Unlike `DateTimeFormatter`, `FixedCalendarDateTimeFormatter` never applies calendar conversions on its input, so it will be a `FixedCalendarDateTimeFormatter<Gregorian, ...>`.

Now we can run datagen with `--markers-for-bin` again:

```console
cargo build --release
icu4x-datagen --markers-for-bin target/release/tutorial --locales ccp --format blob --out ccp_smallest.blob
```

The output will be much shorter:

```console
2025-05-14T14:26:52.306Z INFO  [icu_provider_export::export_impl] Generated marker DatetimeNamesMonthGregorianV1
2025-05-14T14:26:52.308Z INFO  [icu_provider_export::export_impl] Generated marker DatetimeNamesYearGregorianV1
2025-05-14T14:26:52.312Z INFO  [icu_provider_export::export_impl] Generated marker DatetimePatternsDateGregorianV1
2025-05-14T14:26:52.324Z INFO  [icu_provider_export::export_impl] Generated marker DecimalDigitsV1
2025-05-14T14:26:52.325Z INFO  [icu_provider_export::export_impl] Generated marker DecimalSymbolsV1
```

And the blob will also be much smaller at the sizes:

```console
wc -c *.blob
# 5448603 ccp.blob
#   13711 ccp_smaller.blob
#    4711 ccp_smallest.blob
```

Rerun your app with `ccp_smallest.blob` to make sure it still works!

💡 Note: if you're having technical difficulties, this file is available [here](https://storage.googleapis.com/static-493776/icu4x_2025-11-11/ccp_smallest.blob) (generated with icu4x 2.1).

Does the locale `en-u-ca-hebrew` still use the Hebrew calendar? Why is that?
