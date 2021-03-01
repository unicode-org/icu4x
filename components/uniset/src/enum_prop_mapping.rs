// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::enum_props::*;
use litemap::LiteMap;

//
// Single getter function for enumerated property name:
// Enum prop name string -> Rust enum
//

fn get_enum_property_enum(name: &str) -> Option<EnumeratedProperty> {
    let mut m: LiteMap<&str, EnumeratedProperty> = LiteMap::new();
    m.insert("bc", EnumeratedProperty::BidiClass);
    m.insert("bpt", EnumeratedProperty::BidiPairedBracketType);
    m.insert("ccc", EnumeratedProperty::CanonicalCombiningClass);
    m.insert("dt", EnumeratedProperty::DecompositionType);
    m.insert("ea", EnumeratedProperty::EastAsianWidth);
    m.insert("gc", EnumeratedProperty::GeneralCategory);
    m.insert("GCB", EnumeratedProperty::GraphemeClusterBreak);
    m.insert("hst", EnumeratedProperty::HangulSyllableType);
    m.insert("InPC", EnumeratedProperty::IndicPositionalCategory);
    m.insert("InSC", EnumeratedProperty::IndicSyllabicCategory);
    m.insert("jg", EnumeratedProperty::JoiningGroup);
    m.insert("jt", EnumeratedProperty::JoiningType);
    m.insert("lb", EnumeratedProperty::LineBreak);
    m.insert("lccc", EnumeratedProperty::LeadCanonicalCombiningClass);
    m.insert("NFC_QC", EnumeratedProperty::NFCQuickCheck);
    m.insert("NFD_QC", EnumeratedProperty::NFDQuickCheck);
    m.insert("NFKC_QC", EnumeratedProperty::NFKCQuickCheck);
    m.insert("NFKD_QC", EnumeratedProperty::NFKDQuickCheck);
    m.insert("nt", EnumeratedProperty::NumericType);
    m.insert("SB", EnumeratedProperty::SentenceBreak);
    m.insert("tccc", EnumeratedProperty::TrailCanonicalCombiningClass);
    m.insert("vo", EnumeratedProperty::VerticalOrientation);
    m.insert("WB", EnumeratedProperty::WordBreak);
    m.get(name).cloned()
}

//
// Getter function per enumerated property:
// Enum prop val string -> Rust enum
//

fn get_bidi_class_enum(name: &str) -> Option<BidiClass> {
    let mut m: LiteMap<&str, BidiClass> = LiteMap::new();
    m.insert("AL", BidiClass::ArabicLetter);
    m.insert("AN", BidiClass::ArabicNumber);
    m.insert("B", BidiClass::ParagraphSeparator);
    m.insert("BN", BidiClass::BoundaryNeutral);
    m.insert("CS", BidiClass::CommonSeparator);
    m.insert("EN", BidiClass::EuropeanNumber);
    m.insert("ES", BidiClass::EuropeanSeparator);
    m.insert("ET", BidiClass::EuropeanTerminator);
    m.insert("FSI", BidiClass::FirstStrongIsolate);
    m.insert("L", BidiClass::LeftToRight);
    m.insert("LRE", BidiClass::LeftToRightEmbedding);
    m.insert("LRI", BidiClass::LeftToRightIsolate);
    m.insert("LRO", BidiClass::LeftToRightOverride);
    m.insert("NSM", BidiClass::NonspacingMark);
    m.insert("ON", BidiClass::OtherNeutral);
    m.insert("PDF", BidiClass::PopDirectionalFormat);
    m.insert("PDI", BidiClass::PopDirectionalIsolate);
    m.insert("R", BidiClass::RightToLeft);
    m.insert("RLE", BidiClass::RightToLeftEmbedding);
    m.insert("RLI", BidiClass::RightToLeftIsolate);
    m.insert("RLO", BidiClass::RightToLeftOverride);
    m.insert("S", BidiClass::SegmentSeparator);
    m.insert("WS", BidiClass::WhiteSpace);
    m.get(name).cloned()
}

fn get_bidi_paired_bracket_type_enum(name: &str) -> Option<BidiPairedBracketType> {
    let mut m: LiteMap<&str, BidiPairedBracketType> = LiteMap::new();
    m.insert("c", BidiPairedBracketType::Close);
    m.insert("n", BidiPairedBracketType::None);
    m.insert("o", BidiPairedBracketType::Open);
    m.get(name).cloned()
}

