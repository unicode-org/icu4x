// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_uniset::enum_props::*;
use std::str::FromStr;
use tinystr::TinyStr16;

//
// Single getter function for enumerated property name:
// Enum prop name string -> Rust enum
//

fn get_enum_property_enum(name: &str) -> Option<EnumeratedProperty> {
    Some(match name {
        "bc" => EnumeratedProperty::BidiClass,
        "bpt" => EnumeratedProperty::BidiPairedBracketType,
        "ccc" => EnumeratedProperty::CanonicalCombiningClass,
        "dt" => EnumeratedProperty::DecompositionType,
        "ea" => EnumeratedProperty::EastAsianWidth,
        "gc" => EnumeratedProperty::GeneralCategory,
        "GCB" => EnumeratedProperty::GraphemeClusterBreak,
        "hst" => EnumeratedProperty::HangulSyllableType,
        "InPC" => EnumeratedProperty::IndicPositionalCategory,
        "InSC" => EnumeratedProperty::IndicSyllabicCategory,
        "jg" => EnumeratedProperty::JoiningGroup,
        "jt" => EnumeratedProperty::JoiningType,
        "lb" => EnumeratedProperty::LineBreak,
        "lccc" => EnumeratedProperty::LeadCanonicalCombiningClass,
        "NFC_QC" => EnumeratedProperty::NFCQuickCheck,
        "NFD_QC" => EnumeratedProperty::NFDQuickCheck,
        "NFKC_QC" => EnumeratedProperty::NFKCQuickCheck,
        "NFKD_QC" => EnumeratedProperty::NFKDQuickCheck,
        "nt" => EnumeratedProperty::NumericType,
        "SB" => EnumeratedProperty::SentenceBreak,
        "tccc" => EnumeratedProperty::TrailCanonicalCombiningClass,
        "vo" => EnumeratedProperty::VerticalOrientation,
        "WB" => EnumeratedProperty::WordBreak,
        _ => return None,
    })
}

//
// Getter function per enumerated property:
// Enum prop val string -> Rust enum
//

fn get_bidi_class_enum(name: &str) -> Option<BidiClass> {
    Some(match name {
        "AL" => BidiClass::ArabicLetter,
        "AN" => BidiClass::ArabicNumber,
        "B" => BidiClass::ParagraphSeparator,
        "BN" => BidiClass::BoundaryNeutral,
        "CS" => BidiClass::CommonSeparator,
        "EN" => BidiClass::EuropeanNumber,
        "ES" => BidiClass::EuropeanSeparator,
        "ET" => BidiClass::EuropeanTerminator,
        "FSI" => BidiClass::FirstStrongIsolate,
        "L" => BidiClass::LeftToRight,
        "LRE" => BidiClass::LeftToRightEmbedding,
        "LRI" => BidiClass::LeftToRightIsolate,
        "LRO" => BidiClass::LeftToRightOverride,
        "NSM" => BidiClass::NonspacingMark,
        "ON" => BidiClass::OtherNeutral,
        "PDF" => BidiClass::PopDirectionalFormat,
        "PDI" => BidiClass::PopDirectionalIsolate,
        "R" => BidiClass::RightToLeft,
        "RLE" => BidiClass::RightToLeftEmbedding,
        "RLI" => BidiClass::RightToLeftIsolate,
        "RLO" => BidiClass::RightToLeftOverride,
        "S" => BidiClass::SegmentSeparator,
        "WS" => BidiClass::WhiteSpace,
        _ => return None,
    })
}

fn get_bidi_paired_bracket_type_enum(name: &str) -> Option<BidiPairedBracketType> {
    Some(match name {
        "c" => BidiPairedBracketType::Close,
        "n" => BidiPairedBracketType::None,
        "o" => BidiPairedBracketType::Open,
        _ => return None,
    })
}

