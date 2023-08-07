// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: update this module doc (and below //-comment) to reflect the new bidirectional nature of Pass1

//! # Compilation for transliterators
//!
//! This module has three main functions. First, it validates many aspects of transliterators.
//! Second, it compiles them into the zero-copy data struct defined in `icu_transliteration`. Third,
//! it computes the dependencies of the transliterator.
//! is responsible for both directions of a source file, but the rest of this documentation
//! assumes a single direction. The process is simply repeated for the other direction.
//!
//! Conversion rules are encoded using `str`s, and private use code points are used to represent
//! the special constructs that can appear in a conversion rule (UnicodeSets, quantifiers, ...).
//! This works as follows:
//! * We use PUP (15), code points U+F0000 to U+FFFFD (inclusive)
//! * A private use code point simply encodes an integer, obtained by subtracting U+F0000 from it
//! * The integer is used as an index into `VarTable`
//! * As a `VarTable` has multiple `VarZeroVec`s (one for each special construct), an index
//!   overflows into the following `VZV`s:
//!    * An index of `vzv1.len() + vzv2.len() + 4` indexes the third `VZV` at index 4
//! * Thus, if the length of an earlier `VZV` changes, the index of an element in a later `VZV`
//!   will change, and its private use encoding will change
//! * Therefore we must know the sizes of each `VZV` before we can start encoding conversion rules
//!   into `str`s.
//!
//! This module works by performing multiple passes over the rules.
//!
//! ## Pass 1
//! General validation of the rules and computation of the sizes of the `VZV`s in the `VarTable`.
//!
//! One optimization that this pass performs, is that only special constructs for the current
//! direction are accounted for, i.e., the rule `a > [a-z] { b` will not increment the size of the
//! `VZV` for UnicodeSets if the current direction is `forward`, but it will if the current
//! direction is `reverse` (this is because contexts on the target side of a rule are ignored).
//!
//! Similarly, only recursive transliterators and variables actually used for this direction are
//! accounted for.
//!
//! ## Pass 2
//! Encoding of the zero-copy data struct.
//!
//! To encode conversion rules into `str`s, we use the previously described encoded `VarTable`
//! indices. Because we know the sizes of each special construct list (in the form a `VZV`)
//! from the first pass, we can store the offsets for each special construct list (i.e., the sum of
//! the lengths of the previous lists) while encoding the conversion rules, and incrementing the
//! offset of a given special construct when we encode an element. The precomputed sizes mean we
//! never overflow into the indices of the following `VZV`.

// more (data struct compatible) runtime optimization opportunities:
// - deduplicate special constructs ($a = hello ; $b = hello should only generate one hello element)
// - inline single-use variables
// - flatten single-element sets into literals

// compilation speed optimization opportunities:
// - compile both directions simultaneously

// missing (non-correctness-relevant) validations:
// - cursors in unidirectional transliterators should not appear on the source side
// - cursors should not exist in the (implicitly ignored) context of the target side
// - anchors in unidirectional transliterators should not appear on the target side

/*

variable example:
$b = bc+ ;
$a = [a-z] $b ;
$a > ;

b-data.sizes: 1 compound (the definition itself), 1 quantifier plus (c+)
b-data.used_vars: -

a-data.sizes: 1 compound (the definition itself), 1 unicodeset ([a-z])
a-data.used_vars: b

forward-data.sizes: 0 (rules are inlined)
forward-data.used_vars: a

when collecting the sizes (for forward) at the end, we sum over all sizes of the transitive
dependencies of forward (using used_vars), and add the sizes of forward itself.
we also compute the transitive closure of used variables.
this gives us:
final forward-data.sizes: 2 compound, 1 quantifier plus, 1 unicodeset
final forward-data.used_vars: a, b

this final data we give to Pass2, which will produce something like this:
(note that the integer-indexed maps shown here are only semantic, in actuality the indices are implicit,
as described in the zero-copy format, and the maps here are just arrays)

    VarTable {
        compounds: {
            0: "b<2>", // b's definition, bc+
            1: "<3><0>", // a's definition, [a-z] $b
        },
        quantifier_kleene_plus: {
            2: "c", // c+
        },
        unicode_sets: {
            3: <the set of a..z>, // [a-z]
        }
    }
    Rules: [
        {
            source: "<1>", // $a
            target: "",
        }
    ]


*/

