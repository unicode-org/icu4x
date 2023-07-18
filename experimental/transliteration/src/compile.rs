use std::borrow::Cow;
use std::collections::HashMap;
use super::dfaparsing as prs;
use super::translit;

macro_rules! sl {
    () => {
        SourceLocation {
            file: file!(),
            line: line!(),
        }
    };
}

#[derive(Clone)]
pub struct SourceLocation {
    file: &'static str,
    line: u32,
}
impl core::fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\n[{}:{}]", self.file, self.line)
    }
}

use CompileErrorKind as CEK;
use SourceLocation as SL;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_unicodeset_parser::ParseError;

// TODO: replicate unicodeset_parser error behavior?
#[derive(Debug, Clone, Copy)]
pub enum CompileErrorKind {
    Default,
    UnicodeSetParseError(icu_unicodeset_parser::ParseError),
}

#[derive(Debug, Clone)]
pub struct CompileError {
    location: SourceLocation,
    kind: CompileErrorKind,
}

impl CompileError {
    pub fn new(location: SourceLocation, kind: CompileErrorKind) -> Self {
        Self { location, kind }
    }

    pub fn default(location: SourceLocation) -> Self {
        Self::new(location, CEK::Default)
    }

    pub fn unicodeset(location: SourceLocation, inner: icu_unicodeset_parser::ParseError) -> Self {
        Self::new(location, CEK::UnicodeSetParseError(inner))
    }
}
pub type Result<T, E = CompileError> = core::result::Result<T, E>;

pub(super) struct Compiler<'a> {
    variables: HashMap<String, translit::Pattern<'a>>,
}

