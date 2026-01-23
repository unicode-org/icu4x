package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalRoundingIncrementLib: Library {
}
/** Increment used in a rounding operation.
*
*See the [Rust documentation for `RoundingIncrement`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.RoundingIncrement.html) for more information.
*/
enum class DecimalRoundingIncrement {
    MultiplesOf1,
    MultiplesOf2,
    MultiplesOf5,
    MultiplesOf25;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DecimalRoundingIncrementLib> = DecimalRoundingIncrementLib::class.java
        internal val lib: DecimalRoundingIncrementLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DecimalRoundingIncrement {
            return DecimalRoundingIncrement.entries[native]
        }

        fun default(): DecimalRoundingIncrement {
            return MultiplesOf1
        }
    }
}
