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
/** See the [Rust documentation for `GeneralCategory`](https://docs.rs/icu/2.2.0/icu/properties/props/enum.GeneralCategory.html) for more information.
*/
enum class GeneralCategory {
    Unassigned,
    UppercaseLetter,
    LowercaseLetter,
    TitlecaseLetter,
    ModifierLetter,
    OtherLetter,
    NonspacingMark,
    EnclosingMark,
    SpacingMark,
    DecimalNumber,
    LetterNumber,
    OtherNumber,
    SpaceSeparator,
    LineSeparator,
    ParagraphSeparator,
    Control,
    Format,
    PrivateUse,
    Surrogate,
    DashPunctuation,
    OpenPunctuation,
    ClosePunctuation,
    ConnectorPunctuation,
    OtherPunctuation,
    MathSymbol,
    CurrencySymbol,
    ModifierSymbol,
    OtherSymbol,
    InitialPunctuation,
    FinalPunctuation;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<GeneralCategoryLib> = GeneralCategoryLib::class.java
        internal val lib: GeneralCategoryLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): GeneralCategory {
            return GeneralCategory.entries[native]
        }

        fun default(): GeneralCategory {
            return Unassigned
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.2.0/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): GeneralCategory {
            
            val returnVal = lib.icu4x_GeneralCategory_for_char_mv1(ch);
            return (GeneralCategory.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or `CodePointMapData`
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.2.0/icu/properties/props/struct.GeneralCategory.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): GeneralCategory? {
            
            val returnVal = lib.icu4x_GeneralCategory_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return GeneralCategory.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun tryFromStr(s: String): GeneralCategory? {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_GeneralCategory_try_from_str_mv1(sSliceMemory.slice);
            try {
                
                val intermediateOption = returnVal.option() ?: return null
                return GeneralCategory.fromNative(intermediateOption)
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
        
        val returnVal = lib.icu4x_GeneralCategory_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.2.0/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_GeneralCategory_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and `CodePointMapData`
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.2.0/icu/properties/props/struct.GeneralCategory.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_GeneralCategory_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
    
    /** Produces a `GeneralCategoryGroup` mask that can represent a group of general categories
    *
    *See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/2.2.0/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
    */
    fun toGroup(): GeneralCategoryGroup {
        
        val returnVal = lib.icu4x_GeneralCategory_to_group_mv1(this.toNative());
        val returnStruct = GeneralCategoryGroup.fromNative(returnVal)
        return returnStruct
    }
}