fn get_canonical_combining_class_enum(name: &str) -> Option<CanonicalCombiningClass> {
    Some(match name {
        "0" => CanonicalCombiningClass::NotReordered,
        "1" => CanonicalCombiningClass::Overlay,
        "10" => CanonicalCombiningClass::CCC10,
        "103" => CanonicalCombiningClass::CCC103,
        "107" => CanonicalCombiningClass::CCC107,
        "11" => CanonicalCombiningClass::CCC11,
        "118" => CanonicalCombiningClass::CCC118,
        "12" => CanonicalCombiningClass::CCC12,
        "122" => CanonicalCombiningClass::CCC122,
        "129" => CanonicalCombiningClass::CCC129,
        "13" => CanonicalCombiningClass::CCC13,
        "130" => CanonicalCombiningClass::CCC130,
        "132" => CanonicalCombiningClass::CCC132,
        "133" => CanonicalCombiningClass::CCC133,
        "14" => CanonicalCombiningClass::CCC14,
        "15" => CanonicalCombiningClass::CCC15,
        "16" => CanonicalCombiningClass::CCC16,
        "17" => CanonicalCombiningClass::CCC17,
        "18" => CanonicalCombiningClass::CCC18,
        "19" => CanonicalCombiningClass::CCC19,
        "20" => CanonicalCombiningClass::CCC20,
        "200" => CanonicalCombiningClass::AttachedBelowLeft,
        "202" => CanonicalCombiningClass::AttachedBelow,
        "21" => CanonicalCombiningClass::CCC21,
        "214" => CanonicalCombiningClass::AttachedAbove,
        "216" => CanonicalCombiningClass::AttachedAboveRight,
        "218" => CanonicalCombiningClass::BelowLeft,
        "22" => CanonicalCombiningClass::CCC22,
        "220" => CanonicalCombiningClass::Below,
        "222" => CanonicalCombiningClass::BelowRight,
        "224" => CanonicalCombiningClass::Left,
        "226" => CanonicalCombiningClass::Right,
        "228" => CanonicalCombiningClass::AboveLeft,
        "23" => CanonicalCombiningClass::CCC23,
        "230" => CanonicalCombiningClass::Above,
        "232" => CanonicalCombiningClass::AboveRight,
        "233" => CanonicalCombiningClass::DoubleBelow,
        "234" => CanonicalCombiningClass::DoubleAbove,
        "24" => CanonicalCombiningClass::CCC24,
        "240" => CanonicalCombiningClass::IotaSubscript,
        "25" => CanonicalCombiningClass::CCC25,
        "26" => CanonicalCombiningClass::CCC26,
        "27" => CanonicalCombiningClass::CCC27,
        "28" => CanonicalCombiningClass::CCC28,
        "29" => CanonicalCombiningClass::CCC29,
        "30" => CanonicalCombiningClass::CCC30,
        "31" => CanonicalCombiningClass::CCC31,
        "32" => CanonicalCombiningClass::CCC32,
        "33" => CanonicalCombiningClass::CCC33,
        "34" => CanonicalCombiningClass::CCC34,
        "35" => CanonicalCombiningClass::CCC35,
        "36" => CanonicalCombiningClass::CCC36,
        "6" => CanonicalCombiningClass::HanReading,
        "7" => CanonicalCombiningClass::Nukta,
        "8" => CanonicalCombiningClass::KanaVoicing,
        "84" => CanonicalCombiningClass::CCC84,
        "9" => CanonicalCombiningClass::Virama,
        "91" => CanonicalCombiningClass::CCC91,
        _ => return None,
    })
}

fn get_decomposition_type_enum(name: &str) -> Option<DecompositionType> {
    Some(match name {
        "Can" => DecompositionType::Can,
        "Com" => DecompositionType::Com,
        "Enc" => DecompositionType::Enc,
        "Fin" => DecompositionType::Fin,
        "Font" => DecompositionType::Font,
        "Fra" => DecompositionType::Fra,
        "Init" => DecompositionType::Init,
        "Iso" => DecompositionType::Iso,
        "Med" => DecompositionType::Med,
        "Nar" => DecompositionType::Nar,
        "Nb" => DecompositionType::Nb,
        "None" => DecompositionType::None,
        "Sml" => DecompositionType::Sml,
        "Sqr" => DecompositionType::Sqr,
        "Sub" => DecompositionType::Sub,
        "Sup" => DecompositionType::Sup,
        "Vert" => DecompositionType::Vert,
        "Wide" => DecompositionType::Wide,
        _ => return None,
    })
}

fn get_east_asian_width_enum(name: &str) -> Option<EastAsianWidth> {
    Some(match name {
        "A" => EastAsianWidth::Ambiguous,
        "F" => EastAsianWidth::Fullwidth,
        "H" => EastAsianWidth::Halfwidth,
        "N" => EastAsianWidth::Neutral,
        "Na" => EastAsianWidth::Narrow,
        "W" => EastAsianWidth::Wide,
        _ => return None,
    })
}

fn get_general_category_enum(name: &str) -> Option<GeneralCategory> {
    Some(match name {
        "C" => GeneralCategory::Other,
        "Cc" => GeneralCategory::Cntrl,
        "Cf" => GeneralCategory::Format,
        "Cn" => GeneralCategory::Unassigned,
        "Co" => GeneralCategory::PrivateUse,
        "Cs" => GeneralCategory::Surrogate,
        "L" => GeneralCategory::Letter,
        "LC" => GeneralCategory::CasedLetter,
        "Ll" => GeneralCategory::LowercaseLetter,
        "Lm" => GeneralCategory::ModifierLetter,
        "Lo" => GeneralCategory::OtherLetter,
        "Lt" => GeneralCategory::TitlecaseLetter,
        "Lu" => GeneralCategory::UppercaseLetter,
        "M" => GeneralCategory::CombiningMark,
        "Mc" => GeneralCategory::SpacingMark,
        "Me" => GeneralCategory::EnclosingMark,
        "Mn" => GeneralCategory::NonspacingMark,
        "N" => GeneralCategory::Number,
        "Nd" => GeneralCategory::Digit,
        "Nl" => GeneralCategory::LetterNumber,
        "No" => GeneralCategory::OtherNumber,
        "P" => GeneralCategory::Punct,
        "Pc" => GeneralCategory::ConnectorPunctuation,
        "Pd" => GeneralCategory::DashPunctuation,
        "Pe" => GeneralCategory::ClosePunctuation,
        "Pf" => GeneralCategory::FinalPunctuation,
        "Pi" => GeneralCategory::InitialPunctuation,
        "Po" => GeneralCategory::OtherPunctuation,
        "Ps" => GeneralCategory::OpenPunctuation,
        "S" => GeneralCategory::Symbol,
        "Sc" => GeneralCategory::CurrencySymbol,
        "Sk" => GeneralCategory::ModifierSymbol,
        "Sm" => GeneralCategory::MathSymbol,
        "So" => GeneralCategory::OtherSymbol,
        "Z" => GeneralCategory::Separator,
        "Zl" => GeneralCategory::LineSeparator,
        "Zp" => GeneralCategory::ParagraphSeparator,
        "Zs" => GeneralCategory::SpaceSeparator,
        _ => return None,
    })
}

