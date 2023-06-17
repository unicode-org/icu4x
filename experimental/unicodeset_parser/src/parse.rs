use std::{
    collections::HashSet,
    iter::Peekable,
    str::{CharIndices, Chars},
};

use icu_collections::{
    codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder},
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_properties::maps::{load_general_category, load_script};
use icu_properties::provider::*;
use icu_properties::script::{load_script_with_extensions_unstable, ScriptWithExtensions};
use icu_properties::sets::{
    load_for_ecma262_unstable, load_for_general_category_group, CodePointSetData,
};
use icu_properties::{GeneralCategory, Script};
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

// Parses UnicodeSets

#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    UnexpectedChar(char),
    UnknownProperty,
    Eof,
    Internal,
    Unimplemented,
    InvalidEscape,
}

#[derive(Debug, Clone)]
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

    fn unknown_property() -> Self {
        Self::new_without_offset(ParseErrorKind::UnknownProperty)
    }

    fn unimplemented(offset: usize) -> Self {
        Self::new_with_offset(offset, ParseErrorKind::Unimplemented)
    }

    fn invalid_escape(offset: usize) -> Self {
        Self::new_with_offset(offset, ParseErrorKind::InvalidEscape)
    }
}

type Result<T, E = ParseError> = core::result::Result<T, E>;

// note: this ignores the ambiguity between \escapes and \p{} perl properties. it assumes it is in a context where \p is just 'p'
fn legal_char_start(c: char) -> bool {
    !(c == '&'
            || c == '-'
            // legal because it starts an escape sequence
            // || c == '\\'
            || c == '['
            || c == ']'
            || c == '{'
            || c == '}'
            || c.is_ascii_whitespace())
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Union,
    Difference,
    Intersection,
}

