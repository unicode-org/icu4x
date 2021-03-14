// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::enum_props::*;
use std::str::FromStr;
use tinystr::TinyStr16;

//
// Single getter function for enumerated property name:
// Enum prop name string -> Rust enum
//

fn get_enum_property_enum(name: &str) -> Option<EnumeratedProperty> {
    match name {
        "bc" => Some(EnumeratedProperty::BidiClass),
        "bpt" => Some(EnumeratedProperty::BidiPairedBracketType),
        "ccc" => Some(EnumeratedProperty::CanonicalCombiningClass),
        "dt" => Some(EnumeratedProperty::DecompositionType),
        "ea" => Some(EnumeratedProperty::EastAsianWidth),
        "gc" => Some(EnumeratedProperty::GeneralCategory),
        "GCB" => Some(EnumeratedProperty::GraphemeClusterBreak),
        "hst" => Some(EnumeratedProperty::HangulSyllableType),
        "InPC" => Some(EnumeratedProperty::IndicPositionalCategory),
        "InSC" => Some(EnumeratedProperty::IndicSyllabicCategory),
        "jg" => Some(EnumeratedProperty::JoiningGroup),
        "jt" => Some(EnumeratedProperty::JoiningType),
        "lb" => Some(EnumeratedProperty::LineBreak),
        "lccc" => Some(EnumeratedProperty::LeadCanonicalCombiningClass),
        "NFC_QC" => Some(EnumeratedProperty::NFCQuickCheck),
        "NFD_QC" => Some(EnumeratedProperty::NFDQuickCheck),
        "NFKC_QC" => Some(EnumeratedProperty::NFKCQuickCheck),
        "NFKD_QC" => Some(EnumeratedProperty::NFKDQuickCheck),
        "nt" => Some(EnumeratedProperty::NumericType),
        "SB" => Some(EnumeratedProperty::SentenceBreak),
        "tccc" => Some(EnumeratedProperty::TrailCanonicalCombiningClass),
        "vo" => Some(EnumeratedProperty::VerticalOrientation),
        "WB" => Some(EnumeratedProperty::WordBreak),
        _ => None,
    }
}

//
// Getter function per enumerated property:
// Enum prop val string -> Rust enum
//

fn get_bidi_class_enum(name: &str) -> Option<BidiClass> {
    match name {
        "AL" => Some(BidiClass::ArabicLetter),
        "AN" => Some(BidiClass::ArabicNumber),
        "B" => Some(BidiClass::ParagraphSeparator),
        "BN" => Some(BidiClass::BoundaryNeutral),
        "CS" => Some(BidiClass::CommonSeparator),
        "EN" => Some(BidiClass::EuropeanNumber),
        "ES" => Some(BidiClass::EuropeanSeparator),
        "ET" => Some(BidiClass::EuropeanTerminator),
        "FSI" => Some(BidiClass::FirstStrongIsolate),
        "L" => Some(BidiClass::LeftToRight),
        "LRE" => Some(BidiClass::LeftToRightEmbedding),
        "LRI" => Some(BidiClass::LeftToRightIsolate),
        "LRO" => Some(BidiClass::LeftToRightOverride),
        "NSM" => Some(BidiClass::NonspacingMark),
        "ON" => Some(BidiClass::OtherNeutral),
        "PDF" => Some(BidiClass::PopDirectionalFormat),
        "PDI" => Some(BidiClass::PopDirectionalIsolate),
        "R" => Some(BidiClass::RightToLeft),
        "RLE" => Some(BidiClass::RightToLeftEmbedding),
        "RLI" => Some(BidiClass::RightToLeftIsolate),
        "RLO" => Some(BidiClass::RightToLeftOverride),
        "S" => Some(BidiClass::SegmentSeparator),
        "WS" => Some(BidiClass::WhiteSpace),
        _ => None,
    }
}

fn get_bidi_paired_bracket_type_enum(name: &str) -> Option<BidiPairedBracketType> {
    match name {
        "c" => Some(BidiPairedBracketType::Close),
        "n" => Some(BidiPairedBracketType::None),
        "o" => Some(BidiPairedBracketType::Open),
        _ => None,
    }
}