fn get_canonical_combining_class_enum(name: &str) -> Option<CanonicalCombiningClass> {
    let mut m: LiteMap<&str, CanonicalCombiningClass> = LiteMap::new();
    m.insert("0", CanonicalCombiningClass::NotReordered);
    m.insert("1", CanonicalCombiningClass::Overlay);
    m.insert("10", CanonicalCombiningClass::CCC10);
    m.insert("103", CanonicalCombiningClass::CCC103);
    m.insert("107", CanonicalCombiningClass::CCC107);
    m.insert("11", CanonicalCombiningClass::CCC11);
    m.insert("118", CanonicalCombiningClass::CCC118);
    m.insert("12", CanonicalCombiningClass::CCC12);
    m.insert("122", CanonicalCombiningClass::CCC122);
    m.insert("129", CanonicalCombiningClass::CCC129);
    m.insert("13", CanonicalCombiningClass::CCC13);
    m.insert("130", CanonicalCombiningClass::CCC130);
    m.insert("132", CanonicalCombiningClass::CCC132);
    m.insert("133", CanonicalCombiningClass::CCC133);
    m.insert("14", CanonicalCombiningClass::CCC14);
    m.insert("15", CanonicalCombiningClass::CCC15);
    m.insert("16", CanonicalCombiningClass::CCC16);
    m.insert("17", CanonicalCombiningClass::CCC17);
    m.insert("18", CanonicalCombiningClass::CCC18);
    m.insert("19", CanonicalCombiningClass::CCC19);
    m.insert("20", CanonicalCombiningClass::CCC20);
    m.insert("200", CanonicalCombiningClass::AttachedBelowLeft);
    m.insert("202", CanonicalCombiningClass::AttachedBelow);
    m.insert("21", CanonicalCombiningClass::CCC21);
    m.insert("214", CanonicalCombiningClass::AttachedAbove);
    m.insert("216", CanonicalCombiningClass::AttachedAboveRight);
    m.insert("218", CanonicalCombiningClass::BelowLeft);
    m.insert("22", CanonicalCombiningClass::CCC22);
    m.insert("220", CanonicalCombiningClass::Below);
    m.insert("222", CanonicalCombiningClass::BelowRight);
    m.insert("224", CanonicalCombiningClass::Left);
    m.insert("226", CanonicalCombiningClass::Right);
    m.insert("228", CanonicalCombiningClass::AboveLeft);
    m.insert("23", CanonicalCombiningClass::CCC23);
    m.insert("230", CanonicalCombiningClass::Above);
    m.insert("232", CanonicalCombiningClass::AboveRight);
    m.insert("233", CanonicalCombiningClass::DoubleBelow);
    m.insert("234", CanonicalCombiningClass::DoubleAbove);
    m.insert("24", CanonicalCombiningClass::CCC24);
    m.insert("240", CanonicalCombiningClass::IotaSubscript);
    m.insert("25", CanonicalCombiningClass::CCC25);
    m.insert("26", CanonicalCombiningClass::CCC26);
    m.insert("27", CanonicalCombiningClass::CCC27);
    m.insert("28", CanonicalCombiningClass::CCC28);
    m.insert("29", CanonicalCombiningClass::CCC29);
    m.insert("30", CanonicalCombiningClass::CCC30);
    m.insert("31", CanonicalCombiningClass::CCC31);
    m.insert("32", CanonicalCombiningClass::CCC32);
    m.insert("33", CanonicalCombiningClass::CCC33);
    m.insert("34", CanonicalCombiningClass::CCC34);
    m.insert("35", CanonicalCombiningClass::CCC35);
    m.insert("36", CanonicalCombiningClass::CCC36);
    m.insert("6", CanonicalCombiningClass::HanReading);
    m.insert("7", CanonicalCombiningClass::Nukta);
    m.insert("8", CanonicalCombiningClass::KanaVoicing);
    m.insert("84", CanonicalCombiningClass::CCC84);
    m.insert("9", CanonicalCombiningClass::Virama);
    m.insert("91", CanonicalCombiningClass::CCC91);
    m.get(name).cloned()
}

fn get_decomposition_type_enum(name: &str) -> Option<DecompositionType> {
    let mut m: LiteMap<&str, DecompositionType> = LiteMap::new();
    m.insert("Can", DecompositionType::Can);
    m.insert("Com", DecompositionType::Com);
    m.insert("Enc", DecompositionType::Enc);
    m.insert("Fin", DecompositionType::Fin);
    m.insert("Font", DecompositionType::Font);
    m.insert("Fra", DecompositionType::Fra);
    m.insert("Init", DecompositionType::Init);
    m.insert("Iso", DecompositionType::Iso);
    m.insert("Med", DecompositionType::Med);
    m.insert("Nar", DecompositionType::Nar);
    m.insert("Nb", DecompositionType::Nb);
    m.insert("None", DecompositionType::None);
    m.insert("Sml", DecompositionType::Sml);
    m.insert("Sqr", DecompositionType::Sqr);
    m.insert("Sub", DecompositionType::Sub);
    m.insert("Sup", DecompositionType::Sup);
    m.insert("Vert", DecompositionType::Vert);
    m.insert("Wide", DecompositionType::Wide);
    m.get(name).cloned()
}

fn get_east_asian_width_enum(name: &str) -> Option<EastAsianWidth> {
    let mut m: LiteMap<&str, EastAsianWidth> = LiteMap::new();
    m.insert("A", EastAsianWidth::Ambiguous);
    m.insert("F", EastAsianWidth::Fullwidth);
    m.insert("H", EastAsianWidth::Halfwidth);
    m.insert("N", EastAsianWidth::Neutral);
    m.insert("Na", EastAsianWidth::Narrow);
    m.insert("W", EastAsianWidth::Wide);
    m.get(name).cloned()
}

fn get_general_category_enum(name: &str) -> Option<GeneralCategory> {
    let mut m: LiteMap<&str, GeneralCategory> = LiteMap::new();
    m.insert("C", GeneralCategory::Other);
    m.insert("Cc", GeneralCategory::Cntrl);
    m.insert("Cf", GeneralCategory::Format);
    m.insert("Cn", GeneralCategory::Unassigned);
    m.insert("Co", GeneralCategory::PrivateUse);
    m.insert("Cs", GeneralCategory::Surrogate);
    m.insert("L", GeneralCategory::Letter);
    m.insert("LC", GeneralCategory::CasedLetter);
    m.insert("Ll", GeneralCategory::LowercaseLetter);
    m.insert("Lm", GeneralCategory::ModifierLetter);
    m.insert("Lo", GeneralCategory::OtherLetter);
    m.insert("Lt", GeneralCategory::TitlecaseLetter);
    m.insert("Lu", GeneralCategory::UppercaseLetter);
    m.insert("M", GeneralCategory::CombiningMark);
    m.insert("Mc", GeneralCategory::SpacingMark);
    m.insert("Me", GeneralCategory::EnclosingMark);
    m.insert("Mn", GeneralCategory::NonspacingMark);
    m.insert("N", GeneralCategory::Number);
    m.insert("Nd", GeneralCategory::Digit);
    m.insert("Nl", GeneralCategory::LetterNumber);
    m.insert("No", GeneralCategory::OtherNumber);
    m.insert("P", GeneralCategory::Punct);
    m.insert("Pc", GeneralCategory::ConnectorPunctuation);
    m.insert("Pd", GeneralCategory::DashPunctuation);
    m.insert("Pe", GeneralCategory::ClosePunctuation);
    m.insert("Pf", GeneralCategory::FinalPunctuation);
    m.insert("Pi", GeneralCategory::InitialPunctuation);
    m.insert("Po", GeneralCategory::OtherPunctuation);
    m.insert("Ps", GeneralCategory::OpenPunctuation);
    m.insert("S", GeneralCategory::Symbol);
    m.insert("Sc", GeneralCategory::CurrencySymbol);
    m.insert("Sk", GeneralCategory::ModifierSymbol);
    m.insert("Sm", GeneralCategory::MathSymbol);
    m.insert("So", GeneralCategory::OtherSymbol);
    m.insert("Z", GeneralCategory::Separator);
    m.insert("Zl", GeneralCategory::LineSeparator);
    m.insert("Zp", GeneralCategory::ParagraphSeparator);
    m.insert("Zs", GeneralCategory::SpaceSeparator);
    m.get(name).cloned()
}

