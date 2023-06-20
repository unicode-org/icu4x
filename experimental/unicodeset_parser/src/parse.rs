// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Display;
use std::{collections::HashSet, iter::Peekable, str::CharIndices};

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder},
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_properties::maps::{load_general_category, load_script};
use icu_properties::provider::*;
use icu_properties::sets::{load_for_ecma262_unstable, CodePointSetData};
use icu_properties::{GeneralCategory, Script};
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
    /// let set = parse_unstable(source, Default::default(), &icu_testdata::unstable());
    /// assert!(set.is_err());
    /// let err = set.unwrap_err();
    /// assert_eq!(err.fmt_with_source(source).to_string(), "[[abc]-x<-- error: unexpected character 'x'");
    /// ```
    ///
    /// ```
    /// use icu_unicodeset_parser::*;
    ///
    /// let source = r"[\N{LATIN CAPITAL LETTER A}]";
    /// let set = parse_unstable(source, Default::default(), &icu_testdata::unstable());
    /// assert!(set.is_err());
    /// let err = set.unwrap_err();
    /// assert_eq!(err.fmt_with_source(source).to_string(), r"[\N<-- error: unimplemented");
    /// ```
    pub fn fmt_with_source(&self, source: &str) -> impl Display {
        let ParseError { offset, kind } = *self;

        if kind == ParseErrorKind::Eof {
            return format!("{source}<-- error: unexpected end of input");
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
                s.push_str("<-- ");
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

// this ignores the ambiguity between \escapes and \p{} perl properties. it assumes it is in a context where \p is just 'p'
// returns whether the provided char signifies the start of a literal char (raw or escaped - so \ is a legal char start)
fn legal_char_start(c: char) -> bool {
    !(c == '&'
        || c == '-'
        || c == '['
        || c == ']'
        || c == '{'
        || c == '}'
        || is_char_pattern_white_space(c))
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Union,
    Difference,
    Intersection,
}

// TODO: if UnicodeSetBuilder is staying private, this should have a more generic name like ParseOptions or UnicodeSetParseOptions
/// Options for parsing a UnicodeSet.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct UnicodeSetBuilderOptions {
    /// If true, the dollar sign '$' is treated as an anchor and replaced with U+FFFF whenever
    /// it appears as a single codepoint (e.g., `[$]`) or as part of a range (e.g., `[\uFF00-$]`).
    /// Note that '$' is never replaced in a property, nor in a multi-codepoint expression,
    /// no matter what this option is set to.
    ///
    /// This option is useful for implementations that interpret U+FFFF as a special value.
    pub dollar_is_anchor: bool,
}

// this builds the set on-the-fly while parsing it
struct UnicodeSetBuilder<'a, 'b, 'c, P: ?Sized> {
    single_set: CodePointInversionListBuilder,
    multi_set: HashSet<String>,
    iter: &'b mut Peekable<CharIndices<'a>>,
    inverted: bool,
    options: UnicodeSetBuilderOptions,
    property_provider: &'c P,
}