fn get_canonical_combining_class_enum(name: &str) -> Option<CanonicalCombiningClass> {
    match name {
        "0" => Some(CanonicalCombiningClass::NotReordered),
        "1" => Some(CanonicalCombiningClass::Overlay),
        "10" => Some(CanonicalCombiningClass::CCC10),
        "103" => Some(CanonicalCombiningClass::CCC103),
        "107" => Some(CanonicalCombiningClass::CCC107),
        "11" => Some(CanonicalCombiningClass::CCC11),
        "118" => Some(CanonicalCombiningClass::CCC118),
        "12" => Some(CanonicalCombiningClass::CCC12),
        "122" => Some(CanonicalCombiningClass::CCC122),
        "129" => Some(CanonicalCombiningClass::CCC129),
        "13" => Some(CanonicalCombiningClass::CCC13),
        "130" => Some(CanonicalCombiningClass::CCC130),
        "132" => Some(CanonicalCombiningClass::CCC132),
        "133" => Some(CanonicalCombiningClass::CCC133),
        "14" => Some(CanonicalCombiningClass::CCC14),
        "15" => Some(CanonicalCombiningClass::CCC15),
        "16" => Some(CanonicalCombiningClass::CCC16),
        "17" => Some(CanonicalCombiningClass::CCC17),
        "18" => Some(CanonicalCombiningClass::CCC18),
        "19" => Some(CanonicalCombiningClass::CCC19),
        "20" => Some(CanonicalCombiningClass::CCC20),
        "200" => Some(CanonicalCombiningClass::AttachedBelowLeft),
        "202" => Some(CanonicalCombiningClass::AttachedBelow),
        "21" => Some(CanonicalCombiningClass::CCC21),
        "214" => Some(CanonicalCombiningClass::AttachedAbove),
        "216" => Some(CanonicalCombiningClass::AttachedAboveRight),
        "218" => Some(CanonicalCombiningClass::BelowLeft),
        "22" => Some(CanonicalCombiningClass::CCC22),
        "220" => Some(CanonicalCombiningClass::Below),
        "222" => Some(CanonicalCombiningClass::BelowRight),
        "224" => Some(CanonicalCombiningClass::Left),
        "226" => Some(CanonicalCombiningClass::Right),
        "228" => Some(CanonicalCombiningClass::AboveLeft),
        "23" => Some(CanonicalCombiningClass::CCC23),
        "230" => Some(CanonicalCombiningClass::Above),
        "232" => Some(CanonicalCombiningClass::AboveRight),
        "233" => Some(CanonicalCombiningClass::DoubleBelow),
        "234" => Some(CanonicalCombiningClass::DoubleAbove),
        "24" => Some(CanonicalCombiningClass::CCC24),
        "240" => Some(CanonicalCombiningClass::IotaSubscript),
        "25" => Some(CanonicalCombiningClass::CCC25),
        "26" => Some(CanonicalCombiningClass::CCC26),
        "27" => Some(CanonicalCombiningClass::CCC27),
        "28" => Some(CanonicalCombiningClass::CCC28),
        "29" => Some(CanonicalCombiningClass::CCC29),
        "30" => Some(CanonicalCombiningClass::CCC30),
        "31" => Some(CanonicalCombiningClass::CCC31),
        "32" => Some(CanonicalCombiningClass::CCC32),
        "33" => Some(CanonicalCombiningClass::CCC33),
        "34" => Some(CanonicalCombiningClass::CCC34),
        "35" => Some(CanonicalCombiningClass::CCC35),
        "36" => Some(CanonicalCombiningClass::CCC36),
        "6" => Some(CanonicalCombiningClass::HanReading),
        "7" => Some(CanonicalCombiningClass::Nukta),
        "8" => Some(CanonicalCombiningClass::KanaVoicing),
        "84" => Some(CanonicalCombiningClass::CCC84),
        "9" => Some(CanonicalCombiningClass::Virama),
        "91" => Some(CanonicalCombiningClass::CCC91),
        _ => None,
    }
}

fn get_decomposition_type_enum(name: &str) -> Option<DecompositionType> {
    match name {
        "Can" => Some(DecompositionType::Can),
        "Com" => Some(DecompositionType::Com),
        "Enc" => Some(DecompositionType::Enc),
        "Fin" => Some(DecompositionType::Fin),
        "Font" => Some(DecompositionType::Font),
        "Fra" => Some(DecompositionType::Fra),
        "Init" => Some(DecompositionType::Init),
        "Iso" => Some(DecompositionType::Iso),
        "Med" => Some(DecompositionType::Med),
        "Nar" => Some(DecompositionType::Nar),
        "Nb" => Some(DecompositionType::Nb),
        "None" => Some(DecompositionType::None),
        "Sml" => Some(DecompositionType::Sml),
        "Sqr" => Some(DecompositionType::Sqr),
        "Sub" => Some(DecompositionType::Sub),
        "Sup" => Some(DecompositionType::Sup),
        "Vert" => Some(DecompositionType::Vert),
        "Wide" => Some(DecompositionType::Wide),
        _ => None,
    }
}

fn get_east_asian_width_enum(name: &str) -> Option<EastAsianWidth> {
    match name {
        "A" => Some(EastAsianWidth::Ambiguous),
        "F" => Some(EastAsianWidth::Fullwidth),
        "H" => Some(EastAsianWidth::Halfwidth),
        "N" => Some(EastAsianWidth::Neutral),
        "Na" => Some(EastAsianWidth::Narrow),
        "W" => Some(EastAsianWidth::Wide),
        _ => None,
    }
}

