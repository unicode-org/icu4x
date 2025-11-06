# Interactive Date Picker

In this tutorial, you will learn how to build an end-to-end application using ICU4X to format a date and time with some default locales and additional locales loaded dynamically.

This tutorial is written in parallel between **Rust** and **JavaScript** in a web browser.

## 1. Installing ICU4X

Installing dependencies is always your first step.

### Rust Part 1

Verify that Rust is installed. If it's not, you can install it in a few seconds from [https://rustup.rs/](https://rustup.rs/).

```console
cargo --version
# cargo 1.86.0 (adf9b6ad1 2025-02-28)
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
import { Locale, DateFormatter, IsoDate, DateTimeLength } from "https://unpkg.com/icu@2.0.0";
```

This loads the full ICU4X WebAssembly file. Since it may take a few seconds to load on slow connections, we'll create a loading div. Add this to your HTML:

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
use icu::locale::Locale;

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
cargo run
Enter your locale: DE-CH
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

        let locale = Locale.fromString(localeStr);
        let output = locale.toString();

        document.getElementById("output").innerText = output;
    } catch(e) {
        document.getElementById("output").innerText = e;
    }
}

// Run the function whenever the locale input changes:
document.getElementById("localeinput").addEventListener("keyup", update, false);

// Also run the function right now to initialize the UI:
update();
```

Try inputting locales in non-canonical syntax and see them normalized!

> Locale: ES-419
> Output: es-419

## 3. Formatting a Date

Now we will use built-in locale data to produce a formatted date.

### Rust Part 3

We would like to format today's date. We will get this from the `time` crate, which you need to add:

```console
cargo add time --features local-offset
```

Now we can write the Rust code:

```rust
// At the top of the file:
use icu::datetime::{DateTimeFormatter, fieldsets::YMD, input::Date};

let locale = icu::locale::Locale::UNKNOWN; // to make this example compile

// Put the following in the main() function:
let iso_date = {
    let current_offset_date_time = time::OffsetDateTime::now_local().unwrap();
    Date::try_new_iso(
        current_offset_date_time.year(),
        current_offset_date_time.month() as u8,
        current_offset_date_time.day(),
    )
    .unwrap()
};

// Create and use an ICU4X date formatter:
let date_formatter = DateTimeFormatter::try_new(
    locale.into(),
    YMD::medium(),
)
.expect("should have data for specified locale");
println!(
    "Date: {}",
    date_formatter.format(&iso_date)
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

// Put the following in the update() function, inside the try block:
let dateStr = document.getElementById("dateinput").value;

let dateObj = dateStr ? new Date(dateStr) : new Date();
let isoDate = new IsoDate(dateObj.getFullYear(), dateObj.getMonth() + 1, dateObj.getDate());
let dateFormatter = DateFormatter.createYmd(locale, DateTimeLength.Long);
let output = dateFormatter.formatIso(isoDate);

document.getElementById("output").innerText = output;
```

Try this in several locales, like `en` (English), `en-GB` (British English), and `th` (Thai). Observe how differently dates are represented in locales around the world! You can explicitly specify arbitrary calendar systems using the `u-ca` Unicode extension keyword in the locale. Try `en-u-ca-hebrew`!

## 4. Formatting date and time

Now we would also like to format the current time.

### Rust Part 4

Use the API documentation for [`icu::time::DateTime`](https://docs.rs/icu/latest/icu/time/struct.DateTime.html) and [`icu::datetime::fieldsets`](https://docs.rs/icu/latest/icu/datetime/fieldsets/index.html) to expand your app to format both date and time.

### JavaScript Part 4

Use the API documentation for [`Time`](https://icu4x.unicode.org/2_0/tsdoc/classes/Time.html) and [`DateTimeFormatter`](https://icu4x.unicode.org/2_0/tsdoc/classes/DateTimeFormatter.html) to expand your app to format both a date and a time.

Hint: You can create an HTML time picker with

```html
<p><label>Time: <input type="time" id="timeinput" value="10:10"/></label></p>
```

Hint: You can create a `Date` from `dateStr` and `timeStr` with

```javascript
let dateObj = dateStr && timeStr ? new Date(dateStr + " " + timeStr) : new Date();
```

Note that Dates constructed this way will be in UTC.