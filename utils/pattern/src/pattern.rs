use crate::parser::error::ParserError;
use crate::token::PatternToken;
use std::{fmt::Debug, str::FromStr};

/// Input trait for a `Interpolator`.
///
/// This trait is automatically implemented on a [`Vec<PatternToken>>`][`Vec`].
///
/// # Examples
/// ```
/// use icu_pattern::{
///     PatternToken,
///     Interpolator,
///     pattern::IntoIterVec,
/// };
///
/// #[derive(Debug, PartialEq)]
/// enum Element<'s> {
///     Literal(&'s str),
///     Token,
/// }
///
/// impl<'s> From<&'s str> for Element<'s> {
///     fn from(input: &'s str) -> Self {
///         Self::Literal(input)
///     }
/// }
///
/// let pattern: IntoIterVec<_> = vec![
///     PatternToken::Placeholder(0usize),
///     PatternToken::Literal { content: ", ", quoted: false },
///     PatternToken::Placeholder(1),
/// ].into();
///
/// let mut interpolator = Interpolator::new(pattern, vec![
///     vec![
///         Element::Token
///     ],
///     vec![
///         Element::Token
///     ],
/// ]);
///
/// assert_eq!(Ok(Some(Element::Token)), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Literal(", "))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Token)), interpolator.try_next());
/// ```
pub trait PatternIterator<'p, P> {
    fn try_next(&mut self) -> Result<Option<PatternToken<'p, P>>, ParserError<<P as FromStr>::Err>>
    where
        P: FromStr,
        P::Err: Debug;
}

/// An iterator constructed from a [`Vec<PatternToken>>`][`Vec`]
/// which implements [`PatternIterator`] trait.
pub struct IntoIterVec<'p, P> {
    buf: <Vec<PatternToken<'p, P>> as IntoIterator>::IntoIter,
}

impl<'p, P> Iterator for IntoIterVec<'p, P> {
    type Item = PatternToken<'p, P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.next()
    }
}

impl<'p, P> From<Vec<PatternToken<'p, P>>> for IntoIterVec<'p, P> {
    fn from(input: Vec<PatternToken<'p, P>>) -> Self {
        Self {
            buf: input.into_iter(),
        }
    }
}

impl<'p, P> PatternIterator<'p, P> for IntoIterVec<'p, P> {
    fn try_next(&mut self) -> Result<Option<PatternToken<'p, P>>, ParserError<<P as FromStr>::Err>>
    where
        P: FromStr,
        P::Err: Debug,
    {
        Ok(self.buf.next())
    }
}