fn get_general_category_enum(name: &str) -> Option<GeneralCategory> {
    match name {
        "C" => Some(GeneralCategory::Other),
        "Cc" => Some(GeneralCategory::Cntrl),
        "Cf" => Some(GeneralCategory::Format),
        "Cn" => Some(GeneralCategory::Unassigned),
        "Co" => Some(GeneralCategory::PrivateUse),
        "Cs" => Some(GeneralCategory::Surrogate),
        "L" => Some(GeneralCategory::Letter),
        "LC" => Some(GeneralCategory::CasedLetter),
        "Ll" => Some(GeneralCategory::LowercaseLetter),
        "Lm" => Some(GeneralCategory::ModifierLetter),
        "Lo" => Some(GeneralCategory::OtherLetter),
        "Lt" => Some(GeneralCategory::TitlecaseLetter),
        "Lu" => Some(GeneralCategory::UppercaseLetter),
        "M" => Some(GeneralCategory::CombiningMark),
        "Mc" => Some(GeneralCategory::SpacingMark),
        "Me" => Some(GeneralCategory::EnclosingMark),
        "Mn" => Some(GeneralCategory::NonspacingMark),
        "N" => Some(GeneralCategory::Number),
        "Nd" => Some(GeneralCategory::Digit),
        "Nl" => Some(GeneralCategory::LetterNumber),
        "No" => Some(GeneralCategory::OtherNumber),
        "P" => Some(GeneralCategory::Punct),
        "Pc" => Some(GeneralCategory::ConnectorPunctuation),
        "Pd" => Some(GeneralCategory::DashPunctuation),
        "Pe" => Some(GeneralCategory::ClosePunctuation),
        "Pf" => Some(GeneralCategory::FinalPunctuation),
        "Pi" => Some(GeneralCategory::InitialPunctuation),
        "Po" => Some(GeneralCategory::OtherPunctuation),
        "Ps" => Some(GeneralCategory::OpenPunctuation),
        "S" => Some(GeneralCategory::Symbol),
        "Sc" => Some(GeneralCategory::CurrencySymbol),
        "Sk" => Some(GeneralCategory::ModifierSymbol),
        "Sm" => Some(GeneralCategory::MathSymbol),
        "So" => Some(GeneralCategory::OtherSymbol),
        "Z" => Some(GeneralCategory::Separator),
        "Zl" => Some(GeneralCategory::LineSeparator),
        "Zp" => Some(GeneralCategory::ParagraphSeparator),
        "Zs" => Some(GeneralCategory::SpaceSeparator),
        _ => None,
    }
}

fn get_grapheme_cluster_break_enum(name: &str) -> Option<GraphemeClusterBreak> {
    match name {
        "CN" => Some(GraphemeClusterBreak::Control),
        "CR" => Some(GraphemeClusterBreak::CR),
        "EB" => Some(GraphemeClusterBreak::EBase),
        "EBG" => Some(GraphemeClusterBreak::EBaseGAZ),
        "EM" => Some(GraphemeClusterBreak::EModifier),
        "EX" => Some(GraphemeClusterBreak::Extend),
        "GAZ" => Some(GraphemeClusterBreak::GlueAfterZwj),
        "L" => Some(GraphemeClusterBreak::L),
        "LF" => Some(GraphemeClusterBreak::LF),
        "LV" => Some(GraphemeClusterBreak::LV),
        "LVT" => Some(GraphemeClusterBreak::LVT),
        "PP" => Some(GraphemeClusterBreak::Prepend),
        "RI" => Some(GraphemeClusterBreak::RegionalIndicator),
        "SM" => Some(GraphemeClusterBreak::SpacingMark),
        "T" => Some(GraphemeClusterBreak::T),
        "V" => Some(GraphemeClusterBreak::V),
        "XX" => Some(GraphemeClusterBreak::Other),
        "ZWJ" => Some(GraphemeClusterBreak::ZWJ),
        _ => None,
    }
}

fn get_hangul_syllable_type_enum(name: &str) -> Option<HangulSyllableType> {
    match name {
        "L" => Some(HangulSyllableType::LeadingJamo),
        "LV" => Some(HangulSyllableType::LVSyllable),
        "LVT" => Some(HangulSyllableType::LVTSyllable),
        "NA" => Some(HangulSyllableType::NotApplicable),
        "T" => Some(HangulSyllableType::TrailingJamo),
        "V" => Some(HangulSyllableType::VowelJamo),
        _ => None,
    }
}

fn get_indic_positional_category_enum(name: &str) -> Option<IndicPositionalCategory> {
    match name {
        "Bottom" => Some(IndicPositionalCategory::Bottom),
        "Bottom_And_Left" => Some(IndicPositionalCategory::BottomAndLeft),
        "Bottom_And_Right" => Some(IndicPositionalCategory::BottomAndRight),
        "Left" => Some(IndicPositionalCategory::Left),
        "Left_And_Right" => Some(IndicPositionalCategory::LeftAndRight),
        "NA" => Some(IndicPositionalCategory::NA),
        "Overstruck" => Some(IndicPositionalCategory::Overstruck),
        "Right" => Some(IndicPositionalCategory::Right),
        "Top" => Some(IndicPositionalCategory::Top),
        "Top_And_Bottom" => Some(IndicPositionalCategory::TopAndBottom),
        "Top_And_Bottom_And_Left" => Some(IndicPositionalCategory::TopAndBottomAndLeft),
        "Top_And_Bottom_And_Right" => Some(IndicPositionalCategory::TopAndBottomAndRight),
        "Top_And_Left" => Some(IndicPositionalCategory::TopAndLeft),
        "Top_And_Left_And_Right" => Some(IndicPositionalCategory::TopAndLeftAndRight),
        "Top_And_Right" => Some(IndicPositionalCategory::TopAndRight),
        "Visual_Order_Left" => Some(IndicPositionalCategory::VisualOrderLeft),
        _ => None,
    }
}

