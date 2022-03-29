// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of enums for enumerated properties.

#![allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Selection constants for Unicode properties.
/// These constants are used to select one of the Unicode properties.
/// See `UProperty` in ICU4C.
#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
#[repr(i32)]
pub enum EnumeratedProperty {
    /// the Canonical_Combining_Class property.
    CanonicalCombiningClass = 0x1002,
    /// The East_Asian_Width property. See [`EastAsianWidth`].
    EastAsianWidth = 0x1004,
    /// The General_Category property.
    GeneralCategory = 0x1005,
    /// A pseudo-property that is used to represent groupings of `GeneralCategory`.
    GeneralCategoryGroup = 0x2000,
    /// The Line_Break enumerated property. See [`LineBreak`].
    LineBreak = 0x1008,
    /// The Script property. See [`Script`].
    Script = 0x100A,
    /// The Grapheme_Cluster_Break enumerated property. See [`GraphemeClusterBreak`].
    GraphemeClusterBreak = 0x1012,
    /// The Sentence_Break enumerated property. See [`SentenceBreak`].
    SentenceBreak = 0x1013,
    /// The Word_Break enumerated property. See [`WordBreak`].
    WordBreak = 0x1014,
    /// The Script_Extensions property. See [`Script`].
    ScriptExtensions = 0x7000, // TODO(#1160) - this is a Miscellaneous property, not Enumerated
    /// Represents an invalid or unknown Unicode property.
    InvalidCode = -1, // TODO(#1160) - taken from ICU4C UProperty::UCHAR_INVALID_CODE
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
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
#[repr(transparent)]
pub struct GeneralCategoryGroup(pub(crate) u32);

use GeneralCategory as GC;
use GeneralCategoryGroup as GCG;

#[allow(non_upper_case_globals)]
impl GeneralCategoryGroup {
    /// (`Lu`) An uppercase letter
    pub const UppercaseLetter: GeneralCategoryGroup = GCG(1 << (GC::UppercaseLetter as u32));
    /// (`Ll`) A lowercase letter
    pub const LowercaseLetter: GeneralCategoryGroup = GCG(1 << (GC::LowercaseLetter as u32));
    /// (`Lt`) A digraphic letter, with first part uppercase
    pub const TitlecaseLetter: GeneralCategoryGroup = GCG(1 << (GC::TitlecaseLetter as u32));
    /// (`Lm`) A modifier letter
    pub const ModifierLetter: GeneralCategoryGroup = GCG(1 << (GC::ModifierLetter as u32));
    /// (`Lo`) Other letters, including syllables and ideographs
    pub const OtherLetter: GeneralCategoryGroup = GCG(1 << (GC::OtherLetter as u32));
    /// (`LC`) The union of UppercaseLetter, LowercaseLetter, and TitlecaseLetter
    pub const CasedLetter: GeneralCategoryGroup = GCG(1 << (GC::UppercaseLetter as u32)
        | 1 << (GC::LowercaseLetter as u32)
        | 1 << (GC::TitlecaseLetter as u32));
    /// (`L`) The union of all letter categories
    pub const Letter: GeneralCategoryGroup = GCG(1 << (GC::UppercaseLetter as u32)
        | 1 << (GC::LowercaseLetter as u32)
        | 1 << (GC::TitlecaseLetter as u32)
        | 1 << (GC::ModifierLetter as u32)
        | 1 << (GC::OtherLetter as u32));

    /// (`Mn`) A nonspacing combining mark (zero advance width)
    pub const NonspacingMark: GeneralCategoryGroup = GCG(1 << (GC::NonspacingMark as u32));
    /// (`Mc`) A spacing combining mark (positive advance width)
    pub const EnclosingMark: GeneralCategoryGroup = GCG(1 << (GC::EnclosingMark as u32));
    /// (`Me`) An enclosing combining mark
    pub const SpacingMark: GeneralCategoryGroup = GCG(1 << (GC::SpacingMark as u32));
    /// (`M`) The union of all mark categories
    pub const Mark: GeneralCategoryGroup = GCG(1 << (GC::NonspacingMark as u32)
        | 1 << (GC::EnclosingMark as u32)
        | 1 << (GC::SpacingMark as u32));

