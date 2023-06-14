use std::{
    iter::Peekable,
    str::{CharIndices, Chars},
};

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder},
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_properties::provider::*;
use icu_provider::prelude::*;

// Parses UnicodeSets

// type UnicodeSet<'a> = CodePointInversionListAndStringList<'a>;

pub enum ParseErrorKind {
    UnexpectedChar(char),
    Eof,
    Internal,
}

// pub struct ParseError {
//     pub offset: usize,
//     pub kind: ParseErrorKind,
// }

pub enum ParseError {
    WithOffset { offset: usize, kind: ParseErrorKind },
    WithOutOffset(ParseErrorKind),
}

impl ParseError {
    fn new_with_offset(offset: usize, kind: ParseErrorKind) -> Self {
        ParseError::WithOffset { offset, kind }
    }

    fn new_without_offset(kind: ParseErrorKind) -> Self {
        ParseError::WithOutOffset(kind)
    }

    fn eof() -> Self {
        Self::new_without_offset(ParseErrorKind::Eof)
    }

    fn unexpected(offset: usize, c: char) -> Self {
        Self::new_with_offset(offset, ParseErrorKind::UnexpectedChar(c))
    }

    fn internal() -> Self {
        Self::new_without_offset(ParseErrorKind::Internal)
    }
}

type Result<T, E = ParseError> = core::result::Result<T, E>;

// pub struct ParsedPropertySet {
//     pub key: String,
//     pub value: Option<String>,
// }

// pub enum ParsedUnicodeSet {
//     ParsedPropertySet(ParsedPropertySet),
//     ParsedCharSet(ParsedCharSet),
//     ParsedOpSet(ParsedOpSet),
// }

enum Operation {
    Union,
    Difference,
    Intersection,
}

// note: "compiles" the set while building, so no intermediate parse tree, it's directly compiled.
pub struct UnicodeSetBuilder<'a> {
    single_set: CodePointInversionListBuilder,
    multi_set: Vec<String>,
    iter: &'a mut Peekable<CharIndices<'a>>,
    next_op: Operation,
    inverted: bool,
}

impl<'a> UnicodeSetBuilder<'a> {
    // TODO: the parse_ functions might need an "op" argument that tells them whether to add or subtract or intersect the parsed content
    // maybe also rename in that case to collect_x or handle_x? parse could be fine though.

    // move this out into a static parse function that completes (i.e., drops) the created UnicodeSetBuilder before returning
    // fn new(source: &'a str) -> Self {
    //     UnicodeSetBuilder::new_inner(&mut source.char_indices().peekable())
    // }

    fn new_inner(iter: &'a mut Peekable<CharIndices<'a>>) -> Self {
        UnicodeSetBuilder {
            single_set: CodePointInversionListBuilder::new(),
            multi_set: Vec::new(),
            iter,
            next_op: Operation::Union,
            inverted: false,
        }
    }

    fn consume(&mut self, expected: char) -> Result<()> {
        match self.iter.next() {
            None => Err(ParseError::eof()),
            Some((offset, c)) if c != expected => Err(ParseError::unexpected(offset, c)),
            _ => Ok(()),
        }
    }

    fn parse_property_perl(&mut self) -> Result<()> {
        self.consume('\\')?;
        self.consume('p')?;

        // temp: parse what's between { and }
        let mut buffer = String::new();

        self.consume('{')?;
        loop {
            match self.peek_char() {
                None => return Err(ParseError::eof()),
                Some('}') => {
                    self.iter.next();
                    break;
                }
                Some(c) => {
                    buffer.push(*c);
                    self.iter.next();
                }
            }
        }

        self.multi_set.push(buffer);
        Ok(())
    }

    // starts with :, consumes the trailing :]
    fn parse_property_posix(&mut self) -> Result<()> {
        self.consume(':')?;
        // temp: parse what's between : and :
        let mut buffer = String::new();

        loop {
            match self.peek_char() {
                None => return Err(ParseError::eof()),
                Some(':') => {
                    self.iter.next();
                    break;
                }
                Some(c) => {
                    buffer.push(*c);
                    self.iter.next();
                }
            }
        }

        self.consume(']')?;

        self.multi_set.push(buffer);
        Ok(())
    }

