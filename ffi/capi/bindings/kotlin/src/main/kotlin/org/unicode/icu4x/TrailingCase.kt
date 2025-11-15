package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface TrailingCaseLib: Library {
}
/** See the [Rust documentation for `TrailingCase`](https://docs.rs/icu/2.1.1/icu/casemap/options/enum.TrailingCase.html) for more information.
*/
enum class TrailingCase {
    Lower,
    Unchanged;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<TrailingCaseLib> = TrailingCaseLib::class.java
        internal val lib: TrailingCaseLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): TrailingCase {
            return TrailingCase.entries[native]
        }

        fun default(): TrailingCase {
            return Lower
        }
    }
}