// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_transliterator_parser` is a utility crate of the [`ICU4X`] project.
//!
//! This crate provides parsing functionality for [UTS #35 - Transliterators](https://unicode.org/reports/tr35/tr35-general.html#Transforms).
//!
//! See [`parse`](crate::parse()) for more information.
//!
//! [`ICU4X`]: ../icu/index.html

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

use icu_properties::provider::*;
use icu_provider::prelude::*;
use icu_transliteration::provider::RuleBasedTransliterator;
use std::collections::HashMap;

mod compile;
mod errors;
mod parse;

pub use errors::ParseError;
pub use errors::ParseErrorKind;

pub use parse::Direction;

pub use compile::legacy_id_to_internal_id;
pub use compile::Metadata;

/// Parse a rule based transliterator definition into a `TransliteratorDataStruct`.
///
/// See [UTS #35 - Transliterators](https://unicode.org/reports/tr35/tr35-general.html#Transforms) for more information.
///
/// The `gen_direction` [`Direction`] argument
/// determines which of the returned options is populated. The first option will be populated
/// if the direction is [`Forward`](Direction::Forward), the second if the direction is
/// [`Reverse`](Direction::Reverse), and both if the direction is [`Both`](Direction::Both).
///
/// `transliterator_map` is a map from legacy IDs to internal ICU4X IDs. Occurring IDs in `source`
/// that do not have a corresponding entry in `transliterator_map` are still valid, but will be
/// compiled with [`legacy_id_to_internal_id`].
#[cfg(feature = "compiled_data")]
pub fn parse(
    source: &str,
    gen_direction: Direction,
    metadata: Metadata,
    transliterator_map: HashMap<String, String>,
) -> Result<
    (
        Option<RuleBasedTransliterator<'static>>,
        Option<RuleBasedTransliterator<'static>>,
    ),
    ParseError,
> {
    parse_unstable(
        source,
        gen_direction,
        metadata,
        transliterator_map,
        &icu_properties::provider::Baked,
    )
}

#[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, parse())]
pub fn parse_unstable<P>(
    source: &str,
    gen_direction: Direction,
    metadata: Metadata,
    transliterator_map: HashMap<String, String>,
    provider: &P,
) -> Result<
    (
        Option<RuleBasedTransliterator<'static>>,
        Option<RuleBasedTransliterator<'static>>,
    ),
    ParseError,
