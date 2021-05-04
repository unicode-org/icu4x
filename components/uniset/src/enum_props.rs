// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::upper_case_acronyms)]

/// Selection constants for Unicode properties.
/// These constants are used to select one of the Unicode properties.
/// See UProperty in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum EnumeratedProperty {
    BidiClass = 0x1000,
    BidiPairedBracketType = 0x1015,
    CanonicalCombiningClass = 0x1002,
    DecompositionType = 0x1003,
    EastAsianWidth = 0x1004,
    GeneralCategory = 0x1005,
    GraphemeClusterBreak = 0x1012,
    HangulSyllableType = 0x100B,
    IndicPositionalCategory = 0x1016,
    IndicSyllabicCategory = 0x1017,
    JoiningGroup = 0x1006,
    JoiningType = 0x1007,
    LineBreak = 0x1008,
    LeadCanonicalCombiningClass = 0x1010,
    NFCQuickCheck = 0x100E,
    NFDQuickCheck = 0x100C,
    NFKCQuickCheck = 0x100F,
    NFKDQuickCheck = 0x100D,
    NumericType = 0x1009,
    SentenceBreak = 0x1013,
    TrailCanonicalCombiningClass = 0x1011,
    VerticalOrientation = 0x1018,
    WordBreak = 0x1014,
}

/// This specifies the the categories required by the Unicode Bidirectional Algorithm.
/// For more information, see Section 3.2 of Unicode Standard Annex #9, "Unicode Bidirectional Algorithm".
/// See UCharDirection in ICU4C and Bidi_Class in PPUCD.
#[derive(Clone, PartialEq, Debug)]
pub enum BidiClass {
    ArabicLetter = 13,
    ArabicNumber = 5,
    ParagraphSeparator = 7,
    BoundaryNeutral = 18,
    CommonSeparator = 6,
    EuropeanNumber = 2,
    EuropeanSeparator = 3,
    EuropeanTerminator = 4,
    FirstStrongIsolate = 19,
    LeftToRight = 0,
    LeftToRightEmbedding = 11,
    LeftToRightIsolate = 20,
    LeftToRightOverride = 12,
    NonspacingMark = 17,
    OtherNeutral = 10,
    PopDirectionalFormat = 16,
    PopDirectionalIsolate = 22,
    RightToLeft = 1,
    RightToLeftEmbedding = 14,
    RightToLeftIsolate = 21,
    RightToLeftOverride = 15,
    SegmentSeparator = 8,
    WhiteSpace = 9,
}

/// Type of a paired bracket, either opening or closing. 
/// This property is used in the implementation of parenthesis matching.
/// See Unicode Standard Annex #9, "Unicode Bidirectional Algorithm".
/// See UBidiPairedBracketType in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum BidiPairedBracketType {
    Close = 2,
    None = 0,
    Open = 1,
}