fn get_indic_syllabic_category_enum(name: &str) -> Option<IndicSyllabicCategory> {
    match name {
        "Avagraha" => Some(IndicSyllabicCategory::Avagraha),
        "Bindu" => Some(IndicSyllabicCategory::Bindu),
        "Brahmi_Joining_Number" => Some(IndicSyllabicCategory::BrahmiJoiningNumber),
        "Cantillation_Mark" => Some(IndicSyllabicCategory::CantillationMark),
        "Consonant" => Some(IndicSyllabicCategory::Consonant),
        "Consonant_Dead" => Some(IndicSyllabicCategory::ConsonantDead),
        "Consonant_Final" => Some(IndicSyllabicCategory::ConsonantFinal),
        "Consonant_Head_Letter" => Some(IndicSyllabicCategory::ConsonantHeadLetter),
        "Consonant_Initial_Postfixed" => Some(IndicSyllabicCategory::ConsonantInitialPostfixed),
        "Consonant_Killer" => Some(IndicSyllabicCategory::ConsonantKiller),
        "Consonant_Medial" => Some(IndicSyllabicCategory::ConsonantMedial),
        "Consonant_Placeholder" => Some(IndicSyllabicCategory::ConsonantPlaceholder),
        "Consonant_Preceding_Repha" => Some(IndicSyllabicCategory::ConsonantPrecedingRepha),
        "Consonant_Prefixed" => Some(IndicSyllabicCategory::ConsonantPrefixed),
        "Consonant_Subjoined" => Some(IndicSyllabicCategory::ConsonantSubjoined),
        "Consonant_Succeeding_Repha" => Some(IndicSyllabicCategory::ConsonantSucceedingRepha),
        "Consonant_With_Stacker" => Some(IndicSyllabicCategory::ConsonantWithStacker),
        "Gemination_Mark" => Some(IndicSyllabicCategory::GeminationMark),
        "Invisible_Stacker" => Some(IndicSyllabicCategory::InvisibleStacker),
        "Joiner" => Some(IndicSyllabicCategory::Joiner),
        "Modifying_Letter" => Some(IndicSyllabicCategory::ModifyingLetter),
        "Non_Joiner" => Some(IndicSyllabicCategory::NonJoiner),
        "Nukta" => Some(IndicSyllabicCategory::Nukta),
        "Number" => Some(IndicSyllabicCategory::Number),
        "Number_Joiner" => Some(IndicSyllabicCategory::NumberJoiner),
        "Other" => Some(IndicSyllabicCategory::Other),
        "Pure_Killer" => Some(IndicSyllabicCategory::PureKiller),
        "Register_Shifter" => Some(IndicSyllabicCategory::RegisterShifter),
        "Syllable_Modifier" => Some(IndicSyllabicCategory::SyllableModifier),
        "Tone_Letter" => Some(IndicSyllabicCategory::ToneLetter),
        "Tone_Mark" => Some(IndicSyllabicCategory::ToneMark),
        "Virama" => Some(IndicSyllabicCategory::Virama),
        "Visarga" => Some(IndicSyllabicCategory::Visarga),
        "Vowel" => Some(IndicSyllabicCategory::Vowel),
        "Vowel_Dependent" => Some(IndicSyllabicCategory::VowelDependent),
        "Vowel_Independent" => Some(IndicSyllabicCategory::VowelIndependent),
        _ => None,
    }
}

