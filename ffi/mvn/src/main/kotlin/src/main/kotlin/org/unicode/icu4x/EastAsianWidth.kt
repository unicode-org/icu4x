package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface EastAsianWidthLib: Library {
    fun icu4x_EastAsianWidth_for_char_mv1(ch: Int): Int
    fun icu4x_EastAsianWidth_long_name_mv1(inner: Int): OptionSlice
    fun icu4x_EastAsianWidth_short_name_mv1(inner: Int): OptionSlice
    fun icu4x_EastAsianWidth_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_EastAsianWidth_from_integer_value_mv1(other: FFIUint8): OptionInt
    fun icu4x_EastAsianWidth_try_from_str_mv1(s: Slice): OptionInt
}
/** See the [Rust documentation for `EastAsianWidth`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html) for more information.
*/
enum class EastAsianWidth {
    Neutral,
    Ambiguous,
    Halfwidth,
    Fullwidth,
    Narrow,
    Wide;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<EastAsianWidthLib> = EastAsianWidthLib::class.java
        internal val lib: EastAsianWidthLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): EastAsianWidth {
            return EastAsianWidth.entries[native]
        }

        fun default(): EastAsianWidth {
            return Neutral
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): EastAsianWidth {
            
            val returnVal = lib.icu4x_EastAsianWidth_for_char_mv1(ch);
            return (EastAsianWidth.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or `CodePointMapData`
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): EastAsianWidth? {
            
            val returnVal = lib.icu4x_EastAsianWidth_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return EastAsianWidth.fromNative(intermediateOption)
        }
        @JvmStatic
        
        fun tryFromStr(s: String): EastAsianWidth? {
            val sSliceMemory = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_EastAsianWidth_try_from_str_mv1(sSliceMemory.slice);
            
            val intermediateOption = returnVal.option() ?: return null
            return EastAsianWidth.fromNative(intermediateOption)
        }
    }
    
    /** Get the "long" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
    */
    fun longName(): String? {
        
        val returnVal = lib.icu4x_EastAsianWidth_long_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Get the "short" name of this property value (returns empty if property value is unknown)
    *
    *See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
    */
    fun shortName(): String? {
        
        val returnVal = lib.icu4x_EastAsianWidth_short_name_mv1(this.toNative());
        
        val intermediateOption = returnVal.option() ?: return null
            return PrimitiveArrayTools.getUtf8(intermediateOption)
                                
    }
    
    /** Convert to an integer value usable with ICU4C and `CodePointMapData`
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.EastAsianWidth.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_EastAsianWidth_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
}
