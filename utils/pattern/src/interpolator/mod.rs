// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod error;
use crate::{replacement::ReplacementProvider, token::PatternToken};
pub use error::InterpolatorError;
use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};
use writeable::Writeable;

/// The type returned by the [`Interpolator`] iterator.
/// This enum stores references to string literals parsed as
/// part of the pattern and elements returned by the [`ReplacementProvider`].
///
/// # Lifetimes
///
/// - `i`: The life time of a kind that is being interpolated.
/// - `s`: The life time of a string slice literal.
#[derive(Debug, PartialEq)]
pub enum InterpolatedKind<'i, 's, E> {
    Literal(&'i Cow<'s, str>),
    Element(&'i E),
}

impl<'i, 's, E> Writeable for InterpolatedKind<'i, 's, E>
where
    E: Writeable,
{
    fn write_to<W>(&self, sink: &mut W) -> std::result::Result<(), std::fmt::Error>
    where
        W: std::fmt::Write + ?Sized,
    {
        match self {
            Self::Literal(lit) => sink.write_str(lit),
            Self::Element(elem) => elem.write_to(sink),
        }
    }
}

impl<'i, 's, E> Display for InterpolatedKind<'i, 's, E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Literal(lit) => f.write_str(lit),
            Self::Element(elem) => elem.fmt(f),
        }
    }
}

type Result<E, R> = std::result::Result<Option<E>, InterpolatorError<R>>;

/// Placeholder pattern interpolator.
///
/// The interpolator takes a pattern parser iterator and a replacement provider and
/// generates a new iterator over elements.
///
/// The replacement may be any type, and the only bind is on the [`ReplacementProvider`] to
/// be able to return an iterator over elements to be interplotted into the pattern in
/// place of the placeholders based on the placeholder keys.
///
/// # Examples
/// ```
/// use icu_pattern::{
///     InterpolatedKind, Interpolator, Parser, ParserOptions, Pattern,
/// };
/// use std::convert::TryInto;
///
/// #[derive(Debug, PartialEq)]
/// enum Element {
///     Value(usize),
/// }
///
/// let pattern: Pattern<_> = Parser::new(
///     "{0} days ago",
///     ParserOptions {
///         allow_raw_letters: true,
///     },
/// )
/// .try_into()
/// .unwrap();
///
/// let replacements = vec![Element::Value(5)];
///
/// let mut interpolator = Interpolator::new(&pattern, &replacements);
///
/// let mut result = vec![];
///
/// while let Some(element) =
///     interpolator.try_next().expect("Failed to advance iterator")
/// {
///     result.push(element);
/// }
///
/// assert_eq!(
///     result,
///     &[
///         InterpolatedKind::Element(&Element::Value(5)),
///         InterpolatedKind::Literal(&" days ago".into()),
///     ]
/// );
/// ```
///
/// # Type parameters
///
/// - `R`: A replacement provider type implementing [`ReplacementProvider`].
/// - `E`: An element type returned by the [`ReplacementProvider`].
///
/// # Lifetimes
///
/// - `i`: The life time of an input pattern slice.
/// - `p`: The life time of an input [`PatternToken`], which is the life time of the string slice.
///
/// # Element & Replacement Provider
///
/// In order to allow for wide range of inputs to be interpolated using the placeholder pattern,
/// the `Element` and [`ReplacementProvider`] types are generic.
/// This allows the consumer of the API to decide what elements the pattern should return and how
/// will they be identified based on any type of key that can be parsed out of a string slice.
///
/// This design allows for the interpolator to remain agnostic and flexible and handles wide range
/// of ownership and life time models.
///
/// To simplify the common use cases, the [`ReplacementProvider`] comes with implementations for
/// [`Vec`] (where  the placehoder key is [`usize`]) and [`HashMap`] (where the placeholder key is
/// [`String`]) but the consumer is free to implement their own providers for any type they wish.
///
/// # Design Decisions
///
/// The interpolator is written in an intentionally generic way to enable use against wide range
/// of potential placeholder pattern models and use cases.
///
/// ## Fallible Iterator
///
/// Rust providers a strong support for iterators and iterator combinators, which
/// fits very well into the design of this parser/interpolator model.
///
/// Unfortunately, Rust iterators at the moment are infallible, while parsers are inhereantely
/// fallible. As such, the decision has been made to design the API in line with what
/// we hope will become a trait signature of a fallible iterator in the future, rather
/// than implementing a reversed infallible iterator (where the [`Item`] would be
/// `Option<Result<Item>>`).
///
/// Since the interpolator chains on top of the [`Parser`] it inherits the same fallible
/// iterator API and behavior.
///
/// [`Item`]: std::iter::Iterator::Item
/// [`HashMap`]: std::collections::HashMap
/// [`Parser`]: crate::parser::Parser
/// [`IntoIterVec`]: crate::pattern::IntoIterVec
#[derive(Debug)]
pub struct Interpolator<'i, 'p, R, E>
where
    R: ReplacementProvider<'i, E>,
{
    tokens: &'i [PatternToken<'p, R::Key>],
    token_idx: usize,
    replacements: &'i R,
    current_replacement: Option<R::Iter>,
}

