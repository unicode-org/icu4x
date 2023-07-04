// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parse::missingapis::unescape;
use core::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
// use super::*;

// TODO: parse transform rules and filter rules
// TODO: parse escaped characters

macro_rules! t {
    () => (
        Peekable<impl Iterator<Item = char> + Clone>
    );
}

pub(super) fn legal_top_level_char(c: char) -> bool {
    // As specified in: https://unicode.org/reports/tr35/tr35-general.html#Transform_Rules_Syntax
    // "All of the ASCII characters except numbers and letters are reserved for use in the rule syntax, as are the characters →, ←, ↔."
    (c.is_ascii() && c.is_ascii_alphanumeric())
        || (!c.is_ascii() && c != '→' && c != '←' && c != '↔')
}

fn skip_whitespace(it: &mut t!()) {
    // while let Some(&c) = it.peek() {
    //     if c.is_ascii_whitespace() {
    //         it.next();
    //     } else {
    //         break;
    //     }
    // }

    // Skip ascii_whitespace and comments
    while let Some(&c) = it.peek() {
        match c {
            c if c.is_ascii_whitespace() => {
                it.next();
            }
            '#' => {
                while let Some(&c) = it.peek() {
                    if c == '\n' {
                        break;
                    }
                    it.next();
                }
            }
            _ => break,
        }
    }
}

macro_rules! pl {
    () => {
        ParseLocation {
            file: file!(),
            line: line!(),
        }
    };
}

mod missingapis {
    use super::*;

    pub(super) fn unescapable(c: char) -> bool {
        matches!(c, 'n' | 'r' | 't')
    }

    pub(super) fn unescape(c: char) -> char {
        match c {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            _ => c,
        }
    }