use crate::parse;
use crate::parse::{ElementLocation as EL, HalfRule, QuantifierKind};
use parse::Result;
use parse::PEK;
use std::collections::{HashMap, HashSet};

enum SingleDirection {
    Forward,
    Reverse,
}

#[derive(Debug, Clone, Default)]
struct SpecialConstructSizes {
    num_compounds: usize,
    num_quantifiers_opt: usize,
    num_quantifiers_kleene: usize,
    num_quantifiers_kleene_plus: usize,
    num_segments: usize,
    num_unicode_sets: usize,
    num_function_calls: usize,
}

// Data for a given direction or variable definition (the "key")
#[derive(Debug, Clone, Default)]
struct Pass1Data {
    sizes: SpecialConstructSizes,
    // the variables (directly) used by the associated key
    used_variables: HashSet<String>,
    // the recursive transliterators that are (directly) used by the associated key
    dependencies: HashSet<parse::BasicId>,
}

// variable dependencies work as follows:
// - variables used in a conversion rule of a given direction get added to that direction's
//   Pass1Data.used_variables.
// - variables that are used inside a variable definition for variable `x` get added to
//   Pass1.variable_dependencies[x].
// - the actual variables needed by a given direction is union of all variables in the associated
//   Pass1Data.used_variables and their transitive dependencies according to Pass1.variable_dependencies.
#[derive(Debug, Clone)]
struct Pass1<'p> {
    direction: parse::Direction,
    forward_data: Pass1Data,
    reverse_data: Pass1Data,
    variable_data: HashMap<String, Pass1Data>,
    // // key => value: key depends on all variables in value
    // variable_dependencies: HashMap<String, HashSet<String>>,
    // variable definitions (for both forward and reverse)
    variable_definitions: HashMap<String, &'p [parse::Element]>,
    // variables which contain constructs that are only allowed to appear on the source side
    // e.g., $a = c+; $set = [a-z]; ...
    target_disallowed_variables: HashSet<String>,
}

impl<'p> Pass1<'p> {
    fn new(direction: parse::Direction) -> Self {
        Self {
            direction,
            forward_data: Pass1Data::default(),
            reverse_data: Pass1Data::default(),
            variable_data: HashMap::new(),
            variable_definitions: HashMap::new(),
            target_disallowed_variables: HashSet::new(),
        }
    }

    fn run(&mut self, rules: &'p [parse::Rule]) -> Result<()> {
        // first check global filter/global inverse filter.
        // after this check, they may not appear anywhere.
        let rules = self.validate_global_filters(rules)?;

        // iterate through remaining rules and perform checks according to interim specification

        for rule in rules {
            match rule {
                parse::Rule::GlobalFilter(_) | parse::Rule::GlobalInverseFilter(_) => {
                    // the previous step ensures `rules` has no more global filters
                    return Err(PEK::UnexpectedGlobalFilter.into());
                }
                parse::Rule::Transform(forward_id, reverse_id) => {
                    self.validate_transform(forward_id, reverse_id.as_ref())?;
                }
                parse::Rule::VariableDefinition(name, definition) => {
                    self.validate_variable_definition(name, definition)?;
                }
                parse::Rule::Conversion(hr1, dir, hr2) => {
                    self.validate_conversion(hr1, *dir, hr2)?;
                }
            }
        }

        Ok(())
    }

