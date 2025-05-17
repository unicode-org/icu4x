// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use icu_properties::props;

    #[cfg(feature = "compiled_data")]
    use diplomat_runtime::DiplomatChar;

    #[diplomat::rust_link(icu::properties::props::BidiClass, Struct)]
    #[diplomat::enum_convert(icu_properties::props::BidiClass, needs_wildcard)]
    pub enum BidiClass {
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::LeftToRight,
            AssociatedConstantInStruct
        )]
        LeftToRight = 0,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::RightToLeft,
            AssociatedConstantInStruct
        )]
        RightToLeft = 1,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::EuropeanNumber,
            AssociatedConstantInStruct
        )]
        EuropeanNumber = 2,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::EuropeanSeparator,
            AssociatedConstantInStruct
        )]
        EuropeanSeparator = 3,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::EuropeanTerminator,
            AssociatedConstantInStruct
        )]
        EuropeanTerminator = 4,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::ArabicNumber,
            AssociatedConstantInStruct
        )]
        ArabicNumber = 5,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::CommonSeparator,
            AssociatedConstantInStruct
        )]
        CommonSeparator = 6,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::ParagraphSeparator,
            AssociatedConstantInStruct
        )]
        ParagraphSeparator = 7,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::SegmentSeparator,
            AssociatedConstantInStruct
        )]
        SegmentSeparator = 8,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::WhiteSpace,
            AssociatedConstantInStruct
        )]
        WhiteSpace = 9,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::OtherNeutral,
            AssociatedConstantInStruct
        )]
        OtherNeutral = 10,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::LeftToRightEmbedding,
            AssociatedConstantInStruct
        )]
        LeftToRightEmbedding = 11,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::LeftToRightOverride,
            AssociatedConstantInStruct
        )]
        LeftToRightOverride = 12,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::ArabicLetter,
            AssociatedConstantInStruct
        )]
        ArabicLetter = 13,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::RightToLeftEmbedding,
            AssociatedConstantInStruct
        )]
        RightToLeftEmbedding = 14,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::RightToLeftOverride,
            AssociatedConstantInStruct
        )]
        RightToLeftOverride = 15,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::PopDirectionalFormat,
            AssociatedConstantInStruct
        )]
        PopDirectionalFormat = 16,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::NonspacingMark,
            AssociatedConstantInStruct
        )]
        NonspacingMark = 17,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::BoundaryNeutral,
            AssociatedConstantInStruct
        )]
        BoundaryNeutral = 18,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::FirstStrongIsolate,
            AssociatedConstantInStruct
        )]
        FirstStrongIsolate = 19,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::LeftToRightIsolate,
            AssociatedConstantInStruct
        )]
        LeftToRightIsolate = 20,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::RightToLeftIsolate,
            AssociatedConstantInStruct
        )]
        RightToLeftIsolate = 21,
        #[diplomat::rust_link(
            icu::properties::props::BidiClass::PopDirectionalIsolate,
            AssociatedConstantInStruct
        )]
        PopDirectionalIsolate = 22,
    }

    impl BidiClass {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::BidiClass>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[diplomat::rust_link(icu::properties::PropertyNamesLong, Struct, hidden)]
        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed, Struct, hidden)]
        #[diplomat::rust_link(icu::properties::PropertyNamesLong::new, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::new, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::properties::props::NamedEnumeratedProperty::long_name,
            FnInTrait,
            hidden
        )]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::BidiClass>::new().get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[diplomat::rust_link(icu::properties::PropertyNamesShort, Struct, hidden)]
        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed, Struct, hidden)]
        #[diplomat::rust_link(icu::properties::PropertyNamesShort::new, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::new, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::properties::props::NamedEnumeratedProperty::short_name,
            FnInTrait,
            hidden
        )]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::BidiClass>::new().get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::BidiClass::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(icu::properties::props::BidiClass::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::LeftToRight,
                1 => Self::RightToLeft,
                2 => Self::EuropeanNumber,
                3 => Self::EuropeanSeparator,
                4 => Self::EuropeanTerminator,
                5 => Self::ArabicNumber,
                6 => Self::CommonSeparator,
                7 => Self::ParagraphSeparator,
                8 => Self::SegmentSeparator,
                9 => Self::WhiteSpace,
                10 => Self::OtherNeutral,
                11 => Self::LeftToRightEmbedding,
                12 => Self::LeftToRightOverride,
                13 => Self::ArabicLetter,
                14 => Self::RightToLeftEmbedding,
                15 => Self::RightToLeftOverride,
                16 => Self::PopDirectionalFormat,
                17 => Self::NonspacingMark,
                18 => Self::BoundaryNeutral,
                19 => Self::FirstStrongIsolate,
                20 => Self::LeftToRightIsolate,
                21 => Self::RightToLeftIsolate,
                22 => Self::PopDirectionalIsolate,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::Script, Struct)]
    #[diplomat::enum_convert(icu_properties::props::Script, needs_wildcard)]
    pub enum Script {
        #[diplomat::rust_link(icu::properties::props::Script::Adlam, AssociatedConstantInStruct)]
        Adlam = 167,
        #[diplomat::rust_link(icu::properties::props::Script::Ahom, AssociatedConstantInStruct)]
        Ahom = 161,
        #[diplomat::rust_link(
            icu::properties::props::Script::AnatolianHieroglyphs,
            AssociatedConstantInStruct
        )]
        AnatolianHieroglyphs = 156,
        #[diplomat::rust_link(icu::properties::props::Script::Arabic, AssociatedConstantInStruct)]
        Arabic = 2,
        #[diplomat::rust_link(
            icu::properties::props::Script::Armenian,
            AssociatedConstantInStruct
        )]
        Armenian = 3,
        #[diplomat::rust_link(icu::properties::props::Script::Avestan, AssociatedConstantInStruct)]
        Avestan = 117,
        #[diplomat::rust_link(
            icu::properties::props::Script::Balinese,
            AssociatedConstantInStruct
        )]
        Balinese = 62,
        #[diplomat::rust_link(icu::properties::props::Script::Bamum, AssociatedConstantInStruct)]
        Bamum = 130,
        #[diplomat::rust_link(
            icu::properties::props::Script::BassaVah,
            AssociatedConstantInStruct
        )]
        BassaVah = 134,
        #[diplomat::rust_link(icu::properties::props::Script::Batak, AssociatedConstantInStruct)]
        Batak = 63,
        #[diplomat::rust_link(icu::properties::props::Script::Bengali, AssociatedConstantInStruct)]
        Bengali = 4,
        #[diplomat::rust_link(
            icu::properties::props::Script::Bhaiksuki,
            AssociatedConstantInStruct
        )]
        Bhaiksuki = 168,
        #[diplomat::rust_link(
            icu::properties::props::Script::Bopomofo,
            AssociatedConstantInStruct
        )]
        Bopomofo = 5,
        #[diplomat::rust_link(icu::properties::props::Script::Brahmi, AssociatedConstantInStruct)]
        Brahmi = 65,
        #[diplomat::rust_link(icu::properties::props::Script::Braille, AssociatedConstantInStruct)]
        Braille = 46,
        #[diplomat::rust_link(
            icu::properties::props::Script::Buginese,
            AssociatedConstantInStruct
        )]
        Buginese = 55,
        #[diplomat::rust_link(icu::properties::props::Script::Buhid, AssociatedConstantInStruct)]
        Buhid = 44,
        #[diplomat::rust_link(
            icu::properties::props::Script::CanadianAboriginal,
            AssociatedConstantInStruct
        )]
        CanadianAboriginal = 40,
        #[diplomat::rust_link(icu::properties::props::Script::Carian, AssociatedConstantInStruct)]
        Carian = 104,
        #[diplomat::rust_link(
            icu::properties::props::Script::CaucasianAlbanian,
            AssociatedConstantInStruct
        )]
        CaucasianAlbanian = 159,
        #[diplomat::rust_link(icu::properties::props::Script::Chakma, AssociatedConstantInStruct)]
        Chakma = 118,
        #[diplomat::rust_link(icu::properties::props::Script::Cham, AssociatedConstantInStruct)]
        Cham = 66,
        #[diplomat::rust_link(
            icu::properties::props::Script::Cherokee,
            AssociatedConstantInStruct
        )]
        Cherokee = 6,
        #[diplomat::rust_link(
            icu::properties::props::Script::Chorasmian,
            AssociatedConstantInStruct
        )]
        Chorasmian = 189,
        #[diplomat::rust_link(icu::properties::props::Script::Common, AssociatedConstantInStruct)]
        Common = 0,
        #[diplomat::rust_link(icu::properties::props::Script::Coptic, AssociatedConstantInStruct)]
        Coptic = 7,
        #[diplomat::rust_link(
            icu::properties::props::Script::Cuneiform,
            AssociatedConstantInStruct
        )]
        Cuneiform = 101,
        #[diplomat::rust_link(icu::properties::props::Script::Cypriot, AssociatedConstantInStruct)]
        Cypriot = 47,
        #[diplomat::rust_link(
            icu::properties::props::Script::CyproMinoan,
            AssociatedConstantInStruct
        )]
        CyproMinoan = 193,
        #[diplomat::rust_link(
            icu::properties::props::Script::Cyrillic,
            AssociatedConstantInStruct
        )]
        Cyrillic = 8,
        #[diplomat::rust_link(icu::properties::props::Script::Deseret, AssociatedConstantInStruct)]
        Deseret = 9,
        #[diplomat::rust_link(
            icu::properties::props::Script::Devanagari,
            AssociatedConstantInStruct
        )]
        Devanagari = 10,
        #[diplomat::rust_link(
            icu::properties::props::Script::DivesAkuru,
            AssociatedConstantInStruct
        )]
        DivesAkuru = 190,
        #[diplomat::rust_link(icu::properties::props::Script::Dogra, AssociatedConstantInStruct)]
        Dogra = 178,
        #[diplomat::rust_link(
            icu::properties::props::Script::Duployan,
            AssociatedConstantInStruct
        )]
        Duployan = 135,
        #[diplomat::rust_link(
            icu::properties::props::Script::EgyptianHieroglyphs,
            AssociatedConstantInStruct
        )]
        EgyptianHieroglyphs = 71,
        #[diplomat::rust_link(icu::properties::props::Script::Elbasan, AssociatedConstantInStruct)]
        Elbasan = 136,
        #[diplomat::rust_link(icu::properties::props::Script::Elymaic, AssociatedConstantInStruct)]
        Elymaic = 185,
        #[diplomat::rust_link(
            icu::properties::props::Script::Ethiopian,
            AssociatedConstantInStruct
        )]
        Ethiopian = 11,
        #[diplomat::rust_link(
            icu::properties::props::Script::Georgian,
            AssociatedConstantInStruct
        )]
        Georgian = 12,
        #[diplomat::rust_link(
            icu::properties::props::Script::Glagolitic,
            AssociatedConstantInStruct
        )]
        Glagolitic = 56,
        #[diplomat::rust_link(icu::properties::props::Script::Gothic, AssociatedConstantInStruct)]
        Gothic = 13,
        #[diplomat::rust_link(icu::properties::props::Script::Grantha, AssociatedConstantInStruct)]
        Grantha = 137,
        #[diplomat::rust_link(icu::properties::props::Script::Greek, AssociatedConstantInStruct)]
        Greek = 14,
        #[diplomat::rust_link(
            icu::properties::props::Script::Gujarati,
            AssociatedConstantInStruct
        )]
        Gujarati = 15,
        #[diplomat::rust_link(
            icu::properties::props::Script::GunjalaGondi,
            AssociatedConstantInStruct
        )]
        GunjalaGondi = 179,
        #[diplomat::rust_link(
            icu::properties::props::Script::Gurmukhi,
            AssociatedConstantInStruct
        )]
        Gurmukhi = 16,
        #[diplomat::rust_link(icu::properties::props::Script::Han, AssociatedConstantInStruct)]
        Han = 17,
        #[diplomat::rust_link(icu::properties::props::Script::Hangul, AssociatedConstantInStruct)]
        Hangul = 18,
        #[diplomat::rust_link(
            icu::properties::props::Script::HanifiRohingya,
            AssociatedConstantInStruct
        )]
        HanifiRohingya = 182,
        #[diplomat::rust_link(icu::properties::props::Script::Hanunoo, AssociatedConstantInStruct)]
        Hanunoo = 43,
        #[diplomat::rust_link(icu::properties::props::Script::Hatran, AssociatedConstantInStruct)]
        Hatran = 162,
        #[diplomat::rust_link(icu::properties::props::Script::Hebrew, AssociatedConstantInStruct)]
        Hebrew = 19,
        #[diplomat::rust_link(
            icu::properties::props::Script::Hiragana,
            AssociatedConstantInStruct
        )]
        Hiragana = 20,
        #[diplomat::rust_link(
            icu::properties::props::Script::ImperialAramaic,
            AssociatedConstantInStruct
        )]
        ImperialAramaic = 116,
        #[diplomat::rust_link(
            icu::properties::props::Script::Inherited,
            AssociatedConstantInStruct
        )]
        Inherited = 1,
        #[diplomat::rust_link(
            icu::properties::props::Script::InscriptionalPahlavi,
            AssociatedConstantInStruct
        )]
        InscriptionalPahlavi = 122,
        #[diplomat::rust_link(
            icu::properties::props::Script::InscriptionalParthian,
            AssociatedConstantInStruct
        )]
        InscriptionalParthian = 125,
        #[diplomat::rust_link(
            icu::properties::props::Script::Javanese,
            AssociatedConstantInStruct
        )]
        Javanese = 78,
        #[diplomat::rust_link(icu::properties::props::Script::Kaithi, AssociatedConstantInStruct)]
        Kaithi = 120,
        #[diplomat::rust_link(icu::properties::props::Script::Kannada, AssociatedConstantInStruct)]
        Kannada = 21,
        #[diplomat::rust_link(
            icu::properties::props::Script::Katakana,
            AssociatedConstantInStruct
        )]
        Katakana = 22,
        #[diplomat::rust_link(icu::properties::props::Script::Kawi, AssociatedConstantInStruct)]
        Kawi = 198,
        #[diplomat::rust_link(icu::properties::props::Script::KayahLi, AssociatedConstantInStruct)]
        KayahLi = 79,
        #[diplomat::rust_link(
            icu::properties::props::Script::Kharoshthi,
            AssociatedConstantInStruct
        )]
        Kharoshthi = 57,
        #[diplomat::rust_link(
            icu::properties::props::Script::KhitanSmallScript,
            AssociatedConstantInStruct
        )]
        KhitanSmallScript = 191,
        #[diplomat::rust_link(icu::properties::props::Script::Khmer, AssociatedConstantInStruct)]
        Khmer = 23,
        #[diplomat::rust_link(icu::properties::props::Script::Khojki, AssociatedConstantInStruct)]
        Khojki = 157,
        #[diplomat::rust_link(
            icu::properties::props::Script::Khudawadi,
            AssociatedConstantInStruct
        )]
        Khudawadi = 145,
        #[diplomat::rust_link(icu::properties::props::Script::Lao, AssociatedConstantInStruct)]
        Lao = 24,
        #[diplomat::rust_link(icu::properties::props::Script::Latin, AssociatedConstantInStruct)]
        Latin = 25,
        #[diplomat::rust_link(icu::properties::props::Script::Lepcha, AssociatedConstantInStruct)]
        Lepcha = 82,
        #[diplomat::rust_link(icu::properties::props::Script::Limbu, AssociatedConstantInStruct)]
        Limbu = 48,
        #[diplomat::rust_link(icu::properties::props::Script::LinearA, AssociatedConstantInStruct)]
        LinearA = 83,
        #[diplomat::rust_link(icu::properties::props::Script::LinearB, AssociatedConstantInStruct)]
        LinearB = 49,
        #[diplomat::rust_link(icu::properties::props::Script::Lisu, AssociatedConstantInStruct)]
        Lisu = 131,
        #[diplomat::rust_link(icu::properties::props::Script::Lycian, AssociatedConstantInStruct)]
        Lycian = 107,
        #[diplomat::rust_link(icu::properties::props::Script::Lydian, AssociatedConstantInStruct)]
        Lydian = 108,
        #[diplomat::rust_link(
            icu::properties::props::Script::Mahajani,
            AssociatedConstantInStruct
        )]
        Mahajani = 160,
        #[diplomat::rust_link(icu::properties::props::Script::Makasar, AssociatedConstantInStruct)]
        Makasar = 180,
        #[diplomat::rust_link(
            icu::properties::props::Script::Malayalam,
            AssociatedConstantInStruct
        )]
        Malayalam = 26,
        #[diplomat::rust_link(icu::properties::props::Script::Mandaic, AssociatedConstantInStruct)]
        Mandaic = 84,
        #[diplomat::rust_link(
            icu::properties::props::Script::Manichaean,
            AssociatedConstantInStruct
        )]
        Manichaean = 121,
        #[diplomat::rust_link(icu::properties::props::Script::Marchen, AssociatedConstantInStruct)]
        Marchen = 169,
        #[diplomat::rust_link(
            icu::properties::props::Script::MasaramGondi,
            AssociatedConstantInStruct
        )]
        MasaramGondi = 175,
        #[diplomat::rust_link(
            icu::properties::props::Script::Medefaidrin,
            AssociatedConstantInStruct
        )]
        Medefaidrin = 181,
        #[diplomat::rust_link(
            icu::properties::props::Script::MeeteiMayek,
            AssociatedConstantInStruct
        )]
        MeeteiMayek = 115,
        #[diplomat::rust_link(
            icu::properties::props::Script::MendeKikakui,
            AssociatedConstantInStruct
        )]
        MendeKikakui = 140,
        #[diplomat::rust_link(
            icu::properties::props::Script::MeroiticCursive,
            AssociatedConstantInStruct
        )]
        MeroiticCursive = 141,
        #[diplomat::rust_link(
            icu::properties::props::Script::MeroiticHieroglyphs,
            AssociatedConstantInStruct
        )]
        MeroiticHieroglyphs = 86,
        #[diplomat::rust_link(icu::properties::props::Script::Miao, AssociatedConstantInStruct)]
        Miao = 92,
        #[diplomat::rust_link(icu::properties::props::Script::Modi, AssociatedConstantInStruct)]
        Modi = 163,
        #[diplomat::rust_link(
            icu::properties::props::Script::Mongolian,
            AssociatedConstantInStruct
        )]
        Mongolian = 27,
        #[diplomat::rust_link(icu::properties::props::Script::Mro, AssociatedConstantInStruct)]
        Mro = 149,
        #[diplomat::rust_link(icu::properties::props::Script::Multani, AssociatedConstantInStruct)]
        Multani = 164,
        #[diplomat::rust_link(icu::properties::props::Script::Myanmar, AssociatedConstantInStruct)]
        Myanmar = 28,
        #[diplomat::rust_link(
            icu::properties::props::Script::Nabataean,
            AssociatedConstantInStruct
        )]
        Nabataean = 143,
        #[diplomat::rust_link(
            icu::properties::props::Script::NagMundari,
            AssociatedConstantInStruct
        )]
        NagMundari = 199,
        #[diplomat::rust_link(
            icu::properties::props::Script::Nandinagari,
            AssociatedConstantInStruct
        )]
        Nandinagari = 187,
        #[diplomat::rust_link(
            icu::properties::props::Script::Nastaliq,
            AssociatedConstantInStruct
        )]
        Nastaliq = 200,
        #[diplomat::rust_link(
            icu::properties::props::Script::NewTaiLue,
            AssociatedConstantInStruct
        )]
        NewTaiLue = 59,
        #[diplomat::rust_link(icu::properties::props::Script::Newa, AssociatedConstantInStruct)]
        Newa = 170,
        #[diplomat::rust_link(icu::properties::props::Script::Nko, AssociatedConstantInStruct)]
        Nko = 87,
        #[diplomat::rust_link(icu::properties::props::Script::Nushu, AssociatedConstantInStruct)]
        Nushu = 150,
        #[diplomat::rust_link(
            icu::properties::props::Script::NyiakengPuachueHmong,
            AssociatedConstantInStruct
        )]
        NyiakengPuachueHmong = 186,
        #[diplomat::rust_link(icu::properties::props::Script::Ogham, AssociatedConstantInStruct)]
        Ogham = 29,
        #[diplomat::rust_link(icu::properties::props::Script::OlChiki, AssociatedConstantInStruct)]
        OlChiki = 109,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldHungarian,
            AssociatedConstantInStruct
        )]
        OldHungarian = 76,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldItalic,
            AssociatedConstantInStruct
        )]
        OldItalic = 30,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldNorthArabian,
            AssociatedConstantInStruct
        )]
        OldNorthArabian = 142,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldPermic,
            AssociatedConstantInStruct
        )]
        OldPermic = 89,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldPersian,
            AssociatedConstantInStruct
        )]
        OldPersian = 61,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldSogdian,
            AssociatedConstantInStruct
        )]
        OldSogdian = 184,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldSouthArabian,
            AssociatedConstantInStruct
        )]
        OldSouthArabian = 133,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldTurkic,
            AssociatedConstantInStruct
        )]
        OldTurkic = 88,
        #[diplomat::rust_link(
            icu::properties::props::Script::OldUyghur,
            AssociatedConstantInStruct
        )]
        OldUyghur = 194,
        #[diplomat::rust_link(icu::properties::props::Script::Oriya, AssociatedConstantInStruct)]
        Oriya = 31,
        #[diplomat::rust_link(icu::properties::props::Script::Osage, AssociatedConstantInStruct)]
        Osage = 171,
        #[diplomat::rust_link(icu::properties::props::Script::Osmanya, AssociatedConstantInStruct)]
        Osmanya = 50,
        #[diplomat::rust_link(
            icu::properties::props::Script::PahawhHmong,
            AssociatedConstantInStruct
        )]
        PahawhHmong = 75,
        #[diplomat::rust_link(
            icu::properties::props::Script::Palmyrene,
            AssociatedConstantInStruct
        )]
        Palmyrene = 144,
        #[diplomat::rust_link(
            icu::properties::props::Script::PauCinHau,
            AssociatedConstantInStruct
        )]
        PauCinHau = 165,
        #[diplomat::rust_link(icu::properties::props::Script::PhagsPa, AssociatedConstantInStruct)]
        PhagsPa = 90,
        #[diplomat::rust_link(
            icu::properties::props::Script::Phoenician,
            AssociatedConstantInStruct
        )]
        Phoenician = 91,
        #[diplomat::rust_link(
            icu::properties::props::Script::PsalterPahlavi,
            AssociatedConstantInStruct
        )]
        PsalterPahlavi = 123,
        #[diplomat::rust_link(icu::properties::props::Script::Rejang, AssociatedConstantInStruct)]
        Rejang = 110,
        #[diplomat::rust_link(icu::properties::props::Script::Runic, AssociatedConstantInStruct)]
        Runic = 32,
        #[diplomat::rust_link(
            icu::properties::props::Script::Samaritan,
            AssociatedConstantInStruct
        )]
        Samaritan = 126,
        #[diplomat::rust_link(
            icu::properties::props::Script::Saurashtra,
            AssociatedConstantInStruct
        )]
        Saurashtra = 111,
        #[diplomat::rust_link(icu::properties::props::Script::Sharada, AssociatedConstantInStruct)]
        Sharada = 151,
        #[diplomat::rust_link(icu::properties::props::Script::Shavian, AssociatedConstantInStruct)]
        Shavian = 51,
        #[diplomat::rust_link(icu::properties::props::Script::Siddham, AssociatedConstantInStruct)]
        Siddham = 166,
        #[diplomat::rust_link(
            icu::properties::props::Script::SignWriting,
            AssociatedConstantInStruct
        )]
        SignWriting = 112,
        #[diplomat::rust_link(icu::properties::props::Script::Sinhala, AssociatedConstantInStruct)]
        Sinhala = 33,
        #[diplomat::rust_link(icu::properties::props::Script::Sogdian, AssociatedConstantInStruct)]
        Sogdian = 183,
        #[diplomat::rust_link(
            icu::properties::props::Script::SoraSompeng,
            AssociatedConstantInStruct
        )]
        SoraSompeng = 152,
        #[diplomat::rust_link(icu::properties::props::Script::Soyombo, AssociatedConstantInStruct)]
        Soyombo = 176,
        #[diplomat::rust_link(
            icu::properties::props::Script::Sundanese,
            AssociatedConstantInStruct
        )]
        Sundanese = 113,
        #[diplomat::rust_link(
            icu::properties::props::Script::SylotiNagri,
            AssociatedConstantInStruct
        )]
        SylotiNagri = 58,
        #[diplomat::rust_link(icu::properties::props::Script::Syriac, AssociatedConstantInStruct)]
        Syriac = 34,
        #[diplomat::rust_link(icu::properties::props::Script::Tagalog, AssociatedConstantInStruct)]
        Tagalog = 42,
        #[diplomat::rust_link(
            icu::properties::props::Script::Tagbanwa,
            AssociatedConstantInStruct
        )]
        Tagbanwa = 45,
        #[diplomat::rust_link(icu::properties::props::Script::TaiLe, AssociatedConstantInStruct)]
        TaiLe = 52,
        #[diplomat::rust_link(icu::properties::props::Script::TaiTham, AssociatedConstantInStruct)]
        TaiTham = 106,
        #[diplomat::rust_link(icu::properties::props::Script::TaiViet, AssociatedConstantInStruct)]
        TaiViet = 127,
        #[diplomat::rust_link(icu::properties::props::Script::Takri, AssociatedConstantInStruct)]
        Takri = 153,
        #[diplomat::rust_link(icu::properties::props::Script::Tamil, AssociatedConstantInStruct)]
        Tamil = 35,
        #[diplomat::rust_link(icu::properties::props::Script::Tangsa, AssociatedConstantInStruct)]
        Tangsa = 195,
        #[diplomat::rust_link(icu::properties::props::Script::Tangut, AssociatedConstantInStruct)]
        Tangut = 154,
        #[diplomat::rust_link(icu::properties::props::Script::Telugu, AssociatedConstantInStruct)]
        Telugu = 36,
        #[diplomat::rust_link(icu::properties::props::Script::Thaana, AssociatedConstantInStruct)]
        Thaana = 37,
        #[diplomat::rust_link(icu::properties::props::Script::Thai, AssociatedConstantInStruct)]
        Thai = 38,
        #[diplomat::rust_link(icu::properties::props::Script::Tibetan, AssociatedConstantInStruct)]
        Tibetan = 39,
        #[diplomat::rust_link(
            icu::properties::props::Script::Tifinagh,
            AssociatedConstantInStruct
        )]
        Tifinagh = 60,
        #[diplomat::rust_link(icu::properties::props::Script::Tirhuta, AssociatedConstantInStruct)]
        Tirhuta = 158,
        #[diplomat::rust_link(icu::properties::props::Script::Toto, AssociatedConstantInStruct)]
        Toto = 196,
        #[diplomat::rust_link(
            icu::properties::props::Script::Ugaritic,
            AssociatedConstantInStruct
        )]
        Ugaritic = 53,
        #[diplomat::rust_link(icu::properties::props::Script::Unknown, AssociatedConstantInStruct)]
        Unknown = 103,
        #[diplomat::rust_link(icu::properties::props::Script::Vai, AssociatedConstantInStruct)]
        Vai = 99,
        #[diplomat::rust_link(
            icu::properties::props::Script::Vithkuqi,
            AssociatedConstantInStruct
        )]
        Vithkuqi = 197,
        #[diplomat::rust_link(icu::properties::props::Script::Wancho, AssociatedConstantInStruct)]
        Wancho = 188,
        #[diplomat::rust_link(
            icu::properties::props::Script::WarangCiti,
            AssociatedConstantInStruct
        )]
        WarangCiti = 146,
        #[diplomat::rust_link(icu::properties::props::Script::Yezidi, AssociatedConstantInStruct)]
        Yezidi = 192,
        #[diplomat::rust_link(icu::properties::props::Script::Yi, AssociatedConstantInStruct)]
        Yi = 41,
        #[diplomat::rust_link(
            icu::properties::props::Script::ZanabazarSquare,
            AssociatedConstantInStruct
        )]
        ZanabazarSquare = 177,
    }

    impl Script {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::Script>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::Script>::new().get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::PropertyNamesShortBorrowed::get_locale_script,
            FnInStruct,
            hidden
        )]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::Script>::new().get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::Script::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u16 {
            self as u16
        }
        #[diplomat::rust_link(icu::properties::props::Script::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u16) -> Option<Self> {
            Some(match other {
                167 => Self::Adlam,
                161 => Self::Ahom,
                156 => Self::AnatolianHieroglyphs,
                2 => Self::Arabic,
                3 => Self::Armenian,
                117 => Self::Avestan,
                62 => Self::Balinese,
                130 => Self::Bamum,
                134 => Self::BassaVah,
                63 => Self::Batak,
                4 => Self::Bengali,
                168 => Self::Bhaiksuki,
                5 => Self::Bopomofo,
                65 => Self::Brahmi,
                46 => Self::Braille,
                55 => Self::Buginese,
                44 => Self::Buhid,
                40 => Self::CanadianAboriginal,
                104 => Self::Carian,
                159 => Self::CaucasianAlbanian,
                118 => Self::Chakma,
                66 => Self::Cham,
                6 => Self::Cherokee,
                189 => Self::Chorasmian,
                0 => Self::Common,
                7 => Self::Coptic,
                101 => Self::Cuneiform,
                47 => Self::Cypriot,
                193 => Self::CyproMinoan,
                8 => Self::Cyrillic,
                9 => Self::Deseret,
                10 => Self::Devanagari,
                190 => Self::DivesAkuru,
                178 => Self::Dogra,
                135 => Self::Duployan,
                71 => Self::EgyptianHieroglyphs,
                136 => Self::Elbasan,
                185 => Self::Elymaic,
                11 => Self::Ethiopian,
                12 => Self::Georgian,
                56 => Self::Glagolitic,
                13 => Self::Gothic,
                137 => Self::Grantha,
                14 => Self::Greek,
                15 => Self::Gujarati,
                179 => Self::GunjalaGondi,
                16 => Self::Gurmukhi,
                17 => Self::Han,
                18 => Self::Hangul,
                182 => Self::HanifiRohingya,
                43 => Self::Hanunoo,
                162 => Self::Hatran,
                19 => Self::Hebrew,
                20 => Self::Hiragana,
                116 => Self::ImperialAramaic,
                1 => Self::Inherited,
                122 => Self::InscriptionalPahlavi,
                125 => Self::InscriptionalParthian,
                78 => Self::Javanese,
                120 => Self::Kaithi,
                21 => Self::Kannada,
                22 => Self::Katakana,
                198 => Self::Kawi,
                79 => Self::KayahLi,
                57 => Self::Kharoshthi,
                191 => Self::KhitanSmallScript,
                23 => Self::Khmer,
                157 => Self::Khojki,
                145 => Self::Khudawadi,
                24 => Self::Lao,
                25 => Self::Latin,
                82 => Self::Lepcha,
                48 => Self::Limbu,
                83 => Self::LinearA,
                49 => Self::LinearB,
                131 => Self::Lisu,
                107 => Self::Lycian,
                108 => Self::Lydian,
                160 => Self::Mahajani,
                180 => Self::Makasar,
                26 => Self::Malayalam,
                84 => Self::Mandaic,
                121 => Self::Manichaean,
                169 => Self::Marchen,
                175 => Self::MasaramGondi,
                181 => Self::Medefaidrin,
                115 => Self::MeeteiMayek,
                140 => Self::MendeKikakui,
                141 => Self::MeroiticCursive,
                86 => Self::MeroiticHieroglyphs,
                92 => Self::Miao,
                163 => Self::Modi,
                27 => Self::Mongolian,
                149 => Self::Mro,
                164 => Self::Multani,
                28 => Self::Myanmar,
                143 => Self::Nabataean,
                199 => Self::NagMundari,
                187 => Self::Nandinagari,
                200 => Self::Nastaliq,
                59 => Self::NewTaiLue,
                170 => Self::Newa,
                87 => Self::Nko,
                150 => Self::Nushu,
                186 => Self::NyiakengPuachueHmong,
                29 => Self::Ogham,
                109 => Self::OlChiki,
                76 => Self::OldHungarian,
                30 => Self::OldItalic,
                142 => Self::OldNorthArabian,
                89 => Self::OldPermic,
                61 => Self::OldPersian,
                184 => Self::OldSogdian,
                133 => Self::OldSouthArabian,
                88 => Self::OldTurkic,
                194 => Self::OldUyghur,
                31 => Self::Oriya,
                171 => Self::Osage,
                50 => Self::Osmanya,
                75 => Self::PahawhHmong,
                144 => Self::Palmyrene,
                165 => Self::PauCinHau,
                90 => Self::PhagsPa,
                91 => Self::Phoenician,
                123 => Self::PsalterPahlavi,
                110 => Self::Rejang,
                32 => Self::Runic,
                126 => Self::Samaritan,
                111 => Self::Saurashtra,
                151 => Self::Sharada,
                51 => Self::Shavian,
                166 => Self::Siddham,
                112 => Self::SignWriting,
                33 => Self::Sinhala,
                183 => Self::Sogdian,
                152 => Self::SoraSompeng,
                176 => Self::Soyombo,
                113 => Self::Sundanese,
                58 => Self::SylotiNagri,
                34 => Self::Syriac,
                42 => Self::Tagalog,
                45 => Self::Tagbanwa,
                52 => Self::TaiLe,
                106 => Self::TaiTham,
                127 => Self::TaiViet,
                153 => Self::Takri,
                35 => Self::Tamil,
                195 => Self::Tangsa,
                154 => Self::Tangut,
                36 => Self::Telugu,
                37 => Self::Thaana,
                38 => Self::Thai,
                39 => Self::Tibetan,
                60 => Self::Tifinagh,
                158 => Self::Tirhuta,
                196 => Self::Toto,
                53 => Self::Ugaritic,
                103 => Self::Unknown,
                99 => Self::Vai,
                197 => Self::Vithkuqi,
                188 => Self::Wancho,
                146 => Self::WarangCiti,
                192 => Self::Yezidi,
                41 => Self::Yi,
                177 => Self::ZanabazarSquare,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::HangulSyllableType, Struct)]
    #[diplomat::enum_convert(icu_properties::props::HangulSyllableType, needs_wildcard)]
    pub enum HangulSyllableType {
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::NotApplicable,
            AssociatedConstantInStruct
        )]
        NotApplicable = 0,
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::LeadingJamo,
            AssociatedConstantInStruct
        )]
        LeadingJamo = 1,
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::VowelJamo,
            AssociatedConstantInStruct
        )]
        VowelJamo = 2,
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::TrailingJamo,
            AssociatedConstantInStruct
        )]
        TrailingJamo = 3,
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::LeadingVowelSyllable,
            AssociatedConstantInStruct
        )]
        LeadingVowelSyllable = 4,
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::LeadingVowelTrailingSyllable,
            AssociatedConstantInStruct
        )]
        LeadingVowelTrailingSyllable = 5,
    }

    impl HangulSyllableType {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::HangulSyllableType>::new()
                .get32(ch)
                .into()
        }
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::to_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(
            icu::properties::props::HangulSyllableType::from_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::NotApplicable,
                1 => Self::LeadingJamo,
                2 => Self::VowelJamo,
                3 => Self::TrailingJamo,
                4 => Self::LeadingVowelSyllable,
                5 => Self::LeadingVowelTrailingSyllable,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::EastAsianWidth, Struct)]
    #[diplomat::enum_convert(icu_properties::props::EastAsianWidth, needs_wildcard)]
    pub enum EastAsianWidth {
        #[diplomat::rust_link(
            icu::properties::props::EastAsianWidth::Neutral,
            AssociatedConstantInStruct
        )]
        Neutral = 0,
        #[diplomat::rust_link(
            icu::properties::props::EastAsianWidth::Ambiguous,
            AssociatedConstantInStruct
        )]
        Ambiguous = 1,
        #[diplomat::rust_link(
            icu::properties::props::EastAsianWidth::Halfwidth,
            AssociatedConstantInStruct
        )]
        Halfwidth = 2,
        #[diplomat::rust_link(
            icu::properties::props::EastAsianWidth::Fullwidth,
            AssociatedConstantInStruct
        )]
        Fullwidth = 3,
        #[diplomat::rust_link(
            icu::properties::props::EastAsianWidth::Narrow,
            AssociatedConstantInStruct
        )]
        Narrow = 4,
        #[diplomat::rust_link(
            icu::properties::props::EastAsianWidth::Wide,
            AssociatedConstantInStruct
        )]
        Wide = 5,
    }

    impl EastAsianWidth {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::EastAsianWidth>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::EastAsianWidth>::new()
                .get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::EastAsianWidth>::new()
                .get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::EastAsianWidth::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(icu::properties::props::EastAsianWidth::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Neutral,
                1 => Self::Ambiguous,
                2 => Self::Halfwidth,
                3 => Self::Fullwidth,
                4 => Self::Narrow,
                5 => Self::Wide,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::LineBreak, Struct)]
    #[diplomat::enum_convert(icu_properties::props::LineBreak, needs_wildcard)]
    pub enum LineBreak {
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Unknown,
            AssociatedConstantInStruct
        )]
        Unknown = 0,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Ambiguous,
            AssociatedConstantInStruct
        )]
        Ambiguous = 1,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Alphabetic,
            AssociatedConstantInStruct
        )]
        Alphabetic = 2,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::BreakBoth,
            AssociatedConstantInStruct
        )]
        BreakBoth = 3,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::BreakAfter,
            AssociatedConstantInStruct
        )]
        BreakAfter = 4,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::BreakBefore,
            AssociatedConstantInStruct
        )]
        BreakBefore = 5,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::MandatoryBreak,
            AssociatedConstantInStruct
        )]
        MandatoryBreak = 6,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::ContingentBreak,
            AssociatedConstantInStruct
        )]
        ContingentBreak = 7,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::ClosePunctuation,
            AssociatedConstantInStruct
        )]
        ClosePunctuation = 8,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::CombiningMark,
            AssociatedConstantInStruct
        )]
        CombiningMark = 9,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::CarriageReturn,
            AssociatedConstantInStruct
        )]
        CarriageReturn = 10,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Exclamation,
            AssociatedConstantInStruct
        )]
        Exclamation = 11,
        #[diplomat::rust_link(icu::properties::props::LineBreak::Glue, AssociatedConstantInStruct)]
        Glue = 12,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Hyphen,
            AssociatedConstantInStruct
        )]
        Hyphen = 13,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Ideographic,
            AssociatedConstantInStruct
        )]
        Ideographic = 14,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Inseparable,
            AssociatedConstantInStruct
        )]
        Inseparable = 15,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::InfixNumeric,
            AssociatedConstantInStruct
        )]
        InfixNumeric = 16,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::LineFeed,
            AssociatedConstantInStruct
        )]
        LineFeed = 17,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Nonstarter,
            AssociatedConstantInStruct
        )]
        Nonstarter = 18,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Numeric,
            AssociatedConstantInStruct
        )]
        Numeric = 19,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::OpenPunctuation,
            AssociatedConstantInStruct
        )]
        OpenPunctuation = 20,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::PostfixNumeric,
            AssociatedConstantInStruct
        )]
        PostfixNumeric = 21,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::PrefixNumeric,
            AssociatedConstantInStruct
        )]
        PrefixNumeric = 22,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Quotation,
            AssociatedConstantInStruct
        )]
        Quotation = 23,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::ComplexContext,
            AssociatedConstantInStruct
        )]
        ComplexContext = 24,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Surrogate,
            AssociatedConstantInStruct
        )]
        Surrogate = 25,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Space,
            AssociatedConstantInStruct
        )]
        Space = 26,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::BreakSymbols,
            AssociatedConstantInStruct
        )]
        BreakSymbols = 27,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::ZWSpace,
            AssociatedConstantInStruct
        )]
        ZWSpace = 28,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::NextLine,
            AssociatedConstantInStruct
        )]
        NextLine = 29,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::WordJoiner,
            AssociatedConstantInStruct
        )]
        WordJoiner = 30,
        #[diplomat::rust_link(icu::properties::props::LineBreak::H2, AssociatedConstantInStruct)]
        H2 = 31,
        #[diplomat::rust_link(icu::properties::props::LineBreak::H3, AssociatedConstantInStruct)]
        H3 = 32,
        #[diplomat::rust_link(icu::properties::props::LineBreak::JL, AssociatedConstantInStruct)]
        JL = 33,
        #[diplomat::rust_link(icu::properties::props::LineBreak::JT, AssociatedConstantInStruct)]
        JT = 34,
        #[diplomat::rust_link(icu::properties::props::LineBreak::JV, AssociatedConstantInStruct)]
        JV = 35,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::CloseParenthesis,
            AssociatedConstantInStruct
        )]
        CloseParenthesis = 36,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::ConditionalJapaneseStarter,
            AssociatedConstantInStruct
        )]
        ConditionalJapaneseStarter = 37,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::HebrewLetter,
            AssociatedConstantInStruct
        )]
        HebrewLetter = 38,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::RegionalIndicator,
            AssociatedConstantInStruct
        )]
        RegionalIndicator = 39,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::EBase,
            AssociatedConstantInStruct
        )]
        EBase = 40,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::EModifier,
            AssociatedConstantInStruct
        )]
        EModifier = 41,
        #[diplomat::rust_link(icu::properties::props::LineBreak::ZWJ, AssociatedConstantInStruct)]
        ZWJ = 42,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Aksara,
            AssociatedConstantInStruct
        )]
        Aksara = 43,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::AksaraPrebase,
            AssociatedConstantInStruct
        )]
        AksaraPrebase = 44,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::AksaraStart,
            AssociatedConstantInStruct
        )]
        AksaraStart = 45,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::ViramaFinal,
            AssociatedConstantInStruct
        )]
        ViramaFinal = 46,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Virama,
            AssociatedConstantInStruct
        )]
        Virama = 47,
    }

    impl LineBreak {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::LineBreak>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::LineBreak>::new().get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::LineBreak>::new().get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::LineBreak::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(icu::properties::props::LineBreak::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Unknown,
                1 => Self::Ambiguous,
                2 => Self::Alphabetic,
                3 => Self::BreakBoth,
                4 => Self::BreakAfter,
                5 => Self::BreakBefore,
                6 => Self::MandatoryBreak,
                7 => Self::ContingentBreak,
                8 => Self::ClosePunctuation,
                9 => Self::CombiningMark,
                10 => Self::CarriageReturn,
                11 => Self::Exclamation,
                12 => Self::Glue,
                13 => Self::Hyphen,
                14 => Self::Ideographic,
                15 => Self::Inseparable,
                16 => Self::InfixNumeric,
                17 => Self::LineFeed,
                18 => Self::Nonstarter,
                19 => Self::Numeric,
                20 => Self::OpenPunctuation,
                21 => Self::PostfixNumeric,
                22 => Self::PrefixNumeric,
                23 => Self::Quotation,
                24 => Self::ComplexContext,
                25 => Self::Surrogate,
                26 => Self::Space,
                27 => Self::BreakSymbols,
                28 => Self::ZWSpace,
                29 => Self::NextLine,
                30 => Self::WordJoiner,
                31 => Self::H2,
                32 => Self::H3,
                33 => Self::JL,
                34 => Self::JT,
                35 => Self::JV,
                36 => Self::CloseParenthesis,
                37 => Self::ConditionalJapaneseStarter,
                38 => Self::HebrewLetter,
                39 => Self::RegionalIndicator,
                40 => Self::EBase,
                41 => Self::EModifier,
                42 => Self::ZWJ,
                43 => Self::Aksara,
                44 => Self::AksaraPrebase,
                45 => Self::AksaraStart,
                46 => Self::ViramaFinal,
                47 => Self::Virama,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::GraphemeClusterBreak, Struct)]
    #[diplomat::enum_convert(icu_properties::props::GraphemeClusterBreak, needs_wildcard)]
    pub enum GraphemeClusterBreak {
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Other,
            AssociatedConstantInStruct
        )]
        Other = 0,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Control,
            AssociatedConstantInStruct
        )]
        Control = 1,
        #[diplomat::rust_link(icu::properties::props::LineBreak::CR, AssociatedConstantInStruct)]
        CR = 2,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Extend,
            AssociatedConstantInStruct
        )]
        Extend = 3,
        #[diplomat::rust_link(icu::properties::props::LineBreak::L, AssociatedConstantInStruct)]
        L = 4,
        #[diplomat::rust_link(icu::properties::props::LineBreak::LF, AssociatedConstantInStruct)]
        LF = 5,
        #[diplomat::rust_link(icu::properties::props::LineBreak::LV, AssociatedConstantInStruct)]
        LV = 6,
        #[diplomat::rust_link(icu::properties::props::LineBreak::LVT, AssociatedConstantInStruct)]
        LVT = 7,
        #[diplomat::rust_link(icu::properties::props::LineBreak::T, AssociatedConstantInStruct)]
        T = 8,
        #[diplomat::rust_link(icu::properties::props::LineBreak::V, AssociatedConstantInStruct)]
        V = 9,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::SpacingMark,
            AssociatedConstantInStruct
        )]
        SpacingMark = 10,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::Prepend,
            AssociatedConstantInStruct
        )]
        Prepend = 11,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::RegionalIndicator,
            AssociatedConstantInStruct
        )]
        RegionalIndicator = 12,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::EBase,
            AssociatedConstantInStruct
        )]
        EBase = 13,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::EBaseGAZ,
            AssociatedConstantInStruct
        )]
        EBaseGAZ = 14,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::EModifier,
            AssociatedConstantInStruct
        )]
        EModifier = 15,
        #[diplomat::rust_link(
            icu::properties::props::LineBreak::GlueAfterZwj,
            AssociatedConstantInStruct
        )]
        GlueAfterZwj = 16,
        #[diplomat::rust_link(icu::properties::props::LineBreak::ZWJ, AssociatedConstantInStruct)]
        ZWJ = 17,
    }

    impl GraphemeClusterBreak {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::GraphemeClusterBreak>::new()
                .get32(ch)
                .into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GraphemeClusterBreak::to_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(
            icu::properties::props::GraphemeClusterBreak::from_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Other,
                1 => Self::Control,
                2 => Self::CR,
                3 => Self::Extend,
                4 => Self::L,
                5 => Self::LF,
                6 => Self::LV,
                7 => Self::LVT,
                8 => Self::T,
                9 => Self::V,
                10 => Self::SpacingMark,
                11 => Self::Prepend,
                12 => Self::RegionalIndicator,
                13 => Self::EBase,
                14 => Self::EBaseGAZ,
                15 => Self::EModifier,
                16 => Self::GlueAfterZwj,
                17 => Self::ZWJ,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::WordBreak, Struct)]
    #[diplomat::enum_convert(icu_properties::props::WordBreak, needs_wildcard)]
    pub enum WordBreak {
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::Other,
            AssociatedConstantInStruct
        )]
        Other = 0,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::ALetter,
            AssociatedConstantInStruct
        )]
        ALetter = 1,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::Format,
            AssociatedConstantInStruct
        )]
        Format = 2,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::Katakana,
            AssociatedConstantInStruct
        )]
        Katakana = 3,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::MidLetter,
            AssociatedConstantInStruct
        )]
        MidLetter = 4,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::MidNum,
            AssociatedConstantInStruct
        )]
        MidNum = 5,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::Numeric,
            AssociatedConstantInStruct
        )]
        Numeric = 6,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::ExtendNumLet,
            AssociatedConstantInStruct
        )]
        ExtendNumLet = 7,
        #[diplomat::rust_link(icu::properties::props::WordBreak::CR, AssociatedConstantInStruct)]
        CR = 8,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::Extend,
            AssociatedConstantInStruct
        )]
        Extend = 9,
        #[diplomat::rust_link(icu::properties::props::WordBreak::LF, AssociatedConstantInStruct)]
        LF = 10,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::MidNumLet,
            AssociatedConstantInStruct
        )]
        MidNumLet = 11,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::Newline,
            AssociatedConstantInStruct
        )]
        Newline = 12,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::RegionalIndicator,
            AssociatedConstantInStruct
        )]
        RegionalIndicator = 13,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::HebrewLetter,
            AssociatedConstantInStruct
        )]
        HebrewLetter = 14,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::SingleQuote,
            AssociatedConstantInStruct
        )]
        SingleQuote = 15,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::DoubleQuote,
            AssociatedConstantInStruct
        )]
        DoubleQuote = 16,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::EBase,
            AssociatedConstantInStruct
        )]
        EBase = 17,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::EBaseGAZ,
            AssociatedConstantInStruct
        )]
        EBaseGAZ = 18,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::EModifier,
            AssociatedConstantInStruct
        )]
        EModifier = 19,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::GlueAfterZwj,
            AssociatedConstantInStruct
        )]
        GlueAfterZwj = 20,
        #[diplomat::rust_link(icu::properties::props::WordBreak::ZWJ, AssociatedConstantInStruct)]
        ZWJ = 21,
        #[diplomat::rust_link(
            icu::properties::props::WordBreak::WSegSpace,
            AssociatedConstantInStruct
        )]
        WSegSpace = 22,
    }

    impl WordBreak {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::WordBreak>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::WordBreak>::new().get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::WordBreak>::new().get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::WordBreak::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(icu::properties::props::WordBreak::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Other,
                1 => Self::ALetter,
                2 => Self::Format,
                3 => Self::Katakana,
                4 => Self::MidLetter,
                5 => Self::MidNum,
                6 => Self::Numeric,
                7 => Self::ExtendNumLet,
                8 => Self::CR,
                9 => Self::Extend,
                10 => Self::LF,
                11 => Self::MidNumLet,
                12 => Self::Newline,
                13 => Self::RegionalIndicator,
                14 => Self::HebrewLetter,
                15 => Self::SingleQuote,
                16 => Self::DoubleQuote,
                17 => Self::EBase,
                18 => Self::EBaseGAZ,
                19 => Self::EModifier,
                20 => Self::GlueAfterZwj,
                21 => Self::ZWJ,
                22 => Self::WSegSpace,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::SentenceBreak, Struct)]
    #[diplomat::enum_convert(icu_properties::props::SentenceBreak, needs_wildcard)]
    pub enum SentenceBreak {
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Other,
            AssociatedConstantInStruct
        )]
        Other = 0,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::ATerm,
            AssociatedConstantInStruct
        )]
        ATerm = 1,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Close,
            AssociatedConstantInStruct
        )]
        Close = 2,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Format,
            AssociatedConstantInStruct
        )]
        Format = 3,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Lower,
            AssociatedConstantInStruct
        )]
        Lower = 4,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Numeric,
            AssociatedConstantInStruct
        )]
        Numeric = 5,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::OLetter,
            AssociatedConstantInStruct
        )]
        OLetter = 6,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Sep,
            AssociatedConstantInStruct
        )]
        Sep = 7,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Sp,
            AssociatedConstantInStruct
        )]
        Sp = 8,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::STerm,
            AssociatedConstantInStruct
        )]
        STerm = 9,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Upper,
            AssociatedConstantInStruct
        )]
        Upper = 10,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::CR,
            AssociatedConstantInStruct
        )]
        CR = 11,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::Extend,
            AssociatedConstantInStruct
        )]
        Extend = 12,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::LF,
            AssociatedConstantInStruct
        )]
        LF = 13,
        #[diplomat::rust_link(
            icu::properties::props::SentenceBreak::SContinue,
            AssociatedConstantInStruct
        )]
        SContinue = 14,
    }

    impl SentenceBreak {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::SentenceBreak>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::SentenceBreak>::new()
                .get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::SentenceBreak>::new()
                .get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::SentenceBreak::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(icu::properties::props::SentenceBreak::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Other,
                1 => Self::ATerm,
                2 => Self::Close,
                3 => Self::Format,
                4 => Self::Lower,
                5 => Self::Numeric,
                6 => Self::OLetter,
                7 => Self::Sep,
                8 => Self::Sp,
                9 => Self::STerm,
                10 => Self::Upper,
                11 => Self::CR,
                12 => Self::Extend,
                13 => Self::LF,
                14 => Self::SContinue,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::CanonicalCombiningClass, Struct)]
    #[diplomat::enum_convert(icu_properties::props::CanonicalCombiningClass, needs_wildcard)]
    pub enum CanonicalCombiningClass {
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::NotReordered,
            AssociatedConstantInStruct
        )]
        NotReordered = 0,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Overlay,
            AssociatedConstantInStruct
        )]
        Overlay = 1,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::HanReading,
            AssociatedConstantInStruct
        )]
        HanReading = 6,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Nukta,
            AssociatedConstantInStruct
        )]
        Nukta = 7,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::KanaVoicing,
            AssociatedConstantInStruct
        )]
        KanaVoicing = 8,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Virama,
            AssociatedConstantInStruct
        )]
        Virama = 9,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC10,
            AssociatedConstantInStruct
        )]
        CCC10 = 10,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC11,
            AssociatedConstantInStruct
        )]
        CCC11 = 11,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC12,
            AssociatedConstantInStruct
        )]
        CCC12 = 12,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC13,
            AssociatedConstantInStruct
        )]
        CCC13 = 13,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC14,
            AssociatedConstantInStruct
        )]
        CCC14 = 14,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC15,
            AssociatedConstantInStruct
        )]
        CCC15 = 15,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC16,
            AssociatedConstantInStruct
        )]
        CCC16 = 16,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC17,
            AssociatedConstantInStruct
        )]
        CCC17 = 17,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC18,
            AssociatedConstantInStruct
        )]
        CCC18 = 18,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC19,
            AssociatedConstantInStruct
        )]
        CCC19 = 19,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC20,
            AssociatedConstantInStruct
        )]
        CCC20 = 20,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC21,
            AssociatedConstantInStruct
        )]
        CCC21 = 21,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC22,
            AssociatedConstantInStruct
        )]
        CCC22 = 22,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC23,
            AssociatedConstantInStruct
        )]
        CCC23 = 23,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC24,
            AssociatedConstantInStruct
        )]
        CCC24 = 24,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC25,
            AssociatedConstantInStruct
        )]
        CCC25 = 25,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC26,
            AssociatedConstantInStruct
        )]
        CCC26 = 26,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC27,
            AssociatedConstantInStruct
        )]
        CCC27 = 27,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC28,
            AssociatedConstantInStruct
        )]
        CCC28 = 28,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC29,
            AssociatedConstantInStruct
        )]
        CCC29 = 29,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC30,
            AssociatedConstantInStruct
        )]
        CCC30 = 30,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC31,
            AssociatedConstantInStruct
        )]
        CCC31 = 31,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC32,
            AssociatedConstantInStruct
        )]
        CCC32 = 32,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC33,
            AssociatedConstantInStruct
        )]
        CCC33 = 33,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC34,
            AssociatedConstantInStruct
        )]
        CCC34 = 34,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC35,
            AssociatedConstantInStruct
        )]
        CCC35 = 35,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC36,
            AssociatedConstantInStruct
        )]
        CCC36 = 36,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC84,
            AssociatedConstantInStruct
        )]
        CCC84 = 84,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC91,
            AssociatedConstantInStruct
        )]
        CCC91 = 91,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC103,
            AssociatedConstantInStruct
        )]
        CCC103 = 103,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC107,
            AssociatedConstantInStruct
        )]
        CCC107 = 107,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC118,
            AssociatedConstantInStruct
        )]
        CCC118 = 118,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC122,
            AssociatedConstantInStruct
        )]
        CCC122 = 122,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC129,
            AssociatedConstantInStruct
        )]
        CCC129 = 129,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC130,
            AssociatedConstantInStruct
        )]
        CCC130 = 130,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC132,
            AssociatedConstantInStruct
        )]
        CCC132 = 132,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::CCC133,
            AssociatedConstantInStruct
        )]
        CCC133 = 133,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::AttachedBelowLeft,
            AssociatedConstantInStruct
        )]
        AttachedBelowLeft = 200,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::AttachedBelow,
            AssociatedConstantInStruct
        )]
        AttachedBelow = 202,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::AttachedAbove,
            AssociatedConstantInStruct
        )]
        AttachedAbove = 214,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::AttachedAboveRight,
            AssociatedConstantInStruct
        )]
        AttachedAboveRight = 216,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::BelowLeft,
            AssociatedConstantInStruct
        )]
        BelowLeft = 218,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Below,
            AssociatedConstantInStruct
        )]
        Below = 220,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::BelowRight,
            AssociatedConstantInStruct
        )]
        BelowRight = 222,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Left,
            AssociatedConstantInStruct
        )]
        Left = 224,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Right,
            AssociatedConstantInStruct
        )]
        Right = 226,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::AboveLeft,
            AssociatedConstantInStruct
        )]
        AboveLeft = 228,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::Above,
            AssociatedConstantInStruct
        )]
        Above = 230,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::AboveRight,
            AssociatedConstantInStruct
        )]
        AboveRight = 232,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::DoubleBelow,
            AssociatedConstantInStruct
        )]
        DoubleBelow = 233,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::DoubleAbove,
            AssociatedConstantInStruct
        )]
        DoubleAbove = 234,
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::IotaSubscript,
            AssociatedConstantInStruct
        )]
        IotaSubscript = 240,
    }

    impl CanonicalCombiningClass {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::CanonicalCombiningClass>::new()
                .get32(ch)
                .into()
        }
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::to_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(
            icu::properties::props::CanonicalCombiningClass::from_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::NotReordered,
                1 => Self::Overlay,
                6 => Self::HanReading,
                7 => Self::Nukta,
                8 => Self::KanaVoicing,
                9 => Self::Virama,
                10 => Self::CCC10,
                11 => Self::CCC11,
                12 => Self::CCC12,
                13 => Self::CCC13,
                14 => Self::CCC14,
                15 => Self::CCC15,
                16 => Self::CCC16,
                17 => Self::CCC17,
                18 => Self::CCC18,
                19 => Self::CCC19,
                20 => Self::CCC20,
                21 => Self::CCC21,
                22 => Self::CCC22,
                23 => Self::CCC23,
                24 => Self::CCC24,
                25 => Self::CCC25,
                26 => Self::CCC26,
                27 => Self::CCC27,
                28 => Self::CCC28,
                29 => Self::CCC29,
                30 => Self::CCC30,
                31 => Self::CCC31,
                32 => Self::CCC32,
                33 => Self::CCC33,
                34 => Self::CCC34,
                35 => Self::CCC35,
                36 => Self::CCC36,
                84 => Self::CCC84,
                91 => Self::CCC91,
                103 => Self::CCC103,
                107 => Self::CCC107,
                118 => Self::CCC118,
                122 => Self::CCC122,
                129 => Self::CCC129,
                130 => Self::CCC130,
                132 => Self::CCC132,
                133 => Self::CCC133,
                200 => Self::AttachedBelowLeft,
                202 => Self::AttachedBelow,
                214 => Self::AttachedAbove,
                216 => Self::AttachedAboveRight,
                218 => Self::BelowLeft,
                220 => Self::Below,
                222 => Self::BelowRight,
                224 => Self::Left,
                226 => Self::Right,
                228 => Self::AboveLeft,
                230 => Self::Above,
                232 => Self::AboveRight,
                233 => Self::DoubleBelow,
                234 => Self::DoubleAbove,
                240 => Self::IotaSubscript,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::IndicSyllabicCategory, Struct)]
    #[diplomat::enum_convert(icu_properties::props::IndicSyllabicCategory, needs_wildcard)]
    pub enum IndicSyllabicCategory {
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Other,
            AssociatedConstantInStruct
        )]
        Other = 0,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Avagraha,
            AssociatedConstantInStruct
        )]
        Avagraha = 1,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Bindu,
            AssociatedConstantInStruct
        )]
        Bindu = 2,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::BrahmiJoiningNumber,
            AssociatedConstantInStruct
        )]
        BrahmiJoiningNumber = 3,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::CantillationMark,
            AssociatedConstantInStruct
        )]
        CantillationMark = 4,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Consonant,
            AssociatedConstantInStruct
        )]
        Consonant = 5,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantDead,
            AssociatedConstantInStruct
        )]
        ConsonantDead = 6,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantFinal,
            AssociatedConstantInStruct
        )]
        ConsonantFinal = 7,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantHeadLetter,
            AssociatedConstantInStruct
        )]
        ConsonantHeadLetter = 8,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantInitialPostfixed,
            AssociatedConstantInStruct
        )]
        ConsonantInitialPostfixed = 9,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantKiller,
            AssociatedConstantInStruct
        )]
        ConsonantKiller = 10,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantMedial,
            AssociatedConstantInStruct
        )]
        ConsonantMedial = 11,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantPlaceholder,
            AssociatedConstantInStruct
        )]
        ConsonantPlaceholder = 12,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantPrecedingRepha,
            AssociatedConstantInStruct
        )]
        ConsonantPrecedingRepha = 13,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantPrefixed,
            AssociatedConstantInStruct
        )]
        ConsonantPrefixed = 14,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantSucceedingRepha,
            AssociatedConstantInStruct
        )]
        ConsonantSucceedingRepha = 15,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantSubjoined,
            AssociatedConstantInStruct
        )]
        ConsonantSubjoined = 16,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ConsonantWithStacker,
            AssociatedConstantInStruct
        )]
        ConsonantWithStacker = 17,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::GeminationMark,
            AssociatedConstantInStruct
        )]
        GeminationMark = 18,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::InvisibleStacker,
            AssociatedConstantInStruct
        )]
        InvisibleStacker = 19,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Joiner,
            AssociatedConstantInStruct
        )]
        Joiner = 20,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ModifyingLetter,
            AssociatedConstantInStruct
        )]
        ModifyingLetter = 21,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::NonJoiner,
            AssociatedConstantInStruct
        )]
        NonJoiner = 22,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Nukta,
            AssociatedConstantInStruct
        )]
        Nukta = 23,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Number,
            AssociatedConstantInStruct
        )]
        Number = 24,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::NumberJoiner,
            AssociatedConstantInStruct
        )]
        NumberJoiner = 25,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::PureKiller,
            AssociatedConstantInStruct
        )]
        PureKiller = 26,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::RegisterShifter,
            AssociatedConstantInStruct
        )]
        RegisterShifter = 27,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::SyllableModifier,
            AssociatedConstantInStruct
        )]
        SyllableModifier = 28,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ToneLetter,
            AssociatedConstantInStruct
        )]
        ToneLetter = 29,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ToneMark,
            AssociatedConstantInStruct
        )]
        ToneMark = 30,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Virama,
            AssociatedConstantInStruct
        )]
        Virama = 31,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Visarga,
            AssociatedConstantInStruct
        )]
        Visarga = 32,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::Vowel,
            AssociatedConstantInStruct
        )]
        Vowel = 33,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::VowelDependent,
            AssociatedConstantInStruct
        )]
        VowelDependent = 34,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::VowelIndependent,
            AssociatedConstantInStruct
        )]
        VowelIndependent = 35,
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::ReorderingKiller,
            AssociatedConstantInStruct
        )]
        ReorderingKiller = 36,
    }

    impl IndicSyllabicCategory {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::IndicSyllabicCategory>::new()
                .get32(ch)
                .into()
        }
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::to_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(
            icu::properties::props::IndicSyllabicCategory::from_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Other,
                1 => Self::Avagraha,
                2 => Self::Bindu,
                3 => Self::BrahmiJoiningNumber,
                4 => Self::CantillationMark,
                5 => Self::Consonant,
                6 => Self::ConsonantDead,
                7 => Self::ConsonantFinal,
                8 => Self::ConsonantHeadLetter,
                9 => Self::ConsonantInitialPostfixed,
                10 => Self::ConsonantKiller,
                11 => Self::ConsonantMedial,
                12 => Self::ConsonantPlaceholder,
                13 => Self::ConsonantPrecedingRepha,
                14 => Self::ConsonantPrefixed,
                15 => Self::ConsonantSucceedingRepha,
                16 => Self::ConsonantSubjoined,
                17 => Self::ConsonantWithStacker,
                18 => Self::GeminationMark,
                19 => Self::InvisibleStacker,
                20 => Self::Joiner,
                21 => Self::ModifyingLetter,
                22 => Self::NonJoiner,
                23 => Self::Nukta,
                24 => Self::Number,
                25 => Self::NumberJoiner,
                26 => Self::PureKiller,
                27 => Self::RegisterShifter,
                28 => Self::SyllableModifier,
                29 => Self::ToneLetter,
                30 => Self::ToneMark,
                31 => Self::Virama,
                32 => Self::Visarga,
                33 => Self::Vowel,
                34 => Self::VowelDependent,
                35 => Self::VowelIndependent,
                36 => Self::ReorderingKiller,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::JoiningType, Struct)]
    #[diplomat::enum_convert(icu_properties::props::JoiningType, needs_wildcard)]
    pub enum JoiningType {
        #[diplomat::rust_link(
            icu::properties::props::JoiningType::NonJoining,
            AssociatedConstantInStruct
        )]
        NonJoining = 0,
        #[diplomat::rust_link(
            icu::properties::props::JoiningType::JoinCausing,
            AssociatedConstantInStruct
        )]
        JoinCausing = 1,
        #[diplomat::rust_link(
            icu::properties::props::JoiningType::DualJoining,
            AssociatedConstantInStruct
        )]
        DualJoining = 2,
        #[diplomat::rust_link(
            icu::properties::props::JoiningType::LeftJoining,
            AssociatedConstantInStruct
        )]
        LeftJoining = 3,
        #[diplomat::rust_link(
            icu::properties::props::JoiningType::RightJoining,
            AssociatedConstantInStruct
        )]
        RightJoining = 4,
        #[diplomat::rust_link(
            icu::properties::props::JoiningType::Transparent,
            AssociatedConstantInStruct
        )]
        Transparent = 5,
    }

    impl JoiningType {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::JoiningType>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::JoiningType>::new().get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::JoiningType>::new().get(self.into())
        }
        #[diplomat::rust_link(icu::properties::props::JoiningType::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(icu::properties::props::JoiningType::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::NonJoining,
                1 => Self::JoinCausing,
                2 => Self::DualJoining,
                3 => Self::LeftJoining,
                4 => Self::RightJoining,
                5 => Self::Transparent,
                _ => return None,
            })
        }
    }

    #[diplomat::rust_link(icu::properties::props::GeneralCategory, Enum)]
    #[diplomat::enum_convert(icu_properties::props::GeneralCategory)]
    pub enum GeneralCategory {
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::Unassigned, EnumVariant)]
        Unassigned = 0,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::UppercaseLetter,
            EnumVariant
        )]
        UppercaseLetter = 1,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::LowercaseLetter,
            EnumVariant
        )]
        LowercaseLetter = 2,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::TitlecaseLetter,
            EnumVariant
        )]
        TitlecaseLetter = 3,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::ModifierLetter,
            EnumVariant
        )]
        ModifierLetter = 4,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::OtherLetter, EnumVariant)]
        OtherLetter = 5,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::NonspacingMark,
            EnumVariant
        )]
        NonspacingMark = 6,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::SpacingMark, EnumVariant)]
        SpacingMark = 8,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::EnclosingMark, EnumVariant)]
        EnclosingMark = 7,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::DecimalNumber, EnumVariant)]
        DecimalNumber = 9,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::LetterNumber, EnumVariant)]
        LetterNumber = 10,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::OtherNumber, EnumVariant)]
        OtherNumber = 11,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::SpaceSeparator,
            EnumVariant
        )]
        SpaceSeparator = 12,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::LineSeparator, EnumVariant)]
        LineSeparator = 13,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::ParagraphSeparator,
            EnumVariant
        )]
        ParagraphSeparator = 14,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::Control, EnumVariant)]
        Control = 15,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::Format, EnumVariant)]
        Format = 16,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::PrivateUse, EnumVariant)]
        PrivateUse = 17,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::Surrogate, EnumVariant)]
        Surrogate = 18,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::DashPunctuation,
            EnumVariant
        )]
        DashPunctuation = 19,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::OpenPunctuation,
            EnumVariant
        )]
        OpenPunctuation = 20,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::ClosePunctuation,
            EnumVariant
        )]
        ClosePunctuation = 21,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::ConnectorPunctuation,
            EnumVariant
        )]
        ConnectorPunctuation = 22,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::InitialPunctuation,
            EnumVariant
        )]
        InitialPunctuation = 28,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::FinalPunctuation,
            EnumVariant
        )]
        FinalPunctuation = 29,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::OtherPunctuation,
            EnumVariant
        )]
        OtherPunctuation = 23,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::MathSymbol, EnumVariant)]
        MathSymbol = 24,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::CurrencySymbol,
            EnumVariant
        )]
        CurrencySymbol = 25,
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategory::ModifierSymbol,
            EnumVariant
        )]
        ModifierSymbol = 26,
        #[diplomat::rust_link(icu::properties::props::GeneralCategory::OtherSymbol, EnumVariant)]
        OtherSymbol = 27,
    }

    impl GeneralCategory {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::GeneralCategory>::new()
                .get32(ch)
                .into()
        }
        /// Convert to an integer using the ICU4C integer mappings for `General_Category`

        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::GeneralCategory>::new()
                .get(self.into())
        }

        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::GeneralCategory>::new()
                .get(self.into())
        }
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }

        /// Produces a GeneralCategoryGroup mask that can represent a group of general categories
        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup, Struct)]
        pub fn to_group(self) -> GeneralCategoryGroup {
            GeneralCategoryGroup {
                mask: props::GeneralCategoryGroup::from(props::GeneralCategory::from(self)).into(),
            }
        }

        /// Convert from an integer using the ICU4C integer mappings for `General_Category`
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryOutOfBoundsError,
            Struct,
            hidden
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Unassigned,
                1 => Self::UppercaseLetter,
                2 => Self::LowercaseLetter,
                3 => Self::TitlecaseLetter,
                4 => Self::ModifierLetter,
                5 => Self::OtherLetter,
                6 => Self::NonspacingMark,
                8 => Self::SpacingMark,
                7 => Self::EnclosingMark,
                9 => Self::DecimalNumber,
                10 => Self::LetterNumber,
                11 => Self::OtherNumber,
                12 => Self::SpaceSeparator,
                13 => Self::LineSeparator,
                14 => Self::ParagraphSeparator,
                15 => Self::Control,
                16 => Self::Format,
                17 => Self::PrivateUse,
                18 => Self::Surrogate,
                19 => Self::DashPunctuation,
                20 => Self::OpenPunctuation,
                21 => Self::ClosePunctuation,
                22 => Self::ConnectorPunctuation,
                28 => Self::InitialPunctuation,
                29 => Self::FinalPunctuation,
                23 => Self::OtherPunctuation,
                24 => Self::MathSymbol,
                25 => Self::CurrencySymbol,
                26 => Self::ModifierSymbol,
                27 => Self::OtherSymbol,
                _ => return None,
            })
        }
    }

    /// A mask that is capable of representing groups of `General_Category` values.
    #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup, Struct)]
    #[derive(Default)]
    pub struct GeneralCategoryGroup {
        pub mask: u32,
    }

    impl GeneralCategoryGroup {
        #[inline]
        pub(crate) fn into_props_group(self) -> props::GeneralCategoryGroup {
            self.mask.into()
        }

        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup::contains, FnInStruct)]
        pub fn contains(self, val: GeneralCategory) -> bool {
            self.into_props_group().contains(val.into())
        }
        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup::complement, FnInStruct)]
        pub fn complement(self) -> Self {
            self.into_props_group().complement().into()
        }

        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup::all, FnInStruct)]
        pub fn all() -> Self {
            props::GeneralCategoryGroup::all().into()
        }
        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup::empty, FnInStruct)]
        pub fn empty() -> Self {
            props::GeneralCategoryGroup::empty().into()
        }
        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup::union, FnInStruct)]
        #[diplomat::attr(any(c, cpp), rename = "union_")]
        pub fn union(self, other: Self) -> Self {
            self.into_props_group()
                .union(other.into_props_group())
                .into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::intersection,
            FnInStruct
        )]
        pub fn intersection(self, other: Self) -> Self {
            self.into_props_group()
                .intersection(other.into_props_group())
                .into()
        }

        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::CasedLetter,
            AssociatedConstantInStruct
        )]
        pub fn cased_letter() -> Self {
            props::GeneralCategoryGroup::CasedLetter.into()
        }

        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Letter,
            AssociatedConstantInStruct
        )]
        pub fn letter() -> Self {
            props::GeneralCategoryGroup::Letter.into()
        }

        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Mark,
            AssociatedConstantInStruct
        )]
        pub fn mark() -> Self {
            props::GeneralCategoryGroup::Mark.into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Number,
            AssociatedConstantInStruct
        )]
        pub fn number() -> Self {
            props::GeneralCategoryGroup::Number.into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Other,
            AssociatedConstantInStruct
        )]
        pub fn separator() -> Self {
            props::GeneralCategoryGroup::Other.into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Letter,
            AssociatedConstantInStruct
        )]
        pub fn other() -> Self {
            props::GeneralCategoryGroup::Letter.into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Punctuation,
            AssociatedConstantInStruct
        )]
        pub fn punctuation() -> Self {
            props::GeneralCategoryGroup::Punctuation.into()
        }
        #[diplomat::rust_link(
            icu::properties::props::GeneralCategoryGroup::Symbol,
            AssociatedConstantInStruct
        )]
        pub fn symbol() -> Self {
            props::GeneralCategoryGroup::Symbol.into()
        }
    }

    #[diplomat::rust_link(icu::properties::props::VerticalOrientation, Struct)]
    #[diplomat::enum_convert(icu_properties::props::VerticalOrientation, needs_wildcard)]
    pub enum VerticalOrientation {
        #[diplomat::rust_link(
            icu::properties::props::VerticalOrientation::Rotated,
            AssociatedConstantInStruct
        )]
        Rotated = 0,
        #[diplomat::rust_link(
            icu::properties::props::VerticalOrientation::TransformedRotated,
            AssociatedConstantInStruct
        )]
        TransformedRotated = 1,
        #[diplomat::rust_link(
            icu::properties::props::VerticalOrientation::TransformedUpright,
            AssociatedConstantInStruct
        )]
        TransformedUpright = 2,
        #[diplomat::rust_link(
            icu::properties::props::VerticalOrientation::Upright,
            AssociatedConstantInStruct
        )]
        Upright = 3,
    }

    impl VerticalOrientation {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::VerticalOrientation>::new()
                .get32(ch)
                .into()
        }
        #[diplomat::rust_link(icu::properties::PropertyNamesLongBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "long" name of this property value (returns empty if property value is unknown)
        pub fn long_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesLongBorrowed::<props::VerticalOrientation>::new()
                .get(self.into())
        }
        #[diplomat::rust_link(icu::properties::PropertyNamesShortBorrowed::get, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        /// Get the "short" name of this property value (returns empty if property value is unknown)
        pub fn short_name(self) -> Option<&'static str> {
            icu_properties::PropertyNamesShortBorrowed::<props::VerticalOrientation>::new()
                .get(self.into())
        }
        #[diplomat::rust_link(
            icu::properties::props::VerticalOrientation::to_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and CodePointMapData
        pub fn to_integer_value(self) -> u8 {
            self as u8
        }
        #[diplomat::rust_link(
            icu::properties::props::VerticalOrientation::from_icu4c_value,
            FnInStruct
        )]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or CodePointMapData
        pub fn from_integer_value(other: u8) -> Option<Self> {
            Some(match other {
                0 => Self::Rotated,
                1 => Self::TransformedRotated,
                2 => Self::TransformedUpright,
                3 => Self::Upright,
                _ => return None,
            })
        }
    }
}

