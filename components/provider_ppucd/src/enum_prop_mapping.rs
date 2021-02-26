// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_uniset::enum_props::*;
use litemap::LiteMap;


fn get_enum_property_enum(name: &str) -> Option<EnumeratedProperty> {
    let mut m: LiteMap<&str, EnumeratedProperty> = LiteMap::new();
    m.insert("Bidi_Class", EnumeratedProperty::BidiClass);
    m.insert("Bidi_Paired_Bracket_Type", EnumeratedProperty::BidiPairedBracketType);
    m.insert("Canonical_Combining_Class", EnumeratedProperty::CanonicalCombiningClass);
    m.insert("Decomposition_Type", EnumeratedProperty::DecompositionType);
    m.insert("East_Asian_Width", EnumeratedProperty::EastAsianWidth);
    m.insert("General_Category", EnumeratedProperty::GeneralCategory);
    m.insert("Grapheme_Cluster_Break", EnumeratedProperty::GraphemeClusterBreak);
    m.insert("Hangul_Syllable_Type", EnumeratedProperty::HangulSyllableType);
    m.insert("Indic_Positional_Category", EnumeratedProperty::IndicPositionalCategory);
    m.insert("Indic_Syllabic_Category", EnumeratedProperty::IndicSyllabicCategory);
    m.insert("Joining_Group", EnumeratedProperty::JoiningGroup);
    m.insert("Joining_Type", EnumeratedProperty::JoiningType);
    m.insert("Line_Break", EnumeratedProperty::LineBreak);
    m.insert("Lead_Canonical_Combining_Class", EnumeratedProperty::LeadCanonicalCombiningClass);
    m.insert("NFC_Quick_Check", EnumeratedProperty::NFCQuickCheck);
    m.insert("NFD_Quick_Check", EnumeratedProperty::NFDQuickCheck);
    m.insert("NFKC_Quick_Check", EnumeratedProperty::NFKCQuickCheck);
    m.insert("NFKD_Quick_Check", EnumeratedProperty::NFKDQuickCheck);
    m.insert("Numeric_Type", EnumeratedProperty::NumericType);
    m.insert("Sentence_Break", EnumeratedProperty::SentenceBreak);
    m.insert("Trail_Canonical_Combining_Class", EnumeratedProperty::TrailCanonicalCombiningClass);
    m.insert("Vertical_Orientation", EnumeratedProperty::VerticalOrientation);
    m.insert("Word_Break", EnumeratedProperty::WordBreak);
    m.get(name).cloned()
}

fn get_bidi_class_enum(name: &str) -> Option<BidiClass> {
    let mut m: LiteMap<&str, BidiClass> = LiteMap::new();
    m.insert("Arabic_Letter", BidiClass::ArabicLetter);
    m.insert("Arabic_Number", BidiClass::ArabicNumber);
    m.insert("Paragraph_Separator", BidiClass::ParagraphSeparator);
    m.insert("Boundary_Neutral", BidiClass::BoundaryNeutral);
    m.insert("Common_Separator", BidiClass::CommonSeparator);
    m.insert("European_Number", BidiClass::EuropeanNumber);
    m.insert("European_Separator", BidiClass::EuropeanSeparator);
    m.insert("European_Terminator", BidiClass::EuropeanTerminator);
    m.insert("First_Strong_Isolate", BidiClass::FirstStrongIsolate);
    m.insert("Left_To_Right", BidiClass::LeftToRight);
    m.insert("Left_To_Right_Embedding", BidiClass::LeftToRightEmbedding);
    m.insert("Left_To_Right_Isolate", BidiClass::LeftToRightIsolate);
    m.insert("Left_To_Right_Override", BidiClass::LeftToRightOverride);
    m.insert("Nonspacing_Mark", BidiClass::NonspacingMark);
    m.insert("Other_Neutral", BidiClass::OtherNeutral);
    m.insert("Pop_Directional_Format", BidiClass::PopDirectionalFormat);
    m.insert("Pop_Directional_Isolate", BidiClass::PopDirectionalIsolate);
    m.insert("Right_To_Left", BidiClass::RightToLeft);
    m.insert("Right_To_Left_Embedding", BidiClass::RightToLeftEmbedding);
    m.insert("Right_To_Left_Isolate", BidiClass::RightToLeftIsolate);
    m.insert("Right_To_Left_Override", BidiClass::RightToLeftOverride);
    m.insert("Segment_Separator", BidiClass::SegmentSeparator);
    m.insert("White_Space", BidiClass::WhiteSpace,   );
    m.get(name).cloned()
}

