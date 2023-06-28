// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod error;

use crate::{
    interpolator::{InterpolatedKind, Interpolator},
    parser::{Parser, ParserError, ParserOptions},
    replacement::ReplacementProvider,
    token::PatternToken,
};
pub use error::PatternError;
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display, Write},
    ops::Deref,
    str::FromStr,
};
use writeable::Writeable;

/// `Pattern` stores the result of parsing operation as a vector
/// of [`PatternToken`] elements.
///
/// # Type parameters
///
/// - `P`: The type of the placeholder used as a key for the [`ReplacementProvider`].
///
/// # Lifetimes
///
/// - `p`: The life time of an input string slice to be parsed.
///
/// [`ReplacementProvider`]: crate::ReplacementProvider
#[derive(Debug)]
pub struct Pattern<'s, P>(pub(crate) Vec<PatternToken<'s, P>>);

impl<'s, P> Pattern<'s, P> {
    /// Interpolates the `Pattern` with provided replacements and returns
    /// a [`InterpolatedPattern`] structure.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_pattern::{InterpolatedKind, Pattern};
    /// use std::{convert::TryFrom, ops::Deref};
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Element(usize);
    ///
    /// let pattern = Pattern::try_from("${0}").expect("Failed to parse a pattern");
    ///
    /// let replacements = vec![Element(5)];
    ///
    /// let interpolated_pattern = pattern
    ///     .interpolate(&replacements)
    ///     .expect("Failed to interpolate");
    ///
    /// assert_eq!(
    ///     interpolated_pattern.deref(),
    ///     &[
    ///         InterpolatedKind::Literal(&"$".into()),
    ///         InterpolatedKind::Element(&Element(5)),
    ///     ],
    /// );
    /// ```
    ///
    /// For allocation-free interpolation, see `interpolate_to_string` or
    /// `interpolate_to_write`.
    ///
    /// For lower level interpolation iterator see [`Interpolator`].
    pub fn interpolate<'i, E, R>(
        &'i self,
        replacements: &'i R,
    ) -> Result<InterpolatedPattern<'i, 's, E>, PatternError<R::Key>>
    where
        R: ReplacementProvider<'i, E, Key = P>,
        P: Debug + FromStr + PartialEq + Clone,
        <P as FromStr>::Err: Debug + PartialEq,
    {
        let mut interpolator = Interpolator::new(&self.0, replacements);

        let mut result = vec![];
        while let Some(ik) = interpolator.try_next()? {
            result.push(ik);
        }
        Ok(InterpolatedPattern(result))
    }

    /// Interpolates the `Pattern` with provided replacements and a new
    /// [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use std::{convert::TryFrom, fmt::Display};
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Element(usize);
    ///
    /// impl Display for Element {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         self.0.fmt(f)
    ///     }
    /// }
    ///
    /// let pattern = Pattern::try_from("${0}").expect("Failed to parse a pattern");
    ///
    /// let replacements = vec![Element(5)];
    ///
    /// let interpolated_pattern = pattern
    ///     .interpolate_to_string(&replacements)
    ///     .expect("Failed to interpolate");
    ///
    /// assert_eq!(interpolated_pattern, "$5",);
    /// ```
    ///
    /// For buffer write interpolation, see `interpolate_to_write`.
    ///
    /// For lower level interpolation iterator see [`Interpolator`].
    pub fn interpolate_to_string<'i, E, R>(
        &'i self,
        replacements: &'i R,
    ) -> Result<String, PatternError<R::Key>>
    where
        R: ReplacementProvider<'i, E, Key = P>,
        P: Debug + FromStr + PartialEq + Clone,
        <P as FromStr>::Err: Debug + PartialEq,
        E: 'i + Display,
    {
        let mut result = String::new();
        self.interpolate_to_write(replacements, &mut result)?;
        Ok(result)
    }

    /// Interpolates the `Pattern` writing the result into a buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use std::{convert::TryFrom, fmt::Display};
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Element(usize);
    ///
    /// impl Display for Element {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         self.0.fmt(f)
    ///     }
    /// }
    ///
    /// let pattern = Pattern::try_from("${0}").expect("Failed to parse a pattern");
    ///
    /// let replacements = vec![Element(5)];
    ///
    /// let mut result = String::new();
    /// pattern
    ///     .interpolate_to_write(&replacements, &mut result)
    ///     .expect("Failed to interpolate");
    ///
    /// assert_eq!(result, "$5",);
    /// ```
    pub fn interpolate_to_write<'i, E, R, W>(
        &'i self,
        replacements: &'i R,
        sink: &mut W,
    ) -> Result<(), PatternError<R::Key>>
    where
        R: ReplacementProvider<'i, E, Key = P>,
        P: Debug + FromStr + PartialEq + Clone,
        <P as FromStr>::Err: Debug + PartialEq,
        E: 'i + Display,
        W: Write,
    {
        let mut interpolator = Interpolator::new(&self.0, replacements);
        while let Some(ik) = interpolator.try_next()? {
            write!(sink, "{ik}")?;
        }
        Ok(())
    }

    /// Interpolates the `Pattern` writing the result into a [`Writeable`] buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_pattern::Pattern;
    /// use std::convert::TryFrom;
    ///
    /// let pattern = Pattern::try_from("${0}").expect("Failed to parse a pattern");
    ///
    /// let replacements = vec![5];
    ///
    /// let mut result = String::new();
    /// pattern
    ///     .interpolate_to_writeable(&replacements, &mut result)
    ///     .expect("Failed to interpolate");
    ///
    /// assert_eq!(result, "$5",);
    /// ```
    pub fn interpolate_to_writeable<'i, E, R, W>(
        &'i self,
        replacements: &'i R,
        sink: &mut W,
    ) -> Result<(), PatternError<R::Key>>
    where
        R: ReplacementProvider<'i, E, Key = P>,
        P: Debug + FromStr + PartialEq + Clone,
        <P as FromStr>::Err: Debug + PartialEq,
        E: 'i + Writeable,
        W: Write,
    {
        let mut interpolator = Interpolator::new(&self.0, replacements);
        while let Some(ik) = interpolator.try_next()? {
            ik.write_to(sink)?;
        }
        Ok(())
    }
}

