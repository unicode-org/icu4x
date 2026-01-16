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
/** See the [Rust documentation for `Script`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html) for more information.
*/
enum class Script(val inner: Int) {
    Adlam(167),
    Ahom(161),
    AnatolianHieroglyphs(156),
    Arabic(2),
    Armenian(3),
    Avestan(117),
    Balinese(62),
    Bamum(130),
    BassaVah(134),
    Batak(63),
    Bengali(4),
    BeriaErfe(208),
    Bhaiksuki(168),
    Bopomofo(5),
    Brahmi(65),
    Braille(46),
    Buginese(55),
    Buhid(44),
    CanadianAboriginal(40),
    Carian(104),
    CaucasianAlbanian(159),
    Chakma(118),
    Cham(66),
    Cherokee(6),
    Chisoi(254),
    Chorasmian(189),
    Common(0),
    Coptic(7),
    Cuneiform(101),
    Cypriot(47),
    CyproMinoan(193),
    Cyrillic(8),
    Deseret(9),
    Devanagari(10),
    DivesAkuru(190),
    Dogra(178),
    Duployan(135),
    EgyptianHieroglyphs(71),
    Elbasan(136),
    Elymaic(185),
    Ethiopian(11),
    Garay(201),
    Georgian(12),
    Glagolitic(56),
    Gothic(13),
    Grantha(137),
    Greek(14),
    Gujarati(15),
    GunjalaGondi(179),
    Gurmukhi(16),
    GurungKhema(202),
    Han(17),
    Hangul(18),
    HanifiRohingya(182),
    Hanunoo(43),
    Hatran(162),
    Hebrew(19),
    Hiragana(20),
    ImperialAramaic(116),
    Inherited(1),
    InscriptionalPahlavi(122),
    InscriptionalParthian(125),
    Javanese(78),
    Kaithi(120),
    Kannada(21),
    Katakana(22),
    Kawi(198),
    KayahLi(79),
    Kharoshthi(57),
    KhitanSmallScript(191),
    Khmer(23),
    Khojki(157),
    Khudawadi(145),
    KiratRai(203),
    Lao(24),
    Latin(25),
    Lepcha(82),
    Limbu(48),
    LinearA(83),
    LinearB(49),
    Lisu(131),
    Lycian(107),
    Lydian(108),
    Mahajani(160),
    Makasar(180),
    Malayalam(26),
    Mandaic(84),
    Manichaean(121),
    Marchen(169),
    MasaramGondi(175),
    Medefaidrin(181),
    MeeteiMayek(115),
    MendeKikakui(140),
    MeroiticCursive(141),
    MeroiticHieroglyphs(86),
    Miao(92),
    Modi(163),
    Mongolian(27),
    Mro(149),
    Multani(164),
    Myanmar(28),
    Nabataean(143),
    NagMundari(199),
    Nandinagari(187),
    Nastaliq(200),
    NewTaiLue(59),
    Newa(170),
    Nko(87),
    Nushu(150),
    NyiakengPuachueHmong(186),
    Ogham(29),
    OlChiki(109),
    OlOnal(204),
    OldHungarian(76),
    OldItalic(30),
    OldNorthArabian(142),
    OldPermic(89),
    OldPersian(61),
    OldSogdian(184),
    OldSouthArabian(133),
    OldTurkic(88),
    OldUyghur(194),
    Oriya(31),
    Osage(171),
    Osmanya(50),
    PahawhHmong(75),
    Palmyrene(144),
    PauCinHau(165),
    PhagsPa(90),
    Phoenician(91),
    PsalterPahlavi(123),
    Rejang(110),
    Runic(32),
    Samaritan(126),
    Saurashtra(111),
    Sharada(151),
    Shavian(51),
    Siddham(166),
    Sidetic(209),
    SignWriting(112),
    Sinhala(33),
    Sogdian(183),
    SoraSompeng(152),
    Soyombo(176),
    Sundanese(113),
    Sunuwar(205),
    SylotiNagri(58),
    Syriac(34),
    Tagalog(42),
    Tagbanwa(45),
    TaiLe(52),
    TaiTham(106),
    TaiViet(127),
    TaiYo(210),
    Takri(153),
    Tamil(35),
    Tangsa(195),
    Tangut(154),
    Telugu(36),
    Thaana(37),
    Thai(38),
    Tibetan(39),
    Tifinagh(60),
    Tirhuta(158),
    Todhri(206),
    TolongSiki(211),
    Toto(196),
    TuluTigalari(207),
    Ugaritic(53),
    Unknown(103),
    Vai(99),
    Vithkuqi(197),
    Wancho(188),
    WarangCiti(146),
    Yezidi(192),
    Yi(41),
    ZanabazarSquare(177);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<ScriptLib> = ScriptLib::class.java
        internal val lib: ScriptLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): Script {
            return when (native) {
                167 -> Adlam
                161 -> Ahom
                156 -> AnatolianHieroglyphs
                2 -> Arabic
                3 -> Armenian
                117 -> Avestan
                62 -> Balinese
                130 -> Bamum
                134 -> BassaVah
                63 -> Batak
                4 -> Bengali
                208 -> BeriaErfe
                168 -> Bhaiksuki
                5 -> Bopomofo
                65 -> Brahmi
                46 -> Braille
                55 -> Buginese
                44 -> Buhid
                40 -> CanadianAboriginal
                104 -> Carian
                159 -> CaucasianAlbanian
                118 -> Chakma
                66 -> Cham
                6 -> Cherokee
                254 -> Chisoi
                189 -> Chorasmian
                0 -> Common
                7 -> Coptic
                101 -> Cuneiform
                47 -> Cypriot
                193 -> CyproMinoan
                8 -> Cyrillic
                9 -> Deseret
                10 -> Devanagari
                190 -> DivesAkuru
                178 -> Dogra
                135 -> Duployan
                71 -> EgyptianHieroglyphs
                136 -> Elbasan
                185 -> Elymaic
                11 -> Ethiopian
                201 -> Garay
                12 -> Georgian
                56 -> Glagolitic
                13 -> Gothic
                137 -> Grantha
                14 -> Greek
                15 -> Gujarati
                179 -> GunjalaGondi
                16 -> Gurmukhi
                202 -> GurungKhema
                17 -> Han
                18 -> Hangul
                182 -> HanifiRohingya
                43 -> Hanunoo
                162 -> Hatran
                19 -> Hebrew
                20 -> Hiragana
                116 -> ImperialAramaic
                1 -> Inherited
                122 -> InscriptionalPahlavi
                125 -> InscriptionalParthian
                78 -> Javanese
                120 -> Kaithi
                21 -> Kannada
                22 -> Katakana
                198 -> Kawi
                79 -> KayahLi
                57 -> Kharoshthi
                191 -> KhitanSmallScript
                23 -> Khmer
                157 -> Khojki
                145 -> Khudawadi
                203 -> KiratRai
                24 -> Lao
                25 -> Latin
                82 -> Lepcha
                48 -> Limbu
                83 -> LinearA
                49 -> LinearB
                131 -> Lisu
                107 -> Lycian
                108 -> Lydian
                160 -> Mahajani
                180 -> Makasar
                26 -> Malayalam
                84 -> Mandaic
                121 -> Manichaean
                169 -> Marchen
                175 -> MasaramGondi
                181 -> Medefaidrin
                115 -> MeeteiMayek
                140 -> MendeKikakui
                141 -> MeroiticCursive
                86 -> MeroiticHieroglyphs
                92 -> Miao
                163 -> Modi
                27 -> Mongolian
                149 -> Mro
                164 -> Multani
                28 -> Myanmar
                143 -> Nabataean
                199 -> NagMundari
                187 -> Nandinagari
                200 -> Nastaliq
                59 -> NewTaiLue
                170 -> Newa
                87 -> Nko
                150 -> Nushu
                186 -> NyiakengPuachueHmong
                29 -> Ogham
                109 -> OlChiki
                204 -> OlOnal
                76 -> OldHungarian
                30 -> OldItalic
                142 -> OldNorthArabian
                89 -> OldPermic
                61 -> OldPersian
                184 -> OldSogdian
                133 -> OldSouthArabian
                88 -> OldTurkic
                194 -> OldUyghur
                31 -> Oriya
                171 -> Osage
                50 -> Osmanya
                75 -> PahawhHmong
                144 -> Palmyrene
                165 -> PauCinHau
                90 -> PhagsPa
                91 -> Phoenician
                123 -> PsalterPahlavi
                110 -> Rejang
                32 -> Runic
                126 -> Samaritan
                111 -> Saurashtra
                151 -> Sharada
                51 -> Shavian
                166 -> Siddham
                209 -> Sidetic
                112 -> SignWriting
                33 -> Sinhala
                183 -> Sogdian
                152 -> SoraSompeng
                176 -> Soyombo
                113 -> Sundanese
                205 -> Sunuwar
                58 -> SylotiNagri
                34 -> Syriac
                42 -> Tagalog
                45 -> Tagbanwa
                52 -> TaiLe
                106 -> TaiTham
                127 -> TaiViet
                210 -> TaiYo
                153 -> Takri
                35 -> Tamil
                195 -> Tangsa
                154 -> Tangut
                36 -> Telugu
                37 -> Thaana
                38 -> Thai
                39 -> Tibetan
                60 -> Tifinagh
                158 -> Tirhuta
                206 -> Todhri
                211 -> TolongSiki
                196 -> Toto
                207 -> TuluTigalari
                53 -> Ugaritic
                103 -> Unknown
                99 -> Vai
                197 -> Vithkuqi
                188 -> Wancho
                146 -> WarangCiti
                192 -> Yezidi
                41 -> Yi
                177 -> ZanabazarSquare
                else -> throw RuntimeException("Failed to find variant ${native} of type Script")
            }
        }

        fun default(): Script {
            return Adlam
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): Script {
            
            val returnVal = lib.icu4x_Script_for_char_mv1(ch);
            return (Script.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UShort): Script? {
            
            val returnVal = lib.icu4x_Script_from_integer_value_mv1(FFIUint16(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return Script.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun tryFromStr(s: String): Script? {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_Script_try_from_str_mv1(sSlice);
            
            val intermediateOption = returnVal.option() ?: return null
            return Script.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_Script_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_Script_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.Script.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UShort {
        
        val returnVal = lib.icu4x_Script_to_integer_value_mv1(this.toNative());
        return (returnVal.toUShort())
    }
}
