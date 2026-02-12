package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LanguageDisplayLib: Library {
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `LanguageDisplay`](https://docs.rs/icu/2.1.1/icu/experimental/displaynames/enum.LanguageDisplay.html) for more information.
*/
enum class LanguageDisplay {
    Dialect,
    Standard;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LanguageDisplayLib> = LanguageDisplayLib::class.java
        internal val lib: LanguageDisplayLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LanguageDisplay {
            return LanguageDisplay.entries[native]
        }

        fun default(): LanguageDisplay {
            return Dialect
        }
    }
}