impl<'i, 'p, R, E> Interpolator<'i, 'p, R, E>
where
    R: ReplacementProvider<'i, E>,
{
    /// Creates a new `Interpolator`.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{Interpolator, Parser, ParserOptions, Pattern};
    /// use std::convert::TryInto;
    ///
    /// enum Element {
    ///     Literal(String),
    ///     Token,
    /// }
    ///
    /// let pattern: Pattern<usize> = Parser::new(
    ///     "{0}, {1}",
    ///     ParserOptions {
    ///         allow_raw_letters: false,
    ///     },
    /// )
    /// .try_into()
    /// .unwrap();
    /// let replacements = vec![vec![Element::Token]];
    /// let mut interpolator =
    ///     Interpolator::<Vec<Vec<_>>, Element>::new(&pattern, &replacements);
    /// ```
    pub fn new(tokens: &'i [PatternToken<'p, R::Key>], replacements: &'i R) -> Self {
        Self {
            tokens,
            token_idx: 0,
            replacements,
            current_replacement: None,
        }
    }

    /// An iterator method that advances the iterator and returns the result of an attempt to
    /// interpolate parser and replacement provider tokens.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{
    ///     InterpolatedKind, Interpolator, Parser, ParserOptions, Pattern,
    /// };
    /// use std::convert::TryInto;
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum Element {
    ///     TokenOne,
    ///     TokenTwo,
    /// }
    ///
    /// let mut pattern: Pattern<_> = Parser::new(
    ///     "{0}, {1}",
    ///     ParserOptions {
    ///         allow_raw_letters: false,
    ///     },
    /// )
    /// .try_into()
    /// .unwrap();
    ///
    /// let replacements = vec![vec![Element::TokenOne], vec![Element::TokenTwo]];
    /// let mut interpolator = Interpolator::new(&pattern, &replacements);
    ///
    /// // A call to try_next() returns the next value…
    /// assert_eq!(
    ///     Ok(Some(InterpolatedKind::Element(&Element::TokenOne))),
    ///     interpolator.try_next()
    /// );
    /// assert_eq!(
    ///     Ok(Some(InterpolatedKind::Literal(&", ".into()))),
    ///     interpolator.try_next()
    /// );
    /// assert_eq!(
    ///     Ok(Some(InterpolatedKind::Element(&Element::TokenTwo))),
    ///     interpolator.try_next()
    /// );
    ///
    /// // … and then None once it's over.
    /// assert_eq!(Ok(None), interpolator.try_next());
    /// ```
    pub fn try_next(&mut self) -> Result<InterpolatedKind<'i, 'p, E>, R::Key>
    where
        R::Key: Debug + FromStr + PartialEq + Clone,
        <R::Key as FromStr>::Err: Debug + PartialEq,
    {
        loop {
            if let Some(ref mut replacement) = &mut self.current_replacement {
                if let Some(v) = replacement.next() {
                    return Ok(Some(InterpolatedKind::Element(v)));
                } else {
                    self.current_replacement = None;
                }
            }
            match self.tokens.get(self.token_idx) {
                Some(PatternToken::Literal { content, .. }) => {
                    self.token_idx += 1;
                    return Ok(Some(InterpolatedKind::Literal(content)));
                }
                Some(PatternToken::Placeholder(p)) => {
                    self.token_idx += 1;
                    self.current_replacement = self.replacements.take_replacement(p);
                    if self.current_replacement.is_none() {
                        return Err(InterpolatorError::MissingPlaceholder(p.clone()));
                    }
                }
                None => {
                    return Ok(None);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Parser, ParserOptions, Pattern, PatternError};
    use std::convert::TryInto;
    use std::{borrow::Cow, fmt::Display};

    const SAMPLES: &[(&str, &[&[&str]], &str)] = &[
        (
            "'Foo' {0} 'and' {1}",
            &[&["Hello"], &["World"]],
            "Foo Hello and World",
        ),
        (
            "{0}, {1} 'and' {2}",
            &[&["Start"], &["Middle"], &["End"]],
            "Start, Middle and End",
        ),
        ("{0} 'at' {1}", &[&["Hello"], &["World"]], "Hello at World"),
    ];

    #[derive(Debug, PartialEq)]
    pub enum Element<'s> {
        Literal(Cow<'s, str>),
    }

    impl Display for Element<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Literal(s) => f.write_str(s),
            }
        }
    }

    impl<'s> From<&'s str> for Element<'s> {
        fn from(input: &'s str) -> Self {
            Self::Literal(Cow::Borrowed(input))
        }
    }

    #[test]
    fn simple_interpolate() {
        for sample in SAMPLES.iter() {
            let pattern: Pattern<usize> = Parser::new(
                sample.0,
                ParserOptions {
                    allow_raw_letters: false,
                },
            )
            .try_into()
            .unwrap();

            let replacements: Vec<Vec<Element>> = sample
                .1
                .iter()
                .map(|r| r.iter().map(|&t| t.into()).collect())
                .collect();
            let interpolated_pattern = pattern
                .interpolate::<'_, Element, _>(&replacements)
                .unwrap();
            let result = interpolated_pattern.to_string();
            assert_eq!(result, sample.2);
        }
    }

    #[test]
    fn simple_interpolate_hash() {
        let named_samples = vec![(
            "{start}, {middle} 'and' {end}",
            vec![
                ("start", vec!["Start"]),
                ("middle", vec!["Middle"]),
                ("end", vec!["End"]),
            ],
        )];

        for sample in &named_samples {
            let pattern: Pattern<String> = Parser::new(
                sample.0,
                ParserOptions {
                    allow_raw_letters: false,
                },
            )
            .try_into()
            .unwrap();

            let replacements: std::collections::HashMap<String, Vec<Element>> = sample
                .1
                .iter()
                .map(|(k, v)| {
                    (
                        (*k).to_owned(),
                        v.iter().map(|&t| Element::from(t)).collect(),
                    )
                })
                .collect();

            let interpolated_pattern = pattern
                .interpolate::<'_, Element, _>(&replacements)
                .unwrap();
            let _ = interpolated_pattern.to_string();
        }
    }

    #[test]
    fn missing_placeholder() {
        let samples: Vec<(&str, Vec<Element>)> = vec![("{0} days", vec![])];

        for sample in &samples {
            let pattern: Pattern<usize> = Parser::new(
                sample.0,
                ParserOptions {
                    allow_raw_letters: true,
                },
            )
            .try_into()
            .expect("Failed to parse a sample");

            let interpolated_pattern = pattern.interpolate::<'_, Element, _>(&sample.1);
            assert_eq!(
                interpolated_pattern,
                Err(PatternError::Interpolator(
                    InterpolatorError::MissingPlaceholder(0)
                )),
            );
        }
    }
}