fn get_grapheme_cluster_break_enum(name: &str) -> Option<GraphemeClusterBreak> {
    Some(match name {
        "CN" => GraphemeClusterBreak::Control,
        "CR" => GraphemeClusterBreak::CR,
        "EB" => GraphemeClusterBreak::EBase,
        "EBG" => GraphemeClusterBreak::EBaseGAZ,
        "EM" => GraphemeClusterBreak::EModifier,
        "EX" => GraphemeClusterBreak::Extend,
        "GAZ" => GraphemeClusterBreak::GlueAfterZwj,
        "L" => GraphemeClusterBreak::L,
        "LF" => GraphemeClusterBreak::LF,
        "LV" => GraphemeClusterBreak::LV,
        "LVT" => GraphemeClusterBreak::LVT,
        "PP" => GraphemeClusterBreak::Prepend,
        "RI" => GraphemeClusterBreak::RegionalIndicator,
        "SM" => GraphemeClusterBreak::SpacingMark,
        "T" => GraphemeClusterBreak::T,
        "V" => GraphemeClusterBreak::V,
        "XX" => GraphemeClusterBreak::Other,
        "ZWJ" => GraphemeClusterBreak::ZWJ,
        _ => return None,
    })
}

fn get_hangul_syllable_type_enum(name: &str) -> Option<HangulSyllableType> {
    Some(match name {
        "L" => HangulSyllableType::LeadingJamo,
        "LV" => HangulSyllableType::LVSyllable,
        "LVT" => HangulSyllableType::LVTSyllable,
        "NA" => HangulSyllableType::NotApplicable,
        "T" => HangulSyllableType::TrailingJamo,
        "V" => HangulSyllableType::VowelJamo,
        _ => return None,
    })
}

fn get_indic_positional_category_enum(name: &str) -> Option<IndicPositionalCategory> {
    Some(match name {
        "Bottom" => IndicPositionalCategory::Bottom,
        "Bottom_And_Left" => IndicPositionalCategory::BottomAndLeft,
        "Bottom_And_Right" => IndicPositionalCategory::BottomAndRight,
        "Left" => IndicPositionalCategory::Left,
        "Left_And_Right" => IndicPositionalCategory::LeftAndRight,
        "NA" => IndicPositionalCategory::NA,
        "Overstruck" => IndicPositionalCategory::Overstruck,
        "Right" => IndicPositionalCategory::Right,
        "Top" => IndicPositionalCategory::Top,
        "Top_And_Bottom" => IndicPositionalCategory::TopAndBottom,
        "Top_And_Bottom_And_Left" => IndicPositionalCategory::TopAndBottomAndLeft,
        "Top_And_Bottom_And_Right" => IndicPositionalCategory::TopAndBottomAndRight,
        "Top_And_Left" => IndicPositionalCategory::TopAndLeft,
        "Top_And_Left_And_Right" => IndicPositionalCategory::TopAndLeftAndRight,
        "Top_And_Right" => IndicPositionalCategory::TopAndRight,
        "Visual_Order_Left" => IndicPositionalCategory::VisualOrderLeft,
        _ => return None,
    })
}

fn get_indic_syllabic_category_enum(name: &str) -> Option<IndicSyllabicCategory> {
    Some(match name {
        "Avagraha" => IndicSyllabicCategory::Avagraha,
        "Bindu" => IndicSyllabicCategory::Bindu,
        "Brahmi_Joining_Number" => IndicSyllabicCategory::BrahmiJoiningNumber,
        "Cantillation_Mark" => IndicSyllabicCategory::CantillationMark,
        "Consonant" => IndicSyllabicCategory::Consonant,
        "Consonant_Dead" => IndicSyllabicCategory::ConsonantDead,
        "Consonant_Final" => IndicSyllabicCategory::ConsonantFinal,
        "Consonant_Head_Letter" => IndicSyllabicCategory::ConsonantHeadLetter,
        "Consonant_Initial_Postfixed" => IndicSyllabicCategory::ConsonantInitialPostfixed,
        "Consonant_Killer" => IndicSyllabicCategory::ConsonantKiller,
        "Consonant_Medial" => IndicSyllabicCategory::ConsonantMedial,
        "Consonant_Placeholder" => IndicSyllabicCategory::ConsonantPlaceholder,
        "Consonant_Preceding_Repha" => IndicSyllabicCategory::ConsonantPrecedingRepha,
        "Consonant_Prefixed" => IndicSyllabicCategory::ConsonantPrefixed,
        "Consonant_Subjoined" => IndicSyllabicCategory::ConsonantSubjoined,
        "Consonant_Succeeding_Repha" => IndicSyllabicCategory::ConsonantSucceedingRepha,
        "Consonant_With_Stacker" => IndicSyllabicCategory::ConsonantWithStacker,
        "Gemination_Mark" => IndicSyllabicCategory::GeminationMark,
        "Invisible_Stacker" => IndicSyllabicCategory::InvisibleStacker,
        "Joiner" => IndicSyllabicCategory::Joiner,
        "Modifying_Letter" => IndicSyllabicCategory::ModifyingLetter,
        "Non_Joiner" => IndicSyllabicCategory::NonJoiner,
        "Nukta" => IndicSyllabicCategory::Nukta,
        "Number" => IndicSyllabicCategory::Number,
        "Number_Joiner" => IndicSyllabicCategory::NumberJoiner,
        "Other" => IndicSyllabicCategory::Other,
        "Pure_Killer" => IndicSyllabicCategory::PureKiller,
        "Register_Shifter" => IndicSyllabicCategory::RegisterShifter,
        "Syllable_Modifier" => IndicSyllabicCategory::SyllableModifier,
        "Tone_Letter" => IndicSyllabicCategory::ToneLetter,
        "Tone_Mark" => IndicSyllabicCategory::ToneMark,
        "Virama" => IndicSyllabicCategory::Virama,
        "Visarga" => IndicSyllabicCategory::Visarga,
        "Vowel" => IndicSyllabicCategory::Vowel,
        "Vowel_Dependent" => IndicSyllabicCategory::VowelDependent,
        "Vowel_Independent" => IndicSyllabicCategory::VowelIndependent,
        _ => return None,
    })
}

