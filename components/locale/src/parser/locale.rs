use crate::extensions::Extensions;
use crate::parser::errors::ParserError;
use crate::parser::{parse_language_identifier_from_iter, ParserMode};
use crate::Locale;

pub fn parse_locale(t: &[u8]) -> Result<Locale, ParserError> {
    let mut iter = t.split(|c| *c == b'-' || *c == b'_').peekable();

    let langid = parse_language_identifier_from_iter(&mut iter, ParserMode::Locale)?;
    let extensions = if iter.peek().is_some() {
        Extensions::try_from_iter(&mut iter)?
    } else {
        Extensions::default()
    };
    Ok(Locale {
        language: langid.language,
        script: langid.script,
        region: langid.region,
        variants: langid.variants,
        extensions,
    })
}