impl From<icu_properties::props::GeneralCategoryGroup> for ffi::GeneralCategoryGroup {
    #[inline]
    fn from(other: icu_properties::props::GeneralCategoryGroup) -> Self {
        Self { mask: other.into() }
    }
}
#[cfg(test)]
mod test {
    use super::ffi::*;
    use icu_properties::props;

    #[test]
    fn test_all_cases_covered() {
        for prop in props::BidiClass::ALL_VALUES {
            let ffi_prop = BidiClass::from_integer_value(prop.to_icu4c_value())
                .expect("Found BidiClass value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::BidiClass::from(ffi_prop));
        }

        for prop in props::Script::ALL_VALUES {
            let ffi_prop = Script::from_integer_value(prop.to_icu4c_value())
                .expect("Found Script value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::Script::from(ffi_prop));
        }

        for prop in props::HangulSyllableType::ALL_VALUES {
            let ffi_prop = HangulSyllableType::from_integer_value(prop.to_icu4c_value())
                .expect("Found HangulSyllableType value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::HangulSyllableType::from(ffi_prop));
        }
        for prop in props::EastAsianWidth::ALL_VALUES {
            let ffi_prop = EastAsianWidth::from_integer_value(prop.to_icu4c_value())
                .expect("Found EastAsianWidth value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::EastAsianWidth::from(ffi_prop));
        }
        for prop in props::LineBreak::ALL_VALUES {
            let ffi_prop = LineBreak::from_integer_value(prop.to_icu4c_value())
                .expect("Found LineBreak value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::LineBreak::from(ffi_prop));
        }
        for prop in props::GraphemeClusterBreak::ALL_VALUES {
            let ffi_prop = GraphemeClusterBreak::from_integer_value(prop.to_icu4c_value())
                .expect("Found GraphemeClusterBreak value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::GraphemeClusterBreak::from(ffi_prop));
        }
        for prop in props::WordBreak::ALL_VALUES {
            let ffi_prop = WordBreak::from_integer_value(prop.to_icu4c_value())
                .expect("Found WordBreak value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::WordBreak::from(ffi_prop));
        }
        for prop in props::SentenceBreak::ALL_VALUES {
            let ffi_prop = SentenceBreak::from_integer_value(prop.to_icu4c_value())
                .expect("Found SentenceBreak value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::SentenceBreak::from(ffi_prop));
        }
        for prop in props::CanonicalCombiningClass::ALL_VALUES {
            let ffi_prop = CanonicalCombiningClass::from_integer_value(prop.to_icu4c_value())
                .expect("Found CanonicalCombiningClass value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::CanonicalCombiningClass::from(ffi_prop));
        }
        for prop in props::IndicSyllabicCategory::ALL_VALUES {
            let ffi_prop = IndicSyllabicCategory::from_integer_value(prop.to_icu4c_value())
                .expect("Found IndicSyllabicCategory value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::IndicSyllabicCategory::from(ffi_prop));
        }
        for prop in props::JoiningType::ALL_VALUES {
            let ffi_prop = JoiningType::from_integer_value(prop.to_icu4c_value())
                .expect("Found JoiningType value not supported in ffi");
            assert_eq!(prop.to_icu4c_value(), ffi_prop.to_integer_value());
            assert_eq!(*prop, props::JoiningType::from(ffi_prop));
        }
        for prop in props::GeneralCategory::ALL_VALUES {
            let ffi_prop = GeneralCategory::from_integer_value(*prop as u8)
                .expect("Found GeneralCategory value not supported in ffi");
            assert_eq!(*prop as u8, ffi_prop.to_integer_value());
            assert_eq!(*prop, props::GeneralCategory::from(ffi_prop));
        }
    }
}
