// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use icu_collections::codepointinvlist::CodePointInversionList;
use zerovec::VarZeroVec;
use super::*;
use crate::parse::UnicodeSet;

use icu_transliteration::provider as ds;

macro_rules! impl_insert {
    ($fn_name:ident, $vec_field:ident, $elt_type:ty, $current_field:ident, $next_field:ident) => {
        fn $fn_name(&mut self, elt: $elt_type) -> char {
            // pass 1 is responsible for this
            debug_assert!(self.$current_field < self.$next_field - 1);
            #[allow(clippy::unwrap_used)] // the whole PUP (15) consists of valid chars
            let standin = char::try_from(self.$current_field).unwrap();
            self.$vec_field.push(elt);
            self.$current_field += 1;
            standin
        }
    };
}

struct MutVarTableField<T> {
    vec: Vec<T>,
    base: u32,
    current: u32
}


// TODO: refactor these into struct MutVarTableField<T>{vec: Vec<T>, base: u32, current: u32};
struct MutVarTable {
    compounds: Vec<String>,
    compounds_base: u32,
    compounds_current: u32,
    quantifiers_opt: Vec<String>,
    quantifiers_opt_base: u32,
    quantifiers_opt_current: u32,
    quantifiers_kleene: Vec<String>,
    quantifiers_kleene_base: u32,
    quantifiers_kleene_current: u32,
    quantifiers_kleene_plus: Vec<String>,
    quantifiers_kleene_plus_base: u32,
    quantifiers_kleene_plus_current: u32,
    segments: Vec<String>,
    segments_base: u32,
    segments_current: u32,
    unicode_sets: Vec<UnicodeSet>,
    unicode_sets_base: u32,
    unicode_sets_current: u32,
    function_calls: Vec<ds::FunctionCall<'static>>,
    function_calls_base: u32,
    function_calls_current: u32,
    backref_base: u32,
    counts: SpecialConstructCounts,
}

impl MutVarTable {
    const BASE: u32 = '\u{F0000}' as u32;
    const MAX_DYNAMIC: u32 = '\u{FFFF0}' as u32;
    const RESERVED_ANCHOR_START: char = '\u{FFFFC}';
    const RESERVED_ANCHOR_END: char = '\u{FFFFD}';

    fn try_new_from_counts(counts: SpecialConstructCounts) -> Result<Self> {
        if counts.num_total() > (Self::MAX_DYNAMIC as usize - Self::BASE as usize + 1) {
            return Err(PEK::TooManySpecials.into());
        }

        let compounds_base = MutVarTable::BASE;
        let quantifiers_opt_base = compounds_base + counts.num_compounds as u32;
        let quantifiers_kleene_base = quantifiers_opt_base + counts.num_quantifiers_opt as u32;
        let quantifiers_kleene_plus_base =
            quantifiers_kleene_base + counts.num_quantifiers_kleene as u32;
        let segments_base =
            quantifiers_kleene_plus_base + counts.num_quantifiers_kleene_plus as u32;
        let unicode_sets_base = segments_base + counts.num_segments as u32;
        let function_calls_base = unicode_sets_base + counts.num_unicode_sets as u32;

        Ok(Self {
            compounds: Vec::with_capacity(counts.num_compounds),
            compounds_base,
            quantifiers_opt: Vec::with_capacity(counts.num_quantifiers_opt),
            quantifiers_opt_base,
            quantifiers_kleene: Vec::with_capacity(counts.num_quantifiers_kleene),
            quantifiers_kleene_base,
            quantifiers_kleene_plus: Vec::with_capacity(counts.num_quantifiers_kleene_plus),
            quantifiers_kleene_plus_base,
            segments: Vec::with_capacity(counts.num_segments),
            segments_base,
            unicode_sets: Vec::with_capacity(counts.num_unicode_sets),
            unicode_sets_base,
            function_calls: Vec::with_capacity(counts.num_function_calls),
            function_calls_base,
            counts,
            compounds_current: compounds_base,
            quantifiers_opt_current: quantifiers_opt_base,
            quantifiers_kleene_current: quantifiers_kleene_base,
            quantifiers_kleene_plus_current: quantifiers_kleene_plus_base,
            segments_current: segments_base,
            unicode_sets_current: unicode_sets_base,
            function_calls_current: function_calls_base,
            backref_base: function_calls_base + counts.num_function_calls as u32,
        })
    }

