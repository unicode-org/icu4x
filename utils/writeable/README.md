# writeable [![crates.io](https://img.shields.io/crates/v/writeable)](https://crates.io/crates/writeable)

`writeable` is a utility crate of the [`ICU4X`] project.

It includes [`Writeable`], a core trait representing an object that can be written to a
sink implementing `std::fmt::Write`. It is an alternative to `std::fmt::Display` with the
addition of a function indicating the number of bytes to be written.

`Writeable` improves upon `std::fmt::Display` in two ways:

1. More efficient, since the sink can pre-allocate bytes.
2. Smaller code, since the format machinery can be short-circuited.

Types implementing Writeable have a defaulted [`write_to_string`](Writeable::write_to_string)
function. If desired, types implementing `Writeable` can manually implement `ToString`
to wrap `write_to_string`.

## Examples

```rust
use writeable::Writeable;
use writeable::LengthHint;
use writeable::assert_writeable_eq;
use std::fmt;

struct WelcomeMessage<'s>{
    pub name: &'s str,
}

impl<'s> Writeable for WelcomeMessage<'s> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')?;
        Ok(())
    }

    fn write_len(&self) -> LengthHint {
        // "Hello, " + '!' + length of name
        LengthHint::exact(8 + self.name.len())
    }
}

let message = WelcomeMessage { name: "Alice" };
assert_writeable_eq!(&message, "Hello, Alice!");
```

[`ICU4X`]: ../icu/index.html

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