fn get_bidi_paired_bracket_type_enum(name: &str) -> Option<BidiPairedBracketType> {
    let mut m: LiteMap<&str, BidiPairedBracketType> = LiteMap::new();
    m.insert("Clone", BidiPairedBracketType::Close);
    m.insert("None", BidiPairedBracketType::None);
    m.insert("Open", BidiPairedBracketType::Open);
    m.get(name).cloned()
}

fn get_canonical_combining_class_enum(name: &str) -> Option<CanonicalCombiningClass> {
    let mut m: LiteMap<&str, CanonicalCombiningClass> = LiteMap::new();
    m.insert("Not_Reordered", CanonicalCombiningClass::NotReordered);
    m.insert("Overlay", CanonicalCombiningClass::Overlay);
    m.insert("CCC10", CanonicalCombiningClass::CCC10);
    m.insert("CCC103", CanonicalCombiningClass::CCC103);
    m.insert("CCC107", CanonicalCombiningClass::CCC107);
    m.insert("CCC11", CanonicalCombiningClass::CCC11);
    m.insert("CCC118", CanonicalCombiningClass::CCC118);
    m.insert("CCC12", CanonicalCombiningClass::CCC12);
    m.insert("CCC122", CanonicalCombiningClass::CCC122);
    m.insert("CCC129", CanonicalCombiningClass::CCC129);
    m.insert("CCC13", CanonicalCombiningClass::CCC13);
    m.insert("CCC130", CanonicalCombiningClass::CCC130);
    m.insert("CCC132", CanonicalCombiningClass::CCC132);
    m.insert("CCC133", CanonicalCombiningClass::CCC133);
    m.insert("CCC14", CanonicalCombiningClass::CCC14);
    m.insert("CCC15", CanonicalCombiningClass::CCC15);
    m.insert("CCC16", CanonicalCombiningClass::CCC16);
    m.insert("CCC17", CanonicalCombiningClass::CCC17);
    m.insert("CCC18", CanonicalCombiningClass::CCC18);
    m.insert("CCC19", CanonicalCombiningClass::CCC19);
    m.insert("CCC20", CanonicalCombiningClass::CCC20);
    m.insert("Attached_Below_Left", CanonicalCombiningClass::AttachedBelowLeft);
    m.insert("Attached_Below", CanonicalCombiningClass::AttachedBelow);
    m.insert("CCC21", CanonicalCombiningClass::CCC21);
    m.insert("Attached_Above", CanonicalCombiningClass::AttachedAbove);
    m.insert("Attached_Above_Right", CanonicalCombiningClass::AttachedAboveRight);
    m.insert("Below_Left", CanonicalCombiningClass::BelowLeft);
    m.insert("CCC22", CanonicalCombiningClass::CCC22);
    m.insert("Below", CanonicalCombiningClass::Below);
    m.insert("Below_Right", CanonicalCombiningClass::BelowRight);
    m.insert("Left", CanonicalCombiningClass::Left);
    m.insert("Right", CanonicalCombiningClass::Right);
    m.insert("Above_Left", CanonicalCombiningClass::AboveLeft);
    m.insert("CCC23", CanonicalCombiningClass::CCC23);
    m.insert("Above", CanonicalCombiningClass::Above);
    m.insert("Above_Right", CanonicalCombiningClass::AboveRight);
    m.insert("Double_Below", CanonicalCombiningClass::DoubleBelow);
    m.insert("Double_Above", CanonicalCombiningClass::DoubleAbove);
    m.insert("CCC24", CanonicalCombiningClass::CCC24);
    m.insert("Iota_Subscript", CanonicalCombiningClass::IotaSubscript);
    m.insert("CCC25", CanonicalCombiningClass::CCC25);
    m.insert("CCC26", CanonicalCombiningClass::CCC26);
    m.insert("CCC27", CanonicalCombiningClass::CCC27);
    m.insert("CCC28", CanonicalCombiningClass::CCC28);
    m.insert("CCC29", CanonicalCombiningClass::CCC29);
    m.insert("CCC30", CanonicalCombiningClass::CCC30);
    m.insert("CCC31", CanonicalCombiningClass::CCC31);
    m.insert("CCC32", CanonicalCombiningClass::CCC32);
    m.insert("CCC33", CanonicalCombiningClass::CCC33);
    m.insert("CCC34", CanonicalCombiningClass::CCC34);
    m.insert("CCC35", CanonicalCombiningClass::CCC35);
    m.insert("CCC36", CanonicalCombiningClass::CCC36);
    m.insert("Han_Reading", CanonicalCombiningClass::HanReading);
    m.insert("Nukta", CanonicalCombiningClass::Nukta);
    m.insert("Kana_Voicing", CanonicalCombiningClass::KanaVoicing);
    m.insert("CCC84", CanonicalCombiningClass::CCC84);
    m.insert("Virama", CanonicalCombiningClass::Virama);
    m.insert("CCC91", CanonicalCombiningClass::CCC91);
    m.get(name).cloned()
}

