// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(test), no_std)]

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
//! Types implementing Writeable have a defaulted writeable_to_string function.
//! If desired, types implementing Writeable can manually implement ToString
//! to wrap writeable_to_string.
//!
//! # Examples
//!
//! ```
//! use writeable::Writeable;
//! use writeable::LengthHint;
//! use writeable::assert_writeable_eq;
//! use std::fmt;
//!
//! struct WelcomeMessage<'s>{
//!     pub name: &'s str,
//! }
//!
//! impl<'s> Writeable for WelcomeMessage<'s> {
//!     fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
//!         sink.write_str("Hello, ")?;
//!         sink.write_str(self.name)?;
//!         sink.write_char('!')?;
//!         Ok(())
//!     }
//!
//!     fn write_len(&self) -> LengthHint {
//!         // "Hello, " + '!' + length of name
//!         LengthHint::exact(8 + self.name.len())
//!     }
//! }
//!
//! let message = WelcomeMessage { name: "Alice" };
//! assert_writeable_eq!(&message, "Hello, Alice!");
//! ```
//!
//! [`ICU4X`]: ../icu/index.html

extern crate alloc;

mod impls;
mod ops;

use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt;

/// A hint to help consumers of Writeable pre-allocate bytes before they call write_to.
///
/// This behaves like Iterator::size_hint: it is a tuple where the first element is the
/// lower bound, and the second element is the upper bound. If the upper bound is `None`
/// either there is no known upper bound, or the upper bound is larger than usize.
///
/// LengthHint implements std::ops::{Add, Mul} and similar traits for easy composition.
/// During computation, the lower bound will saturate at usize::MAX, while the upper
/// bound will become None if usize::MAX is exceeded.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LengthHint(pub usize, pub Option<usize>);

impl LengthHint {
    pub fn undefined() -> Self {
        Self(0, None)
    }

    /// This is the exact length from write_to.
    pub fn exact(n: usize) -> Self {
        Self(n, Some(n))
    }

    /// This is at least the length from write_to.
    pub fn at_least(n: usize) -> Self {
        Self(n, None)
    }

    /// This is at most the length from write_to.
    pub fn at_most(n: usize) -> Self {
        Self(0, Some(n))
    }

    /// The length from write_to is in between n and m.
    pub fn between(n: usize, m: usize) -> Self {
        Self(Ord::min(n, m), Some(Ord::max(n, m)))
    }

    /// Returns a recommendation for the number of bytes to pre-allocate.
    /// If an upper bound exists, this is used, otherwise the lower bound
    /// (which might be 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use writeable::Writeable;
    ///
    /// fn pre_allocate_string(w: &impl Writeable) -> String {
    ///     String::with_capacity(w.write_len().capacity())
    /// }
    /// ```
    pub fn capacity(&self) -> usize {
        match self {
            Self(lower_bound, None) => *lower_bound,
            Self(_lower_bound, Some(upper_bound)) => *upper_bound,
        }
    }

    /// Returns whether the LengthHint indicates that the string is exactly 0 bytes long.
    pub fn is_zero(&self) -> bool {
        self.1 == Some(0)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Field(pub &'static str);

impl fmt::Display for Field {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Display>::fmt(self.0, f)
    }
}

/// A sink that supports annotation parts of the string with fields.
pub trait FormattedWriteableSink: fmt::Write {
    /// Adds a field to the currently active fields
    fn push_field(&mut self, field: Field) -> fmt::Result;

    /// Removes the last added field from the currently active fields
    fn pop_field(&mut self) -> fmt::Result;
}

/// Writeable is an alternative to std::fmt::Display with the addition of a length function.
pub trait Writeable {
    /// Writes bytes to the given sink. Errors from the sink are bubbled up. The default implementation
    /// delegates to write_to_fmt, ignoring the field information. write_to_fmt delegates back, so one
    /// of these has to be implemented.
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        struct CoreWriteAsFormattedWriteableSink<W: fmt::Write + ?Sized>(W);
        impl<W: fmt::Write + ?Sized> fmt::Write for CoreWriteAsFormattedWriteableSink<W> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.write_str(s)
            }

            fn write_char(&mut self, c: char) -> fmt::Result {
                self.0.write_char(c)
            }
        }

        impl<W: fmt::Write + ?Sized> FormattedWriteableSink for CoreWriteAsFormattedWriteableSink<W> {
            fn push_field(&mut self, _field: Field) -> fmt::Result {
                Ok(())
            }