fn get_grapheme_cluster_break_enum(name: &str) -> Option<GraphemeClusterBreak> {
    let mut m: LiteMap<&str, GraphemeClusterBreak> = LiteMap::new();
    m.insert("CN", GraphemeClusterBreak::Control);
    m.insert("CR", GraphemeClusterBreak::CR);
    m.insert("EB", GraphemeClusterBreak::EBase);
    m.insert("EBG", GraphemeClusterBreak::EBaseGAZ);
    m.insert("EM", GraphemeClusterBreak::EModifier);
    m.insert("EX", GraphemeClusterBreak::Extend);
    m.insert("GAZ", GraphemeClusterBreak::GlueAfterZwj);
    m.insert("L", GraphemeClusterBreak::L);
    m.insert("LF", GraphemeClusterBreak::LF);
    m.insert("LV", GraphemeClusterBreak::LV);
    m.insert("LVT", GraphemeClusterBreak::LVT);
    m.insert("PP", GraphemeClusterBreak::Prepend);
    m.insert("RI", GraphemeClusterBreak::RegionalIndicator);
    m.insert("SM", GraphemeClusterBreak::SpacingMark);
    m.insert("T", GraphemeClusterBreak::T);
    m.insert("V", GraphemeClusterBreak::V);
    m.insert("XX", GraphemeClusterBreak::Other);
    m.insert("ZWJ", GraphemeClusterBreak::ZWJ);
    m.get(name).cloned()
}

fn get_hangul_syllable_type_enum(name: &str) -> Option<HangulSyllableType> {
    let mut m: LiteMap<&str, HangulSyllableType> = LiteMap::new();
    m.insert("L", HangulSyllableType::LeadingJamo);
    m.insert("LV", HangulSyllableType::LVSyllable);
    m.insert("LVT", HangulSyllableType::LVTSyllable);
    m.insert("NA", HangulSyllableType::NotApplicable);
    m.insert("T", HangulSyllableType::TrailingJamo);
    m.insert("V", HangulSyllableType::VowelJamo);
    m.get(name).cloned()
}

fn get_indic_positional_category_enum(name: &str) -> Option<IndicPositionalCategory> {
    let mut m: LiteMap<&str, IndicPositionalCategory> = LiteMap::new();

    m.insert("Bottom", IndicPositionalCategory::Bottom);
    m.insert("Bottom_And_Left", IndicPositionalCategory::BottomAndLeft);
    m.insert("Bottom_And_Right", IndicPositionalCategory::BottomAndRight);
    m.insert("Left", IndicPositionalCategory::Left);
    m.insert("Left_And_Right", IndicPositionalCategory::LeftAndRight);
    m.insert("NA", IndicPositionalCategory::NA);
    m.insert("Overstruck", IndicPositionalCategory::Overstruck);
    m.insert("Right", IndicPositionalCategory::Right);
    m.insert("Top", IndicPositionalCategory::Top);
    m.insert("Top_And_Bottom", IndicPositionalCategory::TopAndBottom);
    m.insert("Top_And_Bottom_And_Left", IndicPositionalCategory::TopAndBottomAndLeft);
    m.insert("Top_And_Bottom_And_Right", IndicPositionalCategory::TopAndBottomAndRight);
    m.insert("Top_And_Left", IndicPositionalCategory::TopAndLeft);
    m.insert("Top_And_Left_And_Right", IndicPositionalCategory::TopAndLeftAndRight);
    m.insert("Top_And_Right", IndicPositionalCategory::TopAndRight);
    m.insert("Visual_Order_Left", IndicPositionalCategory::VisualOrderLeft);
    m.get(name).cloned()
}

