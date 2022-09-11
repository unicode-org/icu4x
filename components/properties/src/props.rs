// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of enums for enumerated properties.

use open_enum::open_enum;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Selection constants for Unicode properties.
/// These constants are used to select one of the Unicode properties.
/// See `UProperty` in ICU4C.
#[allow(dead_code)] // Not currently used but seems like it could be useful
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
#[repr(i32)]
enum EnumeratedProperty {
    /// The Bidi_Class property.
    BidiClass = 0x1000,
    /// The Canonical_Combining_Class property.
    CanonicalCombiningClass = 0x1002,
    /// The East_Asian_Width property. See [`EastAsianWidth`].
    EastAsianWidth = 0x1004,
    /// The General_Category property.
    GeneralCategory = 0x1005,
    /// A pseudo-property that is used to represent groupings of `GeneralCategory`.
    GeneralCategoryGroup = 0x2000,
    /// The Line_Break property. See [`LineBreak`].
    LineBreak = 0x1008,
    /// The Script property. See [`Script`].
    Script = 0x100A,
    /// The Grapheme_Cluster_Break property. See [`GraphemeClusterBreak`].
    GraphemeClusterBreak = 0x1012,
    /// The Sentence_Break property. See [`SentenceBreak`].
    SentenceBreak = 0x1013,
    /// The Word_Break property. See [`WordBreak`].
    WordBreak = 0x1014,
    /// The Script_Extensions property. See [`Script`].
    ScriptExtensions = 0x7000, // TODO(#1160) - this is a Miscellaneous property, not Enumerated
    /// Represents an invalid or unknown Unicode property.
    InvalidCode = -1, // TODO(#1160) - taken from ICU4C UProperty::UCHAR_INVALID_CODE
}

/// Enumerated property Bidi_Class
///
/// These are the categories required by the Unicode Bidirectional Algorithm.
/// For the property values, see [Bidirectional Class Values](https://unicode.org/reports/tr44/#Bidi_Class_Values).
/// For more information, see [Unicode Standard Annex #9](https://unicode.org/reports/tr41/tr41-28.html#UAX9).
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(BidiClassULE)]
pub enum BidiClass {
    /// (`L`) any strong left-to-right character
    LeftToRight,
    /// (`R`) any strong right-to-left (non-Arabic-type) character
    RightToLeft,
    /// (`EN`) any ASCII digit or Eastern Arabic-Indic digit
    EuropeanNumber,
    /// (`ES`) plus and minus signs
    EuropeanSeparator,
    /// (`ET`) a terminator in a numeric format context, includes currency signs
    EuropeanTerminator,
    /// (`AN`) any Arabic-Indic digit
    ArabicNumber,
    /// (`CS`) commas, colons, and slashes
    CommonSeparator,
    /// (`B`) various newline characters
    ParagraphSeparator,
    /// (`S`) various segment-related control codes
    SegmentSeparator,
    /// (`WS`) spaces
    WhiteSpace,
    /// (`ON`) most other symbols and punctuation marks
    OtherNeutral,
    /// (`LRE`) U+202A: the LR embedding control
    LeftToRightEmbedding,
    /// (`LRO`) U+202D: the LR override control
    LeftToRightOverride,
    /// (`AL`) any strong right-to-left (Arabic-type) character
    ArabicLetter,
    /// (`RLE`) U+202B: the RL embedding control
    RightToLeftEmbedding,
    /// (`RLO`) U+202E: the RL override control
    RightToLeftOverride,
    /// (`PDF`) U+202C: terminates an embedding or override control
    PopDirectionalFormat,
    /// (`NSM`) any nonspacing mark
    NonspacingMark,
    /// (`BN`) most format characters, control codes, or noncharacters
    BoundaryNeutral,
    /// (`FSI`) U+2068: the first strong isolate control
    FirstStrongIsolate,
    /// (`LRI`) U+2066: the LR isolate control
    LeftToRightIsolate,
    /// (`RLI`) U+2067: the RL isolate control
    RightToLeftIsolate,
    /// (`PDI`) U+2069: terminates an isolate control
    PopDirectionalIsolate,
}