fn get_decomposition_type_enum(name: &str) -> Option<DecompositionType> {
    let mut m: LiteMap<&str, DecompositionType> = LiteMap::new();
    m.insert("can", DecompositionType::Can);
    m.insert("com", DecompositionType::Com);
    m.insert("enc", DecompositionType::Enc);
    m.insert("fin", DecompositionType::Fin);
    m.insert("font", DecompositionType::Font);
    m.insert("fra", DecompositionType::Fra);
    m.insert("init", DecompositionType::Init);
    m.insert("iso", DecompositionType::Iso);
    m.insert("med", DecompositionType::Med);
    m.insert("nar", DecompositionType::Nar);
    m.insert("nb", DecompositionType::Nb);
    m.insert("none", DecompositionType::None);
    m.insert("sml", DecompositionType::Sml);
    m.insert("sqr", DecompositionType::Sqr);
    m.insert("sub", DecompositionType::Sub);
    m.insert("sup", DecompositionType::Sup);
    m.insert("vert", DecompositionType::Vert);
    m.insert("wide", DecompositionType::Wide);
    m.get(name).cloned()
}

fn get_east_asian_width_enum(name: &str) -> Option<EastAsianWidth> {
    let mut m: LiteMap<&str, EastAsianWidth> = LiteMap::new();
    m.insert("Ambiguous", EastAsianWidth::Ambiguous);
    m.insert("Fullwidth", EastAsianWidth::Fullwidth);
    m.insert("Halfwidth", EastAsianWidth::Halfwidth);
    m.insert("Neutral", EastAsianWidth::Neutral);
    m.insert("Narrow", EastAsianWidth::Narrow);
    m.insert("Wide", EastAsianWidth::Wide);
    m.get(name).cloned()
}

fn get_general_category_enum(name: &str) -> Option<GeneralCategory> {
    let mut m: LiteMap<&str, GeneralCategory> = LiteMap::new();
    m.insert("Other", GeneralCategory::Other);
    m.insert("cntrl", GeneralCategory::Cntrl);
    m.insert("Format", GeneralCategory::Format);
    m.insert("Unassigned", GeneralCategory::Unassigned);
    m.insert("Private_Use", GeneralCategory::PrivateUse);
    m.insert("Surrogate", GeneralCategory::Surrogate);
    m.insert("Letter", GeneralCategory::Letter);
    m.insert("Cased_Letter", GeneralCategory::CasedLetter);
    m.insert("Lowercase_Letter", GeneralCategory::LowercaseLetter);
    m.insert("Modifier_Letter", GeneralCategory::ModifierLetter);
    m.insert("Other_Letter", GeneralCategory::OtherLetter);
    m.insert("Titlecase_Letter", GeneralCategory::TitlecaseLetter);
    m.insert("Uppercase_Letter", GeneralCategory::UppercaseLetter);
    m.insert("Combining_Mark", GeneralCategory::CombiningMark);
    m.insert("Spacing_Mark", GeneralCategory::SpacingMark);
    m.insert("Enclosing_Mark", GeneralCategory::EnclosingMark);
    m.insert("Nonspacing_Mark", GeneralCategory::NonspacingMark);
    m.insert("Number", GeneralCategory::Number);
    m.insert("digit", GeneralCategory::Digit);
    m.insert("Letter_Number", GeneralCategory::LetterNumber);
    m.insert("Other_Number", GeneralCategory::OtherNumber);
    m.insert("punct", GeneralCategory::Punct);
    m.insert("Connector_Punctuation", GeneralCategory::ConnectorPunctuation);
    m.insert("Dash_Punctuation", GeneralCategory::DashPunctuation);
    m.insert("Close_Punctuation", GeneralCategory::ClosePunctuation);
    m.insert("Final_Punctuation", GeneralCategory::FinalPunctuation);
    m.insert("Initial_Punctuation", GeneralCategory::InitialPunctuation);
    m.insert("Other_Punctuation", GeneralCategory::OtherPunctuation);
    m.insert("Open_Punctuation", GeneralCategory::OpenPunctuation);
    m.insert("Symbol", GeneralCategory::Symbol);
    m.insert("Currency_Symbol", GeneralCategory::CurrencySymbol);
    m.insert("Modifier_Symbol", GeneralCategory::ModifierSymbol);
    m.insert("Math_Symbol", GeneralCategory::MathSymbol);
    m.insert("Other_Symbol", GeneralCategory::OtherSymbol);
    m.insert("Separator", GeneralCategory::Separator);
    m.insert("Line_Separator", GeneralCategory::LineSeparator);
    m.insert("Paragraph_Separator", GeneralCategory::ParagraphSeparator);
    m.insert("Space_Separator", GeneralCategory::SpaceSeparator);
    m.get(name).cloned()
}

