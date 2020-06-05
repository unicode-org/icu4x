pub(crate) mod errors;
mod langid;
mod locale;

pub(crate) use errors::ParserError;
pub(crate) use langid::{
    parse_language_identifier, parse_language_identifier_from_iter, ParserMode,
};
pub(crate) use locale::parse_locale;