/// Enumerated property General_Category.
///
/// General_Category specifies the most general classification of a code point, usually
/// determined based on the primary characteristic of the assigned character. For example, is the
/// character a letter, a mark, a number, punctuation, or a symbol, and if so, of what type?
///
/// GeneralCategory only supports specific subcategories (eg `UppercaseLetter`).
/// It does not support grouped categories (eg `Letter`). For grouped categories, use [`GeneralCategoryGroup`].
#[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[allow(clippy::exhaustive_enums)] // this type is stable
#[zerovec::make_ule(GeneralCategoryULE)]
#[repr(u8)]
pub enum GeneralCategory {
    /// (`Cn`) A reserved unassigned code point or a noncharacter
    Unassigned = 0,

    /// (`Lu`) An uppercase letter
    UppercaseLetter = 1,
    /// (`Ll`) A lowercase letter
    LowercaseLetter = 2,
    /// (`Lt`) A digraphic letter, with first part uppercase
    TitlecaseLetter = 3,
    /// (`Lm`) A modifier letter
    ModifierLetter = 4,
    /// (`Lo`) Other letters, including syllables and ideographs
    OtherLetter = 5,

    /// (`Mn`) A nonspacing combining mark (zero advance width)
    NonspacingMark = 6,
    /// (`Mc`) A spacing combining mark (positive advance width)
    SpacingMark = 8,
    /// (`Me`) An enclosing combining mark
    EnclosingMark = 7,

    /// (`Nd`) A decimal digit
    DecimalNumber = 9,
    /// (`Nl`) A letterlike numeric character
    LetterNumber = 10,
    /// (`No`) A numeric character of other type
    OtherNumber = 11,

    /// (`Zs`) A space character (of various non-zero widths)
    SpaceSeparator = 12,
    /// (`Zl`) U+2028 LINE SEPARATOR only
    LineSeparator = 13,
    /// (`Zp`) U+2029 PARAGRAPH SEPARATOR only
    ParagraphSeparator = 14,

    /// (`Cc`) A C0 or C1 control code
    Control = 15,
    /// (`Cf`) A format control character
    Format = 16,
    /// (`Co`) A private-use character
    PrivateUse = 17,
    /// (`Cs`) A surrogate code point
    Surrogate = 18,

    /// (`Pd`) A dash or hyphen punctuation mark
    DashPunctuation = 19,
    /// (`Ps`) An opening punctuation mark (of a pair)
    OpenPunctuation = 20,
    /// (`Pe`) A closing punctuation mark (of a pair)
    ClosePunctuation = 21,
    /// (`Pc`) A connecting punctuation mark, like a tie
    ConnectorPunctuation = 22,
    /// (`Pi`) An initial quotation mark
    InitialPunctuation = 28,
    /// (`Pf`) A final quotation mark
    FinalPunctuation = 29,
    /// (`Po`) A punctuation mark of other type
    OtherPunctuation = 23,

    /// (`Sm`) A symbol of mathematical use
    MathSymbol = 24,
    /// (`Sc`) A currency sign
    CurrencySymbol = 25,
    /// (`Sk`) A non-letterlike modifier symbol
    ModifierSymbol = 26,
    /// (`So`) A symbol of other type
    OtherSymbol = 27,
}

use GeneralCategory as GC;