fn get_joining_group_enum(name: &str) -> Option<JoiningGroup> {
    Some(match name {
        "African_Feh" => JoiningGroup::AfricanFeh,
        "African_Noon" => JoiningGroup::AfricanNoon,
        "African_Qaf" => JoiningGroup::AfricanQaf,
        "Ain" => JoiningGroup::Ain,
        "Alaph" => JoiningGroup::Alaph,
        "Alef" => JoiningGroup::Alef,
        "Beh" => JoiningGroup::Beh,
        "Beth" => JoiningGroup::Beth,
        "Burushaski_Yeh_Barree" => JoiningGroup::BurushaskiYehBarree,
        "Dal" => JoiningGroup::Dal,
        "Dalath_Rish" => JoiningGroup::DalathRish,
        "E" => JoiningGroup::E,
        "Farsi_Yeh" => JoiningGroup::FarsiYeh,
        "Fe" => JoiningGroup::Fe,
        "Feh" => JoiningGroup::Feh,
        "Final_Semkath" => JoiningGroup::FinalSemkath,
        "Gaf" => JoiningGroup::Gaf,
        "Gamal" => JoiningGroup::Gamal,
        "Hah" => JoiningGroup::Hah,
        "Hanifi_Rohingya_Kinna_Ya" => JoiningGroup::HanifiRohingyaKinnaYa,
        "Hanifi_Rohingya_Pa" => JoiningGroup::HanifiRohingyaPa,
        "He" => JoiningGroup::He,
        "Heh" => JoiningGroup::Heh,
        "Heh_Goal" => JoiningGroup::HehGoal,
        "Heth" => JoiningGroup::Heth,
        "Kaf" => JoiningGroup::Kaf,
        "Kaph" => JoiningGroup::Kaph,
        "Khaph" => JoiningGroup::Khaph,
        "Knotted_Heh" => JoiningGroup::KnottedHeh,
        "Lam" => JoiningGroup::Lam,
        "Lamadh" => JoiningGroup::Lamadh,
        "Malayalam_Bha" => JoiningGroup::MalayalamBha,
        "Malayalam_Ja" => JoiningGroup::MalayalamJa,
        "Malayalam_Lla" => JoiningGroup::MalayalamLla,
        "Malayalam_Llla" => JoiningGroup::MalayalamLlla,
        "Malayalam_Nga" => JoiningGroup::MalayalamNga,
        "Malayalam_Nna" => JoiningGroup::MalayalamNna,
        "Malayalam_Nnna" => JoiningGroup::MalayalamNnna,
        "Malayalam_Nya" => JoiningGroup::MalayalamNya,
        "Malayalam_Ra" => JoiningGroup::MalayalamRa,
        "Malayalam_Ssa" => JoiningGroup::MalayalamSsa,
        "Malayalam_Tta" => JoiningGroup::MalayalamTta,
        "Manichaean_Aleph" => JoiningGroup::ManichaeanAleph,
        "Manichaean_Ayin" => JoiningGroup::ManichaeanAyin,
        "Manichaean_Beth" => JoiningGroup::ManichaeanBeth,
        "Manichaean_Daleth" => JoiningGroup::ManichaeanDaleth,
        "Manichaean_Dhamedh" => JoiningGroup::ManichaeanDhamedh,
        "Manichaean_Five" => JoiningGroup::ManichaeanFive,
        "Manichaean_Gimel" => JoiningGroup::ManichaeanGimel,
        "Manichaean_Heth" => JoiningGroup::ManichaeanHeth,
        "Manichaean_Hundred" => JoiningGroup::ManichaeanHundred,
        "Manichaean_Kaph" => JoiningGroup::ManichaeanKaph,
        "Manichaean_Lamedh" => JoiningGroup::ManichaeanLamedh,
        "Manichaean_Mem" => JoiningGroup::ManichaeanMem,
        "Manichaean_Nun" => JoiningGroup::ManichaeanNun,
        "Manichaean_One" => JoiningGroup::ManichaeanOne,
        "Manichaean_Pe" => JoiningGroup::ManichaeanPe,
        "Manichaean_Qoph" => JoiningGroup::ManichaeanQoph,
        "Manichaean_Resh" => JoiningGroup::ManichaeanResh,
        "Manichaean_Sadhe" => JoiningGroup::ManichaeanSadhe,
        "Manichaean_Samekh" => JoiningGroup::ManichaeanSamekh,
        "Manichaean_Taw" => JoiningGroup::ManichaeanTaw,
        "Manichaean_Ten" => JoiningGroup::ManichaeanTen,
        "Manichaean_Teth" => JoiningGroup::ManichaeanTeth,
        "Manichaean_Thamedh" => JoiningGroup::ManichaeanThamedh,
        "Manichaean_Twenty" => JoiningGroup::ManichaeanTwenty,
        "Manichaean_Waw" => JoiningGroup::ManichaeanWaw,
        "Manichaean_Yodh" => JoiningGroup::ManichaeanYodh,
        "Manichaean_Zayin" => JoiningGroup::ManichaeanZayin,
        "Meem" => JoiningGroup::Meem,
        "Mim" => JoiningGroup::Mim,
        "No_Joining_Group" => JoiningGroup::NoJoiningGroup,
        "Noon" => JoiningGroup::Noon,
        "Nun" => JoiningGroup::Nun,
        "Nya" => JoiningGroup::Nya,
        "Pe" => JoiningGroup::Pe,
        "Qaf" => JoiningGroup::Qaf,
        "Qaph" => JoiningGroup::Qaph,
        "Reh" => JoiningGroup::Reh,
        "Reversed_Pe" => JoiningGroup::ReversedPe,
        "Rohingya_Yeh" => JoiningGroup::RohingyaYeh,
        "Sad" => JoiningGroup::Sad,
        "Sadhe" => JoiningGroup::Sadhe,
        "Seen" => JoiningGroup::Seen,
        "Semkath" => JoiningGroup::Semkath,
        "Shin" => JoiningGroup::Shin,
        "Straight_Waw" => JoiningGroup::StraightWaw,
        "Swash_Kaf" => JoiningGroup::SwashKaf,
        "Syriac_Waw" => JoiningGroup::SyriacWaw,
        "Tah" => JoiningGroup::Tah,
        "Taw" => JoiningGroup::Taw,
        "Teh_Marbuta" => JoiningGroup::TehMarbuta,
        "Teh_Marbuta_Goal" => JoiningGroup::TehMarbutaGoal,
        "Teth" => JoiningGroup::Teth,
        "Waw" => JoiningGroup::Waw,
        "Yeh" => JoiningGroup::Yeh,
        "Yeh_Barree" => JoiningGroup::YehBarree,
        "Yeh_With_Tail" => JoiningGroup::YehWithTail,
        "Yudh" => JoiningGroup::Yudh,
        "Yudh_He" => JoiningGroup::YudhHe,
        "Zain" => JoiningGroup::Zain,
        "Zhain" => JoiningGroup::Zhain,
        _ => return None,
    })
}

