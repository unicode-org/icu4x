// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;
use std::fmt::Display;
use std::{collections::HashSet, iter::Peekable, str::CharIndices};

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder},
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_properties::maps::load_script;
use icu_properties::script::load_script_with_extensions_unstable;
use icu_properties::sets::{
    load_for_ecma262_unstable, load_for_general_category_group, load_xid_continue, load_xid_start,
    CodePointSetData,
};
use icu_properties::Script;
use icu_properties::{provider::*, GeneralCategoryGroup};
use icu_provider::prelude::*;

/// The kind of error that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// An unexpected character was encountered. This variant implies the other variants
    /// (notably `UnknownProperty` and `Unimplemented`) do not apply.
    UnexpectedChar(char),
    /// The property name or value is unknown. For property names, make sure you use the spelling
    /// defined in [ECMA-262](https://tc39.es/ecma262/#table-nonbinary-unicode-properties).
    UnknownProperty,
    /// A reference to an unknown variable.
    UnknownVariable,
    /// A variable of a certain type occurring in an unexpected context.
    UnexpectedVariable,
    /// The source is an incomplete unicode set.
    Eof,
    /// Something unexpected went wrong with our code. Please file a bug report on GitHub.
    Internal,
    /// The provided syntax is not supported by us. Note that unknown properties will return the
    /// `UnknownProperty` variant, not this one.
    Unimplemented,
    /// The provided escape sequence is not a valid Unicode code point.
    InvalidEscape,
}
use ParseErrorKind as PEK;
use zerovec::VarZeroVec;

impl ParseErrorKind {
    fn with_offset(self, offset: usize) -> ParseError {
        ParseError {
            offset: Some(offset),
            kind: self,
        }
    }
}

impl From<ParseErrorKind> for ParseError {
    fn from(kind: ParseErrorKind) -> Self {
        ParseError { offset: None, kind }
    }
}

/// The error type returned by the `parse` functions in this crate.
///
/// See [`ParseError::fmt_with_source`] for pretty-printing and [`ParseErrorKind`] of the
/// different types of errors represented by this struct.
#[derive(Debug, Clone)]
pub struct ParseError {
    // offset is the index to an arbitrary byte in the last character in the source that makes sense
    // to display as location for the error, e.g., the unexpected character itself or
    // for an unknown property name the last character of the name.
    offset: Option<usize>,
    kind: ParseErrorKind,
}

type Result<T, E = ParseError> = core::result::Result<T, E>;

impl ParseError {
    /// Pretty-prints this error and if applicable, shows where the error occurred in the source.
    ///
    /// Must be called with the same source that was used to parse the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_unicodeset_parser::*;
    ///
    /// let source = "[[abc]-x]";
    /// let set = parse(source);
    /// assert!(set.is_err());
    /// let err = set.unwrap_err();
    /// assert_eq!(err.fmt_with_source(source).to_string(), "[[abc]-x← error: unexpected character 'x'");
    /// ```
    ///
    /// ```
    /// use icu_unicodeset_parser::*;
    ///
    /// let source = r"[\N{LATIN CAPITAL LETTER A}]";
    /// let set = parse(source);
    /// assert!(set.is_err());
    /// let err = set.unwrap_err();
    /// assert_eq!(err.fmt_with_source(source).to_string(), r"[\N← error: unimplemented");
    /// ```
    pub fn fmt_with_source(&self, source: &str) -> impl Display {
        let ParseError { offset, kind } = *self;

        if kind == ParseErrorKind::Eof {
            return format!("{source}← error: unexpected end of input");
        }
        let mut s = String::new();
        if let Some(offset) = offset {
            if offset < source.len() {
                // offset points to any byte of the last character we want to display.
                // in the case of ASCII, this is easy - we just display bytes [..=offset].
                // however, if the last character is more than one byte in UTF-8
                // we cannot use ..=offset, because that would potentially include only partial
                // bytes of last character in our string. hence we must find the start of the
                // following character and use that as the (exclusive) end of our string.

                // offset points into the last character we want to include, hence the start of the
                // first character we want to exclude is at least offset + 1.
                let mut exclusive_end = offset + 1;
                // TODO: replace this loop with str::ceil_char_boundary once stable
                for _ in 0..3 {
                    // is_char_boundary returns true at the latest once exclusive_end == source.len()
                    if source.is_char_boundary(exclusive_end) {
                        break;
                    }
                    exclusive_end += 1;
                }

                // exclusive_end is at most source.len() due to str::is_char_boundary and at least 0 by type
                #[allow(clippy::indexing_slicing)]
                s.push_str(&source[..exclusive_end]);
                s.push_str("← ");
            }
        }
        s.push_str("error: ");
        match kind {
            ParseErrorKind::UnexpectedChar(c) => {
                s.push_str(&format!("unexpected character '{}'", c.escape_debug()));
            }
            ParseErrorKind::UnknownProperty => {
                s.push_str("unknown property");
            }
            ParseErrorKind::UnknownVariable => {
                s.push_str("unknown variable");
            }
            ParseErrorKind::UnexpectedVariable => {
                s.push_str("unexpected variable");
            }
            ParseErrorKind::Eof => {
                s.push_str("unexpected end of input");
            }
            ParseErrorKind::Internal => {
                s.push_str("internal error");
            }
            ParseErrorKind::Unimplemented => {
                s.push_str("unimplemented");
            }
            ParseErrorKind::InvalidEscape => {
                s.push_str("invalid escape sequence");
            }
        }

        s
    }

