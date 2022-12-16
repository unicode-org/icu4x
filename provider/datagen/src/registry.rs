// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::{DataKey, KeyedDataMarker};

use icu_calendar::provider::*;
use icu_collator::provider::*;
use icu_datetime::provider::calendar::*;
use icu_datetime::provider::time_zones::*;
use icu_decimal::provider::*;
use icu_list::provider::*;
use icu_locid_transform::provider::*;
use icu_normalizer::provider::*;
use icu_plurals::provider::*;
use icu_properties::provider::*;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_provider_adapters::fallback::provider::*;
use icu_timezone::provider::*;

#[cfg(feature = "experimental")]
use icu_casemapping::provider::*;
#[cfg(feature = "experimental")]
use icu_compactdecimal::provider::*;
#[cfg(feature = "experimental")]
use icu_displaynames::provider::*;
#[cfg(feature = "experimental")]
use icu_relativetime::provider::*;
#[cfg(feature = "experimental")]
use icu_segmenter::provider::*;

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

        pub(crate) fn key_to_marker_bake(key: DataKey, env: &databake::CrateEnv) -> databake::TokenStream {
            use databake::Bake;
            // This is a bit naughty, we need the marker's type, but we're actually
            // baking its value. This works as long as all markers are unit structs.
            if key.path() == HelloWorldV1Marker::KEY.path() {
                return HelloWorldV1Marker.bake(env);
            }
            $(
                if key == $marker::KEY {
                    return $marker.bake(env);
                }
            )+
            $(
                #[cfg(feature = "experimental")]
                if key == $exp_marker::KEY {
                    return $exp_marker.bake(env);
                }
            )+
            unreachable!("unregistered marker")
        }
    }
}

registry!(
    AliasesV1Marker,
    AlnumV1Marker,
    AlphabeticV1Marker,
    AndListV1Marker,
    AsciiHexDigitV1Marker,
    BasicEmojiV1Marker,
    BidiClassV1Marker,
    BidiControlV1Marker,
    BidiMirroredV1Marker,
    BlankV1Marker,
    BuddhistDateLengthsV1Marker,
    BuddhistDateSymbolsV1Marker,
    CanonicalCombiningClassV1Marker,
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
    CollationFallbackSupplementV1Marker,
    CollationJamoV1Marker,
    CollationMetadataV1Marker,
    CollationReorderingV1Marker,
    CollationSpecialPrimariesV1Marker,
    CompatibilityDecompositionSupplementV1Marker,
    CompatibilityDecompositionTablesV1Marker,
    CopticDateLengthsV1Marker,
    CopticDateSymbolsV1Marker,
    DashV1Marker,
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
    EthiopianDateLengthsV1Marker,
    EthiopianDateSymbolsV1Marker,
    ExemplarCitiesV1Marker,
    ExemplarCharactersAuxiliaryV1Marker,
    ExemplarCharactersIndexV1Marker,
    ExemplarCharactersMainV1Marker,
    ExemplarCharactersNumbersV1Marker,
    ExemplarCharactersPunctuationV1Marker,
    ExtendedPictographicV1Marker,
    ExtenderV1Marker,
    FullCompositionExclusionV1Marker,
    GeneralCategoryV1Marker,
    GraphemeBaseV1Marker,
    GraphemeClusterBreakV1Marker,
    GraphemeExtendV1Marker,
    GraphemeLinkV1Marker,
    GraphV1Marker,
    GregorianDateLengthsV1Marker,
    GregorianDateSymbolsV1Marker,
    HexDigitV1Marker,
    HyphenV1Marker,
    IdContinueV1Marker,
    IdeographicV1Marker,
    IdsBinaryOperatorV1Marker,
    IdStartV1Marker,
    IdsTrinaryOperatorV1Marker,
    IndianDateLengthsV1Marker,
    IndianDateSymbolsV1Marker,
    JapaneseDateLengthsV1Marker,
    JapaneseDateSymbolsV1Marker,
    JapaneseErasV1Marker,
    JapaneseExtendedDateLengthsV1Marker,
    JapaneseExtendedDateSymbolsV1Marker,
    JapaneseExtendedErasV1Marker,
    JoinControlV1Marker,
    LikelySubtagsV1Marker,
    LineBreakV1Marker,
    LocaleFallbackLikelySubtagsV1Marker,
    LocaleFallbackParentsV1Marker,
    LogicalOrderExceptionV1Marker,
    LowercaseV1Marker,
    MathV1Marker,
    MetazoneGenericNamesLongV1Marker,
    MetazoneGenericNamesShortV1Marker,
    MetazonePeriodV1Marker,
    MetazoneSpecificNamesLongV1Marker,
    MetazoneSpecificNamesShortV1Marker,
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
    TimeLengthsV1Marker,
    TimeSymbolsV1Marker,
    TimeZoneFormatsV1Marker,
    UnifiedIdeographV1Marker,
    UnitListV1Marker,
    UppercaseV1Marker,
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
        DateSkeletonPatternsV1Marker,
        TerritoryDisplayNamesV1Marker,
        LanguageDisplayNamesV1Marker,
        GraphemeClusterBreakDataV1Marker,
        LineBreakDataV1Marker,
        LstmDataV1Marker,
        SentenceBreakDataV1Marker,
        UCharDictionaryBreakDataV1Marker,
        WordBreakDataV1Marker,
        LongSecondRelativeTimeFormatDataV1Marker,
        ShortSecondRelativeTimeFormatDataV1Marker,
        NarrowSecondRelativeTimeFormatDataV1Marker,
        LongMinuteRelativeTimeFormatDataV1Marker,
        ShortMinuteRelativeTimeFormatDataV1Marker,
        NarrowMinuteRelativeTimeFormatDataV1Marker,
        LongHourRelativeTimeFormatDataV1Marker,
        ShortHourRelativeTimeFormatDataV1Marker,
        NarrowHourRelativeTimeFormatDataV1Marker,
        LongDayRelativeTimeFormatDataV1Marker,
        ShortDayRelativeTimeFormatDataV1Marker,
        NarrowDayRelativeTimeFormatDataV1Marker,
        LongWeekRelativeTimeFormatDataV1Marker,
        ShortWeekRelativeTimeFormatDataV1Marker,
        NarrowWeekRelativeTimeFormatDataV1Marker,
        LongMonthRelativeTimeFormatDataV1Marker,
        ShortMonthRelativeTimeFormatDataV1Marker,
        NarrowMonthRelativeTimeFormatDataV1Marker,
        LongQuarterRelativeTimeFormatDataV1Marker,
        ShortQuarterRelativeTimeFormatDataV1Marker,
        NarrowQuarterRelativeTimeFormatDataV1Marker,
        LongYearRelativeTimeFormatDataV1Marker,
        ShortYearRelativeTimeFormatDataV1Marker,
        NarrowYearRelativeTimeFormatDataV1Marker,
        LongCompactDecimalFormatDataV1Marker,
        ShortCompactDecimalFormatDataV1Marker,
    }
);

#[test]
fn no_key_collisions() {
    let mut map = std::collections::BTreeMap::new();
    let mut failed = false;
    for key in all_keys() {
        if let Some(colliding_key) = map.insert(key.hashed(), key) {
            println!(
                "{:?} and {:?} collide at {:?}",
                key.path(),
                colliding_key.path(),
                key.hashed()
            );
            failed = true;
        }
    }
    if failed {
        panic!();
    }
}
