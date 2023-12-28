# Building an Interactive Date Picker with ICU4X

In this tutorial, you will learn how to build an end-to-end application using ICU4X to format a date and time with some default locales and additional locales loaded dynamically.

This tutorial is written in parallel between **Rust** and **JavaScript** in a web browser.

## 1. Installing ICU4X

Installing dependencies is always your first step.

### Rust Part 1

Verify that Rust is installed. If it's not, you can install it in a few seconds from [https://rustup.rs/](https://rustup.rs/).

```console
cargo --version
# cargo 1.71.1 (7f1d04c00 2023-07-29)
```

Create a new Rust binary crate with icu4x as a dependency:

```console
cargo new --bin tutorial
cd tutorial
cargo add icu
```

### JavaScript Part 1

We recommend using [CodePen](https://codepen.io/pen/?editors=1011) to follow along. To load ICU4X into CodePen, you can use this snippet in the JavaScript editor:

```javascript
import { ICU4XLocale, ICU4XDataProvider, ICU4XDateFormatter, ICU4XDateTimeFormatter, ICU4XDateLength, ICU4XIsoDate, ICU4XIsoDateTime, ICU4XTimeLength } from "https://storage.googleapis.com/static-493776/icu4x_2023-11-03/js/index.js";
```

This loads the full development ICU4X WebAssembly file. Since it may take some time to load on slow connections, we'll create a loading div. In future tutorials you will learn how to build an optimized WebAssembly file, reducing the size of the WASM file by 99% or more. Add this to your HTML:

```html
<div id="loading">Loading…</div>

<div id="inputoutput" style="display: none">
  <p>Ready to rumble!</p>
</div>
```

And in JavaScript, add these lines after the import statement:

```javascript
document.getElementById("loading").style.display = "none";
document.getElementById("inputoutput").style.display = "block";
```

## 2. Parsing an Input Locale

Here, we will accept a locale string from the user and parse it into an ICU4X Locale.

### Rust Part 2

First, we will use Rust APIs to accept a string from user input on the command line. Then we can parse the input string as an ICU4X `Locale`. Add the following to your `fn main()`:

```rust,no_run
// At the top of the file:
use icu::locid::Locale;

// In the main() function:
print!("Enter your locale: ");
std::io::Write::flush(&mut std::io::stdout()).unwrap();
let locale_str = {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf
};

// Since the string contains whitespace, we must call `.trim()`:
let locale = match locale_str.trim().parse::<Locale>() {
    Ok(locale) => {
        println!("You entered: {locale}");
        locale
    }
    Err(e) => {
        panic!("Error parsing locale! {e}");
    }
};
```

Try inputting locales in non-canonical syntax and see them normalized!

```bash
$ cargo run
Enter your locale: DE_CH
You entered: de-CH
```

### JavaScript Part 2

In the HTML, create an input element for accepting a locale string input, and an output element to echo it back to the user. Add this inside of the `inputoutput` div:

```html
<!-- inside of div id="inputoutput" -->
<p><label>Locale: <input type="text" id="localeinput" value="en-US"/></label></p>
<p>Output: <output id="output"></output></p>
```

And in JavaScript:

```javascript
// Create a function that updates the UI:
function update() {
    try {
        let localeStr = document.getElementById("localeinput").value;
        let locale = ICU4XLocale.create_from_string(localeStr);
        document.getElementById("output").innerText = locale.to_string();
    } catch(e) {
        document.getElementById("output").innerText = e + ": " + e.error_value;
    }
}

// Run the function whenever the locale input changes:
document.getElementById("localeinput").addEventListener("keyup", update, false);

// Also run the function right now to initialize the UI:
update();
```

Try inputting locales in non-canonical syntax and see them normalized!

> Locale: ES_419\
> Output: es-419

## 3. Formatting a Date

Now we will use built-in locale data to produce a formatted date.

### Rust Part 3

We would like to format today's date. We will get this from the `time` crate, which you need to add:

```console
$ cargo add time --features local-offset
```

Now we can write the Rust code:

```rust
// At the top of the file:
use icu::calendar::{Date, Iso};
use icu::datetime::options::length;
use icu::datetime::DateFormatter;

let locale = icu::locid::Locale::UND; // to make this example compile

/// Helper function to create an ICU4X DateTime for the current local time:
fn get_current_date() -> Date<Iso> {
    let current_offset_date_time = time::OffsetDateTime::now_local().unwrap();
    Date::try_new_iso_date(
        current_offset_date_time.year(),
        current_offset_date_time.month() as u8,
        current_offset_date_time.day(),
    )
    .unwrap()
}

// Put the following in the main() function:
let iso_date = get_current_date();

// Create and use an ICU4X date formatter:
let date_formatter =
    DateFormatter::try_new_with_length(&(&locale).into(), length::Date::Medium)
        .expect("should have data for specified locale");
println!(
    "Date: {}",
    date_formatter
        .format(&iso_date.to_any())
        .expect("date should format successfully")
);
```

Try this in several locales, like `en` (English), `en-GB` (British English), and `th` (Thai). Observe how differently dates are represented in locales around the world! You can explicitly specify arbitrary calendar systems using the `u-ca` Unicode extension keyword in the locale. Try `en-u-ca-hebrew`!

### JavaScript Part 3

In JavaScript, we will create a datetime input field.

Add this to the HTML:

```html
<!-- inside of div id="inputoutput" -->
<p><label>Date: <input type="date" id="dateinput"/></label></p>
```

And this to JavaScript:

```javascript

// Run the function whenever the date input changes:
document.getElementById("dateinput").addEventListener("input", update, false);

/// Helper function to convert the date input to an ICU4X Date:
function getDateFromInput() {
    let dateStr = document.getElementById("dateinput").value;
    let dateObj = dateStr ? new Date(dateStr) : new Date();
    return ICU4XIsoDate.create(
        dateObj.getYear() + 1900,
        dateObj.getMonth() + 1,
        dateObj.getDate(),
    );
}

// Put the following in the update() function, inside the try block:
let isoDate = getDateFromInput();
let dateFormatter = ICU4XDateFormatter.create_with_length(
    ICU4XDataProvider.create_compiled(), // we will learn what this means later
    locale,
    ICU4XDateLength.Medium,
);
document.getElementById("output").innerText = dateFormatter.format_iso_date(isoDate);
```

Try this in several locales, like `en` (English), `en-GB` (British English), and `th` (Thai). Observe how differently dates are represented in locales around the world! You can explicitly specify arbitrary calendar systems using the `u-ca` Unicode extension keyword in the locale. Try `en-u-ca-hebrew`!

## 4. Formatting date and time

Now we would also like to format the current time.

### Rust Part 4

Use the API documentation for [`icu::calendar::DateTime`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html) and [`icu::datetime::DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html) to expand your app to format both date and time.

Hint: You can use `Default::default()` for the `DateTimeFormatterOptions` argument.

### JavaScript Part 4

Use the API documentation for [`ICU4XDateTime`](https://unicode-org.github.io/icu4x/tsdoc/classes/ICU4XDateTime.html) and [`ICU4XDateTimeFormatter`](https://unicode-org.github.io/icu4x/tsdoc/classes/ICU4XDateTimeFormatter.html) to expand your app to format both a date and a time.

Hint: You can create an HTML time picker with

```html
<input type="time" id="timeinput" value="10:10"/>
```

Hint: You can create a `Date` from `dateStr` and `timeStr` with

```javascript
let dateObj = dateStr && timeStr ? new Date(dateStr + " " + timeStr) : new Date();
```