    fn validate_global_filters<'a>(&self, rules: &'a [parse::Rule]) -> Result<&'a [parse::Rule]> {
        let rules = match rules {
            [parse::Rule::GlobalFilter(_), rest @ ..] => rest,
            _ => rules,
        };
        let rules = match rules {
            [rest @ .., parse::Rule::GlobalInverseFilter(_)] => rest,
            _ => rules,
        };

        Ok(rules)
    }

    fn validate_transform(
        &mut self,
        forward_id: &parse::SingleId,
        reverse_id: Option<&parse::SingleId>,
    ) -> Result<()> {
        let fwd_dep = forward_id.basic_id.clone();
        self.forward_data.dependencies.insert(fwd_dep);
        let rev_dep = reverse_id
            .map(|single_id| single_id.basic_id.clone())
            .unwrap_or_else(|| forward_id.basic_id.clone().reverse());
        self.reverse_data.dependencies.insert(rev_dep);
        Ok(())
    }

    fn validate_variable_definition(
        &mut self,
        name: &String,
        definition: &'p [parse::Element],
    ) -> Result<()> {
        if self.variable_definitions.contains_key(name) {
            return Err(PEK::DuplicateVariable.into());
        }
        self.variable_definitions.insert(name.clone(), definition);

        let mut data = Pass1Data::default();
        // the variable definition itself is counted here
        data.sizes.num_compounds = 1;

        let mut validator = VariableDefinitionValidator::new(
            |s| self.variable_definitions.contains_key(s),
            &mut data,
            &mut self.target_disallowed_variables,
            definition,
        );
        validator.validate()?;
        if validator.used_source_only_constructs {
            self.target_disallowed_variables.insert(name.clone());
        }

        self.variable_data.insert(name.clone(), data);

        Ok(())
    }

    fn validate_conversion(
        &mut self,
        source: &HalfRule,
        dir: parse::Direction,
        target: &HalfRule,
    ) -> Result<()> {
        // TODO(#3736): include source location/actual source text in these logs
        if !self.direction.permits(dir) {
            // example: metadata defines this transliterator as forward, but a `<>` or `<` rule is found.
            log::warn!(
                "metadata for transliterator specifies direction {:?} but conversion rule specifies {:?}",
                self.direction,
                dir,
            );
        }
        // logging for useless contexts
        if dir == parse::Direction::Forward && (!target.ante.is_empty() || !target.post.is_empty())
        {
            log::warn!("forward conversion rule has ignored context on target side");
        }
        if dir == parse::Direction::Reverse && (!source.ante.is_empty() || !source.post.is_empty())
        {
            log::warn!("reverse conversion rule has ignored context on target side");
        }

        if self.direction.permits(parse::Direction::Forward)
            && dir.permits(parse::Direction::Forward)
        {
            self.validate_conversion_one_direction(source, target, SingleDirection::Forward)?;
        }
        if self.direction.permits(parse::Direction::Reverse)
            && dir.permits(parse::Direction::Reverse)
        {
            self.validate_conversion_one_direction(target, source, SingleDirection::Reverse)?;
        }

        Ok(())
    }

    fn validate_conversion_one_direction(
        &mut self,
        source: &HalfRule,
        target: &HalfRule,
        dir: SingleDirection,
    ) -> Result<()> {
        let data = match dir {
            SingleDirection::Forward => &mut self.forward_data,
            SingleDirection::Reverse => &mut self.reverse_data,
        };
        let mut source_validator = SourceValidator::new(
            |s| self.variable_definitions.contains_key(s),
            data,
            &source.ante,
            &source.key,
            &source.post,
        );
        source_validator.validate()?;
        let num_source_segments = source_validator.num_segments;

        let mut target_validator = TargetValidator::new(
            |s| self.variable_definitions.contains_key(s),
            &mut self.target_disallowed_variables,
            data,
            &target.key,
            num_source_segments,
        );
        target_validator.validate()?;

        Ok(())
    }
}

struct TargetValidator<'a, 'p, F: Fn(&str) -> bool> {
    is_variable_defined: F,
    target_disallowed_variables: &'a mut HashSet<String>,
    data: &'a mut Pass1Data,
    replacer: &'p [parse::Element],
    // the number of segments defined on the corresponding source side. produced by SourceValidator
    num_segments: u32,
    // true if a cursor has already been encountered, i.e., any further cursors are disallowed
    encountered_cursor: bool,
}

impl<'a, 'p, F: Fn(&str) -> bool> TargetValidator<'a, 'p, F> {
    fn new(
        is_variable_defined: F,
        target_disallowed_variables: &'a mut HashSet<String>,
        data: &'a mut Pass1Data,
        replacer: &'p [parse::Element],
        num_segments: u32,
    ) -> Self {
        Self {
            is_variable_defined,
            target_disallowed_variables,
            data,
            replacer,
            num_segments,
            encountered_cursor: false,
        }
    }

