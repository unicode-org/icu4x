// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::RuleBasedTransliterator;
use crate::{CompileErrorKind, TransliteratorError};

use icu_properties::{provider::*, sets, PropertiesError};
use icu_provider::prelude::*;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, TransliteratorError>;

mod parse;
mod pass1;
mod pass2;
mod rule_group_agg;

/// The direction of a rule-based transliterator in respect to its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Direction {
    /// Forwards, i.e., left-to-right in the source.
    Forward,
    /// Backwards, i.e., right-to-left in the source.
    Reverse,
    /// Both forwards and backwards.
    Both,
}

impl Direction {
    /// Whether `self` is a superset of `other` or not.
    pub fn permits(&self, other: Direction) -> bool {
        match self {
            Direction::Forward => other == Direction::Forward,
            Direction::Reverse => other == Direction::Reverse,
            Direction::Both => true,
        }
    }
}

/// Conversion from an unknown legacy ID to an internal ID is handled by this function.
///
/// Known legacy IDs, i.e., ones that have associated BCP47 IDs in their metadata, are simply
/// that BCP47 ID. For unknown legacy IDs, the output is given by this function.
pub fn legacy_id_to_internal_id(source: &str, target: &str, variant: Option<&str>) -> String {
    // TODO(#3891): Decide representation for unknown BCP47 IDs
    let mut id = format!("x-{source}-{target}");
    if let Some(variant) = variant {
        id.push('-');
        id.push_str(variant);
    }
    // normalizing to ASCII lowercase to avoid duplicate dependencies like Any-null and Any-Null
    id.make_ascii_lowercase();
    id
}

impl RuleBasedTransliterator<'static> {
    /// Parse a rule based transliterator definition into a `TransliteratorDataStruct`.
    ///
    /// See [UTS #35 - Transliterators](https://unicode.org/reports/tr35/tr35-general.html#Transforms) for more information.
    ///
    /// The `direction` [`Direction`] argument
    /// determines which of the returned options is populated. The first option will be populated
    /// if the direction is [`Forward`](Direction::Forward), the second if the direction is
    /// [`Reverse`](Direction::Reverse), and both if the direction is [`Both`](Direction::Both).
    ///
    /// `id_mapping` is a map from legacy IDs to internal ICU4X IDs. Occurring IDs in `source`
    /// that do not have a corresponding entry in `id_mapping` are still valid, but will be
    /// compiled with [`legacy_id_to_internal_id`].
    pub fn compile<P>(
        provider: &P,
        source: &str,
        direction: Direction,
        id_mapping: &HashMap<String, String>,
    ) -> Result<(Option<Self>, Option<Self>)>
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
        let unwrap_props_data_error = |e| {
            let PropertiesError::PropDataLoad(e) = e else {
                unreachable!()
            };
            e
        };

        let xid_start = sets::load_xid_start(provider).map_err(unwrap_props_data_error)?;
        let xid_continue = sets::load_xid_continue(provider).map_err(unwrap_props_data_error)?;
        let pat_ws = sets::load_pattern_white_space(provider).map_err(unwrap_props_data_error)?;

        let rules = parse::Parser::run(
            source,
            &xid_start.to_code_point_inversion_list(),
            &xid_continue.to_code_point_inversion_list(),
            &pat_ws.to_code_point_inversion_list(),
            provider,
        )?;

        // TODO(#3736): decide if validation should be direction dependent
        //  example: transliterator with direction "forward", and a rule `[a-z] < b ;` (invalid)
        //  - if validation is dependent, this rule is valid because it's not used in the forward direction
        //  - if validation is independent, this rule is invalid because the reverse direction is also checked
        let pass1 = pass1::Pass1::run(direction, &rules)?;

        let forward = direction.permits(Direction::Forward).then(|| {
            pass2::Pass2::run(
                pass1.forward_result,
                &pass1.variable_definitions,
                id_mapping,
            )
        });

        let reverse = direction.permits(Direction::Reverse).then(|| {
            pass2::Pass2::run(
                pass1.reverse_result,
                &pass1.variable_definitions,
                id_mapping,
            )
        });

        Ok((forward.transpose()?, reverse.transpose()?))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::provider as ds;
    use zerovec::VarZeroVec;

    fn parse_set(source: &str) -> parse::UnicodeSet {
        icu_unicodeset_parser::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
    }

    fn parse_set_cp(source: &str) -> parse::FilterSet {
        icu_unicodeset_parser::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
            .code_points()
            .clone()
    }

    #[test]
    fn test_source_to_struct() {
        let source = r"
        use variable range 0xFFF 0xFFFF ;
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
        :: NFC ;
        ";

        let id_mapping = HashMap::from([(
            "AnyRev-AddRandomSpaces/FiftyPercent".to_ascii_lowercase(),
            "und-t-s0-anyrev-d0-addrndsp-m0-fifty".into(),
        )]);

        let (forward, reverse) = RuleBasedTransliterator::compile(
            &icu_properties::provider::Baked,
            source,
            Direction::Both,
            &id_mapping,
        )
        .expect("compiling failed");
        let forward = forward.expect("forward transliterator expected");
        let reverse = reverse.expect("reverse transliterator expected");

        {
            let expected_filter = parse_set_cp("[1]");

            let expected_id_group1 = vec![ds::SimpleId {
                filter: parse::FilterSet::all(),
                id: Cow::Borrowed("x-latin-interindic"),
            }];
            let expected_id_group2 = vec![ds::SimpleId {
                filter: parse_set_cp(r"[\ ]"),
                id: Cow::Borrowed("x-any-remove"),
            }];
            let expected_id_group3 = vec![
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-interindic-devanagari"),
                },
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-any-nfc"),
                },
            ];

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
                    "x-any-nfc",
                    "x-any-remove",
                    "x-interindic-devanagari",
                    "x-latin-interindic",
                ]),
            };
            assert_eq!(forward, expected_rbt);
        }
        {
            let expected_filter = parse::FilterSet::all();

            let expected_id_group1 = vec![
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-any-nfd"),
                },
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-devanagari-interindic"),
                },
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("und-t-s0-anyrev-d0-addrndsp-m0-fifty"),
                },
            ];
            let expected_id_group2 = vec![ds::SimpleId {
                filter: parse::FilterSet::all(),
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
                    filter: parse::FilterSet::all(),
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
                    "x-any-nfd",
                    "x-any-revfncall",
                    "x-devanagari-interindic",
                    "x-interindic-latin",
                ]),
            };
            assert_eq!(reverse, expected_rbt);
        }
    }
}
