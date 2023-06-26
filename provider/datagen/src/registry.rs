// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

use icu_calendar::provider::*;
use icu_casemapping::provider::*;
use icu_collator::provider::*;
use icu_compactdecimal::provider::*;
use icu_datetime::provider::calendar::*;
use icu_datetime::provider::time_zones::*;
use icu_decimal::provider::*;
use icu_displaynames::provider::*;
use icu_list::provider::*;
use icu_locid_transform::provider::*;
use icu_normalizer::provider::*;
use icu_plurals::provider::*;
use icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker;
use icu_properties::provider::*;
use icu_provider::hello_world::HelloWorldV1Marker;
use icu_relativetime::provider::*;
use icu_segmenter::provider::*;
use icu_timezone::provider::*;

macro_rules! registry {
    ($($marker:ident,)+ # experimental # { $($exp_marker:ident,)+ }) => {
        /// List of all supported keys, except those that require the "experimental"
        /// feature on the `icu` crate.
        ///
        /// See [all_keys_with_experimental].
        // Excludes the hello world key, as that generally should not be generated.
        pub fn all_keys() -> Vec<DataKey> {
            vec![
                $(
                    <$marker>::KEY,
                )+
            ]
        }

        /// List of all supported keys, including those that require the "experimental"
        /// feature on the `icu` crate.
        ///
        /// See [all_keys].
        pub fn all_keys_with_experimental() -> Vec<DataKey> {
            vec![
                $(
                    <$marker>::KEY,
                )+
                $(
                    <$exp_marker>::KEY,
                )+
            ]
        }

        icu_provider::make_exportable_provider!(
            crate::DatagenProvider,
            [
                HelloWorldV1Marker,
                $($marker,)+
                $($exp_marker,)+
            ]
        );

        #[cfg(feature = "provider_baked")]
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
                if key == $exp_marker::KEY {
                    return $exp_marker.bake(env);
                }
            )+
            unreachable!("unregistered marker")
        }

        #[doc(hidden)]
        pub fn deserialize_and_discard<R>(key: DataKey, buf: DataPayload<BufferMarker>, r: impl Fn() -> R) -> Result<R, DataError> {
            $(
                if key == $marker::KEY {
                    let _reified_data: DataPayload<$marker> = buf.into_deserialized(icu_provider::buf::BufferFormat::Postcard1)?;
                    return Ok(r());
                }
            )+
            $(
                if key == $exp_marker::KEY {
                    let _reified_data: DataPayload<$exp_marker> = buf.into_deserialized(icu_provider::buf::BufferFormat::Postcard1)?;
                    return Ok(r());
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
    BidiClassNameToValueV1Marker,
    BidiClassValueToLongNameV1Marker,
    BidiClassValueToShortNameV1Marker,
    BidiControlV1Marker,
    BidiMirroredV1Marker,
    BidiAuxiliaryPropertiesV1Marker,
    BlankV1Marker,
    BuddhistDateLengthsV1Marker,
    BuddhistDateSymbolsV1Marker,
    CanonicalCombiningClassV1Marker,
    CanonicalCombiningClassNameToValueV1Marker,
    CanonicalCombiningClassValueToLongNameV1Marker,
    CanonicalCombiningClassValueToShortNameV1Marker,
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
    DictionaryForWordLineExtendedV1Marker,
    DictionaryForWordOnlyAutoV1Marker,
    EastAsianWidthV1Marker,
    EastAsianWidthNameToValueV1Marker,
    EastAsianWidthValueToLongNameV1Marker,
    EastAsianWidthValueToShortNameV1Marker,
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
    GeneralCategoryMaskNameToValueV1Marker,
    GeneralCategoryNameToValueV1Marker,
    GeneralCategoryValueToLongNameV1Marker,
    GeneralCategoryValueToShortNameV1Marker,
    GraphemeBaseV1Marker,
    GraphemeClusterBreakV1Marker,
    GraphemeClusterBreakDataV1Marker,
    GraphemeClusterBreakNameToValueV1Marker,
    GraphemeClusterBreakValueToLongNameV1Marker,
    GraphemeClusterBreakValueToShortNameV1Marker,
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
    LikelySubtagsExtendedV1Marker,
    LikelySubtagsForLanguageV1Marker,
    LikelySubtagsForScriptRegionV1Marker,
    LineBreakV1Marker,
    LineBreakDataV1Marker,
    LineBreakNameToValueV1Marker,
    LineBreakValueToLongNameV1Marker,
    LineBreakValueToShortNameV1Marker,
    LocaleFallbackLikelySubtagsV1Marker,
    LocaleFallbackParentsV1Marker,
    LogicalOrderExceptionV1Marker,
    LowercaseV1Marker,
    LstmForWordLineAutoV1Marker,
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
    ScriptNameToValueV1Marker,
    ScriptValueToLongNameV1Marker,
    ScriptValueToShortNameV1Marker,
    ScriptWithExtensionsPropertyV1Marker,
    SegmentStarterV1Marker,
    SentenceBreakV1Marker,
    SentenceBreakDataV1Marker,
    SentenceBreakNameToValueV1Marker,
    SentenceBreakValueToLongNameV1Marker,
    SentenceBreakValueToShortNameV1Marker,
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
    WordBreakDataV1Marker,
    WordBreakNameToValueV1Marker,
    WordBreakValueToLongNameV1Marker,
    WordBreakValueToShortNameV1Marker,
    XdigitV1Marker,
    XidContinueV1Marker,
    XidStartV1Marker,
    # experimental #
    {
        CaseMappingV1Marker,
        DateSkeletonPatternsV1Marker,
        RegionDisplayNamesV1Marker,
        LanguageDisplayNamesV1Marker,
        LocaleDisplayNamesV1Marker,
        ScriptDirectionV1Marker,
        ScriptDisplayNamesV1Marker,
        VariantDisplayNamesV1Marker,
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
    for key in all_keys_with_experimental() {
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