// note: "compiles" the set while building, so no intermediate parse tree, it's directly compiled.
pub struct UnicodeSetBuilder<'a, 'b, 'c, P>
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
    single_set: CodePointInversionListBuilder,
    multi_set: HashSet<String>,
    iter: &'b mut Peekable<CharIndices<'a>>,
    inverted: bool,
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
        provider: &'c P,
    ) -> UnicodeSetBuilder<'a, 'b, 'c, P> {
        UnicodeSetBuilder {
            single_set: CodePointInversionListBuilder::new(),
            multi_set: Default::default(),
            iter,
            inverted: false,
            property_provider: provider,
        }
    }

    fn consume(&mut self, expected: char) -> Result<()> {
        match self.iter.next() {
            None => Err(ParseError::eof()),
            Some((offset, c)) if c != expected => Err(ParseError::unexpected(offset, c)),
            _ => Ok(()),
        }
    }

    // also returns whether the set needs to be inverted or not
    fn load_property_codepoints(&self, key: &str, value: &str) -> Result<(CodePointSetData, bool)> {
        // we support:
        // [:gc = value:]
        // [:sc = value:]
        // [:scx = value:]
        // [:value:] - looks up value in gc, sc and scx
        // [:prop:] - binary property, returns codepoints that have the property
        // [:prop = true:] - same as above

        let mut inverted = false;

        if value.is_empty() {
            // key_buffer is binary property, or value of gc, sc, scx

            // try loading a binary property, if it fails, try loading a gc, sc, scx value
            let set = load_for_ecma262_unstable(self.property_provider, key)
                .or_else(|_| self.try_load_general_category(key))
                .or_else(|_| self.try_load_script(key))?;
            // .or_else(|_| self.try_load_script_extensions(&key_buffer))?;

            Ok((set, inverted))
        } else {
            // key_buffer is gc, sc, scx
            // value_buffer is value
            // OR
            // key_buffer is a binary property and value is a truthy/falsy value

            // UnicodeProperty::parse_ecma262_name would be good to have to avoid this duplication:
            let set = match key {
                "General_Category" | "gc" => self.try_load_general_category(value)?,
                "Script" | "sc" => self.try_load_script(value)?,
                // "Script_Extensions" | "scx" => {
                //     // https://www.unicode.org/reports/tr24/#Script_Extensions
                //     // the UnicodeSet utils page takes lists of scripts as values, but I'm not sure how to implement that using load_script_with_extensions_unstable.
                //     // skipped for now.
                _ => {
                    // try prop = true case
                    let set = load_for_ecma262_unstable(self.property_provider, key)
                        .map_err(|_| ParseError::unknown_property())?;
                    let normalized_value = value.to_ascii_lowercase();
                    let truthy = matches!(normalized_value.as_str(), "true" | "t" | "yes" | "y");
                    let falsy = matches!(normalized_value.as_str(), "false" | "f" | "no" | "n");
                    if truthy == falsy {
                        return Err(ParseError::unknown_property());
                    }
                    inverted = falsy;
                    set
                }
            };

            Ok((set, inverted))
        }
    }

    // TODO: split up this function into key/value parsing, and then property loading
    fn parse_property_inner(&mut self, end: char) -> Result<()> {
        // only supports ECMA-262 for the moment. UnicodeSet spec ignores whitespace, '-', and '_',
        // but ECMA-262 requires '_', so we'll allow that.
        // TODO: support loose matching on keys (e.g., "AS  -_-  CII_Hex_ D-igit")
        // TODO: support other properties

        let mut key_buffer = String::new();
        let mut value_buffer = String::new();

        enum State {
            Begin,
            PropertyName,
            PropertyValueBegin,
            PropertyValue,
        }
        use State::*;

        let mut state = Begin;
        // whether = (true) or ≠ (false) was parsed
        let mut equality = true;

        loop {
            self.skip_whitespace();
            match (state, self.peek_char()) {
                (_, None) => return Err(ParseError::eof()),
                (PropertyName | PropertyValue, Some(c)) if *c == end => {
                    self.iter.next();
                    break;
                }
                (Begin | PropertyName, Some(&c)) if c.is_ascii_alphanumeric() || c == '_' => {
                    key_buffer.push(c);
                    self.iter.next();
                    state = PropertyName;
                }
                (PropertyName, Some(&c @ ('=' | '≠'))) => {
                    equality = c == '=';
                    self.iter.next();
                    state = PropertyValueBegin;
                }
                (PropertyValue | PropertyValueBegin, Some(&c)) => {
                    value_buffer.push(c);
                    self.iter.next();
                    state = PropertyValue;
                }
                (_, Some(&c)) => return Err(ParseError::unexpected(self.peek_index()?, c)),
            }
        }

        if !equality {
            self.inverted = !self.inverted;
        }

        let (set, inverted) = self.load_property_codepoints(&key_buffer, &value_buffer)?;
        if inverted {
            self.inverted = !self.inverted;
        }
        self.single_set.add_set(&set.to_code_point_inversion_list());

        Ok(())
    }

    fn parse_property_perl(&mut self) -> Result<()> {
        self.consume('\\')?;
        match self.iter.next() {
            None => return Err(ParseError::eof()),
            Some((_, 'p')) => {}
            Some((_, 'P')) => self.inverted = true,
            Some((offset, c)) => return Err(ParseError::unexpected(offset, c)),
        }
        self.consume('{')?;

        self.parse_property_inner('}')?;

        Ok(())
    }

    // starts with :, consumes the trailing :]
    fn parse_property_posix(&mut self) -> Result<()> {
        self.consume(':')?;
        match self.peek_char() {
            None => return Err(ParseError::eof()),
            Some(&'^') => {
                self.inverted = true;
                self.iter.next();
            }
            _ => {}
        }

        self.parse_property_inner(':')?;

        self.consume(']')?;

        Ok(())
    }

    fn unicode_set_start(&mut self) -> bool {
        match self.peek_char() {
            Some(&'\\') => {}
            Some(&'[') => return true,
            _ => return false,
        }

        // need to look one more char into the future. Peekable doesnt lend itself well to this, so maybe think about using a different iterator internally

        let mut future = self.iter.clone();
        future.next();

        match future.peek() {
            // perl property
            Some(&(_, 'p')) => true,
            _ => false,
        }
    }

    fn parse_exact_hex_digits<const N: usize>(&mut self) -> Result<[char; N]> {
        let mut result = [0 as char; N];
        for i in 0..N {
            let (offset, c) = self.iter.next().ok_or(ParseError::eof())?;
            if !c.is_ascii_hexdigit() {
                // TODO: is this offset correct? check all offset errors in this file for consistency
                return Err(ParseError::unexpected(offset, c));
            }
            result[i] = c;
        }
        Ok(result)
    }

    // starts with \ and consumes the whole escape sequence
    fn parse_escaped_char(&mut self) -> Result<char> {
        self.consume('\\')?;

        let (offset, next_char) = self.iter.next().ok_or(ParseError::eof())?;

        if !['u', 'U', 'x', 'N'].contains(&next_char) {
            return Ok(next_char);
        }

        match next_char {
            'u' => {
                // 'u' (hex{4} | bracketedHex) -- TODO: implement bracketedHex
                let exact: [char; 4] = self.parse_exact_hex_digits()?;
                let hex_digits = exact.iter().collect::<String>();
                let num =
                    u32::from_str_radix(&hex_digits, 16).map_err(|_| ParseError::internal())?;
                char::try_from(num).map_err(|_| ParseError::invalid_escape(offset))
            }
            'U' => {
                // 'U00' ('0' hex{5} | '10' hex{4})
                self.consume('0')?;
                self.consume('0')?;
                let hex_digits = match self.peek_char() {
                    Some(&'0') => {
                        self.iter.next();
                        let exact: [char; 5] = self.parse_exact_hex_digits()?;
                        exact.iter().collect::<String>()
                    }
                    Some(&'1') => {
                        self.iter.next();
                        self.consume('0')?;
                        let exact: [char; 4] = self.parse_exact_hex_digits()?;
                        ['1', '0'].iter().chain(exact.iter()).collect::<String>()
                    }
                    Some(&c) => return Err(ParseError::unexpected(self.peek_index()?, c)),
                    None => return Err(ParseError::eof()),
                };
                let num =
                    u32::from_str_radix(&hex_digits, 16).map_err(|_| ParseError::internal())?;
                char::try_from(num).map_err(|_| ParseError::invalid_escape(offset))
            }
            'x' => {
                // 'x' (hex{2} | bracketedHex)
                let exact: [char; 2] = self.parse_exact_hex_digits()?;
                let hex_digits = exact.iter().collect::<String>();
                let num =
                    u32::from_str_radix(&hex_digits, 16).map_err(|_| ParseError::internal())?;
                char::try_from(num).map_err(|_| ParseError::invalid_escape(offset))
            }
            'N' => {
                // parse code point with name in {}
                // tracking issue: https://github.com/unicode-org/icu4x/issues/1397
                Err(ParseError::unimplemented(self.peek_index()?))
            }
            _ => Ok(next_char),
        }
    }

    // parses and consumes '{' (s char)+ s '}'
    // TODO: decide on names for multi-codepoint-sequences and adjust both struct fields and fn names
    fn parse_multi(&mut self) -> Result<()> {
        self.consume('{')?;

        let mut buffer = String::new();

        loop {
            self.skip_whitespace();

            match self.peek_char() {
                None => return Err(ParseError::eof()),
                Some(c) if *c == '}' => {
                    self.iter.next();
                    break;
                }
                // TODO: this must also be legal_char, and handle escapes
                Some(&c) if legal_char_start(c) => {
                    let mut c = c;
                    if c == '\\' {
                        c = self.parse_escaped_char()?;
                    } else {
                        self.iter.next();
                    }
                    buffer.push(c);
                }
                Some(&c) => return Err(ParseError::unexpected(self.peek_index()?, c)),
            }
        }

        if buffer.chars().count() > 1 {
            self.multi_set.insert(buffer);
        } else {
            // empty or single char
            if let Some(c) = buffer.chars().next() {
                // a single-codepoint multi-codepoint-sequence is interpreted as a character
                self.single_set.add_char(c);
            }
        }

        Ok(())
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
            }
            Some(&'-') => {
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
            Begin,
            Char,
            CharMinus,
            AfterUnicodeSet,
            AfterOp,
        }
        use State::*;

        let mut state = Begin;
        let mut prev_char = None;
        let mut operation = Operation::Union;

        loop {
            self.skip_whitespace();
            eprintln!("state: {:?}, next: {:?}", state, self.peek_char());

            // handling unicodesets separately, because of ambiguity between escaped characters and perl property names
            if self.unicode_set_start() {
                // maybe put into inner function
                match (state, self.peek_char()) {
                    (_, None) => return Err(ParseError::eof()),
                    (Begin | Char | AfterUnicodeSet | AfterOp, Some(_)) => {
                        if let Some(prev) = prev_char {
                            self.single_set.add_char(prev);
                            prev_char = None;
                        }

                        let mut inner_builder =
                            UnicodeSetBuilder::new_inner(self.iter, self.property_provider);
                        inner_builder.parse_unicode_set()?;
                        let (single, multi) = inner_builder.finalize();

                        self.process_chars(operation, single.build());
                        self.process_strings(operation, multi);
                        operation = Operation::Union;

                        state = AfterUnicodeSet;
                    }
                    (_, Some(&c)) => return Err(ParseError::unexpected(self.peek_index()?, c)),
                }
                continue;
            }

            // no UnicodeSets can occur in this match block, they would've been caught by the above match
            match (state, self.peek_char()) {
                (_, None) => return Err(ParseError::eof()),
                (Begin | Char | AfterUnicodeSet, Some(']')) => {
                    self.iter.next();
                    if let Some(prev) = prev_char {
                        self.single_set.add_char(prev);
                        prev_char = None;
                    }

                    return Ok(());
                }
                (Begin | Char | AfterUnicodeSet, Some(&c)) if legal_char_start(c) => {
                    let mut c = c;
                    if c == '\\' {
                        c = self.parse_escaped_char()?;
                    } else {
                        self.iter.next();
                    }
                    if let Some(prev) = prev_char {
                        self.single_set.add_char(prev);
                    }
                    prev_char = Some(c);
                    state = Char;
                }
                // TODO: handle escapes
                (CharMinus, Some(&end)) if legal_char_start(end) => {
                    // store offset now because parsing escaped char would return an inconsistent offset afterwards
                    let begin_offset = self.peek_index()?;
                    let start = prev_char.ok_or(ParseError::internal())?;
                    let mut end = end;
                    if end == '\\' {
                        end = self.parse_escaped_char()?;
                    } else {
                        self.iter.next();
                    }
                    if start > end {
                        // todo: better error message
                        return Err(ParseError::unexpected(begin_offset, end));
                    }

                    self.single_set.add_range(&(start..=end));
                    prev_char = None;
                    state = Begin;
                }
                (Begin | Char | AfterUnicodeSet, Some(&'{')) => {
                    if let Some(prev) = prev_char {
                        self.single_set.add_char(prev);
                        prev_char = None;
                    }

                    self.parse_multi()?;
                    state = Begin;
                }
                (Char, Some('-')) => {
                    self.iter.next();
                    state = CharMinus;
                }
                (AfterUnicodeSet, Some(&'-')) => {
                    self.iter.next();
                    operation = Operation::Difference;
                    state = AfterOp;
                }
                (AfterUnicodeSet, Some(&'&')) => {
                    self.iter.next();
                    operation = Operation::Intersection;
                    state = AfterOp;
                }
                (_, Some(&c)) => return Err(ParseError::unexpected(self.peek_index()?, c)),
            }
        }
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
            }
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

    // TODO: Everywhere, check if .is_whitespace() matches :Pattern_White_Space:
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.iter.next();
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

    // BELOW IS WRONG, WE CAN HAVE BOTH KINDS AT SAME LEVEL, BUT STILL ONLY NEED op-handling for recursive UNICODE SETS because only UNICODE SETS are allowed to follow an op. others are implicitly additive
    // actually this might only be needed for processing sequences of ops in the case of a ContainerUnicodeSet, in the case
    // of a LeafUnicodeSet (i.e., one that contains chars, char ranges and multi codepoint elements), we cannot have other ops
    // fn process_string(&mut self, s: String) {
    //     match self.next_op
    // }
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

    fn process_chars<'any>(&mut self, op: Operation, other_chars: CodePointInversionList<'any>) {
        match op {
            Operation::Union => self.single_set.add_set(&other_chars),
            Operation::Difference => self.single_set.remove_set(&other_chars),
            Operation::Intersection => self.single_set.retain_set(&other_chars),
        }
    }

    fn try_load_general_category(&self, name: &str) -> Result<CodePointSetData> {
        let name_map = GeneralCategory::get_name_to_enum_mapper(self.property_provider)
            .map_err(|_| ParseError::internal())?;
        let gc_value = name_map
            .as_borrowed()
            .get_loose(name)
            .ok_or(ParseError::unknown_property())?;
        let property_map =
            load_general_category(self.property_provider).map_err(|_| ParseError::internal())?;
        let set = property_map.as_borrowed().get_set_for_value(gc_value);
        Ok(set)
    }

    fn try_load_script(&self, name: &str) -> Result<CodePointSetData> {
        let name_map = Script::get_name_to_enum_mapper(self.property_provider)
            .map_err(|_| ParseError::internal())?;
        let sc_value = name_map
            .as_borrowed()
            .get_loose(name)
            .ok_or(ParseError::unknown_property())?;
        let property_map =
            load_script(self.property_provider).map_err(|_| ParseError::internal())?;
        let set = property_map.as_borrowed().get_set_for_value(sc_value);
        Ok(set)
    }
}

