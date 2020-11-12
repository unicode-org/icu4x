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