fn get_indic_syllabic_category_enum(name: &str) -> Option<IndicSyllabicCategory> {
    let mut m: LiteMap<&str, IndicSyllabicCategory> = LiteMap::new();
    m.insert("Avagraha", IndicSyllabicCategory::Avagraha);
    m.insert("Bindu", IndicSyllabicCategory::Bindu);
    m.insert("Brahmi_Joining_Number", IndicSyllabicCategory::BrahmiJoiningNumber);
    m.insert("Cantillation_Mark", IndicSyllabicCategory::CantillationMark);
    m.insert("Consonant", IndicSyllabicCategory::Consonant);
    m.insert("Consonant_Dead", IndicSyllabicCategory::ConsonantDead);
    m.insert("Consonant_Final", IndicSyllabicCategory::ConsonantFinal);
    m.insert("Consonant_Head_Letter", IndicSyllabicCategory::ConsonantHeadLetter);
    m.insert("Consonant_Initial_Postfixed", IndicSyllabicCategory::ConsonantInitialPostfixed);
    m.insert("Consonant_Killer", IndicSyllabicCategory::ConsonantKiller);
    m.insert("Consonant_Medial", IndicSyllabicCategory::ConsonantMedial);
    m.insert("Consonant_Placeholder", IndicSyllabicCategory::ConsonantPlaceholder);
    m.insert("Consonant_Preceding_Repha", IndicSyllabicCategory::ConsonantPrecedingRepha);
    m.insert("Consonant_Prefixed", IndicSyllabicCategory::ConsonantPrefixed);
    m.insert("Consonant_Subjoined", IndicSyllabicCategory::ConsonantSubjoined);
    m.insert("Consonant_Succeeding_Repha", IndicSyllabicCategory::ConsonantSucceedingRepha);
    m.insert("Consonant_With_Stacker", IndicSyllabicCategory::ConsonantWithStacker);
    m.insert("Gemination_Mark", IndicSyllabicCategory::GeminationMark);
    m.insert("Invisible_Stacker", IndicSyllabicCategory::InvisibleStacker);
    m.insert("Joiner", IndicSyllabicCategory::Joiner);
    m.insert("Modifying_Letter", IndicSyllabicCategory::ModifyingLetter);
    m.insert("Non_Joiner", IndicSyllabicCategory::NonJoiner);
    m.insert("Nukta", IndicSyllabicCategory::Nukta);
    m.insert("Number", IndicSyllabicCategory::Number);
    m.insert("Number_Joiner", IndicSyllabicCategory::NumberJoiner);
    m.insert("Other", IndicSyllabicCategory::Other);
    m.insert("Pure_Killer", IndicSyllabicCategory::PureKiller);
    m.insert("Register_Shifter", IndicSyllabicCategory::RegisterShifter);
    m.insert("Syllable_Modifier", IndicSyllabicCategory::SyllableModifier);
    m.insert("Tone_Letter", IndicSyllabicCategory::ToneLetter);
    m.insert("Tone_Mark", IndicSyllabicCategory::ToneMark);
    m.insert("Virama", IndicSyllabicCategory::Virama);
    m.insert("Visarga", IndicSyllabicCategory::Visarga);
    m.insert("Vowel", IndicSyllabicCategory::Vowel);
    m.insert("Vowel_Dependent", IndicSyllabicCategory::VowelDependent);
    m.insert("Vowel_Independent", IndicSyllabicCategory::VowelIndependent);
    m.get(name).cloned()
}

fn get_joining_group_enum(name: &str) -> Option<JoiningGroup> {
    let mut m: LiteMap<&str, JoiningGroup> = LiteMap::new();
    m.insert("African_Feh", JoiningGroup::AfricanFeh);
    m.insert("African_Noon", JoiningGroup::AfricanNoon);
    m.insert("African_Qaf", JoiningGroup::AfricanQaf);
    m.insert("Ain", JoiningGroup::Ain);
    m.insert("Alaph", JoiningGroup::Alaph);
    m.insert("Alef", JoiningGroup::Alef);
    m.insert("Beh", JoiningGroup::Beh);
    m.insert("Beth", JoiningGroup::Beth);
    m.insert("Burushaski_Yeh_Barree", JoiningGroup::BurushaskiYehBarree);
    m.insert("Dal", JoiningGroup::Dal);
    m.insert("Dalath_Rish", JoiningGroup::DalathRish);
    m.insert("E", JoiningGroup::E);
    m.insert("Farsi_Yeh", JoiningGroup::FarsiYeh);
    m.insert("Fe", JoiningGroup::Fe);
    m.insert("Feh", JoiningGroup::Feh);
    m.insert("Final_Semkath", JoiningGroup::FinalSemkath);
    m.insert("Gaf", JoiningGroup::Gaf);
    m.insert("Gamal", JoiningGroup::Gamal);
    m.insert("Hah", JoiningGroup::Hah);
    m.insert("Hanifi_Rohingya_Kinna_Ya", JoiningGroup::HanifiRohingyaKinnaYa);
    m.insert("Hanifi_Rohingya_Pa", JoiningGroup::HanifiRohingyaPa);
    m.insert("He", JoiningGroup::He);
    m.insert("Heh", JoiningGroup::Heh);
    m.insert("Heh_Goal", JoiningGroup::HehGoal);
    m.insert("Heth", JoiningGroup::Heth);
    m.insert("Kaf", JoiningGroup::Kaf);
    m.insert("Kaph", JoiningGroup::Kaph);
    m.insert("Khaph", JoiningGroup::Khaph);
    m.insert("Knotted_Heh", JoiningGroup::KnottedHeh);
    m.insert("Lam", JoiningGroup::Lam);
    m.insert("Lamadh", JoiningGroup::Lamadh);
    m.insert("Malayalam_Bha", JoiningGroup::MalayalamBha);
    m.insert("Malayalam_Ja", JoiningGroup::MalayalamJa);
    m.insert("Malayalam_Lla", JoiningGroup::MalayalamLla);
    m.insert("Malayalam_Llla", JoiningGroup::MalayalamLlla);
    m.insert("Malayalam_Nga", JoiningGroup::MalayalamNga);
    m.insert("Malayalam_Nna", JoiningGroup::MalayalamNna);
    m.insert("Malayalam_Nnna", JoiningGroup::MalayalamNnna);
    m.insert("Malayalam_Nya", JoiningGroup::MalayalamNya);
    m.insert("Malayalam_Ra", JoiningGroup::MalayalamRa);
    m.insert("Malayalam_Ssa", JoiningGroup::MalayalamSsa);
    m.insert("Malayalam_Tta", JoiningGroup::MalayalamTta);
    m.insert("Manichaean_Aleph", JoiningGroup::ManichaeanAleph);
    m.insert("Manichaean_Ayin", JoiningGroup::ManichaeanAyin);
    m.insert("Manichaean_Beth", JoiningGroup::ManichaeanBeth);
    m.insert("Manichaean_Daleth", JoiningGroup::ManichaeanDaleth);
    m.insert("Manichaean_Dhamedh", JoiningGroup::ManichaeanDhamedh);
    m.insert("Manichaean_Five", JoiningGroup::ManichaeanFive);
    m.insert("Manichaean_Gimel", JoiningGroup::ManichaeanGimel);
    m.insert("Manichaean_Heth", JoiningGroup::ManichaeanHeth);
    m.insert("Manichaean_Hundred", JoiningGroup::ManichaeanHundred);
    m.insert("Manichaean_Kaph", JoiningGroup::ManichaeanKaph);
    m.insert("Manichaean_Lamedh", JoiningGroup::ManichaeanLamedh);
    m.insert("Manichaean_Mem", JoiningGroup::ManichaeanMem);
    m.insert("Manichaean_Nun", JoiningGroup::ManichaeanNun);
    m.insert("Manichaean_One", JoiningGroup::ManichaeanOne);
    m.insert("Manichaean_Pe", JoiningGroup::ManichaeanPe);
    m.insert("Manichaean_Qoph", JoiningGroup::ManichaeanQoph);
    m.insert("Manichaean_Resh", JoiningGroup::ManichaeanResh);
    m.insert("Manichaean_Sadhe", JoiningGroup::ManichaeanSadhe);
    m.insert("Manichaean_Samekh", JoiningGroup::ManichaeanSamekh);
    m.insert("Manichaean_Taw", JoiningGroup::ManichaeanTaw);
    m.insert("Manichaean_Ten", JoiningGroup::ManichaeanTen);
    m.insert("Manichaean_Teth", JoiningGroup::ManichaeanTeth);
    m.insert("Manichaean_Thamedh", JoiningGroup::ManichaeanThamedh);
    m.insert("Manichaean_Twenty", JoiningGroup::ManichaeanTwenty);
    m.insert("Manichaean_Waw", JoiningGroup::ManichaeanWaw);
    m.insert("Manichaean_Yodh", JoiningGroup::ManichaeanYodh);
    m.insert("Manichaean_Zayin", JoiningGroup::ManichaeanZayin);
    m.insert("Meem", JoiningGroup::Meem);
    m.insert("Mim", JoiningGroup::Mim);
    m.insert("No_Joining_Group", JoiningGroup::NoJoiningGroup);
    m.insert("Noon", JoiningGroup::Noon);
    m.insert("Nun", JoiningGroup::Nun);
    m.insert("Nya", JoiningGroup::Nya);
    m.insert("Pe", JoiningGroup::Pe);
    m.insert("Qaf", JoiningGroup::Qaf);
    m.insert("Qaph", JoiningGroup::Qaph);
    m.insert("Reh", JoiningGroup::Reh);
    m.insert("Reversed_Pe", JoiningGroup::ReversedPe);
    m.insert("Rohingya_Yeh", JoiningGroup::RohingyaYeh);
    m.insert("Sad", JoiningGroup::Sad);
    m.insert("Sadhe", JoiningGroup::Sadhe);
    m.insert("Seen", JoiningGroup::Seen);
    m.insert("Semkath", JoiningGroup::Semkath);
    m.insert("Shin", JoiningGroup::Shin);
    m.insert("Straight_Waw", JoiningGroup::StraightWaw);
    m.insert("Swash_Kaf", JoiningGroup::SwashKaf);
    m.insert("Syriac_Waw", JoiningGroup::SyriacWaw);
    m.insert("Tah", JoiningGroup::Tah);
    m.insert("Taw", JoiningGroup::Taw);
    m.insert("Teh_Marbuta", JoiningGroup::TehMarbuta);
    m.insert("Teh_Marbuta_Goal", JoiningGroup::TehMarbutaGoal);
    m.insert("Teth", JoiningGroup::Teth);
    m.insert("Waw", JoiningGroup::Waw);
    m.insert("Yeh", JoiningGroup::Yeh);
    m.insert("Yeh_Barree", JoiningGroup::YehBarree);
    m.insert("Yeh_With_Tail", JoiningGroup::YehWithTail);
    m.insert("Yudh", JoiningGroup::Yudh);
    m.insert("Yudh_He", JoiningGroup::YudhHe);
    m.insert("Zain", JoiningGroup::Zain);
    m.insert("Zhain", JoiningGroup::Zhain);
    m.get(name).cloned()        
}

