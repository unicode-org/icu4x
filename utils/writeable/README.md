`writeable` is a utility crate of the [`ICU4X`] project.

It includes [`Writeable`], a core trait representing an object that can be written to a
sink implementing std::fmt::Write. It is an alternative to std::fmt::Display with the
addition of a function indicating the number of bytes to be written.

Writeable improves upon std::fmt::Display in two ways:

1. More efficient, since the sink can pre-allocate bytes.
2. Smaller code, since the format machinery can be short-circuited.

Types implementing Writeable have a defaulted writeable_to_string function.
If desired, types implementing Writeable can manually implement ToString
to wrap writeable_to_string.

## Example

```rust
use writeable::Writeable;
use writeable::assert_writeable_eq;
use std::fmt;

struct WelcomeMessage<'s>{
    pub name: &'s str,
}

impl<'s> Writeable for WelcomeMessage<'s> {
    fn write_to(&self, sink: &mut dyn fmt::Write) -> fmt::Result {
        sink.write_str("Hello, ")?;
        sink.write_str(self.name)?;
        sink.write_char('!')?;
        Ok(())
    }

    fn write_len(&self) -> usize {
        // "Hello, " + '!' + length of name
        8 + self.name.len()
    }
}

let message = WelcomeMessage { name: "Alice" };
assert_writeable_eq!("Hello, Alice!", &message);
```