fn get_joining_type_enum(name: &str) -> Option<JoiningType> {
    Some(match name {
        "C" => JoiningType::JoinCausing,
        "D" => JoiningType::DualJoining,
        "L" => JoiningType::LeftJoining,
        "R" => JoiningType::RightJoining,
        "T" => JoiningType::Transparent,
        "U" => JoiningType::NonJoining,
        _ => return None,
    })
}

fn get_line_break_enum(name: &str) -> Option<LineBreak> {
    Some(match name {
        "AI" => LineBreak::Ambiguous,
        "AL" => LineBreak::Alphabetic,
        "B2" => LineBreak::BreakBoth,
        "BA" => LineBreak::BreakAfter,
        "BB" => LineBreak::BreakBefore,
        "BK" => LineBreak::MandatoryBreak,
        "CB" => LineBreak::ContingentBreak,
        "CJ" => LineBreak::ConditionalJapaneseStarter,
        "CL" => LineBreak::ClosePunctuation,
        "CM" => LineBreak::CombiningMark,
        "CP" => LineBreak::CloseParenthesis,
        "CR" => LineBreak::CarriageReturn,
        "EB" => LineBreak::EBase,
        "EM" => LineBreak::EModifier,
        "EX" => LineBreak::Exclamation,
        "GL" => LineBreak::Glue,
        "H2" => LineBreak::H2,
        "H3" => LineBreak::H3,
        "HL" => LineBreak::HebrewLetter,
        "HY" => LineBreak::Hyphen,
        "ID" => LineBreak::Ideographic,
        "IN" => LineBreak::Inseperable,
        "IS" => LineBreak::InfixNumeric,
        "JL" => LineBreak::JL,
        "JT" => LineBreak::JT,
        "JV" => LineBreak::JV,
        "LF" => LineBreak::LineFeed,
        "NL" => LineBreak::NextLine,
        "NS" => LineBreak::Nonstarter,
        "NU" => LineBreak::Numeric,
        "OP" => LineBreak::OpenPunctuation,
        "PO" => LineBreak::PostfixNumeric,
        "PR" => LineBreak::PrefixNumeric,
        "QU" => LineBreak::Quotation,
        "RI" => LineBreak::RegionalIndicator,
        "SA" => LineBreak::ComplexContext,
        "SG" => LineBreak::Surrogate,
        "SP" => LineBreak::Space,
        "SY" => LineBreak::BreakSymbols,
        "WJ" => LineBreak::WordJoiner,
        "XX" => LineBreak::Unknown,
        "ZW" => LineBreak::ZWSpace,
        "ZWJ" => LineBreak::ZWJ,
        _ => return None,
    })
}

