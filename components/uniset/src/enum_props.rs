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
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
#[allow(missing_docs)] // TODO(#1030) - Add missing docs.
#[repr(transparent)]
pub struct GeneralCategory(pub(crate) u32);

use GeneralCategory as GC;
use GeneralSubcategory as GS;

#[allow(missing_docs)] // These constants don't need documentation.
#[allow(non_upper_case_globals)]
impl GeneralCategory {
    pub const Unassigned: GeneralCategory = GC(1 << (GS::Unassigned as u32));
    pub const UppercaseLetter: GeneralCategory = GC(1 << (GS::UppercaseLetter as u32));
    pub const LowercaseLetter: GeneralCategory = GC(1 << (GS::LowercaseLetter as u32));
    pub const TitlecaseLetter: GeneralCategory = GC(1 << (GS::TitlecaseLetter as u32));
    pub const ModifierLetter: GeneralCategory = GC(1 << (GS::ModifierLetter as u32));
    pub const OtherLetter: GeneralCategory = GC(1 << (GS::OtherLetter as u32));
    pub const CasedLetter: GeneralCategory = GC(1 << (GS::UppercaseLetter as u32)
        | 1 << (GS::LowercaseLetter as u32)
        | 1 << (GS::TitlecaseLetter as u32));
    pub const Letter: GeneralCategory = GC(1 << (GS::UppercaseLetter as u32)
        | 1 << (GS::LowercaseLetter as u32)
        | 1 << (GS::TitlecaseLetter as u32)
        | 1 << (GS::ModifierLetter as u32)
        | 1 << (GS::OtherLetter as u32));

    pub const NonspacingMark: GeneralCategory = GC(1 << (GS::NonspacingMark as u32));
    pub const EnclosingMark: GeneralCategory = GC(1 << (GS::EnclosingMark as u32));
    pub const SpacingMark: GeneralCategory = GC(1 << (GS::SpacingMark as u32));
    pub const Mark: GeneralCategory = GC(1 << (GS::NonspacingMark as u32)
        | 1 << (GS::EnclosingMark as u32)
        | 1 << (GS::SpacingMark as u32));

    pub const Digit: GeneralCategory = GC(1 << (GS::Digit as u32));
    pub const LetterNumber: GeneralCategory = GC(1 << (GS::LetterNumber as u32));
    pub const OtherNumber: GeneralCategory = GC(1 << (GS::OtherNumber as u32));
    pub const Number: GeneralCategory = GC(1 << (GS::Digit as u32)
        | 1 << (GS::LetterNumber as u32)
        | 1 << (GS::OtherNumber as u32));

    pub const SpaceSeparator: GeneralCategory = GC(1 << (GS::SpaceSeparator as u32));
    pub const LineSeparator: GeneralCategory = GC(1 << (GS::LineSeparator as u32));
    pub const ParagraphSeparator: GeneralCategory = GC(1 << (GS::ParagraphSeparator as u32));
    pub const Separator: GeneralCategory = GC(1 << (GS::SpaceSeparator as u32)
        | 1 << (GS::LineSeparator as u32)
        | 1 << (GS::ParagraphSeparator as u32));

    pub const Control: GeneralCategory = GC(1 << (GS::Control as u32));
    pub const Format: GeneralCategory = GC(1 << (GS::Format as u32));
    pub const PrivateUse: GeneralCategory = GC(1 << (GS::PrivateUse as u32));
    pub const Surrogate: GeneralCategory = GC(1 << (GS::Surrogate as u32));
    pub const Other: GeneralCategory = GC(1 << (GS::Control as u32)
        | 1 << (GS::Format as u32)
        | 1 << (GS::PrivateUse as u32)
        | 1 << (GS::Surrogate as u32));

    pub const DashPunctuation: GeneralCategory = GC(1 << (GS::DashPunctuation as u32));
    pub const OpenPunctuation: GeneralCategory = GC(1 << (GS::OpenPunctuation as u32));
    pub const ClosePunctuation: GeneralCategory = GC(1 << (GS::ClosePunctuation as u32));
    pub const ConnectorPunctuation: GeneralCategory = GC(1 << (GS::ConnectorPunctuation as u32));
    pub const OtherPunctuation: GeneralCategory = GC(1 << (GS::OtherPunctuation as u32));
    pub const InitialPunctuation: GeneralCategory = GC(1 << (GS::InitialPunctuation as u32));
    pub const FinalPunctuation: GeneralCategory = GC(1 << (GS::FinalPunctuation as u32));
    pub const Punctuation: GeneralCategory = GC(1 << (GS::DashPunctuation as u32)
        | 1 << (GS::OpenPunctuation as u32)
        | 1 << (GS::ClosePunctuation as u32)
        | 1 << (GS::ConnectorPunctuation as u32)
        | 1 << (GS::OtherPunctuation as u32)
        | 1 << (GS::InitialPunctuation as u32)
        | 1 << (GS::FinalPunctuation as u32));