/// The classes used for the Canonical Ordering Algorithm in the Unicode Standard. 
/// This property could be considered either an enumerated property or a numeric property: 
/// the principal use of the property is in terms of the numeric values.
/// Same as u_getCombiningClass, returns 8-bit numeric values that specify the
/// combining class of the code point as specified in UnicodeData.txt.
/// See UCHAR_CANONICAL_COMBINING_CLASS in ICU4C
/// and CanonicalCombiningClass in PPUCD.
#[derive(Clone, PartialEq, Debug)]
pub enum CanonicalCombiningClass {
    NotReordered = 0,
    Overlay = 1,
    CCC10 = 10,
    CCC103 = 103,
    CCC107 = 107,
    CCC11 = 11,
    CCC118 = 118,
    CCC12 = 12,
    CCC122 = 122,
    CCC129 = 129,
    CCC13 = 13,
    CCC130 = 130,
    CCC132 = 132,
    CCC133 = 133,
    CCC14 = 14,
    CCC15 = 15,
    CCC16 = 16,
    CCC17 = 17,
    CCC18 = 18,
    CCC19 = 19,
    CCC20 = 20,
    AttachedBelowLeft = 200,
    AttachedBelow = 202,
    CCC21 = 21,
    AttachedAbove = 214,
    AttachedAboveRight = 216,
    BelowLeft = 218,
    CCC22 = 22,
    Below = 220,
    BelowRight = 222,
    Left = 224,
    Right = 226,
    AboveLeft = 228,
    CCC23 = 23,
    Above = 230,
    AboveRight = 232,
    DoubleBelow = 233,
    DoubleAbove = 234,
    CCC24 = 24,
    IotaSubscript = 240,
    CCC25 = 25,
    CCC26 = 26,
    CCC27 = 27,
    CCC28 = 28,
    CCC29 = 29,
    CCC30 = 30,
    CCC31 = 31,
    CCC32 = 32,
    CCC33 = 33,
    CCC34 = 34,
    CCC35 = 35,
    CCC36 = 36,
    HanReading = 6,
    Nukta = 7,
    KanaVoicing = 8,
    CCC84 = 84,
    Virama = 9,
    CCC91 = 91,
}

/// Decomposition Type constants.
/// These are listed as Compatibility Formatting Tags in
/// section 5.7.3 "Character Decomposition Mapping" in Unicode Standard Annex #44,
/// "Unicode Character Database".
/// See UDecompositionType in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum DecompositionType {
    Can = 1,
    Com = 2,
    Enc = 3,
    Fin = 4,
    Font = 5,
    Fra = 6,
    Init = 7,
    Iso = 8,
    Med = 9,
    Nar = 10,
    Nb = 11,
    None = 0,
    Sml = 12,
    Sqr = 13,
    Sub = 14,
    Sup = 15,
    Vert = 16,
    Wide = 17,
}

/// A property for determining the choice of wide versus narrow glyphs in East Asian contexts.
/// Property values are described in Unicode Standard Annex #11, "East Asian Width".
/// See UEastAsianWidth in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum EastAsianWidth {
    Ambiguous = 1,
    Fullwidth = 3,
    Halfwidth = 2,
    Neutral = 0,
    Narrow = 4,
    Wide = 5,
}

// TODO: Ask how to create a stable numbering for enums not 
// represented in ICU4C (ICU4C has ~ 30 enums, PPUCD has ~ 38
// enums). Hunch: the "extra" PPUCD properties are the "rollup"
// properties.
// TODO: ^ If hunch is true, find docs for the precise terminology.
// TODO: make proposal on how to handle. Have a separate enum for
// rollups? If so, what about Letter vs. CasedLetter vs. LowercaseLetter?
// TODO: Why does ICU4C have `Other` and `Unassigned` use the same 
// enum value (discriminant) ?
//
/// Enumerated Unicode general category types.
/// See https://www.unicode.org/reports/tr44/ .
/// See UCharCategory in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum GeneralCategory {
    Other = 0,
    Cntrl = 15,
    Format = 16,
    Unassigned = 0,
    PrivateUse = 17,
    Surrogate = 18,
    // Letter = ,
    // CasedLetter = ,
    LowercaseLetter = 2,
    ModifierLetter = 4,
    OtherLetter = 5,
    TitlecaseLetter = 3,
    UppercaseLetter = 1,
    CombiningMark = 8,
    // SpacingMark = ,
    EnclosingMark = 7,
    NonspacingMark = 6,
    // Number = ,
    Digit = 9,
    LetterNumber = 10,
    OtherNumber = 11,
    // Punct = ,
    ConnectorPunctuation = 22,
    DashPunctuation = 19,
    ClosePunctuation = 21,
    FinalPunctuation = 29,
    InitialPunctuation = 28,
    OtherPunctuation = 23,
    OpenPunctuation = 20,
    // Symbol = ,
    CurrencySymbol = 25,
    ModifierSymbol = 26,
    MathSymbol = 24,
    OtherSymbol = 27,
    // Separator = ,
    LineSeparator = 13,
    ParagraphSeparator = 14,
    SpaceSeparator = 12,
}