impl<'a, 'b, 'c, P> UnicodeSetBuilder<'a, 'b, 'c, P>
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
        + DataProvider<GeneralCategoryNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<XidStartV1Marker>,
{
    fn new_inner(
        iter: &'b mut Peekable<CharIndices<'a>>,
        options: UnicodeSetBuilderOptions,
        provider: &'c P,
    ) -> Self {
        UnicodeSetBuilder {
            single_set: CodePointInversionListBuilder::new(),
            multi_set: Default::default(),
            iter,
            inverted: false,
            options,
            property_provider: provider,
        }
    }

    // the entry point, parses a full UnicodeSet. ignores remaining input
    fn parse_unicode_set(&mut self) -> Result<()> {
        match *self.must_peek_char()? {
            '\\' => self.parse_property_perl(),
            '[' => {
                self.iter.next();
                if let Some(&':') = self.peek_char() {
                    self.parse_property_posix()
                } else {
                    self.parse_unicode_set_inner()
                }
            }
            c => self.error_here(PEK::UnexpectedChar(c)),
        }
    }

    // beginning [ is already consumed
    fn parse_unicode_set_inner(&mut self) -> Result<()> {
        // special cases for the first char after [
        match self.must_peek_char()? {
            '^' => {
                self.iter.next();
                self.inverted = true;
            }
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
        }
        use State::*;

        let mut state = Begin;
        let mut prev_char = None;
        let mut operation = Operation::Union;

        loop {
            self.skip_whitespace();

            // handling unicodesets separately, because of ambiguity between escaped characters and perl property names
            if self.peek_unicode_set_start() {
                match (state, *self.must_peek_char()?) {
                    // parse a recursive unicode set
                    (Begin | Char | AfterUnicodeSet | AfterOp, _) => {
                        if let Some(prev) = prev_char.take() {
                            self.single_set.add_char(prev);
                        }

                        let mut inner_builder = UnicodeSetBuilder::new_inner(
                            self.iter,
                            self.options,
                            self.property_provider,
                        );
                        inner_builder.parse_unicode_set()?;
                        let (single, multi) = inner_builder.finalize();

                        self.process_chars(operation, single.build());
                        self.process_strings(operation, multi);
                        operation = Operation::Union;

                        state = AfterUnicodeSet;
                    }
                    (_, c) => return self.error_here(PEK::UnexpectedChar(c)),
                }
                continue;
            }

            // note: no UnicodeSets can occur in this match block, as they would've been caught by the above match
            match (state, *self.must_peek_char()?) {
                // parse the end of this unicode set
                (Begin | Char | AfterUnicodeSet, ']') => {
                    self.iter.next();
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }

                    return Ok(());
                }
                // parse a literal char (either individually or as the start of a range)
                (Begin | Char | AfterUnicodeSet, c) if legal_char_start(c) => {
                    let c = self.parse_char(self.options.dollar_is_anchor)?;
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }
                    prev_char = Some(c);
                    state = Char;
                }
                // parse a literal char as the end of a range
                (CharMinus, c) if legal_char_start(c) => {
                    let start = prev_char.ok_or(PEK::Eof)?;
                    let end = self.parse_char(self.options.dollar_is_anchor)?;
                    if start > end {
                        // TODO: better error message (e.g., "start greater than end in range")
                        // note: offset - 1, because we already consumed the end char (and its offset)
                        return Err(
                            PEK::UnexpectedChar(end).with_offset(self.must_peek_index()? - 1)
                        );
                    }

                    self.single_set.add_range(&(start..=end));
                    prev_char = None;
                    state = Begin;
                }
                // parse a multi-codepoint-sequence
                (Begin | Char | AfterUnicodeSet, '{') => {
                    if let Some(prev) = prev_char.take() {
                        self.single_set.add_char(prev);
                    }

                    self.parse_multi()?;
                    state = Begin;
                }
                // start parsing a char range
                (Char, '-') => {
                    self.iter.next();
                    state = CharMinus;
                }
                // start parsing a unicode set difference
                (AfterUnicodeSet, '-') => {
                    self.iter.next();
                    operation = Operation::Difference;
                    state = AfterOp;
                }
                // start parsing a unicode set difference
                (AfterUnicodeSet, '&') => {
                    self.iter.next();
                    operation = Operation::Intersection;
                    state = AfterOp;
                }
                (_, c) => return self.error_here(PEK::UnexpectedChar(c)),
            }
        }
    }

    // parses and consumes '{' (s char)+ s '}'
    // TODO: decide on names for multi-codepoint-sequences and adjust both struct fields and fn names
    fn parse_multi(&mut self) -> Result<()> {
        self.consume('{')?;

        let mut buffer = String::new();

        loop {
            self.skip_whitespace();

            match *self.must_peek_char()? {
                '}' => {
                    self.iter.next();
                    break;
                }
                c if legal_char_start(c) => {
                    // '$' in multi-codepoint-sequences is not an anchor, no matter what the option is set to
                    let c = self.parse_char(false)?;
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
    fn parse_escaped_char(&mut self) -> Result<char> {
        self.consume('\\')?;

        let (offset, next_char) = self.must_next()?;

        match next_char {
            'u' | 'x' if self.peek_char() == Some(&'{') => {
                // bracketedHex
                self.iter.next();
                self.skip_whitespace();
                let (hex_digits, end_offset) = self.parse_variable_length_hex()?;
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                let c =
                    char::try_from(num).map_err(|_| PEK::InvalidEscape.with_offset(end_offset))?;
                self.skip_whitespace();
                self.consume('}')?;
                Ok(c)
            }
            'u' => {
                // 'u' hex{4}
                let exact: [char; 4] = self.parse_exact_hex_digits()?;
                let hex_digits = exact.iter().collect::<String>();
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                char::try_from(num).map_err(|_| PEK::InvalidEscape.with_offset(offset))
            }
            'x' => {
                // 'x' hex{2}
                let exact: [char; 2] = self.parse_exact_hex_digits()?;
                let hex_digits = exact.iter().collect::<String>();
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                char::try_from(num).map_err(|_| PEK::InvalidEscape.with_offset(offset))
            }
            'U' => {
                // 'U00' ('0' hex{5} | '10' hex{4})
                self.consume('0')?;
                self.consume('0')?;
                let hex_digits = match *self.must_peek_char()? {
                    '0' => {
                        self.iter.next();
                        let exact: [char; 5] = self.parse_exact_hex_digits()?;
                        exact.iter().collect::<String>()
                    }
                    '1' => {
                        self.iter.next();
                        self.consume('0')?;
                        let exact: [char; 4] = self.parse_exact_hex_digits()?;
                        ['1', '0'].iter().chain(exact.iter()).collect::<String>()
                    }
                    c => return self.error_here(PEK::UnexpectedChar(c)),
                };
                let num = u32::from_str_radix(&hex_digits, 16).map_err(|_| PEK::Internal)?;
                char::try_from(num).map_err(|_| PEK::InvalidEscape.with_offset(offset))
            }
            'N' => {
                // parse code point with name in {}
                // tracking issue: https://github.com/unicode-org/icu4x/issues/1397
                Err(PEK::Unimplemented.with_offset(offset))
            }
            'a' => Ok('\u{0007}'),
            'b' => Ok('\u{0008}'),
            't' => Ok('\u{0009}'),
            'n' => Ok('\u{000A}'),
            'v' => Ok('\u{000B}'),
            'f' => Ok('\u{000C}'),
            'r' => Ok('\u{000D}'),
            _ => Ok(next_char),
        }
    }

    // starts with :, consumes the trailing :]
    fn parse_property_posix(&mut self) -> Result<()> {
        self.consume(':')?;
        if *self.must_peek_char()? == '^' {
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
        // TODO: support loose matching on property names (e.g., "AS  -_-  CII_Hex_ D-igit")
        // TODO: support more properties than ECMA-262

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
            match (state, *self.must_peek_char()?) {
                // parse the end of the property expression
                (PropertyName | PropertyValue, c) if c == end => {
                    // byte index of (full) property name/value is one back
                    property_offset = self.must_peek_index()? - 1;
                    self.iter.next();
                    break;
                }
                // parse the property name
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

        let (set, inverted) = self
            .load_property_codepoints(&key_buffer, &value_buffer)
            // any error that does not already have an offset should use the appropriate property offset
            .map_err(|e| e.or_with_offset(property_offset))?;
        if inverted {
            self.inverted = !self.inverted;
        }
        self.single_set.add_set(&set.to_code_point_inversion_list());

        Ok(())
    }

    // also returns whether the set needs to be inverted or not
    fn load_property_codepoints(&self, key: &str, value: &str) -> Result<(CodePointSetData, bool)> {
        // we support:
        // [:gc = value:]
        // [:sc = value:]
        // [:scx = value:] -- TODO: implement scx
        // [:value:] - looks up value in gc, sc and scx
        // [:prop:] - binary property, returns codepoints that have the property
        // [:prop = truthy/falsy:] - same as above

        let mut inverted = false;

        if value.is_empty() {
            // key is binary property, or a value of gc, sc, scx

            // try loading a binary property, if it fails, try loading a gc, sc, scx value
            let set = load_for_ecma262_unstable(self.property_provider, key)
                .or_else(|_| self.try_load_general_category(key))
                .or_else(|_| self.try_load_script(key))?;
            // .or_else(|_| self.try_load_script_extensions(key))?;

            Ok((set, inverted))
        } else {
            // key is gc, sc, scx
            // value is a property value
            // OR
            // key is a binary property and value is a truthy/falsy value

            // UnicodeProperty::parse_ecma262_name would be good to have to avoid this duplication:
            let set = match key {
                "General_Category" | "gc" => self.try_load_general_category(value)?,
                "Script" | "sc" => self.try_load_script(value)?,
                // "Script_Extensions" | "scx" => {
                //     // https://www.unicode.org/reports/tr24/#Script_Extensions
                //     // the UnicodeSet utils page takes lists of scripts as values, but I'm not sure how to implement that using load_script_with_extensions_unstable.
                //     // skipped for now.
                _ => {
                    // try prop = truthy/falsy case
                    let set = load_for_ecma262_unstable(self.property_provider, key)
                        .map_err(|_| PEK::UnknownProperty)?;
                    let normalized_value = value.to_ascii_lowercase();
                    let truthy = matches!(normalized_value.as_str(), "true" | "t" | "yes" | "y");
                    let falsy = matches!(normalized_value.as_str(), "false" | "f" | "no" | "n");
                    if truthy == falsy {
                        return Err(PEK::UnknownProperty.into());
                    }
                    inverted = falsy;
                    set
                }
            };

            Ok((set, inverted))
        }
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
            Some(&'\\') => {}
            Some(&'[') => return true,
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

    // parses either a raw char or an escaped char. all chars are allowed.
    // anchor_allowed determines whether $ is interpreted as $ or as \uFFFF
    fn parse_char(&mut self, anchor_allowed: bool) -> Result<char> {
        let c = *self.must_peek_char()?;
        match c {
            '\\' => self.parse_escaped_char(),
            '$' if anchor_allowed => {
                self.iter.next();
                Ok('\u{FFFF}')
            }
            _ => {
                self.iter.next();
                Ok(c)
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
            let (unexpected_offset, unexpected_char) = *self.must_peek()?;
            return Err(PEK::UnexpectedChar(unexpected_char).with_offset(unexpected_offset));
        }
        Ok((result, end_offset))
    }

    // parses [0-9a-fA-F]{N}
    fn parse_exact_hex_digits<const N: usize>(&mut self) -> Result<[char; N]> {
        let mut result = [0 as char; N];
        for slot in result.iter_mut() {
            let (offset, c) = self.must_next()?;
            if !c.is_ascii_hexdigit() {
                return Err(PEK::UnexpectedChar(c).with_offset(offset));
            }
            *slot = c;
        }
        Ok(result)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
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
    fn must_peek(&mut self) -> Result<&(usize, char)> {
        self.iter.peek().ok_or(PEK::Eof.into())
    }

    // see must_peek
    fn must_peek_char(&mut self) -> Result<&char> {
        self.must_peek().map(|(_, c)| c)
    }

    // see must_peek
    fn must_peek_index(&mut self) -> Result<usize> {
        self.must_peek().map(|&(idx, _)| idx)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.iter.peek().map(|(_, c)| c)
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

    fn try_load_general_category(&self, name: &str) -> Result<CodePointSetData> {
        let name_map = GeneralCategory::get_name_to_enum_mapper(self.property_provider)
            .map_err(|_| PEK::Internal)?;
        let gc_value = name_map
            .as_borrowed()
            .get_loose(name)
            .ok_or(PEK::UnknownProperty)?;
        let property_map =
            load_general_category(self.property_provider).map_err(|_| PEK::Internal)?;
        let set = property_map.as_borrowed().get_set_for_value(gc_value);
        Ok(set)
    }

    fn try_load_script(&self, name: &str) -> Result<CodePointSetData> {
        let name_map =
            Script::get_name_to_enum_mapper(self.property_provider).map_err(|_| PEK::Internal)?;
        let sc_value = name_map
            .as_borrowed()
            .get_loose(name)
            .ok_or(PEK::UnknownProperty)?;
        let property_map = load_script(self.property_provider).map_err(|_| PEK::Internal)?;
        let set = property_map.as_borrowed().get_set_for_value(sc_value);
        Ok(set)
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
/// use icu_unicodeset_parser::{parse_unstable, UnicodeSetBuilderOptions};
///
/// let set = parse_unstable("[a-zA-Z0-9]", Default::default(), &icu_testdata::unstable()).unwrap();
/// let code_points = set.code_points();
///
/// assert!(code_points.contains_range(&('a'..='z')));
/// assert!(code_points.contains_range(&('A'..='Z')));
/// assert!(code_points.contains_range(&('0'..='9')));
/// ```
///
/// Parse properties, set operations, inner sets
/// ```
/// use icu_unicodeset_parser::{parse_unstable, UnicodeSetBuilderOptions};
///
/// let set = parse_unstable("[[:^ll:]-[^][:gc = Lowercase Letter:]&[^[[^]-[a-z]]]]", Default::default(), &icu_testdata::unstable()).unwrap();
/// let elements = 'a'..='z';
/// assert!(set.code_points().contains_range(&elements));
/// assert_eq!(elements.count(), set.size());
/// ```
///
/// Inversions remove strings
/// ```
/// use icu_unicodeset_parser::{parse_unstable, UnicodeSetBuilderOptions};
///
/// let set = parse_unstable(r"[[a-z{hello\ world}]&[^a-y{hello\ world}]]", Default::default(), &icu_testdata::unstable()).unwrap();
/// assert!(set.contains_char('z'));
/// assert_eq!(set.size(), 1);
/// assert!(!set.has_strings());
/// ```
///
/// Set operators (including the implicit union) have the same precedence and are left-associative
/// ```
/// use icu_unicodeset_parser::{parse_unstable, UnicodeSetBuilderOptions};
///
/// let set = parse_unstable("[[ace][bdf] - [abc][def]]", Default::default(), &icu_testdata::unstable()).unwrap();
/// let elements = 'd'..='f';
/// assert!(set.code_points().contains_range(&elements));
/// assert_eq!(set.size(), elements.count());
/// ```
pub fn parse_unstable<P>(
    source: &str,
    options: UnicodeSetBuilderOptions,
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
        + DataProvider<GeneralCategoryNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<XidStartV1Marker>,
{
    // TODO: add function "parse_overescaped" that uses a custom iterator to de-overescape (i.e., maps \\ to \) on-the-fly
    // ^ will likely need a different iterator type on UnicodeSetBuilder
    // TODO: think about returning byte-length of the parsed unicodeset for use in transliterator, or add public function that accepts a peekable char iterator?

    let mut iter = source.char_indices().peekable();
    let mut builder = UnicodeSetBuilder::new_inner(&mut iter, options, provider);

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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! td {
        () => {
            icu_testdata::unstable()
        };
    }

    const OPTIONS_ANCHOR: UnicodeSetBuilderOptions = UnicodeSetBuilderOptions {
        dollar_is_anchor: true,
    };
    const OPTIONS_NO_ANCHOR: UnicodeSetBuilderOptions = UnicodeSetBuilderOptions {
        dollar_is_anchor: false,
    };

    fn assert_set_equality<'a, 'b>(
        cpinvlistandstrlist: &CodePointInversionListAndStringList<'a>,
        single: impl Iterator<Item = char>,
        multi: impl Iterator<Item = &'b str>,
    ) {
        let mut it_size = 0;
        for c in single {
            it_size += 1;
            assert!(
                cpinvlistandstrlist.contains_char(c),
                "missing char from parsed set: '{}'",
                c.escape_debug()
            );
        }
        for s in multi {
            it_size += 1;
            assert!(
                cpinvlistandstrlist.contains(s),
                "missing string from parsed set: \"{}\"",
                s.escape_debug()
            );
        }
        assert_eq!(cpinvlistandstrlist.size(), it_size);
    }

    fn assert_is_error_and_message_eq(
        options: UnicodeSetBuilderOptions,
        source: &str,
        expected_err: &str,
    ) {
        let result = parse_unstable(source, options, &td!());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.fmt_with_source(source).to_string(), expected_err);
    }

    #[test]
    fn test_semantics() {
        let cases: [(_, _, _, Vec<String>); 10] = [
            (OPTIONS_NO_ANCHOR, "[a]", 'a'..='a', vec![]),
            (OPTIONS_NO_ANCHOR, r"[\n]", '\n'..='\n', vec![]),
            (OPTIONS_NO_ANCHOR, "[\\\n]", '\n'..='\n', vec![]),
            // empty - whitespace is skipped
            (OPTIONS_NO_ANCHOR, "[\n]", 'b'..='a', vec![]),
            (OPTIONS_NO_ANCHOR, "[a-z]", 'a'..='z', vec![]),
            (OPTIONS_NO_ANCHOR, "[$]", '$'..='$', vec![]),
            (OPTIONS_NO_ANCHOR, "[{$}]", '$'..='$', vec![]),
            (OPTIONS_ANCHOR, "[$]", '\u{ffff}'..='\u{ffff}', vec![]),
            (OPTIONS_ANCHOR, r"[\$]", '$'..='$', vec![]),
            (OPTIONS_ANCHOR, "[{$}]", '$'..='$', vec![]),
            // TODO: add more tests, look for ICU tests
        ];
        for (options, source, single, multi) in cases {
            let parsed = parse_unstable(source, options, &td!()).unwrap();
            assert_set_equality(&parsed, single.clone(), multi.iter().map(|s| s.as_str()));
        }
    }

    #[test]
    fn test_error_messages() {
        let cases = [
            (r"[a-z[\]]", r"[a-z[\]]<-- error: unexpected end of input"),
            (r"", r"<-- error: unexpected end of input"),
            (r"[{]", r"[{]<-- error: unexpected character ']'"),
            // we match ECMA-262 strictly, so case matters
            (
                r"[:general_category:]",
                r"[:general_category<-- error: unknown property",
            ),
            (r"[:ll=true:]", r"[:ll=true<-- error: unknown property"),
            (r"[:=", r"[:=<-- error: unexpected character '='"),
            // property names may not be empty
            (r"[::]", r"[::<-- error: unexpected character ':'"),
            (r"[:=hello:]", r"[:=<-- error: unexpected character '='"),
            // property values may not be empty
            (r"[:gc=:]", r"[:gc=:<-- error: unexpected character ':'"),
            (r"[\xag]", r"[\xag<-- error: unexpected character 'g'"),
            (
                r"[{this is a minus -}]",
                r"[{this is a minus -<-- error: unexpected character '-'",
            ),
            (r"[--]", r"[--<-- error: unexpected character '-'"),
            (r"[a-z-]", r"[a-z-<-- error: unexpected character '-'"),
            // TODO: might be better as "[a-\p<-- error: unexpected character 'p'"
            (r"[a-\p{ll}]", r"[a-\<-- error: unexpected character '\\'"),
            (r"[a-&]", r"[a-&<-- error: unexpected character '&'"),
            (r"[a&b]", r"[a&<-- error: unexpected character '&'"),
            (r"[[set]&b]", r"[[set]&b<-- error: unexpected character 'b'"),
            (r"[[set]&]", r"[[set]&]<-- error: unexpected character ']'"),
            (r"[a-\x60]", r"[a-\x60<-- error: unexpected character '`'"),
            (r"[a-`]", r"[a-`<-- error: unexpected character '`'"),
            (r"[\x{6g}]", r"[\x{6g<-- error: unexpected character 'g'"),
            (r"[\x{g}]", r"[\x{g<-- error: unexpected character 'g'"),
            (r"[\x{}]", r"[\x{}<-- error: unexpected character '}'"),
            (
                r"[\x{dabeef}]",
                r"[\x{dabeef<-- error: invalid escape sequence",
            ),
            (
                r"[\x{10ffff0}]",
                r"[\x{10ffff0<-- error: unexpected character '0'",
            ),
            // > 1 byte in UTF-8 edge case
            (r"ä", r"ä<-- error: unexpected character 'ä'"),
            (r"\p{gc=ä}", r"\p{gc=ä<-- error: unknown property"),
            (
                r"[\xe5-\xe4]",
                r"[\xe5-\xe4<-- error: unexpected character 'ä'",
            ),
            (r"[\xe5-ä]", r"[\xe5-ä<-- error: unexpected character 'ä'"),
        ];
        for (source, expected_err) in cases {
            assert_is_error_and_message_eq(OPTIONS_NO_ANCHOR, source, expected_err);
        }
    }
}