/// Groupings of multiple General_Category property values.
///
/// Instances of `GeneralCategoryGroup` represent the defined multi-category
/// values that are useful for users in certain contexts, such as regex. In
/// other words, unlike [`GeneralCategory`], this supports groups of general
/// categories: for example, `Letter` /// is the union of `UppercaseLetter`,
/// `LowercaseLetter`, etc.
///
/// See <https://www.unicode.org/reports/tr44/> .
///
/// The discriminants correspond to the `U_GC_XX_MASK` constants in ICU4C.
/// Unlike [`GeneralCategory`], this supports groups of general categories: for example, `Letter`
/// is the union of `UppercaseLetter`, `LowercaseLetter`, etc.
///
/// See `UCharCategory` and `U_GET_GC_MASK` in ICU4C.
#[open_enum(inner_vis = pub(crate))]
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
#[repr(u32)]
pub enum GeneralCategoryGroup {
    /// (`Lu`) An uppercase letter
    UppercaseLetter = 1 << (GC::UppercaseLetter as u32),
    /// (`Ll`) A lowercase letter
    LowercaseLetter = 1 << (GC::LowercaseLetter as u32),
    /// (`Lt`) A digraphic letter, with first part uppercase
    TitlecaseLetter = 1 << (GC::TitlecaseLetter as u32),
    /// (`Lm`) A modifier letter
    ModifierLetter = 1 << (GC::ModifierLetter as u32),
    /// (`Lo`) Other letters, including syllables and ideographs
    OtherLetter = 1 << (GC::OtherLetter as u32),
    /// (`LC`) The union of UppercaseLetter, LowercaseLetter, and TitlecaseLetter
    CasedLetter = 1 << (GC::UppercaseLetter as u32)
        | 1 << (GC::LowercaseLetter as u32)
        | 1 << (GC::TitlecaseLetter as u32),
    /// (`L`) The union of all letter categories
    Letter = 1 << (GC::UppercaseLetter as u32)
        | 1 << (GC::LowercaseLetter as u32)
        | 1 << (GC::TitlecaseLetter as u32)
        | 1 << (GC::ModifierLetter as u32)
        | 1 << (GC::OtherLetter as u32),

    /// (`Mn`) A nonspacing combining mark (zero advance width)
    NonspacingMark = 1 << (GC::NonspacingMark as u32),
    /// (`Mc`) A spacing combining mark (positive advance width)
    EnclosingMark = 1 << (GC::EnclosingMark as u32),
    /// (`Me`) An enclosing combining mark
    SpacingMark = 1 << (GC::SpacingMark as u32),
    /// (`M`) The union of all mark categories
    Mark = 1 << (GC::NonspacingMark as u32)
        | 1 << (GC::EnclosingMark as u32)
        | 1 << (GC::SpacingMark as u32),

    /// (`Nd`) A decimal digit
    DecimalNumber = 1 << (GC::DecimalNumber as u32),
    /// (`Nl`) A letterlike numeric character
    LetterNumber = 1 << (GC::LetterNumber as u32),
    /// (`No`) A numeric character of other type
    OtherNumber = 1 << (GC::OtherNumber as u32),
    /// (`N`) The union of all number categories
    Number = 1 << (GC::DecimalNumber as u32)
        | 1 << (GC::LetterNumber as u32)
        | 1 << (GC::OtherNumber as u32),

    /// (`Zs`) A space character (of various non-zero widths)
    SpaceSeparator = 1 << (GC::SpaceSeparator as u32),
    /// (`Zl`) U+2028 LINE SEPARATOR only
    LineSeparator = 1 << (GC::LineSeparator as u32),
    /// (`Zp`) U+2029 PARAGRAPH SEPARATOR only
    ParagraphSeparator = 1 << (GC::ParagraphSeparator as u32),
    /// (`Z`) The union of all separator categories
    Separator = 1 << (GC::SpaceSeparator as u32)
        | 1 << (GC::LineSeparator as u32)
        | 1 << (GC::ParagraphSeparator as u32),

    /// (`Cc`) A C0 or C1 control code
    Control = 1 << (GC::Control as u32),
    /// (`Cf`) A format control character
    Format = 1 << (GC::Format as u32),
    /// (`Co`) A private-use character
    PrivateUse = 1 << (GC::PrivateUse as u32),
    /// (`Cs`) A surrogate code point
    Surrogate = 1 << (GC::Surrogate as u32),
    /// (`Cn`) A reserved unassigned code point or a noncharacter
    Unassigned = 1 << (GC::Unassigned as u32),
    /// (`C`) The union of all control code, reserved, and unassigned categories
    Other = 1 << (GC::Control as u32)
        | 1 << (GC::Format as u32)
        | 1 << (GC::PrivateUse as u32)
        | 1 << (GC::Surrogate as u32)
        | 1 << (GC::Unassigned as u32),