/// Grapheme Cluster Break constants.
/// See Section 3.1 "Default Graphame Cluster Boundary Specification" in
/// Unicode Standard Annex #29, "Unicode Text Segmentation".
/// See UGraphemeClusterBreak in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum GraphemeClusterBreak {
    Control = 1,
    CR = 2,
    EBase = 13,
    EBaseGAZ = 14,
    EModifier = 15,
    Extend = 3,
    GlueAfterZwj = 16,
    L = 4,
    LF = 5,
    LV = 6,
    LVT = 7,
    Prepend = 11,
    RegionalIndicator = 12,
    SpacingMark = 10,
    T = 8,
    V = 9,
    Other = 0,
    ZWJ = 17,
}

/// Hangul Syllable Type constants.
/// See the values L, V, T, LV, and LVT used in Chapter 3, Conformance in
/// the Unicode Standard.
/// See UHangulSyllableType in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum HangulSyllableType {
    LeadingJamo = 1,
    LVSyllable = 4,
    LVTSyllable = 5,
    NotApplicable = 0,
    TrailingJamo = 3,
    VowelJamo = 2,
}

/// A property informally defining the positional categories for dependent vowels, viramas, combining marks, and other characters used in Indic scripts.
/// General descriptions of the property values are provided in the header section of the data file IndicPositionalCategory.txt.
/// See UIndicPositionalCategory in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum IndicPositionalCategory {
    Bottom = 1,
    BottomAndLeft = 2,
    BottomAndRight = 3,
    Left = 4,
    LeftAndRight = 5,
    NA = 0,
    Overstruck = 6,
    Right = 7,
    Top = 8,
    TopAndBottom = 9,
    TopAndBottomAndLeft = 15,
    TopAndBottomAndRight = 10,
    TopAndLeft = 11,
    TopAndLeftAndRight = 12,
    TopAndRight = 13,
    VisualOrderLeft = 14,
}

/// A property informally defining the structural categories of syllabic components in Indic scripts.
/// General descriptions of the property values are provided in the header section of the data file IndicSyllabicCategory.txt.
/// See UIndicSyllabicCategory in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum IndicSyllabicCategory {
    Avagraha = 1,
    Bindu = 2,
    BrahmiJoiningNumber = 3,
    CantillationMark = 4,
    Consonant = 5,
    ConsonantDead = 6,
    ConsonantFinal = 7,
    ConsonantHeadLetter = 8,
    ConsonantInitialPostfixed = 9,
    ConsonantKiller = 10,
    ConsonantMedial = 11,
    ConsonantPlaceholder = 12,
    ConsonantPrecedingRepha = 13,
    ConsonantPrefixed = 14,
    ConsonantSubjoined = 15,
    ConsonantSucceedingRepha = 16,
    ConsonantWithStacker = 17,
    GeminationMark = 18,
    InvisibleStacker = 19,
    Joiner = 20,
    ModifyingLetter = 21,
    NonJoiner = 22,
    Nukta = 23,
    Number = 24,
    NumberJoiner = 25,
    Other = 0,
    PureKiller = 26,
    RegisterShifter = 27,
    SyllableModifier = 28,
    ToneLetter = 29,
    ToneMark = 30,
    Virama = 31,
    Visarga = 32,
    Vowel = 33,
    VowelDependent = 34,
    VowelIndependent = 35,
}

