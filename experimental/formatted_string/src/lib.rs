// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod formatted_string;

pub use crate::formatted_string::FormattedString;

use core::fmt;
pub use writeable::{LengthHint, Writeable};

#[derive(Clone, Copy, PartialEq)]
pub struct Field(pub &'static str);

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Similar to Writeable, but also produces field annotations
pub trait FormattedWriteable {
    /// Write bytes and field annotations to the given FormattedWriteableSink. Errors from
    /// the sink are bubbled up.
    fn fmt_write_to<S: FormattedWriteableSink>(&self, sink: &mut S) -> Result<(), S::Error>;

    /// Returns a hint for the number of bytes that will be written to the sink.
    ///
    /// Override this method if it can be computed quickly.
    fn fmt_write_len(&self) -> LengthHint {
        LengthHint::undefined()
    }

    fn writeable_to_fmt_string(&self) -> FormattedString {
        let mut output = FormattedString::with_capacity(self.fmt_write_len().capacity());
        self.fmt_write_to(&mut output)
            .expect("impl FormattedSink for FormattedString is infallible");
        output
    }

    fn as_writeable(self) -> FormattedWriteableAsWriteable<Self>
    where
        Self: Sized,
    {
        FormattedWriteableAsWriteable(self)
    }
}

/// A sink for a FormattedWriteable. See FormattedString for an implementation.
pub trait FormattedWriteableSink {
    type Error: core::fmt::Display;

    /// Writes a string, which is annotated with the currently active fields
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;

    /// Writes a char, which is annotated with the currently active fields
    fn write_char(&mut self, c: char) -> Result<(), Self::Error>;

    /// Adds a field to the currently active fields
    fn push_field(&mut self, field: Field) -> Result<(), Self::Error>;

    /// Removes the last added field from the currently active fields
    fn pop_field(&mut self) -> Result<(), Self::Error>;

    /// Writes a formatted string, which is annotated with its own fields and the
    /// currently active fields.
    fn write_fmt_str(&mut self, s: &FormattedString) -> Result<(), Self::Error>;
}

/// Blanket implementation for all Writeables. Writes the content without any annotations.
/// If you want to implement both Writeable and FormattedWriteable, implement the latter
/// and use as_writeable.
impl<W: Writeable + ?Sized> FormattedWriteable for W {
    fn fmt_write_to<S: FormattedWriteableSink>(&self, sink: &mut S) -> Result<(), S::Error> {
        struct FormattedWriteableSinkAsCoreWrite<'a, S: FormattedWriteableSink>(
            &'a mut S,
            Option<S::Error>,
        );
        impl<S: FormattedWriteableSink> fmt::Write for FormattedWriteableSinkAsCoreWrite<'_, S> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.0.write_str(s) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        // We have to store the S::Error somewhere to return it later.
                        self.1 = Some(e);
                        Err(fmt::Error)
                    }
                }
            }
        }
        let mut wrapper = FormattedWriteableSinkAsCoreWrite(sink, None);
        match self.write_to(&mut wrapper) {
            Ok(_) => Ok(()),
            Err(_) => Err(wrapper.1.unwrap()),
        }
    }

    fn fmt_write_len(&self) -> LengthHint {
        self.write_len()
    }
}

pub struct FormattedWriteableAsWriteable<T: FormattedWriteable>(T);

impl<T: FormattedWriteable> Writeable for FormattedWriteableAsWriteable<T> {
    fn write_len(&self) -> LengthHint {
        self.0.fmt_write_len()
    }
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        struct CoreWriteAsFormattedWriteableSink<W: fmt::Write + ?Sized>(W);

        impl<W: fmt::Write + ?Sized> FormattedWriteableSink for CoreWriteAsFormattedWriteableSink<W> {
            type Error = fmt::Error;

            fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
                self.0.write_str(s)
            }

            fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
                self.0.write_char(c)
            }

            fn push_field(&mut self, _field: Field) -> Result<(), Self::Error> {
                Ok(())
            }

            fn pop_field(&mut self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn write_fmt_str(&mut self, s: &FormattedString) -> Result<(), Self::Error> {
                self.0.write_str(s.as_str())
            }
        }
        self.0
            .fmt_write_to(&mut CoreWriteAsFormattedWriteableSink(sink))
    }
}
