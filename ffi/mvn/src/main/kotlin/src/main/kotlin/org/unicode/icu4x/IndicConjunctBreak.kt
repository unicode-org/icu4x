package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface IndicConjunctBreakLib: Library {
    fun icu4x_IndicConjunctBreak_for_char_mv1(ch: Int): Int
    fun icu4x_IndicConjunctBreak_to_integer_value_mv1(inner: Int): FFIUint8
    fun icu4x_IndicConjunctBreak_from_integer_value_mv1(other: FFIUint8): OptionInt
}
/** See the [Rust documentation for `IndicConjunctBreak`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html) for more information.
*/
enum class IndicConjunctBreak {
    None,
    Consonant,
    Extend,
    Linker;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<IndicConjunctBreakLib> = IndicConjunctBreakLib::class.java
        internal val lib: IndicConjunctBreakLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): IndicConjunctBreak {
            return IndicConjunctBreak.entries[native]
        }

        fun default(): IndicConjunctBreak {
            return None
        }
        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): IndicConjunctBreak {
            
            val returnVal = lib.icu4x_IndicConjunctBreak_for_char_mv1(ch);
            return (IndicConjunctBreak.fromNative(returnVal))
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(other: UByte): IndicConjunctBreak? {
            
            val returnVal = lib.icu4x_IndicConjunctBreak_from_integer_value_mv1(FFIUint8(other));
            
            val intermediateOption = returnVal.option() ?: return null
            return IndicConjunctBreak.fromNative(intermediateOption)
        }
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.IndicConjunctBreak.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): UByte {
        
        val returnVal = lib.icu4x_IndicConjunctBreak_to_integer_value_mv1(this.toNative());
        return (returnVal.toUByte())
    }
}
