# Introduction to ICU4X

`ICU4X` is an implementation of [Internationalization Components of Unicode](https://icu.unicode.org/) (ICU) intended to be modular, performant and flexible.

The library provides a layer of APIs for all software to enable internationalization capabilities.

To use `ICU4X` in the Rust ecosystem one can either add dependencies on selected components, or add a dependency on the meta-crate `icu` which brings the full selection of components in the most user-friendly configuration of features.

In this tutorial we are going to build up to writing an app that uses the `icu::datetime` component to format a date and time, covering various topics in the process.

This tutorial is written in parallel between **Rust** and **JavaScript** in a web browser.

## 1. Requirements

<details>
<summary>Rust</summary>
For this tutorial we assume the user has basic Rust knowledge. If acquiring it is necessary, the [Rust Book](https://doc.rust-lang.org/book/) provides an excellent introduction.
We also assume that the user is familiar with a terminal and have `rust` and `cargo` installed.

To verify that, open a terminal and check that the results are similar to:

```console
cargo --version
# cargo 1.86.0 (adf9b6ad1 2025-02-28)
```
</details>

<details>
<summary>JavaScript</summary>

For this tutorial we assume the user has basic JavaScript knowledge. We recommend using [CodePen](https://codepen.io/pen/?editors=1011) to follow along. 
</details>

## 2. Creating an app with ICU4X as a dependency

<details>
<summary>Rust</summary>

Use `cargo` to initialize a binary application:

```console
cargo new --bin tutorial
cd tutorial
```

Then add a dependency on `ICU4X`'s main crate, `icu`:

```console
cargo add icu
```

Run your application with `cargo run`:

```shell
cargo run
# Hello, world!
```

*Notice:* By default, `cargo run` builds and runs a `debug` mode of the binary. If you want to evaluate performance, memory or binary size, use `cargo run --release`.

</details>

<details>
<summary>JavaScript</summary>

To load ICU4X into CodePen, you can use this snippet in the JavaScript editor:

```javascript
import { Locale, DateFormatter, IsoDate, DateTimeLength } from "https://unpkg.com/icu@2.0.0";
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
</details>

## 3. Locales

`ICU4X` comes with a variety of components allowing to manage various facets of software internationalization.

Most of those features depend on the selection of a `Locale` which is a particular combination of language, script, region with optional variants. An examples of such locales are `en-US` (American English), `sr-Cyrl` (Serbian with Cyrillic script) or `ar-EG-u-nu-latn` (Egyptian Arabic with ASCII numerals).

In `ICU4X` `Locale` is a part of the `locale` component[^1]. Let's use this in our application.

[^1]:  If the user needs just this one feature, they can use `icu_locale_core` crate as a dependency, but since here we already added a dependency on `icu`, we can refer to it via `icu::locale`.

<details>
<summary>Rust</summary>

Open `src/main.rs` and add the following code inside `fn main`:

```rust,no_run
use icu::locale::Locale;

// Pass a locale string on the command line
let locale_str = std::env::args().nth(1).unwrap();

// Since the string contains whitespace, we must call `.trim()`:
let locale = locale_str.trim().parse::<Locale>().unwrap();

if locale.id.language.as_str() == "es" {
    println!("¡Hola!");
}

println!("Your locale: {locale}");
```

After saving it, call `cargo run` and it should display:

```shell
cargo run -- DE-CH
# Your locale: de-CH

cargo run -- es-419
# ¡Hola!
# Your locale: es-419
```

### Convenience macro

The scenario of working with statically declared `Locale`s (and subtags) is common. It's a bit unergonomic to have to parse them at runtime and handle a parser error (or to degrade to string operations), so ICU4X provides macros one can use to parse at compilation time:

```rust,ignore
use icu::locale::subtags::language;

if locale.id.language == language!("es") {
    println!("¡Hola!");
}
```

Try using a malformed string, like "spanish" and call `cargo check`.
</details>

<details>
<summary>JavaScript</summary>

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
        if locale.language == "es" {
            console.log("¡Hola!");
        }
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
>
> Output: es-419

</details>

*Notice:* Here, `ICU4X` canonicalized the locales's syntax which uses lowercase letters for the language portion.

Congratulations! `ICU4X` has been used to semantically operate on a locale!

## 4. Using an ICU4X component

We're going to extend our app to use the `icu::datetime` component to format a date. This component requires data; we will look at custom data generation later and for now use the default included data,
which is exposed through standard constructors  (`try_new` in Rust, `create` in JavaScript).

<details>
<summary>Rust</summary>

We will get the current date from the `time` crate, which you need to add

```console
cargo add time --features local-offset
```

Now add the following code to `fn main`:

```rust,ignore
use icu::datetime::{DateTimeFormatter, fieldsets::YMD, input::Date};

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
    "📅: {}",
    date_formatter.format(&iso_date)
);
```

If all went well, running the app with `cargo run` should display:

```console
cargo run -- de-CH
Your locale: de-CH
📅: 15.05.2025
```
</details>

<details>
<summary>JavaScript</summary>

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
</details>

Try this in several locales, like `en` (English), `en-GB` (British English), and `th` (Thai). Observe how differently dates are represented in locales around the world! You can explicitly specify arbitrary calendar systems using the `u-ca` Unicode extension keyword in the locale. Try `en-u-ca-hebrew`!

## 5. Formatting date and time

Now we would also like to format the current time.

<details>
<summary>Rust</summary>

Use the API documentation for [`icu::time::DateTime`](https://docs.rs/icu/latest/icu/time/struct.DateTime.html) and [`icu::datetime::fieldsets`](https://docs.rs/icu/latest/icu/datetime/fieldsets/index.html) to expand your app to format both date and time.

</details>

<details>
<summary>JavaScript</summary>

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

</details>

## 6. Data Management

While the locale API is purely algorithmic, many internationalization APIs like the date formatting API require more complex data to work. You've seen this in the previous example where we had to call `.expect("should have data for specified locale")` after the constructor.

Data management is a complex and non-trivial area which often requires customizations for particular environments and integrations into a project's ecosystem.

The way `ICU4X` handles data is one of its novelties, aimed at making the data management more flexible and enabling better integration in asynchronous environments.

`ICU4X` by default contains data for a a wide range of CLDR locales[^1], meaning that for most languages, the constructors can be considered infallible and you can `expect` or `unwrap` them, as we did above.

However, shipping the library with all locales will have a size impact on your binary. It also requires you to update your binary whenever CLDR data changes, which happens twice a year. To learn how to solve these problems, see our [data packs](data-packs.md) and [data slimming](data-slimming.md) tutorials.

[^1]: All locales with coverage level `basic`, `moderate`, or `modern` in [`CLDR`](https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/coverageLevels.json)

## 7. Summary

This concludes this introduction tutorial. With the help of `Locale` and `DateTimeFormatter` we formatted a date to Japanese, but that's just the start. 

Internationalization is a broad domain and there are many more components in `ICU4X`.

Next, learn how to [generate optimized data for your binary](data-slimming.md), [create language packs](data-packs.md), [configure your Cargo.toml file](../examples/cargo), or continue exploring by reading [the docs](https://docs.rs/icu/latest/).