fn get_joining_type_enum(name: &str) -> Option<JoiningType> {
    let mut m: LiteMap<&str, JoiningType> = LiteMap::new();
    m.insert("C", JoiningType::JoinCausing);
    m.insert("D", JoiningType::DualJoining);
    m.insert("L", JoiningType::LeftJoining);
    m.insert("R", JoiningType::RightJoining);
    m.insert("T", JoiningType::Transparent);
    m.insert("U", JoiningType::NonJoining,   );
    m.get(name).cloned()
}

fn get_line_break_enum(name: &str) -> Option<LineBreak> {
    let mut m: LiteMap<&str, LineBreak> = LiteMap::new();
    m.insert("AI", LineBreak::Ambiguous);
    m.insert("AL", LineBreak::Alphabetic);
    m.insert("B2", LineBreak::BreakBoth);
    m.insert("BA", LineBreak::BreakAfter);
    m.insert("BB", LineBreak::BreakBefore);
    m.insert("BK", LineBreak::MandatoryBreak);
    m.insert("CB", LineBreak::ContingentBreak);
    m.insert("CJ", LineBreak::ConditionalJapaneseStarter);
    m.insert("CL", LineBreak::ClosePunctuation);
    m.insert("CM", LineBreak::CombiningMark);
    m.insert("CP", LineBreak::CloseParenthesis);
    m.insert("CR", LineBreak::CarriageReturn);
    m.insert("EB", LineBreak::EBase);
    m.insert("EM", LineBreak::EModifier);
    m.insert("EX", LineBreak::Exclamation);
    m.insert("GL", LineBreak::Glue);
    m.insert("H2", LineBreak::H2);
    m.insert("H3", LineBreak::H3);
    m.insert("HL", LineBreak::HebrewLetter);
    m.insert("HY", LineBreak::Hyphen);
    m.insert("ID", LineBreak::Ideographic);
    m.insert("IN", LineBreak::Inseperable);
    m.insert("IS", LineBreak::InfixNumeric);
    m.insert("JL", LineBreak::JL);
    m.insert("JT", LineBreak::JT);
    m.insert("JV", LineBreak::JV);
    m.insert("LF", LineBreak::LineFeed);
    m.insert("NL", LineBreak::NextLine);
    m.insert("NS", LineBreak::Nonstarter);
    m.insert("NU", LineBreak::Numeric);
    m.insert("OP", LineBreak::OpenPunctuation);
    m.insert("PO", LineBreak::PostfixNumeric);
    m.insert("PR", LineBreak::PrefixNumeric);
    m.insert("QU", LineBreak::Quotation);
    m.insert("RI", LineBreak::RegionalIndicator);
    m.insert("SA", LineBreak::ComplexContext);
    m.insert("SG", LineBreak::Surrogate);
    m.insert("SP", LineBreak::Space);
    m.insert("SY", LineBreak::BreakSymbols);
    m.insert("WJ", LineBreak::WordJoiner);
    m.insert("XX", LineBreak::Unknown);
    m.insert("ZW", LineBreak::ZWSpace);
    m.insert("ZWJ", LineBreak::ZWJ);
    m.get(name).cloned()
}