fn get_joining_group_enum(name: &str) -> Option<JoiningGroup> {
    match name {
        "African_Feh" => Some(JoiningGroup::AfricanFeh),
        "African_Noon" => Some(JoiningGroup::AfricanNoon),
        "African_Qaf" => Some(JoiningGroup::AfricanQaf),
        "Ain" => Some(JoiningGroup::Ain),
        "Alaph" => Some(JoiningGroup::Alaph),
        "Alef" => Some(JoiningGroup::Alef),
        "Beh" => Some(JoiningGroup::Beh),
        "Beth" => Some(JoiningGroup::Beth),
        "Burushaski_Yeh_Barree" => Some(JoiningGroup::BurushaskiYehBarree),
        "Dal" => Some(JoiningGroup::Dal),
        "Dalath_Rish" => Some(JoiningGroup::DalathRish),
        "E" => Some(JoiningGroup::E),
        "Farsi_Yeh" => Some(JoiningGroup::FarsiYeh),
        "Fe" => Some(JoiningGroup::Fe),
        "Feh" => Some(JoiningGroup::Feh),
        "Final_Semkath" => Some(JoiningGroup::FinalSemkath),
        "Gaf" => Some(JoiningGroup::Gaf),
        "Gamal" => Some(JoiningGroup::Gamal),
        "Hah" => Some(JoiningGroup::Hah),
        "Hanifi_Rohingya_Kinna_Ya" => Some(JoiningGroup::HanifiRohingyaKinnaYa),
        "Hanifi_Rohingya_Pa" => Some(JoiningGroup::HanifiRohingyaPa),
        "He" => Some(JoiningGroup::He),
        "Heh" => Some(JoiningGroup::Heh),
        "Heh_Goal" => Some(JoiningGroup::HehGoal),
        "Heth" => Some(JoiningGroup::Heth),
        "Kaf" => Some(JoiningGroup::Kaf),
        "Kaph" => Some(JoiningGroup::Kaph),
        "Khaph" => Some(JoiningGroup::Khaph),
        "Knotted_Heh" => Some(JoiningGroup::KnottedHeh),
        "Lam" => Some(JoiningGroup::Lam),
        "Lamadh" => Some(JoiningGroup::Lamadh),
        "Malayalam_Bha" => Some(JoiningGroup::MalayalamBha),
        "Malayalam_Ja" => Some(JoiningGroup::MalayalamJa),
        "Malayalam_Lla" => Some(JoiningGroup::MalayalamLla),
        "Malayalam_Llla" => Some(JoiningGroup::MalayalamLlla),
        "Malayalam_Nga" => Some(JoiningGroup::MalayalamNga),
        "Malayalam_Nna" => Some(JoiningGroup::MalayalamNna),
        "Malayalam_Nnna" => Some(JoiningGroup::MalayalamNnna),
        "Malayalam_Nya" => Some(JoiningGroup::MalayalamNya),
        "Malayalam_Ra" => Some(JoiningGroup::MalayalamRa),
        "Malayalam_Ssa" => Some(JoiningGroup::MalayalamSsa),
        "Malayalam_Tta" => Some(JoiningGroup::MalayalamTta),
        "Manichaean_Aleph" => Some(JoiningGroup::ManichaeanAleph),
        "Manichaean_Ayin" => Some(JoiningGroup::ManichaeanAyin),
        "Manichaean_Beth" => Some(JoiningGroup::ManichaeanBeth),
        "Manichaean_Daleth" => Some(JoiningGroup::ManichaeanDaleth),
        "Manichaean_Dhamedh" => Some(JoiningGroup::ManichaeanDhamedh),
        "Manichaean_Five" => Some(JoiningGroup::ManichaeanFive),
        "Manichaean_Gimel" => Some(JoiningGroup::ManichaeanGimel),
        "Manichaean_Heth" => Some(JoiningGroup::ManichaeanHeth),
        "Manichaean_Hundred" => Some(JoiningGroup::ManichaeanHundred),
        "Manichaean_Kaph" => Some(JoiningGroup::ManichaeanKaph),
        "Manichaean_Lamedh" => Some(JoiningGroup::ManichaeanLamedh),
        "Manichaean_Mem" => Some(JoiningGroup::ManichaeanMem),
        "Manichaean_Nun" => Some(JoiningGroup::ManichaeanNun),
        "Manichaean_One" => Some(JoiningGroup::ManichaeanOne),
        "Manichaean_Pe" => Some(JoiningGroup::ManichaeanPe),
        "Manichaean_Qoph" => Some(JoiningGroup::ManichaeanQoph),
        "Manichaean_Resh" => Some(JoiningGroup::ManichaeanResh),
        "Manichaean_Sadhe" => Some(JoiningGroup::ManichaeanSadhe),
        "Manichaean_Samekh" => Some(JoiningGroup::ManichaeanSamekh),
        "Manichaean_Taw" => Some(JoiningGroup::ManichaeanTaw),
        "Manichaean_Ten" => Some(JoiningGroup::ManichaeanTen),
        "Manichaean_Teth" => Some(JoiningGroup::ManichaeanTeth),
        "Manichaean_Thamedh" => Some(JoiningGroup::ManichaeanThamedh),
        "Manichaean_Twenty" => Some(JoiningGroup::ManichaeanTwenty),
        "Manichaean_Waw" => Some(JoiningGroup::ManichaeanWaw),
        "Manichaean_Yodh" => Some(JoiningGroup::ManichaeanYodh),
        "Manichaean_Zayin" => Some(JoiningGroup::ManichaeanZayin),
        "Meem" => Some(JoiningGroup::Meem),
        "Mim" => Some(JoiningGroup::Mim),
        "No_Joining_Group" => Some(JoiningGroup::NoJoiningGroup),
        "Noon" => Some(JoiningGroup::Noon),
        "Nun" => Some(JoiningGroup::Nun),
        "Nya" => Some(JoiningGroup::Nya),
        "Pe" => Some(JoiningGroup::Pe),
        "Qaf" => Some(JoiningGroup::Qaf),
        "Qaph" => Some(JoiningGroup::Qaph),
        "Reh" => Some(JoiningGroup::Reh),
        "Reversed_Pe" => Some(JoiningGroup::ReversedPe),
        "Rohingya_Yeh" => Some(JoiningGroup::RohingyaYeh),
        "Sad" => Some(JoiningGroup::Sad),
        "Sadhe" => Some(JoiningGroup::Sadhe),
        "Seen" => Some(JoiningGroup::Seen),
        "Semkath" => Some(JoiningGroup::Semkath),
        "Shin" => Some(JoiningGroup::Shin),
        "Straight_Waw" => Some(JoiningGroup::StraightWaw),
        "Swash_Kaf" => Some(JoiningGroup::SwashKaf),
        "Syriac_Waw" => Some(JoiningGroup::SyriacWaw),
        "Tah" => Some(JoiningGroup::Tah),
        "Taw" => Some(JoiningGroup::Taw),
        "Teh_Marbuta" => Some(JoiningGroup::TehMarbuta),
        "Teh_Marbuta_Goal" => Some(JoiningGroup::TehMarbutaGoal),
        "Teth" => Some(JoiningGroup::Teth),
        "Waw" => Some(JoiningGroup::Waw),
        "Yeh" => Some(JoiningGroup::Yeh),
        "Yeh_Barree" => Some(JoiningGroup::YehBarree),
        "Yeh_With_Tail" => Some(JoiningGroup::YehWithTail),
        "Yudh" => Some(JoiningGroup::Yudh),
        "Yudh_He" => Some(JoiningGroup::YudhHe),
        "Zain" => Some(JoiningGroup::Zain),
        "Zhain" => Some(JoiningGroup::Zhain),
        _ => None,
    }
}

