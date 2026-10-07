# AV001: No panics in library code

- **Status:** Proposed
- **Enforced by:** Clippy lint (`clippy::unwrap_used`, `clippy::expect_used`,
  `clippy::indexing_slicing`, `clippy::panic`), and review
- **Applies to:** library code in crates with the standard `lib.rs` header
  ([boilerplate.md](../../process/boilerplate.md#library-annotations)).
  The style guide asks for the header in primary ICU4X crates; it "need not
  extend to utils". Not test code.
- **Source:** [style_guide.md: Don't Panic](../../process/style_guide.md#dont-panic--required),
  [Lints › Panics](../../process/style_guide.md#panics--required),
  [data_safety.md](../../design/data_safety.md)
- **Checked against:** ICU4X 2.3 (`main` @ `2fa9afb769`)

## ❌ Don't

```rust,ignore
let (key, value) = input.split_once('=').unwrap(); // panics if there is no '='
let first = names[0];                              // panics if `names` is empty
let (head, tail) = text.split_at(n);               // panics if `n` is not a char boundary
```

## ✅ Do instead

If the caller can do something about the problem, return an error:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    MissingEquals,
}

pub fn parse_pair(input: &str) -> Result<(&str, &str), ParseError> {
    input.split_once('=').ok_or(ParseError::MissingEquals)
}

assert_eq!(parse_pair("a=b"), Ok(("a", "b")));
assert_eq!(parse_pair("ab"), Err(ParseError::MissingEquals));
```

If the problem can only come from invalid data or a bug, use a fallback and
a debug assertion. From `components/normalizer/src/lib.rs`
(`split_normalized`):

```rust,ignore
text.split_at_checked(up_to).unwrap_or_else(|| {
    // Internal bug, not even GIGO, never supposed to happen
    debug_assert!(false);
    ("", text)
})
```

Use the method that doesn't panic (`get`, `split_at_checked`,
`checked_add`) and handle `None`. Don't add a check only so that you can call
a method that panics.

## Why

- A panic in a library stops the whole program that uses it. The style guide
  says: "never turn an error into a panic by unconditionally unwrapping the
  result in the library".
- ICU4X loads data that it did not create. data_safety.md: "Code should
  never panic at runtime based on invalid data". Code that panics on bad data
  also lets an attacker crash the application (denial of service).
- `debug_assert!` finds the bug in tests and debug builds. In release builds,
  the code continues with the fallback value ("garbage in, garbage out").

## Exceptions

[Lints › Panics](../../process/style_guide.md#panics--required) allows an
`#[allow]`, with a comment, when:

- the panic only happens if fundamental invariants of the codebase are broken;
- the API is documented to panic, and no API that is not documented to panic
  uses it;
- the condition is checked *very nearby*;
- the code is test code.

`.expect("poison")` on a `Mutex` lock is also fine, in crates that use `std`
([Exception: Poison](../../process/style_guide.md#exception-poison)).

## Automation

- **Today:** the `lib.rs` header has
  `#![cfg_attr(not(test), deny(clippy::indexing_slicing, clippy::unwrap_used, clippy::expect_used, clippy::panic))]`,
  and CI runs Clippy (`ci-job-clippy`). `ci-job-test-gigo` runs the tests with
  debug assertions off, so the code after a `debug_assert!` runs in CI too.
- **Not covered:** `split_at`, `unreachable!`, and arithmetic overflow
  ([Do not have unchecked overflow](../../process/style_guide.md#do-not-have-unchecked-overflow--required)).
  Reviewers check them.
- **Idea:** a tidy check that every library crate has the header. Generated
  crates (`provider/data/*`) don't have it. TODO(verify): which other crates
  leave it out on purpose.
- The four lints are set in each `lib.rs`, not in the workspace `[lints]`
  table, because of
  [rust-clippy#13981](https://github.com/rust-lang/rust-clippy/issues/13981)
  (see the comment in the root `Cargo.toml`).
  [#5974](https://github.com/unicode-org/icu4x/issues/5974) tracks moving
  lints to `[lints]`.