fn get_lead_canonical_combining_class_enum(name: &str) -> Option<LeadCanonicalCombiningClass> {
    let mut m: LiteMap<&str, LeadCanonicalCombiningClass> = LiteMap::new();
    m.insert("0", LeadCanonicalCombiningClass::NotReordered);
    m.insert("1", LeadCanonicalCombiningClass::Overlay);
    m.insert("10", LeadCanonicalCombiningClass::CCC10);
    m.insert("103", LeadCanonicalCombiningClass::CCC103);
    m.insert("107", LeadCanonicalCombiningClass::CCC107);
    m.insert("11", LeadCanonicalCombiningClass::CCC11);
    m.insert("118", LeadCanonicalCombiningClass::CCC118);
    m.insert("12", LeadCanonicalCombiningClass::CCC12);
    m.insert("122", LeadCanonicalCombiningClass::CCC122);
    m.insert("129", LeadCanonicalCombiningClass::CCC129);
    m.insert("13", LeadCanonicalCombiningClass::CCC13);
    m.insert("130", LeadCanonicalCombiningClass::CCC130);
    m.insert("132", LeadCanonicalCombiningClass::CCC132);
    m.insert("133", LeadCanonicalCombiningClass::CCC133);
    m.insert("14", LeadCanonicalCombiningClass::CCC14);
    m.insert("15", LeadCanonicalCombiningClass::CCC15);
    m.insert("16", LeadCanonicalCombiningClass::CCC16);
    m.insert("17", LeadCanonicalCombiningClass::CCC17);
    m.insert("18", LeadCanonicalCombiningClass::CCC18);
    m.insert("19", LeadCanonicalCombiningClass::CCC19);
    m.insert("20", LeadCanonicalCombiningClass::CCC20);
    m.insert("200", LeadCanonicalCombiningClass::AttachedBelowLeft);
    m.insert("202", LeadCanonicalCombiningClass::AttachedBelow);
    m.insert("21", LeadCanonicalCombiningClass::CCC21);
    m.insert("214", LeadCanonicalCombiningClass::AttachedAbove);
    m.insert("216", LeadCanonicalCombiningClass::AttachedAboveRight);
    m.insert("218", LeadCanonicalCombiningClass::BelowLeft);
    m.insert("22", LeadCanonicalCombiningClass::CCC22);
    m.insert("220", LeadCanonicalCombiningClass::Below);
    m.insert("222", LeadCanonicalCombiningClass::BelowRight);
    m.insert("224", LeadCanonicalCombiningClass::Left);
    m.insert("226", LeadCanonicalCombiningClass::Right);
    m.insert("228", LeadCanonicalCombiningClass::AboveLeft);
    m.insert("23", LeadCanonicalCombiningClass::CCC23);
    m.insert("230", LeadCanonicalCombiningClass::Above);
    m.insert("232", LeadCanonicalCombiningClass::AboveRight);
    m.insert("233", LeadCanonicalCombiningClass::DoubleBelow);
    m.insert("234", LeadCanonicalCombiningClass::DoubleAbove);
    m.insert("24", LeadCanonicalCombiningClass::CCC24);
    m.insert("240", LeadCanonicalCombiningClass::IotaSubscript);
    m.insert("25", LeadCanonicalCombiningClass::CCC25);
    m.insert("26", LeadCanonicalCombiningClass::CCC26);
    m.insert("27", LeadCanonicalCombiningClass::CCC27);
    m.insert("28", LeadCanonicalCombiningClass::CCC28);
    m.insert("29", LeadCanonicalCombiningClass::CCC29);
    m.insert("30", LeadCanonicalCombiningClass::CCC30);
    m.insert("31", LeadCanonicalCombiningClass::CCC31);
    m.insert("32", LeadCanonicalCombiningClass::CCC32);
    m.insert("33", LeadCanonicalCombiningClass::CCC33);
    m.insert("34", LeadCanonicalCombiningClass::CCC34);
    m.insert("35", LeadCanonicalCombiningClass::CCC35);
    m.insert("36", LeadCanonicalCombiningClass::CCC36);
    m.insert("6", LeadCanonicalCombiningClass::HanReading);
    m.insert("7", LeadCanonicalCombiningClass::Nukta);
    m.insert("8", LeadCanonicalCombiningClass::KanaVoicing);
    m.insert("84", LeadCanonicalCombiningClass::CCC84);
    m.insert("9", LeadCanonicalCombiningClass::Virama);
    m.insert("91", LeadCanonicalCombiningClass::CCC91);
    m.get(name).cloned()
}