fn get_lead_canonical_combining_class_enum(name: &str) -> Option<LeadCanonicalCombiningClass> {
    Some(match name {
        "0" => LeadCanonicalCombiningClass::NotReordered,
        "1" => LeadCanonicalCombiningClass::Overlay,
        "10" => LeadCanonicalCombiningClass::CCC10,
        "103" => LeadCanonicalCombiningClass::CCC103,
        "107" => LeadCanonicalCombiningClass::CCC107,
        "11" => LeadCanonicalCombiningClass::CCC11,
        "118" => LeadCanonicalCombiningClass::CCC118,
        "12" => LeadCanonicalCombiningClass::CCC12,
        "122" => LeadCanonicalCombiningClass::CCC122,
        "129" => LeadCanonicalCombiningClass::CCC129,
        "13" => LeadCanonicalCombiningClass::CCC13,
        "130" => LeadCanonicalCombiningClass::CCC130,
        "132" => LeadCanonicalCombiningClass::CCC132,
        "133" => LeadCanonicalCombiningClass::CCC133,
        "14" => LeadCanonicalCombiningClass::CCC14,
        "15" => LeadCanonicalCombiningClass::CCC15,
        "16" => LeadCanonicalCombiningClass::CCC16,
        "17" => LeadCanonicalCombiningClass::CCC17,
        "18" => LeadCanonicalCombiningClass::CCC18,
        "19" => LeadCanonicalCombiningClass::CCC19,
        "20" => LeadCanonicalCombiningClass::CCC20,
        "200" => LeadCanonicalCombiningClass::AttachedBelowLeft,
        "202" => LeadCanonicalCombiningClass::AttachedBelow,
        "21" => LeadCanonicalCombiningClass::CCC21,
        "214" => LeadCanonicalCombiningClass::AttachedAbove,
        "216" => LeadCanonicalCombiningClass::AttachedAboveRight,
        "218" => LeadCanonicalCombiningClass::BelowLeft,
        "22" => LeadCanonicalCombiningClass::CCC22,
        "220" => LeadCanonicalCombiningClass::Below,
        "222" => LeadCanonicalCombiningClass::BelowRight,
        "224" => LeadCanonicalCombiningClass::Left,
        "226" => LeadCanonicalCombiningClass::Right,
        "228" => LeadCanonicalCombiningClass::AboveLeft,
        "23" => LeadCanonicalCombiningClass::CCC23,
        "230" => LeadCanonicalCombiningClass::Above,
        "232" => LeadCanonicalCombiningClass::AboveRight,
        "233" => LeadCanonicalCombiningClass::DoubleBelow,
        "234" => LeadCanonicalCombiningClass::DoubleAbove,
        "24" => LeadCanonicalCombiningClass::CCC24,
        "240" => LeadCanonicalCombiningClass::IotaSubscript,
        "25" => LeadCanonicalCombiningClass::CCC25,
        "26" => LeadCanonicalCombiningClass::CCC26,
        "27" => LeadCanonicalCombiningClass::CCC27,
        "28" => LeadCanonicalCombiningClass::CCC28,
        "29" => LeadCanonicalCombiningClass::CCC29,
        "30" => LeadCanonicalCombiningClass::CCC30,
        "31" => LeadCanonicalCombiningClass::CCC31,
        "32" => LeadCanonicalCombiningClass::CCC32,
        "33" => LeadCanonicalCombiningClass::CCC33,
        "34" => LeadCanonicalCombiningClass::CCC34,
        "35" => LeadCanonicalCombiningClass::CCC35,
        "36" => LeadCanonicalCombiningClass::CCC36,
        "6" => LeadCanonicalCombiningClass::HanReading,
        "7" => LeadCanonicalCombiningClass::Nukta,
        "8" => LeadCanonicalCombiningClass::KanaVoicing,
        "84" => LeadCanonicalCombiningClass::CCC84,
        "9" => LeadCanonicalCombiningClass::Virama,
        "91" => LeadCanonicalCombiningClass::CCC91,
        _ => return None,
    })
}

fn get_nfc_quick_check_enum(name: &str) -> Option<NFCQuickCheck> {
    Some(match name {
        "M" => NFCQuickCheck::Maybe,
        "N" => NFCQuickCheck::No,
        "Y" => NFCQuickCheck::Yes,
        _ => return None,
    })
}

fn get_nfd_quick_check_enum(name: &str) -> Option<NFDQuickCheck> {
    Some(match name {
        "N" => NFDQuickCheck::No,
        "Y" => NFDQuickCheck::Yes,
        _ => return None,
    })
}

fn get_nfkc_quick_check_enum(name: &str) -> Option<NFKCQuickCheck> {
    Some(match name {
        "M" => NFKCQuickCheck::Maybe,
        "N" => NFKCQuickCheck::No,
        "Y" => NFKCQuickCheck::Yes,
        _ => return None,
    })
}

fn get_nfkd_quick_check_enum(name: &str) -> Option<NFKDQuickCheck> {
    Some(match name {
        "N" => NFKDQuickCheck::No,
        "Y" => NFKDQuickCheck::Yes,
        _ => return None,
    })
}

fn get_numeric_type_enum(name: &str) -> Option<NumericType> {
    Some(match name {
        "De" => NumericType::Decimal,
        "Di" => NumericType::Digit,
        "None" => NumericType::None,
        "Nu" => NumericType::Numeric,
        _ => return None,
    })
}