    /// Returns the [`ParseErrorKind`] of this error.
    pub fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    fn or_with_offset(self, offset: usize) -> Self {
        match self.offset {
            Some(_) => self,
            None => ParseError {
                offset: Some(offset),
                ..self
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum VariableValue<'a> {
    UnicodeSet(CodePointInversionListAndStringList<'a>),
    // in theory, a one-code-point string is always the same as a char, but we might want to keep
    // this variant for efficiency?
    Char(char),
    String(String),
}

pub type VariableMap<'a> = HashMap<String, VariableValue<'a>>;

// necessary helper because char::is_whitespace is not equivalent to [:Pattern_White_Space:]
#[inline]
fn is_char_pattern_white_space(c: char) -> bool {
    matches!(
        c,
        ('\u{0009}'..='\u{000D}')
            | '\u{0020}'
            | '\u{0085}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{2028}'
            | '\u{2029}'
    )
}

// this ignores the ambiguity between \-escapes and \p{} perl properties. it assumes it is in a context where \p is just 'p'
// returns whether the provided char signifies the start of a literal char (raw or escaped - so \ is a legal char start)
fn legal_char_start(c: char) -> bool {
    !(c == '&'
        || c == '-'
        || c == '$'
        || c == '^'
        || c == '['
        || c == ']'
        || c == '{'
        || is_char_pattern_white_space(c))
}

// same as `legal_char_start` but adapted to the charInString nonterminal. \ is allowed due to escapes.
fn legal_char_in_string_start(c: char) -> bool {
    !(c == '}' || is_char_pattern_white_space(c))
}

// invariant: a one-codepoint-element is represented as a char, not a single-char String
#[derive(Debug)]
enum CharOrString {
    Char(char),
    String(String),
}

impl From<char> for CharOrString {
    fn from(c: char) -> Self {
        CharOrString::Char(c)
    }
}

impl From<String> for CharOrString {
    fn from(s: String) -> Self {
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => CharOrString::Char(c),
            _ => CharOrString::String(s),
        }
    }
}

#[derive(Debug)]
enum MainToken<'data> {
    // to be interpreted as value
    CharOrString(CharOrString),
    // inner set
    UnicodeSet(CodePointInversionListAndStringList<'data>),
    // anchor, only at the end of a set ([... $])
    DollarSign,
    // intersection operator, only inbetween two sets ([[...] & [...]])
    Ampersand,
    // difference operator, only inbetween two sets ([[...] - [...]])
    // or
    // range operator, only inbetween two chars ([a-z], [a-{z}])
    Minus,
    // ] to indicate the end of a set
    ClosingBracket,
}

#[derive(Debug)]
enum VarOrAnchor<'a, 'b> {
    A,
    V(&'b VariableValue<'a>),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Union,
    Difference,
    Intersection,
}

// this builds the set on-the-fly while parsing it
struct UnicodeSetBuilder<'a, 'b, 'c, 'var, 'parse, P: ?Sized> {
    single_set: CodePointInversionListBuilder,
    multi_set: HashSet<String>,
    iter: &'b mut Peekable<CharIndices<'a>>,
    inverted: bool,
    variable_map: &'parse VariableMap<'var>,
    // TODO(#3550): Figure out how to store the borrowed sets directly here.
    xid_start: &'parse CodePointInversionList<'parse>,
    xid_continue: &'parse CodePointInversionList<'parse>,
    property_provider: &'c P,
}

impl<'a, 'b, 'c, 'var, 'parse, P> UnicodeSetBuilder<'a, 'b, 'c, 'var, 'parse, P>
where
    P: ?Sized
        + DataProvider<AsciiHexDigitV1Marker>
        + DataProvider<AlphabeticV1Marker>
        + DataProvider<BidiControlV1Marker>
        + DataProvider<BidiMirroredV1Marker>
        + DataProvider<CaseIgnorableV1Marker>
        + DataProvider<CasedV1Marker>
        + DataProvider<ChangesWhenCasefoldedV1Marker>
        + DataProvider<ChangesWhenCasemappedV1Marker>
        + DataProvider<ChangesWhenLowercasedV1Marker>
        + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
        + DataProvider<ChangesWhenTitlecasedV1Marker>
        + DataProvider<ChangesWhenUppercasedV1Marker>
        + DataProvider<DashV1Marker>
        + DataProvider<DefaultIgnorableCodePointV1Marker>
        + DataProvider<DeprecatedV1Marker>
        + DataProvider<DiacriticV1Marker>
        + DataProvider<EmojiV1Marker>
        + DataProvider<EmojiComponentV1Marker>
        + DataProvider<EmojiModifierV1Marker>
        + DataProvider<EmojiModifierBaseV1Marker>
        + DataProvider<EmojiPresentationV1Marker>
        + DataProvider<ExtendedPictographicV1Marker>
        + DataProvider<ExtenderV1Marker>
        + DataProvider<GraphemeBaseV1Marker>
        + DataProvider<GraphemeExtendV1Marker>
        + DataProvider<HexDigitV1Marker>
        + DataProvider<IdsBinaryOperatorV1Marker>
        + DataProvider<IdsTrinaryOperatorV1Marker>
        + DataProvider<IdContinueV1Marker>
        + DataProvider<IdStartV1Marker>
        + DataProvider<IdeographicV1Marker>
        + DataProvider<JoinControlV1Marker>
        + DataProvider<LogicalOrderExceptionV1Marker>
        + DataProvider<LowercaseV1Marker>
        + DataProvider<MathV1Marker>
        + DataProvider<NoncharacterCodePointV1Marker>
        + DataProvider<PatternSyntaxV1Marker>
        + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<QuotationMarkV1Marker>
        + DataProvider<RadicalV1Marker>
        + DataProvider<RegionalIndicatorV1Marker>
        + DataProvider<SentenceTerminalV1Marker>
        + DataProvider<SoftDottedV1Marker>
        + DataProvider<TerminalPunctuationV1Marker>
        + DataProvider<UnifiedIdeographV1Marker>
        + DataProvider<UppercaseV1Marker>
        + DataProvider<VariationSelectorV1Marker>
        + DataProvider<WhiteSpaceV1Marker>
        + DataProvider<XidContinueV1Marker>
        + DataProvider<GeneralCategoryMaskNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<ScriptWithExtensionsPropertyV1Marker>
        + DataProvider<XidStartV1Marker>,
{
    fn new_internal(
        iter: &'b mut Peekable<CharIndices<'a>>,
        variable_map: &'parse VariableMap<'var>,
        xid_start: &'parse CodePointInversionList<'parse>,
        xid_continue: &'parse CodePointInversionList<'parse>,
        provider: &'c P,
    ) -> Self {
        UnicodeSetBuilder {
            single_set: CodePointInversionListBuilder::new(),
            multi_set: Default::default(),
            iter,
            inverted: false,
            variable_map,
            xid_start,
            xid_continue,
            property_provider: provider,
        }
    }

    // the entry point, parses a full UnicodeSet. ignores remaining input
    fn parse_unicode_set(&mut self) -> Result<()> {
        match self.must_peek_char()? {
            '\\' => self.parse_property_perl(),
            '[' => {
                self.iter.next();
                if let Some(':') = self.peek_char() {
                    self.parse_property_posix()
                } else {
                    self.parse_unicode_set_inner()
                }
            }
            '$' => {
                // must be variable ref to a UnicodeSet
                let (offset, v) = self.parse_variable()?;
                match v {
                    VarOrAnchor::V(VariableValue::UnicodeSet(s)) => {
                        self.single_set.add_set(s.code_points());
                        self.multi_set
                            .extend(s.strings().iter().map(ToString::to_string));
                        Ok(())
                    }
                    VarOrAnchor::V(_) => Err(PEK::UnexpectedVariable.with_offset(offset)),
                    VarOrAnchor::A => Err(PEK::UnexpectedChar('$').with_offset(offset)),
                }
            }
            c => self.error_here(PEK::UnexpectedChar(c)),
        }
    }

    // beginning [ is already consumed
    fn parse_unicode_set_inner(&mut self) -> Result<()> {
        // special cases for the first chars after [
        match self.must_peek_char()? {
            '^' => {
                self.iter.next();
                self.inverted = true;
            }
            _ => {}
        }
        // whitespace allowed between ^ and - in `[^ - ....]`
        self.skip_whitespace();
        match self.must_peek_char()? {
            '-' => {
                self.iter.next();
                self.single_set.add_char('-');
            }
            _ => {}
        }

        // repeatedly parse the following:
        // char
        // char-char
        // {multi}
        // unicodeset
        // & and - operators, but only between unicodesets
        // $variables in place of strings, chars, or unicodesets

        #[derive(Debug, Clone, Copy)]
        enum State {
            // a state equivalent to the beginning
            Begin,
            // a state after a char. implies `prev_char` is Some(_), because we need to buffer it
            // in case it is part of a range, e.g., a-z
            Char,
            // in the middle of parsing a range. implies `prev_char` is Some(_), and the next
            // element must be a char as well
            CharMinus,
            // state directly after parsing a recursive unicode set. operators are only allowed
            // in this state
            AfterUnicodeSet,
            // state directly after parsing an operator. forces the next element to be a recursive
            // unicode set
            AfterOp,
            // state after parsing a $ (that was not a variable reference)
            // the only valid next option is a closing bracket
            AfterDollar,
        }
        use State::*;

        const DEFAULT_OP: Operation = Operation::Union;

        let mut state = Begin;
        let mut prev_char = None;
        let mut operation = Operation::Union;

        loop {
            self.skip_whitespace();

            // for error messages
            let (immediate_offset, immediate_char) = self.must_peek()?;
            // get next MainToken
            let (offset, tok) = self.parse_main_token()?;

            use CharOrString as CS;
            use MainToken as MT;
            match (state, tok) {
                // the end of this unicode set
                (Begin | Char | AfterUnicodeSet | AfterDollar, MT::ClosingBracket) => {
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }

                    return Ok(());
                }
                // inner unicode set
                (Begin | Char | AfterUnicodeSet | AfterOp, MT::UnicodeSet(set)) => {
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }

                    self.process_chars(operation, set.code_points().clone());
                    self.process_strings(
                        operation,
                        set.strings().iter().map(ToString::to_string).collect(),
                    );

                    operation = DEFAULT_OP;
                    state = AfterUnicodeSet;
                }
                // a literal char or string (either individually or as the start of a range if char)
                (Begin | Char | AfterUnicodeSet, MT::CharOrString(CS::Char(c))) => {
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }
                    prev_char = Some(c);
                    state = Char;
                }
                // parse a multi-codepoint-sequence (length != 1, by CharOrString invariant)
                (Begin | Char | AfterUnicodeSet, MT::CharOrString(CS::String(s))) => {
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }

                    self.multi_set.insert(s);
                    state = Begin;
                }
                // parse a literal char as the end of a range
                (CharMinus, MT::CharOrString(CS::Char(end))) => {
                    let start = prev_char.ok_or(PEK::Internal.with_offset(offset))?;
                    if start > end {
                        // TODO(#3558): Better error message (e.g., "start greater than end in range")?
                        return Err(
                            PEK::UnexpectedChar(end).with_offset(offset)
                        );
                    }

                    self.single_set.add_range(&(start..=end));
                    prev_char = None;
                    state = Begin;
                }
                // start parsing a char range
                (Char, MT::Minus) => {
                    state = CharMinus;
                }
                // start parsing a unicode set difference
                (AfterUnicodeSet, MT::Minus) => {
                    operation = Operation::Difference;
                    state = AfterOp;
                }
                // start parsing a unicode set difference
                (AfterUnicodeSet, MT::Ampersand) => {
                    operation = Operation::Intersection;
                    state = AfterOp;
                }
                (Begin | Char | AfterUnicodeSet, MT::DollarSign) => {
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }
                    self.single_set.add_char('\u{FFFF}');
                    state = AfterDollar;
                }
                (_, c) => return Err(PEK::UnexpectedChar(immediate_char).with_offset(immediate_offset)),
            }
        }
    }

    fn parse_main_token(&mut self) -> Result<(usize, MainToken<'var>)> {
        let (initial_offset, first) = self.must_peek()?;
        if first == ']' {
            self.iter.next();
            return Ok((initial_offset, MainToken::ClosingBracket));
        }
        let (_, second) = self.must_peek_double()?;
        match (first, second) {
            // variable or anchor
            ('$', _) => {
                let (offset, var_or_anchor) = self.parse_variable()?;
                match var_or_anchor {
                    VarOrAnchor::A => Ok((offset, MainToken::DollarSign)),
                    VarOrAnchor::V(&VariableValue::Char(c)) => Ok((offset, MainToken::CharOrString(c.into()))),
                    VarOrAnchor::V(VariableValue::String(s)) => {
                        // .into() handles 1-length-string -> char conversion
                        Ok((offset, MainToken::CharOrString(s.clone().into())))
                    },
                    VarOrAnchor::V(VariableValue::UnicodeSet(set)) => Ok((offset, MainToken::UnicodeSet(set.clone()))),
                }
            }
            // string
            ('{', _) => {
                self.parse_string().map(|(offset, cs)| (offset, MainToken::CharOrString(cs)))
            }
            // inner set
            ('\\', 'p' | 'P') | ('[', _) => {
                let mut inner_builder = UnicodeSetBuilder::new_internal(
                    self.iter,
                    self.variable_map,
                    // zero-copy clone
                    self.xid_start,
                    // zero-copy clone
                    self.xid_continue,
                    self.property_provider,
                );
                inner_builder.parse_unicode_set()?;
                let (single, multi) = inner_builder.finalize();
                // note: offset - 1, because we already consumed full set
                let offset = self.must_peek_index()? - 1;
                let cpilasl = CodePointInversionListAndStringList::try_from(single.build(), VarZeroVec::from(&multi.into_iter().collect::<Vec<_>>())).map_err(|_| PEK::Internal.with_offset(offset))?;
                Ok((offset, MainToken::UnicodeSet(cpilasl)))
            }
            (c, _) if legal_char_start(c) => {
                self.parse_char().map(|(offset, cs)| (offset, MainToken::CharOrString(cs)))
            }
            ('-', _) => {
                self.iter.next();
                Ok((initial_offset, MainToken::Minus))
            }
            ('&', _) => {
                self.iter.next();
                Ok((initial_offset, MainToken::Ampersand))
            }
            (c, _) => Err(PEK::UnexpectedChar(c).with_offset(initial_offset)),
        }
    }

    // parses a variable or an anchor. expects '$' as next token.
    // is 'context-sensitive' to avoid duplicate work
    // if this is a trailing $ (eg [.... $ ]), then this function returns Ok(Some((offset, VarOrAnchor::A)))
    fn parse_variable(&mut self) -> Result<(usize, VarOrAnchor<'var, 'parse>)> {
        self.consume('$')?;

        let mut res = String::new();
        let (mut var_offset, first_c) = self.must_peek()?;

        if !self.xid_start.contains(first_c) {
            // -1 because we already consumed the '$'
            return Ok((var_offset - 1, VarOrAnchor::A));
        }

        res.push(first_c);
        self.iter.next();
        // important: if we are parsing a root unicodeset as a variable, we might reach EOF as
        // a valid end of the variable name, so we cannot use must_peek here.
        while let Some(&(offset, c)) = self.iter.peek() {
            if !self.xid_continue.contains(c) {
                break;
            }
            // only update the offset if we're adding a new char to our variable
            var_offset = offset;
            self.iter.next();
            res.push(c);
        }

        if let Some(v) = self.variable_map.get(&res) {
            return Ok((var_offset, VarOrAnchor::V(v)));
        }

        Err(PEK::UnknownVariable.with_offset(var_offset))
    }

    // parses and consumes: '{' (s charInString)* s '}'
    fn parse_string(&mut self) -> Result<(usize, CharOrString)> {
        self.consume('{')?;

        let mut buffer = String::new();
        let mut last_offset;

        loop {
            self.skip_whitespace();
            last_offset = self.must_peek_index()?;
            match self.must_peek_char()? {
                '}' => {
                    self.iter.next();
                    break;
                }
                c if legal_char_in_string_start(c) => {
                    // don't need the offset, because '}' will always be the last char
                    let (_, c_or_s) = self.parse_char()?;
                    match c_or_s {
                        CharOrString::Char(c) => buffer.push(c),
                        CharOrString::String(s) => buffer.push_str(&s),
                    }
                }
                c => return self.error_here(PEK::UnexpectedChar(c)),
            }
        }

        // .into() handles 1-length-string -> char conversion
        Ok((last_offset, buffer.into()))
    }

    // parses and consumes '{' (s char)+ s '}'
    // TODO: decide on names for multi-codepoint-sequences and adjust both struct fields and fn names
    fn parse_multi(&mut self) -> Result<()> {
        self.consume('{')?;

        let mut buffer = String::new();

        loop {
            self.skip_whitespace();

            match self.must_peek_char()? {
                '}' => {
                    self.iter.next();
                    break;
                }
                c if legal_char_in_string_start(c) => {
                    let c = self.parse_char()?;
                    buffer.push(c);
                }
                c => return self.error_here(PEK::UnexpectedChar(c)),
            }
        }

        let mut chars = buffer.chars();
        match (chars.next(), chars.next()) {
            (Some(single_char), None) => {
                // multi-codepoint-sequences containing a single char are interpreted as a single char
                self.single_set.add_char(single_char);
            }
            _ => {
                self.multi_set.insert(buffer);
            }
        }

        Ok(())
    }

    // starts with \ and consumes the whole escape sequence
    // TODO(#3557): Multi-code-point escapes. Would mean that this function could return either a char or a String.
    fn parse_escaped_char(&mut self) -> Result<(usize, CharOrString)> {
        self.consume('\\')?;

        let (offset, next_char) = self.must_next()?;

        match next_char {
            'u' | 'x' if self.peek_char() == Some('{') => {
                // bracketedHex
                self.iter.next();
                self.skip_whitespace();
                let (hex_digits, end_offset) = self.parse_variable_length_hex()?;
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                let c =
                    char::try_from(num).map_err(|_| PEK::InvalidEscape.with_offset(end_offset))?;
                self.skip_whitespace();
                let offset = self.must_peek_index()?;
                self.consume('}')?;
                Ok((offset, c.into()))
            }
            'u' => {
                // 'u' hex{4}
                let (offset, exact) = self.parse_exact_hex_digits::<4>()?;
                let hex_digits = exact.iter().collect::<String>();
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                char::try_from(num).map(|c| (offset, c.into())).map_err(|_| PEK::InvalidEscape.with_offset(offset))
            }
            'x' => {
                // 'x' hex{2}
                let (offset, exact) = self.parse_exact_hex_digits::<2>()?;
                let hex_digits = exact.iter().collect::<String>();
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                char::try_from(num).map(|c| (offset, c.into())).map_err(|_| PEK::InvalidEscape.with_offset(offset))
            }
            'U' => {
                // 'U00' ('0' hex{5} | '10' hex{4})
                self.consume('0')?;
                self.consume('0')?;
                let (offset, hex_digits) = match self.must_peek_char()? {
                    '0' => {
                        self.iter.next();
                        let (offset, exact) = self.parse_exact_hex_digits::<5>()?;
                        (offset, exact.iter().collect::<String>())
                    }
                    '1' => {
                        self.iter.next();
                        self.consume('0')?;
                        let (offset, exact) = self.parse_exact_hex_digits::<4>()?;
                        (offset, ['1', '0'].iter().chain(exact.iter()).collect::<String>())
                    }
                    c => return self.error_here(PEK::UnexpectedChar(c)),
                };
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                char::try_from(num).map(|c| (offset, c.into())).map_err(|_| PEK::InvalidEscape.with_offset(offset))
            }
            'N' => {
                // parse code point with name in {}
                // tracking issue: https://github.com/unicode-org/icu4x/issues/1397
                Err(PEK::Unimplemented.with_offset(offset))
            }
            'a' => Ok((offset, '\u{0007}'.into())),
            'b' => Ok((offset, '\u{0008}'.into())),
            't' => Ok((offset, '\u{0009}'.into())),
            'n' => Ok((offset, '\u{000A}'.into())),
            'v' => Ok((offset, '\u{000B}'.into())),
            'f' => Ok((offset, '\u{000C}'.into())),
            'r' => Ok((offset, '\u{000D}'.into())),
            _ => Ok((offset, next_char.into())),
        }
    }

    // starts with :, consumes the trailing :]
    fn parse_property_posix(&mut self) -> Result<()> {
        self.consume(':')?;
        if self.must_peek_char()? == '^' {
            self.inverted = true;
            self.iter.next();
        }

        self.parse_property_inner(':')?;

        self.consume(']')?;

        Ok(())
    }

    // starts with \p{ or \P{, consumes the trailing }
    fn parse_property_perl(&mut self) -> Result<()> {
        self.consume('\\')?;
        match self.must_next()? {
            (_, 'p') => {}
            (_, 'P') => self.inverted = true,
            (offset, c) => return Err(PEK::UnexpectedChar(c).with_offset(offset)),
        }
        self.consume('{')?;

        self.parse_property_inner('}')?;

        Ok(())
    }

    fn parse_property_inner(&mut self, end: char) -> Result<()> {
        // only supports ECMA-262. UnicodeSet spec ignores whitespace, '-', and '_',
        // but ECMA-262 requires '_', so we'll allow that.
        // TODO(#3559): support loose matching on property names (e.g., "AS  -_-  CII_Hex_ D-igit")
        // TODO(#3559): support more properties than ECMA-262

        let property_offset;

        let mut key_buffer = String::new();
        let mut value_buffer = String::new();

        enum State {
            // initial state, nothing parsed yet
            Begin,
            // non-empty property name
            PropertyName,
            // property name parsed, '=' or '≠' parsed, no value parsed yet
            PropertyValueBegin,
            // non-empty property name, non-empty property value
            PropertyValue,
        }
        use State::*;

        let mut state = Begin;
        // whether '=' (true) or '≠' (false) was parsed
        let mut equality = true;

        loop {
            self.skip_whitespace();
            match (state, self.must_peek_char()?) {
                // parse the end of the property expression
                (PropertyName | PropertyValue, c) if c == end => {
                    // byte index of (full) property name/value is one back
                    property_offset = self.must_peek_index()? - 1;
                    self.iter.next();
                    break;
                }
                // parse the property name
                // NOTE: this might be too strict, because in the case of e.g. [:value:], we might want to
                // allow [:lower-case-letter:] ([:gc=lower-case-letter:] works)
                (Begin | PropertyName, c) if c.is_ascii_alphanumeric() || c == '_' => {
                    key_buffer.push(c);
                    self.iter.next();
                    state = PropertyName;
                }
                // parse the name-value separator
                (PropertyName, c @ ('=' | '≠')) => {
                    equality = c == '=';
                    self.iter.next();
                    state = PropertyValueBegin;
                }
                // parse the property value
                (PropertyValue | PropertyValueBegin, c) if c != end => {
                    value_buffer.push(c);
                    self.iter.next();
                    state = PropertyValue;
                }
                (_, c) => return self.error_here(PEK::UnexpectedChar(c)),
            }
        }

        if !equality {
            self.inverted = !self.inverted;
        }

        let inverted = self
            .load_property_codepoints(&key_buffer, &value_buffer)
            // any error that does not already have an offset should use the appropriate property offset
            .map_err(|e| e.or_with_offset(property_offset))?;
        if inverted {
            self.inverted = !self.inverted;
        }

        Ok(())
    }

    // returns whether the set needs to be inverted or not
    fn load_property_codepoints(&mut self, key: &str, value: &str) -> Result<bool> {
        // we support:
        // [:gc = value:]
        // [:sc = value:]
        // [:scx = value:]
        // [:value:] - looks up value in gc, sc
        // [:prop:] - binary property, returns codepoints that have the property
        // [:prop = truthy/falsy:] - same as above

        let mut inverted = false;

        // contains a value for the General_Category property that needs to be tried
        let mut try_gc = Err(PEK::UnknownProperty.into());
        // contains a value for the Script property that needs to be tried
        let mut try_sc = Err(PEK::UnknownProperty.into());
        // contains a value for the Script_Extensions property that needs to be tried
        let mut try_scx = Err(PEK::UnknownProperty.into());
        // contains a supposed binary property name that needs to be tried
        let mut try_binary = Err(PEK::UnknownProperty.into());

        if !value.is_empty() {
            // key is gc, sc, scx
            // value is a property value
            // OR
            // key is a binary property and value is a truthy/falsy value

            match key {
                "General_Category" | "gc" => try_gc = Ok(value),
                "Script" | "sc" => try_sc = Ok(value),
                "Script_Extensions" | "scx" => try_scx = Ok(value),
                _ => {
                    let normalized_value = value.to_ascii_lowercase();
                    let truthy = matches!(normalized_value.as_str(), "true" | "t" | "yes" | "y");
                    let falsy = matches!(normalized_value.as_str(), "false" | "f" | "no" | "n");
                    // value must either match truthy or falsy
                    if truthy == falsy {
                        return Err(PEK::UnknownProperty.into());
                    }
                    // correctness: if we reach this point, only `try_binary` can be Ok, hence
                    // it does not matter that further down we unconditionally return `inverted`,
                    // because only `try_binary` can enter that code path.
                    inverted = falsy;
                    try_binary = Ok(key);
                }
            }
        } else {
            // key is binary property
            // OR a value of gc, sc (only gc or sc are supported as implicit keys by UTS35!)
            try_gc = Ok(key);
            try_sc = Ok(key);
            try_binary = Ok(key);
        }

        try_gc
            .and_then(|value| self.try_load_general_category_set(value))
            .or_else(|_| try_sc.and_then(|value| self.try_load_script_set(value)))
            .or_else(|_| try_scx.and_then(|value| self.try_load_script_extensions_set(value)))
            .or_else(|_| try_binary.and_then(|value| self.try_load_ecma262_binary_set(value)))?;
        Ok(inverted)
    }

    fn finalize(mut self) -> (CodePointInversionListBuilder, HashSet<String>) {
        if self.inverted {
            // code point inversion; removes all strings
            if !self.multi_set.is_empty() {
                log::info!(
                    "Inverting a unicode set with strings. This removes all strings entirely."
                );
            }
            self.multi_set.drain();
            self.single_set.complement();
        }

        (self.single_set, self.multi_set)
    }

    fn peek_unicode_set_start(&mut self) -> bool {
        match self.peek_char() {
            Some('\\') => {}
            Some('[') => return true,
            _ => return false,
        }

        // need to look one more char into the future. Peekable doesnt lend itself well to this,
        // so maybe think about using a different iterator internally
        let mut future = self.iter.clone();
        future.next();

        match future.peek() {
            // perl property
            Some(&(_, 'p' | 'P')) => true,
            _ => false,
        }
    }

    // parses either a raw char or an escaped char. all chars are allowed, the caller must make sure to handle
    // cases where some characters are not allowed
    fn parse_char(&mut self) -> Result<(usize, CharOrString)> {
        let (offset, c) = self.must_peek()?;
        match c {
            '\\' => self.parse_escaped_char(),
            _ => {
                self.iter.next();
                Ok((offset, CharOrString::Char(c)))
            }
        }
    }

    // parses [0-9a-fA-F]{1..6}
    fn parse_variable_length_hex(&mut self) -> Result<(String, usize)> {
        let mut result = String::new();
        let mut end_offset = 0;
        while let Some(&(offset, c)) = self.iter.peek() {
            if result.len() >= 6 || !c.is_ascii_hexdigit() {
                break;
            }
            result.push(c);
            end_offset = offset;
            self.iter.next();
        }
        if result.is_empty() {
            let (unexpected_offset, unexpected_char) = self.must_peek()?;
            return Err(PEK::UnexpectedChar(unexpected_char).with_offset(unexpected_offset));
        }
        Ok((result, end_offset))
    }

    // parses [0-9a-fA-F]{N}, returns the offset of the last digit
    fn parse_exact_hex_digits<const N: usize>(&mut self) -> Result<(usize, [char; N])> {
        debug_assert!(N > 0);

        let mut result = [0 as char; N];
        // N is never zero, so the loop will always run at least once
        let mut last_offset = 0;
        for slot in result.iter_mut() {
            let (offset, c) = self.must_next()?;
            last_offset = offset;
            if !c.is_ascii_hexdigit() {
                return Err(PEK::UnexpectedChar(c).with_offset(offset));
            }
            *slot = c;
        }
        Ok((last_offset, result))
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !is_char_pattern_white_space(c) {
                break;
            }
            self.iter.next();
        }
    }

    fn consume(&mut self, expected: char) -> Result<()> {
        match self.must_next()? {
            (offset, c) if c != expected => Err(PEK::UnexpectedChar(c).with_offset(offset)),
            _ => Ok(()),
        }
    }

    // use this whenever an empty iterator would imply an Eof error
    fn must_next(&mut self) -> Result<(usize, char)> {
        self.iter.next().ok_or(PEK::Eof.into())
    }

    // use this whenever an empty iterator would imply an Eof error
    fn must_peek(&mut self) -> Result<(usize, char)> {
        self.iter.peek().copied().ok_or(PEK::Eof.into())
    }

    // must_peek, but looks two chars ahead. use sparingly
    fn must_peek_double(&mut self) -> Result<(usize, char)> {
        let mut copy = self.iter.clone();
        copy.next();
        copy.next().ok_or(PEK::Eof.into())
    }

    // see must_peek
    fn must_peek_char(&mut self) -> Result<char> {
        self.must_peek().map(|(_, c)| c)
    }

    // see must_peek
    fn must_peek_index(&mut self) -> Result<usize> {
        self.must_peek().map(|(idx, _)| idx)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.iter.peek().map(|&(_, c)| c)
    }

    // TODO: return Result<!> once ! is stable
    #[inline]
    fn error_here<T>(&mut self, kind: ParseErrorKind) -> Result<T> {
        match self.iter.peek() {
            None => Err(kind.into()),
            Some(&(offset, _)) => Err(kind.with_offset(offset)),
        }
    }

    fn process_strings(&mut self, op: Operation, other_strings: HashSet<String>) {
        match op {
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
    }

    fn process_chars(&mut self, op: Operation, other_chars: CodePointInversionList) {
        match op {
            Operation::Union => self.single_set.add_set(&other_chars),
            Operation::Difference => self.single_set.remove_set(&other_chars),
            Operation::Intersection => self.single_set.retain_set(&other_chars),
        }
    }

    fn try_load_general_category_set(&mut self, name: &str) -> Result<()> {
        // TODO(#3550): This could be cached; does not depend on name.
        let name_map = GeneralCategoryGroup::get_name_to_enum_mapper(self.property_provider)
            .map_err(|_| PEK::Internal)?;
        let gc_value = name_map
            .as_borrowed()
            .get_loose(name)
            .ok_or(PEK::UnknownProperty)?;
        // TODO(#3550): This could be cached; does not depend on name.
        let set = load_for_general_category_group(self.property_provider, gc_value)
            .map_err(|_| PEK::Internal)?;
        self.single_set.add_set(&set.to_code_point_inversion_list());
        Ok(())
    }

    fn try_get_script(&self, name: &str) -> Result<Script> {
        // TODO(#3550): This could be cached; does not depend on name.
        let name_map =
            Script::get_name_to_enum_mapper(self.property_provider).map_err(|_| PEK::Internal)?;
        name_map
            .as_borrowed()
            .get_loose(name)
            .ok_or(PEK::UnknownProperty.into())
    }

    fn try_load_script_set(&mut self, name: &str) -> Result<()> {
        let sc_value = self.try_get_script(name)?;
        // TODO(#3550): This could be cached; does not depend on name.
        let property_map = load_script(self.property_provider).map_err(|_| PEK::Internal)?;
        let set = property_map.as_borrowed().get_set_for_value(sc_value);
        self.single_set.add_set(&set.to_code_point_inversion_list());
        Ok(())
    }

    fn try_load_script_extensions_set(&mut self, name: &str) -> Result<()> {
        // TODO(#3550): This could be cached; does not depend on name.
        let scx = load_script_with_extensions_unstable(self.property_provider)
            .map_err(|_| PEK::Internal)?;
        let sc_value = self.try_get_script(name)?;
        let set = scx.as_borrowed().get_script_extensions_set(sc_value);
        self.single_set.add_set(&set);
        Ok(())
    }

    fn try_load_ecma262_binary_set(&mut self, name: &str) -> Result<()> {
        let set = load_for_ecma262_unstable(self.property_provider, name)
            .map_err(|_| PEK::UnknownProperty)?;
        self.single_set.add_set(&set.to_code_point_inversion_list());
        Ok(())
    }
}