fn get_joining_type_enum(name: &str) -> Option<JoiningType> {
    match name {
        "C" => Some(JoiningType::JoinCausing),
        "D" => Some(JoiningType::DualJoining),
        "L" => Some(JoiningType::LeftJoining),
        "R" => Some(JoiningType::RightJoining),
        "T" => Some(JoiningType::Transparent),
        "U" => Some(JoiningType::NonJoining),
        _ => None,
    }
}

fn get_line_break_enum(name: &str) -> Option<LineBreak> {
    match name {
        "AI" => Some(LineBreak::Ambiguous),
        "AL" => Some(LineBreak::Alphabetic),
        "B2" => Some(LineBreak::BreakBoth),
        "BA" => Some(LineBreak::BreakAfter),
        "BB" => Some(LineBreak::BreakBefore),
        "BK" => Some(LineBreak::MandatoryBreak),
        "CB" => Some(LineBreak::ContingentBreak),
        "CJ" => Some(LineBreak::ConditionalJapaneseStarter),
        "CL" => Some(LineBreak::ClosePunctuation),
        "CM" => Some(LineBreak::CombiningMark),
        "CP" => Some(LineBreak::CloseParenthesis),
        "CR" => Some(LineBreak::CarriageReturn),
        "EB" => Some(LineBreak::EBase),
        "EM" => Some(LineBreak::EModifier),
        "EX" => Some(LineBreak::Exclamation),
        "GL" => Some(LineBreak::Glue),
        "H2" => Some(LineBreak::H2),
        "H3" => Some(LineBreak::H3),
        "HL" => Some(LineBreak::HebrewLetter),
        "HY" => Some(LineBreak::Hyphen),
        "ID" => Some(LineBreak::Ideographic),
        "IN" => Some(LineBreak::Inseperable),
        "IS" => Some(LineBreak::InfixNumeric),
        "JL" => Some(LineBreak::JL),
        "JT" => Some(LineBreak::JT),
        "JV" => Some(LineBreak::JV),
        "LF" => Some(LineBreak::LineFeed),
        "NL" => Some(LineBreak::NextLine),
        "NS" => Some(LineBreak::Nonstarter),
        "NU" => Some(LineBreak::Numeric),
        "OP" => Some(LineBreak::OpenPunctuation),
        "PO" => Some(LineBreak::PostfixNumeric),
        "PR" => Some(LineBreak::PrefixNumeric),
        "QU" => Some(LineBreak::Quotation),
        "RI" => Some(LineBreak::RegionalIndicator),
        "SA" => Some(LineBreak::ComplexContext),
        "SG" => Some(LineBreak::Surrogate),
        "SP" => Some(LineBreak::Space),
        "SY" => Some(LineBreak::BreakSymbols),
        "WJ" => Some(LineBreak::WordJoiner),
        "XX" => Some(LineBreak::Unknown),
        "ZW" => Some(LineBreak::ZWSpace),
        "ZWJ" => Some(LineBreak::ZWJ),
        _ => None,
    }
}

fn get_lead_canonical_combining_class_enum(name: &str) -> Option<LeadCanonicalCombiningClass> {
    match name {
        "0" => Some(LeadCanonicalCombiningClass::NotReordered),
        "1" => Some(LeadCanonicalCombiningClass::Overlay),
        "10" => Some(LeadCanonicalCombiningClass::CCC10),
        "103" => Some(LeadCanonicalCombiningClass::CCC103),
        "107" => Some(LeadCanonicalCombiningClass::CCC107),
        "11" => Some(LeadCanonicalCombiningClass::CCC11),
        "118" => Some(LeadCanonicalCombiningClass::CCC118),
        "12" => Some(LeadCanonicalCombiningClass::CCC12),
        "122" => Some(LeadCanonicalCombiningClass::CCC122),
        "129" => Some(LeadCanonicalCombiningClass::CCC129),
        "13" => Some(LeadCanonicalCombiningClass::CCC13),
        "130" => Some(LeadCanonicalCombiningClass::CCC130),
        "132" => Some(LeadCanonicalCombiningClass::CCC132),
        "133" => Some(LeadCanonicalCombiningClass::CCC133),
        "14" => Some(LeadCanonicalCombiningClass::CCC14),
        "15" => Some(LeadCanonicalCombiningClass::CCC15),
        "16" => Some(LeadCanonicalCombiningClass::CCC16),
        "17" => Some(LeadCanonicalCombiningClass::CCC17),
        "18" => Some(LeadCanonicalCombiningClass::CCC18),
        "19" => Some(LeadCanonicalCombiningClass::CCC19),
        "20" => Some(LeadCanonicalCombiningClass::CCC20),
        "200" => Some(LeadCanonicalCombiningClass::AttachedBelowLeft),
        "202" => Some(LeadCanonicalCombiningClass::AttachedBelow),
        "21" => Some(LeadCanonicalCombiningClass::CCC21),
        "214" => Some(LeadCanonicalCombiningClass::AttachedAbove),
        "216" => Some(LeadCanonicalCombiningClass::AttachedAboveRight),
        "218" => Some(LeadCanonicalCombiningClass::BelowLeft),
        "22" => Some(LeadCanonicalCombiningClass::CCC22),
        "220" => Some(LeadCanonicalCombiningClass::Below),
        "222" => Some(LeadCanonicalCombiningClass::BelowRight),
        "224" => Some(LeadCanonicalCombiningClass::Left),
        "226" => Some(LeadCanonicalCombiningClass::Right),
        "228" => Some(LeadCanonicalCombiningClass::AboveLeft),
        "23" => Some(LeadCanonicalCombiningClass::CCC23),
        "230" => Some(LeadCanonicalCombiningClass::Above),
        "232" => Some(LeadCanonicalCombiningClass::AboveRight),
        "233" => Some(LeadCanonicalCombiningClass::DoubleBelow),
        "234" => Some(LeadCanonicalCombiningClass::DoubleAbove),
        "24" => Some(LeadCanonicalCombiningClass::CCC24),
        "240" => Some(LeadCanonicalCombiningClass::IotaSubscript),
        "25" => Some(LeadCanonicalCombiningClass::CCC25),
        "26" => Some(LeadCanonicalCombiningClass::CCC26),
        "27" => Some(LeadCanonicalCombiningClass::CCC27),
        "28" => Some(LeadCanonicalCombiningClass::CCC28),
        "29" => Some(LeadCanonicalCombiningClass::CCC29),
        "30" => Some(LeadCanonicalCombiningClass::CCC30),
        "31" => Some(LeadCanonicalCombiningClass::CCC31),
        "32" => Some(LeadCanonicalCombiningClass::CCC32),
        "33" => Some(LeadCanonicalCombiningClass::CCC33),
        "34" => Some(LeadCanonicalCombiningClass::CCC34),
        "35" => Some(LeadCanonicalCombiningClass::CCC35),
        "36" => Some(LeadCanonicalCombiningClass::CCC36),
        "6" => Some(LeadCanonicalCombiningClass::HanReading),
        "7" => Some(LeadCanonicalCombiningClass::Nukta),
        "8" => Some(LeadCanonicalCombiningClass::KanaVoicing),
        "84" => Some(LeadCanonicalCombiningClass::CCC84),
        "9" => Some(LeadCanonicalCombiningClass::Virama),
        "91" => Some(LeadCanonicalCombiningClass::CCC91),
        _ => None,
    }
}

