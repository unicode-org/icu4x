package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalSignDisplayLib: Library {
}
/** ECMA-402 compatible sign display preference.
*
*See the [Rust documentation for `SignDisplay`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.SignDisplay.html) for more information.
*/
enum class DecimalSignDisplay {
    Auto,
    Never,
    Always,
    ExceptZero,
    Negative;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DecimalSignDisplayLib> = DecimalSignDisplayLib::class.java
        internal val lib: DecimalSignDisplayLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DecimalSignDisplay {
            return DecimalSignDisplay.entries[native]
        }

        fun default(): DecimalSignDisplay {
            return Auto
        }
    }
}