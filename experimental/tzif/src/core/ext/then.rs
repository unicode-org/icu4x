// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::core::{ParseResult, Parsed};
use eyre::{bail, Context};

use super::map::MapValue;

/// A trait that specifies behavior that occurs after parsing a value.
pub trait Then<Value, Source: Clone>: Sized {
    /// Perform an action given the parsed value as an argument.
    /// This is useful for test assertions and logging.
    fn then(self, f: impl FnOnce(&Value)) -> Self;

    /// Parse a subsequent value using the previously parsed value as context
    /// by passing it into the subsequent parse function as an argument.
    fn then_parse_with_context<Out>(
        self,
        f: impl FnOnce(Value, &mut Source) -> ParseResult<Out, Source>,
    ) -> ParseResult<Out, Source>;

    /// Ensure that the value matches some predicate, otherwise
    /// propagate an error message.
    fn then_ensure_or_err(
        self,
        f: impl Fn(&Value) -> bool,
        msg: &'static str,
    ) -> ParseResult<Value, Source>;

    /// Ensure that the value matches some predicate, otherwise
    /// create propagate an error message that may use that value as context.
    fn then_ensure_or_err_with(
        self,
        f: impl Fn(&Value) -> bool,
        e: impl FnOnce(Option<&Value>) -> String,
    ) -> ParseResult<Value, Source>;
}

impl<Value, Source: Clone> Then<Value, Source> for Parsed<Value, Source> {
    /// Performs an action given the [`Parsed`] value as context.
    fn then(self, f: impl FnOnce(&Value)) -> Self {
        f(self.value());
        self
    }

    /// Parse a subsequent value using the previously parsed value as context
    /// by passing it into the subsequent parse function as an argument.
    fn then_parse_with_context<Out>(
        self,
        f: impl FnOnce(Value, &mut Source) -> ParseResult<Out, Source>,
    ) -> ParseResult<Out, Source> {
        let (value, mut source) = self.split();
        f(value, &mut source)
    }

    /// Ensures the [`Parsed`] value matches the predicate, otherwise returns an error message.
    fn then_ensure_or_err(
        self,
        f: impl Fn(&Value) -> bool,
        msg: &'static str,
    ) -> ParseResult<Value, Source> {
        if f(self.value()) {
            return Ok(self);
        }
        bail!(msg)
    }

    /// Ensures the [`Parsed`] value matches the predicate, otherwise returns an error message
    /// given the parsed value as context for the error.
    fn then_ensure_or_err_with(
        self,
        f: impl Fn(&Value) -> bool,
        e: impl FnOnce(Option<&Value>) -> String,
    ) -> ParseResult<Value, Source> {
        if f(self.value()) {
            return Ok(self);
        }
        bail!(e(Some(self.value())))
    }
}

impl<Value, Source: Clone> Then<Value, Source> for ParseResult<Value, Source> {
    /// Performs an action given the [`ParseResult`] value as context.
    /// If `Self` is already [`Err`], propagates the error.
    fn then(self, f: impl FnOnce(&Value)) -> Self {
        self.map(|inner| inner.then(f))
    }

    /// Parse a subsequent value using the previously parsed value as context
    /// by passing it into the subsequent parse function as an argument.
    fn then_parse_with_context<Out>(
        self,
        f: impl FnOnce(Value, &mut Source) -> ParseResult<Out, Source>,
    ) -> ParseResult<Out, Source> {
        self.and_then(|parsed| parsed.then_parse_with_context(f))
    }

    /// Ensures the [`ParseResult`] value matches the predicate, otherwise returns an error message.
    /// If `Self` is already [`Err`], propagates the error.
    fn then_ensure_or_err(
        self,
        f: impl Fn(&Value) -> bool,
        msg: &'static str,
    ) -> ParseResult<Value, Source> {
        if let Ok(parsed) = self {
            return parsed.then_ensure_or_err(f, msg);
        }
        self.wrap_err(msg)
    }

    /// Ensures the [`ParseResult`] value matches the predicate, otherwise returns an error message.
    /// If `Self` is already [`Err`], propagates the error.
    fn then_ensure_or_err_with(
        self,
        f: impl Fn(&Value) -> bool,
        e: impl FnOnce(Option<&Value>) -> String,
    ) -> ParseResult<Value, Source> {
        if let Ok(parsed) = self {
            return parsed.then_ensure_or_err_with(f, e);
        }
        self
    }
}

/// An extension trait for acting upon a parsed [`Option`] value that is [`Some`].
pub trait OnSuccess<Inner, Source: Clone> {
    /// Parse a subsequent value when the previously parsed value is [`Some`].
    fn on_success_then_parse<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source>;

    /// Parse a subsequent value when the previously parsed value is [`Some`].
    /// using the previously parsed value as context by passing it into the
    /// subsequent parse function as an argument.
    fn on_success_then_parse_with_context<
        Value,
        F: Fn(Inner, &mut Source) -> ParseResult<Value, Source>,
    >(
        self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source>;
}

impl<Inner, Source: Clone> OnSuccess<Inner, Source> for Parsed<Option<Inner>, Source> {
    /// Parse a subsequent value when the previously parsed value is [`Some`].
    fn on_success_then_parse<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        mut self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source> {
        match self.value() {
            Some(_) => parse_fn(&mut self.source()).map_value(|value| Some(value)),
            None => Ok(Parsed::new(None, self.source())),
        }
    }

    /// Parse a subsequent value when the previously parsed value is [`Some`].
    /// using the previously parsed value as context by passing it into the
    /// subsequent parse function as an argument.
    fn on_success_then_parse_with_context<
        Value,
        F: Fn(Inner, &mut Source) -> ParseResult<Value, Source>,
    >(
        mut self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source> {
        let mut source = self.source();
        match self.into_value() {
            Some(inner) => parse_fn(inner, &mut source).map_value(|value| Some(value)),
            None => Ok(Parsed::new(None, source)),
        }
    }
}

impl<Inner, Source: Clone> OnSuccess<Inner, Source> for ParseResult<Option<Inner>, Source> {
    /// Parse a subsequent value when the previously parsed value is [`Some`].
    fn on_success_then_parse<Value, F: Fn(&mut Source) -> ParseResult<Value, Source>>(
        self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source> {
        self.and_then(|parsed| parsed.on_success_then_parse(parse_fn))
    }

    /// Parse a subsequent value when the previously parsed value is [`Some`].
    /// using the previously parsed value as context by passing it into the
    /// subsequent parse function as an argument.
    fn on_success_then_parse_with_context<
        Value,
        F: Fn(Inner, &mut Source) -> ParseResult<Value, Source>,
    >(
        self,
        parse_fn: F,
    ) -> ParseResult<Option<Value>, Source> {
        self.and_then(|parsed| parsed.on_success_then_parse_with_context(parse_fn))
    }
}
