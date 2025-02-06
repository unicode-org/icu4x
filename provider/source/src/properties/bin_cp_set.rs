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
    (AsciiHexDigitV1, "AHex"),
    (AlnumV1, "alnum"),
    (AlphabeticV1, "Alpha"),
    (BidiControlV1, "Bidi_C"),
    (BidiMirroredV1, "Bidi_M"),
    (BlankV1, "blank"),
    (CasedV1, "Cased"),
    (CaseIgnorableV1, "CI"),
    (FullCompositionExclusionV1, "Comp_Ex"),
    (ChangesWhenCasefoldedV1, "CWCF"),
    (ChangesWhenCasemappedV1, "CWCM"),
    (ChangesWhenNfkcCasefoldedV1, "CWKCF"),
    (ChangesWhenLowercasedV1, "CWL"),
    (ChangesWhenTitlecasedV1, "CWT"),
    (ChangesWhenUppercasedV1, "CWU"),
    (DashV1, "Dash"),
    (DeprecatedV1, "Dep"),
    (DefaultIgnorableCodePointV1, "DI"),
    (DiacriticV1, "Dia"),
    (EmojiModifierBaseV1, "EBase"),
    (EmojiComponentV1, "EComp"),
    (EmojiModifierV1, "EMod"),
    (EmojiV1, "Emoji"),
    (EmojiPresentationV1, "EPres"),
    (ExtenderV1, "Ext"),
    (ExtendedPictographicV1, "ExtPict"),
    (GraphV1, "graph"),
    (GraphemeBaseV1, "Gr_Base"),
    (GraphemeExtendV1, "Gr_Ext"),
    (GraphemeLinkV1, "Gr_Link"),
    (HexDigitV1, "Hex"),
    (HyphenV1, "Hyphen"),
    (IdContinueV1, "IDC"),
    (IdeographicV1, "Ideo"),
    (IdStartV1, "IDS"),
    (IdsBinaryOperatorV1, "IDSB"),
    (IdsTrinaryOperatorV1, "IDST"),
    (JoinControlV1, "Join_C"),
    (LogicalOrderExceptionV1, "LOE"),
    (LowercaseV1, "Lower"),
    (MathV1, "Math"),
    (NoncharacterCodePointV1, "NChar"),
    (NfcInertV1, "nfcinert"),
    (NfdInertV1, "nfdinert"),
    (NfkcInertV1, "nfkcinert"),
    (NfkdInertV1, "nfkdinert"),
    (PatternSyntaxV1, "Pat_Syn"),
    (PatternWhiteSpaceV1, "Pat_WS"),
    (PrependedConcatenationMarkV1, "PCM"),
    (PrintV1, "print"),
    (QuotationMarkV1, "QMark"),
    (RadicalV1, "Radical"),
    (RegionalIndicatorV1, "RI"),
    (SoftDottedV1, "SD"),
    (SegmentStarterV1, "segstart"),
    (CaseSensitiveV1, "Sensitive"),
    (SentenceTerminalV1, "STerm"),
    (TerminalPunctuationV1, "Term"),
    (UnifiedIdeographV1, "UIdeo"),
    (UppercaseV1, "Upper"),
    (VariationSelectorV1, "VS"),
    (WhiteSpaceV1, "WSpace"),
    (XdigitV1, "xdigit"),
    (XidContinueV1, "XIDC"),
    (XidStartV1, "XIDS")
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
