package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface BlockLib: Library {
    fun icu4x_Block_for_char_mv1(ch: Int): Int
    fun icu4x_Block_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_Block_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_Block_to_integer_value_mv1(inner: Int): FFIUint16
    fun icu4x_Block_from_integer_value_mv1(other: FFIUint16): OptionInt
    fun icu4x_Block_try_from_str_mv1(s: Slice): OptionInt
}
/** See the [Rust documentation for `Block`](https://docs.rs/icu/2.3.1/icu/properties/props/struct.Block.html) for more information.
*/
enum class Block {
    NoBlock,
    BasicLatin,
    Latin1Supplement,
    LatinExtendedA,
    LatinExtendedB,
    IPAExtensions,
    SpacingModifierLetters,
    CombiningDiacriticalMarks,
    GreekAndCoptic,
    Cyrillic,
    Armenian,
    Hebrew,
    Arabic,
    Syriac,
    Thaana,
    Devanagari,
    Bengali,
    Gurmukhi,
    Gujarati,
    Oriya,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Sinhala,
    Thai,
    Lao,
    Tibetan,
    Myanmar,
    Georgian,
    HangulJamo,
    Ethiopic,
    Cherokee,
    UnifiedCanadianAboriginalSyllabics,
    Ogham,
    Runic,
    Khmer,
    Mongolian,
    LatinExtendedAdditional,
    GreekExtended,
    GeneralPunctuation,
    SuperscriptsAndSubscripts,
    CurrencySymbols,
    CombiningDiacriticalMarksForSymbols,
    LetterlikeSymbols,
    NumberForms,
    Arrows,
    MathematicalOperators,
    MiscellaneousTechnical,
    ControlPictures,
    OpticalCharacterRecognition,
    EnclosedAlphanumerics,
    BoxDrawing,
    BlockElements,
    GeometricShapes,
    MiscellaneousSymbols,
    Dingbats,
    BraillePatterns,
    CJKRadicalsSupplement,
    KangxiRadicals,
    IdeographicDescriptionCharacters,
    CJKSymbolsAndPunctuation,
    Hiragana,
    Katakana,
    Bopomofo,
    HangulCompatibilityJamo,
    Kanbun,
    BopomofoExtended,
    EnclosedCJKLettersAndMonths,
    CJKCompatibility,
    CJKUnifiedIdeographsExtensionA,
    CJKUnifiedIdeographs,
    YiSyllables,
    YiRadicals,
    HangulSyllables,
    HighSurrogates,
    HighPrivateUseSurrogates,
    LowSurrogates,
    PrivateUseArea,
    CJKCompatibilityIdeographs,
    AlphabeticPresentationForms,
    ArabicPresentationFormsA,
    CombiningHalfMarks,
    CJKCompatibilityForms,
    SmallFormVariants,
    ArabicPresentationFormsB,
    Specials,
    HalfwidthAndFullwidthForms,
    OldItalic,
    Gothic,
    Deseret,
    ByzantineMusicalSymbols,
    MusicalSymbols,
    MathematicalAlphanumericSymbols,
    CJKUnifiedIdeographsExtensionB,
    CJKCompatibilityIdeographsSupplement,
    Tags,
    CyrillicSupplement,
    Tagalog,
    Hanunoo,
    Buhid,
    Tagbanwa,
    MiscellaneousMathematicalSymbolsA,
    SupplementalArrowsA,
    SupplementalArrowsB,
    MiscellaneousMathematicalSymbolsB,
    SupplementalMathematicalOperators,
    KatakanaPhoneticExtensions,
    VariationSelectors,
    SupplementaryPrivateUseAreaA,
    SupplementaryPrivateUseAreaB,
    Limbu,
    TaiLe,
    KhmerSymbols,
    PhoneticExtensions,
    MiscellaneousSymbolsAndArrows,
    YijingHexagramSymbols,
    LinearBSyllabary,
    LinearBIdeograms,
    AegeanNumbers,
    Ugaritic,
    Shavian,
    Osmanya,
    CypriotSyllabary,
    TaiXuanJingSymbols,
    VariationSelectorsSupplement,
    AncientGreekMusicalNotation,
    AncientGreekNumbers,
    ArabicSupplement,
    Buginese,
    CJKStrokes,
    CombiningDiacriticalMarksSupplement,
    Coptic,
    EthiopicExtended,
    EthiopicSupplement,
    GeorgianSupplement,
    Glagolitic,
    Kharoshthi,
    ModifierToneLetters,
    NewTaiLue,
    OldPersian,
    PhoneticExtensionsSupplement,
    SupplementalPunctuation,
    SylotiNagri,
    Tifinagh,
    VerticalForms,
    NKo,
    Balinese,
    LatinExtendedC,
    LatinExtendedD,
    PhagsPa,
    Phoenician,
    Cuneiform,
    CuneiformNumbersAndPunctuation,
    CountingRodNumerals,
    Sundanese,
    Lepcha,
    OlChiki,
    CyrillicExtendedA,
    Vai,
    CyrillicExtendedB,
    Saurashtra,
    KayahLi,
    Rejang,
    Cham,
    AncientSymbols,
    PhaistosDisc,
    Lycian,
    Carian,
    Lydian,
    MahjongTiles,
    DominoTiles,
    Samaritan,
    UnifiedCanadianAboriginalSyllabicsExtended,
    TaiTham,
    VedicExtensions,
    Lisu,
    Bamum,
    CommonIndicNumberForms,
    DevanagariExtended,
    HangulJamoExtendedA,
    Javanese,
    MyanmarExtendedA,
    TaiViet,
    MeeteiMayek,
    HangulJamoExtendedB,
    ImperialAramaic,
    OldSouthArabian,
    Avestan,
    InscriptionalParthian,
    InscriptionalPahlavi,
    OldTurkic,
    RumiNumeralSymbols,
    Kaithi,
    EgyptianHieroglyphs,
    EnclosedAlphanumericSupplement,
    EnclosedIdeographicSupplement,
    CJKUnifiedIdeographsExtensionC,
    Mandaic,
    Batak,
    EthiopicExtendedA,
    Brahmi,
    BamumSupplement,
    KanaSupplement,
    PlayingCards,
    MiscellaneousSymbolsAndPictographs,
    Emoticons,
    TransportAndMapSymbols,
    AlchemicalSymbols,
    CJKUnifiedIdeographsExtensionD,
    ArabicExtendedA,
    ArabicMathematicalAlphabeticSymbols,
    Chakma,
    MeeteiMayekExtensions,
    MeroiticCursive,
    MeroiticHieroglyphs,
    Miao,
    Sharada,
    SoraSompeng,
    SundaneseSupplement,
    Takri,
    BassaVah,
    CaucasianAlbanian,
    CopticEpactNumbers,
    CombiningDiacriticalMarksExtended,
    Duployan,
    Elbasan,
    GeometricShapesExtended,
    Grantha,
    Khojki,
    Khudawadi,
    LatinExtendedE,
    LinearA,
    Mahajani,
    Manichaean,
    MendeKikakui,
    Modi,
    Mro,
    MyanmarExtendedB,
    Nabataean,
    OldNorthArabian,
    OldPermic,
    OrnamentalDingbats,
    PahawhHmong,
    Palmyrene,
    PauCinHau,
    PsalterPahlavi,
    ShorthandFormatControls,
    Siddham,
    SinhalaArchaicNumbers,
    SupplementalArrowsC,
    Tirhuta,
    WarangCiti,
    Ahom,
    AnatolianHieroglyphs,
    CherokeeSupplement,
    CJKUnifiedIdeographsExtensionE,
    EarlyDynasticCuneiform,
    Hatran,
    Multani,
    OldHungarian,
    SupplementalSymbolsAndPictographs,
    SuttonSignWriting,
    Adlam,
    Bhaiksuki,
    CyrillicExtendedC,
    GlagoliticSupplement,
    IdeographicSymbolsAndPunctuation,
    Marchen,
    MongolianSupplement,
    Newa,
    Osage,
    Tangut,
    TangutComponents,
    CJKUnifiedIdeographsExtensionF,
    KanaExtendedA,
    MasaramGondi,
    Nushu,
    Soyombo,
    SyriacSupplement,
    ZanabazarSquare,
    ChessSymbols,
    Dogra,
    GeorgianExtended,
    GunjalaGondi,
    HanifiRohingya,
    IndicSiyaqNumbers,
    Makasar,
    MayanNumerals,
    Medefaidrin,
    OldSogdian,
    Sogdian,
    EgyptianHieroglyphFormatControls,
    Elymaic,
    Nandinagari,
    NyiakengPuachueHmong,
    OttomanSiyaqNumbers,
    SmallKanaExtension,
    SymbolsAndPictographsExtendedA,
    TamilSupplement,
    Wancho,
    Chorasmian,
    CJKUnifiedIdeographsExtensionG,
    DivesAkuru,
    KhitanSmallScript,
    LisuSupplement,
    SymbolsForLegacyComputing,
    TangutSupplement,
    Yezidi,
    ArabicExtendedB,
    CyproMinoan,
    EthiopicExtendedB,
    KanaExtendedB,
    LatinExtendedF,
    LatinExtendedG,
    OldUyghur,
    Tangsa,
    Toto,
    UnifiedCanadianAboriginalSyllabicsExtendedA,
    Vithkuqi,
    ZnamennyMusicalNotation,
    ArabicExtendedC,
    CJKUnifiedIdeographsExtensionH,
    CyrillicExtendedD,
    DevanagariExtendedA,
    KaktovikNumerals,
    Kawi,
    NagMundari,
    CJKUnifiedIdeographsExtensionI,
    EgyptianHieroglyphsExtendedA,
    Garay,
    GurungKhema,
    KiratRai,
    MyanmarExtendedC,
    OlOnal,
    Sunuwar,
    SymbolsForLegacyComputingSupplement,
    Todhri,
    TuluTigalari,
    BeriaErfe,
    CJKUnifiedIdeographsExtensionJ,
    MiscellaneousSymbolsSupplement,
    SharadaSupplement,
    Sidetic,
    TaiYo,
    TangutComponentsSupplement,
    TolongSiki;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<BlockLib> = BlockLib::class.java
        internal val lib: BlockLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): Block {
            return Block.entries[native]
        }

        fun default(): Block {
            return NoBlock
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.3.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): Block {
            
            val returnVal = lib.icu4x_Block_for_char_mv1(ch);
            return (Block.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or `CodePointMapData`
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.3.1/icu/properties/props/struct.Block.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UShort): Block? {
            
            val returnVal = lib.icu4x_Block_from_integer_value_mv1(FFIUint16(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return Block.fromNative(intermediateOption)
        }
        @JvmStatic
        
        /** Creates a `Block` from a string.
        *
        *Short names, long names, and aliases are supported, and matching is case-insensitive.
        */
        fun tryFromStr(s: String): Block? {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_Block_try_from_str_mv1(sSliceMemory.slice);
            try {
                
                val intermediateOption = returnVal.option() ?: return null
                return Block.fromNative(intermediateOption)
            } finally {
                sSliceMemory.close()
            }
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.3.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_Block_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.3.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_Block_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and `CodePointMapData`
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.3.1/icu/properties/props/struct.Block.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UShort {
        
        val returnVal = lib.icu4x_Block_to_integer_value_mv1(this.toNative());
        return (returnVal.toUShort())
    }
}