fn get_grapheme_cluster_break_enum(name: &str) -> Option<GraphemeClusterBreak> {
    let mut m: LiteMap<&str, GraphemeClusterBreak> = LiteMap::new();
    m.insert("Control", GraphemeClusterBreak::Control);
    m.insert("CR", GraphemeClusterBreak::CR);
    m.insert("E_Base", GraphemeClusterBreak::EBase);
    m.insert("E_Base_GAZ", GraphemeClusterBreak::EBaseGAZ);
    m.insert("E_Modifier", GraphemeClusterBreak::EModifier);
    m.insert("Extend", GraphemeClusterBreak::Extend);
    m.insert("Glue_After_Zwj", GraphemeClusterBreak::GlueAfterZwj);
    m.insert("L", GraphemeClusterBreak::L);
    m.insert("LF", GraphemeClusterBreak::LF);
    m.insert("LV", GraphemeClusterBreak::LV);
    m.insert("LVT", GraphemeClusterBreak::LVT);
    m.insert("Prepend", GraphemeClusterBreak::Prepend);
    m.insert("Regional_Indicator", GraphemeClusterBreak::RegionalIndicator);
    m.insert("SpacingMark", GraphemeClusterBreak::SpacingMark);
    m.insert("T", GraphemeClusterBreak::T);
    m.insert("V", GraphemeClusterBreak::V);
    m.insert("Other", GraphemeClusterBreak::Other);
    m.insert("ZWJ", GraphemeClusterBreak::ZWJ);
    m.get(name).cloned()
}

fn get_hangul_syllable_type_enum(name: &str) -> Option<HangulSyllableType> {
    let mut m: LiteMap<&str, HangulSyllableType> = LiteMap::new();
    m.insert("Leading_Jamo", HangulSyllableType::LeadingJamo);
    m.insert("LV_Syllable", HangulSyllableType::LVSyllable);
    m.insert("LVT_Syllable", HangulSyllableType::LVTSyllable);
    m.insert("Not_Applicable", HangulSyllableType::NotApplicable);
    m.insert("Trailing_Jamo", HangulSyllableType::TrailingJamo);
    m.insert("Vowel_Jamo", HangulSyllableType::VowelJamo);
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
    m.insert("Hamza_On_Heh_Goal", JoiningGroup::HamzaOnHehGoal);
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
    m.insert("Join_Causing", JoiningType::JoinCausing);
    m.insert("Dual_Joining", JoiningType::DualJoining);
    m.insert("Left_Joining", JoiningType::LeftJoining);
    m.insert("Right_Joining", JoiningType::RightJoining);
    m.insert("Transparent", JoiningType::Transparent);
    m.insert("Non_Joining", JoiningType::NonJoining,   );
    m.get(name).cloned()
}

