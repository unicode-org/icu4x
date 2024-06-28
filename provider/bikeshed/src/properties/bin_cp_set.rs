// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::DatagenProvider;
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::properties::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DatagenProvider {
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
            impl DataProvider<$marker> for DatagenProvider {
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
                            PropertyCodePointSetV1::InversionList(inv_list),
                        ),
                    })
                }
            }

            impl crate::IterableDataProviderCached<$marker> for DatagenProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    self.get_binary_prop_for_code_point_set($prop_name)?;

                    Ok(HashSet::from_iter([Default::default()]))
                }
            }
        )+
    };
}

expand!(
    (AsciiHexDigitV1Marker, "AHex"),
    (AlnumV1Marker, "alnum"),
    (AlphabeticV1Marker, "Alpha"),
    (BidiControlV1Marker, "Bidi_C"),
    (BidiMirroredV1Marker, "Bidi_M"),
    (BlankV1Marker, "blank"),
    (CasedV1Marker, "Cased"),
    (CaseIgnorableV1Marker, "CI"),
    (FullCompositionExclusionV1Marker, "Comp_Ex"),
    (ChangesWhenCasefoldedV1Marker, "CWCF"),
    (ChangesWhenCasemappedV1Marker, "CWCM"),
    (ChangesWhenNfkcCasefoldedV1Marker, "CWKCF"),
    (ChangesWhenLowercasedV1Marker, "CWL"),
    (ChangesWhenTitlecasedV1Marker, "CWT"),
    (ChangesWhenUppercasedV1Marker, "CWU"),
    (DashV1Marker, "Dash"),
    (DeprecatedV1Marker, "Dep"),
    (DefaultIgnorableCodePointV1Marker, "DI"),
    (DiacriticV1Marker, "Dia"),
    (EmojiModifierBaseV1Marker, "EBase"),
    (EmojiComponentV1Marker, "EComp"),
    (EmojiModifierV1Marker, "EMod"),
    (EmojiV1Marker, "Emoji"),
    (EmojiPresentationV1Marker, "EPres"),
    (ExtenderV1Marker, "Ext"),
    (ExtendedPictographicV1Marker, "ExtPict"),
    (GraphV1Marker, "graph"),
    (GraphemeBaseV1Marker, "Gr_Base"),
    (GraphemeExtendV1Marker, "Gr_Ext"),
    (GraphemeLinkV1Marker, "Gr_Link"),
    (HexDigitV1Marker, "Hex"),
    (HyphenV1Marker, "Hyphen"),
    (IdContinueV1Marker, "IDC"),
    (IdeographicV1Marker, "Ideo"),
    (IdStartV1Marker, "IDS"),
    (IdsBinaryOperatorV1Marker, "IDSB"),
    (IdsTrinaryOperatorV1Marker, "IDST"),
    (JoinControlV1Marker, "Join_C"),
    (LogicalOrderExceptionV1Marker, "LOE"),
    (LowercaseV1Marker, "Lower"),
    (MathV1Marker, "Math"),
    (NoncharacterCodePointV1Marker, "NChar"),
    (NfcInertV1Marker, "nfcinert"),
    (NfdInertV1Marker, "nfdinert"),
    (NfkcInertV1Marker, "nfkcinert"),
    (NfkdInertV1Marker, "nfkdinert"),
    (PatternSyntaxV1Marker, "Pat_Syn"),
    (PatternWhiteSpaceV1Marker, "Pat_WS"),
    (PrependedConcatenationMarkV1Marker, "PCM"),
    (PrintV1Marker, "print"),
    (QuotationMarkV1Marker, "QMark"),
    (RadicalV1Marker, "Radical"),
    (RegionalIndicatorV1Marker, "RI"),
    (SoftDottedV1Marker, "SD"),
    (SegmentStarterV1Marker, "segstart"),
    (CaseSensitiveV1Marker, "Sensitive"),
    (SentenceTerminalV1Marker, "STerm"),
    (TerminalPunctuationV1Marker, "Term"),
    (UnifiedIdeographV1Marker, "UIdeo"),
    (UppercaseV1Marker, "Upper"),
    (VariationSelectorV1Marker, "VS"),
    (WhiteSpaceV1Marker, "WSpace"),
    (XdigitV1Marker, "xdigit"),
    (XidContinueV1Marker, "XIDC"),
    (XidStartV1Marker, "XIDS")
);

#[test]
fn test_basic() {
    let provider = DatagenProvider::new_testing();

    let whitespace = icu::properties::sets::load_white_space(&provider).unwrap();
    let whitespace = whitespace.as_code_point_inversion_list().unwrap();

    assert!(whitespace.contains(' '));
    assert!(whitespace.contains('\n'));
    assert!(whitespace.contains('\u{3000}')); // U+3000 IDEOGRAPHIC SPACE

    assert!(!whitespace.contains('A'));
}
