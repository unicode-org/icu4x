# writeable [![crates.io](https://img.shields.io/crates/v/writeable)](https://crates.io/crates/writeable)

<!-- cargo-rdme start -->

`writeable` is a utility crate of the [`ICU4X`] project.

It includes [`Writeable`], a core trait representing an object that can be written to a
sink implementing `std::fmt::Write`. It is an alternative to `std::fmt::Display` with the
addition of a function indicating the number of bytes to be written.

`Writeable` improves upon `std::fmt::Display` in two ways:

1. More efficient, since the sink can pre-allocate bytes.
2. Smaller code, since the format machinery can be short-circuited.

## Examples

```rust
use std::fmt;
use writeable::assert_writeable_eq;
use writeable::LengthHint;
use writeable::Writeable;

struct WelcomeMessage<'s> {
    pub name: &'s str,
}

impl<'s> Writeable for WelcomeMessage<'s> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')?;
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        // "Hello, " + '!' + length of name
        LengthHint::exact(8 + self.name.len())
    }
}

let message = WelcomeMessage { name: "Alice" };
assert_writeable_eq!(&message, "Hello, Alice!");

// Types implementing `Writeable` are recommended to also implement `fmt::Display`.
// This can be simply done by redirecting to the `Writeable` implementation:
writeable::impl_display_with_writeable!(WelcomeMessage<'_>);
```

[`ICU4X`]: ../icu/index.html

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
