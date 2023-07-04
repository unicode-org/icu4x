// This file just serves as a API-only design of the transliterator datastruct

use std::borrow::Cow;

use zerovec::*;

struct Transliterator {
    filter: Option<UnicodeSet>,
    // required for the case where the first RuleSet appears before the first recursive ID
    prefix_rule_set: Option<RuleSet>,
    groups: Vec<TransliterationGroup>,
}

/*

optional filter
conversionrule?
(recursive id, conversion rule*)*


 */

// exactly one of:    :: Any-Any ;
// zero or more of:   x > b ; a > b ; ...
struct TransliterationGroup {
    // :: Any-Any ;
    // exactly one
    inner: SimpleID,
    // x > b ; a > b ; ...
    // zero or more
    rules: RuleSet,
}

struct RuleSet(Vec<Rule>);

// e.g.: [a-z] Any-Remove/BGN;
// but actually, in BCP47: [a-z] und
struct SimpleID {
    filter: Option<UnicodeSet>,
    // TODO: Discuss if this should be in auxiliary
    translit: Cow<'a, str>,
}