>
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
    let parsed = parse::parse_unstable(source, provider)?;
    compile::compile(parsed, gen_direction, metadata, transliterator_map)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::parse::{FilterSet, UnicodeSet};
    use icu_transliteration::provider as ds;
    use zerovec::VarZeroVec;

    fn parse_set(source: &str) -> UnicodeSet {
        icu_unicodeset_parser::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
    }

    fn parse_set_cp(source: &str) -> FilterSet {
        icu_unicodeset_parser::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
            .code_points()
            .clone()
    }

    #[test]
    fn test_source_to_struct() {
        let source = r#"
        :: [1] ;
        :: Latin-InterIndic ;
        $a = [a] [b]+ ;
        $unused = [c{string}]+? ;
        $b = $a? 'literal chars' ;
        x } [a-z] > |@@@y ;
        $a > ab ;
        'reverse output:' &RevFnCall($1 'padding') @@ | < ($b) ;
        ^ left } $ <> ^ { rig|ht } [0-9] $ ;
        :: [\ ] Remove (AnyRev-AddRandomSpaces/FiftyPercent) ;
        # splits up the forward rules
        forward rule that > splits up rule groups ;
        :: InterIndic-Devanagari ;
        "#;

        let t_map = HashMap::from([(
            "AnyRev-AddRandomSpaces/FiftyPercent".to_ascii_lowercase(),
            "und-t-s0-anyrev-d0-addrndsp-m0-fifty".into(),
        )]);

        let metadata = Metadata {
            direction: Direction::Both,
            visible: true,
        };

        let (forward, reverse) =
            parse(source, Direction::Both, metadata, t_map).expect("parsing failed");
        let forward = forward.expect("forward transliterator expected");
        let reverse = reverse.expect("reverse transliterator expected");

        {
            let expected_filter = parse_set_cp("[1]");

            let expected_id_group1 = vec![ds::SimpleId {
                filter: FilterSet::all(),
                id: Cow::Borrowed("x-latin-interindic"),
            }];
            let expected_id_group2 = vec![ds::SimpleId {
                filter: parse_set_cp(r"[\ ]"),
                id: Cow::Borrowed("x-any-remove"),
            }];
            let expected_id_group3 = vec![ds::SimpleId {
                filter: FilterSet::all(),
                id: Cow::Borrowed("x-interindic-devanagari"),
            }];

            let expected_id_group_list: Vec<VarZeroVec<'_, ds::SimpleIdULE>> = vec![
                VarZeroVec::from(&expected_id_group1),
                VarZeroVec::from(&expected_id_group2),
                VarZeroVec::from(&expected_id_group3),
            ];

            let expected_rule_group1 = vec![
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("x"),
                    post: Cow::Borrowed("\u{F0002}"),
                    replacer: Cow::Borrowed("\u{F0007}y"), // `|@@@`
                },
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("\u{F0000}"),
                    post: Cow::Borrowed(""),
                    replacer: Cow::Borrowed("ab"),
                },
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("\u{FFFFC}left"),
                    post: Cow::Borrowed("\u{FFFFD}"),
                    replacer: Cow::Borrowed("rig\u{FFFFB}ht"), // `|`
                },
            ];
            let expected_rule_group2 = vec![ds::Rule {
                ante: Cow::Borrowed(""),
                key: Cow::Borrowed("forwardrulethat"),
                post: Cow::Borrowed(""),
                replacer: Cow::Borrowed("splitsuprulegroups"),
            }];

            let expected_rule_group_list: Vec<VarZeroVec<'_, ds::RuleULE>> = vec![
                VarZeroVec::from(&expected_rule_group1),
                VarZeroVec::from(&expected_rule_group2),
                VarZeroVec::new(), // empty rule group after the last transform rule
            ];

            let expected_compounds = vec![
                "\u{F0003}\u{F0001}", // [a] and [b]+ (the quantifier contains [b])
            ];

            let expected_quantifiers_kleene_plus = vec![
                "\u{F0004}", // [b] from [b]+
            ];

            let expected_unicode_sets =
                vec![parse_set("[a-z]"), parse_set("[a]"), parse_set("[b]")];

            let expected_var_table = ds::VarTable {
                compounds: VarZeroVec::from(&expected_compounds),
                quantifiers_opt: VarZeroVec::new(),
                quantifiers_kleene: VarZeroVec::new(),
                quantifiers_kleene_plus: VarZeroVec::from(&expected_quantifiers_kleene_plus),
                segments: VarZeroVec::new(),
                unicode_sets: VarZeroVec::from(&expected_unicode_sets),
                function_calls: VarZeroVec::new(),
                max_left_placeholder_count: 0,
                max_right_placeholder_count: 3,
            };

            let expected_rbt = ds::RuleBasedTransliterator {
                filter: expected_filter,
                id_group_list: VarZeroVec::from(&expected_id_group_list),
                rule_group_list: VarZeroVec::from(&expected_rule_group_list),
                variable_table: expected_var_table,
                visibility: true,
                dependencies: VarZeroVec::from(&[
                    "x-any-remove",
                    "x-interindic-devanagari",
                    "x-latin-interindic",
                ]),
            };
            assert_eq!(forward, expected_rbt);
        }
        {
            let expected_filter = FilterSet::all();

            let expected_id_group1 = vec![
                ds::SimpleId {
                    filter: FilterSet::all(),
                    id: Cow::Borrowed("x-devanagari-interindic"),
                },
                ds::SimpleId {
                    filter: FilterSet::all(),
                    id: Cow::Borrowed("und-t-s0-anyrev-d0-addrndsp-m0-fifty"),
                },
            ];
            let expected_id_group2 = vec![ds::SimpleId {
                filter: FilterSet::all(),
                id: Cow::Borrowed("x-interindic-latin"),
            }];

            let expected_id_group_list: Vec<VarZeroVec<'_, ds::SimpleIdULE>> = vec![
                VarZeroVec::from(&expected_id_group1),
                VarZeroVec::from(&expected_id_group2),
            ];

            let expected_rule_group1 = vec![
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("\u{F0004}"),
                    post: Cow::Borrowed(""),
                    replacer: Cow::Borrowed("reverse output:\u{F0008}\u{F000A}"), // `@@|` and function call
                },
                ds::Rule {
                    ante: Cow::Borrowed("\u{FFFFC}"), // start anchor
                    key: Cow::Borrowed("right"),
                    post: Cow::Borrowed("\u{F0007}\u{FFFFD}"), // [0-9] and end anchor
                    replacer: Cow::Borrowed("left"),
                },
            ];

            let expected_rule_group_list: Vec<VarZeroVec<'_, ds::RuleULE>> =
                vec![VarZeroVec::from(&expected_rule_group1), VarZeroVec::new()];

            let expected_compounds = vec![
                "\u{F0005}\u{F0003}",     // $a = [a] [b]+ (quantifier contains [b])
                "\u{F0002}literal chars", // $b = $a? (quantifier contains $a)
            ];

            let expected_quantifers_opt = vec![
                "\u{F0000}", // $a from $a?
            ];

            let expected_quantifiers_kleene_plus = vec![
                "\u{F0006}", // [b] from [b]+
            ];

            let expected_segments = vec![ds::Segment {
                idx: 0,
                content: Cow::Borrowed("\u{F0001}"), // $b from ($b)
            }];

            let expected_unicode_sets =
                vec![parse_set("[a]"), parse_set("[b]"), parse_set("[0-9]")];

            let expected_function_calls = vec![ds::FunctionCall {
                arg: Cow::Borrowed("\u{F000B}padding"), // $1 and 'padding'
                translit: ds::SimpleId {
                    filter: FilterSet::all(),
                    id: Cow::Borrowed("x-any-revfncall"),
                },
            }];

            let expected_var_table = ds::VarTable {
                compounds: VarZeroVec::from(&expected_compounds),
                quantifiers_opt: VarZeroVec::from(&expected_quantifers_opt),
                quantifiers_kleene: VarZeroVec::new(),
                quantifiers_kleene_plus: VarZeroVec::from(&expected_quantifiers_kleene_plus),
                segments: VarZeroVec::from(&expected_segments),
                unicode_sets: VarZeroVec::from(&expected_unicode_sets),
                function_calls: VarZeroVec::from(&expected_function_calls),
                max_left_placeholder_count: 2,
                max_right_placeholder_count: 0,
            };

            let expected_rbt = ds::RuleBasedTransliterator {
                filter: expected_filter,
                id_group_list: VarZeroVec::from(&expected_id_group_list),
                rule_group_list: VarZeroVec::from(&expected_rule_group_list),
                variable_table: expected_var_table,
                visibility: true,
                dependencies: VarZeroVec::from(&[
                    "und-t-s0-anyrev-d0-addrndsp-m0-fifty",
                    "x-any-revfncall",
                    "x-devanagari-interindic",
                    "x-interindic-latin",
                ]),
            };
            assert_eq!(reverse, expected_rbt);
        }
    }
}
