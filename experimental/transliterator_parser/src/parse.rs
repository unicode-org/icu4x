// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;
use std::{iter::Peekable, str::CharIndices};

use icu_collections::{
    codepointinvlist::CodePointInversionList,
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_properties::provider::*;
use icu_properties::sets::{load_pattern_white_space, load_xid_continue, load_xid_start};
use icu_provider::prelude::*;
use icu_unicodeset_parser::VariableMap;

/// The kind of error that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// An unexpected character was encountered. This variant implies the other variants
    /// (notably `UnknownProperty` and `Unimplemented`) do not apply.
    UnexpectedChar(char),
    /// A reference to an unknown variable.
    UnknownVariable,
    /// The source is incomplete.
    Eof,
    /// Something unexpected went wrong with our code. Please file a bug report on GitHub.
    Internal,
    /// The provided syntax is not supported by us. Please file an issue on GitHub if you need
    /// this feature.
    Unimplemented,
    /// The provided escape sequence is not a valid Unicode code point.
    InvalidEscape,
    /// The provided transform ID is invalid.
    InvalidId,
    /// The provided number is invalid, which likely means it's too big.
    InvalidNumber,
    /// Duplicate variable definition.
    DuplicateVariable,
    /// Invalid UnicodeSet syntax. See `icu_unicodeset_parser`'s [`ParseError`](icu_unicodeset_parser::ParseError).
    UnicodeSetError(icu_unicodeset_parser::ParseError),
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

/// The error type returned by the `parse` functions in this crate.
#[derive(Debug, Clone, Copy)]
pub struct ParseError {
    // offset is the index to an arbitrary byte in the last character in the source that makes sense
    // to display as location for the error, e.g., the unexpected character itself or
    // for an unknown property name the last character of the name.
    offset: Option<usize>,
    kind: ParseErrorKind,
}

impl ParseError {
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

impl From<ParseErrorKind> for ParseError {
    fn from(kind: ParseErrorKind) -> Self {
        ParseError { offset: None, kind }
    }
}

impl From<icu_unicodeset_parser::ParseError> for ParseError {
    fn from(e: icu_unicodeset_parser::ParseError) -> Self {
        ParseError {
            offset: None,
            kind: PEK::UnicodeSetError(e),
        }
    }
}

type Result<T, E = ParseError> = core::result::Result<T, E>;

// the only UnicodeSets used in this crate are parsed, and thus 'static.
type UnicodeSet = CodePointInversionListAndStringList<'static>;

#[derive(Debug, Clone, Copy)]
pub(crate) enum QuantifierKind {
    // ?
    ZeroOrOne,
    // *
    ZeroOrMore,
    // +
    OneOrMore,
}

// source-target/variant
#[derive(Debug, Clone)]
pub(crate) struct BasicId {
    source: String,
    target: String,
    variant: String,
}

impl Default for BasicId {
    fn default() -> Self {
        Self {
            source: "Any".to_string(),
            target: "Null".to_string(),
            variant: "".to_string(),
        }
    }
}

// [set] source-target/variant
#[derive(Debug, Clone)]
pub(crate) struct SingleId {
    filter: Option<UnicodeSet>,
    basic_id: BasicId,
}

#[derive(Debug, Clone)]
pub(crate) enum Element {
    // Examples:
    //  - hello\ world
    //  - 'hello world'
    Literal(String),
    // Example: $my_var
    VariableRef(String),
    // Example: $12
    BackRef(u32),
    // Examples:
    //  - <element>?
    //  - <element>*
    //  - <element>+
    // note: Box<Element> instead of Section, because a quantifier only ever refers to the immediately preceding element.
    // segments or variable refs are used to group multiple elements together.
    Quantifier(QuantifierKind, Box<Element>),
    // Example: (<element> <element> ...)
    Segment(Section),
    // Example: [:^L:]
    UnicodeSet(UnicodeSet),
    // Example: &[a-z] Any-Remove(<element> <element> ...)
    // single id, function arguments
    FunctionCall(SingleId, Section),
    // '|', only valid on the output side
    Cursor,
    // '@', only valid on the output side
    Placeholder,
    // '^'
    AnchorStart,
    // '$'
    AnchorEnd,
}

type Section = Vec<Element>;

#[derive(Debug, Clone)]
pub(crate) struct HalfRule {
    ante: Section,
    key: Section,
    post: Section,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Forward,
    Reverse,
    Both,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Rule {
    GlobalFilter(UnicodeSet),
    GlobalInverseFilter(UnicodeSet),
    // forward and backward IDs.
    // "A (B)" is Transform(A, Some(B)),
    // "(B)" is Transform(Null, Some(B)),
    // "A" is Transform(A, None),
    // "A ()" is Transform(A, Some(Null))
    Transform(SingleId, Option<SingleId>),
    Conversion(HalfRule, Direction, HalfRule),
    VariableDefinition(String, Section),
}

type ParseVariableMap = HashMap<String, Section>;

#[derive(Debug, Clone)]
enum UsetVariableValue {
    String(String),
    UnicodeSet(UnicodeSet),
}
type UsetVariableMap = HashMap<String, UsetVariableValue>;

struct TransliteratorParser<'a, 'b, P: ?Sized> {
    // TODO: if we also keep a reference to source we can pass that onto the UnicodeSet parser without allocating
    iter: &'a mut Peekable<CharIndices<'b>>,
    variable_map: ParseVariableMap,
    // flattened variable map specifically for unicodesets, i.e., only contains variables that
    // are chars, strings, or UnicodeSets when all variables are inlined.
    uset_variable_map: UsetVariableMap,
    // cached set for the special set .
    dot_set: Option<UnicodeSet>,
    // for variable identifiers (XID Start, XID Continue)
    xid_start: &'a CodePointInversionList<'a>,
    xid_continue: &'a CodePointInversionList<'a>,
    // for skipped whitespace (Pattern White Space)
    pat_ws: &'a CodePointInversionList<'a>,
    property_provider: &'a P,
}

impl<'a, 'b, P> TransliteratorParser<'a, 'b, P>
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
    // initiates a line comment
    const COMMENT: char = '#';
    // terminates a line comment
    const COMMENT_END: char = '\n';
    // terminates a rule
    const RULE_END: char = ';';
    // initiates a filter or transform rule, as part of '::'
    const SPECIAL_START: char = ':';
    // initiates a UnicodeSet
    // TODO: does \p (and \P) work to initiate a UnicodeSet?
    const SET_START: char = '[';
    // equivalent to the UnicodeSet [^[:Zp:][:Zl:]\r\n$]
    const DOT: char = '.';
    const DOT_SET: &'static str = r"[^[:Zp:][:Zl:]\r\n$]";
    // matches the beginning of the input
    const ANCHOR_START: char = '^';
    // initiates a segment or the reverse portion of an ID
    const OPEN_PAREN: char = '(';
    // terminates a segment or the reverse portion of an ID
    const CLOSE_PAREN: char = ')';
    // separates source and target of an ID
    const ID_SEP: char = '-';
    // separates variant from ID
    const VARIANT_SEP: char = '/';
    // variable reference prefix, and anchor end character
    const VAR_PREFIX: char = '$';
    // variable definition operator
    const VAR_DEF_OP: char = '=';
    // left context
    const LEFT_CONTEXT: char = '{';
    // right context
    const RIGHT_CONTEXT: char = '}';
    // optional quantifier
    const OPTIONAL: char = '?';
    // zero or more quantifier
    const ZERO_OR_MORE: char = '*';
    // one or more quantifier
    const ONE_OR_MORE: char = '+';
    // function prefix
    const FUNCTION_PREFIX: char = '&';
    // quoted literals
    const QUOTE: char = '\'';
    // escape character
    const ESCAPE: char = '\\';

    fn new(
        iter: &'a mut Peekable<CharIndices<'b>>,
        xid_start: &'a CodePointInversionList<'a>,
        xid_continue: &'a CodePointInversionList<'a>,
        pat_ws: &'a CodePointInversionList<'a>,
        provider: &'a P,
    ) -> Self {
        Self {
            iter,
            variable_map: Default::default(),
            uset_variable_map: Default::default(),
            dot_set: None,
            xid_start,
            xid_continue,
            pat_ws,
            property_provider: provider,
        }
    }

    fn parse_rules(&mut self) -> Result<Vec<Rule>> {
        let mut rules = Vec::new();

        loop {
            self.skip_whitespace();
            if self.iter.peek().is_none() {
                break;
            }
            // we skipped whitespace and comments, so any other chars must be part of a rule
            rules.push(self.parse_rule()?);
        }

        Ok(rules)
    }

    // expects a rule
    fn parse_rule(&mut self) -> Result<Rule> {
        match self.must_peek_char()? {
            Self::SPECIAL_START => self.parse_filter_or_transform_rule(),
            // must be a conversion or variable rule
            _ => self.parse_conversion_or_variable_rule(),
        }
    }

    // any rules starting with '::'
    fn parse_filter_or_transform_rule(&mut self) -> Result<Rule> {
        // Syntax:
        // '::' <unicodeset> ';'                  # global filter
        // '::' '(' <unicodeset> ')' ';'          # global inverse filter
        // '::' <single-id> (<single-id>)? ';'    # transform rule

        self.consume(Self::SPECIAL_START)?;
        self.consume(Self::SPECIAL_START)?;

        // because all three options can start with a UnicodeSet, we just try to parse everything
        // into options, and assemble at the end

        let (forward_filter, forward_basic_id, reverse_filter, reverse_basic_id, has_reverse) =
            self.parse_filter_or_transform_rule_parts()?;

        self.skip_whitespace();

        // the offset of ';'
        let meta_err_offset = self.must_peek_index()?;
        self.consume(Self::RULE_END)?;

        // try to assemble the rule
        // first try global filters
        match (
            forward_filter.is_some(),
            forward_basic_id.is_some(),
            reverse_filter.is_some(),
            reverse_basic_id.is_some(),
        ) {
            (true, false, false, false) => {
                // safety: by match, forward_filter.is_some() is true
                #[allow(clippy::unwrap_used)]
                return Ok(Rule::GlobalFilter(forward_filter.unwrap()));
            }
            (false, false, true, false) => {
                // safety: by match, reverse_filter.is_some() is true
                #[allow(clippy::unwrap_used)]
                return Ok(Rule::GlobalInverseFilter(reverse_filter.unwrap()));
            }
            _ => {}
        }

        // if this is not a global filter rule, this must be a transform rule

        // either forward_basic_id or reverse_basic_id must be nonempty
        if forward_basic_id.is_none() && reverse_basic_id.is_none() {
            return Err(PEK::InvalidId.with_offset(meta_err_offset));
        }

        if !has_reverse {
            // we must have a forward id due to:
            //  1. !has_reverse implying reverse_basic_id.is_none()
            //  2. the above none checks implying forward_basic_id.is_some()
            // because this is difficult to verify, returning a PEK::Internal anyway
            // instead of unwrapping, despite technically being unnecessary
            let forward_basic_id = forward_basic_id.ok_or(PEK::Internal)?;
            return Ok(Rule::Transform(
                SingleId {
                    basic_id: forward_basic_id,
                    filter: forward_filter,
                },
                None,
            ));
        }

        if forward_filter.is_some() && forward_basic_id.is_none()
            || reverse_filter.is_some() && reverse_basic_id.is_none()
        {
            // cannot have a filter without a basic id
            return Err(PEK::InvalidId.with_offset(meta_err_offset));
        }

        // an empty forward rule, such as ":: (R) ;" is equivalent to ":: Any-Null (R) ;"
        let forward_basic_id = forward_basic_id.unwrap_or(BasicId::default());
        // an empty reverse rule, such as ":: F () ;" is equivalent to ":: F (Any-Null) ;"
        let reverse_basic_id = reverse_basic_id.unwrap_or(BasicId::default());

        let forward_single_id = SingleId {
            basic_id: forward_basic_id,
            filter: forward_filter,
        };
        let reverse_single_id = SingleId {
            basic_id: reverse_basic_id,
            filter: reverse_filter,
        };

        Ok(Rule::Transform(forward_single_id, Some(reverse_single_id)))
    }

    // consumes everything between '::' and ';', exclusive.
    #[allow(clippy::type_complexity)] // used internally in one place only
    fn parse_filter_or_transform_rule_parts(
        &mut self,
    ) -> Result<(
        Option<UnicodeSet>,
        Option<BasicId>,
        Option<UnicodeSet>,
        Option<BasicId>,
        bool,
    )> {
        // parse forward things, i.e., everything until Self::OPEN_PAREN
        self.skip_whitespace();
        let forward_filter = self.try_parse_unicode_set()?;
        self.skip_whitespace();
        let forward_basic_id = self.try_parse_basic_id()?;
        self.skip_whitespace();

        let has_reverse = match self.must_peek_char()? {
            // initiates a reverse id
            Self::OPEN_PAREN => true,
            // we're done parsing completely, no reverse id
            Self::RULE_END => false,
            _ => return self.unexpected_char_here(),
        };

        let reverse_filter;
        let reverse_basic_id;

        if has_reverse {
            // if we have a reverse, parse it
            self.consume(Self::OPEN_PAREN)?;
            self.skip_whitespace();
            reverse_filter = self.try_parse_unicode_set()?;
            self.skip_whitespace();
            reverse_basic_id = self.try_parse_basic_id()?;
            self.skip_whitespace();
            self.consume(Self::CLOSE_PAREN)?;
        } else {
            reverse_filter = None;
            reverse_basic_id = None;
        }

        Ok((
            forward_filter,
            forward_basic_id,
            reverse_filter,
            reverse_basic_id,
            has_reverse,
        ))
    }

    fn parse_conversion_or_variable_rule(&mut self) -> Result<Rule> {
        // Syntax:
        // <variable_ref> '=' <section> ';'           # variable rule
        // <half-rule> <direction> <half-rule> ';'    # conversion rule

        // try parsing into a variable rule
        eprintln!("here");

        let first_elt = if Self::VAR_PREFIX == self.must_peek_char()? {
            let elt = self.parse_variable_or_backref_or_anchor_end()?;
            self.skip_whitespace();
            if Self::VAR_DEF_OP == self.must_peek_char()? {
                // must be variable ref
                let var_name = match elt {
                    Element::VariableRef(var_name) => var_name,
                    _ => return self.unexpected_char_here(),
                };
                self.iter.next();
                let section = self.parse_section(None)?;
                let err_offset = self.must_peek_index()?;
                self.consume(Self::RULE_END)?;
                self.add_variable(var_name.clone(), section.clone(), err_offset);
                return Ok(Rule::VariableDefinition(var_name, section));
            }
            Some(elt)
        } else {
            None
        };
        eprintln!("here");

        // must be conversion rule
        // passing down first_elt that was already parsed for the variable rule check
        let first_half = self.parse_half_rule(first_elt)?;
        eprintln!("here");

        let dir = self.parse_direction()?;
        eprintln!("here");

        let second_half = self.parse_half_rule(None)?;
        self.consume(Self::RULE_END)?;
        Ok(Rule::Conversion(first_half, dir, second_half))
    }

    fn parse_single_id(&mut self) -> Result<SingleId> {
        // Syntax:
        // <unicodeset>? <basic-id>

        self.skip_whitespace();
        let filter = self.try_parse_unicode_set()?;
        self.skip_whitespace();
        let basic_id = self.parse_basic_id()?;
        Ok(SingleId { filter, basic_id })
    }

    fn try_parse_basic_id(&mut self) -> Result<Option<BasicId>> {
        if let Some(c) = self.peek_char() {
            if self.xid_start.contains(c) {
                return Ok(Some(self.parse_basic_id()?));
            }
        }
        Ok(None)
    }

    // TODO: factor this out for runtime ID parsing?
    fn parse_basic_id(&mut self) -> Result<BasicId> {
        // Syntax:
        // <identifier> ('-' <identifier>)? ('/' <identifier>)?

        // we must have at least one identifier. the implicit "Null" id is only allowed
        // in a '::'-rule, which is handled explicitly.
        let mut first_id = self.parse_unicode_identifier()?;

        self.skip_whitespace();
        let second_id = self.try_parse_sep_and_unicode_identifier(Self::ID_SEP)?;
        self.skip_whitespace();
        let variant_id = self.try_parse_sep_and_unicode_identifier(Self::VARIANT_SEP)?;

        let (source, target) = match second_id {
            None => ("Any".to_string(), first_id),
            Some(second_id) => (first_id, second_id),
        };

        Ok(BasicId {
            source,
            target,
            variant: variant_id.unwrap_or("".to_string()),
        })
    }

    fn try_parse_sep_and_unicode_identifier(&mut self, sep: char) -> Result<Option<String>> {
        if Some(sep) == self.peek_char() {
            self.iter.next();
            self.skip_whitespace();
            // at this point we must be parsing a identifier
            return Ok(Some(self.parse_unicode_identifier()?));
        }
        Ok(None)
    }

    // parses an XID-based identifier
    fn parse_unicode_identifier(&mut self) -> Result<String> {
        // Syntax:
        // <xid_start> (<xid_continue>)*

        let mut id = String::new();

        let (first_offset, first_c) = self.must_peek()?;
        if !self.xid_start.contains(first_c) {
            return Err(PEK::UnexpectedChar(first_c).with_offset(first_offset));
        }
        self.iter.next();
        id.push(first_c);

        loop {
            let c = self.must_peek_char()?;
            if !self.xid_continue.contains(c) {
                break;
            }
            id.push(c);
            self.iter.next();
        }

        Ok(id)
    }

    fn parse_half_rule(&mut self, prev_elt: Option<Element>) -> Result<HalfRule> {
        // Syntax:
        // (<section> '{')? <section> ('}' <section>)?

        let ante;
        let key;
        let post;
        eprintln!("before");
        let first = self.parse_section(prev_elt)?;
        eprintln!("after");
        if Self::LEFT_CONTEXT == self.must_peek_char()? {
            self.iter.next();
            ante = first;
            key = self.parse_section(None)?;
        } else {
            ante = vec![];
            key = first;
        }
        if Self::RIGHT_CONTEXT == self.must_peek_char()? {
            self.iter.next();
            post = self.parse_section(None)?;
        } else {
            post = vec![];
        }

        Ok(HalfRule { ante, key, post })
    }

    fn parse_direction(&mut self) -> Result<Direction> {
        // Syntax:
        // '<' | '>' | '<>' | '→' | '←' | '↔'

        match self.must_peek_char()? {
            '>' | '→' => {
                self.iter.next();
                Ok(Direction::Forward)
            }
            '↔' => {
                self.iter.next();
                Ok(Direction::Both)
            }
            '←' => {
                self.iter.next();
                Ok(Direction::Reverse)
            }
            '<' => {
                self.iter.next();
                match self.must_peek_char()? {
                    '>' => {
                        self.iter.next();
                        Ok(Direction::Both)
                    }
                    _ => Ok(Direction::Reverse),
                }
            }
            _ => self.unexpected_char_here(),
        }
    }

    // whitespace before and after is consumed
    fn parse_section(&mut self, prev_elt: Option<Element>) -> Result<Section> {
        let mut section = Section::new();
        let mut prev_elt = prev_elt;

        loop {
            self.skip_whitespace();
            let c = self.must_peek_char()?;
            eprintln!("next_char {c}");
            if self.is_section_end(c) {
                if let Some(elt) = prev_elt.take() {
                    section.push(elt);
                }
                break;
            }

            let next_elt = self.parse_element(&mut prev_elt)?;

            if let Some(elt) = prev_elt {
                section.push(elt);
            }
            eprintln!("{:?}", section);
            prev_elt = Some(next_elt);
        }

        Ok(section)
    }

    fn parse_quantifier_kind(&mut self) -> Result<QuantifierKind> {
        match self.must_peek_char()? {
            Self::OPTIONAL => {
                self.iter.next();
                Ok(QuantifierKind::ZeroOrOne)
            }
            Self::ZERO_OR_MORE => {
                self.iter.next();
                Ok(QuantifierKind::ZeroOrMore)
            }
            Self::ONE_OR_MORE => {
                self.iter.next();
                Ok(QuantifierKind::OneOrMore)
            }
            _ => self.unexpected_char_here(),
        }
    }

    fn parse_element(&mut self, prev_elt: &mut Option<Element>) -> Result<Element> {
        match self.must_peek_char()? {
            Self::VAR_PREFIX => {
                let elt = self.parse_variable_or_backref_or_anchor_end()?;
                Ok(elt)
            }
            Self::ANCHOR_START => {
                self.iter.next();
                Ok(Element::AnchorStart)
            }
            Self::SET_START => Ok(Element::UnicodeSet(self.parse_unicode_set()?)),
            Self::OPEN_PAREN => {
                Ok(self.parse_segment()?)
            }
            Self::DOT => {
                self.iter.next();
                Ok(Element::UnicodeSet(self.get_dot_set()?))
            }
            c @ (Self::OPTIONAL | Self::ZERO_OR_MORE | Self::ONE_OR_MORE) => {
                let quantifier = self.parse_quantifier_kind()?;
                if let Some(elt) = prev_elt.take() {
                    Ok(Element::Quantifier(quantifier, Box::new(elt)))
                } else {
                    self.unexpected_char_here()
                }
            }
            Self::FUNCTION_PREFIX => Ok(self.parse_function_call()?),
            Self::QUOTE => Ok(Element::Literal(self.parse_quoted_literal()?)),
            c if self.is_valid_unquoted_literal(c) => Ok(Element::Literal(self.parse_literal()?)),
            _ => self.unexpected_char_here(),
        }
    }

    fn parse_variable_or_backref_or_anchor_end(&mut self) -> Result<Element> {
        self.consume(Self::VAR_PREFIX)?;

        match self.must_peek_char()? {
            c if c.is_ascii_digit() => {
                // we have a backref
                let num = self.parse_number()?;
                Ok(Element::BackRef(num))
            }
            c if self.xid_start.contains(c) => {
                // we have a variable
                let variable_id = self.parse_unicode_identifier()?;
                Ok(Element::VariableRef(variable_id))
            }
            _ => {
                // this was an anchor end
                Ok(Element::AnchorEnd)
            }
        }
    }

    fn parse_number(&mut self) -> Result<u32> {
        let mut buf = String::new();
        let (first_offset, first_c) = self.must_next()?;
        if !matches!(first_c, '1'..='9') {
            return Err(PEK::UnexpectedChar(first_c).with_offset(first_offset));
        }
        buf.push(first_c);

        loop {
            let c = self.must_peek_char()?;
            if !c.is_ascii_digit() {
                break;
            }
            buf.push(c);
            self.iter.next();
        }

        buf.parse().map_err(|_| PEK::InvalidNumber.into())
    }

    fn parse_literal(&mut self) -> Result<String> {
        let mut buf = String::new();
        loop {
            self.skip_whitespace();
            let c = self.must_peek_char()?;
            if c == Self::ESCAPE {
                buf.push(self.parse_escaped_char()?);
                continue;
            }
            if !self.is_valid_unquoted_literal(c) {
                break;
            }
            self.iter.next();
            buf.push(c);
        }
        Ok(buf)
    }

    fn parse_quoted_literal(&mut self) -> Result<String> {
        // Syntax:
        // \' [^']* \'

        let mut buf = String::new();
        self.consume(Self::QUOTE)?;
        loop {
            let c = self.must_next_char()?;
            if c == Self::QUOTE {
                break;
            }
            buf.push(c);
        }
        if buf.is_empty() {
            // '' is the escaped version of a quote
            buf.push(Self::QUOTE);
        }
        Ok(buf)
    }

    // parses all supported escapes. code is somewhat duplicated from icu_unicodeset_parser
    // but multi-escapes are not supported
    fn parse_escaped_char(&mut self) -> Result<char> {
        self.consume(Self::ESCAPE)?;

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
                let hex_digits = match self.must_peek_char()? {
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
                    _ => return self.unexpected_char_here(),
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

    fn parse_segment(&mut self) -> Result<Element> {
        self.consume(Self::OPEN_PAREN)?;
        let elt = Element::Segment(self.parse_section(None)?);
        self.consume(Self::CLOSE_PAREN)?;
        Ok(elt)
    }

    fn try_parse_unicode_set(&mut self) -> Result<Option<UnicodeSet>> {
        if Some(Self::SET_START) == self.peek_char() {
            return Ok(Some(self.parse_unicode_set()?));
        }
        Ok(None)
    }

    fn parse_unicode_set(&mut self) -> Result<UnicodeSet> {
        // TODO: we should really have a mechanism to allow the unicodeset parser to take over,
        //  and continue where it left off

        let mut set = String::new();
        self.consume('[')?;
        set.push('[');

        let mut depth = 1;
        let mut escaped = false;
        let mut inside_curly = false;

        loop {
            let c = self.must_next_char()?;
            set.push(c);
            match c {
                '[' if !escaped && !inside_curly => depth += 1,
                ']' if !escaped && !inside_curly => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                '{' if !escaped => inside_curly = true,
                '}' if !escaped && inside_curly => inside_curly = false,
                '\\' if !escaped => escaped = true,
                _ => escaped = false,
            }
        }

        self.unicode_set_from_str(&set)
    }

    fn get_dot_set(&mut self) -> Result<UnicodeSet> {
        match &self.dot_set {
            Some(set) => Ok(set.clone()),
            None => {
                let set = self
                    .unicode_set_from_str(Self::DOT_SET)
                    .map_err(|_| PEK::Internal)?;
                self.dot_set = Some(set.clone());
                Ok(set)
            }
        }
    }

    fn unicode_set_from_str(&self, set: &str) -> Result<UnicodeSet> {
        let set = icu_unicodeset_parser::parse_unstable_with_variables(
            set,
            &self.borrow_uset_variable_map(),
            self.property_provider,
        )?;
        Ok(set)
    }

    fn parse_function_call(&mut self) -> Result<Element> {
        self.consume(Self::FUNCTION_PREFIX)?;

        // parse single-id
        let single_id = self.parse_single_id()?;
        self.skip_whitespace();
        self.consume(Self::OPEN_PAREN)?;
        let section = self.parse_section(None)?;
        self.consume(Self::CLOSE_PAREN)?;

        Ok(Element::FunctionCall(single_id, section))
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

    fn add_variable(&mut self, name: String, value: Section, offset: usize) -> Result<()> {
        if self.variable_map.contains_key(&name) {
            return Err(PEK::DuplicateVariable.with_offset(offset));
        }

        if let Some(uset_value) = self.try_uset_flatten_section(&value) {
            self.uset_variable_map.insert(name.to_string(), uset_value);
        }

        // TODO: actually, during parsing we might not need this one, only the flattened one
        self.variable_map.insert(name, value);
        Ok(())
    }

    fn try_uset_flatten_section(&self, section: &Section) -> Option<UsetVariableValue> {
        // is this just a unicode set?
        if let [Element::UnicodeSet(set)] = &section[..] {
            return Some(UsetVariableValue::UnicodeSet(set.clone()));
        }
        // if it's just a variable that is already a valid uset variable, we return that
        if let [Element::VariableRef(name)] = &section[..] {
            if let Some(value) = self.uset_variable_map.get(name) {
                return Some(value.clone());
            }
            return None;
        }

        // if not, must be a string literal
        let mut combined_literal = String::new();
        for elt in section {
            match elt {
                Element::Literal(s) => combined_literal.push_str(s),
                Element::VariableRef(name) => {
                    if let Some(UsetVariableValue::String(s)) = self.uset_variable_map.get(name) {
                        combined_literal.push_str(s);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        Some(UsetVariableValue::String(combined_literal))
    }

    fn borrow_uset_variable_map(&self) -> VariableMap {
        let mut map = VariableMap::new();
        for (name, value) in self.uset_variable_map.iter() {
            match value {
                UsetVariableValue::UnicodeSet(uset) => {
                    map.insert_set(name.to_string(), uset.clone());
                }
                UsetVariableValue::String(s) => {
                    map.insert_str(name.to_string(), s);
                }
            }
        }
        map
    }

    fn consume(&mut self, expected: char) -> Result<()> {
        match self.must_next()? {
            (offset, c) if c != expected => Err(PEK::UnexpectedChar(c).with_offset(offset)),
            _ => Ok(()),
        }
    }

    // skips whitespace and comments
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == Self::COMMENT {
                self.skip_until(Self::COMMENT_END);
            }
            if !self.pat_ws.contains(c) {
                break;
            }
            self.iter.next();
        }
    }

    // skips until the next occurrence of c, which is also consumed
    fn skip_until(&mut self, end: char) {
        for (_, c) in self.iter.by_ref() {
            if c == end {
                break;
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.iter.peek().map(|(_, c)| *c)
    }

    fn peek_offset(&mut self) -> Option<usize> {
        self.iter.peek().map(|(i, _)| *i)
    }

    // use this whenever an empty iterator would imply an Eof error
    fn must_next(&mut self) -> Result<(usize, char)> {
        self.iter.next().ok_or(PEK::Eof.into())
    }

    // see must_next
    fn must_next_char(&mut self) -> Result<char> {
        self.must_next().map(|(_, c)| c)
    }

    // use this whenever an empty iterator would imply an Eof error
    fn must_peek(&mut self) -> Result<(usize, char)> {
        self.iter.peek().copied().ok_or(PEK::Eof.into())
    }

    // see must_peek
    fn must_peek_char(&mut self) -> Result<char> {
        self.must_peek().map(|(_, c)| c)
    }

    // see must_peek
    fn must_peek_index(&mut self) -> Result<usize> {
        self.must_peek().map(|(idx, _)| idx)
    }

    fn unexpected_char_here<T>(&mut self) -> Result<T> {
        let (offset, char) = self.must_peek()?;
        Err(PEK::UnexpectedChar(char).with_offset(offset))
    }

    fn is_section_end(&self, c: char) -> bool {
        matches!(
            c,
            Self::RULE_END
                | Self::CLOSE_PAREN
                | Self::RIGHT_CONTEXT
                | Self::LEFT_CONTEXT
                | Self::VAR_DEF_OP
                | '<'
                | '>'
                | '→'
                | '←'
                | '↔'
        )
    }

    fn is_valid_unquoted_literal(&self, c: char) -> bool {
        // allowing \ since it's used for escapes, which are allowed in an unquoted context
        c.is_ascii() && (c.is_ascii_alphanumeric() || c == '\\')
            || (!c.is_ascii() && c != '→' && c != '←' && c != '↔')
    }
}

#[cfg(feature = "compiled_data")]
pub(crate) fn parse(source: &str) -> Result<Vec<Rule>> {
    parse_unstable(source, &icu_properties::provider::Baked)
}

pub(crate) fn parse_unstable<P>(source: &str, provider: &P) -> Result<Vec<Rule>>
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
    let mut iter = source.char_indices().peekable();

    let xid_start = load_xid_start(provider).map_err(|_| PEK::Internal)?;
    let xid_start_list = xid_start.to_code_point_inversion_list();
    let xid_continue = load_xid_continue(provider).map_err(|_| PEK::Internal)?;
    let xid_continue_list = xid_continue.to_code_point_inversion_list();

    let pat_ws = load_pattern_white_space(provider).map_err(|_| PEK::Internal)?;
    let pat_ws_list = pat_ws.to_code_point_inversion_list();

    let mut parser = TransliteratorParser::new(
        &mut iter,
        &xid_start_list,
        &xid_continue_list,
        &pat_ws_list,
        provider,
    );
    parser.parse_rules()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let source = r##"
        :: [a-z\]] ; :: [b-z] Latin/BGN ;
        :: Source-Target/Variant () ;::([b-z]Target-Source/Variant) ;
        :: [a-z] Any ([b-z] Target-Source/Variant);

        $my_var = an arbitrary section ',' some quantifiers *+? 'and other variables: $var' $var  ;
        $innerMinus = '-' ;
        $minus = $innerMinus ;
        $good_set = [a $minus z] ;

        ^ (start) { key ' key '+ $good_set } > $102 }  post\-context$;
        # contexts are optional
        target < source ;
        # contexts can be empty
        { 'source-or-target' } <> { 'target-or-source' } ;

        (nested (sections)+ are () so fun) > ;

        . > ;

        :: ([{Inverse]-filter}]) ;
        "##;

        if let Err(e) = parse(source) {
            panic!("Failed to parse {:?}: {:?}", source, e);
        }
    }

    #[test]
    fn test_conversion_rules_ok() {
        let sources = [
            r"a > b ;",
            r"a < b ;",
            r"a <> b ;",
            r"a → b ;",
            r"a ← b ;",
            r"a ↔ b ;",
            r"a \> > b ;",
            r"a \→ > b ;",
            r"{ a > b ;",
            r"{ a } > b ;",
            r"{ a } > { b ;",
            r"{ a } > { b } ;",
            r"^ pre [a-z] { a } post [$] $ > ^ [$] pre { b [b-z] } post $ ;",
            r"[äöü] > ;",
            r"([äöü]) > &Remove($1) ;",
            r"[äöü] { ([äöü]+) > &Remove($1) ;",
        ];

        for source in sources {
            if let Err(e) = parse(source) {
                panic!("Failed to parse {:?}: {:?}", source, e);
            }
        }
    }

    #[test]
    fn test_conversion_rules_err() {
        let sources = [
            r"a > > b ;",
            r"a >< b ;",
            r"(a > b) > b ;",
            r"a \← b ;",
            r"a ↔ { b > } ;",
            r"a ↔ { b > } ;",
            r"a > b",
        ];

        for source in sources {
            if let Ok(rules) = parse(source) {
                panic!("Parsed invalid source {:?}: {:?}", source, rules);
            }
        }
    }

    #[test]
    fn test_variable_rules_ok() {
        let sources = [
            r" $my_var = [a-z] ;",
            r"$my_var = äüöÜ ;",
            r"$my_var = [a-z] literal ; $other_var = [A-Z] [b-z];",
            r"$my_var = [a-z] ; $other_var = [A-Z] [b-z];",
            r"$my_var = [a-z] ; $other_var = $my_var + $2222;",
            r"$my_var = [a-z] ; $other_var = $my_var \+\ \$2222 \\ 'hello\';",
            r"
            $innerMinus = '-' ;
            $minus = $innerMinus ;
            $good_set = [a $minus z] ;
            ",
        ];

        for source in sources {
            if let Err(e) = parse(source) {
                panic!("Failed to parse {:?}: {:?}", source, e);
            }
        }
    }

    #[test]
    fn test_variable_rules_err() {
        let sources = [
            r" $ my_var = a ;",
            r" $my_var = a_2 ;",
            r"$my_var 2 = [a-z] literal ;",
            r"$my_var = [$doesnt_exist] ;",
        ];

        for source in sources {
            if let Ok(rules) = parse(source) {
                panic!("Parsed invalid source {:?}: {:?}", source, rules);
            }
        }
    }

    #[test]
    fn test_global_filters_ok() {
        let sources = [
            r":: [^\[$] ;",
            r":: [^\[{[}$] ;",
            r":: [^\[{]}$] ;",
            r":: [^\[{]\}]}$] ;",
            r":: ([^\[$]) ;",
            r":: ( [^\[$] ) ;",
            r":: [^[a-z[]][]] ;",
            r":: [^[a-z\[\]]\]] ;",
            r":: [^\]] ;",
        ];

        for source in sources {
            if let Err(e) = parse(source) {
                panic!("Failed to parse {:?}: {:?}", source, e);
            }
        }
    }

    #[test]
    fn test_global_filters_err() {
        let sources = [
            r":: [^\[$ ;",
            r":: [^[$] ;",
            r":: [^\[$]) ;",
            r":: ( [^\[$]  ;",
            r":: [^[a-z[]][]] [] ;",
            r":: [^[a-z\[\]]\]] ([a-z]);",
            r":: [a$-^\]] ;",
            r":: ( [] [] ) ;",
            r":: () [] ;",
        ];

        for source in sources {
            if let Ok(rules) = parse(source) {
                panic!("Parsed invalid source {:?}: {:?}", source, rules);
            }
        }
    }

    #[test]
    fn test_function_calls_ok() {
        let sources = [
            r"$fn = & Any-Any/Variant ($var literal 'quoted literal' $1) ;",
            r"$fn = &[a-z] Any-Any/Variant ($var literal 'quoted literal' $1) ;",
            r"$fn = &[a-z]Any-Any/Variant ($var literal 'quoted literal' $1) ;",
            r"$fn = &[a-z]Any/Variant ($var literal 'quoted literal' $1) ;",
            r"$fn = &Any/Variant ($var literal 'quoted literal' $1) ;",
            r"$fn = &[a-z]Any ($var literal 'quoted literal' $1) ;",
            r"$fn = &Any($var literal 'quoted literal' $1) ;",
        ];

        for source in sources {
            if let Err(e) = parse(source) {
                panic!("Failed to parse {:?}: {:?}", source, e);
            }
        }
    }

    #[test]
    fn test_function_calls_err() {
        let sources = [
            r"$fn = &[a-z]($var literal 'quoted literal' $1) ;",
            r"$fn = &[a-z] ($var literal 'quoted literal' $1) ;",
            r"$fn = &($var literal 'quoted literal' $1) ;",
        ];

        for source in sources {
            if let Ok(rules) = parse(source) {
                panic!("Parsed invalid source {:?}: {:?}", source, rules);
            }
        }
    }

    #[test]
    fn test_transform_rules_ok() {
        let sources = [
            ":: NFD; :: NFKC;",
            ":: Latin ;",
            ":: any - Latin;",
            ":: any - Latin/bgn;",
            ":: any - Latin/bgn ();",
            ":: any - Latin/bgn ([a-z] a-z);",
            ":: ([a-z] a-z);",
            ":: (a-z);",
            ":: (a-z / variant);",
            ":: [a-z] latin/variant (a-z / variant);",
            ":: [a-z] latin/variant (a-z / variant) ;",
            ":: [a-z] latin (  );",
            ":: [a-z] latin ;",
            "::[];",
        ];

        for source in sources {
            if let Err(e) = parse(source) {
                panic!("Failed to parse {:?}: {:?}", source, e);
            }
        }
    }

    #[test]
    fn test_transform_rules_err() {
        let sources = [
            r":: a a ;",
            r":: (a a) ;",
            r":: a - z - b ;",
            r":: ( a - z - b) ;",
            r":: [] ( a - z) ;",
            r":: a-z ( [] ) ;",
            r":: a-z / ( [] a-z ) ;",
            r":: Latin-ASCII/BGN Arab-Greek/UNGEGN ;",
            r":: (Latin-ASCII/BGN Arab-Greek/UNGEGN) ;",
        ];

        for source in sources {
            if let Ok(rules) = parse(source) {
                panic!("Parsed invalid source {:?}: {:?}", source, rules);
            }
        }
    }
}
