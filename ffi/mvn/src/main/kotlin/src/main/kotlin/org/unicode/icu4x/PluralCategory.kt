package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PluralCategoryLib: Library {
    fun icu4x_PluralCategory_get_for_cldr_string_mv1(s: Slice): OptionInt
}
/** See the [Rust documentation for `PluralCategory`](https://docs.rs/icu/2.1.1/icu/plurals/enum.PluralCategory.html) for more information.
*/
enum class PluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<PluralCategoryLib> = PluralCategoryLib::class.java
        internal val lib: PluralCategoryLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): PluralCategory {
            return PluralCategory.entries[native]
        }

        fun default(): PluralCategory {
            return Zero
        }
        @JvmStatic
        
        /** Construct from a string in the format
        *[specified in TR35](https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules)
        *
        *See the [Rust documentation for `get_for_cldr_string`](https://docs.rs/icu/2.1.1/icu/plurals/enum.PluralCategory.html#method.get_for_cldr_string) for more information.
        *
        *See the [Rust documentation for `get_for_cldr_bytes`](https://docs.rs/icu/2.1.1/icu/plurals/enum.PluralCategory.html#method.get_for_cldr_bytes) for more information.
        */
        fun getForCldrString(s: String): PluralCategory? {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_PluralCategory_get_for_cldr_string_mv1(sSlice);
            
            val intermediateOption = returnVal.option() ?: return null
            return PluralCategory.fromNative(intermediateOption)
        }
    }
}