fn get_nfc_quick_check_enum(name: &str) -> Option<NFCQuickCheck> {
    match name {
        "M" => Some(NFCQuickCheck::Maybe),
        "N" => Some(NFCQuickCheck::No),
        "Y" => Some(NFCQuickCheck::Yes),
        _ => None,
    }
}

fn get_nfd_quick_check_enum(name: &str) -> Option<NFDQuickCheck> {
    match name {
        "N" => Some(NFDQuickCheck::No),
        "Y" => Some(NFDQuickCheck::Yes),
        _ => None,
    }
}

fn get_nfkc_quick_check_enum(name: &str) -> Option<NFKCQuickCheck> {
    match name {
        "M" => Some(NFKCQuickCheck::Maybe),
        "N" => Some(NFKCQuickCheck::No),
        "Y" => Some(NFKCQuickCheck::Yes),
        _ => None,
    }
}

fn get_nfkd_quick_check_enum(name: &str) -> Option<NFKDQuickCheck> {
    match name {
        "N" => Some(NFKDQuickCheck::No),
        "Y" => Some(NFKDQuickCheck::Yes),
        _ => None,
    }
}

fn get_numeric_type_enum(name: &str) -> Option<NumericType> {
    match name {
        "De" => Some(NumericType::Decimal),
        "Di" => Some(NumericType::Digit),
        "None" => Some(NumericType::None),
        "Nu" => Some(NumericType::Numeric),
        _ => None,
    }
}

fn get_sentence_break_enum(name: &str) -> Option<SentenceBreak> {
    match name {
        "AT" => Some(SentenceBreak::ATerm),
        "CL" => Some(SentenceBreak::Close),
        "CR" => Some(SentenceBreak::CR),
        "EX" => Some(SentenceBreak::Extend),
        "FO" => Some(SentenceBreak::Format),
        "LE" => Some(SentenceBreak::OLetter),
        "LF" => Some(SentenceBreak::LF),
        "LO" => Some(SentenceBreak::Lower),
        "NU" => Some(SentenceBreak::Numeric),
        "SC" => Some(SentenceBreak::SContinue),
        "SE" => Some(SentenceBreak::Sep),
        "SP" => Some(SentenceBreak::Sp),
        "ST" => Some(SentenceBreak::STerm),
        "UP" => Some(SentenceBreak::Upper),
        "XX" => Some(SentenceBreak::Other),
        _ => None,
    }
}

