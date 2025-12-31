package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface CanonicalCombiningClassLib: Library {
    fun icu4x_CanonicalCombiningClass_for_char_mv1(ch: Int): Int
    fun icu4x_CanonicalCombiningClass_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_CanonicalCombiningClass_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_CanonicalCombiningClass_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_CanonicalCombiningClass_from_integer_value_mv1(other: FFIUint8): OptionInt
}
/** See the [Rust documentation for `CanonicalCombiningClass`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html) for more information.
*/
enum class CanonicalCombiningClass(val inner: Int) {
    NotReordered(0),
    Overlay(1),
    HanReading(6),
    Nukta(7),
    KanaVoicing(8),
    Virama(9),
    CCC10(10),
    CCC11(11),
    CCC12(12),
    CCC13(13),
    CCC14(14),
    CCC15(15),
    CCC16(16),
    CCC17(17),
    CCC18(18),
    CCC19(19),
    CCC20(20),
    CCC21(21),
    CCC22(22),
    CCC23(23),
    CCC24(24),
    CCC25(25),
    CCC26(26),
    CCC27(27),
    CCC28(28),
    CCC29(29),
    CCC30(30),
    CCC31(31),
    CCC32(32),
    CCC33(33),
    CCC34(34),
    CCC35(35),
    CCC36(36),
    CCC84(84),
    CCC91(91),
    CCC103(103),
    CCC107(107),
    CCC118(118),
    CCC122(122),
    CCC129(129),
    CCC130(130),
    CCC132(132),
    CCC133(133),
    AttachedBelowLeft(200),
    AttachedBelow(202),
    AttachedAbove(214),
    AttachedAboveRight(216),
    BelowLeft(218),
    Below(220),
    BelowRight(222),
    Left(224),
    Right(226),
    AboveLeft(228),
    Above(230),
    AboveRight(232),
    DoubleBelow(233),
    DoubleAbove(234),
    IotaSubscript(240);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<CanonicalCombiningClassLib> = CanonicalCombiningClassLib::class.java
        internal val lib: CanonicalCombiningClassLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): CanonicalCombiningClass {
            return when (native) {
                0 -> NotReordered
                1 -> Overlay
                6 -> HanReading
                7 -> Nukta
                8 -> KanaVoicing
                9 -> Virama
                10 -> CCC10
                11 -> CCC11
                12 -> CCC12
                13 -> CCC13
                14 -> CCC14
                15 -> CCC15
                16 -> CCC16
                17 -> CCC17
                18 -> CCC18
                19 -> CCC19
                20 -> CCC20
                21 -> CCC21
                22 -> CCC22
                23 -> CCC23
                24 -> CCC24
                25 -> CCC25
                26 -> CCC26
                27 -> CCC27
                28 -> CCC28
                29 -> CCC29
                30 -> CCC30
                31 -> CCC31
                32 -> CCC32
                33 -> CCC33
                34 -> CCC34
                35 -> CCC35
                36 -> CCC36
                84 -> CCC84
                91 -> CCC91
                103 -> CCC103
                107 -> CCC107
                118 -> CCC118
                122 -> CCC122
                129 -> CCC129
                130 -> CCC130
                132 -> CCC132
                133 -> CCC133
                200 -> AttachedBelowLeft
                202 -> AttachedBelow
                214 -> AttachedAbove
                216 -> AttachedAboveRight
                218 -> BelowLeft
                220 -> Below
                222 -> BelowRight
                224 -> Left
                226 -> Right
                228 -> AboveLeft
                230 -> Above
                232 -> AboveRight
                233 -> DoubleBelow
                234 -> DoubleAbove
                240 -> IotaSubscript
                else -> throw RuntimeException("Failed to find variant ${native} of type CanonicalCombiningClass")
            }
        }

        fun default(): CanonicalCombiningClass {
            return NotReordered
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): CanonicalCombiningClass {
            
            val returnVal = lib.icu4x_CanonicalCombiningClass_for_char_mv1(ch);
            return (CanonicalCombiningClass.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): CanonicalCombiningClass? {
            
            val returnVal = lib.icu4x_CanonicalCombiningClass_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return CanonicalCombiningClass.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_CanonicalCombiningClass_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_CanonicalCombiningClass_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.CanonicalCombiningClass.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_CanonicalCombiningClass_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
}