fn get_sentence_break_enum(name: &str) -> Option<SentenceBreak> {
    Some(match name {
        "AT" => SentenceBreak::ATerm,
        "CL" => SentenceBreak::Close,
        "CR" => SentenceBreak::CR,
        "EX" => SentenceBreak::Extend,
        "FO" => SentenceBreak::Format,
        "LE" => SentenceBreak::OLetter,
        "LF" => SentenceBreak::LF,
        "LO" => SentenceBreak::Lower,
        "NU" => SentenceBreak::Numeric,
        "SC" => SentenceBreak::SContinue,
        "SE" => SentenceBreak::Sep,
        "SP" => SentenceBreak::Sp,
        "ST" => SentenceBreak::STerm,
        "UP" => SentenceBreak::Upper,
        "XX" => SentenceBreak::Other,
        _ => return None,
    })
}

fn get_trail_canonical_combining_class_enum(name: &str) -> Option<TrailCanonicalCombiningClass> {
    Some(match name {
        "0" => TrailCanonicalCombiningClass::NotReordered,
        "1" => TrailCanonicalCombiningClass::Overlay,
        "10" => TrailCanonicalCombiningClass::CCC10,
        "103" => TrailCanonicalCombiningClass::CCC103,
        "107" => TrailCanonicalCombiningClass::CCC107,
        "11" => TrailCanonicalCombiningClass::CCC11,
        "118" => TrailCanonicalCombiningClass::CCC118,
        "12" => TrailCanonicalCombiningClass::CCC12,
        "122" => TrailCanonicalCombiningClass::CCC122,
        "129" => TrailCanonicalCombiningClass::CCC129,
        "13" => TrailCanonicalCombiningClass::CCC13,
        "130" => TrailCanonicalCombiningClass::CCC130,
        "132" => TrailCanonicalCombiningClass::CCC132,
        "133" => TrailCanonicalCombiningClass::CCC133,
        "14" => TrailCanonicalCombiningClass::CCC14,
        "15" => TrailCanonicalCombiningClass::CCC15,
        "16" => TrailCanonicalCombiningClass::CCC16,
        "17" => TrailCanonicalCombiningClass::CCC17,
        "18" => TrailCanonicalCombiningClass::CCC18,
        "19" => TrailCanonicalCombiningClass::CCC19,
        "20" => TrailCanonicalCombiningClass::CCC20,
        "200" => TrailCanonicalCombiningClass::AttachedBelowLeft,
        "202" => TrailCanonicalCombiningClass::AttachedBelow,
        "21" => TrailCanonicalCombiningClass::CCC21,
        "214" => TrailCanonicalCombiningClass::AttachedAbove,
        "216" => TrailCanonicalCombiningClass::AttachedAboveRight,
        "218" => TrailCanonicalCombiningClass::BelowLeft,
        "22" => TrailCanonicalCombiningClass::CCC22,
        "220" => TrailCanonicalCombiningClass::Below,
        "222" => TrailCanonicalCombiningClass::BelowRight,
        "224" => TrailCanonicalCombiningClass::Left,
        "226" => TrailCanonicalCombiningClass::Right,
        "228" => TrailCanonicalCombiningClass::AboveLeft,
        "23" => TrailCanonicalCombiningClass::CCC23,
        "230" => TrailCanonicalCombiningClass::Above,
        "232" => TrailCanonicalCombiningClass::AboveRight,
        "233" => TrailCanonicalCombiningClass::DoubleBelow,
        "234" => TrailCanonicalCombiningClass::DoubleAbove,
        "24" => TrailCanonicalCombiningClass::CCC24,
        "240" => TrailCanonicalCombiningClass::IotaSubscript,
        "25" => TrailCanonicalCombiningClass::CCC25,
        "26" => TrailCanonicalCombiningClass::CCC26,
        "27" => TrailCanonicalCombiningClass::CCC27,
        "28" => TrailCanonicalCombiningClass::CCC28,
        "29" => TrailCanonicalCombiningClass::CCC29,
        "30" => TrailCanonicalCombiningClass::CCC30,
        "31" => TrailCanonicalCombiningClass::CCC31,
        "32" => TrailCanonicalCombiningClass::CCC32,
        "33" => TrailCanonicalCombiningClass::CCC33,
        "34" => TrailCanonicalCombiningClass::CCC34,
        "35" => TrailCanonicalCombiningClass::CCC35,
        "36" => TrailCanonicalCombiningClass::CCC36,
        "6" => TrailCanonicalCombiningClass::HanReading,
        "7" => TrailCanonicalCombiningClass::Nukta,
        "8" => TrailCanonicalCombiningClass::KanaVoicing,
        "84" => TrailCanonicalCombiningClass::CCC84,
        "9" => TrailCanonicalCombiningClass::Virama,
        "91" => TrailCanonicalCombiningClass::CCC91,
        _ => return None,
    })
}

fn get_vertical_orientation_enum(name: &str) -> Option<VerticalOrientation> {
    Some(match name {
        "R" => VerticalOrientation::Rotated,
        "Tr" => VerticalOrientation::TransformedRotated,
        "Tu" => VerticalOrientation::TransformedUpright,
        "U" => VerticalOrientation::Upright,
        _ => return None,
    })
}

