// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use combine::parser::combinator::Either;
use combine::parser::error::{unexpected_any, Unexpected};
use combine::parser::token::Value;
use combine::{value, Parser, Stream};

/// Parser definition for POSIX time-zone strings as specified by
/// <https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html>
pub mod posix;

/// Ensures that the predicate is [`true`], otherwise returns an error with the provided
/// messages though combine's error machinery.
fn ensure<Input: Stream, L>(
    output: L,
    predicate: impl FnOnce(&L) -> bool,
    message: &'static str,
) -> Either<Value<Input, L>, Unexpected<Input, L, &'static str>>
where
    L: Clone,
{
    if predicate(&output) {
        value(output).left()
    } else {
        Parser::right(unexpected_any(message))
    }
}
