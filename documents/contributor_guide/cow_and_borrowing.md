# Cow and borrowing

- **Prerequisites:** ownership, references, and lifetimes (chapters 4 and 10
  of [the Rust book](https://doc.rust-lang.org/book/)).
- **Related docs:**
  - [style_guide.md](../process/style_guide.md) has the rules:
    [Avoid implicit allocations](../process/style_guide.md#avoid-implicit-allocations--suggested),
    [Avoid `to_string`](../process/style_guide.md#avoid-to_string--suggested),
    [Conventions for strings in structs](../process/style_guide.md#conventions-for-strings-in-structs--suggested),
    and [Zero-copy in DataProvider structs](../process/style_guide.md#zero-copy-in-dataprovider-structs--required).
    This chapter explains why they exist.
  - [string_representation.md](../design/string_representation.md) covers
    text encodings and caller-allocated output at the API boundary. This
    chapter does not repeat it.
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`, 2026-10-06)

## Why this matters in ICU4X

ICU4X runs on servers and phones, in browsers (WebAssembly), and on small
devices. Some of its crates even work without a heap (see
`tools/noalloctest/README.md`). So ICU4X allocates memory only when it has
to.

Many operations often return their input unchanged:

- Normalizing `"hello"` to NFC gives `"hello"`.
- Lowercasing `"hello world"` gives `"hello world"`.
- A locale string that is already in canonical form stays the same.

A function that returns `String` allocates on every call, even when nothing
changed. A function that returns `Cow<'a, str>` can return the input
(`Cow::Borrowed`). It allocates only when the output is different
(`Cow::Owned`).

Locale data has the same problem. One data struct must work with data from
three sources:

| Data source | Where the bytes are | What the struct holds |
|---|---|---|
| Compiled data (baked into the binary) | Static memory | `&'static` references |
| Blob data (loaded at runtime) | A buffer that the provider owns | References into the buffer |
| Data built at runtime (for example by datagen) | The heap | Owned values |

With `Cow`, one struct type supports all three. This is why ICU4X data
structs have so many `Cow<'data, str>` fields.

## The concept (outside ICU4X)

`Cow<'a, B>` ("clone on write") is an enum from the `alloc` crate. `std`
re-exports it as `std::borrow::Cow`. Simplified:

```rust,ignore
pub enum Cow<'a, B: ?Sized + ToOwned> {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned), // for B = str, this is String
}
```

A function can return either variant. The caller reads both in the same way,
because `Cow` dereferences to `&B`:

```rust
use std::borrow::Cow;

/// Replaces "\r\n" with "\n". Allocates only if the input contains "\r\n".
fn unix_newlines(input: &str) -> Cow<'_, str> {
    if input.contains("\r\n") {
        Cow::Owned(input.replace("\r\n", "\n"))
    } else {
        Cow::Borrowed(input)
    }
}

let unchanged = unix_newlines("one\ntwo");
assert!(matches!(unchanged, Cow::Borrowed(_))); // no allocation

let changed = unix_newlines("one\r\ntwo");
assert!(matches!(changed, Cow::Owned(_)));
assert_eq!(changed, "one\ntwo");

// Need a String? `into_owned` allocates only if the value is borrowed.
let s: String = unchanged.into_owned();
assert_eq!(s, "one\ntwo");
```

A struct field can be a `Cow`, too. Then the same type can borrow or own:

```rust
use std::borrow::Cow;

struct Label<'data> {
    text: Cow<'data, str>,
}

// 1. Borrow from a string literal (like compiled data).
let compiled: Label<'static> = Label { text: Cow::Borrowed("Hello") };

// 2. Borrow from a buffer that someone else owns (like blob data).
let buffer = String::from("Bonjour");
let loaded: Label<'_> = Label { text: Cow::Borrowed(&buffer) };

// 3. Own the value (like data built at runtime). It is still `'static`.
let built: Label<'static> = Label { text: Cow::Owned(String::from("Hallo")) };

assert_eq!(compiled.text, "Hello");
assert_eq!(loaded.text, "Bonjour");
assert_eq!(built.text, "Hallo");
```

A `&'data str` field can do 1 and 2, but not 3. A `String` field can do 3,
but must copy in 1 and 2. Only `Cow` does all three.

Which type to use:

| Situation | Use |
|---|---|
| A parameter that is only read | `&str`, `&[T]`, `&T` |
| A parameter that the function stores | `String` or `T` by value, so the caller decides when to allocate |
| A result that is often the input, unchanged | `Cow<'a, str>` |
| A result that is new, formatted text | A type that implements `Writeable` (see below) |
| A string field in a struct | See [Conventions for strings in structs](../process/style_guide.md#conventions-for-strings-in-structs--suggested): `Cow<'data, str>` in data structs |

## In ICU4X

### 1. Return the input when nothing changes

`components/normalizer/src/lib.rs`, method `normalize` (inside the
`normalizer_methods!` macro):

```rust,ignore
pub fn normalize<'a>(&self, text: &'a str) -> Cow<'a, str> {
    let (head, tail) = self.split_normalized(text);
    if tail.is_empty() {
        return Cow::Borrowed(head);
    }
    let mut ret = String::new();
    ret.reserve(text.len());
    ret.push_str(head);
    let _ = self.normalize_to(tail, &mut ret);
    Cow::Owned(ret)
}
```

`split_normalized` finds the longest prefix that is already normalized. If
that is the whole text, the function borrows. If not, it allocates once and
normalizes only the rest.

Many other APIs get the same behavior from one helper:
`writeable::to_string_or_borrow` (`utils/writeable/src/to_string_or_borrow.rs`).
It compares the output with the input while it writes, and allocates only
when they start to differ. From `components/casemap/src/casemapper.rs`:

```rust,ignore
pub fn lowercase_to_string<'s>(
    self,
    src: &'s str,
    langid: &LanguageIdentifier,
) -> Cow<'s, str> {
    writeable::to_string_or_borrow(&self.lowercase(src, langid), src.as_bytes())
}
```

You can see both cases:

```rust
use icu::casemap::CaseMapper;
use icu::locale::{langid, Locale};
use std::borrow::Cow;

let cm = CaseMapper::new();
let root = langid!("und");

assert!(matches!(cm.lowercase_to_string("hello world", &root), Cow::Borrowed(_)));
assert!(matches!(cm.lowercase_to_string("Hello World", &root), Cow::Owned(_)));

// `Locale::normalize` uses the same helper.
assert!(matches!(Locale::normalize("pl-Latn-PL"), Ok(Cow::Borrowed(_))));
assert!(matches!(Locale::normalize("pL-latn-pl"), Ok(Cow::Owned(_))));
```

### 2. Return a `Writeable`, not a `String`

A formatter creates new text, so there is nothing to borrow. But the caller
may not need a `String` at all. It may write into an existing buffer, a
`fmt::Formatter`, or a sink from another language. So ICU4X formatters
return a value that implements `writeable::Writeable`, and the caller picks
the sink. This follows "Prefer working with caller-allocated memory" in
[string_representation.md](../design/string_representation.md).

From `components/decimal/src/decimal_formatter.rs`:

```rust,ignore
pub fn format<'l>(&'l self, value: &'l Decimal) -> FormattedDecimal<'l> {
    FormattedDecimal(self.format_sign(
        value.sign,
        self.format_unsigned(Cow::Borrowed(&value.absolute)),
    ))
}

#[cfg(feature = "alloc")]
pub fn format_to_string(&self, value: &Decimal) -> String {
    use writeable::Writeable;
    self.format(value).write_to_string().into_owned()
}
```

`format` only borrows the formatter and the input. It does not format
anything yet. `format_to_string` is a shortcut that is built on `format`,
and it needs the `alloc` feature. The FFI layer writes the same value
straight into a sink that the caller owns (`ffi/capi/src/decimal.rs`):

```rust,ignore
pub fn format(&self, value: &Decimal, write: &mut diplomat_runtime::DiplomatWrite) {
    let _infallible = self.0.format(&value.0).write_to(write);
}
```

`Writeable::write_to_string` returns a `Cow`, too. This is its default
implementation (`utils/writeable/src/lib.rs`):

```rust,ignore
fn write_to_string(&self) -> Cow<'_, str> {
    if let Some(borrow) = self.writeable_borrow() {
        return Cow::Borrowed(borrow);
    }
    let hint = self.writeable_length_hint();
    if hint.is_zero() {
        return Cow::Borrowed("");
    }
    let mut output = String::with_capacity(hint.capacity());
    let _ = self.write_to(&mut output);
    Cow::Owned(output)
}
```

It borrows when it can. If not, it allocates once, and uses the length hint
as the capacity. This is why a correct `writeable_length_hint` matters.

```rust
use icu::decimal::input::Decimal;
use icu::decimal::DecimalFormatter;
use icu::locale::locale;
use writeable::assert_writeable_eq;

let formatter = DecimalFormatter::try_new(locale!("en").into(), Default::default())
    .expect("compiled data includes en");
let value = Decimal::from(1234567);

// `assert_writeable_eq!` checks the text, and also the length hint.
assert_writeable_eq!(formatter.format(&value), "1,234,567");
```

### 3. Data structs: `Cow<'data, str>` with `#[serde(borrow)]`

`components/datetime/src/provider/time_zones.rs` (abridged):

```rust,ignore
#[derive(PartialEq, Debug, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct TimeZoneEssentials<'data> {
    pub offset_separator: Cow<'data, str>,
    pub offset_pattern: Cow<'data, SinglePlaceholderPattern>,
    pub offset_unknown: Cow<'data, str>,
}

// Inside the hand-written `Deserialize` impl:
#[derive(serde::Deserialize)]
struct Raw<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    offset_separator: Cow<'data, str>,
    // ...
}
```

- Compiled data creates this struct with `Cow::Borrowed` and `&'static`
  strings.
- Blob data deserializes it. With `#[serde(borrow)]`, each `Cow` points into
  the blob buffer.
- Datagen builds it from CLDR with `Cow::Owned`.

Without `#[serde(borrow)]`, serde always creates `Cow::Owned`. The code still
works, but every load allocates. CI catches this: the zero-copy check
(`ZeroCopyCheckExporter` in `tools/make/bakeddata/src/main.rs`) deserializes
every payload and counts the allocations. When it fails, it says:

```text
Common cause: did you forget to add `serde(borrow)` to all of the fields in your data struct?
```

Why is `Deserialize` hand-written here? To keep old and new data compatible
after [#8250](https://github.com/unicode-org/icu4x/pull/8250) removed a field.
See [AV005](avoid/AV005_stable_serialized_layout.md).

Some newer data structs use `VarZeroCow<'data, str>` instead (for example
`ListFormatterPatterns` in `components/list/src/provider/mod.rs`).
TODO(verify): which of the two new data structs should use.

### 4. Borrowed and owned types: `CaseMapperBorrowed` and `CaseMapper`

The same idea works for whole types. From
`components/casemap/src/casemapper.rs` (abridged):

```rust,ignore
pub struct CaseMapper {
    pub(crate) data: DataPayload<CaseMapV1>,
}

#[derive(Clone, Debug, Copy)]
pub struct CaseMapperBorrowed<'a> {
    pub(crate) data: &'a CaseMap<'a>,
}

impl CaseMapper {
    pub const fn new() -> CaseMapperBorrowed<'static> { /* compiled data */ }
    pub fn as_borrowed(&self) -> CaseMapperBorrowed<'_> { /* ... */ }
}

impl CaseMapperBorrowed<'static> {
    pub const fn static_to_owned(self) -> CaseMapper { /* ... */ }
}
```

- `CaseMapperBorrowed<'a>` holds a plain reference. It is `Copy`. With
  compiled data, you can even create it in a `const`.
- `CaseMapper` owns a `DataPayload`. You need it when the data comes from a
  provider at runtime (`try_new_unstable`, `try_new_with_buffer_provider`).
  `DataPayload` works like a `Cow` for data structs: it holds either a
  `&'static` struct (`DataPayload::from_static_ref`) or data that it owns.
- [graduation.md](../process/graduation.md) requires this shape: if there is
  a borrowed type, `Foo::new()` returns `FooBorrowed`
  ([#5440](https://github.com/unicode-org/icu4x/issues/5440)).

```rust
use icu::casemap::{CaseMapper, CaseMapperBorrowed};
use icu::locale::langid;

// Compiled data: no allocation, and it works in a `const`.
const CM: CaseMapperBorrowed<'static> = CaseMapper::new();
assert_eq!(CM.uppercase_to_string("Straße", &langid!("und")), "STRASSE");

// An owned value, for code that has to store a `CaseMapper`:
let owned: CaseMapper = CM.static_to_owned();
assert_eq!(owned.as_borrowed().lowercase_to_string("ABC", &langid!("und")), "abc");
```

### 5. `Cow` needs `alloc`

`Cow` comes from the `alloc` crate. ICU4X library crates are `no_std`
([AV003](avoid/AV003_no_std_in_library_crates.md)), so write
`alloc::borrow::Cow`, not `std::borrow::Cow`.

A crate that must also work without `alloc` can't use `Cow` at all. For
example, `components/decimal/src/lib.rs` defines its own small `Cow` enum
when the `alloc` feature is off. This is rare. If you think you need it, ask
in your issue first.

## Common mistakes

| Mistake | Do instead | Rule |
|---|---|---|
| Return `String` when the result is often the input | Return `Cow<'a, str>` | [AV002](avoid/AV002_no_needless_allocation.md) |
| Call `format!` or `to_string()` in library code | Return a type that implements `Writeable` | [AV002](avoid/AV002_no_needless_allocation.md) |
| Take `&T`, then clone it inside | Take `T` by value | [AV002](avoid/AV002_no_needless_allocation.md) |
| Call `.into_owned()` "just in case" | Keep the `Cow` until you really need a `String` | [AV002](avoid/AV002_no_needless_allocation.md) |
| Use `String`, `Vec`, or `&'data str` in a data struct | Use `Cow<'data, str>` or a `zerovec` type | [AV004](avoid/AV004_zero_copy_data_structs.md) |
| Forget `#[serde(borrow)]` on a `Cow` field | Add `#[cfg_attr(feature = "serde", serde(borrow))]` | [AV004](avoid/AV004_zero_copy_data_structs.md) |
| Use `std::borrow::Cow` in a library crate | Use `alloc::borrow::Cow` | [AV003](avoid/AV003_no_std_in_library_crates.md) |

## Exercises

### Exercise 1: Borrow when you can

a) Write `typographic_apostrophes`. It replaces `'` with `’`
(U+2019 RIGHT SINGLE QUOTATION MARK). It must not allocate when the input
has no `'`.

```rust,ignore
fn typographic_apostrophes(input: &str) -> Cow<'_, str> {
    todo!()
}

assert!(matches!(typographic_apostrophes("hello"), Cow::Borrowed("hello")));
assert_eq!(typographic_apostrophes("don't"), "don’t");
```

b) This code does not compile. Why not? How do you fix it?

```rust,compile_fail
use std::borrow::Cow;

fn shout(input: &str) -> Cow<'_, str> {
    let upper = input.to_uppercase();
    Cow::Borrowed(&upper)
}
```

<details>
<summary>Solution</summary>

a)

```rust
use std::borrow::Cow;

fn typographic_apostrophes(input: &str) -> Cow<'_, str> {
    if input.contains('\'') {
        Cow::Owned(input.replace('\'', "’"))
    } else {
        Cow::Borrowed(input)
    }
}

assert!(matches!(typographic_apostrophes("hello"), Cow::Borrowed("hello")));
assert_eq!(typographic_apostrophes("don't"), "don’t");
```

b) `upper` is a local `String`. It is dropped when `shout` returns, so a
reference to it can't leave the function (error E0515). Return the owned
value instead: `Cow::Owned(upper)`. A real implementation would also return
`Cow::Borrowed(input)` when the text is already uppercase, like
`CaseMapperBorrowed::uppercase_to_string` does.

</details>

### Exercise 2: Find the allocation

This made-up data struct compiles and works. But each time it is loaded from
blob data, it allocates. Why?

```rust,ignore
#[derive(serde::Deserialize)]
pub struct CityNames<'data> {
    pub capital: Cow<'data, str>,
    pub largest: Cow<'data, str>,
}
```

<details>
<summary>Solution</summary>

The fields have no `#[serde(borrow)]`. Without it, serde deserializes every
`Cow` as `Cow::Owned`, which copies the string to the heap. In ICU4X, add
`#[cfg_attr(feature = "serde", serde(borrow))]` to each field (serde is an
optional dependency). The CI zero-copy check reports this bug (see section 3).
Note that `#[serde(borrow)]` has known problems with `Option` fields
([serde#2016](https://github.com/serde-rs/serde/issues/2016)).

You can see the difference without a data file. serde's
`BorrowedStrDeserializer` acts like a format that lends out its strings:

```rust
use serde::de::value::{BorrowedStrDeserializer, Error, SeqDeserializer};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

#[derive(Deserialize)]
struct WithBorrow<'data> {
    #[serde(borrow)]
    name: Cow<'data, str>,
}

#[derive(Deserialize)]
struct WithoutBorrow<'data> {
    name: Cow<'data, str>,
}

fn input() -> impl Deserializer<'static, Error = Error> {
    SeqDeserializer::new([BorrowedStrDeserializer::new("Oslo")].into_iter())
}

let with = WithBorrow::deserialize(input()).expect("valid input");
let without = WithoutBorrow::deserialize(input()).expect("valid input");
assert!(matches!(with.name, Cow::Borrowed("Oslo")));
assert!(matches!(without.name, Cow::Owned(_)));
```

</details>

### Exercise 3: Return a `Writeable`

This function allocates on every call:

```rust
fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

assert_eq!(greeting("Ada"), "Hello, Ada!");
```

Write a type `Greeting<'a>` that implements `writeable::Writeable`, with an
exact `writeable_length_hint`. Give it `Display` with
`writeable::impl_display_with_writeable!`. Test it with
`writeable::assert_writeable_eq!`.

<details>
<summary>Solution</summary>

```rust
use core::fmt;
use writeable::{LengthHint, Writeable};

struct Greeting<'a> {
    name: &'a str,
}

impl Writeable for Greeting<'_> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')
    }

    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact("Hello, !".len()) + self.name.len()
    }
}

writeable::impl_display_with_writeable!(Greeting<'_>);

writeable::assert_writeable_eq!(Greeting { name: "Ada" }, "Hello, Ada!");
```

Now the caller chooses: write into an existing sink with no new allocation,
or call `write_to_string()` and get one allocation of the right size.

</details>

## Checklist before your PR

- [ ] Parameters that are only read are references (`&str`, `&T`).
      Parameters that are stored are taken by value.
- [ ] Functions that often return their input unchanged return `Cow`. Tests
      cover both `Cow::Borrowed` and `Cow::Owned`.
- [ ] Formatting APIs return a `Writeable` type. Any `*_to_string` helper is
      built on it.
- [ ] Every `Writeable` has a correct `writeable_length_hint`, tested with
      `assert_writeable_eq!`.
- [ ] Strings in data structs are `Cow<'data, str>` or `zerovec` types, each
      with `#[cfg_attr(feature = "serde", serde(borrow))]`.
- [ ] If you changed a data struct, `cargo make bakeddata <component>` passes.
      It runs the zero-copy check.
- [ ] Library code has no `std::` paths. Use `core::` and `alloc::`.