    /// (`Pd`) A dash or hyphen punctuation mark
    DashPunctuation = 1 << (GC::DashPunctuation as u32),
    /// (`Ps`) An opening punctuation mark (of a pair)
    OpenPunctuation = 1 << (GC::OpenPunctuation as u32),
    /// (`Pe`) A closing punctuation mark (of a pair)
    ClosePunctuation = 1 << (GC::ClosePunctuation as u32),
    /// (`Pc`) A connecting punctuation mark, like a tie
    ConnectorPunctuation = 1 << (GC::ConnectorPunctuation as u32),
    /// (`Pi`) An initial quotation mark
    InitialPunctuation = 1 << (GC::InitialPunctuation as u32),
    /// (`Pf`) A final quotation mark
    FinalPunctuation = 1 << (GC::FinalPunctuation as u32),
    /// (`Po`) A punctuation mark of other type
    OtherPunctuation = 1 << (GC::OtherPunctuation as u32),
    /// (`P`) The union of all punctuation categories
    Punctuation = 1 << (GC::DashPunctuation as u32)
        | 1 << (GC::OpenPunctuation as u32)
        | 1 << (GC::ClosePunctuation as u32)
        | 1 << (GC::ConnectorPunctuation as u32)
        | 1 << (GC::OtherPunctuation as u32)
        | 1 << (GC::InitialPunctuation as u32)
        | 1 << (GC::FinalPunctuation as u32),

    /// (`Sm`) A symbol of mathematical use
    MathSymbol = 1 << (GC::MathSymbol as u32),
    /// (`Sc`) A currency sign
    CurrencySymbol = 1 << (GC::CurrencySymbol as u32),
    /// (`Sk`) A non-letterlike modifier symbol
    ModifierSymbol = 1 << (GC::ModifierSymbol as u32),
    /// (`So`) A symbol of other type
    OtherSymbol = 1 << (GC::OtherSymbol as u32),
    /// (`S`) The union of all symbol categories
    Symbol = 1 << (GC::MathSymbol as u32)
        | 1 << (GC::CurrencySymbol as u32)
        | 1 << (GC::ModifierSymbol as u32)
        | 1 << (GC::OtherSymbol as u32),
}

impl GeneralCategoryGroup {
    /// Return whether the code point belongs in the provided multi-value category.
    ///
    /// ```
    /// use icu::properties::{maps, GeneralCategory, GeneralCategoryGroup};
    /// use icu_collections::codepointtrie::CodePointTrie;
    ///
    /// let data = maps::load_general_category(&icu_testdata::unstable())
    ///     .expect("The data should be valid");
    /// let gc = data.as_borrowed();
    ///
    /// assert_eq!(gc.get('A'), GeneralCategory::UppercaseLetter);
    /// assert!(GeneralCategoryGroup::CasedLetter.contains(gc.get('A')));
    ///
    /// // U+0B1E ORIYA LETTER NYA
    /// assert_eq!(gc.get('ଞ'), GeneralCategory::OtherLetter);
    /// assert!(GeneralCategoryGroup::Letter.contains(gc.get('ଞ')));
    /// assert!(!GeneralCategoryGroup::CasedLetter.contains(gc.get('ଞ')));
    ///
    /// // U+0301 COMBINING ACUTE ACCENT
    /// assert_eq!(gc.get32(0x0301), GeneralCategory::NonspacingMark);
    /// assert!(GeneralCategoryGroup::Mark.contains(gc.get32(0x0301)));
    /// assert!(!GeneralCategoryGroup::Letter.contains(gc.get32(0x0301)));
    ///
    /// assert_eq!(gc.get('0'), GeneralCategory::DecimalNumber);
    /// assert!(GeneralCategoryGroup::Number.contains(gc.get('0')));
    /// assert!(!GeneralCategoryGroup::Mark.contains(gc.get('0')));
    ///
    /// assert_eq!(gc.get('('), GeneralCategory::OpenPunctuation);
    /// assert!(GeneralCategoryGroup::Punctuation.contains(gc.get('(')));
    /// assert!(!GeneralCategoryGroup::Number.contains(gc.get('(')));
    ///
    /// // U+2713 CHECK MARK
    /// assert_eq!(gc.get('✓'), GeneralCategory::OtherSymbol);
    /// assert!(GeneralCategoryGroup::Symbol.contains(gc.get('✓')));
    /// assert!(!GeneralCategoryGroup::Punctuation.contains(gc.get('✓')));
    ///
    /// assert_eq!(gc.get(' '), GeneralCategory::SpaceSeparator);
    /// assert!(GeneralCategoryGroup::Separator.contains(gc.get(' ')));
    /// assert!(!GeneralCategoryGroup::Symbol.contains(gc.get(' ')));
    ///
    /// // U+E007F CANCEL TAG
    /// assert_eq!(gc.get32(0xE007F), GeneralCategory::Format);
    /// assert!(GeneralCategoryGroup::Other.contains(gc.get32(0xE007F)));
    /// assert!(!GeneralCategoryGroup::Separator.contains(gc.get32(0xE007F)));
    /// ```
    pub fn contains(&self, val: GeneralCategory) -> bool {
        0 != (1 << (val as u32)) & self.0
    }
}

