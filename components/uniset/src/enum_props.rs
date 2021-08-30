// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of enums for enumerated properties.

/// Selection constants for Unicode properties.
/// These constants are used to select one of the Unicode properties.
/// See UProperty in ICU4C.
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)] // TODO(#686) - Add missing docs.
#[non_exhaustive]
pub enum EnumeratedProperty {
    GeneralCategory = 0x1005,
    Script = 0x100A,
    ScriptExtensions = 0x7000,
}

/// Enumerated Unicode general category types.
/// The discriminants correspond to the U_GC_XX_MASK constants in ICU4C.
/// This supports groups of general categories: for example, `Letter`
/// is the union of `UppercaseLetter`, `LowercaseLetter`, etc...
/// See https://www.unicode.org/reports/tr44/ .
/// See UCharCategory and U_GET_GC_MASK in ICU4C.
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)] // TODO(#686) - Add missing docs.
#[repr(u32)]
#[non_exhaustive]
pub enum GeneralCategory {
    Unassigned = 0,

    UppercaseLetter = 1 << 1,
    LowercaseLetter = 1 << 2,
    TitlecaseLetter = 1 << 3,
    ModifierLetter = 1 << 4,
    OtherLetter = 1 << 5,
    CasedLetter =
        Self::UppercaseLetter as u32 | Self::LowercaseLetter as u32 | Self::TitlecaseLetter as u32,
    Letter = Self::CasedLetter as u32 | Self::ModifierLetter as u32 | Self::OtherLetter as u32,

    NonspacingMark = 1 << 6,
    EnclosingMark = 1 << 7,
    SpacingMark = 1 << 8,
    Mark = Self::NonspacingMark as u32 | Self::EnclosingMark as u32 | Self::SpacingMark as u32,

    Digit = 1 << 9,
    LetterNumber = 1 << 10,
    OtherNumber = 1 << 11,
    Number = Self::Digit as u32 | Self::LetterNumber as u32 | Self::OtherNumber as u32,

    SpaceSeparator = 1 << 12,
    LineSeparator = 1 << 13,
    ParagraphSeparator = 1 << 14,
    Separator =
        Self::SpaceSeparator as u32 | Self::LineSeparator as u32 | Self::ParagraphSeparator as u32,

    Control = 1 << 15,
    Format = 1 << 16,
    PrivateUse = 1 << 17,
    Surrogate = 1 << 18,
    Other = Self::Control as u32
        | Self::Format as u32
        | Self::PrivateUse as u32
        | Self::Surrogate as u32,

    DashPunctuation = 1 << 19,
    OpenPunctuation = 1 << 20,
    ClosePunctuation = 1 << 21,
    ConnectorPunctuation = 1 << 22,
    OtherPunctuation = 1 << 23,
    InitialPunctuation = 1 << 28,
    FinalPunctuation = 1 << 29,
    Punctuation = Self::DashPunctuation as u32
        | Self::OpenPunctuation as u32
        | Self::ClosePunctuation as u32
        | Self::ConnectorPunctuation as u32
        | Self::OtherPunctuation as u32
        | Self::InitialPunctuation as u32
        | Self::FinalPunctuation as u32,

    MathSymbol = 1 << 24,
    CurrencySymbol = 1 << 25,
    ModifierSymbol = 1 << 26,
    OtherSymbol = 1 << 27,
    Symbol = Self::MathSymbol as u32
        | Self::CurrencySymbol as u32
        | Self::ModifierSymbol as u32
        | Self::OtherSymbol as u32,
}

/// Enumerated property Script.
///
/// For more information, see UAX #24: http://www.unicode.org/reports/tr24/.
/// See UScriptCode in ICU4C.
///
/// This enum only contains variants for scripts that are used in the Unicode
/// Property Database.
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)] // The variants should not need documenting.
#[non_exhaustive]
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
    Ethiopic = 11,
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
