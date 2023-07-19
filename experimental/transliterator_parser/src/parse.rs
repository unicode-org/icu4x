// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: remove this
#![allow(unused)]

use std::collections::HashMap;
use std::{iter::Peekable, str::CharIndices};

use icu_collections::{
    codepointinvlist::CodePointInversionList,
    codepointinvliststringlist::CodePointInversionListAndStringList,
};
use icu_properties::provider::*;
use icu_properties::sets::{load_pattern_white_space, load_xid_continue, load_xid_start};
use icu_provider::prelude::*;

/// The kind of error that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// An unexpected character was encountered. This variant implies the other variants
    /// (notably `UnknownProperty` and `Unimplemented`) do not apply.
    UnexpectedChar(char),
    /// A reference to an unknown variable.
    UnknownVariable,
    /// The source is an incomplete unicode set.
    Eof,
    /// Something unexpected went wrong with our code. Please file a bug report on GitHub.
    Internal,
    /// The provided syntax is not supported by us. Note that unknown properties will return the
    /// `UnknownProperty` variant, not this one.
    Unimplemented,
    /// The provided escape sequence is not a valid Unicode code point or represents too many
    /// code points.
    InvalidEscape,
    /// Invalid UnicodeSet syntax. See `icu_unicodeset_parser`'s [`ParseErrorKind`](icu_unicodeset_parser::ParseErrorKind).
    UnicodeSetError(icu_unicodeset_parser::ParseErrorKind),
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
#[derive(Debug, Clone, Copy)]
pub struct ParseError {
    // offset is the index to an arbitrary byte in the last character in the source that makes sense
    // to display as location for the error, e.g., the unexpected character itself or
    // for an unknown property name the last character of the name.
    offset: Option<usize>,
    kind: ParseErrorKind,
}

type Result<T, E = ParseError> = core::result::Result<T, E>;

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
}

type Section = Vec<Element>;

#[derive(Debug, Clone)]
pub(crate) struct HalfRule {
    ante: Section,
    key: Section,
    post: Section,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Dir {
    Forward,
    Backward,
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
    Conversion(HalfRule, Dir, HalfRule),
    VariableDefinition(String, Section),
}

type ParseVariableMap = HashMap<String, Section>;

struct TransliteratorParser<'a, 'b, P: ?Sized> {
    // TODO: if we also keep a reference to source we can pass that onto the UnicodeSet parser without allocating
    iter: &'a mut Peekable<CharIndices<'b>>,
    variable_map: ParseVariableMap,
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
            xid_start,
            xid_continue,
            pat_ws,
            property_provider: provider,
        }
    }

    fn parse_rules(&mut self) -> Result<Vec<Rule>> {
        todo!()
    }

    fn parse_rule(&mut self) -> Result<Rule> {
        todo!()
    }

    // any rules starting with '::'
    fn parse_filter_or_transform_rule(&mut self) -> Result<Rule> {
        todo!()
    }

    // TODO: factor this out for runtime ID parsing?
    fn parse_basic_id(&mut self) -> Result<BasicId> {
        todo!()
    }

    fn parse_half_rule(&mut self) -> Result<HalfRule> {
        todo!()
    }

    fn parse_dir(&mut self) -> Result<Dir> {
        todo!()
    }

    fn parse_section(&mut self) -> Result<Section> {
        todo!()
    }

    fn parse_element(&mut self) -> Result<Element> {
        todo!()
    }

    fn parse_variable_or_backref(&mut self) -> Result<Element> {
        todo!()
    }

    fn parse_quoted_literal(&mut self) -> Result<String> {
        todo!()
    }

    // parses all supported escapes. code is somewhat duplicated from icu_unicodeset_parser
    fn parse_escaped_char(&mut self) -> Result<char> {
        todo!()
    }

    fn parse_segment(&mut self) -> Result<Element> {
        todo!()
    }

    fn parse_unicode_set(&mut self) -> Result<UnicodeSet> {
        todo!()
    }

    fn parse_function_call(&mut self) -> Result<Element> {
        todo!()
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
