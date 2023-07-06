// This file just serves as a API-only design of the transliterator datastruct

use std::borrow::Cow;

use serde;
use icu_collections::codepointinvliststringlist::{CodePointInversionListAndStringList, CodePointInversionListAndStringListULE};
use zerovec::*;

type UnicodeSet<'a> = CodePointInversionListAndStringList<'a>;


// The datastruct for a full rule file (modulo naming and direction, direction is assumed forward)
/*
# Valid syntax is as such:
filter?       
(recursive_simple_id | conversion_rule)*

# because of semantics, I want to represent contiguous conversion_rules as a single block.
# the following "parsed" syntax enforces this:

filter?
(recursive_simple_id_list conversion_rule_list)*




LEGEND:
<filter> ::=                '::' <unicode-set> ';'
<recursive_simple_id> ::=   '::' <unicode-set>? (<source-name> '-')<target-name>('/' <variant-name>)? ';'
<conversion_rule> ::=       <source_matcher>+ '>' <replacer>* ';'
<conversion_rule_list> ::=  <conversion_rule>*
<recursive_simple_id_list> ::=  <recursive_simple_id>*
 */
#[derive(serde::Serialize, serde::Deserialize)]
struct Transliterator<'a> {
    // `false` for transliterators that should not be exposed to clients, such as using some InterIndic source/target as root
    visibility: bool,
    variable_table: VarTable<'a>,

    // only characters in this set are affected by the transliterator. None is equivalent to the full set
    // because of ULE things, removing the Option<> and adopting full = None semantics might be easier. 
    // filter?
    #[serde(borrow)]
    filter: UnicodeSet<'a>,
    // (recursive_simple_id_list conversion_rule_list)* is represented as a VZV of IDs and a VZV of conversion_rules, with the (weak) invariant
    // that IDs_list[i] is before CRULEs_list[i], eg <id> <id> <rule> <rule> <rule> <id> is represented as ids: [[<id>, <id>], [<id>]], rules: [[<rule>, <rule>, <rule>], []] 
    #[serde(borrow)]
    id_group_list: VarZeroVec<'a, VarZeroSlice<SimpleIDULE>>,
    #[serde(borrow)]
    rule_group_list: VarZeroVec<'a, VarZeroSlice<RuleULE>>,
}

// // exactly one of:    :: Any-Any ;
// // zero or more of:   x > b ; a > b ; ...
// #[make_varule(TransliterationGroupULE)]
// #[zerovec::skip_derive(Ord)]
// #[zerovec::derive(Serialize, Deserialize)]
// #[derive(serde::Serialize, serde::Deserialize)]
// struct TransliterationGroup<'a> {
//     // :: Any-Any ;
//     // exactly one
//     #[zerovec::varule(SimpleIDULE)]
//     #[serde(borrow)]
//     inner: SimpleID<'a>,
//     // x > b ; a > b ; ...
//     // zero or more
//     #[serde(borrow)]
//     rules: VarZeroVec<'a, RuleULE>,
// }

// replaced directly by the inner vec
// #[make_varule(RuleSetULE)]
// #[zerovec::skip_derive(Ord)]
// #[zerovec::derive(Serialize, Deserialize)]
// struct RuleSet<'a>(VarZeroVec<'a, RuleULE>);

// e.g.: [a-z] Any-Remove/BGN;
// but actually, in BCP47: [a-z] und
#[derive(Debug, Clone)]
#[make_varule(SimpleIDULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize)]
struct SimpleID<'a> {
    #[zerovec::varule(CodePointInversionListAndStringListULE)]
    #[serde(borrow)]
    filter: UnicodeSet<'a>,
    // TODO: Discuss if this should be in auxiliary
    #[serde(borrow)]
    translit: Cow<'a, str>,
}

#[make_varule(RuleULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Rule<'a> {
    // special case matchers (such as segments, quantifiers, unicodesets, variables...) are represented as chars in the PUA
    #[serde(borrow)]
    ante: Cow<'a, str>,
    #[serde(borrow)]
    key: Cow<'a, str>,
    #[serde(borrow)]
    post: Cow<'a, str>,

    // special case replacers (such as variables, backrefs, function calls)
    #[serde(borrow)]
    replacer: Cow<'a, str>,
}