/// Parses a UnicodeSet pattern and returns a UnicodeSet in the form of a [`CodePointInversionListAndStringList`](icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList).
///
/// Supports a subset of the syntax described in [UTS #35 - Unicode Sets](https://unicode.org/reports/tr35/#Unicode_Sets).
/// (_Note: This is technically wrong, as we do support `[]` and `[^]`, which is disallowed by UTS35,
/// as accepted syntax for the empty set and the full (code point) set, respectively._)
///
/// The error type of the returned Result can be pretty-printed with [`ParseError::fmt_with_source`].
///
/// # Limitations
///
/// * Currently, we only support the [ECMA-262 properties](https://tc39.es/ecma262/#table-nonbinary-unicode-properties) except `Script_Extensions`.
/// The property names must match the exact spelling listed in ECMA-262. Note that we do support UTS35 syntax for elided `General_Category`
/// and `Script` property names, i.e., `[:Latn:]` and `[:Ll:]` are both valid, with the former implying the `Script` property, and the latter the
/// `General_Category` property.
/// * We do not support `\N{Unicode code point name}` character escaping. Use any other escape method described in UTS35.
///
/// # Stability
///
/// [📚 Help choosing a constructor](icu_provider::constructors)
/// <div class="stab unstable">
/// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
/// </div>
///
/// # Examples
///
/// Parse ranges
/// ```
/// use icu_unicodeset_parser::parse;
///
/// let set = parse("[a-zA-Z0-9]").unwrap();
/// let code_points = set.code_points();
///
/// assert!(code_points.contains_range(&('a'..='z')));
/// assert!(code_points.contains_range(&('A'..='Z')));
/// assert!(code_points.contains_range(&('0'..='9')));
/// ```
///
/// Parse properties, set operations, inner sets
/// ```
/// use icu_unicodeset_parser::parse;
///
/// let set = parse("[[:^ll:]-[^][:gc = Lowercase Letter:]&[^[[^]-[a-z]]]]").unwrap();
/// let elements = 'a'..='z';
/// assert!(set.code_points().contains_range(&elements));
/// assert_eq!(elements.count(), set.size());
/// ```
///
/// Inversions remove strings
/// ```
/// use icu_unicodeset_parser::parse;
///
/// let set = parse(r"[[a-z{hello\ world}]&[^a-y{hello\ world}]]").unwrap();
/// assert!(set.contains_char('z'));
/// assert_eq!(set.size(), 1);
/// assert!(!set.has_strings());
/// ```
///
/// Set operators (including the implicit union) have the same precedence and are left-associative
/// ```
/// use icu_unicodeset_parser::parse;
///
/// let set = parse("[[ace][bdf] - [abc][def]]").unwrap();
/// let elements = 'd'..='f';
/// assert!(set.code_points().contains_range(&elements));
/// assert_eq!(set.size(), elements.count());
/// ```
#[cfg(feature = "compiled_data")]
pub fn parse(source: &str) -> Result<CodePointInversionListAndStringList<'static>> {
    parse_unstable(source, &icu_properties::provider::Baked)
}