fn get_line_break_enum(name: &str) -> Option<LineBreak> {
    let mut m: LiteMap<&str, LineBreak> = LiteMap::new();
    m.insert("Ambiguous", LineBreak::Ambiguous);
    m.insert("Alphabetic", LineBreak::Alphabetic);
    m.insert("Break_Both", LineBreak::BreakBoth);
    m.insert("Break_After", LineBreak::BreakAfter);
    m.insert("Break_Before", LineBreak::BreakBefore);
    m.insert("Mandatory_Break", LineBreak::MandatoryBreak);
    m.insert("Contingent_Break", LineBreak::ContingentBreak);
    m.insert("Conditional_Japanese_Starter", LineBreak::ConditionalJapaneseStarter);
    m.insert("Close_Punctuation", LineBreak::ClosePunctuation);
    m.insert("Combining_Mark", LineBreak::CombiningMark);
    m.insert("Close_Parenthesis", LineBreak::CloseParenthesis);
    m.insert("Carriage_Return", LineBreak::CarriageReturn);
    m.insert("E_Base", LineBreak::EBase);
    m.insert("E_Modifier", LineBreak::EModifier);
    m.insert("Exclamation", LineBreak::Exclamation);
    m.insert("Glue", LineBreak::Glue);
    m.insert("H2", LineBreak::H2);
    m.insert("H3", LineBreak::H3);
    m.insert("Hebrew_Letter", LineBreak::HebrewLetter);
    m.insert("Hyphen", LineBreak::Hyphen);
    m.insert("Ideographic", LineBreak::Ideographic);
    m.insert("Inseperable", LineBreak::Inseperable);
    m.insert("Infix_Numeric", LineBreak::InfixNumeric);
    m.insert("JL", LineBreak::JL);
    m.insert("JT", LineBreak::JT);
    m.insert("JV", LineBreak::JV);
    m.insert("Line_Feed", LineBreak::LineFeed);
    m.insert("Next_Line", LineBreak::NextLine);
    m.insert("Nonstarter", LineBreak::Nonstarter);
    m.insert("Numeric", LineBreak::Numeric);
    m.insert("Open_Punctuation", LineBreak::OpenPunctuation);
    m.insert("Postfix_Numeric", LineBreak::PostfixNumeric);
    m.insert("Prefix_Numeric", LineBreak::PrefixNumeric);
    m.insert("Quotation", LineBreak::Quotation);
    m.insert("Regional_Indicator", LineBreak::RegionalIndicator);
    m.insert("Complex_Context", LineBreak::ComplexContext);
    m.insert("Surrogate", LineBreak::Surrogate);
    m.insert("Space", LineBreak::Space);
    m.insert("Break_Symbols", LineBreak::BreakSymbols);
    m.insert("Word_Joiner", LineBreak::WordJoiner);
    m.insert("Unknown", LineBreak::Unknown);
    m.insert("ZWSpace", LineBreak::ZWSpace);
    m.insert("ZWJ", LineBreak::ZWJ);
    m.get(name).cloned()
}

