package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface LanguageDisplayUnstableLib: Library {
}
/** 🚧 This API is unstable and may experience breaking changes outside major releases.
*
*See the [Rust documentation for `LanguageDisplay`](https://docs.rs/icu/2.3.1/icu/locale/names/enum.LanguageDisplay.html) for more information.
*/
enum class LanguageDisplayUnstable {
    Dialect,
    Standard;

    fun toNative(): Int {
        return this.ordinal
    }


    companion object {
        internal val libClass: Class<LanguageDisplayUnstableLib> = LanguageDisplayUnstableLib::class.java
        internal val lib: LanguageDisplayUnstableLib = Native.load("icu4x", libClass) 
        fun fromNative(native: Int): LanguageDisplayUnstable {
            return LanguageDisplayUnstable.entries[native]
        }

        fun default(): LanguageDisplayUnstable {
            return Dialect
        }
    }
}
