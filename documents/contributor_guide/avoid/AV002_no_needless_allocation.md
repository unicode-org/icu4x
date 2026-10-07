# AV002: Don't allocate when you can borrow

- **Status:** Proposed
- **Enforced by:** review only
- **Applies to:** library code, especially public APIs. Not test code or
  datagen.
- **Source:** [style_guide.md: Avoid implicit allocations](../../process/style_guide.md#avoid-implicit-allocations--suggested),
  [Avoid `to_string`](../../process/style_guide.md#avoid-to_string--suggested),
  [string_representation.md: Prefer working with caller-allocated memory](../../design/string_representation.md#prefer-working-with-caller-allocated-memory)
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`)

## ❌ Don't

```rust,ignore
// Allocates even when `src` is already lowercase.
pub fn lowercase(&self, src: &str) -> String { /* ... */ }

// Every caller gets a new `String`, even if it has a buffer to write into.
pub fn format(&self, value: &Decimal) -> String { /* ... */ }

// Takes a reference, then clones it.
pub fn try_new(locale: &Locale) -> Result<Self, Error> {
    Ok(Self { locale: locale.clone() })
}
```

## ✅ Do instead

```rust,ignore
// Returns `Cow::Borrowed(src)` when nothing changes.
pub fn lowercase_to_string<'s>(self, src: &'s str, langid: &LanguageIdentifier) -> Cow<'s, str>

// `FormattedDecimal` implements `Writeable`. The caller picks the sink.
pub fn format<'l>(&'l self, value: &'l Decimal) -> FormattedDecimal<'l>

// Takes the value, so the caller can move it in.
pub fn try_new(locale: Locale) -> Result<Self, Error>

// Need a `String` from a `&str`? Use `to_owned`, not `to_string`.
let owned: String = src.to_owned();
```

Real examples: `CaseMapperBorrowed` in `components/casemap/src/casemapper.rs`
and `DecimalFormatter` in `components/decimal/src/decimal_formatter.rs`.
[cow_and_borrowing.md](../cow_and_borrowing.md) explains these patterns.

## Why

- Many operations often return their input unchanged: normalizing text that
  is already normalized, or lowercasing lowercase text. A `String` result
  allocates even then. A `Cow` result borrows the input, and allocates only
  when the output is different.
- The caller of a formatter may not need a `String`. It may write into a
  buffer that it already has, a `fmt::Formatter`, or an FFI sink
  (`DiplomatWrite`). A `Writeable` lets the caller choose, and
  `writeable_length_hint` lets it allocate once, with the right size.
- `to_string` "delegates to the `Display` trait which is bad for code size as
  it pulls in a lot of formatting code" (style_guide.md).
- An API that takes `&T` and clones it hides an allocation. The style guide:
  "the public API should not take a reference, if the value is going to be
  cloned internally".

## Exceptions

- An API that needs to allocate only in rare cases may take a reference, if
  the API makes this explicit. The style guide gives `HashSet::get_or_insert`
  and `get_or_insert_owned` as the example.
- `*_to_string` shortcuts are fine when they are built on the `Cow` or
  `Writeable` API, like `DecimalFormatter::format_to_string`.
- Tests and docs may allocate. When they compare results, the style guide
  says to "prefer semantic equality over stringified equality".

## Automation

- **Today:** review only.
- **Idea:** Clippy's `str_to_string` lint (in the `restriction` group) flags
  `to_string()` on a `&str`. ICU4X does not enable it. TODO(verify): whether
  it should.