fn get_lead_canonical_combining_class_enum(name: &str) -> Option<LeadCanonicalCombiningClass> {
    let mut m: LiteMap<&str, LeadCanonicalCombiningClass> = LiteMap::new();
    m.insert("Not_Reordered", LeadCanonicalCombiningClass::NotReordered);
    m.insert("Overlay", LeadCanonicalCombiningClass::Overlay);
    m.insert("CCC10", LeadCanonicalCombiningClass::CCC10);
    m.insert("CCC103", LeadCanonicalCombiningClass::CCC103);
    m.insert("CCC107", LeadCanonicalCombiningClass::CCC107);
    m.insert("CCC11", LeadCanonicalCombiningClass::CCC11);
    m.insert("CCC118", LeadCanonicalCombiningClass::CCC118);
    m.insert("CCC12", LeadCanonicalCombiningClass::CCC12);
    m.insert("CCC122", LeadCanonicalCombiningClass::CCC122);
    m.insert("CCC129", LeadCanonicalCombiningClass::CCC129);
    m.insert("CCC13", LeadCanonicalCombiningClass::CCC13);
    m.insert("CCC130", LeadCanonicalCombiningClass::CCC130);
    m.insert("CCC132", LeadCanonicalCombiningClass::CCC132);
    m.insert("CCC133", LeadCanonicalCombiningClass::CCC133);
    m.insert("CCC14", LeadCanonicalCombiningClass::CCC14);
    m.insert("CCC15", LeadCanonicalCombiningClass::CCC15);
    m.insert("CCC16", LeadCanonicalCombiningClass::CCC16);
    m.insert("CCC17", LeadCanonicalCombiningClass::CCC17);
    m.insert("CCC18", LeadCanonicalCombiningClass::CCC18);
    m.insert("CCC19", LeadCanonicalCombiningClass::CCC19);
    m.insert("CCC20", LeadCanonicalCombiningClass::CCC20);
    m.insert("Attached_Below_Left", LeadCanonicalCombiningClass::AttachedBelowLeft);
    m.insert("Attached_Below", LeadCanonicalCombiningClass::AttachedBelow);
    m.insert("CCC21", LeadCanonicalCombiningClass::CCC21);
    m.insert("Attached_Above", LeadCanonicalCombiningClass::AttachedAbove);
    m.insert("Attached_Above_Right", LeadCanonicalCombiningClass::AttachedAboveRight);
    m.insert("Below_Left", LeadCanonicalCombiningClass::BelowLeft);
    m.insert("CCC22", LeadCanonicalCombiningClass::CCC22);
    m.insert("Below", LeadCanonicalCombiningClass::Below);
    m.insert("Below_Right", LeadCanonicalCombiningClass::BelowRight);
    m.insert("Left", LeadCanonicalCombiningClass::Left);
    m.insert("Right", LeadCanonicalCombiningClass::Right);
    m.insert("Above_Left", LeadCanonicalCombiningClass::AboveLeft);
    m.insert("CCC23", LeadCanonicalCombiningClass::CCC23);
    m.insert("Above", LeadCanonicalCombiningClass::Above);
    m.insert("Above_Right", LeadCanonicalCombiningClass::AboveRight);
    m.insert("Double_Below", LeadCanonicalCombiningClass::DoubleBelow);
    m.insert("Double_Above", LeadCanonicalCombiningClass::DoubleAbove);
    m.insert("CCC24", LeadCanonicalCombiningClass::CCC24);
    m.insert("Iota_Subscript", LeadCanonicalCombiningClass::IotaSubscript);
    m.insert("CCC25", LeadCanonicalCombiningClass::CCC25);
    m.insert("CCC26", LeadCanonicalCombiningClass::CCC26);
    m.insert("CCC27", LeadCanonicalCombiningClass::CCC27);
    m.insert("CCC28", LeadCanonicalCombiningClass::CCC28);
    m.insert("CCC29", LeadCanonicalCombiningClass::CCC29);
    m.insert("CCC30", LeadCanonicalCombiningClass::CCC30);
    m.insert("CCC31", LeadCanonicalCombiningClass::CCC31);
    m.insert("CCC32", LeadCanonicalCombiningClass::CCC32);
    m.insert("CCC33", LeadCanonicalCombiningClass::CCC33);
    m.insert("CCC34", LeadCanonicalCombiningClass::CCC34);
    m.insert("CCC35", LeadCanonicalCombiningClass::CCC35);
    m.insert("CCC36", LeadCanonicalCombiningClass::CCC36);
    m.insert("Han_Reading", LeadCanonicalCombiningClass::HanReading);
    m.insert("Nukta", LeadCanonicalCombiningClass::Nukta);
    m.insert("Kana_Voicing", LeadCanonicalCombiningClass::KanaVoicing);
    m.insert("CCC84", LeadCanonicalCombiningClass::CCC84);
    m.insert("Virama", LeadCanonicalCombiningClass::Virama);
    m.insert("CCC91", LeadCanonicalCombiningClass::CCC91);
    m.get(name).cloned()
}

