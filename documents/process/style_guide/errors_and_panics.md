Errors and Panics
=================

This page is part of the [ICU4X Style Guide](README.md). Rules are **required** or **suggested**; see the [Preamble](README.md#preamble).

## Error Handling

See also the [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) chapter in the Rust Book.

The ICU4X library should be designed so as to never fail at runtime. Now obviously that's something you'd expect all library writers to say, but in Rust you can control the places where code can fail explicitly, so it's much easier to write self-documenting code where the assumptions around failure are obvious.

Most core rust APIs (traits) have two ways to access data, a version that can "panic" , and one which returns a [Result](https://doc.rust-lang.org/std/result/index.html) or an [Option](https://doc.rust-lang.org/std/option/enum.Option.html). Rust is a language which likes to avoid "unnecessary" overhead, and in some cases it is perfectly correct to use an API which can "panic" because you have already checked the arguments carefully (and performance will be better).

Note that in cases where the Rust compiler can statically determine that a check is sufficient to avoid panic, it will remove the internal check and panic related code, leaving just a provably safe data access.

### Where Result is needed, use IcuResult<T> :: required

While it's still an open question in the Rust community as to what the best way to handle error is, the current ICU4X consensus is that we should start simple and expect to revisit this topic again at some point. The simplest reasonable starting point would be to have a `IcuResult<T>`, which is type as `Result<T, IcuError>`, where:

```rust
// Nesting semantically interesting error information inside the generic error type.
enum IcuError {
    Parser(parser::ParseError),
    Runtime(...)
}
```

A couple of crates by `@dtolnay` and `@yaahc` that are considered "new wave of good error APIs" and are complementary to each other:

* https://github.com/dtolnay/thiserror
* https://docs.rs/eyre/0.6.5/eyre/

Other links on error handling:

* https://blog.yoshuawuyts.com/error-handling-survey/
* https://sled.rs/errors
* https://boats.gitlab.io/blog/post/failure-to-fehler/
* https://boats.gitlab.io/blog/post/why-ok-wrapping/
* https://vorner.github.io/2020/04/09/wrapping-mental-models.html
* https://yaah.dev/try-blocks

## Panicking APIs

The most common example of an API which can panic is access to slices and elements of a slice. This includes accessing `array`, `str`, `Vec`, `HashMap` etc. via the `[`,`]` (square bracket) operator, implemented by the [Index](https://doc.rust-lang.org/std/ops/trait.Index.html) trait.

Thus statements like:

```rust
let x = self.data[n];
```

are always prone to "panic" if the index/key is incorrect. (⚠️ and this includes accessing maps ⚠️).

This is because these APIs return a value (or value reference) which cannot be "null", so there is no way for them to signal failure via the return type.

## Non-Panicking APIs

The alternative to using direct data accessors which can panic is to use a method which can return **Option** or **Result**. In the case of collections and strings, where a simple data item is being requested, this is most often provided by functions such as "get" (or "get_mut" for mutable references) which return `Option`.

If data access is expected to fail occasionally (e.g. looking up properties in a map) then the resulting [Option can be unwrapped](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) or propagated accordingly.

If missing data signals a "hard" error from which the function cannot recover (e.g. user supplies incorrect input) then any returned `Option` should be [propagated into a `Result` immediately](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or), with an appropriate error value.

### Don't Panic :: required

Call non-panicking data access APIs whenever data is not guaranteed to be safe.

This should not include the contract of code in a different Crate. I.e. if a function in a different Crate promises to return a valid map key, but it's not a compile time checked type (like an enum), then the calling code must allow for it to fail.

See also: the [Panics](README.md#panics--required) section of the style guide.

**Why:** A panic in a library stops the whole program that uses it. ICU4X also loads data that it did not create, and [data_safety.md](../../design/data_safety.md) says that code should never panic at runtime based on invalid data. Otherwise, bad data can crash the application.

**❌ Don't:**

```rust
let (key, value) = input.split_once('=').unwrap(); // panics if there is no '='
let first = names[0];                              // panics if `names` is empty
```

**✅ Do:** If the caller can do something about the problem, return an error:

```rust
pub fn parse_pair(input: &str) -> Result<(&str, &str), ParseError> {
    input.split_once('=').ok_or(ParseError::MissingEquals)
}
```

If the problem can only come from invalid data or an internal bug, use a fallback value and a debug assertion. Tests and debug builds find the bug, and release builds continue with the fallback ("garbage in, garbage out"). From `split_normalized` in `components/normalizer/src/lib.rs`:

```rust
text.split_at_checked(up_to).unwrap_or_else(|| {
    // Internal bug, not even GIGO, never supposed to happen
    debug_assert!(false);
    ("", text)
})
```

#### Special Case: `split_at`

The standard library functions such as `slice::split_at` are panicky, but they are not covered by our Clippy lints. Be careful to use `slice::split_at_checked` instead.

#### Exception: Poison

The `lock`, `read`, and `write` methods on `Mutex` and `RwLock` return `Result`s in case the lock got poisoned. This happens when the process holding the lock panics, so it is fine to panic when encountering a poison. For consistency we require poisons to be handled with `.expect("poison")`.

### Avoid `to_string` :: suggested

`to_string` delegates to the `Display` trait which is bad for code size as it pulls in a lot of formatting code.

There are three types on which the standard library has *specialized implementations* for `to_string` that do not go through formatting logic: `&str`, `String`, and `char`. It would generally be acceptable to use `to_string` on these types, however types might not always be obvious to a reader, and implicit types might change with surrounding code (such as from `&str` to `&&str`, which is *not specialized*), so use `str::to_owned`, `String::clone`, and `String::from(char)` for clarity.

Any other invocation of `to_string` should be carefully evaluated. It is usually better to work with values that implement `Display` and/or `Writeable` than to eagerly format and allocate strings.

When comparing items in unit tests or docs, prefer semantic equality over stringified equality.

### Don't Handle Errors :: suggested

Functions which can error for any reason must return a `Result`, and APIs should be designed such that you should not generally need to recover from an [Err](https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err) internally (which should normally be immediately propagated up to the user by using the [`?` operator](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html)). I.e. don't generally write library code which recovers from its own "errors", since if it can be recovered from, then it wasn't an "error".

This approach should mean that error handling and the design of functions which can propagate errors is consistent everywhere. For non-error cases, where different types of result are possible, use a normal enum.

Since an `Err` in `Result` is more expressive than a `None` in `Option`, there may be cases in which it is appropriate to handle a recoverable error. For example, you may call a function with one set of inputs, and if that call fails, you attempt to call it with a second set of inputs, before propagating the error.

Finally, and fairly obviously, **never turn an error into a panic by unconditionally unwrapping the result in the library**.

### Comment Use of Panicking Calls :: required

Use panicking methods only when the input has been explicitly checked to be correct.

```rust
// Attribute keys are checked for validity during data loading by ...
let x = self.attribute_map[char_attribute.key];
```

If this check does not occur immediately before the data access (i.e. shortly before in the same function), comment clearly where it does occur.

For example, if indices obtained from ICU data are to be trusted for indexed access, the data itself must have been validated at some earlier time (e.g. via a checking pass during data loading or use of a trusted hash).

However, you should **never add a check purely in order to call a method which could otherwise panic**; in that situation you should always prefer to call the non-panicking equivalent and handle the Option or Result idiomatically.

### Use Result over Option for errors :: suggested

When creating functions which can fail to return a value:
* Use **IcuResult** for all errors, or any cases where a user facing message is needed.
* Use **Option** for data accessors where "no data available" is a valid response (i.e. it's not an error per se).
  * Especially in cases where we expect the caller to have a reasonable response to getting [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
* Use a different enum for non-error cases with multiple return types (which can't use `Option`).

Examples:
* Does file with this path exists? - Option.
* Is there an element with this key in the list? - Option
* Try to open a file - Result
* Try to parse a string into a valid Language Identifier - Result

### Test all error cases :: required

You should write unit tests that cover all code paths that can generate an error.

If you find a bit of error-handling code that is unreachable by a unit test, you should consider replacing that code with `unreachable!()`.

## Integer Overflow

### Do not have unchecked overflow :: required

Malformed user input should not be able to cause integer overflow inside ICU4X implementation code.  You should return an error result if the user's input is too big and may cause integer overflow.

#### Use appropriately-sized integer types

You don't need a `usize` to represent a decimal digit 0-9.  By using the smallest possible integer type, you can reason better about cases where overflow can or cannot occur.

#### Checked Arithmetic

In cases where a hard limit on input is not possible, you can use methods such as [checked_add](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add).

#### Bounds Testing

By default, the `+` operator in Rust will panic upon overflow in debug mode.  You should employ thorough testing of boundaries to ensure that your arithmetic is never able to overflow.

### Don't accept infinite iterators :: suggested

Consider using `enumerate` to avoid an infinite iterator causing integer overflow by checking that the index does not exceed a bound that you set on the input.

Please do not mind that the following example could be written with `fold` instead of `enumerate`.

```rust
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Error {
  Limit,
}

/// iter: must contain fewer than usize::MAX elements
pub fn count_zeros<T>(iter: T) -> Result<usize, Error>
where
  T: Iterator<Item=u8>
{
  let mut result: usize = 0;
  for (i, v) in iter.enumerate() {
    if i == std::usize::MAX {
      return Err(Error::Limit)
    }
    if v == 0 {
      result += 1;
    }
  }
  return Ok(result)
}
```