    impl_insert!(
        insert_compound,
        compounds,
        String,
        compounds_current,
        quantifiers_opt_base
    );
    impl_insert!(
        insert_quantifier_opt,
        quantifiers_opt,
        String,
        quantifiers_opt_current,
        quantifiers_kleene_base
    );
    impl_insert!(
        insert_quantifier_kleene,
        quantifiers_kleene,
        String,
        quantifiers_kleene_current,
        quantifiers_kleene_plus_base
    );
    impl_insert!(
        insert_quantifier_kleene_plus,
        quantifiers_kleene_plus,
        String,
        quantifiers_kleene_plus_current,
        segments_base
    );
    impl_insert!(
        insert_segment,
        segments,
        String,
        segments_current,
        unicode_sets_base
    );
    impl_insert!(
        insert_unicode_set,
        unicode_sets,
        UnicodeSet,
        unicode_sets_current,
        function_calls_base
    );
    impl_insert!(
        insert_function_call,
        function_calls,
        ds::FunctionCall<'static>,
        function_calls_current,
        backref_base
    );

    fn standin_for_backref(&self, backref_num: u32) -> char {
        debug_assert!(backref_num > 0);
        // TODO: move this step into fallible constructor, collect max_backref_num in pass1
        debug_assert!(self.backref_base + backref_num - 1 <= Self::MAX_DYNAMIC);
        #[allow(clippy::unwrap_used)] // debug asserts imply this is in range
        char::try_from(self.backref_base + backref_num - 1).unwrap()
    }

    fn finalize(&self) -> ds::VarTable<'static> {
        // we computed the exact counts, so when we call finalize, we should be full
        debug_assert!(self.is_full());

        ds::VarTable {
            compounds: VarZeroVec::from(&self.compounds),
            quantifiers_opt: VarZeroVec::from(&self.quantifiers_opt),
            quantifiers_kleene: VarZeroVec::from(&self.quantifiers_kleene),
            quantifiers_kleene_plus: VarZeroVec::from(&self.quantifiers_kleene_plus),
            segments: VarZeroVec::from(&self.segments),
            unicode_sets: VarZeroVec::from(&self.unicode_sets),
            function_calls: VarZeroVec::from(&self.function_calls),
        }
    }

    fn is_full(&self) -> bool {
        self.compounds.len() == self.counts.num_compounds
            && self.quantifiers_opt.len() == self.counts.num_quantifiers_opt
            && self.quantifiers_kleene.len() == self.counts.num_quantifiers_kleene
            && self.quantifiers_kleene_plus.len() == self.counts.num_quantifiers_kleene_plus
            && self.segments.len() == self.counts.num_segments
            && self.unicode_sets.len() == self.counts.num_unicode_sets
            && self.function_calls.len() == self.counts.num_function_calls
    }
}

struct MutRule {
    ante: String,
    key: String,
    post: String,
    replacer: String,
    cursor_offset: i32,
}

enum LiteralOrStandin<'a> {
    Literal(&'a str),
    Standin(char),
}

impl ToString for LiteralOrStandin<'_> {
    fn to_string(&self) -> String {
        match *self {
            LiteralOrStandin::Literal(s) => s.to_owned(),
            LiteralOrStandin::Standin(c) => c.to_string(),
        }
    }
}

struct Pass2<'a, 'p> {
    var_table: MutVarTable,
    var_definitions: &'a HashMap<String, &'p [parse::Element]>,
    // the inverse of VarTable.compounds
    var_to_char: HashMap<String, char>,

    id_group_list: Vec<VarZeroVec<'static, ds::SimpleIdULE>>,
    conversion_group_list: Vec<VarZeroVec<'static, ds::RuleULE>>,
}

impl<'a, 'p> Pass2<'a, 'p> {
    fn try_new(data: &'a Pass1Data, var_definitions: &'a HashMap<String, &'p [parse::Element]>) -> Result<Self> {
        Ok(Pass2 {
            var_table: MutVarTable::try_new_from_counts(data.counts)?,
            var_definitions,
            var_to_char: HashMap::new(),
            id_group_list: Vec::new(),
            conversion_group_list: Vec::new(),
        })
    }