fn get_nfc_quick_check_enum(name: &str) -> Option<NFCQuickCheck> {
    let mut m: LiteMap<&str, NFCQuickCheck> = LiteMap::new();
    m.insert("Maybe", NFCQuickCheck::Maybe);
    m.insert("No", NFCQuickCheck::No);
    m.insert("Yes", NFCQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_nfd_quick_check_enum(name: &str) -> Option<NFDQuickCheck> {
    let mut m: LiteMap<&str, NFDQuickCheck> = LiteMap::new();
    m.insert("No", NFDQuickCheck::No);
    m.insert("Yes", NFDQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_nfkc_quick_check_enum(name: &str) -> Option<NFKCQuickCheck> {
    let mut m: LiteMap<&str, NFKCQuickCheck> = LiteMap::new();
    m.insert("Maybe", NFKCQuickCheck::Maybe);
    m.insert("No", NFKCQuickCheck::No);
    m.insert("Yes", NFKCQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_nfkd_quick_check_enum(name: &str) -> Option<NFKDQuickCheck> {
    let mut m: LiteMap<&str, NFKDQuickCheck> = LiteMap::new();
    m.insert("No", NFKDQuickCheck::No);
    m.insert("Yes", NFKDQuickCheck::Yes);
    m.get(name).cloned()
}

fn get_numeric_type_enum(name: &str) -> Option<NumericType> {
    let mut m: LiteMap<&str, NumericType> = LiteMap::new();
    m.insert("Decimal", NumericType::Decimal);
    m.insert("Digit", NumericType::Digit);
    m.insert("None", NumericType::None);
    m.insert("Numeric", NumericType::Numeric);
    m.get(name).cloned()
}

fn get_sentence_break_enum(name: &str) -> Option<SentenceBreak> {
    let mut m: LiteMap<&str, SentenceBreak> = LiteMap::new();
    m.insert("ATerm", SentenceBreak::ATerm);
    m.insert("Close", SentenceBreak::Close);
    m.insert("CR", SentenceBreak::CR);
    m.insert("Extend", SentenceBreak::Extend);
    m.insert("Format", SentenceBreak::Format);
    m.insert("OLetter", SentenceBreak::OLetter);
    m.insert("LF", SentenceBreak::LF);
    m.insert("Lower", SentenceBreak::Lower);
    m.insert("Numeric", SentenceBreak::Numeric);
    m.insert("SContinue", SentenceBreak::SContinue);
    m.insert("Sep", SentenceBreak::Sep);
    m.insert("Sp", SentenceBreak::Sp);
    m.insert("STerm", SentenceBreak::STerm);
    m.insert("Upper", SentenceBreak::Upper);
    m.insert("Other", SentenceBreak::Other);
    m.get(name).cloned()    
}

fn get_trail_canonical_combining_class_enum(name: &str) -> Option<TrailCanonicalCombiningClass> {
    let mut m: LiteMap<&str, TrailCanonicalCombiningClass> = LiteMap::new();
    m.insert("Not_Reordered", TrailCanonicalCombiningClass::NotReordered);
    m.insert("Overlay", TrailCanonicalCombiningClass::Overlay);
    m.insert("CCC10", TrailCanonicalCombiningClass::CCC10);
    m.insert("CCC103", TrailCanonicalCombiningClass::CCC103);
    m.insert("CCC107", TrailCanonicalCombiningClass::CCC107);
    m.insert("CCC11", TrailCanonicalCombiningClass::CCC11);
    m.insert("CCC118", TrailCanonicalCombiningClass::CCC118);
    m.insert("CCC12", TrailCanonicalCombiningClass::CCC12);
    m.insert("CCC122", TrailCanonicalCombiningClass::CCC122);
    m.insert("CCC129", TrailCanonicalCombiningClass::CCC129);
    m.insert("CCC13", TrailCanonicalCombiningClass::CCC13);
    m.insert("CCC130", TrailCanonicalCombiningClass::CCC130);
    m.insert("CCC132", TrailCanonicalCombiningClass::CCC132);
    m.insert("CCC133", TrailCanonicalCombiningClass::CCC133);
    m.insert("CCC14", TrailCanonicalCombiningClass::CCC14);
    m.insert("CCC15", TrailCanonicalCombiningClass::CCC15);
    m.insert("CCC16", TrailCanonicalCombiningClass::CCC16);
    m.insert("CCC17", TrailCanonicalCombiningClass::CCC17);
    m.insert("CCC18", TrailCanonicalCombiningClass::CCC18);
    m.insert("CCC19", TrailCanonicalCombiningClass::CCC19);
    m.insert("CCC20", TrailCanonicalCombiningClass::CCC20);
    m.insert("Attached_Below_Left", TrailCanonicalCombiningClass::AttachedBelowLeft);
    m.insert("Attached_Below", TrailCanonicalCombiningClass::AttachedBelow);
    m.insert("CCC21", TrailCanonicalCombiningClass::CCC21);
    m.insert("Attached_Above", TrailCanonicalCombiningClass::AttachedAbove);
    m.insert("Attached_Above_Right", TrailCanonicalCombiningClass::AttachedAboveRight);
    m.insert("Below_Left", TrailCanonicalCombiningClass::BelowLeft);
    m.insert("CCC22", TrailCanonicalCombiningClass::CCC22);
    m.insert("Below", TrailCanonicalCombiningClass::Below);
    m.insert("Below_Right", TrailCanonicalCombiningClass::BelowRight);
    m.insert("Left", TrailCanonicalCombiningClass::Left);
    m.insert("Right", TrailCanonicalCombiningClass::Right);
    m.insert("Above_Left", TrailCanonicalCombiningClass::AboveLeft);
    m.insert("CCC23", TrailCanonicalCombiningClass::CCC23);
    m.insert("Above", TrailCanonicalCombiningClass::Above);
    m.insert("Above_Right", TrailCanonicalCombiningClass::AboveRight);
    m.insert("Double_Below", TrailCanonicalCombiningClass::DoubleBelow);
    m.insert("Double_Above", TrailCanonicalCombiningClass::DoubleAbove);
    m.insert("CCC24", TrailCanonicalCombiningClass::CCC24);
    m.insert("Iota_Subscript", TrailCanonicalCombiningClass::IotaSubscript);
    m.insert("CCC25", TrailCanonicalCombiningClass::CCC25);
    m.insert("CCC26", TrailCanonicalCombiningClass::CCC26);
    m.insert("CCC27", TrailCanonicalCombiningClass::CCC27);
    m.insert("CCC28", TrailCanonicalCombiningClass::CCC28);
    m.insert("CCC29", TrailCanonicalCombiningClass::CCC29);
    m.insert("CCC30", TrailCanonicalCombiningClass::CCC30);
    m.insert("CCC31", TrailCanonicalCombiningClass::CCC31);
    m.insert("CCC32", TrailCanonicalCombiningClass::CCC32);
    m.insert("CCC33", TrailCanonicalCombiningClass::CCC33);
    m.insert("CCC34", TrailCanonicalCombiningClass::CCC34);
    m.insert("CCC35", TrailCanonicalCombiningClass::CCC35);
    m.insert("CCC36", TrailCanonicalCombiningClass::CCC36);
    m.insert("Han_Reading", TrailCanonicalCombiningClass::HanReading);
    m.insert("Nukta", TrailCanonicalCombiningClass::Nukta);
    m.insert("Kana_Voicing", TrailCanonicalCombiningClass::KanaVoicing);
    m.insert("CCC84", TrailCanonicalCombiningClass::CCC84);
    m.insert("Virama", TrailCanonicalCombiningClass::Virama);
    m.insert("CCC91", TrailCanonicalCombiningClass::CCC91);
    m.get(name).cloned()
}

fn get_vertical_orientation_enum(name: &str) -> Option<VerticalOrientation> {
    let mut m: LiteMap<&str, VerticalOrientation> = LiteMap::new();
    m.insert("Rotated", VerticalOrientation::Rotated);
    m.insert("Transformed_Rotated", VerticalOrientation::TransformedRotated);
    m.insert("Transformed_Upright", VerticalOrientation::TransformedUpright);
    m.insert("Upright", VerticalOrientation::Upright);
    m.get(name).cloned()
}

fn get_word_break_enum(name: &str) -> Option<WordBreak> {
    let mut m: LiteMap<&str, WordBreak> = LiteMap::new();
    m.insert("CR", WordBreak::CR);
    m.insert("Double_Quote", WordBreak::DoubleQuote);
    m.insert("E_Base", WordBreak::EBase);
    m.insert("E_Base_GAZ", WordBreak::EBaseGAZ);
    m.insert("E_Modifier", WordBreak::EModifier);
    m.insert("ExtendNumLet", WordBreak::ExtendNumLet);
    m.insert("Extend", WordBreak::Extend);
    m.insert("Format", WordBreak::Format);
    m.insert("Glue_After_Zwj", WordBreak::GlueAfterZwj);
    m.insert("Hebrew_Letter", WordBreak::HebrewLetter);
    m.insert("Katakana", WordBreak::Katakana);
    m.insert("ALetter", WordBreak::ALetter);
    m.insert("LF", WordBreak::LF);
    m.insert("MidNumLet", WordBreak::MidNumLet);
    m.insert("MidLetter", WordBreak::MidLetter);
    m.insert("MidNum", WordBreak::MidNum);
    m.insert("Newline", WordBreak::Newline);
    m.insert("Numeric", WordBreak::Numeric);
    m.insert("Regional_Indicator", WordBreak::RegionalIndicator);
    m.insert("Single_Quote", WordBreak::SingleQuote);
    m.insert("WSegSpace", WordBreak::WSegSpace);
    m.insert("Other", WordBreak::Other);
    m.insert("ZWJ", WordBreak::ZWJ);
    m.get(name).cloned()
}


#[cfg(test)]
mod enum_tests {
    use super::*;

    #[test]
    fn str_to_enum_fn_test() {
        assert_eq!(get_line_break_enum("Line_Feed"), Some(LineBreak::LineFeed));
        assert_eq!(get_line_break_enum("cheezburger"), None);

        assert_eq!(get_canonical_combining_class_enum("CCC21"), Some(CanonicalCombiningClass::CCC21));
        assert_eq!(get_canonical_combining_class_enum("cheezburger"), None);
    }
}