// TODO: suggest update (to include more script categories) for description in UAX 44.
/// Joining Group constants.
/// See UJoiningGroup in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum JoiningGroup {
    AfricanFeh = 86,
    AfricanNoon = 87,
    AfricanQaf = 88,
    Ain = 1,
    Alaph = 2,
    Alef = 3,
    Beh = 4,
    Beth = 5,
    BurushaskiYehBarree = 54,
    Dal = 6,
    DalathRish = 7,
    E = 8,
    FarsiYeh = 55,
    Fe = 51,
    Feh = 9,
    FinalSemkath = 10,
    Gaf = 11,
    Gamal = 12,
    Hah = 13,
    HanifiRohingyaKinnaYa = 100,
    HanifiRohingyaPa = 101,
    He = 15,
    Heh = 16,
    HehGoal = 17,
    Heth = 18,
    Kaf = 19,
    Kaph = 20,
    Khaph = 52,
    KnottedHeh = 21,
    Lam = 22,
    Lamadh = 23,
    MalayalamBha = 89,
    MalayalamJa = 90,
    MalayalamLla = 91,
    MalayalamLlla = 92,
    MalayalamNga = 93,
    MalayalamNna = 94,
    MalayalamNnna = 95,
    MalayalamNya = 96,
    MalayalamRa = 97,
    MalayalamSsa = 98,
    MalayalamTta = 99,
    ManichaeanAleph = 58,
    ManichaeanAyin = 59,
    ManichaeanBeth = 60,
    ManichaeanDaleth = 61,
    ManichaeanDhamedh = 62,
    ManichaeanFive = 63,
    ManichaeanGimel = 64,
    ManichaeanHeth = 65,
    ManichaeanHundred = 66,
    ManichaeanKaph = 67,
    ManichaeanLamedh = 68,
    ManichaeanMem = 69,
    ManichaeanNun = 70,
    ManichaeanOne = 71,
    ManichaeanPe = 72,
    ManichaeanQoph = 73,
    ManichaeanResh = 74,
    ManichaeanSadhe = 75,
    ManichaeanSamekh = 76,
    ManichaeanTaw = 77,
    ManichaeanTen = 78,
    ManichaeanTeth = 79,
    ManichaeanThamedh = 80,
    ManichaeanTwenty = 81,
    ManichaeanWaw = 82,
    ManichaeanYodh = 83,
    ManichaeanZayin = 84,
    Meem = 24,
    Mim = 25,
    NoJoiningGroup = 0,
    Noon = 26,
    Nun = 27,
    Nya = 56,
    Pe = 28,
    Qaf = 29,
    Qaph = 30,
    Reh = 31,
    ReversedPe = 32,
    RohingyaYeh = 57,
    Sad = 33,
    Sadhe = 34,
    Seen = 35,
    Semkath = 36,
    Shin = 37,
    StraightWaw = 85,
    SwashKaf = 38,
    SyriacWaw = 39,
    Tah = 40,
    Taw = 41,
    TehMarbuta = 42,
    TehMarbutaGoal = 14,
    Teth = 43,
    Waw = 44,
    Yeh = 45,
    YehBarree = 46,
    YehWithTail = 47,
    Yudh = 48,
    YudhHe = 49,
    Zain = 50,
    Zhain = 53,
}

/// Joining Type constants.
/// See UJoiningType in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum JoiningType {
    JoinCausing = 1,
    DualJoining = 2,
    LeftJoining = 3,
    RightJoining = 4,
    Transparent = 5,
    NonJoining = 0,
}

/// Line Break constants.
/// See ULineBreak in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum LineBreak {
    Ambiguous = 1,
    Alphabetic = 2,
    BreakBoth = 3,
    BreakAfter = 4,
    BreakBefore = 5,
    MandatoryBreak = 6,
    ContingentBreak = 7,
    ConditionalJapaneseStarter = 37,
    ClosePunctuation = 8,
    CombiningMark = 9,
    CloseParenthesis = 36,
    CarriageReturn = 10,
    EBase = 40,
    EModifier = 41,
    Exclamation = 11,
    Glue = 12,
    H2 = 31,
    H3 = 32,
    HebrewLetter = 38,
    Hyphen = 13,
    Ideographic = 14,
    Inseparable = 15,
    InfixNumeric = 16,
    JL = 33,
    JT = 34,
    JV = 35,
    LineFeed = 17,
    NextLine = 29,
    Nonstarter = 18,
    Numeric = 19,
    OpenPunctuation = 20,
    PostfixNumeric = 21,
    PrefixNumeric = 22,
    Quotation = 23,
    RegionalIndicator = 39,
    ComplexContext = 24,
    Surrogate = 25,
    Space = 26,
    BreakSymbols = 27,
    WordJoiner = 30,
    Unknown = 0,
    ZWSpace = 28,
    ZWJ = 42,
}

