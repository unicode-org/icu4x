// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of enums for enumerated properties.

use num_enum::{TryFromPrimitive, UnsafeFromPrimitive};

/// Selection constants for Unicode properties.
/// These constants are used to select one of the Unicode properties.
/// See UProperty in ICU4C.
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)] // TODO(#1030) - Add missing docs.
#[non_exhaustive]
pub enum EnumeratedProperty {
    GeneralCategory = 0x1005,
    Script = 0x100A,
    ScriptExtensions = 0x7000,
}

/// Enumerated Unicode general category types.
/// GeneralSubcategory only supports specific subcategories (eg UppercaseLetter).
/// It does not support grouped categories (eg Letter). For grouped categories, use GeneralCategory.
#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive, UnsafeFromPrimitive)]
#[allow(missing_docs)] // TODO(#1030) - Add missing docs.
#[repr(u8)]
pub enum GeneralSubcategory {
    Unassigned = 0,

    UppercaseLetter = 1,
    LowercaseLetter = 2,
    TitlecaseLetter = 3,
    ModifierLetter = 4,
    OtherLetter = 5,

    NonspacingMark = 6,
    EnclosingMark = 7,
    SpacingMark = 8,

    Digit = 9,
    LetterNumber = 10,
    OtherNumber = 11,

    SpaceSeparator = 12,
    LineSeparator = 13,
    ParagraphSeparator = 14,

    Control = 15,
    Format = 16,
    PrivateUse = 17,
    Surrogate = 18,

    DashPunctuation = 19,
    OpenPunctuation = 20,
    ClosePunctuation = 21,
    ConnectorPunctuation = 22,
    OtherPunctuation = 23,
    InitialPunctuation = 28,
    FinalPunctuation = 29,

    MathSymbol = 24,
    CurrencySymbol = 25,
    ModifierSymbol = 26,
    OtherSymbol = 27,
}

/// Enumerated Unicode general category types.
/// The discriminants correspond to the U_GC_XX_MASK constants in ICU4C.
/// Unlike GeneralSubcategory, this supports groups of general categories: for example, `Letter`
/// is the union of `UppercaseLetter`, `LowercaseLetter`, etc...
/// See https://www.unicode.org/reports/tr44/ .
/// See UCharCategory and U_GET_GC_MASK in ICU4C.
#[derive(Clone, PartialEq, Debug)]
#[allow(missing_docs)] // TODO(#1030) - Add missing docs.
#[repr(u32)]
pub enum GeneralCategory {
    Unassigned = 0,

    UppercaseLetter = 1 << (GeneralSubcategory::UppercaseLetter as u32),
    LowercaseLetter = 1 << (GeneralSubcategory::LowercaseLetter as u32),
    TitlecaseLetter = 1 << (GeneralSubcategory::TitlecaseLetter as u32),
    ModifierLetter = 1 << (GeneralSubcategory::ModifierLetter as u32),
    OtherLetter = 1 << (GeneralSubcategory::OtherLetter as u32),
    CasedLetter = 1 << (GeneralSubcategory::UppercaseLetter as u32)
        | 1 << (GeneralSubcategory::LowercaseLetter as u32)
        | 1 << (GeneralSubcategory::TitlecaseLetter as u32),
    Letter = 1 << (GeneralSubcategory::UppercaseLetter as u32)
        | 1 << (GeneralSubcategory::LowercaseLetter as u32)
        | 1 << (GeneralSubcategory::TitlecaseLetter as u32)
        | 1 << (GeneralSubcategory::ModifierLetter as u32)
        | 1 << (GeneralSubcategory::OtherLetter as u32),

    NonspacingMark = 1 << (GeneralSubcategory::NonspacingMark as u32),
    EnclosingMark = 1 << (GeneralSubcategory::EnclosingMark as u32),
    SpacingMark = 1 << (GeneralSubcategory::SpacingMark as u32),
    Mark = 1 << (GeneralSubcategory::NonspacingMark as u32)
        | 1 << (GeneralSubcategory::EnclosingMark as u32)
        | 1 << (GeneralSubcategory::SpacingMark as u32),

    Digit = 1 << (GeneralSubcategory::Digit as u32),
    LetterNumber = 1 << (GeneralSubcategory::LetterNumber as u32),
    OtherNumber = 1 << (GeneralSubcategory::OtherNumber as u32),
    Number =
        1 << (GeneralSubcategory::Digit as u32) | 1 << (GeneralSubcategory::LetterNumber as u32) | 1 << (GeneralSubcategory::OtherNumber as u32),

    SpaceSeparator = 1 << (GeneralSubcategory::SpaceSeparator as u32),
    LineSeparator = 1 << (GeneralSubcategory::LineSeparator as u32),
    ParagraphSeparator = 1 << (GeneralSubcategory::ParagraphSeparator as u32),
    Separator = 1 << (GeneralSubcategory::SpaceSeparator as u32)
        | 1 << (GeneralSubcategory::LineSeparator as u32)
        | 1 << (GeneralSubcategory::ParagraphSeparator as u32),

    Control = 1 << (GeneralSubcategory::Control as u32),
    Format = 1 << (GeneralSubcategory::Format as u32),
    PrivateUse = 1 << (GeneralSubcategory::PrivateUse as u32),
    Surrogate = 1 << (GeneralSubcategory::Surrogate as u32),
    Other = 1 << (GeneralSubcategory::Control as u32)
        | 1 << (GeneralSubcategory::Format as u32)
        | 1 << (GeneralSubcategory::PrivateUse as u32)
        | 1 << (GeneralSubcategory::Surrogate as u32),

    DashPunctuation = 1 << (GeneralSubcategory::DashPunctuation as u32),
    OpenPunctuation = 1 << (GeneralSubcategory::OpenPunctuation as u32),
    ClosePunctuation = 1 << (GeneralSubcategory::ClosePunctuation as u32),
    ConnectorPunctuation = 1 << (GeneralSubcategory::ConnectorPunctuation as u32),
    OtherPunctuation = 1 << (GeneralSubcategory::OtherPunctuation as u32),
    InitialPunctuation = 1 << (GeneralSubcategory::InitialPunctuation as u32),
    FinalPunctuation = 1 << (GeneralSubcategory::FinalPunctuation as u32),
    Punctuation = 1 << (GeneralSubcategory::DashPunctuation as u32)
        | 1 << (GeneralSubcategory::OpenPunctuation as u32)
        | 1 << (GeneralSubcategory::ClosePunctuation as u32)
        | 1 << (GeneralSubcategory::ConnectorPunctuation as u32)
        | 1 << (GeneralSubcategory::OtherPunctuation as u32)
        | 1 << (GeneralSubcategory::InitialPunctuation as u32)
        | 1 << (GeneralSubcategory::FinalPunctuation as u32),

    MathSymbol = 1 << (GeneralSubcategory::MathSymbol as u32),
    CurrencySymbol = 1 << (GeneralSubcategory::CurrencySymbol as u32),
    ModifierSymbol = 1 << (GeneralSubcategory::ModifierSymbol as u32),
    OtherSymbol = 1 << (GeneralSubcategory::OtherSymbol as u32),
    Symbol = 1 << (GeneralSubcategory::MathSymbol as u32)
        | 1 << (GeneralSubcategory::CurrencySymbol as u32)
        | 1 << (GeneralSubcategory::ModifierSymbol as u32)
        | 1 << (GeneralSubcategory::OtherSymbol as u32),
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