impl From<GeneralCategory> for GeneralCategoryGroup {
    fn from(subcategory: GeneralCategory) -> Self {
        GeneralCategoryGroup(1 << (subcategory as u32))
    }
}
impl From<u32> for GeneralCategoryGroup {
    fn from(mask: u32) -> Self {
        GeneralCategoryGroup(mask)
    }
}
/// Enumerated property Script.
///
/// This is used with both the Script and Script_Extensions Unicode properties.
/// Each character is assigned a single Script, but characters that are used in
/// a particular subset of scripts will be in more than one Script_Extensions set.
/// For example, DEVANAGARI DIGIT NINE has Script=Devanagari, but is also in the
/// Script_Extensions set for Dogra, Kaithi, and Mahajani.
///
/// For more information, see UAX #24: <http://www.unicode.org/reports/tr24/>.
/// See `UScriptCode` in ICU4C.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[allow(missing_docs)] // These constants don't need individual documentation.
#[repr(u16)]
#[zerovec::make_ule(ScriptULE)]
pub enum Script {
    Adlam = 167,
    Ahom = 161,
    AnatolianHieroglyphs = 156,
    Arabic = 2,
    Armenian = 3,
    Avestan = 117,
    Balinese = 62,
    Bamum = 130,
    BassaVah = 134,
    Batak = 63,
    Bengali = 4,
    Bhaiksuki = 168,
    Bopomofo = 5,
    Brahmi = 65,
    Braille = 46,
    Buginese = 55,
    Buhid = 44,
    CanadianAboriginal = 40,
    Carian = 104,
    CaucasianAlbanian = 159,
    Chakma = 118,
    Cham = 66,
    Cherokee = 6,
    Chorasmian = 189,
    Common = 0,
    Coptic = 7,
    Cuneiform = 101,
    Cypriot = 47,
    CyproMinoan = 193,
    Cyrillic = 8,
    Deseret = 9,
    Devanagari = 10,
    DivesAkuru = 190,
    Dogra = 178,
    Duployan = 135,
    EgyptianHieroglyphs = 71,
    Elbasan = 136,
    Elymaic = 185,
    Ethiopian = 11,
    Georgian = 12,
    Glagolitic = 56,
    Gothic = 13,
    Grantha = 137,
    Greek = 14,
    Gujarati = 15,
    GunjalaGondi = 179,
    Gurmukhi = 16,
    Han = 17,
    Hangul = 18,
    HanifiRohingya = 182,
    Hanunoo = 43,
    Hatran = 162,
    Hebrew = 19,
    Hiragana = 20,
    ImperialAramaic = 116,
    Inherited = 1,
    InscriptionalPahlavi = 122,
    InscriptionalParthian = 125,
    Javanese = 78,
    Kaithi = 120,
    Kannada = 21,
    Katakana = 22,
    KayahLi = 79,
    Kharoshthi = 57,
    KhitanSmallScript = 191,
    Khmer = 23,
    Khojki = 157,
    Khudawadi = 145,
    Lao = 24,
    Latin = 25,
    Lepcha = 82,
    Limbu = 48,
    LinearA = 83,
    LinearB = 49,
    Lisu = 131,
    Lycian = 107,
    Lydian = 108,
    Mahajani = 160,
    Makasar = 180,
    Malayalam = 26,
    Mandaic = 84,
    Manichaean = 121,
    Marchen = 169,
    MasaramGondi = 175,
    Medefaidrin = 181,
    MeeteiMayek = 115,
    MendeKikakui = 140,
    MeroiticCursive = 141,
    MeroiticHieroglyphs = 86,
    Miao = 92,
    Modi = 163,
    Mongolian = 27,
    Mro = 149,
    Multani = 164,
    Myanmar = 28,
    Nabataean = 143,
    Nandinagari = 187,
    NewTaiLue = 59,
    Newa = 170,
    Nko = 87,
    Nushu = 150,
    NyiakengPuachueHmong = 186,
    Ogham = 29,
    OlChiki = 109,
    OldHungarian = 76,
    OldItalic = 30,
    OldNorthArabian = 142,
    OldPermic = 89,
    OldPersian = 61,
    OldSogdian = 184,
    OldSouthArabian = 133,
    OldTurkic = 88,
    OldUyghur = 194,
    Oriya = 31,
    Osage = 171,
    Osmanya = 50,
    PahawhHmong = 75,
    Palmyrene = 144,
    PauCinHau = 165,
    PhagsPa = 90,
    Phoenician = 91,
    PsalterPahlavi = 123,
    Rejang = 110,
    Runic = 32,
    Samaritan = 126,
    Saurashtra = 111,
    Sharada = 151,
    Shavian = 51,
    Siddham = 166,
    SignWriting = 112,
    Sinhala = 33,
    Sogdian = 183,
    SoraSompeng = 152,
    Soyombo = 176,
    Sundanese = 113,
    SylotiNagri = 58,
    Syriac = 34,
    Tagalog = 42,
    Tagbanwa = 45,
    TaiLe = 52,
    TaiTham = 106,
    TaiViet = 127,
    Takri = 153,
    Tamil = 35,
    Tangsa = 195,
    Tangut = 154,
    Telugu = 36,
    Thaana = 37,
    Thai = 38,
    Tibetan = 39,
    Tifinagh = 60,
    Tirhuta = 158,
    Toto = 196,
    Ugaritic = 53,
    Unknown = 103,
    Vai = 99,
    Vithkuqi = 197,
    Wancho = 188,
    WarangCiti = 146,
    Yezidi = 192,
    Yi = 41,
    ZanabazarSquare = 177,
}