    fn validate(&mut self) -> Result<()> {
        let section = self.replacer;
        // strip pre or post cursor
        let section = match section {
            [parse::Element::Cursor(pre, _), rest @ ..] => {
                if self.encountered_cursor {
                    return Err(PEK::DuplicateCursor.into());
                }
                self.encountered_cursor = true;
                if *pre != 0 {
                    // corrseponds to `@@@|...`, i.e., placeholders in front of the cursor
                    return Err(PEK::InvalidCursor.into());
                }
                rest
            }
            _ => section,
        };
        let section = match section {
            [rest @ .., parse::Element::Cursor(_, post)] => {
                if self.encountered_cursor {
                    return Err(PEK::DuplicateCursor.into());
                }
                self.encountered_cursor = true;
                if *post != 0 {
                    // corrseponds to `...|@@@`, i.e., placeholders after the cursor
                    return Err(PEK::InvalidCursor.into());
                }
                rest
            }
            _ => section,
        };

        self.validate_section(section, true)
    }

    fn validate_section(&mut self, section: &[parse::Element], top_level: bool) -> Result<()> {
        section
            .iter()
            .try_for_each(|element| self.validate_element(element, top_level))
    }

    fn validate_element(&mut self, element: &parse::Element, top_level: bool) -> Result<()> {
        match element {
            parse::Element::Literal(_) => {}
            parse::Element::VariableRef(name) => {
                if !(self.is_variable_defined)(name) {
                    return Err(PEK::UnknownVariable.into());
                }
                if self.target_disallowed_variables.contains(name) {
                    return Err(PEK::SourceOnlyVariable.into());
                }
                self.data.used_variables.insert(name.clone());
            }
            parse::Element::BackRef(num) => {
                if *num > self.num_segments {
                    return Err(PEK::BackReferenceOutOfRange.into());
                }
            }
            parse::Element::FunctionCall(id, inner) => {
                self.validate_section(inner, false)?;
                self.data.dependencies.insert(id.basic_id.clone());
                self.data.sizes.num_function_calls += 1;
            }
            parse::Element::Cursor(pre, post) => {
                if self.encountered_cursor {
                    return Err(PEK::DuplicateCursor.into());
                }
                if !top_level || *pre != 0 || *post != 0 {
                    // pre and post must be 0 if the cursor does not appear at the very beginning or the very end
                    // we account for the beginning or the end in `validate`.
                    return Err(PEK::InvalidCursor.into());
                }
                self.encountered_cursor = true;
            }
            parse::Element::AnchorStart => {
                // while anchors have no effect on the target side, they may still appear
            }
            parse::Element::AnchorEnd => {
                // while anchors have no effect on the target side, they may still appear
            }
            elt => {
                return Err(PEK::UnexpectedElement(elt.kind(), EL::Target).into());
            }
        }
        Ok(())
    }
}

struct SourceValidator<'a, 'p, F: Fn(&str) -> bool> {
    is_variable_defined: F,
    data: &'a mut Pass1Data,
    ante: &'p [parse::Element],
    key: &'p [parse::Element],
    post: &'p [parse::Element],
    // the number of segments this rule defines. consumed by TargetValidator.
    num_segments: u32,
}
struct VariableDefinitionValidator<'a, 'p, F: Fn(&str) -> bool> {
    is_variable_defined: F,
    target_disallowed_variables: &'a mut HashSet<String>,
    data: &'a mut Pass1Data,
    definition: &'p [parse::Element],
    used_source_only_constructs: bool,
}

impl<'a, 'p, F: Fn(&str) -> bool> SourceValidator<'a, 'p, F> {
    fn new(
        is_variable_defined: F,
        data: &'a mut Pass1Data,
        ante: &'p [parse::Element],
        key: &'p [parse::Element],
        post: &'p [parse::Element],
    ) -> Self {
        Self {
            is_variable_defined,
            data,
            ante,
            key,
            post,
            num_segments: 0,
        }
    }

    fn validate(&mut self) -> Result<()> {
        // first validate position of ^ and $ anchors, if they exist
        // ^: if ante is non-empty, must be its first element, otherwise must be first element of key
        // $: if post is non-empty, must be its last element, otherwise must be last element of key

        let sections = [self.ante, self.key, self.post];
        // split off first element if it is a start anchor
        let sections = match sections {
            [[parse::Element::AnchorStart, ante @ ..], key, post] => [ante, key, post],
            [[], [parse::Element::AnchorStart, key @ ..], post] => [&[], key, post],
            _ => sections,
        };
        // split off last element if it is an end anchor
        let sections = match sections {
            [ante, key, [post @ .., parse::Element::AnchorEnd]] => [ante, key, post],
            [ante, [key @ .., parse::Element::AnchorEnd], []] => [ante, key, &[]],
            _ => sections,
        };

        // now neither start nor end anchors may appear anywhere in `order`

        sections.iter().try_for_each(|s| self.validate_section(s))
    }