    pub const MathSymbol: GeneralCategory = GC(1 << (GS::MathSymbol as u32));
    pub const CurrencySymbol: GeneralCategory = GC(1 << (GS::CurrencySymbol as u32));
    pub const ModifierSymbol: GeneralCategory = GC(1 << (GS::ModifierSymbol as u32));
    pub const OtherSymbol: GeneralCategory = GC(1 << (GS::OtherSymbol as u32));
    pub const Symbol: GeneralCategory = GC(1 << (GS::MathSymbol as u32)
        | 1 << (GS::CurrencySymbol as u32)
        | 1 << (GS::ModifierSymbol as u32)
        | 1 << (GS::OtherSymbol as u32));
}

impl From<GeneralSubcategory> for GeneralCategory {
    fn from(subcategory: GeneralSubcategory) -> Self {
        GeneralCategory(1 << (subcategory as u32))
    }
}

/// Enumerated property Script.
///
/// For more information, see UAX #24: http://www.unicode.org/reports/tr24/.
/// See UScriptCode in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Script(pub(crate) u16);

#[allow(missing_docs)] // These constants don't need documentation.
#[allow(non_upper_case_globals)]
impl Script {
    pub const Adlam: Script = Script(167);
    pub const Ahom: Script = Script(161);
    pub const AnatolianHieroglyphs: Script = Script(156);
    pub const Arabic: Script = Script(2);
    pub const Armenian: Script = Script(3);
    pub const Avestan: Script = Script(117);
    pub const Balinese: Script = Script(62);
    pub const Bamum: Script = Script(130);
    pub const BassaVah: Script = Script(134);
    pub const Batak: Script = Script(63);
    pub const Bengali: Script = Script(4);
    pub const Bhaiksuki: Script = Script(168);
    pub const Bopomofo: Script = Script(5);
    pub const Brahmi: Script = Script(65);
    pub const Braille: Script = Script(46);
    pub const Buginese: Script = Script(55);
    pub const Buhid: Script = Script(44);
    pub const CanadianAboriginal: Script = Script(40);
    pub const Carian: Script = Script(104);
    pub const CaucasianAlbanian: Script = Script(159);
    pub const Chakma: Script = Script(118);
    pub const Cham: Script = Script(66);
    pub const Cherokee: Script = Script(6);
    pub const Chorasmian: Script = Script(189);
    pub const Common: Script = Script(0);
    pub const Coptic: Script = Script(7);
    pub const Cuneiform: Script = Script(101);
    pub const Cypriot: Script = Script(47);
    pub const CyproMinoan: Script = Script(193);
    pub const Cyrillic: Script = Script(8);
    pub const Deseret: Script = Script(9);
    pub const Devanagari: Script = Script(10);
    pub const DivesAkuru: Script = Script(190);
    pub const Dogra: Script = Script(178);
    pub const Duployan: Script = Script(135);
    pub const EgyptianHieroglyphs: Script = Script(71);
    pub const Elbasan: Script = Script(136);
    pub const Elymaic: Script = Script(185);
    pub const Ethiopic: Script = Script(11);
    pub const Georgian: Script = Script(12);
    pub const Glagolitic: Script = Script(56);
    pub const Gothic: Script = Script(13);
    pub const Grantha: Script = Script(137);
    pub const Greek: Script = Script(14);
    pub const Gujarati: Script = Script(15);
    pub const GunjalaGondi: Script = Script(179);
    pub const Gurmukhi: Script = Script(16);
    pub const Han: Script = Script(17);
    pub const Hangul: Script = Script(18);
    pub const HanifiRohingya: Script = Script(182);
    pub const Hanunoo: Script = Script(43);
    pub const Hatran: Script = Script(162);
    pub const Hebrew: Script = Script(19);
    pub const Hiragana: Script = Script(20);
    pub const ImperialAramaic: Script = Script(116);
    pub const Inherited: Script = Script(1);
    pub const InscriptionalPahlavi: Script = Script(122);
    pub const InscriptionalParthian: Script = Script(125);
    pub const Javanese: Script = Script(78);
    pub const Kaithi: Script = Script(120);
    pub const Kannada: Script = Script(21);
    pub const Katakana: Script = Script(22);
    pub const KayahLi: Script = Script(79);
    pub const Kharoshthi: Script = Script(57);
    pub const KhitanSmallScript: Script = Script(191);
    pub const Khmer: Script = Script(23);
    pub const Khojki: Script = Script(157);
    pub const Khudawadi: Script = Script(145);
    pub const Lao: Script = Script(24);
    pub const Latin: Script = Script(25);
    pub const Lepcha: Script = Script(82);
    pub const Limbu: Script = Script(48);
    pub const LinearA: Script = Script(83);
    pub const LinearB: Script = Script(49);
    pub const Lisu: Script = Script(131);
    pub const Lycian: Script = Script(107);
    pub const Lydian: Script = Script(108);
    pub const Mahajani: Script = Script(160);
    pub const Makasar: Script = Script(180);
    pub const Malayalam: Script = Script(26);
    pub const Mandaic: Script = Script(84);
    pub const Manichaean: Script = Script(121);
    pub const Marchen: Script = Script(169);
    pub const MasaramGondi: Script = Script(175);
    pub const Medefaidrin: Script = Script(181);
    pub const MeeteiMayek: Script = Script(115);
    pub const MendeKikakui: Script = Script(140);
    pub const MeroiticCursive: Script = Script(141);
    pub const MeroiticHieroglyphs: Script = Script(86);
    pub const Miao: Script = Script(92);
    pub const Modi: Script = Script(163);
    pub const Mongolian: Script = Script(27);
    pub const Mro: Script = Script(149);
    pub const Multani: Script = Script(164);
    pub const Myanmar: Script = Script(28);
    pub const Nabataean: Script = Script(143);
    pub const Nandinagari: Script = Script(187);
    pub const NewTaiLue: Script = Script(59);
    pub const Newa: Script = Script(170);
    pub const Nko: Script = Script(87);
    pub const Nushu: Script = Script(150);
    pub const NyiakengPuachueHmong: Script = Script(186);
    pub const Ogham: Script = Script(29);
    pub const OlChiki: Script = Script(109);
    pub const OldHungarian: Script = Script(76);
    pub const OldItalic: Script = Script(30);
    pub const OldNorthArabian: Script = Script(142);
    pub const OldPermic: Script = Script(89);
    pub const OldPersian: Script = Script(61);
    pub const OldSogdian: Script = Script(184);
    pub const OldSouthArabian: Script = Script(133);
    pub const OldTurkic: Script = Script(88);
    pub const OldUyghur: Script = Script(194);
    pub const Oriya: Script = Script(31);
    pub const Osage: Script = Script(171);
    pub const Osmanya: Script = Script(50);
    pub const PahawhHmong: Script = Script(75);
    pub const Palmyrene: Script = Script(144);
    pub const PauCinHau: Script = Script(165);
    pub const PhagsPa: Script = Script(90);
    pub const Phoenician: Script = Script(91);
    pub const PsalterPahlavi: Script = Script(123);
    pub const Rejang: Script = Script(110);
    pub const Runic: Script = Script(32);
    pub const Samaritan: Script = Script(126);
    pub const Saurashtra: Script = Script(111);
    pub const Sharada: Script = Script(151);
    pub const Shavian: Script = Script(51);
    pub const Siddham: Script = Script(166);
    pub const SignWriting: Script = Script(112);
    pub const Sinhala: Script = Script(33);
    pub const Sogdian: Script = Script(183);
    pub const SoraSompeng: Script = Script(152);
    pub const Soyombo: Script = Script(176);
    pub const Sundanese: Script = Script(113);
    pub const SylotiNagri: Script = Script(58);
    pub const Syriac: Script = Script(34);
    pub const Tagalog: Script = Script(42);
    pub const Tagbanwa: Script = Script(45);
    pub const TaiLe: Script = Script(52);
    pub const TaiTham: Script = Script(106);
    pub const TaiViet: Script = Script(127);
    pub const Takri: Script = Script(153);
    pub const Tamil: Script = Script(35);
    pub const Tangsa: Script = Script(195);
    pub const Tangut: Script = Script(154);
    pub const Telugu: Script = Script(36);
    pub const Thaana: Script = Script(37);
    pub const Thai: Script = Script(38);
    pub const Tibetan: Script = Script(39);
    pub const Tifinagh: Script = Script(60);
    pub const Tirhuta: Script = Script(158);
    pub const Toto: Script = Script(196);
    pub const Ugaritic: Script = Script(53);
    pub const Unknown: Script = Script(103);
    pub const Vai: Script = Script(99);
    pub const Vithkuqi: Script = Script(197);
    pub const Wancho: Script = Script(188);
    pub const WarangCiti: Script = Script(146);
    pub const Yezidi: Script = Script(192);
    pub const Yi: Script = Script(41);
    pub const ZanabazarSquare: Script = Script(177);
}