/// Enumerated property East_Asian_Width.
///
/// See "Definition" in UAX #11 for the summary of each property value:
/// <https://www.unicode.org/reports/tr11/#Definitions>
///
/// The numeric value is compatible with `UEastAsianWidth` in ICU4C.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(EastAsianWidthULE)]
pub enum EastAsianWidth {
    /// name="N"
    Neutral,
    /// name="A"
    Ambiguous,
    /// name="H"
    Halfwidth,
    /// name="F"
    Fullwidth,
    /// name="Na"
    Narrow,
    /// name="W"
    Wide,
}

/// Enumerated property Line_Break.
///
/// See "Line Breaking Properties" in UAX #14 for the summary of each property
/// value: <https://www.unicode.org/reports/tr14/#Properties>
///
/// The numeric value is compatible with `ULineBreak` in ICU4C.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(LineBreakULE)]
pub enum LineBreak {
    /// name="XX"
    Unknown,
    /// name="AI"
    Ambiguous,
    /// name="AL"
    Alphabetic,
    /// name="B2"
    BreakBoth,
    /// name="BA"
    BreakAfter,
    /// name="BB"
    BreakBefore,
    /// name="BK"
    MandatoryBreak,
    /// name="CB"
    ContingentBreak,
    /// name="CL"
    ClosePunctuation,
    /// name="CM"
    CombiningMark,
    /// name="CR"
    CarriageReturn,
    /// name="EX"
    Exclamation,
    /// name="GL"
    Glue,
    /// name="HY"
    Hyphen,
    /// name="ID"
    Ideographic,
    /// name="IN"
    Inseparable,
    /// name="IS"
    InfixNumeric,
    /// name="LF"
    LineFeed,
    /// name="NS"
    Nonstarter,
    /// name="NU"
    Numeric,
    /// name="OP"
    OpenPunctuation,
    /// name="PO"
    PostfixNumeric,
    /// name="PR"
    PrefixNumeric,
    /// name="QU"
    Quotation,
    /// name="SA"
    ComplexContext,
    /// name="SG"
    Surrogate,
    /// name="SP"
    Space,
    /// name="SY"
    BreakSymbols,
    /// name="ZW"
    ZWSpace,
    /// name="NL"
    NextLine,
    /// name="WJ"
    WordJoiner,
    /// name="H2"
    H2,
    /// name="H3"
    H3,
    /// name="JL"
    JL,
    /// name="JT"
    JT,
    /// name="JV"
    JV,
    /// name="CP"
    CloseParenthesis,
    /// name="CJ"
    ConditionalJapaneseStarter,
    /// name="HL"
    HebrewLetter,
    /// name="RI"
    RegionalIndicator,
    /// name="EB"
    EBase,
    /// name="EM"
    EModifier,
    /// name="ZWJ"
    ZWJ,
}

