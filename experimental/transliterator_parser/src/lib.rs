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

mod compile;
mod parse;

pub use parse::ElementKind;
pub use parse::ElementLocation;
pub use parse::ParseError;
pub use parse::ParseErrorKind;

/// Parse a rule based transliterator definition into a `TransliteratorDataStruct`.
///
/// See [UTS #35 - Transliterators](https://unicode.org/reports/tr35/tr35-general.html#Transforms) for more information.
#[cfg(feature = "compiled_data")]
pub fn parse(
    source: &str,
) -> Result<
    (
        Option<RuleBasedTransliterator<'static>>,
        Option<RuleBasedTransliterator<'static>>,
    ),
    parse::ParseError,
> {
    parse_unstable(source, &icu_properties::provider::Baked)
}

#[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, parse())]
pub fn parse_unstable<P>(
    source: &str,
    provider: &P,
) -> Result<
    (
        Option<RuleBasedTransliterator<'static>>,
        Option<RuleBasedTransliterator<'static>>,
    ),
    parse::ParseError,
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
    // TODO(#3736): pass direction from metadata
    compile::compile(parsed, parse::Direction::Both)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::UnicodeSet;
    use icu_collections::codepointinvlist::CodePointInversionList;
    use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
    use icu_transliteration::provider as ds;
    use zerofrom::ZeroFrom;

    fn parse_set(source: &str) -> UnicodeSet {
        icu_unicodeset_parser::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
    }

    #[test]
    fn test_source_to_struct() {
        let source = r#"
        :: [1] ;
        :: Latin-InterIndic ;
        $a = [a] [b]+ ;
        $unused = [c{string}]+? ;
        $b = $a? 'literal chars' ;
        x } [a-z] > y ;
        $a > ab ;
        'reverse output:' &RevFnCall($1 'padding') < ($b) ;
        ^ left } $ <> ^ { right } [0-9] $ ;
        :: [\ ] Remove (AnyRev-AddRandomSpaces/FiftyPercent) ;
        # splits up the forward rules
        forward rule that > splits up rule groups ;
        :: InterIndic-Devanagari ;
        "#;

        let (forward, reverse) = parse(source).expect("parsing failed");
        let forward = forward.expect("forward transliterator expected");
        let reverse = reverse.expect("reverse transliterator expected");

        {
            assert_eq!(&forward.filter, parse_set("[1]").code_points());

            let vt = &forward.variable_table;
            assert_eq!(vt.compounds.len(), 1);
            assert_eq!(vt.quantifiers_opt.len(), 0);
            assert_eq!(vt.quantifiers_kleene.len(), 0);
            assert_eq!(vt.quantifiers_kleene_plus.len(), 1);
            assert_eq!(vt.segments.len(), 0);
            assert_eq!(vt.unicode_sets.len(), 3);
            assert_eq!(vt.function_calls.len(), 0);

            assert_eq!(&vt.compounds[0], "\u{F0003}\u{F0001}"); // [a] and [b]+ (the quantifier contains [b])
            assert_eq!(&vt.quantifiers_kleene_plus[0], "\u{F0004}"); // [b] from [b]+
            let uset1 = CodePointInversionListAndStringList::zero_from(&vt.unicode_sets[0]);
            assert_eq!(uset1, parse_set("[a-z]"));
            let uset2 = CodePointInversionListAndStringList::zero_from(&vt.unicode_sets[1]);
            assert_eq!(uset2, parse_set("[a]"));
            let uset3 = CodePointInversionListAndStringList::zero_from(&vt.unicode_sets[2]);
            assert_eq!(uset3, parse_set("[b]"));

            assert_eq!(forward.id_group_list.len(), 3);
            assert_eq!(forward.rule_group_list.len(), 3);

            assert_eq!(forward.id_group_list[0].len(), 1);
            assert_eq!(forward.id_group_list[1].len(), 1);
            assert_eq!(forward.id_group_list[2].len(), 1);

            assert_eq!(forward.rule_group_list[0].len(), 3);
            assert_eq!(forward.rule_group_list[1].len(), 1);
            assert_eq!(forward.rule_group_list[2].len(), 0);

            let rule1_1 = ds::Rule::zero_from(&forward.rule_group_list[0][0]);
            assert_eq!(rule1_1.ante, "");
            assert_eq!(rule1_1.key, "x");
            assert_eq!(rule1_1.post, "\u{F0002}"); // [a-z]
            assert_eq!(rule1_1.replacer, "y");

            let rule1_2 = ds::Rule::zero_from(&forward.rule_group_list[0][1]);
            assert_eq!(rule1_2.ante, "");
            assert_eq!(rule1_2.key, "\u{F0000}"); // $a
            assert_eq!(rule1_2.post, "");
            assert_eq!(rule1_2.replacer, "ab");

            let rule1_3 = ds::Rule::zero_from(&forward.rule_group_list[0][2]);
            assert_eq!(rule1_3.ante, "");
            assert_eq!(rule1_3.key, "\u{FFFFC}left"); // start anchor
            assert_eq!(rule1_3.post, "\u{FFFFD}"); // end anchor
            assert_eq!(rule1_3.replacer, "right");

            let rule2_1 = ds::Rule::zero_from(&forward.rule_group_list[1][0]);
            assert_eq!(rule2_1.ante, "");
            assert_eq!(rule2_1.key, "forwardrulethat");
            assert_eq!(rule2_1.post, "");
            assert_eq!(rule2_1.replacer, "splitsuprulegroups");

            let id1 = ds::SimpleId::zero_from(&forward.id_group_list[0][0]);
            assert_eq!(id1.id, "Latin-InterIndic");
            assert_eq!(id1.filter, CodePointInversionList::all());

            let id2 = ds::SimpleId::zero_from(&forward.id_group_list[1][0]);
            assert_eq!(id2.id, "Any-Remove");
            assert_eq!(&id2.filter, parse_set(r"[\ ]").code_points());

            let id3 = ds::SimpleId::zero_from(&forward.id_group_list[2][0]);
            assert_eq!(id3.id, "InterIndic-Devanagari");
            assert_eq!(id3.filter, CodePointInversionList::all());
        }
        {
            assert_eq!(&reverse.filter, &CodePointInversionList::all());

            let vt = &reverse.variable_table;
            assert_eq!(vt.compounds.len(), 2); // base: \u{F0000}
            assert_eq!(vt.quantifiers_opt.len(), 1); // base: \u{F0002}
            assert_eq!(vt.quantifiers_kleene.len(), 0); // base: \u{F0003}
            assert_eq!(vt.quantifiers_kleene_plus.len(), 1); // base: \u{F0003}
            assert_eq!(vt.segments.len(), 1); // base: \u{F0004}
            assert_eq!(vt.unicode_sets.len(), 3); // base: \u{F0005}
            assert_eq!(vt.function_calls.len(), 1); // base: \u{F0008}
                                                    // backref base: \u{F0009}

            assert_eq!(&vt.compounds[0], "\u{F0005}\u{F0003}"); // $a = [a] [b]+ (quantifier contains [b])
            assert_eq!(&vt.compounds[1], "\u{F0002}literal chars"); // $b = $a? (quantifier contains $a)
            assert_eq!(&vt.quantifiers_opt[0], "\u{F0000}"); // $a from $a?
            assert_eq!(&vt.quantifiers_kleene_plus[0], "\u{F0006}"); // [b] from [b]+
            assert_eq!(&vt.segments[0], "\u{F0001}"); // $b from ($b)
            let uset1 = CodePointInversionListAndStringList::zero_from(&vt.unicode_sets[0]);
            assert_eq!(uset1, parse_set("[a]"));
            let uset2 = CodePointInversionListAndStringList::zero_from(&vt.unicode_sets[1]);
            assert_eq!(uset2, parse_set("[b]"));
            let uset3 = CodePointInversionListAndStringList::zero_from(&vt.unicode_sets[2]);
            assert_eq!(uset3, parse_set("[0-9]"));
            let fcall = ds::FunctionCall::zero_from(&vt.function_calls[0]);
            assert_eq!(fcall.translit.id, "Any-RevFnCall");
            assert_eq!(fcall.translit.filter, CodePointInversionList::all());
            assert_eq!(fcall.arg, "\u{F0009}padding"); // $1 and 'padding'

            assert_eq!(reverse.id_group_list.len(), 2);
            assert_eq!(reverse.rule_group_list.len(), 2);

            assert_eq!(reverse.id_group_list[0].len(), 2);
            assert_eq!(reverse.id_group_list[1].len(), 1);

            assert_eq!(reverse.rule_group_list[0].len(), 2);
            assert_eq!(reverse.rule_group_list[1].len(), 0);

            let rule1_1 = ds::Rule::zero_from(&reverse.rule_group_list[0][0]);
            assert_eq!(rule1_1.ante, "");
            assert_eq!(rule1_1.key, "\u{F0004}");
            assert_eq!(rule1_1.post, ""); // [a-z]
            assert_eq!(rule1_1.replacer, "reverse output:\u{F0008}"); // function call

            let rule1_2 = ds::Rule::zero_from(&reverse.rule_group_list[0][1]);
            assert_eq!(rule1_2.ante, "\u{FFFFC}"); // start anchor
            assert_eq!(rule1_2.key, "right");
            assert_eq!(rule1_2.post, "\u{F0007}\u{FFFFD}"); // [0-9] and end anchor
            assert_eq!(rule1_2.replacer, "left");

            let id1_1 = ds::SimpleId::zero_from(&reverse.id_group_list[0][0]);
            assert_eq!(id1_1.id, "Devanagari-InterIndic");
            assert_eq!(id1_1.filter, CodePointInversionList::all());

            let id1_2 = ds::SimpleId::zero_from(&reverse.id_group_list[0][1]);
            assert_eq!(id1_2.id, "AnyRev-AddRandomSpaces/FiftyPercent");
            assert_eq!(id1_2.filter, CodePointInversionList::all());

            let id2_1 = ds::SimpleId::zero_from(&reverse.id_group_list[1][0]);
            assert_eq!(id2_1.id, "InterIndic-Latin");
            assert_eq!(id2_1.filter, CodePointInversionList::all());
        }
    }
}