// The Rust enum determinant values are meaningful in ICU4C for collation
// implementation purposes, so those numerical values are preserved here, too.
/// Enumerated property Lead_Canonical_Combining_Class.
/// ICU-specific property for the ccc of the first code point
/// of the decomposition, or lccc(c)=ccc(NFD(c)[0]).
/// Useful for checking for canonically ordered text;
/// see UNORM_FCD and http://www.unicode.org/notes/tn5/#FCD .
/// Returns 8-bit numeric values like [`CanonicalCombiningClass`].
/// See UCHAR_LEAD_CANONICAL_COMBINING_CLASS in ICU4C
/// and LeadCanonicalCombiningClass in PPUCD.
#[derive(Clone, PartialEq, Debug)]
pub enum LeadCanonicalCombiningClass {
    NotReordered = 0,
    Overlay = 1,
    CCC10 = 10,
    CCC103 = 103,
    CCC107 = 107,
    CCC11 = 11,
    CCC118 = 118,
    CCC12 = 12,
    CCC122 = 122,
    CCC129 = 129,
    CCC13 = 13,
    CCC130 = 130,
    CCC132 = 132,
    CCC133 = 133,
    CCC14 = 14,
    CCC15 = 15,
    CCC16 = 16,
    CCC17 = 17,
    CCC18 = 18,
    CCC19 = 19,
    CCC20 = 20,
    AttachedBelowLeft = 200,
    AttachedBelow = 202,
    CCC21 = 21,
    AttachedAbove = 214,
    AttachedAboveRight = 216,
    BelowLeft = 218,
    CCC22 = 22,
    Below = 220,
    BelowRight = 222,
    Left = 224,
    Right = 226,
    AboveLeft = 228,
    CCC23 = 23,
    Above = 230,
    AboveRight = 232,
    DoubleBelow = 233,
    DoubleAbove = 234,
    CCC24 = 24,
    IotaSubscript = 240,
    CCC25 = 25,
    CCC26 = 26,
    CCC27 = 27,
    CCC28 = 28,
    CCC29 = 29,
    CCC30 = 30,
    CCC31 = 31,
    CCC32 = 32,
    CCC33 = 33,
    CCC34 = 34,
    CCC35 = 35,
    CCC36 = 36,
    HanReading = 6,
    Nukta = 7,
    KanaVoicing = 8,
    CCC84 = 84,
    Virama = 9,
    CCC91 = 91,
}

/// See UNormalizationCheckResult, UNormalizationMode, and  unorm_quickCheck() in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum NFCQuickCheck {
    Maybe = 0,
    No = 1,
    Yes = 2,
}

/// See UNormalizationCheckResult, UNormalizationMode, and  unorm_quickCheck() in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum NFDQuickCheck {
    No = 1,
    Yes = 2,
}

/// See UNormalizationCheckResult, UNormalizationMode, and  unorm_quickCheck() in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum NFKCQuickCheck {
    Maybe = 0,
    No = 1,
    Yes = 2,
}

/// See UNormalizationCheckResult, UNormalizationMode, and  unorm_quickCheck() in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum NFKDQuickCheck {
    No = 1,
    Yes = 2,
}

/// Numeric Type constants.
/// See UNumericType in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum NumericType {
    Decimal = 1,
    Digit = 2,
    None = 0,
    Numeric = 3,
}

