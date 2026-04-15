// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder};
use icu::properties::props::BinaryProperty;
use icu::properties::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl SourceDataProvider {
    // get the source data for a Unicode binary property that only defines values for code points
    pub(super) fn get_binary_prop_for_code_point_set<'a>(
        &'a self,
        name: &str,
        short_name: &str,
    ) -> Result<&'a super::uprops_serde::binary::BinaryProperty, DataError> {
        let data = self
            .icuexport()?
            .read_and_parse_toml::<super::uprops_serde::binary::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                short_name
            ))?
            .binary_property
            .first()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.into_error())?;

        if name != data.long_name
            || short_name != data.short_name.as_ref().unwrap_or(&data.long_name)
        {
            return Err(DataError::custom("Property name mismatch").with_display_context(name));
        }

        Ok(data)
    }
}

impl super::uprops_serde::binary::BinaryProperty {
    pub(crate) fn build_inversion_list(&self) -> CodePointInversionList<'static> {
        let mut builder = CodePointInversionListBuilder::new();
        for (start, end) in &self.ranges {
            builder.add_range32(start..=end);
        }
        builder.build()
    }
}

macro_rules! expand {
    ($(($prop:ty, $marker:ident)),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_binary_prop_for_code_point_set(
                        core::str::from_utf8(<$prop as BinaryProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as BinaryProperty>::SHORT_NAME).unwrap()
                    )?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(data.build_inversion_list()))
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    Ok(HashSet::from_iter([Default::default()]))
                }
            }
        )+
    };
}