    /// (`Nd`) A decimal digit
    pub const DecimalNumber: GeneralCategoryGroup = GCG(1 << (GC::DecimalNumber as u32));
    /// (`Nl`) A letterlike numeric character
    pub const LetterNumber: GeneralCategoryGroup = GCG(1 << (GC::LetterNumber as u32));
    /// (`No`) A numeric character of other type
    pub const OtherNumber: GeneralCategoryGroup = GCG(1 << (GC::OtherNumber as u32));
    /// (`N`) The union of all number categories
    pub const Number: GeneralCategoryGroup = GCG(1 << (GC::DecimalNumber as u32)
        | 1 << (GC::LetterNumber as u32)
        | 1 << (GC::OtherNumber as u32));

    /// (`Zs`) A space character (of various non-zero widths)
    pub const SpaceSeparator: GeneralCategoryGroup = GCG(1 << (GC::SpaceSeparator as u32));
    /// (`Zl`) U+2028 LINE SEPARATOR only
    pub const LineSeparator: GeneralCategoryGroup = GCG(1 << (GC::LineSeparator as u32));
    /// (`Zp`) U+2029 PARAGRAPH SEPARATOR only
    pub const ParagraphSeparator: GeneralCategoryGroup = GCG(1 << (GC::ParagraphSeparator as u32));
    /// (`Z`) The union of all separator categories
    pub const Separator: GeneralCategoryGroup = GCG(1 << (GC::SpaceSeparator as u32)
        | 1 << (GC::LineSeparator as u32)
        | 1 << (GC::ParagraphSeparator as u32));

    /// (`Cc`) A C0 or C1 control code
    pub const Control: GeneralCategoryGroup = GCG(1 << (GC::Control as u32));
    /// (`Cf`) A format control character
    pub const Format: GeneralCategoryGroup = GCG(1 << (GC::Format as u32));
    /// (`Co`) A private-use character
    pub const PrivateUse: GeneralCategoryGroup = GCG(1 << (GC::PrivateUse as u32));
    /// (`Cs`) A surrogate code point
    pub const Surrogate: GeneralCategoryGroup = GCG(1 << (GC::Surrogate as u32));
    /// (`Cn`) A reserved unassigned code point or a noncharacter
    pub const Unassigned: GeneralCategoryGroup = GCG(1 << (GC::Unassigned as u32));
    /// (`C`) The union of all control code, reserved, and unassigned categories
    pub const Other: GeneralCategoryGroup = GCG(1 << (GC::Control as u32)
        | 1 << (GC::Format as u32)
        | 1 << (GC::PrivateUse as u32)
        | 1 << (GC::Surrogate as u32)
        | 1 << (GC::Unassigned as u32));

    /// (`Pd`) A dash or hyphen punctuation mark
    pub const DashPunctuation: GeneralCategoryGroup = GCG(1 << (GC::DashPunctuation as u32));
    /// (`Ps`) An opening punctuation mark (of a pair)
    pub const OpenPunctuation: GeneralCategoryGroup = GCG(1 << (GC::OpenPunctuation as u32));
    /// (`Pe`) A closing punctuation mark (of a pair)
    pub const ClosePunctuation: GeneralCategoryGroup = GCG(1 << (GC::ClosePunctuation as u32));
    /// (`Pc`) A connecting punctuation mark, like a tie
    pub const ConnectorPunctuation: GeneralCategoryGroup =
        GCG(1 << (GC::ConnectorPunctuation as u32));
    /// (`Pi`) An initial quotation mark
    pub const InitialPunctuation: GeneralCategoryGroup = GCG(1 << (GC::InitialPunctuation as u32));
    /// (`Pf`) A final quotation mark
    pub const FinalPunctuation: GeneralCategoryGroup = GCG(1 << (GC::FinalPunctuation as u32));
    /// (`Po`) A punctuation mark of other type
    pub const OtherPunctuation: GeneralCategoryGroup = GCG(1 << (GC::OtherPunctuation as u32));
    /// (`P`) The union of all punctuation categories
    pub const Punctuation: GeneralCategoryGroup = GCG(1 << (GC::DashPunctuation as u32)
        | 1 << (GC::OpenPunctuation as u32)
        | 1 << (GC::ClosePunctuation as u32)
        | 1 << (GC::ConnectorPunctuation as u32)
        | 1 << (GC::OtherPunctuation as u32)
        | 1 << (GC::InitialPunctuation as u32)
        | 1 << (GC::FinalPunctuation as u32));