    fn parse_unicode_set_inner(&mut self) -> Result<()> {
        self.consume('[')?;

        match self.peek_char() {
            None => Err(ParseError::eof()),
            Some(&':') => self.parse_property_posix(),
            Some(c) => {
                // parse other kind of set, either char or unicode based, also need to check for ^
                // TODO: check precedence of [^ set1 op set2] - when is ^ applied - is applied to whole set after parsing
                todo!()
            }
        }
    }

    pub fn parse_unicode_set(&mut self) -> Result<()> {
        match self.peek_char() {
            None => Err(ParseError::eof()),
            Some('\\') => self.parse_property_perl(),
            Some('[') => self.parse_unicode_set_inner(),
            Some(&c) => Err(ParseError::unexpected(self.peek_index()?, c)),
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.iter.peek().map(|(_, c)| c)
    }

    // returns a result for ergonomics in the usual use-case of knowing that the iterator is not empty, without resorting to .unwrap()
    fn peek_index(&mut self) -> Result<usize> {
        self.iter
            .peek()
            .map(|&(idx, _)| idx)
            .ok_or(ParseError::internal())
    }
}

// fn parse_unicode_set(source: &str) -> Result<ParsedUnicodeSet, ParseError> {
//     let mut it = source.char_indices().peekable();

// }

// // https://www.unicode.org/reports/tr35/#Unicode_Sets
// // Only ECMA-262 properties are supported at the moment
// pub fn parse_unicode_set_unstable<P>(provider: &P, source: &str) -> Result<ParsedUnicodeSet, ParseError>
// where
//     P: ?Sized
//         + DataProvider<AsciiHexDigitV1Marker>
//         + DataProvider<AlphabeticV1Marker>
//         + DataProvider<BidiControlV1Marker>
//         + DataProvider<BidiMirroredV1Marker>
//         + DataProvider<CaseIgnorableV1Marker>
//         + DataProvider<CasedV1Marker>
//         + DataProvider<ChangesWhenCasefoldedV1Marker>
//         + DataProvider<ChangesWhenCasemappedV1Marker>
//         + DataProvider<ChangesWhenLowercasedV1Marker>
//         + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
//         + DataProvider<ChangesWhenTitlecasedV1Marker>
//         + DataProvider<ChangesWhenUppercasedV1Marker>
//         + DataProvider<DashV1Marker>
//         + DataProvider<DefaultIgnorableCodePointV1Marker>
//         + DataProvider<DeprecatedV1Marker>
//         + DataProvider<DiacriticV1Marker>
//         + DataProvider<EmojiV1Marker>
//         + DataProvider<EmojiComponentV1Marker>
//         + DataProvider<EmojiModifierV1Marker>
//         + DataProvider<EmojiModifierBaseV1Marker>
//         + DataProvider<EmojiPresentationV1Marker>
//         + DataProvider<ExtendedPictographicV1Marker>
//         + DataProvider<ExtenderV1Marker>
//         + DataProvider<GraphemeBaseV1Marker>
//         + DataProvider<GraphemeExtendV1Marker>
//         + DataProvider<HexDigitV1Marker>
//         + DataProvider<IdsBinaryOperatorV1Marker>
//         + DataProvider<IdsTrinaryOperatorV1Marker>
//         + DataProvider<IdContinueV1Marker>
//         + DataProvider<IdStartV1Marker>
//         + DataProvider<IdeographicV1Marker>
//         + DataProvider<JoinControlV1Marker>
//         + DataProvider<LogicalOrderExceptionV1Marker>
//         + DataProvider<LowercaseV1Marker>
//         + DataProvider<MathV1Marker>
//         + DataProvider<NoncharacterCodePointV1Marker>
//         + DataProvider<PatternSyntaxV1Marker>
//         + DataProvider<PatternWhiteSpaceV1Marker>
//         + DataProvider<QuotationMarkV1Marker>
//         + DataProvider<RadicalV1Marker>
//         + DataProvider<RegionalIndicatorV1Marker>
//         + DataProvider<SentenceTerminalV1Marker>
//         + DataProvider<SoftDottedV1Marker>
//         + DataProvider<TerminalPunctuationV1Marker>
//         + DataProvider<UnifiedIdeographV1Marker>
//         + DataProvider<UppercaseV1Marker>
//         + DataProvider<VariationSelectorV1Marker>
//         + DataProvider<WhiteSpaceV1Marker>
//         + DataProvider<XidContinueV1Marker>
//         + DataProvider<XidStartV1Marker>,
// {
//     // Hmm... how to factor out the data loading if the function I want to use uses a DataProvider interface.

// }
