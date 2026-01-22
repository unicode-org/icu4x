package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalSignedRoundingModeLib: Library {
}
/** Mode used in a rounding operation for signed numbers.
*
*See the [Rust documentation for `SignedRoundingMode`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.SignedRoundingMode.html) for more information.
*/
enum class DecimalSignedRoundingMode {
    Expand,
    Trunc,
    HalfExpand,
    HalfTrunc,
    HalfEven,
    Ceil,
    Floor,
    HalfCeil,
    HalfFloor;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DecimalSignedRoundingModeLib> = DecimalSignedRoundingModeLib::class.java
        internal val lib: DecimalSignedRoundingModeLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DecimalSignedRoundingMode {
            return DecimalSignedRoundingMode.entries[native]
        }

        fun default(): DecimalSignedRoundingMode {
            return Expand
        }
    }
}
