# Building an Interactive DateTime Picker with ICU4X

In this tutorial, you will learn how to build an end-to-end application using ICU4X to format a date and time with some default locales and additional locales loaded dynamically.

This tutorial is written in parallel between **Rust** and **JavaScript** in a web browser.

## 1. Installing ICU4X

Installing dependencies is always your first step.

### Rust Part 1

Please create a new Rust crate with icu4x as a dependency. For instructions, please follow steps 1-2 in [intro.md](./intro.md).

Some of this tutorial overlaps with steps 3-5 in the intro tutorial.

### JavaScript Part 1

We recommend using [CodePen](https://codepen.io/pen/?editors=1011) to follow along. To load ICU4X into CodePen, you can use this snippet in the JavaScript editor:

```javascript
import { ICU4XLocale, ICU4XDataProvider } from "https://storage.googleapis.com/static-493776/icu4x_2023-11-03/js/index.js";
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
// At top of file:
use icu::locid::Locale;

// In the main() function:
let mut locale_str = String::new();
print!("Enter your locale: ");
std::io::Write::flush(&mut std::io::stdout()).unwrap();
std::io::stdin().read_line(&mut locale_str).unwrap();

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
    let textResult;
    try {
        let localeStr = document.getElementById("localeinput").value;
        let locale = ICU4XLocale.create_from_string(localeStr);
        textResult = locale.to_string();
    } catch(e) {
        textResult = e + ": " + e.error_value;
    }
    document.getElementById("output").innerText = textResult;
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
// At top of file:
use icu::calendar::DateTime;
use icu::datetime::options::length;
use icu::datetime::DateFormatter;

/// Helper function to create an ICU4X DateTime for the current local time:
fn get_current_datetime() -> DateTime<icu::calendar::Iso> {
    let current_offset_date_time = time::OffsetDateTime::now_local().unwrap();
    DateTime::try_new_iso_datetime(
        current_offset_date_time.year(),
        current_offset_date_time.month() as u8,
        current_offset_date_time.day(),
        current_offset_date_time.hour(),
        current_offset_date_time.minute(),
        current_offset_date_time.second(),
    )
    .unwrap()
}

let locale = icu::locid::Locale::default(); // so the example compiles; don't include this

// Put the following in the main() function:
let icu4x_datetime = get_current_datetime();

// Create and use an ICU4X date formatter:
let date_formatter =
    DateFormatter::try_new_with_length(&(&locale).into(), length::Date::Medium)
        .expect("should have data for specified locale");
println!(
    "Date: {}",
    date_formatter
        .format(&icu4x_datetime.to_any())
        .expect("date should format successfully")
);
```

### JavaScript Part 3

In JavaScript, we will create a datetime input field.

Add this to the HTML:

```html
<!-- inside of div id="inputoutput" -->
<p><label>Date: <input type="date" id="dateinput"/></label></p>
```

And to JavaScript:

```javascript
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
let isoDateTime = getDateFromInput();
let dataProvider = ICU4XDataProvider.create_compiled();
let dateFormatter = ICU4XDateFormatter.create_with_length(
    dataProvider,
    locale,
    ICU4XDateLength.Medium,
);
textResult += ": " + dateFormatter.format_iso_date(isoDateTime);
```