// Shanes suggestion: VarTable = VarZeroVec<Uset | Quantifier | Variable | ..>


#[derive(serde::Serialize, serde::Deserialize)]
struct VarTable<'a> {
    // Examples:
    // $a = hello ;
    // $b = $a ' ' world ;
    //
    // is: $a => PUA_offset + 0, $b => PUA_offset + 1
    // and compounds: ["hello", "<PUA_offset + 0> world"]
    #[serde(borrow)]
    compounds: VarZeroVec<'a, str>,

    // having quantifiers_opt: [42 => "<PUA_offset + 5> world"] is the representation of
    // "(?:<some special matcher that has ID 5> ' ' world)?" in the rules  (if ?: was valid non-capturing syntax)
    //
    //
    #[serde(borrow)]
    quantifiers_opt: VarZeroVec<'a, str>, 
    // zero or more
    #[serde(borrow)]
    quantifiers_kleene: VarZeroVec<'a, str>, 
    // one or more
    #[serde(borrow)]
    quantifiers_kleene_plus: VarZeroVec<'a, str>, 

    #[serde(borrow)]
    segments: VarZeroVec<'a, str>,

    #[serde(borrow)]
    unicode_sets: VarZeroVec<'a, CodePointInversionListAndStringListULE>,

    #[serde(borrow)]
    function_calls: VarZeroVec<'a, FunctionCallULE>,
}

#[derive(Debug, Clone)]
#[make_varule(FunctionCallULE)]
#[zerovec::skip_derive(Ord)]
#[zerovec::derive(Serialize, Deserialize)]
#[derive(serde::Serialize, serde::Deserialize)]
struct FunctionCall<'a> {
    #[zerovec::varule(SimpleIDULE)]
    #[serde(borrow)]
    translit: SimpleID<'a>,
    #[serde(borrow)]
    arg: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use icu_collections::codepointinvlist::CodePointInversionList;
    use zerofrom::ZeroFrom;

    use super::*;

    const ALL: CodePointInversionList = CodePointInversionList::all();

    #[test]
    fn test_serialize() {
        let all_filter = CodePointInversionListAndStringList::try_from(ALL, VarZeroVec::new()).unwrap();
        
        // filter
        let filter = all_filter.clone();

        // groups
        let simple_id = SimpleID {
            filter: all_filter.clone(),
            translit: "Latin-Greek/BGN".into(),
        };
        let rule = Rule {
            ante: "prefix".into(),
            key: "replace_me".into(),
            post: "suffix".into(),
            replacer: "X".into(),
        };
        let rules = VarZeroVec::from(&[rule.clone()]);
        let ids = VarZeroVec::from(&[simple_id.clone()]);

        // vartable
        let function_call = FunctionCall {
            arg: "<assume this is a standing (PUA char) for some backref (aka segment)>".into(),
            translit: simple_id.clone(),
        };
        let var_table = VarTable {
            quantifiers_kleene_plus: VarZeroVec::from(&["help", "hello"]),
            compounds: VarZeroVec::new(),
            quantifiers_kleene: VarZeroVec::new(),
            segments: VarZeroVec::new(),
            quantifiers_opt: VarZeroVec::new(),
            unicode_sets: VarZeroVec::from(&[all_filter.clone()]),
            function_calls: VarZeroVec::from(&[function_call]),
        };

        let transliterator = Transliterator {
            visibility: true,
            filter: filter,
            variable_table: var_table,
            id_group_list: VarZeroVec::from(&[ids]),
            rule_group_list: VarZeroVec::from(&[rules]),
        };

        let x: &VarZeroSlice<RuleULE> = transliterator.rule_group_list.get(0).unwrap();
        let post: &str = unsafe {x.get(0).unwrap().unsized_fields.get_field(2)};
        assert_eq!(post, rule.post);

        let post_zero = x.get(0).unwrap();
        let post_zero: Rule = ZeroFrom::zero_from(post_zero);
        let post_zero = post_zero.post;
        assert_eq!(post_zero, rule.post);



        

        ()
    }
}