    fn run(
        &mut self,
        rule_groups: super::RuleGroups<'p>,
        global_filter: Option<UnicodeSet>,
    ) -> Result<ds::RuleBasedTransliterator<'static>> {
        for (transform_group, conversion_group) in rule_groups {
            let mut compiled_transform_group = Vec::new();
            for id in transform_group {
                compiled_transform_group.push(self.compile_single_id(&id));
            }
            self.id_group_list.push(VarZeroVec::from(&compiled_transform_group));

            let mut compiled_conversion_group = Vec::new();
            for rule in conversion_group {
                // TODO: depending on source or target, remove the ignored special constructs (anchor, cursor)
                let ante = self.compile_section(&rule.ante);
                let key = self.compile_section(&rule.key);
                let post = self.compile_section(&rule.post);
                let replacer = self.compile_section(&rule.replacement);
                let cursor_offset = rule.cursor_offset;
                compiled_conversion_group.push(ds::Rule {
                    ante: ante.into(),
                    key: key.into(),
                    post: post.into(),
                    replacer: replacer.into(),
                    cursor_offset,
                });
            }
            self.conversion_group_list.push(VarZeroVec::from(&compiled_conversion_group));
        }

        let res = ds::RuleBasedTransliterator {
            visibility: true,
            filter: global_filter.map(|f| f.code_points().clone()).unwrap_or(CodePointInversionList::all()),
            id_group_list: VarZeroVec::from(&self.id_group_list),
            rule_group_list: VarZeroVec::from(&self.conversion_group_list),
            variable_table: self.var_table.finalize(),
        };

        Ok(res)
    }

    fn compile_single_id(&mut self, id: &parse::SingleId) -> ds::SimpleId<'static> {
        let id_string = id.basic_id.source.clone(); // TODO(#3736): map legacy ID to internal ID and use here

        ds::SimpleId {
            id: id_string.into(),
            filter: id.filter.as_ref().map(|f| f.code_points().clone()).unwrap_or(CodePointInversionList::all()),
        }
    }

    // returns the standin char
    fn compile_variable(&mut self, var: &str) -> char {
        if let Some(c) = self.var_to_char.get(var) {
            return *c;
        }
        let definition = self.var_definitions[var];
        let compiled = self.compile_section(definition);
        let standin = self.var_table.insert_compound(compiled);
        self.var_to_char.insert(var.to_owned(), standin);
        standin
    }

    fn compile_section(&mut self, section: &'p [parse::Element]) -> String {
        let mut result = String::new();
        for elt in section {
            match self.compile_element(elt) {
                LiteralOrStandin::Literal(s) => result.push_str(s),
                LiteralOrStandin::Standin(c) => result.push(c),
            }
        }
        result
    }

    fn compile_element(&mut self, elt: &'p parse::Element) -> LiteralOrStandin<'p> {
        match elt {
            parse::Element::Literal(s) => LiteralOrStandin::Literal(s),
            parse::Element::VariableRef(v) => LiteralOrStandin::Standin(self.compile_variable(v)),
            parse::Element::Quantifier(q, inner) => {
                let inner = self.compile_element(inner).to_string();
                let standin = match q {
                    parse::QuantifierKind::ZeroOrOne => self.var_table.insert_quantifier_opt(inner),
                    parse::QuantifierKind::ZeroOrMore => {
                        self.var_table.insert_quantifier_kleene(inner)
                    }
                    parse::QuantifierKind::OneOrMore => {
                        self.var_table.insert_quantifier_kleene_plus(inner)
                    }
                };
                LiteralOrStandin::Standin(standin)
            }
            parse::Element::BackRef(num) => {
                LiteralOrStandin::Standin(self.var_table.standin_for_backref(*num))
            }
            parse::Element::AnchorEnd => {
                LiteralOrStandin::Standin(MutVarTable::RESERVED_ANCHOR_END)
            }
            parse::Element::AnchorStart => {
                LiteralOrStandin::Standin(MutVarTable::RESERVED_ANCHOR_START)
            }
            parse::Element::UnicodeSet(set) => {
                let standin = self.var_table.insert_unicode_set(set.clone());
                LiteralOrStandin::Standin(standin)
            }
            parse::Element::Segment(inner) => {
                let inner = self.compile_section(inner);
                let standin = self.var_table.insert_segment(inner);
                LiteralOrStandin::Standin(standin)
            }
            parse::Element::FunctionCall(id, inner) => {
                let inner = self.compile_section(inner);
                let id = self.compile_single_id(id);
                let standin = self.var_table.insert_function_call(ds::FunctionCall {
                    translit: id,
                    arg: inner.into(),
                });
                LiteralOrStandin::Standin(standin)
            }
            parse::Element::Cursor(..) => {
                // TODO: compile this in some other step
                LiteralOrStandin::Literal("")
            }
        }
    }
}