fn get_word_break_enum(name: &str) -> Option<WordBreak> {
    Some(match name {
        "CR" => WordBreak::CR,
        "DQ" => WordBreak::DoubleQuote,
        "EB" => WordBreak::EBase,
        "EBG" => WordBreak::EBaseGAZ,
        "EM" => WordBreak::EModifier,
        "EX" => WordBreak::ExtendNumLet,
        "Extend" => WordBreak::Extend,
        "FO" => WordBreak::Format,
        "GAZ" => WordBreak::GlueAfterZwj,
        "HL" => WordBreak::HebrewLetter,
        "KA" => WordBreak::Katakana,
        "LE" => WordBreak::ALetter,
        "LF" => WordBreak::LF,
        "MB" => WordBreak::MidNumLet,
        "ML" => WordBreak::MidLetter,
        "MN" => WordBreak::MidNum,
        "NL" => WordBreak::Newline,
        "NU" => WordBreak::Numeric,
        "RI" => WordBreak::RegionalIndicator,
        "SQ" => WordBreak::SingleQuote,
        "WSegSpace" => WordBreak::WSegSpace,
        "XX" => WordBreak::Other,
        "ZWJ" => WordBreak::ZWJ,
        _ => return None,
    })
}

//
// Helper fn to help generate identifer for the prop_name=prop_val `UnicodeProperty`
//

fn get_prop_name_val_as_i32(prop_name: &str, prop_val: &str) -> Option<(i32, i32)> {
    let name_enum_opt = get_enum_property_enum(prop_name);
    let val_enum_i32_opt = match name_enum_opt {
        Some(EnumeratedProperty::BidiClass) => get_bidi_class_enum(prop_val).map(|x| x as i32),
        Some(EnumeratedProperty::BidiPairedBracketType) => {
            get_bidi_paired_bracket_type_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::CanonicalCombiningClass) => {
            get_canonical_combining_class_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::DecompositionType) => {
            get_decomposition_type_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::EastAsianWidth) => {
            get_east_asian_width_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::GeneralCategory) => {
            get_general_category_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::GraphemeClusterBreak) => {
            get_grapheme_cluster_break_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::HangulSyllableType) => {
            get_hangul_syllable_type_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::IndicPositionalCategory) => {
            get_indic_positional_category_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::IndicSyllabicCategory) => {
            get_indic_syllabic_category_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::JoiningGroup) => {
            get_joining_group_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::JoiningType) => get_joining_type_enum(prop_val).map(|x| x as i32),
        Some(EnumeratedProperty::LineBreak) => get_line_break_enum(prop_val).map(|x| x as i32),
        Some(EnumeratedProperty::LeadCanonicalCombiningClass) => {
            get_lead_canonical_combining_class_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::NFCQuickCheck) => {
            get_nfc_quick_check_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::NFDQuickCheck) => {
            get_nfd_quick_check_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::NFKCQuickCheck) => {
            get_nfkc_quick_check_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::NFKDQuickCheck) => {
            get_nfkd_quick_check_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::NumericType) => get_numeric_type_enum(prop_val).map(|x| x as i32),
        Some(EnumeratedProperty::SentenceBreak) => {
            get_sentence_break_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::TrailCanonicalCombiningClass) => {
            get_trail_canonical_combining_class_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::VerticalOrientation) => {
            get_vertical_orientation_enum(prop_val).map(|x| x as i32)
        }
        Some(EnumeratedProperty::WordBreak) => get_word_break_enum(prop_val).map(|x| x as i32),
        _ => None,
    };
    let name_enum_i32_opt = name_enum_opt.map(|x| x as i32);
    match (name_enum_i32_opt, val_enum_i32_opt) {
        (Some(name_i32), Some(val_i32)) => Some((name_i32, val_i32)),
        _ => None,
    }
}

pub fn get_prop_name_identifier(prop_name: &str, prop_val: &str) -> Option<TinyStr16> {
    let name_val_i32_opt = get_prop_name_val_as_i32(prop_name, prop_val);
    let name_val_string_opt = match name_val_i32_opt {
        Some((name_i32, val_i32)) => Some(format!("{}={}", name_i32, val_i32)),
        _ => None,
    };
    match name_val_string_opt {
        Some(id_str) => TinyStr16::from_str(&id_str).ok(),
        _ => None,
    }
}

#[cfg(test)]
mod enum_tests {
    use super::*;

    #[test]
    fn prop_name_str_to_enum_fn_test() {
        assert_eq!(get_line_break_enum("LF"), Some(LineBreak::LineFeed));
        assert_eq!(get_line_break_enum("cheezburger"), None);
    }

    #[test]
    fn prop_value_str_to_enum_fn_test() {
        assert_eq!(
            get_canonical_combining_class_enum("21"),
            Some(CanonicalCombiningClass::CCC21)
        );
        assert_eq!(get_canonical_combining_class_enum("cheezburger"), None);
    }

    #[test]
    fn get_prop_name_val_as_i32_test() {
        let act_prop_i32_tuple_opt_1 = get_prop_name_val_as_i32("lb", "LF");
        let exp_prop_i32_tuple_opt_1 = Some((
            EnumeratedProperty::LineBreak as i32,
            LineBreak::LineFeed as i32,
        ));
        assert_eq!(act_prop_i32_tuple_opt_1, exp_prop_i32_tuple_opt_1);

        assert_eq!(get_prop_name_val_as_i32("lb", "cheezburger"), None);
        assert_eq!(get_prop_name_val_as_i32("cheezburger", "LF"), None);
        assert_eq!(get_prop_name_val_as_i32("cheez", "cheez"), None);
    }

    #[test]
    fn get_prop_name_identifier_test() {
        assert_eq!(
            get_prop_name_identifier("lb", "LF"),
            TinyStr16::from_str("12=26").ok()
        );
        assert_eq!(
            get_prop_name_identifier("ccc", "230"),
            TinyStr16::from_str("2=230").ok()
        );
    }
}
