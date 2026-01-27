package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface HangulSyllableTypeLib: Library {
    fun icu4x_HangulSyllableType_for_char_mv1(ch: Int): Int
    fun icu4x_HangulSyllableType_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_HangulSyllableType_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_HangulSyllableType_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_HangulSyllableType_from_integer_value_mv1(other: FFIUint8): OptionInt
    fun icu4x_HangulSyllableType_try_from_str_mv1(s: Slice): OptionInt
}
/** See the [Rust documentation for `HangulSyllableType`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html) for more information.
*/
enum class HangulSyllableType {
    NotApplicable,
    LeadingJamo,
    VowelJamo,
    TrailingJamo,
    LeadingVowelSyllable,
    LeadingVowelTrailingSyllable;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<HangulSyllableTypeLib> = HangulSyllableTypeLib::class.java
        internal val lib: HangulSyllableTypeLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): HangulSyllableType {
            return HangulSyllableType.entries[native]
        }

        fun default(): HangulSyllableType {
            return NotApplicable
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): HangulSyllableType {
            
            val returnVal = lib.icu4x_HangulSyllableType_for_char_mv1(ch);
            return (HangulSyllableType.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or `CodePointMapData`
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): HangulSyllableType? {
            
            val returnVal = lib.icu4x_HangulSyllableType_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return HangulSyllableType.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun tryFromStr(s: String): HangulSyllableType? {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_HangulSyllableType_try_from_str_mv1(sSlice);
            
            val intermediateOption = returnVal.option() ?: return null
            return HangulSyllableType.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_HangulSyllableType_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_HangulSyllableType_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and `CodePointMapData`
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.HangulSyllableType.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_HangulSyllableType_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
}
