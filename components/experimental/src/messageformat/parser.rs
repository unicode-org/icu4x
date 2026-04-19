// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `MessageFormat` 2 parser.
//!
//! Composed of a [`lexer`] (token stream) and a recursive-descent [`grammar`].
//! The top-level [`parse`] function dispatches to the grammar.

pub mod grammar;
pub mod lexer;

use super::ast::Message;
use super::error::ParseError;

/// Parse an MF2 source string into a [`Message`].
pub fn parse(source: &str) -> Result<Message, ParseError> {
    grammar::parse_message(source)
}