/// Enumerated property Grapheme_Cluster_Break.
///
/// See "Default Grapheme Cluster Boundary Specification" in UAX #29 for the
/// summary of each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Grapheme_Cluster_Table>
///
/// The numeric value is compatible with `UGraphemeClusterBreak` in ICU4C.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(GraphemeClusterBreakULE)]
pub enum GraphemeClusterBreak {
    /// name="XX"
    Other,
    /// name="CN"
    Control,
    /// name="CR"
    CR,
    /// name="EX"
    Extend,
    /// name="L"
    L,
    /// name="LF"
    LF,
    /// name="LV"
    LV,
    /// name="LVT"
    LVT,
    /// name="T"
    T,
    /// name="V"
    V,
    /// name="SM"
    SpacingMark,
    /// name="PP"
    Prepend,
    /// name="RI"
    RegionalIndicator,
    /// This value is obsolete and unused.
    /// name="EB"
    EBase,
    /// This value is obsolete and unused.
    /// name="EBG"
    EBaseGAZ,
    /// This value is obsolete and unused.
    /// name="EM"
    EModifier,
    /// This value is obsolete and unused.
    /// name="GAZ"
    GlueAfterZwj,
    /// name="ZWJ"
    ZWJ,
}

/// Enumerated property Word_Break.
///
/// See "Default Word Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// The numeric value is compatible with `UWordBreakValues` in ICU4C.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(WordBreakULE)]
pub enum WordBreak {
    /// name="XX"
    Other,
    /// name="LE"
    ALetter,
    /// name="FO"
    Format,
    /// name="KA"
    Katakana,
    /// name="ML"
    MidLetter,
    /// name="MN"
    MidNum,
    /// name="NU"
    Numeric,
    /// name="EX"
    ExtendNumLet,
    /// name="CR"
    CR,
    /// name="Extend"
    Extend,
    /// name="LF"
    LF,
    /// name="MB"
    MidNumLet,
    /// name="NL"
    Newline,
    /// name="RI"
    RegionalIndicator,
    /// name="HL"
    HebrewLetter,
    /// name="SQ"
    SingleQuote,
    /// name=DQ
    DoubleQuote,
    /// This value is obsolete and unused.
    /// name="EB"
    EBase,
    /// This value is obsolete and unused.
    /// name="EBG"
    EBaseGAZ,
    /// This value is obsolete and unused.
    /// name="EM"
    EModifier,
    /// This value is obsolete and unused.
    /// name="GAZ"
    GlueAfterZwj,
    /// name="ZWJ"
    ZWJ,
    /// name="WSegSpace"
    WSegSpace,
}

/// Enumerated property Sentence_Break.
/// See "Default Sentence Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// The numeric value is compatible with `USentenceBreak` in ICU4C.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(SentenceBreakULE)]
pub enum SentenceBreak {
    /// name="XX"
    Other,
    /// name="AT"
    ATerm,
    /// name="CL"
    Close,
    /// name="FO"
    Format,
    /// name="LO"
    Lower,
    /// name="NU"
    Numeric,
    /// name="LE"
    OLetter,
    /// name="SE"
    Sep,
    /// name="SP"
    Sp,
    /// name="ST"
    STerm,
    /// name="UP"
    Upper,
    /// name="CR"
    CR,
    /// name="EX"
    Extend,
    /// name="LF"
    LF,
    /// name="SC"
    SContinue,
}

