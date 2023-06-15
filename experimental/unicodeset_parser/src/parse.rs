use std::{
    collections::HashSet,
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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Union,
    Difference,
    Intersection,
}

// note: "compiles" the set while building, so no intermediate parse tree, it's directly compiled.
pub struct UnicodeSetBuilder<'a, 'b> {
    single_set: CodePointInversionListBuilder,
    multi_set: HashSet<String>,
    iter: &'b mut Peekable<CharIndices<'a>>,
    next_op: Operation,
    inverted: bool,
}

impl<'a, 'b> UnicodeSetBuilder<'a, 'b> {
    // TODO: the parse_ functions might need an "op" argument that tells them whether to add or subtract or intersect the parsed content
    // maybe also rename in that case to collect_x or handle_x? parse could be fine though.

    // move this out into a static parse function that completes (i.e., drops) the created UnicodeSetBuilder before returning
    // fn new(source: &'a str) -> Self {
    //     UnicodeSetBuilder::new_inner(&mut source.char_indices().peekable())
    // }

    fn new_inner(iter: &'b mut Peekable<CharIndices<'a>>) -> UnicodeSetBuilder<'a, 'b> {
        UnicodeSetBuilder {
            single_set: CodePointInversionListBuilder::new(),
            multi_set: Default::default(),
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

    fn parse_property_inner(&mut self, end: char) -> Result<()> {
        // temp until property data loading: parse what comes until end
        let mut buffer = String::new();

        loop {
            match self.peek_char() {
                None => return Err(ParseError::eof()),
                Some(c) if *c == end => {
                    self.iter.next();
                    break;
                }
                Some(c) => {
                    buffer.push(*c);
                    self.iter.next();
                }
            }
        }

        self.process_strings([buffer].into());

        Ok(())


    }

    fn parse_property_perl(&mut self) -> Result<()> {
        self.consume('\\')?;
        self.consume('p')?;
        self.consume('{')?;

        self.parse_property_inner('}')?;

        Ok(())
    }

    // starts with :, consumes the trailing :]
    fn parse_property_posix(&mut self) -> Result<()> {
        self.consume(':')?;

        self.parse_property_inner(':')?;

        self.consume(']')?;

        Ok(())
    }

    fn unicode_set_start(&mut self) -> bool {
        match self.peek_char() {
            Some(&'\\') => {},
            Some(&'[') => return true,
            _ => return false,
        }

        // need to look one more char into the future. Peekable doesnt lend itself well to this, so maybe think about using a different iterator internally

        let mut future = self.iter.clone();
        future.next();

        match future.peek() {
            Some(&(_, 'p')) => true,
            _ => false,
        }
    }

    // beginning [ is already consumed
    //fn parse_unicode_set_inner<'b: 'a>(&'b mut self) -> Result<()> {
    fn parse_unicode_set_inner(&mut self) -> Result<()> {
    //fn parse_unicode_set_inner(&mut self) -> Result<()> {
        // special cases for the first char after [
        match self.peek_char() {
            None => return Err(ParseError::eof()),
            Some(&'^') => {
                self.iter.next();
                self.inverted = true;
            },
            Some(&'-') => {
                self.iter.next();
                self.single_set.add_char('-');
            },
            _ => {},
        }

        // repeatedly parse the following:
        // char
        // char-char
        // {multi}
        // unicodeset
        // & and - operators, but only between unicodesets

        fn legal_char(c: char) -> bool {
            !(
                c == '&'
                || c == '-'
                // actually maybe \\ is fine, because that is valid for an escape sequence, which is valid when we want a char.
                // problem: need to differentiate between \escape and \p...
                || c == '\\'
                || c == '['
                || c == ']'
                || c == '{'
                || c == '}'
                || c.is_ascii_whitespace()
            )
        }

        enum State {
            Begin,
            Char,
            CharMinus,
            AfterUnicodeSet,
            AfterOp,
        }
        use State::*;

        let mut state = Begin;
        let mut prev_char = None;

        loop {
            match (state, self.peek_char().copied()) {
                (_, None) => return Err(ParseError::eof()),
                (Begin | Char | AfterUnicodeSet, Some(']')) => {
                    self.iter.next();
                    if let Some(prev) = prev_char {
                        self.single_set.add_char(prev);
                        prev_char = None;
                    }

                    return Ok(());
                },
                (Begin | Char | AfterUnicodeSet, Some(c)) if legal_char(c)  => {
                    self.iter.next();
                    if let Some(prev) = prev_char {
                        self.single_set.add_char(prev);
                    }
                    prev_char = Some(c);
                    state = Char;
                },
                (CharMinus, Some(end)) if legal_char(end) => {
                    let start = prev_char.ok_or(ParseError::internal())?;
                    if start > end {
                        // todo: better error message
                        return Err(ParseError::unexpected(self.peek_index()?, end));
                    }

                    self.iter.next();

                    self.single_set.add_range(&(start..=end));
                    prev_char = None;
                    state = Begin;
                },
                (Begin | Char | AfterUnicodeSet | AfterOp, Some(_)) if self.unicode_set_start() => {
                    if let Some(prev) = prev_char {
                        self.single_set.add_char(prev);
                        prev_char = None;
                    }

                    let mut inner_builder = UnicodeSetBuilder::new_inner(self.iter);
                    inner_builder.parse_unicode_set()?;
                    let (single, multi) = inner_builder.finalize();

                    self.process_chars(single.build());
                    self.process_strings(multi);
                    
                    state = AfterUnicodeSet;
                },
                (Char, Some('-')) => {
                    self.iter.next();

                    state = CharMinus;
                },
                (AfterUnicodeSet, Some(op@('-' | '&'))) => {
                    let op = match op {
                        '&' => Operation::Intersection,
                        '-' => Operation::Difference,
                        _ => return Err(ParseError::internal()),
                    };
                    // I suppose this could also be a variable in this function, doesnt need to be on the builder?
                    self.next_op = op;

                    state = AfterOp;
                },
                (_, Some(c)) => return Err(ParseError::unexpected(self.peek_index()?, c)),
            }
        }


        todo!()
    }


    // the entry point, parses a full UnicodeSet. ignores remaining input
    pub fn parse_unicode_set(&mut self) -> Result<()> {
        match self.peek_char() {
            None => Err(ParseError::eof()),
            Some('\\') => self.parse_property_perl(),
            Some('[') => {
                self.iter.next();
                if let Some(&':') = self.peek_char() {
                    self.parse_property_posix()
                } else {
                    self.parse_unicode_set_inner()
                }
            },
            Some(&c) => Err(ParseError::unexpected(self.peek_index()?, c)),
        }
    }

    fn finalize(mut self) -> (CodePointInversionListBuilder, HashSet<String>) {
        if self.inverted {
            // code point inversion, removes all strings
            self.multi_set.drain();
            self.single_set.complement();
        }

        (self.single_set, self.multi_set)
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

    // BELOW IS WRONG, WE CAN HAVE BOTH KINDS AT SAME LEVEL, BUT STILL ONLY NEED op-handling for recursive UNICODE SETS because only UNICODE SETS are allowed to follow an op. others are implicitly additive
    // actually this might only be needed for processing sequences of ops in the case of a ContainerUnicodeSet, in the case
    // of a LeafUnicodeSet (i.e., one that contains chars, char ranges and multi codepoint elements), we cannot have other ops
    // fn process_string(&mut self, s: String) {
    //     match self.next_op
    // }
    fn process_strings(&mut self, other_strings: HashSet<String>) {
        match self.next_op {
            Operation::Union => self.multi_set.extend(other_strings.into_iter()),
            Operation::Difference => {
                self.multi_set = self.multi_set.difference(&other_strings).cloned().collect()
            }
            Operation::Intersection => {
                self.multi_set = self
                    .multi_set
                    .intersection(&other_strings)
                    .cloned()
                    .collect()
            }
        }
        // we consumed the operation, so we reset to the default Union
        self.reset_op();
    }

    fn process_chars<'any>(&mut self, other_chars: CodePointInversionList<'any>) {
        match self.next_op {
            Operation::Union => self.single_set.add_set(&other_chars),
            Operation::Difference => self.single_set.remove_set(&other_chars),
            Operation::Intersection => self.single_set.retain_set(&other_chars),
        }

        // we consumed the operation, so we reset to the default Union
        self.reset_op();
    }

    fn reset_op(&mut self) {
        self.next_op = Operation::Union;
    }
}

pub fn parse(source: &str) -> Result<()> {
    let mut iter = source.char_indices().peekable();
    let mut builder = UnicodeSetBuilder::new_inner(&mut iter);

    builder.parse_unicode_set()?;

    // todo continue here, build set, return it, impl debug on stuff, and have test run

    // convert builder.single and builder.multi to an actual CodePointInversionListAndStringList

    Ok(())
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