#[cfg(feature = "compiled_data")]
pub fn parse_with_variables<'data>(
    source: &str,
    variable_map: &VariableMap<'data>,
) -> Result<CodePointInversionListAndStringList<'static>> {
    parse_unstable_with_variables(source, variable_map, &icu_properties::provider::Baked)
}

#[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, parse_with_variables)]
pub fn parse_unstable_with_variables<'data, P>(
    source: &str,
    variable_map: &VariableMap<'data>,
    provider: &P,
) -> Result<CodePointInversionListAndStringList<'static>>
where
    P: ?Sized
        + DataProvider<AsciiHexDigitV1Marker>
        + DataProvider<AlphabeticV1Marker>
        + DataProvider<BidiControlV1Marker>
        + DataProvider<BidiMirroredV1Marker>
        + DataProvider<CaseIgnorableV1Marker>
        + DataProvider<CasedV1Marker>
        + DataProvider<ChangesWhenCasefoldedV1Marker>
        + DataProvider<ChangesWhenCasemappedV1Marker>
        + DataProvider<ChangesWhenLowercasedV1Marker>
        + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
        + DataProvider<ChangesWhenTitlecasedV1Marker>
        + DataProvider<ChangesWhenUppercasedV1Marker>
        + DataProvider<DashV1Marker>
        + DataProvider<DefaultIgnorableCodePointV1Marker>
        + DataProvider<DeprecatedV1Marker>
        + DataProvider<DiacriticV1Marker>
        + DataProvider<EmojiV1Marker>
        + DataProvider<EmojiComponentV1Marker>
        + DataProvider<EmojiModifierV1Marker>
        + DataProvider<EmojiModifierBaseV1Marker>
        + DataProvider<EmojiPresentationV1Marker>
        + DataProvider<ExtendedPictographicV1Marker>
        + DataProvider<ExtenderV1Marker>
        + DataProvider<GraphemeBaseV1Marker>
        + DataProvider<GraphemeExtendV1Marker>
        + DataProvider<HexDigitV1Marker>
        + DataProvider<IdsBinaryOperatorV1Marker>
        + DataProvider<IdsTrinaryOperatorV1Marker>
        + DataProvider<IdContinueV1Marker>
        + DataProvider<IdStartV1Marker>
        + DataProvider<IdeographicV1Marker>
        + DataProvider<JoinControlV1Marker>
        + DataProvider<LogicalOrderExceptionV1Marker>
        + DataProvider<LowercaseV1Marker>
        + DataProvider<MathV1Marker>
        + DataProvider<NoncharacterCodePointV1Marker>
        + DataProvider<PatternSyntaxV1Marker>
        + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<QuotationMarkV1Marker>
        + DataProvider<RadicalV1Marker>
        + DataProvider<RegionalIndicatorV1Marker>
        + DataProvider<SentenceTerminalV1Marker>
        + DataProvider<SoftDottedV1Marker>
        + DataProvider<TerminalPunctuationV1Marker>
        + DataProvider<UnifiedIdeographV1Marker>
        + DataProvider<UppercaseV1Marker>
        + DataProvider<VariationSelectorV1Marker>
        + DataProvider<WhiteSpaceV1Marker>
        + DataProvider<XidContinueV1Marker>
        + DataProvider<GeneralCategoryMaskNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<ScriptWithExtensionsPropertyV1Marker>
        + DataProvider<XidStartV1Marker>,
{
    // TODO(#3550): Add function "parse_overescaped" that uses a custom iterator to de-overescape (i.e., maps \\ to \) on-the-fly?
    // ^ will likely need a different iterator type on UnicodeSetBuilder
    // TODO(#3550): Think about returning byte-length of the parsed UnicodeSet for use in the transliterator parser, or add public function that accepts a peekable char iterator?

    let mut iter = source.char_indices().peekable();

    let xid_start = load_xid_start(provider).map_err(|_| PEK::Internal)?;
    let xid_start_list = xid_start.to_code_point_inversion_list();
    let xid_continue = load_xid_continue(provider).map_err(|_| PEK::Internal)?;
    let xid_continue_list = xid_continue.to_code_point_inversion_list();

    let mut builder = UnicodeSetBuilder::new_internal(
        &mut iter,
        variable_map,
        &xid_start_list,
        &xid_continue_list,
        provider,
    );

    builder.parse_unicode_set()?;
    let (single, multi) = builder.finalize();
    let built_single = single.build();

    let mut strings = multi.into_iter().collect::<Vec<_>>();
    strings.sort();
    let zerovec = (&strings).into();

    let cpinvlistandstrlist = CodePointInversionListAndStringList::try_from(built_single, zerovec)
        .map_err(|_| PEK::Internal)?;

    Ok(cpinvlistandstrlist)
}