/// Property Canonical_Combining_Class.
/// See UAX #15:
/// <https://www.unicode.org/reports/tr15/>.
///
/// See `icu_normalizer::properties::CanonicalCombiningClassMap` for the API
/// to look up the Canonical_Combining_Class property by scalar value.
//
// NOTE: The Pernosco debugger has special knowledge
// of this type. Please do not change the bit layout
// or the crate-module-qualified name of this type
// without coordination.
#[open_enum]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties))]
#[repr(u8)]
#[zerovec::make_ule(CanonicalCombiningClassULE)]
pub enum CanonicalCombiningClass {
    // These constant names come from PropertyValueAliases.txt
    /// name="NR"
    NotReordered = 0,
    /// name="OV"
    Overlay = 1,
    /// name="HANR"
    HanReading = 6,
    /// name="NK"
    Nukta = 7,
    /// name="KV"
    KanaVoicing = 8,
    /// name="VR"
    Virama = 9,
    /// name="CCC10"
    CCC10 = 10,
    /// name="CCC11"
    CCC11 = 11,
    /// name="CCC12"
    CCC12 = 12,
    /// name="CCC13"
    CCC13 = 13,
    /// name="CCC14"
    CCC14 = 14,
    /// name="CCC15"
    CCC15 = 15,
    /// name="CCC16"
    CCC16 = 16,
    /// name="CCC17"
    CCC17 = 17,
    /// name="CCC18"
    CCC18 = 18,
    /// name="CCC19"
    CCC19 = 19,
    /// name="CCC20"
    CCC20 = 20,
    /// name="CCC21"
    CCC21 = 21,
    /// name="CCC22"
    CCC22 = 22,
    /// name="CCC23"
    CCC23 = 23,
    /// name="CCC24"
    CCC24 = 24,
    /// name="CCC25"
    CCC25 = 25,
    /// name="CCC26"
    CCC26 = 26,
    /// name="CCC27"
    CCC27 = 27,
    /// name="CCC28"
    CCC28 = 28,
    /// name="CCC29"
    CCC29 = 29,
    /// name="CCC30"
    CCC30 = 30,
    /// name="CCC31"
    CCC31 = 31,
    /// name="CCC32"
    CCC32 = 32,
    /// name="CCC33"
    CCC33 = 33,
    /// name="CCC34"
    CCC34 = 34,
    /// name="CCC35"
    CCC35 = 35,
    /// name="CCC36"
    CCC36 = 36,
    /// name="CCC84"
    CCC84 = 84,
    /// name="CCC91"
    CCC91 = 91,
    /// name="CCC103"
    CCC103 = 103,
    /// name="CCC107"
    CCC107 = 107,
    /// name="CCC118"
    CCC118 = 118,
    /// name="CCC122"
    CCC122 = 122,
    /// name="CCC129"
    CCC129 = 129,
    /// name="CCC130"
    CCC130 = 130,
    /// name="CCC132"
    CCC132 = 132,
    /// name="CCC133" (RESERVED)
    CCC133 = 133,
    /// name="ATBL"
    AttachedBelowLeft = 200,
    /// name="ATB"
    AttachedBelow = 202,
    /// name="ATA"
    AttachedAbove = 214,
    /// name="ATAR"
    AttachedAboveRight = 216,
    /// name="BL"
    BelowLeft = 218,
    /// name="B"
    Below = 220,
    /// name="BR"
    BelowRight = 222,
    /// name="L"
    Left = 224,
    /// name="R"
    Right = 226,
    /// name="AL"
    AboveLeft = 228,
    /// name="A"
    Above = 230,
    /// name="AR"
    AboveRight = 232,
    /// name="DB"
    DoubleBelow = 233,
    /// name="DA"
    DoubleAbove = 234,
    /// name="IS"
    IotaSubscript = 240,
}