impl<'a> Compiler<'a> {
    pub(super) fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn compile_unicode_set(&self, s: &str) -> Result<CodePointInversionListAndStringList<'a>> {
        icu_unicodeset_parser::parse(s).map_err(|e| CompileError::unicodeset(sl!(), e))
    }

    fn compile_target_pattern(&self, parsed_pattern: &[prs::PatternElement]) -> Result<translit::Replacer<'a>> {
        let mut replacement = String::new();
        let mut cursor_from_beginning = None;
        for (idx, element) in parsed_pattern.into_iter().enumerate() {
            match element {
                prs::PatternElement::Literal(s) => {
                    replacement.push_str(s);
                }
                prs::PatternElement::Variable(s) => {
                    let variable_pattern = &self.variables
                        .get(s)
                        .ok_or_else(|| CompileError::default(sl!()))?
                        .0;
                    // this variable is being used as the target, it must only contain literals
                    // due to the concatenation rule, that means it must only contain a single literal
                    let [translit::PatternElement::Literal(ref s)] = variable_pattern[..] else {
                        return Err(CompileError::default(sl!()));
                    };
                    replacement.push_str(s);
                }
                prs::PatternElement::Cursor(cursor) => {
                    if cursor_from_beginning.is_some() {
                        // can only have one cursor
                        return Err(CompileError::default(sl!()));
                    }
                    if cursor.pre_spacing != 0 && cursor.post_spacing != 0 {
                        // a cursor with both pre and post spacing ( ..@|@.. ) is not allowed
                        return Err(CompileError::default(sl!()));
                    }
                    if cursor.post_spacing != 0 && !replacement.is_empty() {
                        // a cursor with post spacing ( |@.. ) must appear at the beginning of the target
                        return Err(CompileError::default(sl!()));
                    }
                    if cursor.pre_spacing != 0 && idx != parsed_pattern.len() - 1 {
                        // a cursor with pre spacing ( ..@| ) must appear at the end of the target
                        return Err(CompileError::default(sl!()));
                    }
                    let curr_size = replacement.chars().count() as i32;
                    cursor_from_beginning = Some(curr_size + cursor.pre_spacing as i32 - cursor.post_spacing as i32);
                }
                prs::PatternElement::UnicodeSet(_) => {
                    return Err(CompileError::default(sl!()));
                }
            }
        }

        let cursor_from_end = cursor_from_beginning.map(|cfb| cfb - replacement.chars().count() as i32).unwrap_or(0);
        Ok(translit::Replacer {
            replacement: replacement.into(),
            cursor: cursor_from_end,
        })
    }

    fn compile_source_pattern(&self, parsed_pattern: &[prs::PatternElement]) -> Result<translit::Pattern<'a>> {
        let mut pattern = Vec::new();
        let mut literal = String::new();
        for element in parsed_pattern {
            match element {
                prs::PatternElement::Literal(s) => {
                    literal.push_str(s);
                }
                prs::PatternElement::UnicodeSet(s) => {
                    if !literal.is_empty() {
                        pattern.push(translit::PatternElement::Literal(literal.into()));
                        literal = String::new();
                    }
                    let set = self.compile_unicode_set(s)?;
                    pattern.push(translit::PatternElement::UnicodeSet(set));
                }
                prs::PatternElement::Variable(s) => {
                    let variable_pattern = &self.variables
                        .get(s)
                        .ok_or_else(|| CompileError::default(sl!()))?
                        .0;

                    // TODO: perhaps replace this complex match with the following:
                    //  * push the literal
                    //  * push whole variable_pattern
                    //  * check (old_end, old_end + 1) for (Literal, Literal), if so, concat
                    //  * check new_end for Literal, if so, unpush

                    // need some checks to uphold literal-concatenation invariant
                    // only need to check first and last element.
                    match (variable_pattern.len(), variable_pattern.first(), variable_pattern.last()) {
                        (0, _, _) => continue,
                        (1, Some(translit::PatternElement::Literal(s)), _) => {
                            literal.push_str(s);
                            continue;
                        },
                        (1, Some(u@translit::PatternElement::UnicodeSet(_)), _) => {
                            if !literal.is_empty() {
                                pattern.push(translit::PatternElement::Literal(literal.into()));
                                literal = String::new();
                            }
                            pattern.push(u.clone());
                            continue;
                        },
                        (2.., _, _) => {
                            // handle first element
                            if let Some(translit::PatternElement::Literal(s)) = variable_pattern.first() {
                                literal.push_str(s);
                                pattern.push(translit::PatternElement::Literal(literal.into()));
                                literal = String::new();
                            } else {
                                if !literal.is_empty() {
                                    pattern.push(translit::PatternElement::Literal(literal.into()));
                                    literal = String::new();
                                }
                                pattern.push(variable_pattern[0].clone());
                            }
                            // handle middle elements
                            pattern.extend(variable_pattern[1..(variable_pattern.len()-1)].iter().cloned());
                            // handle last element
                            if let Some(translit::PatternElement::Literal(s)) = variable_pattern.last() {
                                literal.push_str(s);
                                continue;
                            } else {
                                pattern.push(variable_pattern[variable_pattern.len()-1].clone());
                            }
                        },
                        // unreachable
                        _ => return Err(CompileError::default(sl!())),
                    }
                }
                // skipped on the source
                prs::PatternElement::Cursor(_) => {}
            }
        }
        if !literal.is_empty() {
            pattern.push(translit::PatternElement::Literal(literal.into()));
        }
        Ok(translit::Pattern(pattern))
    }

    fn compile_converison_rule_directed(&self, source: &prs::HalfRule, target: &prs::HalfRule) -> Result<translit::Rule<'a>> {
        let ante = source.ante.as_ref().map(|ante| self.compile_source_pattern(&ante.0)).transpose()?;
        let key = self.compile_source_pattern(&source.key.0)?;
        let post = source.post.as_ref().map(|post| self.compile_source_pattern(&post.0)).transpose()?;

        let target = self.compile_target_pattern(&target.key.0)?;

        Ok(translit::Rule {
            ante,
            key,
            post,
            target,
        })
    }

    // returns (fwd, bwd)
    fn compile_conversion_rule(&self, parsed_rule: &prs::ConversionRule) -> Result<(Option<translit::Rule<'a>>, Option<translit::Rule<'a>>)> {
        // we're doing some duplicate work here (compiling source and target key twice), but code is simpler

        let mut fwd_rule = None;
        let mut bwd_rule = None;

        let parsed_source = &parsed_rule.source;
        let parsed_target = &parsed_rule.target;
        let parsed_dir = parsed_rule.dir;

        let mut do_forward = false;
        let mut do_backward = false;
        match parsed_dir {
            prs::Direction::Forward => do_forward = true,
            prs::Direction::Reverse => do_backward = true,
            prs::Direction::Bidirectional => {
                do_forward = true;
                do_backward = true;
            }
        }

        if do_forward {
            fwd_rule = Some(self.compile_converison_rule_directed(parsed_source, parsed_target)?);
        }
        if do_backward {
            bwd_rule = Some(self.compile_converison_rule_directed(parsed_target, parsed_source)?);
        }

        Ok((fwd_rule, bwd_rule))
    }

    fn compile_vardef(&mut self, parsed_vardef: &prs::VariableDef) -> Result<()> {
        let pattern = self.compile_source_pattern(&parsed_vardef.pattern.0)?;
        self.variables.insert(parsed_vardef.name.clone(), pattern);
        Ok(())
    }

    // returns (forward_transliterator, backwards_transliterator)
    pub(super) fn compile(&mut self, parsed_rules: &[prs::Rule]) -> Result<(translit::Transliterator<'a>, translit::Transliterator<'a>)> {
        let mut fwd_rules = Vec::new();
        let mut bwd_rules = Vec::new();

        for parsed_rule in parsed_rules {
            match parsed_rule {
                prs::Rule::VariableDef(vardef) => {
                    self.compile_vardef(vardef)?;
                }
                prs::Rule::ConversionRule(conversion_rule) => {
                    let (fwd_rule, bwd_rule) = self.compile_conversion_rule(conversion_rule)?;
                    if let Some(fwd_rule) = fwd_rule {
                        fwd_rules.push(fwd_rule);
                    }
                    if let Some(bwd_rule) = bwd_rule {
                        bwd_rules.push(bwd_rule);
                    }
                }
            }
        }



        Ok((translit::Transliterator {
            rules: fwd_rules,
        }, translit::Transliterator {
            rules: bwd_rules,
        }))
    }
}