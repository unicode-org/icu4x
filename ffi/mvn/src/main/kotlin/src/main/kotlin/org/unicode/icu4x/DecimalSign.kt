package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalSignLib: Library {
}
/** The sign of a Decimal, as shown in formatting.
*
*See the [Rust documentation for `Sign`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.Sign.html) for more information.
*/
enum class DecimalSign {
    None,
    Negative,
    Positive;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DecimalSignLib> = DecimalSignLib::class.java
        internal val lib: DecimalSignLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DecimalSign {
            return DecimalSign.entries[native]
        }

        fun default(): DecimalSign {
            return None
        }
    }
}