fn get_trail_canonical_combining_class_enum(name: &str) -> Option<TrailCanonicalCombiningClass> {
    match name {
        "0" => Some(TrailCanonicalCombiningClass::NotReordered),
        "1" => Some(TrailCanonicalCombiningClass::Overlay),
        "10" => Some(TrailCanonicalCombiningClass::CCC10),
        "103" => Some(TrailCanonicalCombiningClass::CCC103),
        "107" => Some(TrailCanonicalCombiningClass::CCC107),
        "11" => Some(TrailCanonicalCombiningClass::CCC11),
        "118" => Some(TrailCanonicalCombiningClass::CCC118),
        "12" => Some(TrailCanonicalCombiningClass::CCC12),
        "122" => Some(TrailCanonicalCombiningClass::CCC122),
        "129" => Some(TrailCanonicalCombiningClass::CCC129),
        "13" => Some(TrailCanonicalCombiningClass::CCC13),
        "130" => Some(TrailCanonicalCombiningClass::CCC130),
        "132" => Some(TrailCanonicalCombiningClass::CCC132),
        "133" => Some(TrailCanonicalCombiningClass::CCC133),
        "14" => Some(TrailCanonicalCombiningClass::CCC14),
        "15" => Some(TrailCanonicalCombiningClass::CCC15),
        "16" => Some(TrailCanonicalCombiningClass::CCC16),
        "17" => Some(TrailCanonicalCombiningClass::CCC17),
        "18" => Some(TrailCanonicalCombiningClass::CCC18),
        "19" => Some(TrailCanonicalCombiningClass::CCC19),
        "20" => Some(TrailCanonicalCombiningClass::CCC20),
        "200" => Some(TrailCanonicalCombiningClass::AttachedBelowLeft),
        "202" => Some(TrailCanonicalCombiningClass::AttachedBelow),
        "21" => Some(TrailCanonicalCombiningClass::CCC21),
        "214" => Some(TrailCanonicalCombiningClass::AttachedAbove),
        "216" => Some(TrailCanonicalCombiningClass::AttachedAboveRight),
        "218" => Some(TrailCanonicalCombiningClass::BelowLeft),
        "22" => Some(TrailCanonicalCombiningClass::CCC22),
        "220" => Some(TrailCanonicalCombiningClass::Below),
        "222" => Some(TrailCanonicalCombiningClass::BelowRight),
        "224" => Some(TrailCanonicalCombiningClass::Left),
        "226" => Some(TrailCanonicalCombiningClass::Right),
        "228" => Some(TrailCanonicalCombiningClass::AboveLeft),
        "23" => Some(TrailCanonicalCombiningClass::CCC23),
        "230" => Some(TrailCanonicalCombiningClass::Above),
        "232" => Some(TrailCanonicalCombiningClass::AboveRight),
        "233" => Some(TrailCanonicalCombiningClass::DoubleBelow),
        "234" => Some(TrailCanonicalCombiningClass::DoubleAbove),
        "24" => Some(TrailCanonicalCombiningClass::CCC24),
        "240" => Some(TrailCanonicalCombiningClass::IotaSubscript),
        "25" => Some(TrailCanonicalCombiningClass::CCC25),
        "26" => Some(TrailCanonicalCombiningClass::CCC26),
        "27" => Some(TrailCanonicalCombiningClass::CCC27),
        "28" => Some(TrailCanonicalCombiningClass::CCC28),
        "29" => Some(TrailCanonicalCombiningClass::CCC29),
        "30" => Some(TrailCanonicalCombiningClass::CCC30),
        "31" => Some(TrailCanonicalCombiningClass::CCC31),
        "32" => Some(TrailCanonicalCombiningClass::CCC32),
        "33" => Some(TrailCanonicalCombiningClass::CCC33),
        "34" => Some(TrailCanonicalCombiningClass::CCC34),
        "35" => Some(TrailCanonicalCombiningClass::CCC35),
        "36" => Some(TrailCanonicalCombiningClass::CCC36),
        "6" => Some(TrailCanonicalCombiningClass::HanReading),
        "7" => Some(TrailCanonicalCombiningClass::Nukta),
        "8" => Some(TrailCanonicalCombiningClass::KanaVoicing),
        "84" => Some(TrailCanonicalCombiningClass::CCC84),
        "9" => Some(TrailCanonicalCombiningClass::Virama),
        "91" => Some(TrailCanonicalCombiningClass::CCC91),
        _ => None,
    }
}

fn get_vertical_orientation_enum(name: &str) -> Option<VerticalOrientation> {
    match name {
        "R" => Some(VerticalOrientation::Rotated),
        "Tr" => Some(VerticalOrientation::TransformedRotated),
        "Tu" => Some(VerticalOrientation::TransformedUpright),
        "U" => Some(VerticalOrientation::Upright),
        _ => None,
    }
}

fn get_word_break_enum(name: &str) -> Option<WordBreak> {
    match name {
        "CR" => Some(WordBreak::CR),
        "DQ" => Some(WordBreak::DoubleQuote),
        "EB" => Some(WordBreak::EBase),
        "EBG" => Some(WordBreak::EBaseGAZ),
        "EM" => Some(WordBreak::EModifier),
        "EX" => Some(WordBreak::ExtendNumLet),
        "Extend" => Some(WordBreak::Extend),
        "FO" => Some(WordBreak::Format),
        "GAZ" => Some(WordBreak::GlueAfterZwj),
        "HL" => Some(WordBreak::HebrewLetter),
        "KA" => Some(WordBreak::Katakana),
        "LE" => Some(WordBreak::ALetter),
        "LF" => Some(WordBreak::LF),
        "MB" => Some(WordBreak::MidNumLet),
        "ML" => Some(WordBreak::MidLetter),
        "MN" => Some(WordBreak::MidNum),
        "NL" => Some(WordBreak::Newline),
        "NU" => Some(WordBreak::Numeric),
        "RI" => Some(WordBreak::RegionalIndicator),
        "SQ" => Some(WordBreak::SingleQuote),
        "WSegSpace" => Some(WordBreak::WSegSpace),
        "XX" => Some(WordBreak::Other),
        "ZWJ" => Some(WordBreak::ZWJ),
        _ => None,
    }
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