    fn validate_section(&mut self, section: &[parse::Element]) -> Result<()> {
        section
            .iter()
            .try_for_each(|element| self.validate_element(element))
    }

    fn validate_element(&mut self, element: &parse::Element) -> Result<()> {
        match element {
            parse::Element::Literal(_) => {}
            parse::Element::VariableRef(name) => {
                if !(self.is_variable_defined)(name) {
                    return Err(PEK::UnknownVariable.into());
                }
                self.data.used_variables.insert(name.clone());
            }
            parse::Element::Quantifier(kind, inner) => {
                self.validate_element(inner)?;
                match *kind {
                    QuantifierKind::ZeroOrOne => self.data.sizes.num_quantifiers_opt += 1,
                    QuantifierKind::ZeroOrMore => self.data.sizes.num_quantifiers_kleene += 1,
                    QuantifierKind::OneOrMore => self.data.sizes.num_quantifiers_kleene_plus += 1,
                }
            }
            parse::Element::Segment(inner) => {
                self.validate_section(inner)?;
                // increment the count for this specific rule
                self.num_segments += 1;
                // increment the count for this direction of the entire transliterator
                self.data.sizes.num_segments += 1;
            }
            parse::Element::UnicodeSet(_) => {
                self.data.sizes.num_unicode_sets += 1;
            }
            parse::Element::Cursor(_, _) => {
                // while cursors have no effect on the source side, they may appear nonetheless
                // TargetValidator validates these
            }
            parse::Element::AnchorStart => {
                // we check for these in `validate`
                return Err(PEK::AnchorStartNotAtStart.into());
            }
            parse::Element::AnchorEnd => {
                // we check for these in `validate`
                return Err(PEK::AnchorEndNotAtEnd.into());
            }
            elt => {
                return Err(PEK::UnexpectedElement(elt.kind(), EL::Source).into());
            }
        }
        Ok(())
    }
}

impl<'a, 'p, F: Fn(&str) -> bool> VariableDefinitionValidator<'a, 'p, F> {
    fn new(
        is_variable_defined: F,
        data: &'a mut Pass1Data,
        target_disallowed_variables: &'a mut HashSet<String>,
        definition: &'p [parse::Element],
    ) -> Self {
        Self {
            is_variable_defined,
            data,
            target_disallowed_variables,
            definition,
            used_source_only_constructs: true,
        }
    }

    fn validate(&mut self) -> Result<()> {
        self.validate_section(self.definition)
    }

    fn validate_section(&mut self, section: &[parse::Element]) -> Result<()> {
        section
            .iter()
            .try_for_each(|element| self.validate_element(element))
    }

    fn validate_element(&mut self, element: &parse::Element) -> Result<()> {
        match element {
            parse::Element::Literal(_) => {}
            parse::Element::VariableRef(name) => {
                if !(self.is_variable_defined)(name) {
                    return Err(PEK::UnknownVariable.into());
                }
                if self.target_disallowed_variables.contains(name) {
                    self.used_source_only_constructs = false;
                }
                self.data.used_variables.insert(name.clone());
            }
            parse::Element::Quantifier(_, inner) => {
                self.used_source_only_constructs = false;
                self.validate_element(inner)?;
            }
            parse::Element::UnicodeSet(_) => {
                self.used_source_only_constructs = false;
                self.data.sizes.num_unicode_sets += 1;
            }
            elt => {
                return Err(PEK::UnexpectedElement(elt.kind(), EL::VariableDefinition).into());
            }
        }
        Ok(())
    }
}

pub(crate) fn compile(
    rules: Vec<parse::Rule>,
    direction: parse::Direction,
) -> Result<icu_transliteration::provider::RuleBasedTransliterator<'static>> {
    let mut pass1 = Pass1::new(direction);
    pass1.run(&rules)?;

    todo!()
}