expand!(
    (
        icu::properties::props::AsciiHexDigit,
        PropertyBinaryAsciiHexDigitV1
    ),
    (icu::properties::props::Alnum, PropertyBinaryAlnumV1),
    (
        icu::properties::props::Alphabetic,
        PropertyBinaryAlphabeticV1
    ),
    (
        icu::properties::props::BidiControl,
        PropertyBinaryBidiControlV1
    ),
    (
        icu::properties::props::BidiMirrored,
        PropertyBinaryBidiMirroredV1
    ),
    (icu::properties::props::Blank, PropertyBinaryBlankV1),
    (icu::properties::props::Cased, PropertyBinaryCasedV1),
    (
        icu::properties::props::CaseIgnorable,
        PropertyBinaryCaseIgnorableV1
    ),
    (
        icu::properties::props::FullCompositionExclusion,
        PropertyBinaryFullCompositionExclusionV1
    ),
    (
        icu::properties::props::ChangesWhenCasefolded,
        PropertyBinaryChangesWhenCasefoldedV1
    ),
    (
        icu::properties::props::ChangesWhenCasemapped,
        PropertyBinaryChangesWhenCasemappedV1
    ),
    (
        icu::properties::props::ChangesWhenNfkcCasefolded,
        PropertyBinaryChangesWhenNfkcCasefoldedV1
    ),
    (
        icu::properties::props::ChangesWhenLowercased,
        PropertyBinaryChangesWhenLowercasedV1
    ),
    (
        icu::properties::props::ChangesWhenTitlecased,
        PropertyBinaryChangesWhenTitlecasedV1
    ),
    (
        icu::properties::props::ChangesWhenUppercased,
        PropertyBinaryChangesWhenUppercasedV1
    ),
    (icu::properties::props::Dash, PropertyBinaryDashV1),
    (
        icu::properties::props::Deprecated,
        PropertyBinaryDeprecatedV1
    ),
    (
        icu::properties::props::DefaultIgnorableCodePoint,
        PropertyBinaryDefaultIgnorableCodePointV1
    ),
    (icu::properties::props::Diacritic, PropertyBinaryDiacriticV1),
    (
        icu::properties::props::EmojiModifierBase,
        PropertyBinaryEmojiModifierBaseV1
    ),
    (
        icu::properties::props::EmojiComponent,
        PropertyBinaryEmojiComponentV1
    ),
    (
        icu::properties::props::EmojiModifier,
        PropertyBinaryEmojiModifierV1
    ),
    (icu::properties::props::Emoji, PropertyBinaryEmojiV1),
    (
        icu::properties::props::EmojiPresentation,
        PropertyBinaryEmojiPresentationV1
    ),
    (icu::properties::props::Extender, PropertyBinaryExtenderV1),
    (
        icu::properties::props::ExtendedPictographic,
        PropertyBinaryExtendedPictographicV1
    ),
    (icu::properties::props::Graph, PropertyBinaryGraphV1),
    (
        icu::properties::props::GraphemeBase,
        PropertyBinaryGraphemeBaseV1
    ),
    (
        icu::properties::props::GraphemeExtend,
        PropertyBinaryGraphemeExtendV1
    ),
    (
        icu::properties::props::GraphemeLink,
        PropertyBinaryGraphemeLinkV1
    ),
    (icu::properties::props::HexDigit, PropertyBinaryHexDigitV1),
    (icu::properties::props::Hyphen, PropertyBinaryHyphenV1),
    (
        icu::properties::props::IdCompatMathContinue,
        PropertyBinaryIdCompatMathContinueV1
    ),
    (
        icu::properties::props::IdCompatMathStart,
        PropertyBinaryIdCompatMathStartV1
    ),
    (
        icu::properties::props::IdContinue,
        PropertyBinaryIdContinueV1
    ),
    (
        icu::properties::props::Ideographic,
        PropertyBinaryIdeographicV1
    ),
    (icu::properties::props::IdStart, PropertyBinaryIdStartV1),
    (
        icu::properties::props::IdsBinaryOperator,
        PropertyBinaryIdsBinaryOperatorV1
    ),
    (
        icu::properties::props::IdsTrinaryOperator,
        PropertyBinaryIdsTrinaryOperatorV1
    ),
    (
        icu::properties::props::IdsUnaryOperator,
        PropertyBinaryIdsUnaryOperatorV1
    ),
    (
        icu::properties::props::JoinControl,
        PropertyBinaryJoinControlV1
    ),
    (
        icu::properties::props::LogicalOrderException,
        PropertyBinaryLogicalOrderExceptionV1
    ),
    (icu::properties::props::Lowercase, PropertyBinaryLowercaseV1),
    (icu::properties::props::Math, PropertyBinaryMathV1),
    (
        icu::properties::props::ModifierCombiningMark,
        PropertyBinaryModifierCombiningMarkV1
    ),
    (
        icu::properties::props::NoncharacterCodePoint,
        PropertyBinaryNoncharacterCodePointV1
    ),
    (icu::properties::props::NfcInert, PropertyBinaryNfcInertV1),
    (icu::properties::props::NfdInert, PropertyBinaryNfdInertV1),
    (icu::properties::props::NfkcInert, PropertyBinaryNfkcInertV1),
    (icu::properties::props::NfkdInert, PropertyBinaryNfkdInertV1),
    (
        icu::properties::props::PatternSyntax,
        PropertyBinaryPatternSyntaxV1
    ),
    (
        icu::properties::props::PatternWhiteSpace,
        PropertyBinaryPatternWhiteSpaceV1
    ),
    (
        icu::properties::props::PrependedConcatenationMark,
        PropertyBinaryPrependedConcatenationMarkV1
    ),
    (icu::properties::props::Print, PropertyBinaryPrintV1),
    (
        icu::properties::props::QuotationMark,
        PropertyBinaryQuotationMarkV1
    ),
    (icu::properties::props::Radical, PropertyBinaryRadicalV1),
    (
        icu::properties::props::RegionalIndicator,
        PropertyBinaryRegionalIndicatorV1
    ),
    (
        icu::properties::props::SoftDotted,
        PropertyBinarySoftDottedV1
    ),
    (
        icu::properties::props::SegmentStarter,
        PropertyBinarySegmentStarterV1
    ),
    (
        icu::properties::props::CaseSensitive,
        PropertyBinaryCaseSensitiveV1
    ),
    (
        icu::properties::props::SentenceTerminal,
        PropertyBinarySentenceTerminalV1
    ),
    (
        icu::properties::props::TerminalPunctuation,
        PropertyBinaryTerminalPunctuationV1
    ),
    (
        icu::properties::props::UnifiedIdeograph,
        PropertyBinaryUnifiedIdeographV1
    ),
    (icu::properties::props::Uppercase, PropertyBinaryUppercaseV1),
    (
        icu::properties::props::VariationSelector,
        PropertyBinaryVariationSelectorV1
    ),
    (
        icu::properties::props::WhiteSpace,
        PropertyBinaryWhiteSpaceV1
    ),
    (icu::properties::props::Xdigit, PropertyBinaryXdigitV1),
    (
        icu::properties::props::XidContinue,
        PropertyBinaryXidContinueV1
    ),
    (icu::properties::props::XidStart, PropertyBinaryXidStartV1)
);

#[test]
fn test_basic() {
    use icu::properties::{props::WhiteSpace, CodePointSetData};

    let provider = SourceDataProvider::new_testing();

    let whitespace = CodePointSetData::try_new_unstable::<WhiteSpace>(&provider).unwrap();
    let whitespace = whitespace.as_code_point_inversion_list().unwrap();

    assert!(whitespace.contains(' '));
    assert!(whitespace.contains('\n'));
    assert!(whitespace.contains('\u{3000}')); // U+3000 IDEOGRAPHIC SPACE

    assert!(!whitespace.contains('A'));
}