fn get_nfc_quick_check_enum(name: &str) -> Option<NFCQuickCheck> {
    let mut m: LiteMap<&str, NFCQuickCheck> = LiteMap::new();
    m.insert("M", NFCQuickCheck::Maybe);
    m.insert("N", NFCQuickCheck::No);
    m.insert("Y", NFCQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_nfd_quick_check_enum(name: &str) -> Option<NFDQuickCheck> {
    let mut m: LiteMap<&str, NFDQuickCheck> = LiteMap::new();
    m.insert("N", NFDQuickCheck::No);
    m.insert("Y", NFDQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_nfkc_quick_check_enum(name: &str) -> Option<NFKCQuickCheck> {
    let mut m: LiteMap<&str, NFKCQuickCheck> = LiteMap::new();
    m.insert("M", NFKCQuickCheck::Maybe);
    m.insert("N", NFKCQuickCheck::No);
    m.insert("Y", NFKCQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_nfkd_quick_check_enum(name: &str) -> Option<NFKDQuickCheck> {
    let mut m: LiteMap<&str, NFKDQuickCheck> = LiteMap::new();
    m.insert("N", NFKDQuickCheck::No);
    m.insert("Y", NFKDQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_numeric_type_enum(name: &str) -> Option<NumericType> {
    let mut m: LiteMap<&str, NumericType> = LiteMap::new();
    m.insert("De", NumericType::Decimal);
    m.insert("Di", NumericType::Digit);
    m.insert("None", NumericType::None);
    m.insert("Nu", NumericType::Numeric);
    m.get(name).cloned()
}

fn get_sentence_break_enum(name: &str) -> Option<SentenceBreak> {
    let mut m: LiteMap<&str, SentenceBreak> = LiteMap::new();
    m.insert("AT", SentenceBreak::ATerm);
    m.insert("CL", SentenceBreak::Close);
    m.insert("CR", SentenceBreak::CR);
    m.insert("EX", SentenceBreak::Extend);
    m.insert("FO", SentenceBreak::Format);
    m.insert("LE", SentenceBreak::OLetter);
    m.insert("LF", SentenceBreak::LF);
    m.insert("LO", SentenceBreak::Lower);
    m.insert("NU", SentenceBreak::Numeric);
    m.insert("SC", SentenceBreak::SContinue);
    m.insert("SE", SentenceBreak::Sep);
    m.insert("SP", SentenceBreak::Sp);
    m.insert("ST", SentenceBreak::STerm);
    m.insert("UP", SentenceBreak::Upper);
    m.insert("XX", SentenceBreak::Other);
    m.get(name).cloned()    
}

fn get_trail_canonical_combining_class_enum(name: &str) -> Option<TrailCanonicalCombiningClass> {
    let mut m: LiteMap<&str, TrailCanonicalCombiningClass> = LiteMap::new();
    m.insert("0", TrailCanonicalCombiningClass::NotReordered);
    m.insert("1", TrailCanonicalCombiningClass::Overlay);
    m.insert("10", TrailCanonicalCombiningClass::CCC10);
    m.insert("103", TrailCanonicalCombiningClass::CCC103);
    m.insert("107", TrailCanonicalCombiningClass::CCC107);
    m.insert("11", TrailCanonicalCombiningClass::CCC11);
    m.insert("118", TrailCanonicalCombiningClass::CCC118);
    m.insert("12", TrailCanonicalCombiningClass::CCC12);
    m.insert("122", TrailCanonicalCombiningClass::CCC122);
    m.insert("129", TrailCanonicalCombiningClass::CCC129);
    m.insert("13", TrailCanonicalCombiningClass::CCC13);
    m.insert("130", TrailCanonicalCombiningClass::CCC130);
    m.insert("132", TrailCanonicalCombiningClass::CCC132);
    m.insert("133", TrailCanonicalCombiningClass::CCC133);
    m.insert("14", TrailCanonicalCombiningClass::CCC14);
    m.insert("15", TrailCanonicalCombiningClass::CCC15);
    m.insert("16", TrailCanonicalCombiningClass::CCC16);
    m.insert("17", TrailCanonicalCombiningClass::CCC17);
    m.insert("18", TrailCanonicalCombiningClass::CCC18);
    m.insert("19", TrailCanonicalCombiningClass::CCC19);
    m.insert("20", TrailCanonicalCombiningClass::CCC20);
    m.insert("200", TrailCanonicalCombiningClass::AttachedBelowLeft);
    m.insert("202", TrailCanonicalCombiningClass::AttachedBelow);
    m.insert("21", TrailCanonicalCombiningClass::CCC21);
    m.insert("214", TrailCanonicalCombiningClass::AttachedAbove);
    m.insert("216", TrailCanonicalCombiningClass::AttachedAboveRight);
    m.insert("218", TrailCanonicalCombiningClass::BelowLeft);
    m.insert("22", TrailCanonicalCombiningClass::CCC22);
    m.insert("220", TrailCanonicalCombiningClass::Below);
    m.insert("222", TrailCanonicalCombiningClass::BelowRight);
    m.insert("224", TrailCanonicalCombiningClass::Left);
    m.insert("226", TrailCanonicalCombiningClass::Right);
    m.insert("228", TrailCanonicalCombiningClass::AboveLeft);
    m.insert("23", TrailCanonicalCombiningClass::CCC23);
    m.insert("230", TrailCanonicalCombiningClass::Above);
    m.insert("232", TrailCanonicalCombiningClass::AboveRight);
    m.insert("233", TrailCanonicalCombiningClass::DoubleBelow);
    m.insert("234", TrailCanonicalCombiningClass::DoubleAbove);
    m.insert("24", TrailCanonicalCombiningClass::CCC24);
    m.insert("240", TrailCanonicalCombiningClass::IotaSubscript);
    m.insert("25", TrailCanonicalCombiningClass::CCC25);
    m.insert("26", TrailCanonicalCombiningClass::CCC26);
    m.insert("27", TrailCanonicalCombiningClass::CCC27);
    m.insert("28", TrailCanonicalCombiningClass::CCC28);
    m.insert("29", TrailCanonicalCombiningClass::CCC29);
    m.insert("30", TrailCanonicalCombiningClass::CCC30);
    m.insert("31", TrailCanonicalCombiningClass::CCC31);
    m.insert("32", TrailCanonicalCombiningClass::CCC32);
    m.insert("33", TrailCanonicalCombiningClass::CCC33);
    m.insert("34", TrailCanonicalCombiningClass::CCC34);
    m.insert("35", TrailCanonicalCombiningClass::CCC35);
    m.insert("36", TrailCanonicalCombiningClass::CCC36);
    m.insert("6", TrailCanonicalCombiningClass::HanReading);
    m.insert("7", TrailCanonicalCombiningClass::Nukta);
    m.insert("8", TrailCanonicalCombiningClass::KanaVoicing);
    m.insert("84", TrailCanonicalCombiningClass::CCC84);
    m.insert("9", TrailCanonicalCombiningClass::Virama);
    m.insert("91", TrailCanonicalCombiningClass::CCC91);
    m.get(name).cloned()
}

fn get_vertical_orientation_enum(name: &str) -> Option<VerticalOrientation> {
    let mut m: LiteMap<&str, VerticalOrientation> = LiteMap::new();
    m.insert("R", VerticalOrientation::Rotated);
    m.insert("Tr", VerticalOrientation::TransformedRotated);
    m.insert("Tu", VerticalOrientation::TransformedUpright);
    m.insert("U", VerticalOrientation::Upright);
    m.get(name).cloned()
}

fn get_word_break_enum(name: &str) -> Option<WordBreak> {
    let mut m: LiteMap<&str, WordBreak> = LiteMap::new();
    m.insert("CR", WordBreak::CR);
    m.insert("DQ", WordBreak::DoubleQuote);
    m.insert("EB", WordBreak::EBase);
    m.insert("EBG", WordBreak::EBaseGAZ);
    m.insert("EM", WordBreak::EModifier);
    m.insert("EX", WordBreak::ExtendNumLet);
    m.insert("Extend", WordBreak::Extend);
    m.insert("FO", WordBreak::Format);
    m.insert("GAZ", WordBreak::GlueAfterZwj);
    m.insert("HL", WordBreak::HebrewLetter);
    m.insert("KA", WordBreak::Katakana);
    m.insert("LE", WordBreak::ALetter);
    m.insert("LF", WordBreak::LF);
    m.insert("MB", WordBreak::MidNumLet);
    m.insert("ML", WordBreak::MidLetter);
    m.insert("MN", WordBreak::MidNum);
    m.insert("NL", WordBreak::Newline);
    m.insert("NU", WordBreak::Numeric);
    m.insert("RI", WordBreak::RegionalIndicator);
    m.insert("SQ", WordBreak::SingleQuote);
    m.insert("WSegSpace", WordBreak::WSegSpace);
    m.insert("XX", WordBreak::Other);
    m.insert("ZWJ", WordBreak::ZWJ);
    m.get(name).cloned()
}

//
// Helper fn to help generate identifer for the prop_name=prop_val `UnicodeProperty`
//

fn get_prop_name_val_as_i32(prop_name: &str, prop_val: &str) -> Option<(i32, i32)> {
    let name_enum_opt = get_enum_property_enum(prop_name);
    let val_enum_i32_opt = match name_enum_opt {
        Some(EnumeratedProperty::BidiClass) => {
            get_bidi_class_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::BidiPairedBracketType) => {
            get_bidi_paired_bracket_type_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::CanonicalCombiningClass) => {
            get_canonical_combining_class_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::DecompositionType) => {
            get_decomposition_type_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::EastAsianWidth) => {
            get_east_asian_width_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::GeneralCategory) => {
            get_general_category_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::GraphemeClusterBreak) => {
            get_grapheme_cluster_break_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::HangulSyllableType) => {
            get_hangul_syllable_type_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::IndicPositionalCategory) => {
            get_indic_positional_category_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::IndicSyllabicCategory) => {
            get_indic_syllabic_category_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::JoiningGroup) => {
            get_joining_group_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::JoiningType) => {
            get_joining_type_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::LineBreak) => {
            get_line_break_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::LeadCanonicalCombiningClass) => {
            get_lead_canonical_combining_class_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::NFCQuickCheck) => {
            get_nfc_quick_check_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::NFDQuickCheck) => {
            get_nfd_quick_check_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::NFKCQuickCheck) => {
            get_nfkc_quick_check_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::NFKDQuickCheck) => {
            get_nfkd_quick_check_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::NumericType) => {
            get_numeric_type_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::SentenceBreak) => {
            get_sentence_break_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::TrailCanonicalCombiningClass) => {
            get_trail_canonical_combining_class_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::VerticalOrientation) => {
            get_vertical_orientation_enum(prop_val).map(|x| x as i32)
        },
        Some(EnumeratedProperty::WordBreak) => {
            get_word_break_enum(prop_val).map(|x| x as i32)
        },
        _ => None,
    };
    let name_enum_i32_opt = name_enum_opt.map(|x| x as i32);
    match (name_enum_i32_opt, val_enum_i32_opt) {
        (Some(name_i32), Some(val_i32)) => Some((name_i32, val_i32)),
        _ => None
    }
}

pub fn get_prop_name_identifier(prop_name: &str, prop_val: &str) -> Option<String> {
    let name_val_i32_opt = get_prop_name_val_as_i32(prop_name, prop_val);
    match name_val_i32_opt {
        Some((name_i32, val_i32)) => Some(format!("{}={}", name_i32, val_i32)),
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
        assert_eq!(get_canonical_combining_class_enum("21"), Some(CanonicalCombiningClass::CCC21));
        assert_eq!(get_canonical_combining_class_enum("cheezburger"), None);
    }

    #[test]
    fn get_prop_name_val_as_i32_test() {
        let act_prop_i32_tuple_opt_1 = get_prop_name_val_as_i32("lb", "LF");
        let exp_prop_i32_tuple_opt_1 = Some(
            (EnumeratedProperty::LineBreak as i32,
            LineBreak::LineFeed as i32)
        );
        assert_eq!(act_prop_i32_tuple_opt_1, exp_prop_i32_tuple_opt_1);
        
        assert_eq!(get_prop_name_val_as_i32("lb", "cheezburger"), None);
        assert_eq!(get_prop_name_val_as_i32("cheezburger", "LF"), None);
        assert_eq!(get_prop_name_val_as_i32("cheez", "cheez"), None);
    }

    #[test]
    fn get_prop_name_identifier_test() {
        assert_eq!(get_prop_name_identifier("lb", "LF"), Some("12=26".to_string()));
        assert_eq!(get_prop_name_identifier("ccc", "230"), Some("2=230".to_string()));
    }

}