impl<'s, P> TryFrom<&'s str> for Pattern<'s, P>
where
    P: FromStr,
    <P as FromStr>::Err: Debug,
{
    type Error = ParserError<<P as FromStr>::Err>;

    fn try_from(input: &'s str) -> Result<Self, Self::Error> {
        Parser::new(
            input,
            ParserOptions {
                allow_raw_letters: false,
            },
        )
        .try_into()
    }
}

impl<'p, P> TryFrom<Parser<'p, P>> for Pattern<'p, P>
where
    P: FromStr,
    <P as FromStr>::Err: Debug,
{
    type Error = ParserError<<P as FromStr>::Err>;

    fn try_from(mut parser: Parser<'p, P>) -> Result<Self, Self::Error> {
        let mut result = vec![];
        while let Some(token) = parser.try_next()? {
            result.push(token);
        }
        Ok(Self(result))
    }
}

impl<'p, P> From<Vec<PatternToken<'p, P>>> for Pattern<'p, P>
where
    P: FromStr,
    <P as FromStr>::Err: Debug,
{
    fn from(tokens: Vec<PatternToken<'p, P>>) -> Self {
        Self(tokens)
    }
}

impl<'p, P> Deref for Pattern<'p, P> {
    type Target = [PatternToken<'p, P>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `InterpolatedPattern` stores the result of parsing operation as a vector
/// of [`InterpolatedKind`] elements.
///
/// # Type parameters
///
/// - `E`: An element type returned by the `ReplacementProvider`.
///
/// # Lifetimes
///
/// - `i`: The life time of `ReplacementProvider`.
/// - `s`: The life time of literals stored in the `E`
///
/// [`ReplacementProvider`]: crate::ReplacementProvider
#[derive(Debug, PartialEq)]
pub struct InterpolatedPattern<'i, 's, E>(Vec<InterpolatedKind<'i, 's, E>>);

impl<'i, 's, E> Writeable for InterpolatedPattern<'i, 's, E>
where
    E: Writeable,
{
    fn write_to<W>(&self, sink: &mut W) -> std::result::Result<(), std::fmt::Error>
    where
        W: std::fmt::Write + ?Sized,
    {
        for elem in &self.0 {
            elem.write_to(sink)?;
        }
        Ok(())
    }
}

impl<'i, 's, E> Display for InterpolatedPattern<'i, 's, E>
where
    E: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in &self.0 {
            write!(f, "{elem}")?;
        }
        Ok(())
    }
}

impl<'i, 's, E> Deref for InterpolatedPattern<'i, 's, E> {
    type Target = [InterpolatedKind<'i, 's, E>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
