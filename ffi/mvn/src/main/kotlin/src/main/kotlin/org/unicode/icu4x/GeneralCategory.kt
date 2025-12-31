package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface GeneralCategoryLib: Library {
    fun icu4x_GeneralCategory_for_char_mv1(ch: Int): Int
    fun icu4x_GeneralCategory_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_GeneralCategory_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_GeneralCategory_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_GeneralCategory_from_integer_value_mv1(other: FFIUint8): OptionInt
    fun icu4x_GeneralCategory_try_from_str_mv1(s: Slice): OptionInt
    fun icu4x_GeneralCategory_to_group_mv1(inner: Int): GeneralCategoryGroupNative
}
/** See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.1.1/icu/properties/props/enum.GeneralCategory.html) for more information.
*/
enum class GeneralCategory(val inner: Int) {
    Unassigned(0),
    UppercaseLetter(1),
    LowercaseLetter(2),
    TitlecaseLetter(3),
    ModifierLetter(4),
    OtherLetter(5),
    NonspacingMark(6),
    SpacingMark(8),
    EnclosingMark(7),
    DecimalNumber(9),
    LetterNumber(10),
    OtherNumber(11),
    SpaceSeparator(12),
    LineSeparator(13),
    ParagraphSeparator(14),
    Control(15),
    Format(16),
    PrivateUse(17),
    Surrogate(18),
    DashPunctuation(19),
    OpenPunctuation(20),
    ClosePunctuation(21),
    ConnectorPunctuation(22),
    InitialPunctuation(28),
    FinalPunctuation(29),
    OtherPunctuation(23),
    MathSymbol(24),
    CurrencySymbol(25),
    ModifierSymbol(26),
    OtherSymbol(27);

    fun toNative(): Int {
        return this.inner
    }


    companion object {
        internal val libClass: Class<GeneralCategoryLib> = GeneralCategoryLib::class.java
        internal val lib: GeneralCategoryLib = Native.load("icu4x", libClass)
        fun fromNative(native: Int): GeneralCategory {
            return when (native) {
                0 -> Unassigned
                1 -> UppercaseLetter
                2 -> LowercaseLetter
                3 -> TitlecaseLetter
                4 -> ModifierLetter
                5 -> OtherLetter
                6 -> NonspacingMark
                8 -> SpacingMark
                7 -> EnclosingMark
                9 -> DecimalNumber
                10 -> LetterNumber
                11 -> OtherNumber
                12 -> SpaceSeparator
                13 -> LineSeparator
                14 -> ParagraphSeparator
                15 -> Control
                16 -> Format
                17 -> PrivateUse
                18 -> Surrogate
                19 -> DashPunctuation
                20 -> OpenPunctuation
                21 -> ClosePunctuation
                22 -> ConnectorPunctuation
                28 -> InitialPunctuation
                29 -> FinalPunctuation
                23 -> OtherPunctuation
                24 -> MathSymbol
                25 -> CurrencySymbol
                26 -> ModifierSymbol
                27 -> OtherSymbol
                else -> throw RuntimeException("Failed to find variant ${native} of type GeneralCategory")
            }
        }

        fun default(): GeneralCategory {
            return Unassigned
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): GeneralCategory {
            
            val returnVal = lib.icu4x_GeneralCategory_for_char_mv1(ch);
            return (GeneralCategory.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategory.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): GeneralCategory? {
            
            val returnVal = lib.icu4x_GeneralCategory_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return GeneralCategory.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun tryFromStr(s: String): GeneralCategory? {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_GeneralCategory_try_from_str_mv1(sSlice);
            
            val intermediateOption = returnVal.option() ?: return null
            return GeneralCategory.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_GeneralCategory_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_GeneralCategory_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategory.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_GeneralCategory_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
    
    /** Produces a GeneralCategoryGroup mask that can represent a group of general categories
    *
    *See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
    */
    fun toGroup(): GeneralCategoryGroup {
        
        val returnVal = lib.icu4x_GeneralCategory_to_group_mv1(this.toNative());
        
        val returnStruct = GeneralCategoryGroup.fromNative(returnVal)
        return returnStruct
    }
}
