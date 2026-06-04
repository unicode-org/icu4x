// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::casemap::options::TitlecaseOptions;
use icu::collections::codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder};
use icu::locale::LanguageIdentifier;
use icu::properties::props::BinaryProperty;
use icu::properties::{CodePointMapData, provider::*};
use icu_provider::prelude::*;
use std::collections::HashSet;

impl SourceDataProvider {
    pub(super) fn validate_property_name(
        &self,
        name: &str,
        short_name: &str,
    ) -> Result<(), DataError> {
        let sn = self
            .unicode()?
            .read_to_string("ucd/PropertyAliases.txt")?
            .lines()
            .filter_map(|l| Some(l.split('#').next().unwrap().trim()).filter(|l| !l.is_empty()))
            .find_map(|l| {
                let mut fields = l.split(';').map(str::trim);
                let sn = fields.next()?;
                let n = fields.next()?;
                if n == name { Some(sn) } else { None }
            });

        if let Some(sn) = sn
            && sn != short_name
        {
            return Err(DataError::custom("Property name mismatch")
                .with_display_context(name)
                .with_debug_context(&(sn, short_name)));
        }

        Ok(())
    }

    // get the source data for a Unicode binary property that only defines values for code points
    pub(super) fn get_binary_prop(
        &self,
        name: &str,
        short_name: &str,
    ) -> Result<CodePointInversionList<'static>, DataError> {
        let mut builder = CodePointInversionListBuilder::new();

        self.validate_property_name(name, short_name)?;

        let file = match name {
            "Alphabetic"
            | "Case_Ignorable"
            | "Cased"
            | "Changes_When_Casefolded"
            | "Changes_When_Casemapped"
            | "Changes_When_Lowercased"
            | "Changes_When_Titlecased"
            | "Changes_When_Uppercased"
            | "Default_Ignorable_Code_Point"
            | "Grapheme_Base"
            | "Grapheme_Extend"
            | "Grapheme_Link"
            | "ID_Continue"
            | "ID_Start"
            | "Lowercase"
            | "Math"
            | "Uppercase"
            | "XID_Continue"
            | "XID_Start" => "ucd/DerivedCoreProperties.txt",
            "Changes_When_NFKC_Casefolded" | "Full_Composition_Exclusion" => {
                "ucd/DerivedNormalizationProps.txt"
            }
            "Emoji_Component"
            | "Emoji_Modifier_Base"
            | "Emoji_Modifier"
            | "Emoji_Presentation"
            | "Emoji"
            | "Extended_Pictographic" => "ucd/emoji/emoji-data.txt",
            "Bidi_Mirrored" => "ucd/extracted/DerivedBinaryProperties.txt",
            _ => "ucd/PropList.txt",
        };

        for line in self.unicode()?.read_to_string(file)?.lines() {
            let line = line.split('#').next().unwrap().trim();
            if line.is_empty() {
                continue;
            }

            let mut parts = line.split(';').map(str::trim);
            let range = parts.next().unwrap();
            if parts.next() != Some(name) {
                continue;
            }

            let (a, b) = range.split_once("..").unwrap_or((range, range));
            let a = u32::from_str_radix(a, 16).unwrap();
            let b = u32::from_str_radix(b, 16).unwrap();

            builder.add_range32(a..=b);
        }

        Ok(builder.build())
    }
}

macro_rules! impl_unicode_property {
    ($(($prop:ty, $marker:ident)),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let data = self.get_binary_prop(
                        core::str::from_utf8(<$prop as BinaryProperty>::NAME).unwrap(),
                        core::str::from_utf8(<$prop as BinaryProperty>::SHORT_NAME).unwrap()
                    )?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(data))
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

impl_unicode_property!(
    (
        icu::properties::props::AsciiHexDigit,
        PropertyBinaryAsciiHexDigitV1
    ),
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
    (
        icu::properties::props::XidContinue,
        PropertyBinaryXidContinueV1
    ),
    (icu::properties::props::XidStart, PropertyBinaryXidStartV1)
);

macro_rules! impl_icu4c_property {
    ($(($prop:ty, $marker:ident)),+) => {
        $(
            #[allow(deprecated)]
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;

                    let name = core::str::from_utf8(<$prop as BinaryProperty>::NAME).unwrap();
                    let short_name = core::str::from_utf8(<$prop as BinaryProperty>::SHORT_NAME).unwrap();

                    let mut builder = CodePointInversionListBuilder::new();
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
                    for (start, end) in &data.ranges {
                        builder.add_range32(start..=end);
                    }

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(builder.build()))
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

impl DataProvider<PropertyBinarySegmentStarterV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyBinarySegmentStarterV1>, DataError> {
        self.check_req::<PropertyBinarySegmentStarterV1>(req)?;

        let decomposer = icu::normalizer::DecomposingNormalizer::try_new_nfd_unstable(&self)?;
        let decomposer = decomposer.as_borrowed();

        // ccc=0 and do not occur in non-initial position of the canonical decomposition of any character
        // https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/uchar_8h.html#ae40d616419e74ecc7c80a9febab03199a1200d63bfdb0379aa9cdbe8e14d71a26
        let non_initial_decomposition_characters = (0..(char::MAX as u32))
            .filter_map(char::from_u32)
            .flat_map(|cp| decomposer.normalize_iter([cp].into_iter()).skip(1))
            .map(|c| c as u32)
            .collect::<HashSet<_>>();

        let mut builder = CodePointInversionListBuilder::new();

        for range in CodePointMapData::try_new_unstable(&self)?
            .as_borrowed()
            .get_set_for_value(icu::properties::props::CanonicalCombiningClass::NotReordered)
            .as_borrowed()
            .iter_ranges()
        {
            for cp in range {
                if !non_initial_decomposition_characters.contains(&cp) {
                    builder.add32(cp);
                }
            }
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(builder.build())),
        })
    }
}

