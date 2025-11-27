package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalGroupingStrategyLib: Library {
}
/** See the [Rust documentation for `GroupingStrategy`](https://docs.rs/icu/2.1.1/icu/decimal/options/enum.GroupingStrategy.html) for more information.
*/
enum class DecimalGroupingStrategy {
    Auto,
    Never,
    Always,
    Min2;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<DecimalGroupingStrategyLib> = DecimalGroupingStrategyLib::class.java
        internal val lib: DecimalGroupingStrategyLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): DecimalGroupingStrategy {
            return DecimalGroupingStrategy.entries[native]
        }

        fun default(): DecimalGroupingStrategy {
            return Auto
        }
    }
}