#[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, parse)]
pub fn parse_unstable<P>(
    source: &str,
    provider: &P,
) -> Result<CodePointInversionListAndStringList<'static>>
where
    P: ?Sized
        + DataProvider<AsciiHexDigitV1Marker>
        + DataProvider<AlphabeticV1Marker>
        + DataProvider<BidiControlV1Marker>
        + DataProvider<BidiMirroredV1Marker>
        + DataProvider<CaseIgnorableV1Marker>
        + DataProvider<CasedV1Marker>
        + DataProvider<ChangesWhenCasefoldedV1Marker>
        + DataProvider<ChangesWhenCasemappedV1Marker>
        + DataProvider<ChangesWhenLowercasedV1Marker>
        + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
        + DataProvider<ChangesWhenTitlecasedV1Marker>
        + DataProvider<ChangesWhenUppercasedV1Marker>
        + DataProvider<DashV1Marker>
        + DataProvider<DefaultIgnorableCodePointV1Marker>
        + DataProvider<DeprecatedV1Marker>
        + DataProvider<DiacriticV1Marker>
        + DataProvider<EmojiV1Marker>
        + DataProvider<EmojiComponentV1Marker>
        + DataProvider<EmojiModifierV1Marker>
        + DataProvider<EmojiModifierBaseV1Marker>
        + DataProvider<EmojiPresentationV1Marker>
        + DataProvider<ExtendedPictographicV1Marker>
        + DataProvider<ExtenderV1Marker>
        + DataProvider<GraphemeBaseV1Marker>
        + DataProvider<GraphemeExtendV1Marker>
        + DataProvider<HexDigitV1Marker>
        + DataProvider<IdsBinaryOperatorV1Marker>
        + DataProvider<IdsTrinaryOperatorV1Marker>
        + DataProvider<IdContinueV1Marker>
        + DataProvider<IdStartV1Marker>
        + DataProvider<IdeographicV1Marker>
        + DataProvider<JoinControlV1Marker>
        + DataProvider<LogicalOrderExceptionV1Marker>
        + DataProvider<LowercaseV1Marker>
        + DataProvider<MathV1Marker>
        + DataProvider<NoncharacterCodePointV1Marker>
        + DataProvider<PatternSyntaxV1Marker>
        + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<QuotationMarkV1Marker>
        + DataProvider<RadicalV1Marker>
        + DataProvider<RegionalIndicatorV1Marker>
        + DataProvider<SentenceTerminalV1Marker>
        + DataProvider<SoftDottedV1Marker>
        + DataProvider<TerminalPunctuationV1Marker>
        + DataProvider<UnifiedIdeographV1Marker>
        + DataProvider<UppercaseV1Marker>
        + DataProvider<VariationSelectorV1Marker>
        + DataProvider<WhiteSpaceV1Marker>
        + DataProvider<XidContinueV1Marker>
        + DataProvider<GeneralCategoryMaskNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<ScriptWithExtensionsPropertyV1Marker>
        + DataProvider<XidStartV1Marker>,
{
    // TODO: avoid allocation. Maybe use some map on the stack, or some wrapper enum (enum MapOrEmpty { Empty, Map(Map) })
    let dummy = Default::default();
    parse_unstable_with_variables(source, &dummy, provider)
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::*;

    // "aabxzz" => [a..=a, b..=x, z..=z]
    fn range_iter_from_str(s: &str) -> impl Iterator<Item = RangeInclusive<u32>> {
        debug_assert_eq!(
            s.chars().count() % 2,
            0,
            "string \"{}\" does not contain an even number of code points",
            s.escape_debug()
        );
        let mut res = vec![];
        let mut skip = false;
        for (a, b) in s.chars().zip(s.chars().skip(1)) {
            if skip {
                skip = false;
                continue;
            }
            let a = a as u32;
            let b = b as u32;
            res.push(a..=b);
            skip = true;
        }

        res.into_iter()
    }

    fn assert_set_equality<'a>(
        source: &str,
        cpinvlistandstrlist: &CodePointInversionListAndStringList,
        single: impl Iterator<Item = RangeInclusive<u32>>,
        multi: impl Iterator<Item = &'a str>,
    ) {
        let expected_ranges: HashSet<_> = single.collect();
        let actual_ranges: HashSet<_> = cpinvlistandstrlist.code_points().iter_ranges().collect();
        assert_eq!(
            actual_ranges,
            expected_ranges,
            "got unexpected ranges {:?}, expected {:?} for parsed set \"{}\"",
            actual_ranges,
            expected_ranges,
            source.escape_debug()
        );
        let mut expected_size = cpinvlistandstrlist.code_points().size();
        for s in multi {
            expected_size += 1;
            assert!(
                cpinvlistandstrlist.contains(s),
                "missing string \"{}\" from parsed set \"{}\"",
                s.escape_debug(),
                source.escape_debug()
            );
        }
        let actual_size = cpinvlistandstrlist.size();
        assert_eq!(
            actual_size,
            expected_size,
            "got unexpected size {}, expected {} for parsed set \"{}\"",
            actual_size,
            expected_size,
            source.escape_debug()
        );
    }

    fn assert_is_error_and_message_eq(source: &str, expected_err: &str, vm: &VariableMap<'_>) {
        let result = parse_with_variables(source, vm);
        assert!(result.is_err(), "{source} does not cause an error!");
        let err = result.unwrap_err();
        assert_eq!(err.fmt_with_source(source).to_string(), expected_err);
    }

    #[test]
    fn test_semantics_with_variables() {
        let map_char_char = HashMap::from([
            ("a".to_string(), VariableValue::Char('a')),
            ("var2".to_string(), VariableValue::Char('z')),
        ]);

        let map_headache = HashMap::from([("hehe".to_string(), VariableValue::Char('-'))]);

        let map_char_string = HashMap::from([
            ("a".to_string(), VariableValue::Char('a')),
            ("var2".to_string(), VariableValue::String("abc".into())),
        ]);

        let set = parse(r"[a-z {Hello,\ World!}]").unwrap();
        let map_char_set = HashMap::from([
            ("a".to_string(), VariableValue::Char('a')),
            ("set".to_string(), VariableValue::UnicodeSet(set)),
        ]);

        let cases: Vec<(_, _, _, Vec<&str>)> = vec![
            // simple
            (&map_char_char, "[$a]", "aa", vec![]),
            (&map_char_char, "[ $a ]", "aa", vec![]),
            (&map_char_char, "[$a$]", "aa\u{ffff}\u{ffff}", vec![]),
            (&map_char_char, "[$a$ ]", "aa\u{ffff}\u{ffff}", vec![]),
            (&map_char_char, "[$a$var2]", "aazz", vec![]),
            (&map_char_char, "[$a - $var2]", "az", vec![]),
            (&map_char_char, "[$a-$var2]", "az", vec![]),
            (&map_headache, "[a $hehe z]", "aazz--", vec![]),
            (
                &map_char_char,
                "[[$]var2]]",
                "\u{ffff}\u{ffff}vvaarr22",
                vec![],
            ),
            // variable prefix escaping
            (&map_char_char, r"[\$var2]", "$$vvaarr22", vec![]),
            (&map_char_char, r"[\\$var2]", r"\\zz", vec![]),
            // no variable dereferencing in strings
            (&map_char_char, "[{$a}]", "", vec!["$a"]),
            // set operations
            (&map_char_set, "[$set & [b-z]]", "bz", vec![]),
            (&map_char_set, "[[a-z]-[b-z]]", "aa", vec![]),
            (&map_char_set, "[$set-[b-z]]", "aa", vec!["Hello, World!"]),
            (&map_char_set, "[$set-$set]", "", vec![]),
            (&map_char_set, "[[a-zA]-$set]", "AA", vec![]),
            (&map_char_set, "[$set[b-z]]", "az", vec!["Hello, World!"]),
            (&map_char_set, "[[a-a]$set]", "az", vec!["Hello, World!"]),
            (&map_char_set, "$set", "az", vec!["Hello, World!"]),
            // strings
            (&map_char_string, "[$var2]", "", vec!["abc"]),
        ];
        for (variable_map, source, single, multi) in cases {
            let parsed = parse_with_variables(source, variable_map);
            if let Err(err) = parsed {
                assert!(
                    false,
                    "{source} results in an error: {}",
                    err.fmt_with_source(source)
                );
                continue;
            }
            let parsed = parsed.unwrap();
            assert_set_equality(
                source,
                &parsed,
                range_iter_from_str(single),
                multi.into_iter(),
            );
        }
    }

    #[test]
    fn test_semantics() {
        const ALL_CHARS: &str = "\x00\u{10FFFF}";
        let cases: Vec<(_, _, Vec<&str>)> = vec![
            // simple
            ("[a]", "aa", vec![]),
            ("[]", "", vec![]),
            ("[qax]", "aaqqxx", vec![]),
            ("[a-z]", "az", vec![]),
            // whitespace escaping
            (r"[\n]", "\n\n", vec![]),
            ("[\\\n]", "\n\n", vec![]),
            // empty - whitespace is skipped
            ("[\n]", "", vec![]),
            ("[\u{9}]", "", vec![]),
            ("[\u{A}]", "", vec![]),
            ("[\u{B}]", "", vec![]),
            ("[\u{C}]", "", vec![]),
            ("[\u{D}]", "", vec![]),
            ("[\u{20}]", "", vec![]),
            ("[\u{85}]", "", vec![]),
            ("[\u{200E}]", "", vec![]),
            ("[\u{200F}]", "", vec![]),
            ("[\u{2028}]", "", vec![]),
            ("[\u{2029}]", "", vec![]),
            // but not all "whitespace", only Pattern_White_Space:
            ("[\u{A0}]", "\u{A0}\u{A0}", vec![]), // non-breaking space
            // anchor
            ("[$]", "\u{ffff}\u{ffff}", vec![]),
            (r"[\$]", "$$", vec![]),
            ("[{$}]", "$$", vec![]),
            // set operations
            ("[[a-z]&[b-z]]", "bz", vec![]),
            ("[[a-z]-[b-z]]", "aa", vec![]),
            ("[[a-z][b-z]]", "az", vec![]),
            ("[[a-a][b-z]]", "az", vec![]),
            ("[[a-z{abc}]&[b-z{abc}{abx}]]", "bz", vec!["abc"]),
            ("[[{abx}a-z{abc}]&[b-z{abc}]]", "bz", vec!["abc"]),
            ("[[a-z{abx}]-[{abx}b-z{abc}]]", "aa", vec![]),
            ("[[a-z{abx}{abc}]-[{abx}b-z]]", "aa", vec!["abc"]),
            ("[[a-z{abc}][b-z{abx}]]", "az", vec!["abc", "abx"]),
            // strings
            ("[{this is a minus -}]", "", vec!["thisisaminus-"]),
            // associativity
            ("[[a-a][b-z] - [a-d][e-z]]", "ez", vec![]),
            ("[[a-a][b-z] - [a-d]&[e-z]]", "ez", vec![]),
            ("[[a-a][b-z] - [a-z][]]", "", vec![]),
            ("[[a-a][b-z] - [a-z]&[]]", "", vec![]),
            ("[[a-a][b-z] & [a-z]-[]]", "az", vec![]),
            ("[[a-a][b-z] & []-[a-z]]", "", vec![]),
            ("[[a-a][b-z] & [a-b][x-z]]", "abxz", vec![]),
            ("[[a-z]-[a-b]-[y-z]]", "cx", vec![]),
            // escape tests
            (r"[\x61-\x63]", "ac", vec![]),
            (r"[a-\x63]", "ac", vec![]),
            (r"[\x61-c]", "ac", vec![]),
            (r"[\u0061-\x63]", "ac", vec![]),
            (r"[\U00000061-\x63]", "ac", vec![]),
            (r"[\x{61}-\x63]", "ac", vec![]),
            (r"[\u{61}-\x63]", "ac", vec![]),
            (r"[\u{61}{hello\ world}]", "aa", vec!["hello world"]),
            (r"[{hello\ world}\u{61}]", "aa", vec!["hello world"]),
            (r"[{h\u{65}llo\ world}]", "", vec!["hello world"]),
            // complement tests
            (r"[^]", ALL_CHARS, vec![]),
            (r"[[^]-[^a-z]]", "az", vec![]),
            (r"[^{h\u{65}llo\ world}]", ALL_CHARS, vec![]),
            (
                r"[^[{h\u{65}llo\ world}]-[{hello\ world}]]",
                ALL_CHARS,
                vec![],
            ),
            (
                r"[^[\x00-\U0010FFFF]-[\u0100-\U0010FFFF]]",
                "\u{100}\u{10FFFF}",
                vec![],
            ),
            (r"[^[^a-z]]", "az", vec![]),
            (r"[^[^\^]]", "^^", vec![]),
            // binary properties
            (r"[:AHex:]", "09afAF", vec![]),
            (r"[:AHex=True:]", "09afAF", vec![]),
            (r"[:AHex=T:]", "09afAF", vec![]),
            (r"[:AHex=Yes:]", "09afAF", vec![]),
            (r"[:AHex=Y:]", "09afAF", vec![]),
            (r"[:^AHex≠True:]", "09afAF", vec![]),
            (r"[:AHex≠False:]", "09afAF", vec![]),
            (r"[[:^AHex≠False:]&[\x00-\x10]]", "\0\x10", vec![]),
            (r"\p{AHex}", "09afAF", vec![]),
            (r"\p{AHex=True}", "09afAF", vec![]),
            (r"\p{AHex=T}", "09afAF", vec![]),
            (r"\p{AHex=Yes}", "09afAF", vec![]),
            (r"\p{AHex=Y}", "09afAF", vec![]),
            (r"\P{AHex≠True}", "09afAF", vec![]),
            (r"\p{AHex≠False}", "09afAF", vec![]),
            // general category
            (r"[[:gc=lower-case-letter:]&[a-zA-Z]]", "az", vec![]),
            (r"[[:lower case letter:]&[a-zA-Z]]", "az", vec![]),
            // general category groups
            // equivalence between L and the union of all the L* categories
            (
                r"[[[:L:]-[\p{Ll}\p{Lt}\p{Lu}\p{Lo}\p{Lm}]][[\p{Ll}\p{Lt}\p{Lu}\p{Lo}\p{Lm}]-[:L:]]]",
                "",
                vec![],
            ),
            // script
            (r"[[:sc=latn:]&[a-zA-Z]]", "azAZ", vec![]),
            (r"[[:sc=Latin:]&[a-zA-Z]]", "azAZ", vec![]),
            (r"[[:Latin:]&[a-zA-Z]]", "azAZ", vec![]),
            (r"[[:latn:]&[a-zA-Z]]", "azAZ", vec![]),
            // script extensions
            (r"[[:scx=latn:]&[a-zA-Z]]", "azAZ", vec![]),
            (r"[[:scx=Latin:]&[a-zA-Z]]", "azAZ", vec![]),
            (r"[[:scx=Hira:]&[\u30FC]]", "\u{30FC}\u{30FC}", vec![]),
            (r"[[:sc=Hira:]&[\u30FC]]", "", vec![]),
            (r"[[:scx=Kana:]&[\u30FC]]", "\u{30FC}\u{30FC}", vec![]),
            (r"[[:sc=Kana:]&[\u30FC]]", "", vec![]),
            (r"[[:sc=Common:]&[\u30FC]]", "\u{30FC}\u{30FC}", vec![]),
            // TODO(#3556): Add more tests (specifically conformance tests if they exist)
        ];
        for (source, single, multi) in cases {
            let parsed = parse(source);
            if let Err(err) = parsed {
                assert!(
                    false,
                    "{source} results in an error: {}",
                    err.fmt_with_source(source)
                );
                continue;
            }
            let parsed = parsed.unwrap();
            assert_set_equality(
                source,
                &parsed,
                range_iter_from_str(single),
                multi.into_iter(),
            );
        }
    }

    #[test]
    fn test_error_messages() {
        let cases = [
            (r"[a-z[\]]", r"[a-z[\]]← error: unexpected end of input"),
            (r"", r"← error: unexpected end of input"),
            (r"[{]", r"[{]← error: unexpected end of input"),
            // we match ECMA-262 strictly, so case matters
            (
                r"[:general_category:]",
                r"[:general_category← error: unknown property",
            ),
            (r"[:ll=true:]", r"[:ll=true← error: unknown property"),
            (r"[:=", r"[:=← error: unexpected character '='"),
            // property names may not be empty
            (r"[::]", r"[::← error: unexpected character ':'"),
            (r"[:=hello:]", r"[:=← error: unexpected character '='"),
            // property values may not be empty
            (r"[:gc=:]", r"[:gc=:← error: unexpected character ':'"),
            (r"[\xag]", r"[\xag← error: unexpected character 'g'"),
            (r"[--]", r"[--← error: unexpected character '-'"),
            (r"[a-z-]", r"[a-z-← error: unexpected character '-'"),
            (r"[a-b-z]", r"[a-b-← error: unexpected character '-'"),
            // TODO(#3558): Might be better as "[a-\p← error: unexpected character 'p'"?
            (r"[a-\p{ll}]", r"[a-\← error: unexpected character '\\'"),
            (r"[a-&]", r"[a-&← error: unexpected character '&'"),
            (r"[a&b]", r"[a&← error: unexpected character '&'"),
            (r"[[set]&b]", r"[[set]&b← error: unexpected character 'b'"),
            (r"[[set]&]", r"[[set]&]← error: unexpected character ']'"),
            (r"[a-\x60]", r"[a-\x60← error: unexpected character '`'"),
            (r"[a-`]", r"[a-`← error: unexpected character '`'"),
            (r"[\x{6g}]", r"[\x{6g← error: unexpected character 'g'"),
            (r"[\x{g}]", r"[\x{g← error: unexpected character 'g'"),
            (r"[\x{}]", r"[\x{}← error: unexpected character '}'"),
            (
                r"[\x{dabeef}]",
                r"[\x{dabeef← error: invalid escape sequence",
            ),
            (
                r"[\x{10ffff0}]",
                r"[\x{10ffff0← error: unexpected character '0'",
            ),
            // > 1 byte in UTF-8 edge case
            (r"ä", r"ä← error: unexpected character 'ä'"),
            (r"\p{gc=ä}", r"\p{gc=ä← error: unknown property"),
            (
                r"[\xe5-\xe4]",
                r"[\xe5-\xe4← error: unexpected character 'ä'",
            ),
            (r"[\xe5-ä]", r"[\xe5-ä← error: unexpected character 'ä'"),
        ];
        let vm = Default::default();
        for (source, expected_err) in cases {
            assert_is_error_and_message_eq(source, expected_err, &vm);
        }
    }
}
