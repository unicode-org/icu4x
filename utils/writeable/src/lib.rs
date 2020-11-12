// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `writeable` is a utility crate of the [`ICU4X`] project.
//!
//! It includes [`Writeable`], a core trait representing an object that can be written to a
//! sink implementing std::fmt::Write. It is an alternative to std::fmt::Display with the
//! addition of a function indicating the number of bytes to be written.
//!
//! Writeable improves upon std::fmt::Display in two ways:
//!
//! 1. More efficient, since the sink can pre-allocate bytes.
//! 2. Smaller code, since the format machinery can be short-circuited.
//!
//! Types implementing Writeable automatically implement ToString. Because of this, you cannot
//! implement both Writeable and std::fmt::Display on the same type.
//!
//! [`Writeable`]: ./trait.Writeable.html
//! [`ICU4X`]: ../icu/index.html

use std::fmt;

/// Writeable is an alternative to std::fmt::Display with the addition of a length function.
pub trait Writeable {
    /// Writes bytes to the given sink. Errors from the sink are bubbled up.
    fn write_to(&self, sink: &mut dyn fmt::Write) -> fmt::Result;

    /// Returns an estimation of the number of bytes that will be written to the sink. The actual
    /// number of bytes may be slightly different than what this function returns.
    ///
    /// This function may return an enumeration in the future. See:
    /// https://github.com/unicode-org/icu4x/issues/370
    fn write_len(&self) -> usize;

    /// Creates a new String with the data from this Writeable. Like ToString, but faster.
    fn writeable_to_string(&self) -> String {
        let mut output = String::with_capacity(self.write_len());
        self.write_to(&mut output)
            .expect("impl Write for String is infallible");
        output
    }
}

/// Testing macro for types implementing Writeable. The first argument should be a string, and
/// the second argument should be a `&dyn Writeable`.
///
/// The macro tests for equality of both string content and string length. If your Writeable
/// implementation returns an inexact string length, don't use this macro.
///
/// # Example
///
/// ```
/// use writeable::Writeable;
/// use writeable::assert_writeable_eq;
/// use std::fmt;
///
/// struct Demo;
/// impl Writeable for Demo {
///     fn write_to(&self, sink: &mut dyn fmt::Write) -> fmt::Result {
///         sink.write_str("foo")
///     }
///     fn write_len(&self) -> usize {
///         3
///     }
/// }
///
/// assert_writeable_eq!("foo", &Demo);
/// ```
#[macro_export]
macro_rules! assert_writeable_eq {
    ($expected_str:expr, $actual_writeable:expr) => {
        let writeable: &dyn Writeable = $actual_writeable;
        assert_eq!($expected_str, writeable.writeable_to_string());
        assert_eq!($expected_str.len(), writeable.write_len());
    };
}
