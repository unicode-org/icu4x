// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod error;
pub use crate::replacement::ReplacementProvider;
use crate::{pattern::PatternIterator, token::PatternToken};
pub use error::InterpolatorError;
use std::str::FromStr;

type Result<E, R> = std::result::Result<Option<E>, InterpolatorError<<R as FromStr>::Err>>;

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
/// use icu_pattern::{Parser, Interpolator};
///
/// #[derive(Debug, PartialEq)]
/// enum Token {
///     Variant1,
///     Variant2
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum Element<'s> {
///     Token(Token),
///     Literal(&'s str),
/// }
///
/// impl<'s> From<&'s str> for Element<'s> {
///     fn from(input: &'s str) -> Self {
///         Self::Literal(input)
///     }
/// }
///
/// let mut parser = Parser::new("{0}, {1}");
///
/// let replacements = vec![
///     vec![
///         Element::Token(Token::Variant1),
///         Element::Literal(" foo "),
///         Element::Token(Token::Variant2),
///     ],
///     vec![
///         Element::Token(Token::Variant2),
///         Element::Literal(" bar "),
///         Element::Token(Token::Variant1),
///     ],
/// ];
///
/// let mut interpolator = Interpolator::new(parser, replacements);
///
/// let mut result = vec![];
///
/// while let Some(element) = interpolator.try_next().expect("Failed to advance iterator") {
///     result.push(element);
/// }
///
/// assert_eq!(result, &[
///     Element::Token(Token::Variant1),
///     Element::Literal(" foo "),
///     Element::Token(Token::Variant2),
///     Element::Literal(", "),
///     Element::Token(Token::Variant2),
///     Element::Literal(" bar "),
///     Element::Token(Token::Variant1),
/// ]);
/// ```
///
/// # Type parameters
///
/// - `P`: A type implementing [`PatternIterator`] trait, like [`Parser`] or [`IntoIterVec`].
/// - `R`: A replacement provider type implementing [`ReplacementProvider`].
/// - `K`: A key type used to retrieve associated replacement pattern in place of a placeholder.
/// - `E`: An element type returned by the `Interpolator` which must implement
/// [`From<&str>`][`From`]
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
/// Serveral design decisions have been made that the reader should be aware of when using the API.
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
pub struct Interpolator<P, R, K, E>
where
    R: ReplacementProvider<K, E>,
{
    pattern: P,
    replacements: R,
    current_replacement: Option<R::Iter>,
}

impl<P, R, K, E> Interpolator<P, R, K, E>
where
    R: ReplacementProvider<K, E>,
{
    /// Creates a new `Interpolator`.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{Parser, Interpolator};
    ///
    /// enum Element {
    ///     Literal(String),
    ///     Token,
    /// }
    ///
    /// let mut parser = Parser::new("{0}, {1}");
    /// let mut interpolator = Interpolator::new(parser, vec![
    ///     vec![
    ///         Element::Token
    ///     ]
    /// ]);
    /// ```
    pub fn new(pattern: P, replacements: R) -> Self {
        Self {
            pattern,
            replacements,
            current_replacement: None,
        }
    }

    /// An iterator method that advances the iterator and returns the result of an attempt to
    /// interpolate parser and replacement provider tokens.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::{Parser, Interpolator};
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum Element {
    ///     Literal(String),
    ///     TokenOne,
    ///     TokenTwo,
    /// }
    ///
    /// impl From<&str> for Element {
    ///     fn from(input: &str) -> Self {
    ///        Self::Literal(input.to_string())
    ///     }
    /// }
    ///
    /// let mut parser = Parser::new("{0}, {1}");
    /// let mut interpolator = Interpolator::new(parser, vec![
    ///     vec![
    ///         Element::TokenOne
    ///     ],
    ///     vec![
    ///         Element::TokenTwo
    ///     ]
    /// ]);
    ///
    ///
    /// // A call to try_next() returns the next value…
    /// assert_eq!(Ok(Some(Element::TokenOne)), interpolator.try_next());
    /// assert_eq!(Ok(Some(Element::Literal(", ".to_string()))), interpolator.try_next());
    /// assert_eq!(Ok(Some(Element::TokenTwo)), interpolator.try_next());
    ///
    /// // … and then None once it's over.
    /// assert_eq!(Ok(None), interpolator.try_next());
    /// ```
    ///
    /// # Lifetimes
    ///
    /// - `p`: The life time of an input parser, which is the life time of the string slice to be parsed.
    pub fn try_next<'p>(&mut self) -> Result<E, K>
    where
        E: From<&'p str>,
        P: PatternIterator<'p, K>,
        K: FromStr + std::fmt::Display,
        K::Err: std::fmt::Debug,
    {
        loop {
            if let Some(ref mut replacement) = &mut self.current_replacement {
                if let Some(v) = replacement.next() {
                    return Ok(Some(v));
                } else {
                    self.current_replacement = None;
                }
            }
            match self.pattern.try_next().map_err(InterpolatorError::Parser)? {
                Some(PatternToken::Literal { content, .. }) => {
                    return Ok(Some(content.into()));
                }
                Some(PatternToken::Placeholder(p)) => {
                    self.current_replacement = self.replacements.take_replacement(&p);
                    if self.current_replacement.is_none() {
                        return Err(InterpolatorError::MissingPlaceholder(p.to_string()));
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
    use crate::Parser;
    use std::fmt::{Display, Write};

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

    #[derive(Debug)]
    pub enum Element<'s> {
        Literal(&'s str),
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
            Self::Literal(input)
        }
    }

    #[test]
    fn simple_interpolate() {
        for sample in SAMPLES.iter() {
            let iter = Parser::new(&sample.0);

            let replacements: Vec<Vec<Element>> = sample
                .1
                .iter()
                .map(|r| r.iter().map(|&t| t.into()).collect())
                .collect();
            let mut i = Interpolator::new(iter, replacements);
            let mut result = String::new();
            while let Some(elem) = i.try_next().unwrap() {
                write!(result, "{}", elem).unwrap();
            }
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
            let iter = Parser::new(&sample.0);

            let replacements: std::collections::HashMap<String, Vec<Element>> = sample
                .1
                .iter()
                .map(|(k, v)| (k.to_string(), v.iter().map(|&t| t.into()).collect()))
                .collect();

            let mut i = Interpolator::new(iter, replacements);
            while let Some(_) = i.try_next().unwrap() {}
        }
    }
}
