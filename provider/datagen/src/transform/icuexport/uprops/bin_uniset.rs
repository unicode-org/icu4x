// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceData;
use icu_properties::provider::*;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use icu_uniset::CodePointSetBuilder;

/// A data provider reading from TOML files produced by the ICU4C icuexportdata tool.
pub struct BinaryPropertyCodePointSetDataProvider {
    source: SourceData,
}

impl From<&SourceData> for BinaryPropertyCodePointSetDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

fn get_binary<'a>(
    source: &'a SourceData,
    key: &str,
) -> Result<&'a super::uprops_serde::binary::BinaryProperty, DataError> {
    source
        .icuexport()?
        .read_and_parse_toml::<super::uprops_serde::binary::Main>(&format!(
            "uprops/{}/{}.toml",
            source.trie_type(),
            key
        ))?
        .binary_property
        .get(0)
        .ok_or_else(|| DataErrorKind::MissingDataKey.into_error())
}

macro_rules! expand {
    ($(($marker:ident, $prop_name:literal)),+) => {
        $(
            impl DataProvider<$marker> for BinaryPropertyCodePointSetDataProvider {
                fn load_resource(
                    &self,
                    _: &DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    let data = get_binary(&self.source, $prop_name)?;

                    let mut builder = CodePointSetBuilder::new();
                    for (start, end) in &data.ranges {
                        builder.add_range_u32(&(start..=end));
                    }
                    let inv_list = builder.build();

                    Ok(DataResponse {
                        metadata: DataResponseMetadata::default(),
                        payload: Some(DataPayload::from_owned(
                            PropertyCodePointSetV1::InversionList(inv_list),
                        )),
                    })
                }
            }

            impl IterableDataProvider<$marker> for BinaryPropertyCodePointSetDataProvider {
                fn supported_options(
                    &self,
                ) -> Result<Vec<DataOptions>, DataError> {
                    get_binary(&self.source, $prop_name)?;

                    Ok(vec![Default::default()])
                }
            }
        )+

        icu_provider::make_exportable_provider!(BinaryPropertyCodePointSetDataProvider, [$($marker),+,]);
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
    use icu_properties::provider::PropertyCodePointSetV1;
    use icu_properties::provider::WhiteSpaceV1Marker;
    use icu_uniset::CodePointSet;

    let provider = BinaryPropertyCodePointSetDataProvider::from(&SourceData::for_test());

    let payload: DataPayload<WhiteSpaceV1Marker> = provider
        .load_resource(&DataRequest::default())
        .and_then(DataResponse::take_payload)
        .expect("Loading was successful");

    let whitespace: &CodePointSet = match payload.get() {
        PropertyCodePointSetV1::InversionList(ref l) => l,
        _ => unreachable!("Should have serialized to an inversion list"),
    };

    assert!(whitespace.contains(' '));
    assert!(whitespace.contains('\n'));
    assert!(whitespace.contains('\u{3000}')); // U+3000 IDEOGRAPHIC SPACE

    assert!(!whitespace.contains('A'));
}