pub fn parse(source: &str) -> Result<CodePointInversionListAndStringList<'static>> {
    // TODO: turn this into an arg
    let provider = icu_testdata::unstable();

    let mut iter = source.char_indices().peekable();
    let mut builder = UnicodeSetBuilder::new_inner(&mut iter, &provider);

    builder.parse_unicode_set()?;
    let (mut single, multi) = builder.finalize();
    let built_single = single.build();

    let mut strings = multi.into_iter().collect::<Vec<_>>();
    strings.sort();
    let zerovec = (&strings).into();

    // debug things (TODO: delete):
    eprintln!("UnicodeSet: {source}");
    eprintln!("Single:");
    eprint!("[");
    for c in built_single.iter_chars() {
        eprint!("{},", c);
    }
    eprintln!("]");
    if !strings.is_empty() {
        eprintln!("Strings:");
        eprint!("[");
        for s in &strings {
            eprint!("{{{}}},", s);
        }
        eprintln!("]");
    }

    let cpinvlistandstrlist = CodePointInversionListAndStringList::try_from(built_single, zerovec)
        .map_err(|_| ParseError::internal())?;

    // todo continue here, build set, return it, impl debug on stuff, and have test run

    // convert builder.single and builder.multi to an actual CodePointInversionListAndStringList

    Ok(cpinvlistandstrlist)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_contains<'a, 'b>(
        cpinvlistandstrlist: &CodePointInversionListAndStringList<'a>,
        single: impl Iterator<Item = char>,
        multi: impl Iterator<Item = &'b str>,
    ) {
        let mut it_size = 0;
        for c in single {
            it_size += 1;
            assert!(cpinvlistandstrlist.contains_char(c));
        }
        for s in multi {
            it_size += 1;
            assert!(cpinvlistandstrlist.contains(s));
        }
        assert_eq!(cpinvlistandstrlist.size(), it_size);
    }

    #[test]
    fn test_parse() {
        let parsed = parse("[a-z]").unwrap();
        let v: Vec<String> = vec![];
        assert_contains(&parsed, 'a'..='z', v.iter().map(|s| s.as_str()));
    }

    // TODO: delete
    #[test]
    fn test_playground() {
        parse("[a-zA-Z : ]").unwrap();
        parse(
            "[a-zA-Z[^043]&[-2]-[-]{   h\
        ell o}]",
        )
        .unwrap();
        parse("[[abc][def]-[abc][def]]").unwrap();
        parse("[^[^]]").unwrap();
        parse("[:g c =Lowe rCASEl etter:]").unwrap();
        parse("[[:g c ≠Lowe rCASEl etter:]&[0-z]]").unwrap();
        parse("[:ll:]").unwrap();
        parse(r"\p{ll}").unwrap();
        parse("[:Case_Ignorable:]").unwrap();
        parse("[[:Case_Ignorable=false:]&[0-Z]]").unwrap();
        parse("[[:^Case_Ignorable=false:]&[0-Z]]").unwrap();
        parse(r"[\\ \  \[]").unwrap();
        parse(r"[{A \u0308}]").unwrap();
        parse(r"[{A \U00000308}]").unwrap();
        parse(r"[{\x61 \U00000308}]").unwrap();
        parse(r"[\x61{\x61 \U00000308}]").unwrap();
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