impl crate::IterableDataProviderCached<PropertyBinarySegmentStarterV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<PropertyBinaryCaseSensitiveV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyBinaryCaseSensitiveV1>, DataError> {
        self.check_req::<PropertyBinaryCaseSensitiveV1>(req)?;

        let mapper = icu::casemap::CaseMapper::try_new_unstable(&self)?;
        let mapper = mapper.as_borrowed();

        // Either the source of a case mapping or in the target of a case mapping.
        // https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/uchar_8h.html#ae40d616419e74ecc7c80a9febab03199ae3156debc89072569efeb31a468c3150
        let set = (0..(char::MAX as u32))
            .filter_map(char::from_u32)
            .flat_map(|cp| {
                let mut buf = [0; 4];
                let s = cp.encode_utf8(&mut buf);
                let lower = mapper.lowercase_to_string(s, &LanguageIdentifier::UNKNOWN);
                let upper = mapper.uppercase_to_string(s, &LanguageIdentifier::UNKNOWN);
                let title = mapper.titlecase_segment_with_only_case_data_to_string(
                    s,
                    &LanguageIdentifier::UNKNOWN,
                    {
                        let mut o = TitlecaseOptions::default();
                        o.leading_adjustment = Some(icu::casemap::options::LeadingAdjustment::None);
                        o
                    },
                );

                let lower = Some(&lower).filter(|l| *l != s);
                let upper = Some(&upper).filter(|l| *l != s);
                let title = Some(&title).filter(|l| *l != s);

                // Source
                (lower.is_some() || upper.is_some() || title.is_some())
                    .then_some(cp)
                    .into_iter()
                    // Target
                    .chain(
                        [lower, upper, title]
                            .into_iter()
                            .flatten()
                            .flat_map(|s| s.chars()),
                    )
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .collect::<HashSet<_>>();

        let mut builder = CodePointInversionListBuilder::new();
        for c in set {
            builder.add_char(c);
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(builder.build())),
        })
    }
}

impl crate::IterableDataProviderCached<PropertyBinaryCaseSensitiveV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl_icu4c_property!(
    (icu::properties::props::NfcInert, PropertyBinaryNfcInertV1),
    (icu::properties::props::NfdInert, PropertyBinaryNfdInertV1),
    (icu::properties::props::NfkcInert, PropertyBinaryNfkcInertV1),
    (icu::properties::props::NfkdInert, PropertyBinaryNfkdInertV1)
);

macro_rules! impl_posix_property {
    ($(($prop:ty, $marker:ident, $set_string:literal)),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;

                    #[cfg(not(feature = "unstable"))]
                    use icu::properties::unstable_unicodeset_parse::parse_unstable;
                    #[cfg(feature = "unstable")]
                    use icu::properties::unicodeset_parse::parse_unstable;

                    let set = parse_unstable($set_string, self)
                        .map_err(|e| e.fmt_with_source($set_string).to_string())
                        .unwrap()
                        .0
                        .code_points()
                        .clone();

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(set))
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

// UTS #18 Annex C Compatibility Properties
impl_posix_property!(
    (
        icu::properties::props::Alnum,
        PropertyBinaryAlnumV1,
        r#"[\p{Alphabetic}\p{gc=Decimal_Number}]"#
    ),
    (
        icu::properties::props::Blank,
        PropertyBinaryBlankV1,
        r#"[\p{gc=Space_Separator} \u0009]"#
    ),
    (
        icu::properties::props::Graph,
        PropertyBinaryGraphV1,
        r#"[^\p{White_Space}\p{gc=Control}\p{gc=Surrogate}\p{gc=Unassigned}]"#
    ),
    (
        icu::properties::props::Print,
        PropertyBinaryPrintV1,
        r#"[[^\p{White_Space}\p{gc=Control}\p{gc=Surrogate}\p{gc=Unassigned}] \p{gc=Space_Separator}]"#
    ),
    (
        icu::properties::props::Xdigit,
        PropertyBinaryXdigitV1,
        r#"[\p{gc=Decimal_Number}\p{Hex_Digit}]"#
    )
);

#[test]
fn test_basic() {
    use icu::properties::{CodePointSetData, props::WhiteSpace};

    let provider = SourceDataProvider::new_testing();

    let whitespace = CodePointSetData::try_new_unstable::<WhiteSpace>(&provider).unwrap();
    let whitespace = whitespace.as_code_point_inversion_list().unwrap();

    assert!(whitespace.contains(' '));
    assert!(whitespace.contains('\n'));
    assert!(whitespace.contains('\u{3000}')); // U+3000 IDEOGRAPHIC SPACE

    assert!(!whitespace.contains('A'));
}