            fn pop_field(&mut self) -> fmt::Result {
                Ok(())
            }
        }
        self.write_to_fmt(&mut CoreWriteAsFormattedWriteableSink(sink))
    }

    /// Write bytes and field annotations to the given FormattedWriteableSink. Errors from the sink are
    /// bubbled up. The default implementation delegates to write_to, and doesn't produce any field
    /// annotations. write_to delegates back, so one of these has to be implemented.
    fn write_to_fmt<S: FormattedWriteableSink + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        self.write_to(sink)
    }

    /// Returns a hint for the number of bytes that will be written to the sink.
    ///
    /// Override this method if it can be computed quickly.
    fn write_len(&self) -> LengthHint {
        LengthHint::undefined()
    }

    /// Creates a new String with the data from this Writeable. Like ToString,
    /// but smaller and faster.
    ///
    /// The default impl allocates an owned String. However, if it is possible to return a
    /// borrowed string, overwrite this method to return a `Cow::Borrowed`.
    ///
    /// To remove the `Cow` wrapper, call `.into_owned()`.
    ///
    /// # Examples
    ///
    /// Inspect a `Writeable` before writing it to the sink:
    ///
    /// ```
    /// use writeable::Writeable;
    /// use core::fmt::{Write, Result};
    ///
    /// fn write_if_ascii<W, S>(w: &W, sink: &mut S) -> Result
    /// where
    ///     W: Writeable + ?Sized,
    ///     S: Write + ?Sized,
    /// {
    ///     let s = w.writeable_to_string();
    ///     if s.is_ascii() {
    ///         sink.write_str(&s)
    ///     } else {
    ///         Ok(())
    ///     }
    /// }
    /// ```
    ///
    /// Convert the `Writeable` into a fully owned `String`:
    ///
    /// ```
    /// use writeable::Writeable;
    ///
    /// fn make_string(w: &impl Writeable) -> String {
    ///     w.writeable_to_string().into_owned()
    /// }
    /// ```
    fn writeable_to_string(&self) -> Cow<str> {
        let mut output = String::with_capacity(self.write_len().capacity());
        self.write_to(&mut output)
            .expect("impl Write for String is infallible");
        Cow::Owned(output)
    }
}

/// Testing macros for types implementing Writeable. The first argument should be a
/// `&dyn Writeable`, and the second argument either a string (assert_writeable_eq),
/// or a formatted debug string (assert_writeable_fmt_eq).
///
/// The macros tests for equality of string content, annotations (*_fmt_eq only), and
/// verify the size hint.
///
/// # Examples
///
/// ```
/// use writeable::Writeable;
/// use writeable::LengthHint;
/// use writeable::assert_writeable_eq;
/// use writeable::assert_writeable_fmt_eq;
/// use std::fmt;
///
/// struct Demo;
/// impl Writeable for Demo {
///     fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
///         sink.push_field(Field("word"))?;
///         sink.write_str("foo")?;
///         sink.pop_field()
///     }
///     fn write_len(&self) -> LengthHint {
///         LengthHint::exact(3)
///     }
/// }
///
/// assert_writeable_eq!(&Demo, "foo");
/// assert_writeable_eq!(&Demo, "foo", "Message: {}", "Hello World");
///
///
/// assert_writeable_fmt_eq!(
///     &Demo,
///     "foo\n\
///      ┏━━\n\
///      ┗ word"
/// );
/// assert_writeable_fmt_eq!(
///     &Demo,
///     "foo\n\
///      ┏━━\n\
///      ┗ word",
///     "Message: {}",
///     "Hello World"
/// );
/// ```
#[macro_export]
macro_rules! assert_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => {
        {
            let writeable = $actual_writeable;
            let actual = $crate::Writeable::writeable_to_string(&writeable);
            assert_eq!($expected_str, actual);
            let length_hint = $crate::Writeable::write_len(&writeable);
            assert!(length_hint.0 <= actual.len());
            if let Some(upper) = length_hint.1 {
                assert!(actual.len() <= upper);
            }
        }
    };

    ($actual_writeable:expr, $expected_str:expr, $($arg:tt)+) => {
        {
            let writeable = $actual_writeable;
            let actual = $crate::Writeable::writeable_to_string(&writeable);
            assert_eq!($expected_str, actual, $($arg)+);
            let length_hint = $crate::Writeable::write_len(&writeable);
            assert!(length_hint.0 <= $expected_str.len(), $($arg)+);
            if let Some(upper) = length_hint.1 {
                assert!($expected_str.len() <= upper, $($arg)+);
            }
        }
    };
}

pub mod formatted_string;

#[macro_export]
macro_rules! assert_writeable_fmt_eq {
    ($actual_writeable:expr, $expected_fmt_str:expr $(,)?) => {
        {
            let writeable = $actual_writeable;
            let expected = $expected_fmt_str;
            let actual = $crate::formatted_string::FormattedString::from_writeable(&writeable).unwrap();
            let debug_actual = format!("{:?}", actual);
            assert!(debug_actual == expected, "Expected:\n{}\nActual:\n{}", expected, debug_actual);
            let length_hint = $crate::Writeable::write_len(&writeable);
            assert!(length_hint.0 <= actual.len());
            if let Some(upper) = length_hint.1 {
                assert!(actual.len() <= upper);
            }
        }
    };
    ($actual_writeable:expr, $expected_fmt_str:expr, $($arg:tt)+) => {
        {
            let writeable = $actual_writeable;
            let expected = $expected_fmt_str;
            let actual = $crate::formatted_string::FormattedString::from_writeable(&writeable).unwrap();
            let debug_actual = format!("{:?}", actual);
            assert!(debug_actual == expected, "Expected:\n{}\nActual:\n{}\n{}", expected, debug_actual, format!($($arg)+));
            let length_hint = $crate::Writeable::write_len(&writeable);
            assert!(length_hint.0 <= actual.len(), $($arg)+);
            if let Some(upper) = length_hint.1 {
                assert!(actual.len() <= upper, $($arg)+);
            }
        }
    };
}