    // conform to ID_Start property
    pub(super) fn is_id_start(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    // conform to ID_Continue property
    pub(super) fn is_id_continue(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    pub(super) fn parse_unicode_set(it: &mut t!()) -> Result<UnicodeSet> {
        let mut set = String::new();
        let Some('[') = it.next() else {
            return Err(ParseError::new(pl!(), PEK::Legacy));
        };
        set.push('[');

        let mut depth = 1;

        // parse until we find a closing bracket
        let mut escaped = false;
        loop {
            match it.next() {
                None => return Err(ParseError::new(pl!(), PEK::Legacy)),
                Some(']') if !escaped => {
                    set.push(']');
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Some('\\') if !escaped => escaped = true,
                Some('[') if !escaped => {
                    depth += 1;
                    set.push('[');
                }
                Some(c) => {
                    if escaped {
                        set.push('\\');
                        escaped = false;
                    }
                    set.push(c);
                }
            }
        }
        // dbg!(set.clone());
        Ok(set)
    }
}

// #[derive(Debug, Clone)]
// pub(super) enum ParseError {
//     UnexpectedEof,
//     UnexpectedChar(char),
//     Legacy,
// }

#[derive(Debug, Clone)]
pub(super) enum ParseErrorKind {
    UnexpectedEof,
    UnexpectedChar(char),
    Legacy,
}
// #[derive(Debug, Clone)]
// pub(super) enum ParseLocation {
//     LiteralQuoted,
//     LiteralUnquoted,
//     Literal,
//     UnicodeSet,
//     Variable,
//     PatternElement,
//     Pattern,
//     HalfRule,
//     Direction,
//     Rule,
// }
#[derive(Clone)]
pub(super) struct ParseLocation {
    file: &'static str,
    line: u32,
}
impl core::fmt::Debug for ParseLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\n[{}:{}]", self.file, self.line)
    }
}

use ParseErrorKind as PEK;

#[allow(unused)]
#[derive(Debug, Clone)]
pub(super) struct ParseError {
    pub(super) location: ParseLocation,
    pub(super) kind: ParseErrorKind,
}

impl ParseError {
    pub(super) fn new(location: ParseLocation, kind: ParseErrorKind) -> Self {
        Self { location, kind }
    }
}
type Result<T, E = ParseError> = core::result::Result<T, E>;

type Literal = String;
type UnicodeSet = String;

#[derive(Debug, Clone)]
pub(super) struct Cursor {
    pub(super) pre_spacing: u32,
    pub(super) post_spacing: u32,
}

#[derive(Debug, Clone)]
pub(super) enum PatternElement {
    Literal(Literal),
    UnicodeSet(UnicodeSet),
    Variable(String),
    Cursor(Cursor),
}

impl PatternElement {
    fn string(&self) -> String {
        match self {
            Self::Literal(l) => {
                // todo: refactor this function into one pass
                // check if any char needs escaping
                if l.chars().any(|c| c == '\'') {
                    // cannot escape in quoted literals, escape using \
                    let mut res = String::new();
                    for c in l.chars() {
                        if !legal_top_level_char(c) {
                            res.push('\\');
                        }
                        res.push(c);
                    }
                    res
                } else if !l.chars().all(legal_top_level_char) {
                    // needs escaping, can be escaped with quotes
                    format!("'{}'", l)
                } else {
                    // no escaping needed
                    l.clone()
                }
            }
            Self::UnicodeSet(u) => u.clone(),
            Self::Variable(v) => format!("${}", v),
            Self::Cursor(c) => {
                format!(
                    "{}|{}",
                    "@".repeat(c.pre_spacing as usize),
                    "@".repeat(c.post_spacing as usize)
                )
            }
        }
    }
}

fn parse_quoted_literal(it: &mut t!()) -> Result<Literal> {
    // dbg!(it.peek());
    // No escaping in quoted literals?
    let mut literal = String::new();
    match it.next() {
        None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
        Some('\'') => {}
        Some(c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
    }

    for c in it.by_ref() {
        if c == '\'' {
            break;
        } else {
            literal.push(c);
        }
    }

    // special case: '' is an escaped single quote
    if literal.is_empty() {
        literal.push('\'');
    }

    Ok(literal)
}

fn parse_unquoted_literal(it: &mut t!()) -> Result<Literal> {
    // dbg!(it.peek());
    let mut literal = String::new();
    // collect consecutive legal_top_level_chars and escaped any other chars
    while let Some(&c) = it.peek() {
        if legal_top_level_char(c) {
            literal.push(c);
            it.next();
        } else if c == '\\' {
            it.next();
            match it.next() {
                None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
                Some(c) if missingapis::unescapable(c) => literal.push(unescape(c)),
                Some(c) => literal.push(c),
            }
        } else {
            break;
        }
    }

    Ok(literal)
}

fn parse_literal(it: &mut t!()) -> Result<Literal> {
    // dbg!(it.peek());
    // a literal is either a sequence of characters that are not special or escaped,
    // or a quoted sequence of any chars
    match it.peek() {
        None => Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
        Some(&'\'') => parse_quoted_literal(it),
        Some(_) => parse_unquoted_literal(it),
    }
}

fn parse_unicode_set(it: &mut t!()) -> Result<UnicodeSet> {
    // dbg!(it.peek());
    missingapis::parse_unicode_set(it)
}

fn parse_variable(it: &mut t!()) -> Result<String> {
    // dbg!(it.peek());
    let mut name = String::new();
    match it.next() {
        Some('$') => {}
        Some(c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
        None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
    }

    match it.next() {
        // a variable has at least one character
        Some(c) if missingapis::is_id_start(c) => name.push(c),
        Some(c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
        None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
    }

    // take_while is a problem because it consumes the first char after the id-continue sequence
    // it.take_while(|c| missingapis::is_id_continue(*c))
    //     .for_each(|c| name.push(c));
    // so implement as peeking loop
    loop {
        match it.peek() {
            None => break,
            Some(&c) if missingapis::is_id_continue(c) => {
                it.next();
                name.push(c);
            }
            _ => break,
        }
    }

    Ok(name)
}

fn parse_cursor(it: &mut t!()) -> Result<Cursor> {
    // dbg!(it.peek());
    let mut pre_spacing = 0;
    let mut post_spacing = 0;
    match it.peek() {
        Some(&'@') | Some(&'|') => {}
        Some(&c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
        None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
    }

    skip_whitespace(it);
    while let Some('@') = it.peek() {
        it.next();
        skip_whitespace(it);
        pre_spacing += 1;
    }

    match it.next() {
        Some('|') => {}
        Some(c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
        None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
    }

    skip_whitespace(it);
    while let Some('@') = it.peek() {
        it.next();
        skip_whitespace(it);
        post_spacing += 1;
    }

    Ok(Cursor {
        pre_spacing,
        post_spacing,
    })
}

fn parse_pattern_element(it: &mut t!()) -> Result<PatternElement> {
    // dbg!(it.peek());
    match it.peek() {
        None => Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
        Some(&'$') => Ok(PatternElement::Variable(parse_variable(it)?)),
        Some(&'[') => Ok(PatternElement::UnicodeSet(parse_unicode_set(it)?)),
        Some(&'@') | Some(&'|') => Ok(PatternElement::Cursor(parse_cursor(it)?)),
        Some(&'\\') => {
            // need lookahead to decide between \p{x=y} (unicode set) and \escape (unqouted literal)
            let mut lookahead_it = it.clone();
            // consume \
            let _ = lookahead_it.next();
            match lookahead_it.next() {
                None => Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
                Some('p') => {
                    // Perl property syntax
                    Ok(PatternElement::UnicodeSet(parse_unicode_set(it)?))
                }
                Some(_) => Ok(PatternElement::Literal(parse_literal(it)?)),
            }
        }
        Some(_) => Ok(PatternElement::Literal(parse_literal(it)?)),
    }
}

#[derive(Debug, Clone)]
pub(super) struct Pattern(pub(super) Vec<PatternElement>);

impl Pattern {
    fn flat_empty(self) -> Option<Self> {
        if self.0.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn string(&self) -> String {
        self.0.iter().map(|e| e.string()).collect()
    }
}

fn is_pattern_end(c: char) -> bool {
    c == ';'
        || c == '>'
        || c == '<'
        || c == '→'
        || c == '←'
        || c == '↔'
        || c == '='
        || c == '{'
        || c == '}'
}

fn parse_pattern(it: &mut t!()) -> Result<Pattern> {
    // dbg!(it.peek());
    let mut elements = Vec::new();
    loop {
        skip_whitespace(it);
        match it.peek() {
            None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
            Some(&c) if is_pattern_end(c) => break,
            Some(_) => elements.push(parse_pattern_element(it)?),
        }
    }

    Ok(Pattern(elements))
}

#[derive(Debug, Clone)]
pub(super) struct HalfRule {
    pub(super) ante: Option<Pattern>,
    pub(super) key: Pattern,
    pub(super) post: Option<Pattern>,
    /* add cursor here, e.g. index into Pattern. that would imply a literal like "ab > aa|b" would
    be split up into Pattern([aa, b]) and cursor = 1
    a cursor could also be a special patternelement in the uncompiled rules, the same for spacing
    or spacing could be a property of the pattern element, like cursor.spacing_before = 0 and .spacing_after = 0 by default
    // TODO: compilation will need to do some runtime checks, e.g., the target key of a rule can never be a set,
    //       cursors may not exist in contexts or the source, spacing can only happen on either end of the target (e.g. a|@a is illegal, as is @|a)
    */
}

impl Display for HalfRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // ante { key } post
        if let Some(ante) = &self.ante {
            write!(f, "{} {{ ", ante.string())?;
        }
        write!(f, "{}", self.key.string())?;
        if let Some(post) = &self.post {
            write!(f, " }} {}", post.string())?;
        }
        Ok(())
    }
}

fn is_half_rule_end(c: char) -> bool {
    c == ';' || c == '>' || c == '<' || c == '→' || c == '←' || c == '↔' || c == '='
}

fn parse_half_rule(it: &mut t!()) -> Result<HalfRule> {
    // dbg!(it.peek());

    let pattern1 = parse_pattern(it)?;
    // ante and post are Option<Option<..>> to detect cases like "{ a { x > ;" (which are invalid)
    let mut ante = None;
    let mut key = pattern1;
    let mut post = None;

    loop {
        // loop invariant: the state after each iteration must represent a valid half rule, i.e.,
        // if the half rule is over, we are allowed to return the current state as valid half rule
        match it.peek() {
            None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
            Some(&c) if is_half_rule_end(c) => {
                // the half rule is over
                return Ok(HalfRule {
                    ante: ante.unwrap_or(None),
                    key,
                    post: post.unwrap_or(None),
                });
            }
            Some(&'{') if ante.is_none() && post.is_none() => {
                // the pattern that we parsed in the beginning was actually the ante, we're parsing the key now
                // also, post must not have been parsed yet. that ensures rules like "} x { > ;" throw an error
                it.next();
                ante = Some(key.flat_empty());
                key = parse_pattern(it)?;
            }
            Some(&'}') if post.is_none() => {
                // the next pattern we parse is the post context
                it.next();
                post = Some(parse_pattern(it)?.flat_empty());
            }
            Some(&c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct VariableDef {
    pub(super) name: String,
    pub(super) pattern: Pattern,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Direction {
    Forward,
    Reverse,
    Bidirectional,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Direction::Forward => write!(f, "→"),
            Direction::Reverse => write!(f, "←"),
            Direction::Bidirectional => write!(f, "↔"),
        }
    }
}

fn parse_direction(it: &mut t!()) -> Result<Direction> {
    // dbg!(it.peek());
    match it.next() {
        None => Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
        Some('>') => Ok(Direction::Forward),
        Some('<') => {
            // if <> then bidirectional
            if let Some(&'>') = it.peek() {
                it.next();
                return Ok(Direction::Bidirectional);
            }
            Ok(Direction::Reverse)
        }
        Some('→') => Ok(Direction::Forward),
        Some('←') => Ok(Direction::Reverse),
        Some('↔') => Ok(Direction::Bidirectional),
        Some(c) => Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
    }
}

#[derive(Debug, Clone)]
pub(super) enum RuleKind {
    Conversion(Direction),
    VariableDef,
}

fn parse_rule_kind(it: &mut t!()) -> Result<RuleKind> {
    // dbg!(it.peek());
    match it.peek() {
        None => Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
        Some(&'=') => {
            it.next();
            Ok(RuleKind::VariableDef)
        }
        Some(&('<' | '>' | '→' | '←' | '↔')) => {
            let dir = parse_direction(it)?;
            Ok(RuleKind::Conversion(dir))
        }
        Some(&c) => Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
    }
}

#[derive(Debug, Clone)]
pub(super) struct ConversionRule {
    pub(super) source: HalfRule,
    pub(super) target: HalfRule,
    pub(super) dir: Direction,
}

#[derive(Debug, Clone)]
pub(super) enum Rule {
    ConversionRule(ConversionRule),
    VariableDef(VariableDef),
}

fn parse_rule(it: &mut t!()) -> Result<Rule> {
    // dbg!(it.peek());
    let half_rule1 = parse_half_rule(it)?;
    // stopped because a is_half_rule_end char appeared
    // dbg!(half_rule1.clone());
    skip_whitespace(it);
    let rule_kind = parse_rule_kind(it)?;
    // it should be one going in the middle
    skip_whitespace(it);
    let half_rule2 = parse_half_rule(it)?;
    // stopped because a is_half_rule_end char appeared, should only be ;
    match it.next() {
        None => return Err(ParseError::new(pl!(), PEK::UnexpectedEof)),
        Some(';') => {}
        Some(c) => return Err(ParseError::new(pl!(), PEK::UnexpectedChar(c))),
    }
    match rule_kind {
        RuleKind::VariableDef => {
            // perform some runtime checks for variable defs
            if half_rule1.ante.is_some()
                || half_rule1.post.is_some()
                || half_rule2.ante.is_some()
                || half_rule2.post.is_some()
            {
                return Err(ParseError::new(pl!(), PEK::Legacy));
            }
            if half_rule1.key.0.len() != 1 {
                return Err(ParseError::new(pl!(), PEK::Legacy));
            }

            let PatternElement::Variable(name) = half_rule1.key.0[0].clone() else {
                return Err(ParseError::new(pl!(), PEK::Legacy));
            };

            Ok(Rule::VariableDef(VariableDef {
                name,
                pattern: half_rule2.key,
            }))
        }
        RuleKind::Conversion(dir) => Ok(Rule::ConversionRule(ConversionRule {
            source: half_rule1,
            target: half_rule2,
            dir,
        })),
    }
}

pub(super) fn parse_rules(it: &mut t!()) -> Result<Vec<Rule>> {
    // dbg!(it.peek());
    let mut rules = Vec::new();
    loop {
        skip_whitespace(it);
        if it.peek().is_none() {
            break;
        }
        let rule = parse_rule(it)?;
        // dbg!(rule.clone());
        rules.push(rule);
    }
    Ok(rules)
}

pub(super) fn pretty_print_rules(rules: &[Rule]) {
    eprintln!("[");

    for rule in rules {
        match rule {
            Rule::ConversionRule(rule) => {
                let source = &rule.source;
                let target = &rule.target;
                let dir = &rule.dir;
                eprintln!("  {source} {dir} {target} ;");
            }
            Rule::VariableDef(rule) => {
                let name = &rule.name;
                let pattern = &rule.pattern.string();
                eprintln!("  ${name} = {pattern} ;");
            }
        }
    }
    eprintln!("]");
}
