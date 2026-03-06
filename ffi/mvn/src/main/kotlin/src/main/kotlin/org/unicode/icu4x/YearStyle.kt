package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface YearStyleLib: Library {
}
/** See the [Rust documentation for `YearStyle`](https://docs.rs/icu/2.1.1/icu/datetime/options/enum.YearStyle.html) for more information.
*/
enum class YearStyle {
    Auto,
    Full,
    WithEra,
    NoEra;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<YearStyleLib> = YearStyleLib::class.java
        internal val lib: YearStyleLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): YearStyle {
            return YearStyle.entries[native]
        }

        fun default(): YearStyle {
            return Auto
        }
    }
}