/// Sentence Break constants.
/// See USentenceBreak in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum SentenceBreak {
    ATerm = 1,
    Close = 2,
    CR = 11,
    Extend = 12,
    Format = 3,
    OLetter = 6,
    LF = 13,
    Lower = 4,
    Numeric = 5,
    SContinue = 14,
    Sep = 7,
    Sp = 8,
    STerm = 9,
    Upper = 10,
    Other = 0,
}

// The Rust enum determinant values are meaningful in ICU4C for collation
// implementation purposes, so those numerical values are preserved here, too.
/// Enumerated property Trail_Canonical_Combining_Class.
/// ICU-specific property for the ccc of the last code point
/// of the decomposition, or tccc(c)=ccc(NFD(c)[last]).
/// Useful for checking for canonically ordered text;
/// see UNORM_FCD and http://www.unicode.org/notes/tn5/#FCD .
/// Returns 8-bit numeric values like [`CanonicalCombiningClass`].
/// See UCHAR_TRAIL_CANONICAL_COMBINING_CLASS in ICU4C and
/// TrailCanonicalCombiningClass in PPUCD.
#[derive(Clone, PartialEq, Debug)]
pub enum TrailCanonicalCombiningClass {
    NotReordered = 0,
    Overlay = 1,
    CCC10 = 10,
    CCC103 = 103,
    CCC107 = 107,
    CCC11 = 11,
    CCC118 = 118,
    CCC12 = 12,
    CCC122 = 122,
    CCC129 = 129,
    CCC13 = 13,
    CCC130 = 130,
    CCC132 = 132,
    CCC133 = 133,
    CCC14 = 14,
    CCC15 = 15,
    CCC16 = 16,
    CCC17 = 17,
    CCC18 = 18,
    CCC19 = 19,
    CCC20 = 20,
    AttachedBelowLeft = 200,
    AttachedBelow = 202,
    CCC21 = 21,
    AttachedAbove = 214,
    AttachedAboveRight = 216,
    BelowLeft = 218,
    CCC22 = 22,
    Below = 220,
    BelowRight = 222,
    Left = 224,
    Right = 226,
    AboveLeft = 228,
    CCC23 = 23,
    Above = 230,
    AboveRight = 232,
    DoubleBelow = 233,
    DoubleAbove = 234,
    CCC24 = 24,
    IotaSubscript = 240,
    CCC25 = 25,
    CCC26 = 26,
    CCC27 = 27,
    CCC28 = 28,
    CCC29 = 29,
    CCC30 = 30,
    CCC31 = 31,
    CCC32 = 32,
    CCC33 = 33,
    CCC34 = 34,
    CCC35 = 35,
    CCC36 = 36,
    HanReading = 6,
    Nukta = 7,
    KanaVoicing = 8,
    CCC84 = 84,
    Virama = 9,
    CCC91 = 91,
}

/// A property used to establish a default for the
/// correct orientation of characters when used in vertical text layout,
/// as described in Unicode Standard Annex #50, "Unicode Vertical Text Layout".
/// See UVerticalOrientation in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum VerticalOrientation {
    Rotated = 0,
    TransformedRotated = 1,
    TransformedUpright = 2,
    Upright = 3,
}

/// Word Break constants.
/// See UWordBreakValues in ICU4C.
#[derive(Clone, PartialEq, Debug)]
pub enum WordBreak {
    CR = 8,
    DoubleQuote = 16,
    EBase = 17,
    EBaseGAZ = 18,
    EModifier = 19,
    ExtendNumLet = 7,
    Extend = 9,
    Format = 2,
    GlueAfterZwj = 20,
    HebrewLetter = 14,
    Katakana = 3,
    ALetter = 1,
    LF = 10,
    MidNumLet = 11,
    MidLetter = 4,
    MidNum = 5,
    Newline = 12,
    Numeric = 6,
    RegionalIndicator = 13,
    SingleQuote = 15,
    WSegSpace = 22,
    Other = 0,
    ZWJ = 21,
}
