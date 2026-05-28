package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface ScriptLib: Library {
    fun icu4x_Script_for_char_mv1(ch: Int): Int
    fun icu4x_Script_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_Script_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_Script_to_integer_value_mv1(inner: Int): FFIUint16
    fun icu4x_Script_from_integer_value_mv1(other: FFIUint16): OptionInt
    fun icu4x_Script_try_from_str_mv1(s: Slice): OptionInt
}
/** See the [Rust documentation for `Script`](https://docs.rs/icu/2.2.0/icu/properties/props/struct.Script.html) for more information.
*/
enum class Script(val inner: Int) {
    Common(0),
    Inherited(1),
    Arabic(2),
    Armenian(3),
    Bengali(4),
    Bopomofo(5),
    Cherokee(6),
    Coptic(7),
    Cyrillic(8),
    Deseret(9),
    Devanagari(10),
    Ethiopian(11),
    Georgian(12),
    Gothic(13),
    Greek(14),
    Gujarati(15),
    Gurmukhi(16),
    Han(17),
    Hangul(18),
    Hebrew(19),
    Hiragana(20),
    Kannada(21),
    Katakana(22),
    Khmer(23),
    Lao(24),
    Latin(25),
    Malayalam(26),
    Mongolian(27),
    Myanmar(28),
    Ogham(29),
    OldItalic(30),
    Oriya(31),
    Runic(32),
    Sinhala(33),
    Syriac(34),
    Tamil(35),
    Telugu(36),
    Thaana(37),
    Thai(38),
    Tibetan(39),
    CanadianAboriginal(40),
    Yi(41),
    Tagalog(42),
    Hanunoo(43),
    Buhid(44),
    Tagbanwa(45),
    Braille(46),
    Cypriot(47),
    Limbu(48),
    LinearB(49),
    Osmanya(50),
    Shavian(51),
    TaiLe(52),
    Ugaritic(53),
    KatakanaOrHiragana(54),
    Buginese(55),
    Glagolitic(56),
    Kharoshthi(57),
    SylotiNagri(58),
    NewTaiLue(59),
    Tifinagh(60),
    OldPersian(61),
    Balinese(62),
    Batak(63),
    BlisSymbols(64),
    Brahmi(65),
    Cham(66),
    Cirth(67),
    OldChurchSlavonicCyrillic(68),
    DemoticEgyptian(69),
    HieraticEgyptian(70),
    EgyptianHieroglyphs(71),
    Khutsuri(72),
    SimplifiedHan(73),
    TraditionalHan(74),
    PahawhHmong(75),
    OldHungarian(76),
    HarappanIndus(77),
    Javanese(78),
    KayahLi(79),
    LatinFraktur(80),
    LatinGaelic(81),
    Lepcha(82),
    LinearA(83),
    Mandaic(84),
    MayanHieroglyphs(85),
    MeroiticHieroglyphs(86),
    Nko(87),
    OldTurkic(88),
    OldPermic(89),
    PhagsPa(90),
    Phoenician(91),
    Miao(92),
    Rongorongo(93),
    Sarati(94),
    EstrangeloSyriac(95),
    WesternSyriac(96),
    EasternSyriac(97),
    Tengwar(98),
    Vai(99),
    VisibleSpeech(100),
    Cuneiform(101),
    UnwrittenLanguages(102),
    Unknown(103),
    Carian(104),
    Japanese(105),
    TaiTham(106),
    Lycian(107),
    Lydian(108),
    OlChiki(109),
    Rejang(110),
    Saurashtra(111),
    SignWriting(112),
    Sundanese(113),
    Moon(114),
    MeeteiMayek(115),
    ImperialAramaic(116),
    Avestan(117),
    Chakma(118),
    Korean(119),
    Kaithi(120),
    Manichaean(121),
    InscriptionalPahlavi(122),
    PsalterPahlavi(123),
    BookPahlavi(124),
    InscriptionalParthian(125),
    Samaritan(126),
    TaiViet(127),
    MathematicalNotation(128),
    Symbols(129),
    Bamum(130),
    Lisu(131),
    NakhiGeba(132),
    OldSouthArabian(133),
    BassaVah(134),
    Duployan(135),
    Elbasan(136),
    Grantha(137),
    Kpelle(138),
    Loma(139),
    MendeKikakui(140),
    MeroiticCursive(141),
    OldNorthArabian(142),
    Nabataean(143),
    Palmyrene(144),
    Khudawadi(145),
    WarangCiti(146),
    Afaka(147),
    Jurchen(148),
    Mro(149),
    Nushu(150),
    Sharada(151),
    SoraSompeng(152),
    Takri(153),
    Tangut(154),
    Woleai(155),
    AnatolianHieroglyphs(156),
    Khojki(157),
    Tirhuta(158),
    CaucasianAlbanian(159),
    Mahajani(160),
    Ahom(161),
    Hatran(162),
    Modi(163),
    Multani(164),
    PauCinHau(165),
    Siddham(166),
    Adlam(167),
    Bhaiksuki(168),
    Marchen(169),
    Newa(170),
    Osage(171),
    HanWithBopomofo(172),
    Jamo(173),
    SymbolsEmoji(174),
    MasaramGondi(175),
    Soyombo(176),
    ZanabazarSquare(177),
    Dogra(178),
    GunjalaGondi(179),
    Makasar(180),
    Medefaidrin(181),
    HanifiRohingya(182),
    Sogdian(183),
    OldSogdian(184),
    Elymaic(185),
    NyiakengPuachueHmong(186),
    Nandinagari(187),
    Wancho(188),
    Chorasmian(189),
    DivesAkuru(190),
    KhitanSmallScript(191),
    Yezidi(192),
    CyproMinoan(193),
    OldUyghur(194),
    Tangsa(195),
    Toto(196),
    Vithkuqi(197),
    Kawi(198),
    NagMundari(199),
    Nastaliq(200),
    Garay(201),
    GurungKhema(202),
    KiratRai(203),
    OlOnal(204),
    Sunuwar(205),
    Todhri(206),
    TuluTigalari(207),
    BeriaErfe(208),
    Sidetic(209),
    TaiYo(210),
    TolongSiki(211),
    TraditionalHanWithLatin(212),
    Chisoi(254);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<ScriptLib> = ScriptLib::class.java
        internal val lib: ScriptLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): Script {
            return when (native) {
                0 -> Common
                1 -> Inherited
                2 -> Arabic
                3 -> Armenian
                4 -> Bengali
                5 -> Bopomofo
                6 -> Cherokee
                7 -> Coptic
                8 -> Cyrillic
                9 -> Deseret
                10 -> Devanagari
                11 -> Ethiopian
                12 -> Georgian
                13 -> Gothic
                14 -> Greek
                15 -> Gujarati
                16 -> Gurmukhi
                17 -> Han
                18 -> Hangul
                19 -> Hebrew
                20 -> Hiragana
                21 -> Kannada
                22 -> Katakana
                23 -> Khmer
                24 -> Lao
                25 -> Latin
                26 -> Malayalam
                27 -> Mongolian
                28 -> Myanmar
                29 -> Ogham
                30 -> OldItalic
                31 -> Oriya
                32 -> Runic
                33 -> Sinhala
                34 -> Syriac
                35 -> Tamil
                36 -> Telugu
                37 -> Thaana
                38 -> Thai
                39 -> Tibetan
                40 -> CanadianAboriginal
                41 -> Yi
                42 -> Tagalog
                43 -> Hanunoo
                44 -> Buhid
                45 -> Tagbanwa
                46 -> Braille
                47 -> Cypriot
                48 -> Limbu
                49 -> LinearB
                50 -> Osmanya
                51 -> Shavian
                52 -> TaiLe
                53 -> Ugaritic
                54 -> KatakanaOrHiragana
                55 -> Buginese
                56 -> Glagolitic
                57 -> Kharoshthi
                58 -> SylotiNagri
                59 -> NewTaiLue
                60 -> Tifinagh
                61 -> OldPersian
                62 -> Balinese
                63 -> Batak
                64 -> BlisSymbols
                65 -> Brahmi
                66 -> Cham
                67 -> Cirth
                68 -> OldChurchSlavonicCyrillic
                69 -> DemoticEgyptian
                70 -> HieraticEgyptian
                71 -> EgyptianHieroglyphs
                72 -> Khutsuri
                73 -> SimplifiedHan
                74 -> TraditionalHan
                75 -> PahawhHmong
                76 -> OldHungarian
                77 -> HarappanIndus
                78 -> Javanese
                79 -> KayahLi
                80 -> LatinFraktur
                81 -> LatinGaelic
                82 -> Lepcha
                83 -> LinearA
                84 -> Mandaic
                85 -> MayanHieroglyphs
                86 -> MeroiticHieroglyphs
                87 -> Nko
                88 -> OldTurkic
                89 -> OldPermic
                90 -> PhagsPa
                91 -> Phoenician
                92 -> Miao
                93 -> Rongorongo
                94 -> Sarati
                95 -> EstrangeloSyriac
                96 -> WesternSyriac
                97 -> EasternSyriac
                98 -> Tengwar
                99 -> Vai
                100 -> VisibleSpeech
                101 -> Cuneiform
                102 -> UnwrittenLanguages
                103 -> Unknown
                104 -> Carian
                105 -> Japanese
                106 -> TaiTham
                107 -> Lycian
                108 -> Lydian
                109 -> OlChiki
                110 -> Rejang
                111 -> Saurashtra
                112 -> SignWriting
                113 -> Sundanese
                114 -> Moon
                115 -> MeeteiMayek
                116 -> ImperialAramaic
                117 -> Avestan
                118 -> Chakma
                119 -> Korean
                120 -> Kaithi
                121 -> Manichaean
                122 -> InscriptionalPahlavi
                123 -> PsalterPahlavi
                124 -> BookPahlavi
                125 -> InscriptionalParthian
                126 -> Samaritan
                127 -> TaiViet
                128 -> MathematicalNotation
                129 -> Symbols
                130 -> Bamum
                131 -> Lisu
                132 -> NakhiGeba
                133 -> OldSouthArabian
                134 -> BassaVah
                135 -> Duployan
                136 -> Elbasan
                137 -> Grantha
                138 -> Kpelle
                139 -> Loma
                140 -> MendeKikakui
                141 -> MeroiticCursive
                142 -> OldNorthArabian
                143 -> Nabataean
                144 -> Palmyrene
                145 -> Khudawadi
                146 -> WarangCiti
                147 -> Afaka
                148 -> Jurchen
                149 -> Mro
                150 -> Nushu
                151 -> Sharada
                152 -> SoraSompeng
                153 -> Takri
                154 -> Tangut
                155 -> Woleai
                156 -> AnatolianHieroglyphs
                157 -> Khojki
                158 -> Tirhuta
                159 -> CaucasianAlbanian
                160 -> Mahajani
                161 -> Ahom
                162 -> Hatran
                163 -> Modi
                164 -> Multani
                165 -> PauCinHau
                166 -> Siddham
                167 -> Adlam
                168 -> Bhaiksuki
                169 -> Marchen
                170 -> Newa
                171 -> Osage
                172 -> HanWithBopomofo
                173 -> Jamo
                174 -> SymbolsEmoji
                175 -> MasaramGondi
                176 -> Soyombo
                177 -> ZanabazarSquare
                178 -> Dogra
                179 -> GunjalaGondi
                180 -> Makasar
                181 -> Medefaidrin
                182 -> HanifiRohingya
                183 -> Sogdian
                184 -> OldSogdian
                185 -> Elymaic
                186 -> NyiakengPuachueHmong
                187 -> Nandinagari
                188 -> Wancho
                189 -> Chorasmian
                190 -> DivesAkuru
                191 -> KhitanSmallScript
                192 -> Yezidi
                193 -> CyproMinoan
                194 -> OldUyghur
                195 -> Tangsa
                196 -> Toto
                197 -> Vithkuqi
                198 -> Kawi
                199 -> NagMundari
                200 -> Nastaliq
                201 -> Garay
                202 -> GurungKhema
                203 -> KiratRai
                204 -> OlOnal
                205 -> Sunuwar
                206 -> Todhri
                207 -> TuluTigalari
                208 -> BeriaErfe
                209 -> Sidetic
                210 -> TaiYo
                211 -> TolongSiki
                212 -> TraditionalHanWithLatin
                254 -> Chisoi
                else -> throw RuntimeException("Failed to find variant ${native} of type Script")
            }
        }

        fun default(): Script {
            return Common
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.2.0/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): Script {
            
            val returnVal = lib.icu4x_Script_for_char_mv1(ch);
            return (Script.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or `CodePointMapData`
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.2.0/icu/properties/props/struct.Script.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UShort): Script? {
            
            val returnVal = lib.icu4x_Script_from_integer_value_mv1(FFIUint16(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return Script.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun tryFromStr(s: String): Script? {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_Script_try_from_str_mv1(sSliceMemory.slice);
            try {
                
                val intermediateOption = returnVal.option() ?: return null
                return Script.fromNative(intermediateOption)
            } finally {
                sSliceMemory.close()
            }
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.2.0/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_Script_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.2.0/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_Script_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and `CodePointMapData`
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.2.0/icu/properties/props/struct.Script.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UShort {
        
        val returnVal = lib.icu4x_Script_to_integer_value_mv1(this.toNative());
        return (returnVal.toUShort())
    }
}