    /// (`Sm`) A symbol of mathematical use
    pub const MathSymbol: GeneralCategoryGroup = GCG(1 << (GC::MathSymbol as u32));
    /// (`Sc`) A currency sign
    pub const CurrencySymbol: GeneralCategoryGroup = GCG(1 << (GC::CurrencySymbol as u32));
    /// (`Sk`) A non-letterlike modifier symbol
    pub const ModifierSymbol: GeneralCategoryGroup = GCG(1 << (GC::ModifierSymbol as u32));
    /// (`So`) A symbol of other type
    pub const OtherSymbol: GeneralCategoryGroup = GCG(1 << (GC::OtherSymbol as u32));
    /// (`S`) The union of all symbol categories
    pub const Symbol: GeneralCategoryGroup = GCG(1 << (GC::MathSymbol as u32)
        | 1 << (GC::CurrencySymbol as u32)
        | 1 << (GC::ModifierSymbol as u32)
        | 1 << (GC::OtherSymbol as u32));

    /// Return whether the code point belongs in the provided multi-value category.
    ///
    /// ```
    /// use icu::properties::{maps, GeneralCategory, GeneralCategoryGroup};
    /// use icu_codepointtrie::CodePointTrie;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload =
    ///     maps::get_general_category(&provider)
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let gc = &data_struct.code_point_trie;
    ///
    /// assert_eq!(gc.get('A' as u32), GeneralCategory::UppercaseLetter);
    /// assert!(GeneralCategoryGroup::CasedLetter.contains(gc.get('A' as u32)));
    ///
    /// // U+0B1E ORIYA LETTER NYA
    /// assert_eq!(gc.get('ଞ' as u32), GeneralCategory::OtherLetter);
    /// assert!(GeneralCategoryGroup::Letter.contains(gc.get('ଞ' as u32)));
    /// assert!(!GeneralCategoryGroup::CasedLetter.contains(gc.get('ଞ' as u32)));
    ///
    /// // U+0301 COMBINING ACUTE ACCENT
    /// assert_eq!(gc.get(0x0301), GeneralCategory::NonspacingMark);
    /// assert!(GeneralCategoryGroup::Mark.contains(gc.get(0x0301)));
    /// assert!(!GeneralCategoryGroup::Letter.contains(gc.get(0x0301)));
    ///
    /// assert_eq!(gc.get('0' as u32), GeneralCategory::DecimalNumber);
    /// assert!(GeneralCategoryGroup::Number.contains(gc.get('0' as u32)));
    /// assert!(!GeneralCategoryGroup::Mark.contains(gc.get('0' as u32)));
    ///
    /// assert_eq!(gc.get('(' as u32), GeneralCategory::OpenPunctuation);
    /// assert!(GeneralCategoryGroup::Punctuation.contains(gc.get('(' as u32)));
    /// assert!(!GeneralCategoryGroup::Number.contains(gc.get('(' as u32)));
    ///
    /// // U+2713 CHECK MARK
    /// assert_eq!(gc.get('✓' as u32), GeneralCategory::OtherSymbol);
    /// assert!(GeneralCategoryGroup::Symbol.contains(gc.get('✓' as u32)));
    /// assert!(!GeneralCategoryGroup::Punctuation.contains(gc.get('✓' as u32)));
    ///
    /// assert_eq!(gc.get(' ' as u32), GeneralCategory::SpaceSeparator);
    /// assert!(GeneralCategoryGroup::Separator.contains(gc.get(' ' as u32)));
    /// assert!(!GeneralCategoryGroup::Symbol.contains(gc.get(' ' as u32)));
    ///
    /// // U+E007F CANCEL TAG
    /// assert_eq!(gc.get(0xE007F), GeneralCategory::Format);
    /// assert!(GeneralCategoryGroup::Other.contains(gc.get(0xE007F)));
    /// assert!(!GeneralCategoryGroup::Separator.contains(gc.get(0xE007F)));
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(ScriptULE)]
pub struct Script(pub u16);

#[allow(missing_docs)] // These constants don't need individual documentation.
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

/// Enumerated property East_Asian_Width.
///
/// See "Definition" in UAX #11 for the summary of each property value:
/// <https://www.unicode.org/reports/tr11/#Definitions>
///
/// The numeric value is compatible with `UEastAsianWidth` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(EastAsianWidthULE)]
pub struct EastAsianWidth(pub u8);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl EastAsianWidth {
    pub const Neutral: EastAsianWidth = EastAsianWidth(0); //name="N"
    pub const Ambiguous: EastAsianWidth = EastAsianWidth(1); //name="A"
    pub const Halfwidth: EastAsianWidth = EastAsianWidth(2); //name="H"
    pub const Fullwidth: EastAsianWidth = EastAsianWidth(3); //name="F"
    pub const Narrow: EastAsianWidth = EastAsianWidth(4); //name="Na"
    pub const Wide: EastAsianWidth = EastAsianWidth(5); //name="W"
}

/// Enumerated property Line_Break.
///
/// See "Line Breaking Properties" in UAX #14 for the summary of each property
/// value: <https://www.unicode.org/reports/tr14/#Properties>
///
/// The numeric value is compatible with `ULineBreak` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(LineBreakULE)]
pub struct LineBreak(pub u8);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl LineBreak {
    pub const Unknown: LineBreak = LineBreak(0); // name="XX"
    pub const Ambiguous: LineBreak = LineBreak(1); // name="AI"
    pub const Alphabetic: LineBreak = LineBreak(2); // name="AL"
    pub const BreakBoth: LineBreak = LineBreak(3); // name="B2"
    pub const BreakAfter: LineBreak = LineBreak(4); // name="BA"
    pub const BreakBefore: LineBreak = LineBreak(5); // name="BB"
    pub const MandatoryBreak: LineBreak = LineBreak(6); // name="BK"
    pub const ContingentBreak: LineBreak = LineBreak(7); // name="CB"
    pub const ClosePunctuation: LineBreak = LineBreak(8); // name="CL"
    pub const CombiningMark: LineBreak = LineBreak(9); // name="CM"
    pub const CarriageReturn: LineBreak = LineBreak(10); // name="CR"
    pub const Exclamation: LineBreak = LineBreak(11); // name="EX"
    pub const Glue: LineBreak = LineBreak(12); // name="GL"
    pub const Hyphen: LineBreak = LineBreak(13); // name="HY"
    pub const Ideographic: LineBreak = LineBreak(14); // name="ID"
    pub const Inseparable: LineBreak = LineBreak(15); // name="IN"
    pub const InfixNumeric: LineBreak = LineBreak(16); // name="IS"
    pub const LineFeed: LineBreak = LineBreak(17); // name="LF"
    pub const Nonstarter: LineBreak = LineBreak(18); // name="NS"
    pub const Numeric: LineBreak = LineBreak(19); // name="NU"
    pub const OpenPunctuation: LineBreak = LineBreak(20); // name="OP"
    pub const PostfixNumeric: LineBreak = LineBreak(21); // name="PO"
    pub const PrefixNumeric: LineBreak = LineBreak(22); // name="PR"
    pub const Quotation: LineBreak = LineBreak(23); // name="QU"
    pub const ComplexContext: LineBreak = LineBreak(24); // name="SA"
    pub const Surrogate: LineBreak = LineBreak(25); // name="SG"
    pub const Space: LineBreak = LineBreak(26); // name="SP"
    pub const BreakSymbols: LineBreak = LineBreak(27); // name="SY"
    pub const ZWSpace: LineBreak = LineBreak(28); // name="ZW"
    pub const NextLine: LineBreak = LineBreak(29); // name="NL"
    pub const WordJoiner: LineBreak = LineBreak(30); // name="WJ"
    pub const H2: LineBreak = LineBreak(31); // name="H2"
    pub const H3: LineBreak = LineBreak(32); // name="H3"
    pub const JL: LineBreak = LineBreak(33); // name="JL"
    pub const JT: LineBreak = LineBreak(34); // name="JT"
    pub const JV: LineBreak = LineBreak(35); // name="JV"
    pub const CloseParenthesis: LineBreak = LineBreak(36); // name="CP"
    pub const ConditionalJapaneseStarter: LineBreak = LineBreak(37); // name="CJ"
    pub const HebrewLetter: LineBreak = LineBreak(38); // name="HL"
    pub const RegionalIndicator: LineBreak = LineBreak(39); // name="RI"
    pub const EBase: LineBreak = LineBreak(40); // name="EB"
    pub const EModifier: LineBreak = LineBreak(41); // name="EM"
    pub const ZWJ: LineBreak = LineBreak(42); // name="ZWJ"
}

/// Enumerated property Grapheme_Cluster_Break.
///
/// See "Default Grapheme Cluster Boundary Specification" in UAX #29 for the
/// summary of each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Grapheme_Cluster_Table>
///
/// The numeric value is compatible with `UGraphemeClusterBreak` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(GraphemeClusterBreakULE)]
pub struct GraphemeClusterBreak(pub u8);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl GraphemeClusterBreak {
    pub const Other: GraphemeClusterBreak = GraphemeClusterBreak(0); // name="XX"
    pub const Control: GraphemeClusterBreak = GraphemeClusterBreak(1); // name="CN"
    pub const CR: GraphemeClusterBreak = GraphemeClusterBreak(2); // name="CR"
    pub const Extend: GraphemeClusterBreak = GraphemeClusterBreak(3); // name="EX"
    pub const L: GraphemeClusterBreak = GraphemeClusterBreak(4); // name="L"
    pub const LF: GraphemeClusterBreak = GraphemeClusterBreak(5); // name="LF"
    pub const LV: GraphemeClusterBreak = GraphemeClusterBreak(6); // name="LV"
    pub const LVT: GraphemeClusterBreak = GraphemeClusterBreak(7); // name="LVT"
    pub const T: GraphemeClusterBreak = GraphemeClusterBreak(8); // name="T"
    pub const V: GraphemeClusterBreak = GraphemeClusterBreak(9); // name="V"
    pub const SpacingMark: GraphemeClusterBreak = GraphemeClusterBreak(10); // name="SM"
    pub const Prepend: GraphemeClusterBreak = GraphemeClusterBreak(11); // name="PP"
    pub const RegionalIndicator: GraphemeClusterBreak = GraphemeClusterBreak(12); // name="RI"
    /// This value is obsolete and unused.
    pub const EBase: GraphemeClusterBreak = GraphemeClusterBreak(13); // name="EB"
    /// This value is obsolete and unused.
    pub const EBaseGAZ: GraphemeClusterBreak = GraphemeClusterBreak(14); // name="EBG"
    /// This value is obsolete and unused.
    pub const EModifier: GraphemeClusterBreak = GraphemeClusterBreak(15); // name="EM"
    /// This value is obsolete and unused.
    pub const GlueAfterZwj: GraphemeClusterBreak = GraphemeClusterBreak(16); // name="GAZ"
    pub const ZWJ: GraphemeClusterBreak = GraphemeClusterBreak(17); // name="ZWJ"
}

/// Enumerated property Word_Break.
///
/// See "Default Word Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// The numeric value is compatible with `UWordBreakValues` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(WordBreakULE)]
pub struct WordBreak(pub u8);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl WordBreak {
    pub const Other: WordBreak = WordBreak(0); // name="XX"
    pub const ALetter: WordBreak = WordBreak(1); // name="LE"
    pub const Format: WordBreak = WordBreak(2); // name="FO"
    pub const Katakana: WordBreak = WordBreak(3); // name="KA"
    pub const MidLetter: WordBreak = WordBreak(4); // name="ML"
    pub const MidNum: WordBreak = WordBreak(5); // name="MN"
    pub const Numeric: WordBreak = WordBreak(6); // name="NU"
    pub const ExtendNumLet: WordBreak = WordBreak(7); // name="EX"
    pub const CR: WordBreak = WordBreak(8); // name="CR"
    pub const Extend: WordBreak = WordBreak(9); // name="Extend"
    pub const LF: WordBreak = WordBreak(10); // name="LF"
    pub const MidNumLet: WordBreak = WordBreak(11); // name="MB"
    pub const Newline: WordBreak = WordBreak(12); // name="NL"
    pub const RegionalIndicator: WordBreak = WordBreak(13); // name="RI"
    pub const HebrewLetter: WordBreak = WordBreak(14); // name="HL"
    pub const SingleQuote: WordBreak = WordBreak(15); // name="SQ"
    pub const DoubleQuote: WordBreak = WordBreak(16); // name=DQ
    /// This value is obsolete and unused.
    pub const EBase: WordBreak = WordBreak(17); // name="EB"
    /// This value is obsolete and unused.
    pub const EBaseGAZ: WordBreak = WordBreak(18); // name="EBG"
    /// This value is obsolete and unused.
    pub const EModifier: WordBreak = WordBreak(19); // name="EM"
    /// This value is obsolete and unused.
    pub const GlueAfterZwj: WordBreak = WordBreak(20); // name="GAZ"
    pub const ZWJ: WordBreak = WordBreak(21); // name="ZWJ"
    pub const WSegSpace: WordBreak = WordBreak(22); // name="WSegSpace"
}

/// Enumerated property Sentence_Break.
/// See "Default Sentence Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// The numeric value is compatible with `USentenceBreak` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(SentenceBreakULE)]
pub struct SentenceBreak(pub u8);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl SentenceBreak {
    pub const Other: SentenceBreak = SentenceBreak(0); // name="XX"
    pub const ATerm: SentenceBreak = SentenceBreak(1); // name="AT"
    pub const Close: SentenceBreak = SentenceBreak(2); // name="CL"
    pub const Format: SentenceBreak = SentenceBreak(3); // name="FO"
    pub const Lower: SentenceBreak = SentenceBreak(4); // name="LO"
    pub const Numeric: SentenceBreak = SentenceBreak(5); // name="NU"
    pub const OLetter: SentenceBreak = SentenceBreak(6); // name="LE"
    pub const Sep: SentenceBreak = SentenceBreak(7); // name="SE"
    pub const Sp: SentenceBreak = SentenceBreak(8); // name="SP"
    pub const STerm: SentenceBreak = SentenceBreak(9); // name="ST"
    pub const Upper: SentenceBreak = SentenceBreak(10); // name="UP"
    pub const CR: SentenceBreak = SentenceBreak(11); // name="CR"
    pub const Extend: SentenceBreak = SentenceBreak(12); // name="EX"
    pub const LF: SentenceBreak = SentenceBreak(13); // name="LF"
    pub const SContinue: SentenceBreak = SentenceBreak(14); // name="SC"
}

/// Property Canonical_Combining_Class.
/// See UAX #15:
/// <https://www.unicode.org/reports/tr15/>.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
#[zerovec::make_ule(CanonicalCombiningClassULE)]
pub struct CanonicalCombiningClass(pub u8);

// These constant names come from PropertyValueAliases.txt
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl CanonicalCombiningClass {
    pub const NotReordered: CanonicalCombiningClass = CanonicalCombiningClass(0); // name="NR"
    pub const Overlay: CanonicalCombiningClass = CanonicalCombiningClass(1); // name="OV"
    pub const HanReading: CanonicalCombiningClass = CanonicalCombiningClass(6); // name="HANR"
    pub const Nukta: CanonicalCombiningClass = CanonicalCombiningClass(7); // name="NK"
    pub const KanaVoicing: CanonicalCombiningClass = CanonicalCombiningClass(8); // name="KV"
    pub const Virama: CanonicalCombiningClass = CanonicalCombiningClass(9); // name="VR"
    pub const CCC10: CanonicalCombiningClass = CanonicalCombiningClass(10); // name="CCC10"
    pub const CCC11: CanonicalCombiningClass = CanonicalCombiningClass(11); // name="CCC11"
    pub const CCC12: CanonicalCombiningClass = CanonicalCombiningClass(12); // name="CCC12"
    pub const CCC13: CanonicalCombiningClass = CanonicalCombiningClass(13); // name="CCC13"
    pub const CCC14: CanonicalCombiningClass = CanonicalCombiningClass(14); // name="CCC14"
    pub const CCC15: CanonicalCombiningClass = CanonicalCombiningClass(15); // name="CCC15"
    pub const CCC16: CanonicalCombiningClass = CanonicalCombiningClass(16); // name="CCC16"
    pub const CCC17: CanonicalCombiningClass = CanonicalCombiningClass(17); // name="CCC17"
    pub const CCC18: CanonicalCombiningClass = CanonicalCombiningClass(18); // name="CCC18"
    pub const CCC19: CanonicalCombiningClass = CanonicalCombiningClass(19); // name="CCC19"
    pub const CCC20: CanonicalCombiningClass = CanonicalCombiningClass(20); // name="CCC20"
    pub const CCC21: CanonicalCombiningClass = CanonicalCombiningClass(21); // name="CCC21"
    pub const CCC22: CanonicalCombiningClass = CanonicalCombiningClass(22); // name="CCC22"
    pub const CCC23: CanonicalCombiningClass = CanonicalCombiningClass(23); // name="CCC23"
    pub const CCC24: CanonicalCombiningClass = CanonicalCombiningClass(24); // name="CCC24"
    pub const CCC25: CanonicalCombiningClass = CanonicalCombiningClass(25); // name="CCC25"
    pub const CCC26: CanonicalCombiningClass = CanonicalCombiningClass(26); // name="CCC26"
    pub const CCC27: CanonicalCombiningClass = CanonicalCombiningClass(27); // name="CCC27"
    pub const CCC28: CanonicalCombiningClass = CanonicalCombiningClass(28); // name="CCC28"
    pub const CCC29: CanonicalCombiningClass = CanonicalCombiningClass(29); // name="CCC29"
    pub const CCC30: CanonicalCombiningClass = CanonicalCombiningClass(30); // name="CCC30"
    pub const CCC31: CanonicalCombiningClass = CanonicalCombiningClass(31); // name="CCC31"
    pub const CCC32: CanonicalCombiningClass = CanonicalCombiningClass(32); // name="CCC32"
    pub const CCC33: CanonicalCombiningClass = CanonicalCombiningClass(33); // name="CCC33"
    pub const CCC34: CanonicalCombiningClass = CanonicalCombiningClass(34); // name="CCC34"
    pub const CCC35: CanonicalCombiningClass = CanonicalCombiningClass(35); // name="CCC35"
    pub const CCC36: CanonicalCombiningClass = CanonicalCombiningClass(36); // name="CCC36"
    pub const CCC84: CanonicalCombiningClass = CanonicalCombiningClass(84); // name="CCC84"
    pub const CCC91: CanonicalCombiningClass = CanonicalCombiningClass(91); // name="CCC91"
    pub const CCC103: CanonicalCombiningClass = CanonicalCombiningClass(103); // name="CCC103"
    pub const CCC107: CanonicalCombiningClass = CanonicalCombiningClass(107); // name="CCC107"
    pub const CCC118: CanonicalCombiningClass = CanonicalCombiningClass(118); // name="CCC118"
    pub const CCC122: CanonicalCombiningClass = CanonicalCombiningClass(122); // name="CCC122"
    pub const CCC129: CanonicalCombiningClass = CanonicalCombiningClass(129); // name="CCC129"
    pub const CCC130: CanonicalCombiningClass = CanonicalCombiningClass(130); // name="CCC130"
    pub const CCC132: CanonicalCombiningClass = CanonicalCombiningClass(132); // name="CCC132"
    pub const CCC133: CanonicalCombiningClass = CanonicalCombiningClass(133); // name="CCC133" // RESERVED
    pub const AttachedBelowLeft: CanonicalCombiningClass = CanonicalCombiningClass(200); // name="ATBL"
    pub const AttachedBelow: CanonicalCombiningClass = CanonicalCombiningClass(202); // name="ATB"
    pub const AttachedAbove: CanonicalCombiningClass = CanonicalCombiningClass(214); // name="ATA"
    pub const AttachedAboveRight: CanonicalCombiningClass = CanonicalCombiningClass(216); // name="ATAR"
    pub const BelowLeft: CanonicalCombiningClass = CanonicalCombiningClass(218); // name="BL"
    pub const Below: CanonicalCombiningClass = CanonicalCombiningClass(220); // name="B"
    pub const BelowRight: CanonicalCombiningClass = CanonicalCombiningClass(222); // name="BR"
    pub const Left: CanonicalCombiningClass = CanonicalCombiningClass(224); // name="L"
    pub const Right: CanonicalCombiningClass = CanonicalCombiningClass(226); // name="R"
    pub const AboveLeft: CanonicalCombiningClass = CanonicalCombiningClass(228); // name="AL"
    pub const Above: CanonicalCombiningClass = CanonicalCombiningClass(230); // name="A"
    pub const AboveRight: CanonicalCombiningClass = CanonicalCombiningClass(232); // name="AR"
    pub const DoubleBelow: CanonicalCombiningClass = CanonicalCombiningClass(233); // name="DB"
    pub const DoubleAbove: CanonicalCombiningClass = CanonicalCombiningClass(234); // name="DA"
    pub const IotaSubscript: CanonicalCombiningClass = CanonicalCombiningClass(240); // name="IS"
}
