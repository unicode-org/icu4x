# Building an Interactive DateTime Picker with ICU4X

In this tutorial, you will learn how to build an end-to-end application using ICU4X to format a date and time with some default locales and additional locales loaded dynamically.

This tutorial is written in parallel between **Rust** and **JavaScript** in a web browser.

## 1. Installing ICU4X

Installing dependencies is always your first step.

### Rust

Please create a new Rust crate with icu4x as a dependency. For instructions, please follow steps 1-2 in [intro.md](./intro.md).

Some of this tutorial overlaps with steps 3-5 in the intro tutorial.

### JavaScript

We recommend using [CodePen](https://codepen.io/pen/?editors=1011) to follow along. To load ICU4X into CodePen, you can use this snippet in the JavaScript editor:

```javascript
import { ICU4XLocale, ICU4XDataProvider } from "https://storage.googleapis.com/static-493776/icu4x_2023-11-03/js/index.js";
```

## 2. Creating a Locale Input

Here, we will accept a locale string from the user and parse it into an ICU4X Locale.

### Rust

First, we will use Rust APIs to accept a string from user input on the command line. Add the following to your `fn main()`:

```rust,no_run
// Get the locale from user input:
let mut locale_str = String::new();
print!("Enter your locale: ");
std::io::Write::flush(&mut std::io::stdout()).unwrap();
std::io::stdin().read_line(&mut locale_str).unwrap();
```
