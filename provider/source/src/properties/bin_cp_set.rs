// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::properties::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl SourceDataProvider {
    // get the source data for a Unicode binary property that only defines values for code points
    pub(super) fn get_binary_prop_for_code_point_set<'a>(
        &'a self,
        key: &str,
    ) -> Result<&'a super::uprops_serde::binary::BinaryProperty, DataError> {
        self.icuexport()?
            .read_and_parse_toml::<super::uprops_serde::binary::Main>(&format!(
                "uprops/{}/{}.toml",
                self.trie_type(),
                key
            ))?
            .binary_property
            .first()
            .ok_or_else(|| DataErrorKind::MarkerNotFound.into_error())
    }
}

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_binary_prop_for_code_point_set($prop_name)?;

                    let mut builder = CodePointInversionListBuilder::new();
                    for (start, end) in &data.ranges {
                        builder.add_range32(start..=end);
                    }
                    let inv_list = builder.build();

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(
                            PropertyCodePointSet::InversionList(inv_list),
                        ),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    self.get_binary_prop_for_code_point_set($prop_name)?;

                    Ok(HashSet::from_iter([Default::default()]))
                }
            }
        )+
    };
}

expand!(
    (PropertyBinaryAsciiHexDigitV1, "AHex"),
    (PropertyBinaryAlnumV1, "alnum"),
    (PropertyBinaryAlphabeticV1, "Alpha"),
    (PropertyBinaryBidiControlV1, "Bidi_C"),
    (PropertyBinaryBidiMirroredV1, "Bidi_M"),
    (PropertyBinaryBlankV1, "blank"),
    (PropertyBinaryCasedV1, "Cased"),
    (PropertyBinaryCaseIgnorableV1, "CI"),
    (PropertyBinaryFullCompositionExclusionV1, "Comp_Ex"),
    (PropertyBinaryChangesWhenCasefoldedV1, "CWCF"),
    (PropertyBinaryChangesWhenCasemappedV1, "CWCM"),
    (PropertyBinaryChangesWhenNfkcCasefoldedV1, "CWKCF"),
    (PropertyBinaryChangesWhenLowercasedV1, "CWL"),
    (PropertyBinaryChangesWhenTitlecasedV1, "CWT"),
    (PropertyBinaryChangesWhenUppercasedV1, "CWU"),
    (PropertyBinaryDashV1, "Dash"),
    (PropertyBinaryDeprecatedV1, "Dep"),
    (PropertyBinaryDefaultIgnorableCodePointV1, "DI"),
    (PropertyBinaryDiacriticV1, "Dia"),
    (PropertyBinaryEmojiModifierBaseV1, "EBase"),
    (PropertyBinaryEmojiComponentV1, "EComp"),
    (PropertyBinaryEmojiModifierV1, "EMod"),
    (PropertyBinaryEmojiV1, "Emoji"),
    (PropertyBinaryEmojiPresentationV1, "EPres"),
    (PropertyBinaryExtenderV1, "Ext"),
    (PropertyBinaryExtendedPictographicV1, "ExtPict"),
    (PropertyBinaryGraphV1, "graph"),
    (PropertyBinaryGraphemeBaseV1, "Gr_Base"),
    (PropertyBinaryGraphemeExtendV1, "Gr_Ext"),
    (PropertyBinaryGraphemeLinkV1, "Gr_Link"),
    (PropertyBinaryHexDigitV1, "Hex"),
    (PropertyBinaryHyphenV1, "Hyphen"),
    (PropertyBinaryIdContinueV1, "IDC"),
    (PropertyBinaryIdeographicV1, "Ideo"),
    (PropertyBinaryIdStartV1, "IDS"),
    (PropertyBinaryIdsBinaryOperatorV1, "IDSB"),
    (PropertyBinaryIdsTrinaryOperatorV1, "IDST"),
    (PropertyBinaryJoinControlV1, "Join_C"),
    (PropertyBinaryLogicalOrderExceptionV1, "LOE"),
    (PropertyBinaryLowercaseV1, "Lower"),
    (PropertyBinaryMathV1, "Math"),
    (PropertyBinaryNoncharacterCodePointV1, "NChar"),
    (PropertyBinaryNfcInertV1, "nfcinert"),
    (PropertyBinaryNfdInertV1, "nfdinert"),
    (PropertyBinaryNfkcInertV1, "nfkcinert"),
    (PropertyBinaryNfkdInertV1, "nfkdinert"),
    (PropertyBinaryPatternSyntaxV1, "Pat_Syn"),
    (PropertyBinaryPatternWhiteSpaceV1, "Pat_WS"),
    (PropertyBinaryPrependedConcatenationMarkV1, "PCM"),
    (PropertyBinaryPrintV1, "print"),
    (PropertyBinaryQuotationMarkV1, "QMark"),
    (PropertyBinaryRadicalV1, "Radical"),
    (PropertyBinaryRegionalIndicatorV1, "RI"),
    (PropertyBinarySoftDottedV1, "SD"),
    (PropertyBinarySegmentStarterV1, "segstart"),
    (PropertyBinaryCaseSensitiveV1, "Sensitive"),
    (PropertyBinarySentenceTerminalV1, "STerm"),
    (PropertyBinaryTerminalPunctuationV1, "Term"),
    (PropertyBinaryUnifiedIdeographV1, "UIdeo"),
    (PropertyBinaryUppercaseV1, "Upper"),
    (PropertyBinaryVariationSelectorV1, "VS"),
    (PropertyBinaryWhiteSpaceV1, "WSpace"),
    (PropertyBinaryXdigitV1, "xdigit"),
    (PropertyBinaryXidContinueV1, "XIDC"),
    (PropertyBinaryXidStartV1, "XIDS")
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
