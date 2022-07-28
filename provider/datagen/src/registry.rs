// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataKey, KeyedDataMarker};

use icu_calendar::provider::*;
use icu_collator::provider::*;
use icu_datetime::provider::calendar::*;
use icu_datetime::provider::time_zones::*;
use icu_datetime::provider::week_data::*;
use icu_decimal::provider::*;
use icu_list::provider::*;
use icu_locale_canonicalizer::provider::*;
use icu_normalizer::provider::*;
use icu_plurals::provider::*;
use icu_properties::provider::*;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider_adapters::fallback::provider::*;
use icu_timezone::provider::*;

#[cfg(feature = "experimental")]
use icu_casemapping::provider::*;
#[cfg(feature = "experimental")]
use icu_segmenter::*;

macro_rules! registry {
    ($($marker:ident,)+ #[cfg(feature = "experimental")] { $($exp_marker:ident,)+ }) => {
        /// List of all supported keys
        pub fn all_keys() -> Vec<DataKey> {
            vec![
                $(
                    <$marker>::KEY,
                )+
                $(
                    #[cfg(feature = "experimental")]
                    <$exp_marker>::KEY,
                )+
            ]
        }

        #[cfg(feature = "experimental")]
        icu_provider::make_exportable_provider!(
            crate::DatagenProvider,
            [
                HelloWorldV1Marker,
                $($marker,)+
                $($exp_marker,)+
            ]
        );

        #[cfg(not(feature = "experimental"))]
        icu_provider::make_exportable_provider!(
            crate::DatagenProvider,
            [
                HelloWorldV1Marker,
                $($marker,)+
            ]
        );
    }
}

registry!(
    AliasesV1Marker,
    AlnumV1Marker,
    AlphabeticV1Marker,
    AndListV1Marker,
    AsciiHexDigitV1Marker,
    BidiClassV1Marker,
    BidiControlV1Marker,
    BidiMirroredV1Marker,
    BlankV1Marker,
    CanonicalCombiningClassV1Marker,
    CanonicalCompositionPassthroughV1Marker,
    CanonicalCompositionsV1Marker,
    CanonicalDecompositionDataV1Marker,
    CanonicalDecompositionTablesV1Marker,
    CardinalV1Marker,
    CasedV1Marker,
    CaseIgnorableV1Marker,
    CaseSensitiveV1Marker,
    ChangesWhenCasefoldedV1Marker,
    ChangesWhenCasemappedV1Marker,
    ChangesWhenLowercasedV1Marker,
    ChangesWhenNfkcCasefoldedV1Marker,
    ChangesWhenTitlecasedV1Marker,
    ChangesWhenUppercasedV1Marker,
    CollationDataV1Marker,
    CollationDiacriticsV1Marker,
    CollationJamoV1Marker,
    CollationMetadataV1Marker,
    CollationReorderingV1Marker,
    CollationSpecialPrimariesV1Marker,
    CompatibilityCompositionPassthroughV1Marker,
    CompatibilityDecompositionSupplementV1Marker,
    CompatibilityDecompositionTablesV1Marker,
    DashV1Marker,
    DatePatternsV1Marker,
    DateSkeletonPatternsV1Marker,
    DateSymbolsV1Marker,
    DecimalSymbolsV1Marker,
    DefaultIgnorableCodePointV1Marker,
    DeprecatedV1Marker,
    DiacriticV1Marker,
    EastAsianWidthV1Marker,
    EmojiComponentV1Marker,
    EmojiModifierBaseV1Marker,
    EmojiModifierV1Marker,
    EmojiPresentationV1Marker,
    EmojiV1Marker,
    ExemplarCitiesV1Marker,
    ExtendedPictographicV1Marker,
    ExtenderV1Marker,
    FullCompositionExclusionV1Marker,
    GeneralCategoryV1Marker,
    GraphemeBaseV1Marker,
    GraphemeClusterBreakV1Marker,
    GraphemeExtendV1Marker,
    GraphemeLinkV1Marker,
    GraphV1Marker,
    HexDigitV1Marker,
    HyphenV1Marker,
    IdContinueV1Marker,
    IdeographicV1Marker,
    IdsBinaryOperatorV1Marker,
    IdStartV1Marker,
    IdsTrinaryOperatorV1Marker,
    JapaneseErasV1Marker,
    JoinControlV1Marker,
    LikelySubtagsV1Marker,
    LineBreakV1Marker,
    LocaleFallbackLikelySubtagsV1Marker,
    LocaleFallbackParentsV1Marker,
    LogicalOrderExceptionV1Marker,
    LowercaseV1Marker,
    MathV1Marker,
    MetaZoneGenericNamesLongV1Marker,
    MetaZoneGenericNamesShortV1Marker,
    MetaZoneSpecificNamesLongV1Marker,
    MetaZoneSpecificNamesShortV1Marker,
        MetaZonePeriodV1Marker,
    NfcInertV1Marker,
    NfdInertV1Marker,
    NfkcInertV1Marker,
    NfkdInertV1Marker,
    NoncharacterCodePointV1Marker,
    NonRecursiveDecompositionSupplementV1Marker,
    OrdinalV1Marker,
    OrListV1Marker,
    PatternSyntaxV1Marker,
    PatternWhiteSpaceV1Marker,
    PrependedConcatenationMarkV1Marker,
    PrintV1Marker,
    QuotationMarkV1Marker,
    RadicalV1Marker,
    RegionalIndicatorV1Marker,
    ScriptV1Marker,
    ScriptWithExtensionsPropertyV1Marker,
    SegmentStarterV1Marker,
    SentenceBreakV1Marker,
    SentenceTerminalV1Marker,
    SoftDottedV1Marker,
    TerminalPunctuationV1Marker,
    TimePatternsV1Marker,
    TimeSymbolsV1Marker,
    TimeZoneFormatsV1Marker,
    UnifiedIdeographV1Marker,
    UnitListV1Marker,
    UppercaseV1Marker,
    Uts46CompositionPassthroughV1Marker,
    Uts46DecompositionSupplementV1Marker,
    VariationSelectorV1Marker,
    WeekDataV1Marker,
    WhiteSpaceV1Marker,
    WordBreakV1Marker,
    XdigitV1Marker,
    XidContinueV1Marker,
    XidStartV1Marker,
    #[cfg(feature = "experimental")]
    {
        CaseMappingV1Marker,
        GraphemeClusterBreakDataV1Marker,
        LineBreakDataV1Marker,
        LstmDataV1Marker,
        SentenceBreakDataV1Marker,
        UCharDictionaryBreakDataV1Marker,
        WordBreakDataV1Marker,
    }
);

#[test]
fn no_key_collisions() {
    let mut map = std::collections::BTreeMap::new();
    let mut failed = false;
    for key in all_keys() {
        if let Some(colliding_key) = map.insert(key.get_hash(), key) {
            println!(
                "{:?} and {:?} collide at {:?}",
                key.get_path(),
                colliding_key.get_path(),
                key.get_hash()
            );
            failed = true;
        }
    }
    if failed {
        panic!();
    }
}
